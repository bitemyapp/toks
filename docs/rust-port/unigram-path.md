# Unigram path-emission follow-up

The Unigram encoder now reverses its winning predecessor chain into forward
lengths and visits only the selected token boundaries during emission. It no
longer scans every input byte after scoring. Scoring, tie rules, token IDs and
unknown-span handling are unchanged. The [preservation argument](optimizations.md)
also covers scratch reuse and the saved-ID buffer boundary.

## Compared with the preceding Rust implementation

The baseline is the native core from `77e5448`, unchanged through `9d5f1bf`.
Each host ran T5 on English, source code, multilingual and CJK corpora at
4096-byte chunks. Five alternating ABBA/BAAB blocks contain two baseline and
two candidate processes each, with nine inner repetitions per cache state.
Linux is pinned to CPU 8; macOS uses its normal scheduler. Both versions use
`target-cpu=native`, optimization level 3, thin LTO and one codegen unit.

Ratios above 1 favor the new implementation. All aggregates below are geometric
means of paired block median-time ratios, equally weighted across cells. Inner
repetitions are not independent observations. Per-cell intervals bootstrap
whole blocks, with 10,000 resamples and seed 20261008; five-block intervals are
exploratory. No slower observations are discarded.

| Host | Fresh scratch | Stream | Replay |
| --- | ---: | ---: | ---: |
| Apple M5 Max | 1.046× | 1.049× | 1.051× |
| Threadripper PRO 9985WX | 1.081× | 1.077× | 1.069× |

The arm64 CJK case is essentially tied: fresh-scratch 1.007× (0.998–1.015),
stream 1.000× (0.997–1.002), replay 0.998× (0.992–1.004). The gain is workload
dependent. These results do not update the entire historical 84-cell matrix
or measure Python adapter overhead.

## Compared with frozen C

The same candidate is compared directly with C at `55a5230`, using matched
native CPU tuning and thin LTO. This is a 12-cell T5 matrix per host: the same
four corpora at 64-byte, 4096-byte and whole-document chunk settings. Chunked
calls end at the first newline at or beyond the requested size. Fresh scratch
clears semantic caches before each call; stream starts empty after an earlier
pass; replay immediately repeats it. CPU caches can remain hot.

| Host / all 12 cells | Fresh scratch | Stream | Replay |
| --- | ---: | ---: | ---: |
| Apple M5 Max | 1.056× | 1.088× | 1.107× |
| Threadripper PRO 9985WX | 1.099× | 1.118× | 1.133× |

Arm64 English at 4096 bytes is a fresh-scratch tie (1.000×); the gain is not
uniform. All 480 C/candidate timing processes match the original frozen C
stream hashes, and all 12 hashes also agree across architectures. The separate
before/after Rust comparison contributes 160 more matching process outputs.

## Profile and rejected candidate

A fresh five-second arm64 `sample` capture of T5 multilingual encoding placed
about 60% of 4,219 samples in `uni_piece`. Inspecting the native instructions
first suggested removing two saved-ID bounds checks. That experiment failed
to establish a useful cross-architecture gain and was reverted before the
path-emission change. Its paired fresh-scratch/stream/replay ratios versus
the preceding Rust executable were 1.000×/1.009×/1.007× on arm64 and
0.995×/0.994×/0.997× on x86. The saved-ID checks remain in production.

The rejected patch is `experiments/unigram-bounds.patch`. The raw observations
and paired estimates are `receipts/unigram-bounds-rejected-*`; the initial
C/Rust profiles are `receipts/unigram-path-before-*-sample.txt`.
A second five-second capture after the change still identifies `uni_piece`
as the main hot function; it is retained as
`receipts/unigram-path-after-rust-sample.txt`. Removing the emission scan does
not remove the model's dynamic-programming and trie-lookup work.

## Correctness and reproduction

All measured complete token streams must match the frozen C goldens from the
prior full matrix. Both architectures also pass the 12-model pinned Unigram
oracle: each model checks 17,500 encodes, 2,500 piece splits and decode cases,
against separately linked C and Rust archives. ALBERT/XLNet remain excluded
from this generator because its Python model lacks their normalizers; the
target-model caller covers them.

The reused build directories contained three old continuation fixtures whose
generator corrections were already committed: MiniLM truncation and LLM-jp
3/4's start-prefix rule. C and Rust failed those old streams identically. The
corrected streams were recovered and checked against the SHA-256 values in
the existing committed oracle receipts before rerunning all 12 models. The
harness now records both archive paths and hashes and defaults to the separate
C archive instead of the production build directory, which now contains Rust.

The new owned-API boundary test passes in debug and release on both hosts.
Its 504 configurations exercise full/short output, invalid and multibyte
UTF-8, unknowns, continuation and virtual prefixes around the 512-byte saved-ID
boundary. The same explicit expectations pass against the frozen C wheel.
The target, breadth, bounds and misalignment callers pass natively and with
both protected-page placements on both hosts. Details and source hashes are
in `receipts/unigram-path-validation.json`; model/archive hashes and logs are
in `receipts/unigram-path-oracle-{arm64,x86}.json`.
Default portable libraries and CPython 3.13 wheels were then rebuilt on both
hosts. Both installed wheels pass all 40 API, threading and boundary checks;
their hashes and test receipts are included in the validation file. These 40
checks are not a full local Python parity rerun; that suite is also part of the
fork's CI workflow.

Reproduce the paired measurements with the build flags and fixed corpora from
[the full matrix](benchmarks.md), using the same `tools/bench/e2e.c` caller for
every archive:

```sh
python3 tools/rust-port/bench.py --c <previous-rust-e2e> \
  --rust <updated-rust-e2e> --models uni_t5base --chunks 4096 \
  --rounds 5 --reps 9 --golden <prior-c-golden.json> --out <before-after-dir>
python3 tools/rust-port/bench.py --c <native-lto-c-e2e> \
  --rust <updated-rust-e2e> --models uni_t5base --chunks 64 4096 0 \
  --rounds 5 --reps 9 --golden <prior-c-golden.json> --out <c-comparison-dir>
# Add --cpu 8 on the measured Linux host.
python3 tools/rust-port/report.py <results-dir>/results.json --out <paired.json>
```

The harness labels the comparator `c` even when `--c` deliberately supplies
the preceding Rust executable; binary paths and hashes in the raw receipts
identify both sides. All raw process metrics are retained in
`receipts/unigram-path-vs-{rust,c}-{arm64,x86}.json.gz`, with paired estimates
in the corresponding `-paired.json` files.

## Every paired cell

Intervals below are the exploratory 95% bootstrap intervals described above.

### Apple M5 Max versus preceding Rust

| Corpus–chunk | Fresh scratch | Stream | Replay |
| --- | ---: | ---: | ---: |
| en-4096 | 1.044× (1.016–1.072) | 1.047× (1.024–1.070) | 1.043× (1.023–1.062) |
| code-4096 | 1.088× (1.056–1.117) | 1.110× (1.100–1.121) | 1.103× (1.092–1.113) |
| ml-4096 | 1.048× (0.953–1.108) | 1.044× (0.977–1.083) | 1.065× (1.052–1.076) |
| cjk-4096 | 1.007× (0.998–1.015) | 1.000× (0.997–1.002) | 0.998× (0.992–1.004) |

### Threadripper PRO 9985WX versus preceding Rust

| Corpus–chunk | Fresh scratch | Stream | Replay |
| --- | ---: | ---: | ---: |
| en-4096 | 1.074× (1.069–1.082) | 1.062× (1.060–1.064) | 1.048× (1.045–1.051) |
| code-4096 | 1.128× (1.122–1.134) | 1.133× (1.127–1.139) | 1.126× (1.120–1.131) |
| ml-4096 | 1.113× (1.112–1.114) | 1.098× (1.095–1.101) | 1.089× (1.085–1.092) |
| cjk-4096 | 1.011× (0.996–1.024) | 1.017× (1.009–1.022) | 1.015× (1.008–1.020) |

### Apple M5 Max versus C

| Corpus–chunk | Fresh scratch | Stream | Replay |
| --- | ---: | ---: | ---: |
| en-64 | 1.031× (1.005–1.063) | 1.085× (1.049–1.133) | 1.100× (1.056–1.149) |
| en-4096 | 1.000× (0.968–1.030) | 1.084× (1.054–1.109) | 1.135× (1.101–1.165) |
| en-0 | 1.078× (1.036–1.122) | 1.086× (1.056–1.133) | 1.122× (1.089–1.167) |
| code-64 | 1.083× (1.046–1.119) | 1.112× (1.080–1.155) | 1.129× (1.097–1.173) |
| code-4096 | 1.063× (1.043–1.083) | 1.125× (1.106–1.148) | 1.148× (1.128–1.169) |
| code-0 | 1.112× (1.071–1.148) | 1.138× (1.116–1.161) | 1.162× (1.139–1.185) |
| ml-64 | 1.045× (1.008–1.092) | 1.097× (1.075–1.118) | 1.117× (1.101–1.133) |
| ml-4096 | 1.043× (1.027–1.061) | 1.095× (1.081–1.111) | 1.113× (1.102–1.124) |
| ml-0 | 1.110× (1.100–1.117) | 1.123× (1.113–1.141) | 1.139× (1.130–1.155) |
| cjk-64 | 1.031× (1.018–1.040) | 1.038× (1.025–1.052) | 1.040× (1.027–1.054) |
| cjk-4096 | 1.037× (1.028–1.048) | 1.045× (1.029–1.062) | 1.041× (1.020–1.062) |
| cjk-0 | 1.042× (1.028–1.056) | 1.037× (1.017–1.055) | 1.043× (1.025–1.059) |

### Threadripper PRO 9985WX versus C

| Corpus–chunk | Fresh scratch | Stream | Replay |
| --- | ---: | ---: | ---: |
| en-64 | 1.079× (1.071–1.084) | 1.118× (1.115–1.121) | 1.147× (1.143–1.151) |
| en-4096 | 1.083× (1.077–1.091) | 1.134× (1.129–1.140) | 1.169× (1.162–1.177) |
| en-0 | 1.129× (1.123–1.133) | 1.136× (1.132–1.142) | 1.172× (1.166–1.180) |
| code-64 | 1.103× (1.094–1.119) | 1.124× (1.101–1.144) | 1.141× (1.126–1.158) |
| code-4096 | 1.120× (1.118–1.123) | 1.159× (1.158–1.161) | 1.179× (1.176–1.183) |
| code-0 | 1.168× (1.160–1.185) | 1.152× (1.135–1.162) | 1.172× (1.156–1.182) |
| ml-64 | 1.094× (1.091–1.097) | 1.131× (1.128–1.134) | 1.137× (1.134–1.141) |
| ml-4096 | 1.107× (1.104–1.109) | 1.150× (1.147–1.154) | 1.158× (1.154–1.162) |
| ml-0 | 1.153× (1.148–1.158) | 1.156× (1.152–1.159) | 1.164× (1.159–1.167) |
| cjk-64 | 1.034× (1.012–1.058) | 1.037× (1.027–1.044) | 1.036× (1.026–1.043) |
| cjk-4096 | 1.062× (1.047–1.073) | 1.068× (1.064–1.073) | 1.067× (1.063–1.071) |
| cjk-0 | 1.066× (1.060–1.077) | 1.064× (1.057–1.072) | 1.063× (1.055–1.071) |
