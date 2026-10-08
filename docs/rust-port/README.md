# Rust migration

The Rust library implements the tokenizer core and parallel worker pool. Cargo
compiles Rust and the existing native assembly; it does not compile any C
implementation. The public C ABI and table layouts are retained so the existing
callers, kernel tests, and benchmark driver can exercise either implementation.

The initial core was translated with C2Rust
0.22.1 and then adapted to stable Rust. It remains mostly unsafe Rust. The OS
boundary was written in Rust; alignment, atomics, Darwin worker support, and
scratch-header initialization required manual corrections. The architecture map
is in [rust-architecture.md](../rust-architecture.md).

The [final native encoder matrix](benchmarks.md) covers 84 workloads on each
architecture, with all outputs equal to the frozen C reference. Paired aggregate
fresh-scratch/stream/replay speedups are 1.009×/1.025×/1.039× on arm64 and
1.015×/1.030×/1.042× on x86. These use matched native CPU tuning; portable
defaults are unchanged. Some workloads regress, and the Python adapter remains
slower than C despite its optimizations.

## Build and check

```sh
cargo build --release
python3 tools/ci/fetch_tokenizers.py
python3 tools/rust-port/test.py
TOKS_TIER=scalar python3 tools/rust-port/test.py --out build/rust-tests-scalar
```

The output is `target/release/libtoks.a` and the platform's shared library. Rust
1.94.1 on Linux x86-64 and Rust 1.96.0 on macOS arm64 have been exercised. Windows
support remains open. `make` builds the Rust library; `make test` runs the owned
API and upstream callers, and `make asmcheck` checks the assembly formats and
register contracts. `make reference` builds the C implementation separately;
`make IMPL=c` selects its retained `Makefile.reference`. The Python adapter uses PyO3.

The default `make test` passed all 45 callers and seven owned-API tests on both
hosts (`receipts/default-build-*.json`). `tools/ci/suites.py --rust` requires the
assembly-format and register audits, nonempty owned Rust tests, a complete
successful Rust caller harness, and every upstream suite/critical model. The
original C control-flow audit and source-size budgets remain under `IMPL=c`;
they are not reported as Rust passes. The fork's `Rust port` workflow exercises
native/scalar, guarded pages and the installed Python package on a hosted Linux
runner. Local and workstation results, rather than that shared runner, supply
performance measurements.

The hosted [CJK-core verification run](https://github.com/bitemyapp/toks/actions/runs/37765164320)
passed at `77e5448`: native/scalar checks, both protected-page geometries, source
archive creation, and 331 installed-wheel Python tests with 43 explicit skips.
Its receipt is `receipts/ci-77e5448.json`. The later Python compact-integer change
has separate local/workstation verification described below.

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

| Area | Evidence | Limitations and follow-ups |
| --- | --- | --- |
| ABI, ownership, scratch, allocation | upstream tests; protected pages on both hosts; all 45 callers under AddressSanitizer on x86 | arm64 sanitizer linker compatibility |
| BPE and SentencePiece BPE | kernel twins, exact IDs, short/long/cache/tie cases | broader differential corpus |
| WordPiece and Unigram | breadth, primitives, targets, normalization; 12-model oracle and full Python parity suite on both hosts | broader differential corpus |
| Parallelism | persistent pool, state, stall tests on both hosts | sanitizer coverage |
| Native assembly | NEON, AVX2, scalar; optional AVX-512 bucket probe tested on x86 | AVX-512 performance improvement (currently tied) |
| Python | latest full suite on CPython 3.13, both hosts; x86 API matrix 3.10–3.14; standalone source archive | export visibility, adapter performance |
| Portability | macOS arm64 and Linux x86-64 | Windows and minimum target checks |

After the compact-integer optimization, installed Rust wheels pass 332 tests
with 43 skips on arm64 and 331 tests with 44 skips on x86. The latter initially
lacked the separate C library; its capacity-comparison test then passed after
`make reference`. Skips remain explicit: mostly intentionally refused fixtures,
plus absent generated case sets, a tiktoken-specific target lane and a missing
wrapper alias. The oracle reports retain 105,732 target encodes across 89 models
with zero differences, plus generated-text families and field-by-field surface
checks. See `receipts/python-compact-validation.json` for both full reports.

The latest x86 API/thread/boundary suite passes on CPython 3.10, 3.11, 3.12, 3.13
and 3.14: 39 checks initially pass on each, and the missing C comparison then
passes separately for every version. All 40 checks also pass against the frozen
C wheel on arm64. `receipts/python-compact-versions-x86.json` records versions,
wheel hashes, initial skips and follow-up results. The comparison helper now
searches only the separate C-reference build, so it cannot silently compare the
Rust extension against a Rust shared library from the default build.

`uv build --sdist python` creates a standalone source archive containing the
adapter, Rust core, assembly, layout headers and locked Cargo dependencies.
No C implementation is included. Building its wheel in a fresh temporary
directory outside the repository and running all 40 API/thread/boundary checks
passed on arm64; `receipts/python-compact-validation.json` records artifact hashes.
Both wheel and source builds require Rust and an assembler-capable C driver;
the driver compiles only assembly. Python extension tests are run from installed
wheels; `cargo test --workspace` passes seven owned-API tests and the exhaustive
CJK identity-normalization proof.

The replacement retains the Python vocabulary helper and public signatures,
pickles, subtype factories, scratch leases, cached Python integers and stream
hold ownership. Immutable str/bytes remain borrowed across GIL release. Other
input buffers are snapshotted while attached, and `encode_into` copies from Rust
staging into the writable export while attached; the accepted formats and exact
prefix behavior remain the same. This changes the old mutable-buffer zero-copy
performance contract, so adapter overhead is measured separately from the core.

The x86 sanitizer run instruments both Rust and the C test callers (assembly is
covered by the protected-page and ABI tests). It uses nightly-2026-08-28,
`-Zsanitizer=address -Zexternal-clangrt -Cforce-frame-pointers=yes`, an explicit
Rust target, and `test.py --sanitize address --rust-target x86_64-unknown-linux-gnu`.
The arm64 archive builds, but Apple ld rejects its sanitizer initializer with
`initializer pointer has no target`; a second nightly and a no-LTO build did not
resolve that tooling issue. Those link failures are not counted as passing tests.

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
and SHA-256. The initial matrix is seven tokenizers by English, source
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
