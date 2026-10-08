#!/usr/bin/env python3
"""Link upstream C test callers to the Rust implementation, record every result."""
import argparse
import concurrent.futures
import json
import os
import platform
from pathlib import Path
import subprocess
import time
ROOT = Path(__file__).resolve().parents[2]
os.chdir(ROOT)
p = argparse.ArgumentParser()
p.add_argument('--lib', default='target/release/libtoks.a')
p.add_argument('--jobs', type=int, default=8)
p.add_argument('--tests', nargs='*')
p.add_argument('--out', default='build/rust-tests')
a = p.parse_args()
isa = 'arm64' if platform.machine() in ('arm64', 'aarch64') else 'x86_64'
out = Path(a.out); out.mkdir(parents=True, exist_ok=True)
asm = sorted((ROOT / 'src/asm' / isa).glob('*.S'))
flags = ['clang', '-std=c17', '-O2', '-fno-strict-aliasing', '-fwrapv', '-Wall', '-Wextra', '-Werror', '-Iinclude', '-Isrc/core', '-Isrc/platform', '-Itests/common', '-Itests/c']
flags += [f'-DTOKS_HAVE_{f.stem.upper()}=1' for f in asm if f.stem.startswith('k')]
flags += ['-DTOKS_ASM_SOURCES="' + ' '.join(f.name for f in asm) + '"']
common = ['tests/common/guard.c', f'tests/common/abicheck_{isa}.S', a.lib, '-lpthread', '-lm']
if platform.system() == 'Linux': common += ['-ldl', '-lrt', '-lutil']
# These exercise replacements or source-included C internals. They require their
# own Rust test builds; keep them visibly pending instead of testing C by mistake.
special = {'test_api': 'test-api', 'test_alloc': 'test-allocator', 'test_cuts': 'test-internals', 'test_memo_hash': 'test-degenerate', 'test_state_hash': 'test-degenerate'}
sources = sorted(ROOT.glob('tests/c/*.c'))
tier = 'neon' if isa == 'arm64' else 'avx2'
asm_bpe = out / f'test_bpe_{tier}.c'
asm_bpe.write_text(f'#define K6_BPE toks_k6_bpe_{tier}\n#define K5_ENCODE toks_k5_encode_{tier}\n#define TEST_BPE_TIER "{tier}"\n#define TEST_BPE_FEAT TOKS_FEAT_{tier.upper()}_TIER\n#include "{ROOT / "tests/c/test_bpe.c"}"\n')
sources.append(asm_bpe)
if a.tests: sources = [s for s in sources if s.stem in a.tests]
variants = {}
for feature in sorted({special[s.stem] for s in sources if s.stem in special}):
    dest = out / feature
    with (out/f'{feature}.build.log').open('wb') as log:
        subprocess.run(['cargo', 'rustc', '-p', 'toks', '--lib', '--crate-type', 'staticlib', '--release', '--features', feature, '--target-dir', str(dest)], stdout=log, stderr=subprocess.STDOUT, check=True)
    variants[feature] = str(dest / 'release/libtoks.a')
def source(s):
    text = s.read_text()
    if s.stem not in ('test_cuts', 'test_memo_hash', 'test_state_hash'): return s
    if s.stem == 'test_cuts':
        text = text.replace('#include "../../src/core/api.c"', '''#include "core.h"
typedef toks_emit emit;
extern int py_isspace(uint32_t);
extern void run_cuts(const toks_ctx *, toks_scratch *, const uint8_t *, uint64_t, uint64_t, int, emit *, int, int);''')
    else:
        text = text.replace('#include "../../src/core/api.c"', '').replace('#include "../../src/core/k5_long.c"', '')
    path = out / s.name
    path.write_text(text)
    return path
def run(s):
    name = s.stem
    start = time.monotonic()
    cmd = flags + ['-o', str(out/name), str(source(s))] + [variants.get(special.get(name), v) if v == a.lib else v for v in common]
    cp = subprocess.run(cmd, stdout=subprocess.PIPE, stderr=subprocess.STDOUT)
    (out/f'{name}.build.log').write_bytes(cp.stdout)
    if cp.returncode: return {'test': name, 'status': 'build-fail', 'rc': cp.returncode}
    with (out/f'{name}.log').open('wb') as f:
        try: rc = subprocess.run([str(out/name)], stdout=f, stderr=subprocess.STDOUT, timeout=900).returncode
        except subprocess.TimeoutExpired: rc = 124
    result = {'test': name, 'status': 'pass' if rc == 0 else 'fail', 'rc': rc, 'seconds': time.monotonic()-start}
    print(json.dumps(result), flush=True)
    return result
with concurrent.futures.ThreadPoolExecutor(max_workers=a.jobs) as pool:
    results = list(pool.map(run, [s for s in sources if s.stem != 'test_stall']))
for s in sources:
    if s.stem == 'test_stall': results.append(run(s))
report = {'host': platform.platform(), 'tier': os.environ.get('TOKS_TIER', 'auto'), 'library': a.lib, 'results': results}
(out/'results.json').write_text(json.dumps(report, indent=2)+'\n')
print(json.dumps(report, indent=2))
raise SystemExit(any(r['status'] != 'pass' for r in results))
