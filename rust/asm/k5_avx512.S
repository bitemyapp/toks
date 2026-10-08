/* Keep the K5 lookup implementation; misses use Rust's AVX-512 K6 path. */
#define toks_k5_encode_avx2 toks_k5_encode_avx512
#define toks_k5_long_avx2 toks_k5_long_avx512
#include "../../src/asm/x86_64/k5_avx2.S"
