# Optimization record

## Follow-up: emit the winning Unigram path directly

A fresh arm64 T5 multilingual profile put about 60% of samples in `uni_piece`.
The completed dynamic program already stores a predecessor length at each
reachable end. Previously emission marked the winning ends, then inspected
every byte to find those marks. Reverse just the winning predecessor chain in
place, saving each old predecessor before replacing it with a forward length.
Emission can then jump directly between the chosen boundaries. Opportunity
score: impact 3 × confidence 4 / effort 2 = 6.

Let the winning boundaries be `0 = v0 < v1 < ... < vm = L`. Before reversal,
`delta[vi] = vi - v(i-1)`, with each length in `1..=127`. Walking backward saves
the predecessor's old length before writing `delta[v(i-1)] = vi - v(i-1)`.
Thus the subsequent forward walk visits precisely `v1, ..., vm`. Empty input
returns before either walk. Every read/write remains within the initialized
`L + 1` bytes, and callers do not inspect these lengths after emission. The
next invocation clears the buffer again.

- Ordering preserved: the same chosen ends are emitted in ascending order;
  unknown spans are coalesced by the unchanged emission code.
- Tie-breaking unchanged: DP traversal and strict-greater comparisons are
  untouched. Short pieces read the same saved IDs; long pieces look up the same
  spans.
- Floating-point: identical score additions in the original order.
- RNG seeds: no production randomness; benchmark resampling seed is unchanged.
- Golden outputs: every measured full token stream matches the frozen C
  goldens; the 12-model pinned oracle passes against separate C/Rust archives.

The added owned-API test covers 504 input configurations and 1,008 full/short
output calls, including the 512-byte saved-ID boundary, multibyte and invalid
UTF-8, unknowns, virtual prefixes and continuation. The expected results also
pass against the frozen C wheel. Debug/release tests and targeted native and
both guard-page placements pass on both architectures. The full measurements,
scope and receipts are in [unigram-path.md](unigram-path.md). Reverting the
commit that introduces this walk restores the old marking/scan algorithm.

The first candidate removed two saved-ID bounds checks (score 3 × 5 / 1 = 15).
It tied on arm64 and regressed slightly on x86, so it was rejected. Production
keeps those checks. Its patch and all paired observations are retained with
the follow-up receipts; it is not part of the measured winning candidate.

## 1. Initialize Unigram dynamic-programming entries when reached

The initial translation cleared a 128-double score ring and a 512-ID array on
every piece. An arm64 profile of T5 English recorded 263 samples at the ID-array
clear and 50 at the score-array clear, out of roughly 4,200 samples. C does not
clear these arrays. Opportunity score: impact 3 × confidence 5 / effort 1 = 15.

Use `MaybeUninit` elements and initialize each reached end, retaining the delta
array's initialization. `best[0]` starts at zero. Every atom has a single-piece
or unknown edge, so the next atom's score is initialized. Comparisons read a score
only after its delta is nonzero. Edges span at most 127 bytes, so no live score is
overwritten in the 128-slot ring. Backtracking reads an ID only for a reached end
and only when the complete piece is shorter than the 512-ID array.

No traversal order, arithmetic, score comparison, tie rule, or randomness
changes. In particular, scores remain f64 additions in the original order, and
ties retain the first candidate through strict `>` comparisons.

Validation: targeted bounds, breadth, primitives, target-model, misalignment and
end-to-end tests; full C-generated token-stream hashes for four public corpora;
12 pinned Unigram model oracle streams (2,500 generated texts per model plus
decode cases), checked against both C and Rust. ALBERT/XLNet are excluded from
this particular generator because its Python model lacks their normalizers;
they remain in the C test harness. A generator correction was needed for MiniLM:
continuation chunks must disable document truncation/padding, as `toks.h` states.
Before that correction, C and Rust emitted identical logs with 264 mismatches;
afterwards both pass. The LLM-jp continuation oracle also compared a regex
containing a newline escape with a Python string containing an actual newline.
Using the correct raw string removes that start prefix. Before this correction
both C and Rust reported the same 2,500 continuation mismatches for each of
LLM-jp 3 and 4; all 12 corrected model streams now pass on both architectures.

The five-block paired arm64 comparison with the preceding Rust executable gives
a 1.082× geometric mean cold speedup across the four corpora (block estimator),
with individual estimates 1.056–1.107×. English's interval includes 1.0. Against
C, the geometric mean of median ratios is 0.922× on arm64 and 0.970× on x86 for
cold inputs: this change recovers a translation regression; it does not establish
an overall win over C. Raw observations, hashes and host details are retained in
`receipts/uni-lazy-*.json`.

## 2. AVX-512 eight-slot bucket probe (opt-in experiment)

`cargo build --release --features avx512` compiles a K5/K6 path whose merge probe
compares all eight keys with a 512-bit load and mask comparison. The default
AVX2 source remains unchanged after preprocessing. The wrapper shares all merge,
heap, tie, and long-piece code with that kernel. Runtime feature checks select
it only with the required CPU/OS features, and an explicit AVX2 selection still
uses AVX2. Other kernels fall back to their existing AVX2 implementations.

Preservation argument: the new load covers precisely the same 64-byte bucket as
the two original 32-byte loads. Mask bit j represents the same matching slot j;
the same trailing-zero selection, empty-slot termination, bounded probing,
priority extraction, and lowest-rank/leftmost merge order follow. No arithmetic,
floating-point, RNG, table layout, or caller allocation changes.

The Threadripper 9985WX passed all 46 native test executables, including the
added direct AVX-512 BPE twin, ABI checks, tier selection, collision tests and
scalar comparisons. Disassembly confirms `vpandnq zmm` and `vpcmpeqq k1` in
`toks_k6_bpe_avx512`. The paired matrix checks full token streams against frozen
C goldens for GPT-2, Llama 3, o200k and Qwen 3.8 on all four corpora.

This first probe experiment does **not** establish a speedup: across those 16
cells, geometric means of the median ratios are 0.996× cold, 1.002× pass-same,
and 0.993× warm relative to C. It remains off by default. The records in
`receipts/avx512-vs-c-x86*.json` include every cell and the paired block intervals.

## 3. Leave SentencePiece assembly work uninitialized until used

The arm64 Gemma 4 CJK profile attributed approximately 15% of samples to clearing
the 4,352-byte assembly work buffer at each model miss, plus further samples to
clearing the symbol array. Both clears were introduced by translation. Use
64-aligned `MaybeUninit` storage, passing raw pointers to the existing writers.
Score: impact 4 × confidence 5 / effort 1 = 20.

`symbols` initializes exactly its returned count; the K6 assembly initializes its
own work region. The zero-symbol branch now returns before reading any symbol,
while the one-symbol branch reads the initialized first entry. The merge order,
rank ties, output bytes, and cache behavior are unchanged; no floating-point or
RNG operations are involved. Targeted SPM, BPE, K5, misalignment, bounds and model
tests pass on both architectures, as do every measured full-token hash.

The four-corpus Gemma 4 measurements versus C give geometric means of median
ratios of 0.960×/1.147×/1.168× on arm64 and 0.822×/0.912×/0.924× on x86 for
cold/pass-same/warm. This recovers some of the initial regression but does not
make all SentencePiece workloads faster than C. The remaining x86 CJK regression
is particularly significant and remains an optimization target.

## 4. Restore queued table prefetches

C2Rust omitted the prefetch builtins in the WordPiece, Unigram and SentencePiece
lookup queues. Restore read-prefetch hints at exactly the original addresses,
using the target's native instruction. Each address is inside an existing table
or cache bucket; the existing nonnull guards remain. Prefetch does not change
data, ordering, ties, floating-point or RNG behavior. Score: impact 2 × confidence
5 / effort 1 = 10. The WordPiece code-corpus profile attributes about half its
samples to encoding/lookup, making this a relevant region to restore.

The six targeted test executables pass on both architectures. Full-stream hashes
match C for the three affected tokenizer families on all four corpora. Across
those 12 cells the cold/pass-same/warm geometric means versus C are
0.983×/1.074×/1.088× on arm64 and 0.937×/0.974×/0.985× on x86. The x86
SentencePiece gap remains; restoring these hints alone does not explain it.

## 5. Select the native SentencePiece scanner on x86

The translation was generated with both native-kernel macro sets defined. The
original `#if NEON / #elif AVX2` in K7 dispatch therefore selected only NEON's
tier predicate. Although the function symbol was correctly linked to AVX2 on
x86, the tier check prevented calls to it. Every x86 SPM scan used scalar Rust.
The corrected predicate selects tier 2 on arm64 and tiers 3/4 on x86; scalar
still bypasses native scanning. This is a dispatch repair, with no algorithm or
memory-layout change. Score: impact 4 × confidence 5 / effort 1 = 20.

The SPM, K7, tier, breadth, target and bounds tests pass on x86. Across the four
Gemma 4 corpora, C-golden token streams match. Cold/pass-same/warm geometric means
against C improve to 0.962×/1.094×/1.110×. The CJK cold cell improves from 0.738×
to 0.976×; the native path closes the main unexplained regression.

## 6. Restore architecture-specific short scanner thresholds

The translated K3 glue retained x86 crossover thresholds on arm64. Restore the
original arm64 thresholds and select the scanner family with a const generic,
removing function-address comparisons (Rust may merge or duplicate functions).
Each branch still uses the same exact scalar/native scanner twins, keeps cut and
capacity behavior, and changes only where the crossover occurs. No floating-
point or RNG operations are involved. Score: impact 2 × confidence 5 / effort 1
= 10. Native K3, tier, dispatch, model-target and misalignment tests cover this
choice.

The 16-cell arm64 byte-BPE matrix matches every C token-stream golden. Its
cold/pass-same/warm geometric means of median ratios against C are
1.027×/0.983×/0.986×. These small, mixed changes require confirmation in the
final matrix; they are not an all-workload win.
# Python integer-input conversion

The first installed Rust wheel preserved results but regressed on adapter
overhead. An arm64 `sample` capture of repeated GPT-2 decode attributed about
63% of samples to ID conversion (Python tuple copies, per-element extraction,
and vector growth), 14% to a redundant UTF-8 validation pass, and 12% to the
native decoder. Input conversion scored impact 5, confidence 5, effort 2
(12.5), so it was addressed first.

Exact list/tuple inputs containing only exact built-in integers now use the
CPython integer API directly under the GIL and reserve their output once. This
path cannot invoke a user conversion callback. Encountering a bool, subclass
or other object takes the existing full tuple snapshot before any `__index__`
call; negative and oversized integers still raise OverflowError. Mutable
buffers keep their snapshot policy. The reentrant mutation test passes against
both implementations, and the updated Rust wheel passes all 38 API, thread,
and boundary tests.

The initial and updated paired observations are in
`receipts/python-baseline-arm64.json` and `receipts/python-fastids-arm64.json`.
These are exploratory three-block warm-call measurements; the adapter still
has regressions against C. Every measured input retains its complete ID and
decoded-byte digest.

## Python Unicode result construction

The same decode profile attributed 14% of samples to `String::from_utf8_lossy`
before `PyString::new` scanned the bytes again. Score: impact 3 × confidence 5 /
effort 1 = 15. Pass the core's repaired UTF-8 directly to
`PyUnicode_DecodeUTF8(..., "strict")`, as the C adapter does, for ordinary and
streaming decode. Raw decoding is unchanged. The core still performs every
replacement, ordering and token lookup; no tie, floating-point or RNG behavior
changes. Invalid UTF-8 returned by a defective core now raises the same Python
exception as the C adapter instead of being repaired a second time.

All 38 API, threading and boundary checks pass. All benchmark token/decoded
byte digests match C. In the exploratory three-block arm64 measurements, long
GPT-2 decode improves from 0.441× to 0.616× C's speed and long T5 decode from
0.823× to 0.903×; short decode is essentially unchanged. The adapter remains
slower than C. Raw observations are in `receipts/python-utf8-arm64.json`.

## Python ID-list result construction

The encode profile attributed 544 of 3,594 samples to `int_list`, including
allocation and destruction of a temporary Rust reference vector. Score:
impact 4 × confidence 5 / effort 2 = 10. Allocate the final Python list before
locking the integer cache, then fill its private slots directly with owned
integer references. Allocating first allows cyclic GC to reenter without
deadlocking; integer construction and slot assignment cannot call user code.
The list remains private until complete, and its uninitialized slots are NULL,
which CPython safely handles on failure. Token order, cached object identity,
numeric values, floating-point and RNG behavior are unchanged.

All 38 API/thread/boundary tests pass and every benchmark digest matches C.
The three-block arm64 estimate for long GPT-2 encode improves from 0.530× to
0.944× C's speed, and batch4 from 0.545× to 0.954×. Short encode improves from
0.607× to 0.700×. The corresponding T5 estimates are 0.983×, 0.988× and 0.718×.
These exploratory results are in `receipts/python-intlist-arm64.json`.

## Python FASTCALL entry points

The encode profile shows CPython constructing a positional tuple for
`method_vectorcall_VARARGS_KEYWORDS`, followed by the adapter allocating its
argument vector. Score: impact 3 × confidence 5 / effort 2 = 7.5. Explicit PyO3
signatures now generate FASTCALL parsing for encode, decode and their batch/
stream variants. A custom argument extractor retains explicit `None` instead
of conflating it with an omitted option. Keyword-only and positional limits,
truth callbacks, tiktoken/Hugging Face interface conflicts, token ordering,
ties, floating-point and RNG behavior remain unchanged. Argument-error wording
is now supplied by PyO3; exception types remain the same.

All 39 API/thread/boundary checks pass, including a new omission/None and
reentrant truth-callback test which also passes against the C wheel. All timed
digests match C. The three-block arm64 short-call estimates improve from
0.700×/0.602×/0.530× to 0.805×/0.753×/0.627× C speed for GPT-2 encode,
encode_into and decode; T5 improves from 0.718×/0.662×/0.710× to
0.840×/0.814×/0.825×. Longer-call results remain mixed and are fully retained
in `receipts/python-fastcall-arm64.json`.

## Stronger C baseline and rejected SIMD case-folding experiment

The 28-cell core matrix was repeated for five ABBA/BAAB blocks with nine inner
repetitions. C now uses native CPU tuning and thin LTO in addition to its
original strict arithmetic/aliasing flags; Rust uses its portable release
profile with thin LTO. Every stream matches the original frozen C goldens.
The descriptive cold/pass-same/warm geometric means of median ratios are
0.991×/1.018×/1.023× on Apple M5 Max and 0.994×/1.016×/1.022× on Threadripper
PRO 9985WX. This establishes neither a fresh-input win nor a general win across
all workloads. The full observations and build flags are in
`receipts/strong-c-{arm64,x86}.json`.

The WordPiece profile places 1,827 of 4,236 samples in encoding/lookup, including
key preparation. An experiment replaced two scalar ASCII folds of a 16-byte
key with baseline NEON/SSE2 comparisons. Score: impact 3 × confidence 4 /
effort 2 = 6. It retained every non-A–Z byte, zero padding, lookup order, hash,
ties and output, with no floating-point or RNG changes. All byte values at all
16 positions, randomized keys, and the WordPiece/target/breadth suites passed
on both hosts. Two mistyped extra suite selectors silently selected nothing;
the harness now rejects unknown or empty selections. All four corpus token
streams matched C.

The experiment is **rejected**: arm64's median-based ratios were only
1.006×/1.005×/1.007× the preceding Rust build, while x86 regressed to
0.984×/0.983×/0.982×. Most arm64 per-cell paired-block intervals included 1;
CJK improved about 1%. The extra architecture-specific implementation does not
justify this mixed result. `experiments/ascii-fold.patch` preserves the candidate,
and `receipts/ascii-rejected-*.json` preserves all measurements. Production code
retains scalar folding.

## Native CPU tuning

Both cores were then tuned for the host CPU: Rust adds
`RUSTFLAGS='-C target-cpu=native'` to the same release profile. All 45 callers and
seven owned-API tests passed on each host. Every full-matrix token-stream digest
matches the original C golden. The five-block cold/pass-same/warm descriptive
geometric means are 0.996×/1.023×/1.027× C on arm64 and
0.999×/1.029×/1.032× on x86. Native tuning helps modestly, but fresh-input
performance is still essentially tied. These host-specific binaries require
matching CPU capabilities; the default library and wheel remain portable.
All measurements and build flags are in `receipts/native-cpu-*.json`.

## Skip identity normalization for CJK WordPiece characters

A fresh CJK profile placed 599 of 3,872 samples (15.5%) inside `toks_norm_char`
and its decomposition/mapping helpers. The scanner has already obtained the
character's BERT class. When that class has no decomposition, lowercase mapping,
nonzero combining class or Mn removal, normalization is the identity for every
supported BERT flag combination. Score: impact 4 × confidence 5 / effort 2 = 10.
Reuse that classification to emit the unchanged character, retaining the
original normalizer for all other CJK characters.

An exhaustive Rust test enumerates every Unicode scalar, selects all 80,262
CJK characters satisfying the predicate, and checks the unchanged normalizer
under all 16 BERT flag combinations: 1,284,192 identity checks. It passes on
both architectures. The eight WordPiece/e2e, normalizer/driver, misalignment,
target, breadth and bound suites also pass on both hosts. The split, materialized
buffer, maximum-character and exact-prefix paths retain their original order;
only a pure call is skipped. There are no score, tie, floating-point or RNG
changes. Every measured token stream matches the frozen C golden.

Against the preceding native-tuned Rust build, CJK cold/pass-same/warm improve
1.207×/1.259×/1.263× on arm64 and 1.184×/1.235×/1.235× on x86. A separate paired
comparison against native-tuned thin-LTO C measures CJK at
1.092×/1.152×/1.161× on arm64 and 1.059×/1.099×/1.105× on x86. Across all four
WordPiece corpora, the C-relative geometric means are 1.037×/1.054×/1.056× and
1.030×/1.048×/1.054×, respectively. English/code/multilingual observations,
including small losses and variance, remain in `receipts/cjk-identity-*.json`.
The profile captures are `receipts/wp-cjk-*-sample.txt`. These results justify
keeping the change. The [complete native matrix](benchmarks.md) and subsequent
Python verification now cover the final implementation.

## Bind CPython's compact-integer conversion in Rust

A fresh profile of the FASTCALL adapter captured 3,028 samples. Of those, 925
(30.5%) were in `PyLong_AsUnsignedLong` or its dynamic-link stub, even after the
earlier tuple/extraction optimization. Score: impact 4 × confidence 5 / effort
2 = 10. The C adapter already uses CPython's inline compact-integer API. The
Rust adapter now binds the same known representation for exact nonnegative
integers on CPython 3.12–3.14 with the GIL and 30-bit/u32 digits. The isolated
binding excludes other implementations, limited/free-threaded/trace-reference
ABIs, future versions and other digit formats. `PyLong_GetInfo` verifies the
digit format; zero's unspecified digit is never read. All other cases retain
the public conversion function.

The tag determines sign and digit count. Tag 1 (ignoring the 3.14 small-integer
flag) means zero; tag 8 means one positive digit, which necessarily fits u32.
Raw field reads avoid forming a reference over trailing struct padding. The
caller holds the GIL and checks the exact integer type before these reads; no
Python callbacks intervene. List order, snapshots before user callbacks,
negative/overflow errors, core ID validation, token bytes, ties and floating-
point/RNG behavior remain unchanged. The layout comes from CPython's
[`longintrepr.h`](https://github.com/python/cpython/blob/3.14/Include/cpython/longintrepr.h).

The entire GPT-2 vocabulary is checked through both exact list/tuple inputs and
the independent integer-buffer path, together with compact/multi-digit bounds,
negative/oversized values, booleans and integer subclasses. These checks also
pass against the original C wheel. All 40 API/thread/boundary checks pass on
arm64. On x86, CPython 3.10, 3.11, 3.12, 3.13 and 3.14 each pass 39 checks and
initially skip the missing C shared-library comparison; that comparison then
passes separately for every version after building `make reference`. The full
arm64 installed-wheel suite passes 332 tests with 43 explicit skips. A wheel
built from the standalone source archive outside the checkout passes all 40
API/thread/boundary checks; the archive contains no C implementation.

Five preceding-adapter and seven candidate ABBA/BAAB blocks retain every ID and
decoded-byte digest. Long GPT-2 decode improves from 0.648× to 0.825× C speed
(about 27% relative); T5 improves from 0.916× to 0.972×. Short decode improves
from 0.594× to 0.631× and from 0.800× to 0.834×. These measurements justify
keeping the change, but the Python adapter still regresses against C. Unchanged
encode/batch controls include considerable variance and are retained in full.
The preceding wheel predates the CJK-only WordPiece change; neither timed model
uses WordPiece. Receipts are `python-compact-*.json` and
`python-compact-profile-arm64.txt`.

The full x86 suite subsequently passed 331 tests with 44 explicit skips. Its
initially missing C-capacity comparison passed separately once the C reference
was built. Both full-suite oracle reports retain 105,732 target encodes across
89 models with no differences, and are saved in
`receipts/python-compact-validation.json`.
