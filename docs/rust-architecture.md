# Rust port architecture and coverage

This is the port map for the C implementation at upstream commit
`55a5230b08a75916a2b36f92c320f057376833ea`. It records the existing behavior and
the evidence needed to replace it. An unchecked item below is a verification
requirement, not a claim that the corresponding Rust code is missing.

The library has 35 handwritten C implementation units, 14,170 lines in
`src/core`, `src/platform`, and `src/par`, plus eight generated C data units.
The production Python extension has a separate C adapter in
`python/toks/_toks.c`. Assembly is separate from both: eight `.S` files per ISA
including the ABI self-test, under `src/asm/arm64` and `src/asm/x86_64`.
There was no `AGENTS.md` in this checkout or its parent directories when this
map was prepared.

## Contract and data flow

`include/toks.h` is the public ABI: version 0.4, library release 0.3.2 at the
baseline. It exposes 27 functions. A context is immutable after a successful
load and shareable across threads. A scratch buffer and stream belong to one
thread at a time. The serial encoding and decoding paths allocate no memory.
The companion pool is explicitly outside that restriction and may grow its
scratch and staging buffers.

Loading follows this path:

```text
file/directory or copied bytes
  -> JSON reader, or tiktoken directory reader
  -> configuration and feature validation
  -> table compiler and selected model builder
  -> decoder tables, vocabulary index, capacity bound
  -> CPU feature detection and available-tier selection
  -> immutable context
```

Encoding follows this path:

```text
argument and scratch validation
  -> phase-0 added tokens on original bytes
  -> normalizer on each gap
  -> phase-1 added tokens on normalized bytes
  -> pre-tokenizer pieces
  -> byte BPE / SentencePiece BPE / Unigram / WordPiece
  -> truncation, single-sequence template, padding
  -> exact output prefix plus total count
```

Some steps are fused or folded into lookup tables. This is observable in the
compiled-path information, scratch layout and performance, but must not alter
the token IDs. `toks_pieces` reports offsets into normalized text when a
materializing normalizer changed it. A piece boundary is not automatically a
safe split point. `toks_split_points` returns only certified cuts, with flags
and continuation state accounted for.

## Public ABI coverage

Every row needs Rust symbol exports, argument/error behavior, and tests. C
clients and the existing Python adapter can use a Rust static/shared library
without changing this header.

| Surface | Symbols | Primary implementation | Essential behavior |
|---|---|---|---|
| Loading | `toks_load`, `toks_load_mem_copy`, `toks_unload` | `load.c`, `config.c`, `tiktoken.c` | Directory discovery, copied input ownership, diagnostics, all four model families, failure cleanup |
| Scratch | `toks_scratch_bytes`, `toks_scratch_init` | `api.c`, `core.h` | Any input alignment, size formula, context binding, cache flags, repeat initialization |
| Encoding | `toks_encode`, `toks_pieces` | `api.c`, `wp_api.c`, `uni_api.c` | Six flag bits, all added-token modes, exact short-capacity prefix, total count |
| Template and capacity | `toks_template`, `toks_encode_bound` | `api.c` | Prefix/suffix type IDs, sizing call, atomic short-capacity rejection, saturating bound |
| Splitting | `toks_split_points` | `split.c`, `spm_c.c` | Nearest certified cut, tie rule, bounded search, continuation, truncation/padding restrictions |
| Decoding | `toks_decode`, `toks_token` | `api.c`, `stream.c`, family code | Raw or repaired bytes, special-token skip semantics, invalid IDs, zero-copy token lifetime |
| Vocabulary | `toks_token_to_id`, `toks_id_flags`, `toks_added` | `vocab.c` | Raw-byte keys, duplicate precedence, added-token attributes and ID order |
| Streaming | `toks_stream_init`, `toks_stream_bound`, `toks_stream_push`, `toks_stream_flush`, `toks_stream_hold` | `stream.c` | 64-byte public state, atomic errors, caller-owned hold migration, valid/incomplete/invalid byte-fallback runs |
| Information | `toks_get_info`, `toks_version` | `api.c`, `version.c` | ABI 0.3's 184-byte `toks_info` compatibility, current structure, source hash, feature bits, tier |
| Pool | `toks_par_create`, `toks_par_destroy`, `toks_par_encode`, `toks_par_encode_batch`, `toks_par_get_info` | `par/par.c` | Persistent workers, concurrent-call serialization, exact serial results, per-item errors, pool measurements |

Important limits are 2^29 text bytes, a 256 MiB source file, token strings up
to 65,535 bytes, added strings up to 255 bytes, and IDs below 2^21 - 1.
Unigram has the additional 127-byte piece limit. Unsupported valid features
return `TOKS_E_UNSUPPORTED`; malformed input returns `TOKS_E_FORMAT`. Preserve
that distinction and the remaining stable negative error values.

## State, ownership, and layout

| Type | Responsibility and lifetime |
|---|---|
| `toks_config` | Temporary parsed configuration. Vocabulary and strings reference parse memory; it is discarded after all final tables are constructed. |
| `toks_ctx` | Owns the model/table arenas, decoder table, vocabulary index and optional generic program. Immutable after load; `toks_unload` releases every owned block. |
| `toks_tables` | Common compiled tables and flags read by C twins and assembly. Fixed offsets in `layout.h` are a binary contract. |
| `toks_scratch` | A 128-byte header at the first 64-byte-aligned address within caller memory. Holds binding, region offsets, counters, epoch and long-cache state. |
| `toks_stream` | Eight public `u64` words; private decode state may refer to a caller-owned hold buffer. Copying the stream shares that external buffer. |
| `toks_par` | Owns workers, each worker's scratch, job units, per-part staging and the measured scheduling model. Borrows the context; context must outlive the pool. |

The context owns separate `mem_tables`, `mem_bpe`, `mem_spm`, `mem_wp`,
`mem_uni`, `mem_voc` arena blocks, an optional `mem_gen` allocation, and a
decoder allocation. The loaders free source and parse memory on every path.
A Rust port should establish this ownership before constructing higher-level
safe wrappers; no borrowed string from parse memory may escape a successful
load.

The scratch contains piece ends, a short piece cache, an optional long cache,
an optional segment memo, model work, family-specific extra memory, bounce
output and optional normalization memory. Default BPE budgets are a 2 MiB
piece cache and 4 MiB memo. WordPiece and Unigram have no memo. Enlarged piece
caches use power-of-two 4–128 MiB budgets. The bounce permits an internal
four-ID store while the public output retains its exact-prefix contract.

`layout.h` asserts the sizes and offsets of `toks_tables`, added entries and
K1/K3/K5/K6/K7 argument blocks. `spm.h`, `wp.h`, `unigram.h` and `bpe.h`
contain additional shared structures. Mirror these with `#[repr(C)]`, exact
integer widths, and Rust size/alignment/offset assertions before retaining
assembly or linking internal C tests. For the initial port, retaining the
internal context and scratch layouts also permits the existing benchmark to
read its counters unchanged.

## Implementation coverage checklist

This list accounts for all handwritten library C files, including the scalar
implementations used when SIMD is unavailable or deliberately disabled.

- [ ] `alloc.c`: SHA-256, the CRC table and byte-level alphabet; identical source hashes, hash primitives and alphabet mapping.
- [ ] `json.c`: bounded RFC 8259 parsing, UTF-8 and escape checks, retained number lexemes and object-member order.
- [ ] `config.c`: supported feature shapes, duplicate-key policy, algorithms, added tokens, post-processors, truncation and padding.
- [ ] `tiktoken.c`: rank files, companion configuration, Qwen and Kimi directory conventions, wrapper cuts and specials.
- [ ] `compile.c`: template certification, added-token indices, vocabulary bytes, common tables and capacity-bound terms.
- [ ] `classes.c`: Unicode scanner classification and certified shortcut flags.
- [ ] `bpe_build.c`: merge rank semantics, byte and pair tables, static words, premerge tables and vocabulary probes.
- [ ] `k1_c.c`, `segment.c`: scalar added-token finding and phase/strip/single-word precedence.
- [ ] `k3_c.c`, `k3_o200k_c.c`, `k3_dsv3_c.c`: every compiled scalar scanner template and parameter combination.
- [ ] `k5_c.c`, `k5_long.c`: static/short/long cache lookup, exact byte keys, epochs and SIMD-callable long helpers.
- [ ] `k6_c.c`: byte-level BPE merge ordering, short scan and long heap paths.
- [ ] `norm.c`: Unicode normalization and BERT steps, unchanged-input path and normalization-size accounting.
- [ ] `precompiled.c`: canonical base64, SentencePiece charsmap validation, compilation and lookup.
- [ ] `gen.c`: exact generic pre-tokenizer compiler and interpreter, including supported regex forms and bounded stacks.
- [ ] `spm_build.c`, `spm_c.c`: SentencePiece BPE normalizer folding, K7 scalar scanner, model, cache, decoder and cuts.
- [ ] `unigram.c`, `uni_api.c`: exact decimal score parsing, double-array trie, Viterbi tie decisions, charsmap/grapheme behavior, normalizer and decoder chains.
- [ ] `wp.c`, `wp_scan.c`, `wp_api.c`: WordPiece tables/tries, BERT normalization, greedy subwords, added-token phases, truncation/padding and cleanup decoder.
- [ ] `api.c`: validation, scratch, BPE driver, memo, wrapper cuts, public encode/decode/information surface.
- [ ] `stream.c`: fast decoder table and all incremental decode state transitions and recovery paths.
- [ ] `vocab.c`: raw-byte index, added-content ordering, duplicate winners, special/byte flags and added-token enumeration.
- [ ] `split.c`: certified cut planning and template helpers consumed by the pool.
- [ ] `load.c`, `version.c`: lifetime management, path dispatch, diagnostics, CPU/tier selection and release symbol.
- [ ] `platform/cpu.c`: runtime feature bits and OS-enabled vector register state.
- [ ] `platform/file.c`: bounded whole-file reads, directory search, environment values and nonregular-file rejection.
- [ ] `platform/mem.c`: fallible allocations, 2 MiB-aligned arenas, page-size-aware mapping/freeing and huge-page hints.
- [ ] `par/par.c`: worker lifecycle, atomics, sleeping/spinning, affinity/QoS, planning, staging, exact serial fallback and scheduling measurements.
- [ ] Header inline logic in `core.h`, `layout.h`, `kernels.h`, `bpe.h`, `da.h`, `norm.h`, `spm.h`, `unigram.h`, `wp.h`, `json.h`, `config.h`, `compile.h`, `split.h`, `tiktoken.h`, `classes.h`, `cpu.h`: no runtime behavior left behind by translating `.c` files alone.
- [ ] Generated data in `bert_tables.c`, `dict.c`, `grapheme17.c`, `han_ranges.c`, `norm_nfc.c`, `pow10.c`, `rx_word.c`, `ucd_flags.c`: byte-identical constants emitted as Rust data; generators updated or a reproducible conversion supplied.
- [ ] Production Python adapter in `python/toks/_toks.c`: account for its status separately. Linking the existing adapter to Rust replaces the library runtime, but is not itself a rewrite of this remaining production C file.

Generated Unicode versions are part of behavior. Do not substitute the
current Unicode tables of a convenient dependency without proving the same
results. Unigram scores intentionally match `serde_json 1.0.151` without its
`float_roundtrip` feature; Rust's ordinary float parser is not an assumed
replacement for that numerical contract.

## Rust and platform hazards

1. **Caller alignment.** The public header permits unaligned buffers, including
   `uint32_t` IDs and `uint64_t` cut offsets. Forming Rust references or slices
   from those pointers may already violate alignment before a load occurs.
   Use byte pointers and unaligned loads/stores where the C code uses `memcpy`.
   Null pointers with zero lengths need separate handling: a zero-length Rust
   slice still requires a non-null aligned pointer.
2. **Initialization.** `toks_scratch_init` inspects the old header to advance
   cache epochs, even on first use of caller memory. The current C source has
   an MSan unpoison hook; older prose in `docs/hardening.md` predates it. Rust
   reading an uninitialized integer is undefined behavior. This path needs an
   explicit sound design rather than a mechanical dereference or an MSan-only
   suppression. Cache validity and repeat-init speed are separate checks.
3. **Aliasing.** C is built with `-fno-strict-aliasing`; Rust mutable references
   promise exclusivity. Shared raw context/scratch views and assembly calls
   need a reviewed aliasing boundary. Do not blanket-convert every raw pointer
   to `&mut`.
4. **Integer and float semantics.** C uses `-fwrapv`, unsigned wraparound,
   deliberate casts, sentinel bit patterns and explicit saturation. Preserve
   those operations with wrapping/checked arithmetic as appropriate; avoid
   debug-only overflow failures and unwinding through the C ABI. Preserve
   score operation order and tie behavior; avoid fast-math reassociation.
5. **Vector extents.** All declared table padding is zero at the baseline.
   Text and output may end immediately before an inaccessible page. Retained
   or new SIMD code must respect exact extents and declared alignment; heap
   slack is not padding.
6. **Features and tiers.** NEON requires CRC32 as well as baseline vector
   support. AVX2 requires BMI1, BMI2, LZCNT, POPCNT and SSE4.2 CRC. x86 detection
   also checks XCR0. AVX-512 enum/detection/plumbing exists, but the baseline
   ships no AVX-512 kernels: forcing it returns `TOKS_E_TIER` and capable
   machines normally use AVX2. A CPU supporting AVX-512 is not proof a benchmark
   executed AVX-512 code.
7. **OS behavior.** Linux arena alignment/huge-page first-touch policy matters
   to speed. macOS commonly has 16 KiB pages, so 4 KiB assumptions break
   unmapping. Windows reservations need their original base released. The
   file layer avoids newer glibc `stat` symbols to preserve wheel compatibility.
8. **Pool synchronization.** Workers sleep on futex, Darwin ulock or Windows
   WaitOnAddress words. The original uses aligned C atomics and explicit job
   admission/completion rules. Mirror memory ordering and layout deliberately;
   Rust wrappers must not claim exclusive access to memory concurrent workers
   access through raw pointers.
9. **No hidden runtime work.** A Rust `staticlib` is not proof that the core is
   allocation-free, lock-free or panic-free. Inspect reachable code and
   dependencies after linking, and separate load-time/pool functionality from
   serial calls. Standard library CPU feature caches or lazy initialization
   must not silently move OS calls into encode.

The assembly long-piece cache calls back into `toks_k5_long_neon` or
`toks_k5_long_avx2`. Preserve their unmangled C calling convention. Mach-O
underscore decoration, ELF visibility/BTI/CET and Windows unwind/register
rules are handled by the assembly macro layers; retaining those assembly
sources is simpler than re-creating their ABI machinery in the initial port.

## Existing evidence and test linkage

`make test` builds every `tests/c/*.c` program plus tier-specific BPE tests.
It also assembles the sources for Mach-O, ELF and COFF for both ISAs, audits
callee-saved registers and object-code constraints, and checks size budgets.
Test logs preserve each program's exit code. The suite has several different
linkage requirements:

| Test kind | Examples | Rust integration |
|---|---|---|
| Public ABI | `test_misalign`, `test_par`, `test_primitives`, `test_version`, `test_split`, `tests/driver/toks_driver.c` | Compile unchanged C test clients against Rust static/shared output. |
| Internal symbols/layout | `test_api`, `test_compile`, `test_bpe`, `test_k*`, `test_spm`, `test_wp`, `test_bound`, `test_vocab`, `test_targets` | Keep initial `repr(C)` layouts and internal test exports, or replace with equivalently scoped Rust tests. |
| Source inclusion | `test_cuts`, `test_memo_hash`, `test_state_hash` | These include implementation `.c` files. A plain relink would still test C for those paths. Port the checks or provide explicit Rust test configurations. |
| Allocator substitution | `test_alloc` | The C executable defines platform allocation symbols to inject failures. Use an explicit Rust test allocator seam; a monolithic Rust archive can otherwise create duplicate symbols. |
| Table/scratch geometry | `test_guard`, `make test-guard` | Port table registration, sealing, scratch-region remapping and both boundary geometries; linking guard.c alone does not instrument Rust internals. |
| Object audit and size | `tests/abi/cf_audit.py`, `tools/size.sh` | Adapt classification to Rust objects and runtime symbols while retaining the underlying constraints. The existing script only knows the C build layout. |

The source-inclusion collision variants are substantive tests: hashes are
forced to collide, and exact key comparison must still protect all answers.
They must not become silently omitted tests or tests of the old implementation.

The fixture tree includes tokenizer shapes, malformed/refused configurations,
vocabulary precedence, Unicode, normalizers, primitives, all model algorithms,
streaming and synthetic merge cases. Real model files live outside the tree.
`python3 tools/ci/fetch_tokenizers.py` downloads and verifies the pinned files;
`--check` verifies the cache without downloading. The defaults are
`~/.cache/toks/tokenizers` and `~/.cache/toks/kimik3`, overridden by
`TOKS_TOKENIZER_CACHE` and `TOKS_KIMI_DIR`. Missing files can make `make test`
pass with `SKIP` lines. `tools/ci/suites.py` checks counts and critical-target
skips, so raw exit zero is insufficient evidence of model coverage.

The unchanged driver in `tests/driver/toks_driver.c` uses only the public ABI.
Build it twice, once with the frozen C reference and once with Rust. The Python
runner `tests/parity/run.py` compares encode, pieces, decode and stream behavior
against Hugging Face `tokenizers==0.23.2`; `run_kimi.py` and the tiktoken oracle
cover wrapper-specific Kimi behavior. `vocab_sweep.py` covers vocabulary lookup.
WordPiece, SentencePiece and Unigram have additional pinned differential
suites under `tests/wordpiece`, `tests/spm`, and `tests/unigram`.

`tests/hardening/idsdiff.c` is a useful C-versus-Rust bridge: it produces a
deterministic digest of IDs and piece ends for arbitrary byte texts, all three
added-token modes, and post-processing on/off. It is not a complete public API
test: streaming, decoding, loader errors, capacities, pool behavior and the
newer truncation/padding flags still need their own coverage.

## Benchmarks and recommended port order

The initial replacement should preserve algorithms and layouts, expose the C
ABI, retain the existing assembly, and compile all handwritten runtime logic
and generated constants as Rust. A transliteration is a useful starting point,
but C preprocessing, inline helpers, casts, initialization and build-time
feature macros all need review. Inventory exported and undefined symbols so a
successful build cannot conceal retained C runtime objects.

Bring up the ABI and primitive/layout checks first, then loaders and synthetic
fixtures, all four model paths, stream/vocab/split semantics, and the pool.
Keep a frozen optimized C reference built with the original flags. Use the
same external C test/benchmark clients for each implementation where possible.
Replace C source-inclusion tests and geometry instrumentation before declaring
the corresponding coverage complete. Retarget Python's `setup.py` away from
its current hard-coded C Makefile build once the Rust library passes the ABI
tests, then run the existing Python surface, parity and thread tests.

`tools/bench/e2e.c` is the main single-thread workload. It measures short chunks
and whole inputs, hashes the exact ID stream, reports scratch counters, and
distinguishes cold, pass, warm, lang, warmo and coldo states. Cold clears semantic
caches before each call outside the timer; warm is an exact replay and may be
answered entirely by the memo. Record cache/memo budgets and CPU-cache state
with every comparison. Retaining the initial scratch layout allows this same
client to inspect counters in either implementation.

`tools/bench/e2e_ab.sh` supplies an ABBA timing pattern but compares two tiers
of one binary. C-versus-Rust measurements require separate executables or an
adapted runner. Its default sample is also narrower than the release table.
The main corpus selectors and pins are in `tools/bench/common.sh`,
`tools/bench/corpus.sha256` and `tools/bench/e2e.sh`. Model-specific stage tools,
`tests/k3/bench_k3.c`, cache sweeps, `tests/par/bench_par.c` and
`tests/stream/bench_stream.c` help locate the remaining costs.

For the requested comparison, record local arm64 and `wx-workstation` x86
machine/compiler/CPU details, pin x86 timing to an appropriate core, alternate
C and Rust runs, and prevent parallel experiments from contending with measured
runs. Compare scalar against scalar and native against native; on AVX-512
hardware explicitly record whether AVX2 or a new AVX-512 kernel executed.
Validate exactness before timing, report per-cell ratios and variability, and
include all four algorithm families and distinct cache states. Faster memo
replay alone does not establish faster tokenization.

The first optimization targets should come from those measurements. Likely
places to examine are Rust inlining across the driver/scalar helpers, bounds
and aliasing code generation, scalar CRC32C hashing where hardware dispatch is
valid, table/cache layout, and the long-piece BPE path. These are hypotheses,
not measured speedups. Keep a behavior proof or differential check attached to
each change and remeasure the full affected workload set.

## Completion evidence

- [ ] All 27 public symbols have matching ABI and behavior, including legacy information structure sizes.
- [ ] Every implementation group above has Rust coverage; production builds do not compile or link C runtime logic or generated C data.
- [ ] Remaining C files are explicitly classified as reference/test clients or separately accounted-for adapters.
- [ ] Native and scalar tests pass on local arm64 and x86, with real model pins present and critical skips ruled out.
- [ ] All four algorithms, Unicode normalization, generic scanners, tiktoken wrappers, stream recovery, vocabulary precedence, capacities, continuation and pool results have differential evidence.
- [ ] Collision, allocation-failure and guard tests execute the Rust implementation, with equivalent fault detection and count coverage.
- [ ] SIMD calling conventions and exact memory extents remain tested on supported object formats.
- [ ] Python packaging/tests use the Rust library and retain the documented behavior.
- [ ] Paired arm64 and x86 timing receipts show the speed result against optimized C, with every relevant regression visible and cache budgets matched.
- [ ] AVX-512-capable-host results identify the actual executed tier; any new AVX-512 implementation has its own dispatch, exactness and speed evidence.

This document describes requirements and a verification route. It does not
claim the port or any performance target has passed them.
