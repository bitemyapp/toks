# Optimization record

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
