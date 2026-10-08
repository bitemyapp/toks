/* Rust's optional AVX-512 probe shares the established BPE merge order and
 * memory layout. Only the eight-slot bucket comparison changes. */
#define TOKS_K6_AVX512 1
#define toks_k6_bpe_avx2 toks_k6_bpe_avx512
#define toks_k6_merge_avx2 toks_k6_merge_avx512
#include "../../src/asm/x86_64/k6_avx2.S"
