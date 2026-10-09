#!/usr/bin/env python3
"""Generate pinned HF oracle streams once, then check C and Rust against them."""
import argparse
from concurrent.futures import ThreadPoolExecutor
import hashlib
import json
import os
from pathlib import Path
import platform
import subprocess

ROOT = Path(__file__).resolve().parents[2]
os.chdir(ROOT)
p = argparse.ArgumentParser()
p.add_argument('--models', nargs='+')
p.add_argument('--n', type=int, default=2500)
p.add_argument('--jobs', type=int, default=4)
p.add_argument('--out', default='build/rust-unigram-oracle')
p.add_argument('--reuse', action='store_true', help='use existing pinned oracle streams')
p.add_argument('--python', help='Python with tokenizers==0.23.2; otherwise use uv')
p.add_argument('--c-lib', help='separate C reference archive (default: build/c-reference/<platform>/libtoks.a)')
p.add_argument('--rust-lib', default='target/release/libtoks.a')
a = p.parse_args()
out = Path(a.out); out.mkdir(parents=True, exist_ok=True)
cache = Path(os.environ.get('TOKS_TOKENIZER_CACHE', Path.home()/'.cache/toks/tokenizers'))
# Match tests/unigram/run_all.sh's generator model set. The separate Python
# model used to construct this generator's context lacks ALBERT/XLNet normalizers;
# this is a generator limitation, not a claim about the library's support.
models = a.models or sorted(x.name for x in cache.glob('uni_*') if x.name not in ('uni_albert', 'uni_xlnet'))
isa = 'arm64' if platform.machine() in ('arm64','aarch64') else 'x86_64'
plat = 'macos' if platform.system() == 'Darwin' else 'linux'
libraries = {'c': a.c_lib or f'build/c-reference/{plat}-{isa}/libtoks.a', 'rust': a.rust_lib}
for side, lib in libraries.items():
    subprocess.run(['clang', '-O2', '-std=c17', '-Iinclude', 'tests/unigram/e2e_check.c', lib,
                    '-lpthread', '-lm', '-o', str(out/side)] + (['-ldl'] if plat=='linux' else []), check=True)
def run(model):
    stream = out/(model+'.bin')
    if not a.reuse:
        with stream.open('wb') as f, (out/(model+'.generator.log')).open('wb') as err:
            py = [a.python] if a.python else ['uv','run','-q','--with','tokenizers==0.23.2','python']
            subprocess.run(py + ['tests/unigram/e2e_gen.py','--tok',model,'--kind','random','--n',str(a.n)],
                           stdout=f, stderr=err, check=True)
    result = {'model': model, 'model_sha256': hashlib.sha256((cache/model).read_bytes()).hexdigest(),
              'oracle_sha256': hashlib.sha256(stream.read_bytes()).hexdigest(), 'generator': 'tokenizers==0.23.2',
              'random_cases_requested': a.n, 'results': {}}
    for side in ('c','rust'):
        with stream.open('rb') as f:
            cp = subprocess.run([str(out/side),str(cache/model)],stdin=f,stdout=subprocess.PIPE,stderr=subprocess.STDOUT)
        log = cp.stdout.decode(errors='replace')
        (out/(model+'.'+side+'.log')).write_text(log)
        result['results'][side] = {'status': cp.returncode, 'log': log}
    print(model, {s: r['status'] for s,r in result['results'].items()}, flush=True)
    return result
with ThreadPoolExecutor(max_workers=a.jobs) as pool:
    results = list(pool.map(run, models))
(out/'results.json').write_text(json.dumps({'host': platform.platform(),
    'libraries': {s: {'path': lib, 'sha256': hashlib.sha256(Path(lib).read_bytes()).hexdigest()} for s,lib in libraries.items()}, 'models': results,
    'generator_exclusions': [] if a.models else ['uni_albert', 'uni_xlnet']},indent=2)+'\n')
raise SystemExit(any(r['status'] != 0 for m in results for r in m['results'].values()))
