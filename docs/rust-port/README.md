# Rust migration

The Rust library implements the tokenizer core and parallel worker pool. Cargo
compiles Rust and the existing native assembly; it does not compile any C
implementation. The public C ABI and table layouts are retained so the existing
callers, kernel tests, and benchmark driver can exercise either implementation.

This is a migration in progress. The initial core was translated with C2Rust
0.22.1 and then adapted to stable Rust. It remains mostly unsafe Rust. The OS
boundary was written in Rust; alignment, atomics, Darwin worker support, and
scratch-header initialization required manual corrections. The architecture map
is in [rust-architecture.md](../rust-architecture.md).

## Build and check

```sh
cargo build --release
python3 tools/ci/fetch_tokenizers.py
python3 tools/rust-port/test.py
TOKS_TIER=scalar python3 tools/rust-port/test.py --out build/rust-tests-scalar
```

The output is `target/release/libtoks.a` and the platform's shared library. Rust
1.94.1 on Linux x86-64 and Rust 1.96.0 on macOS arm64 have been exercised. Windows
support, the Python adapter, and build-system integration remain open. The C
Makefile still builds the frozen reference implementation.

The owned Rust API uses `Tokenizer`, `Encoder` and `Decoder`. A tokenizer's clones
share immutable tables; each encoder owns its scratch and caches, and each stream
owns any growing byte-fallback hold. The owner stays alive while an encoder or
decoder exists. `encode_into` preserves the C API's total-count and exact-prefix
semantics. `encode_to` retains the output allocation between calls.

```rust,no_run
use toks::{Tokenizer, Tier, ScratchOptions, EncodeFlags, DecodeFlags};

let tokenizer = Tokenizer::from_file("tokenizer.json", Tier::Auto)?;
let mut encoder = tokenizer.encoder(ScratchOptions::default())?;
let ids = encoder.encode(b"hello world", EncodeFlags::ALL)?;
let bytes = tokenizer.decode(&ids, DecodeFlags::SKIP_SPECIAL)?;
# Ok::<(), toks::Error>(())
```

`cargo test -p toks --test owned` (also with `--release`) covers all four
tokenizer families, native/scalar parity, expansion requiring an output retry,
concurrent ownership, short outputs, and stream hold growth and failed-push
recovery. Vocabulary byte slices borrow their tokenizer; they use the core's
decoded-byte namespace, which can differ from Hugging Face's written spelling.

The harness links all 44 upstream C test callers plus the native BPE kernel test
against Rust. Four test features expose internals or replace allocation/kernel
functions for the upstream instrumented tests. They are not production options.
Both architectures passed all 45 executables in native and scalar mode. This is
not a claim that every optional fixture was available: the receipts preserve
skips for the upstream private benchmark/fuzz corpora and additional model names.
All 122 files in the public tokenizer manifest were downloaded; the critical
target test reports 91 targets, no skips, and no failures.

Protected-page builds now preserve the upstream table, scratch, ownership and
arena hooks. Every table and scratch region is moved beside an inaccessible
page, with both end and start boundary placements exercised. All 45 callers
passed both placements on macOS arm64 and Linux x86-64; the four `guard*.json`
receipts record these runs. Build the instrumented archive as a static library
because the test callers provide its hook implementations:

```sh
cargo rustc -p toks --lib --crate-type staticlib --release --features test-guard --target-dir build/rust-guard
python3 tools/rust-port/test.py --guard 1 --lib build/rust-guard/release/libtoks.a --out build/rust-tests-guard1
python3 tools/rust-port/test.py --guard 2 --lib build/rust-guard/release/libtoks.a --out build/rust-tests-guard2
```

| Area | Evidence so far | Still required |
| --- | --- | --- |
| ABI, ownership, scratch, allocation | upstream ABI/API/allocation/state tests; both protected-page placements on both hosts | sanitizer builds |
| BPE and SentencePiece BPE | kernel twins, exact IDs, short/long/cache/tie cases | broader differential corpus |
| WordPiece and Unigram | breadth, primitives, targets, normalization; 12-model Unigram oracle on both hosts | broader Python oracle suite |
| Parallelism | persistent pool, state, stall tests on both hosts | sanitizer coverage |
| Native assembly | NEON, AVX2, scalar; optional AVX-512 bucket probe tested on x86 | AVX-512 performance improvement (currently tied) |
| Python | original adapter retained as reference | Rust adapter and package validation |
| Portability | macOS arm64 and Linux x86-64 | Windows and minimum target checks |

## Performance protocol

The C reference is upstream commit
`55a5230b08a75916a2b36f92c320f057376833ea`. Both implementations link the same
`tools/bench/e2e.c` caller, use the same model files, input bytes, chunks, and cache
states, and emit complete token streams for SHA-256 equality checks. Timings
exclude loading and output validation. `tools/rust-port/bench.py` alternates
ABBA/BAAB process order, retains every repetition, and reports every cell,
including regressions. Linux runs pin to CPU 8; host, toolchain, binary hashes,
load, and sibling information are retained with results.

`tools/rust-port/corpora.py` obtains a separate reproducible public corpus. It is
not the unavailable 23-file upstream benchmark corpus. Its manifest records URLs
and SHA-256. The initial matrix is seven tokenizer families by English, source
code, multilingual Latin text, and CJK text at 4096-byte chunks, with cold,
pass-same, and warm cache states. A warm cache hit is a different workload from
first-pass tokenization; report the states separately.

An arm64 `sample` capture of the Llama 3 English cold workload attributed roughly
54% of samples to BPE merging, 31% to piece lookup, and 12% to scanning. Linux
hardware profiling is currently unavailable because `perf_event_paranoid=4`.
Timing measurements are still available there. No performance win is claimed
for the initial translation.

| Candidate | Impact (1–5) | Confidence (1–5) | Effort (1–5) | Score |
| --- | ---: | ---: | ---: | ---: |
| AVX-512 eight-slot merge bucket probe | 4 | 3 | 3 | 4.0 |
| Restore arm64-specific short scanner dispatch | 2 | 4 | 1 | 8.0 |
| Hardware CRC in Rust table/cache lookup | 3 | 4 | 2 | 6.0 |
| Broad algorithm changes before differential coverage | 4 | 1 | 5 | 0.8 |

Each optimization must preserve the lowest-rank, leftmost BPE merge order,
Unigram score and tie behavior, exact normalizer/decoder bytes, and cache collision
checks. It must be compared with both the preceding Rust build and the frozen C
reference using recorded golden token streams. Rejected experiments should stay
documented; faster cells do not excuse silent output differences.
