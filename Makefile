# Rust is the production implementation. IMPL=c selects the frozen C reference.
IMPL ?= rust
ifeq ($(IMPL),c)
BUILD_DIR ?= build/c-reference
include Makefile.reference
else ifeq ($(IMPL),rust)
.DEFAULT_GOAL := lib
CARGO ?= cargo
PYTHON ?= python3
JOBS ?= 8
CARGO_TARGET_DIR ?= target
RUST_TARGET ?=
FEATURES ?=
MACHINE := $(if $(RUST_TARGET),$(RUST_TARGET),$(shell uname -m))
OS := $(if $(RUST_TARGET),$(if $(findstring darwin,$(RUST_TARGET)),macos,linux),$(if $(filter Darwin,$(shell uname -s)),macos,linux))
ISA := $(if $(filter arm64% aarch64%,$(MACHINE)),arm64,x86_64)
BUILD_DIR ?= build/$(OS)-$(ISA)
RUST_FLAGS := $(if $(RUST_TARGET),--target $(RUST_TARGET)) $(if $(FEATURES),--features $(FEATURES))
RUST_OUT := $(CARGO_TARGET_DIR)/$(if $(RUST_TARGET),$(RUST_TARGET)/)release
SHLIB := $(if $(filter macos,$(OS)),libtoks.dylib,libtoks.so)
TEST_FLAGS := $(if $(findstring avx512,$(FEATURES)),--avx512)
COMMA := ,

ifneq ($(TARGET),)
$(error Use RUST_TARGET=<Rust triple> for Rust cross builds, or IMPL=c TARGET=<clang triple> for the C reference)
endif
ifneq ($(findstring windows,$(RUST_TARGET)),)
$(error The Rust OS and worker-pool port currently supports macOS and Linux; use IMPL=c for the Windows reference)
endif

.PHONY: all lib test test-scalar test-guard asm asmcheck reference clean
all: lib
lib:
	$(CARGO) build --locked --release -p toks --target-dir $(CARGO_TARGET_DIR) $(RUST_FLAGS)
	@mkdir -p $(BUILD_DIR)
	cp $(RUST_OUT)/libtoks.a $(RUST_OUT)/$(SHLIB) $(BUILD_DIR)/

test: lib asmcheck
	$(CARGO) test --locked --release -p toks --test owned --target-dir $(CARGO_TARGET_DIR) $(RUST_FLAGS)
	$(PYTHON) tools/rust-port/test.py --lib $(BUILD_DIR)/libtoks.a --out $(BUILD_DIR)/tests --jobs $(JOBS) --print-logs $(TEST_FLAGS)

test-scalar:
	TOKS_TIER=scalar $(MAKE) test

test-guard:
	$(CARGO) rustc --locked -p toks --lib --crate-type staticlib --release --features test-guard$(if $(FEATURES),$(COMMA)$(FEATURES)) --target-dir $(BUILD_DIR)/guard
	$(PYTHON) tools/rust-port/test.py --guard 1 --lib $(BUILD_DIR)/guard/release/libtoks.a --out $(BUILD_DIR)/guard1 --jobs $(JOBS) --print-logs $(TEST_FLAGS)
	$(PYTHON) tools/rust-port/test.py --guard 2 --lib $(BUILD_DIR)/guard/release/libtoks.a --out $(BUILD_DIR)/guard2 --jobs $(JOBS) --print-logs $(TEST_FLAGS)

# The assembly format/register audit remains independent of either core.
asm:
	$(MAKE) -f Makefile.reference asm BUILD_DIR=$(BUILD_DIR)
asmcheck:
	$(MAKE) -f Makefile.reference asmcheck-only BUILD_DIR=$(BUILD_DIR)

reference:
	$(MAKE) -f Makefile.reference lib BUILD_DIR=build/c-reference/$(OS)-$(ISA)

clean:
	$(CARGO) clean
	rm -rf build
else
$(error Unknown IMPL=$(IMPL); choose rust or c)
endif
