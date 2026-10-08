#!/usr/bin/env python3
"""Paired, exact-output C/Rust benchmark using the SAME upstream e2e caller.

Every ABBA run emits raw logs and full token-stream SHA-256. Timings exclude
load and output validation. Report all cells; never discard slow Rust cells.
"""
import argparse
import hashlib
import json
import math
import os
from pathlib import Path
import platform
import statistics
import subprocess
import time
ROOT = Path(__file__).resolve().parents[2]
os.chdir(ROOT)
ap = argparse.ArgumentParser()
ap.add_argument('--c', default='build/rust-bench/c-e2e')
ap.add_argument('--rust', default='build/rust-bench/rust-e2e')
ap.add_argument('--models', nargs='+', default=['gpt2', 'llama3', 'o200k', 'qwen38', 'gemma4', 'wp-bert-uncased', 'uni_t5base'])
ap.add_argument('--corpora', nargs='+', default=['en', 'code', 'ml', 'cjk'])
ap.add_argument('--chunks', nargs='+', type=int, default=[4096])
ap.add_argument('--rounds', type=int, default=3)
ap.add_argument('--reps', type=int, default=5)
ap.add_argument('--cpu', type=int)
ap.add_argument('--out', default='build/rust-bench/paired')
a = ap.parse_args()
if a.rounds < 1 or a.reps < 1: ap.error('rounds and reps must be positive')
out = Path(a.out); out.mkdir(parents=True, exist_ok=True)
def sha(p):
    h = hashlib.sha256()
    with open(p, 'rb') as f:
        for b in iter(lambda: f.read(1 << 20), b''): h.update(b)
    return h.hexdigest()
def command(cmd):
    return subprocess.check_output(cmd, text=True, stderr=subprocess.STDOUT).strip()
meta = {'host': platform.platform(), 'cpu': a.cpu, 'args': vars(a),
        'commit': command(['git', 'rev-parse', 'HEAD']) if (ROOT/'.git').exists() else 'rsync checkout',
        'rustc': command(['rustc', '-Vv']), 'clang': command(['clang', '--version']),
        'binaries': {k: {'path': v, 'sha256': sha(v)} for k,v in [('c', a.c), ('rust', a.rust)]},
        'started': time.time(), 'cells': []}
meta['processor'] = command(['sysctl','-n','machdep.cpu.brand_string']) if platform.system() == 'Darwin' else command(['lscpu'])
if a.cpu is not None:
    meta['siblings'] = Path(f'/sys/devices/system/cpu/cpu{a.cpu}/topology/thread_siblings_list').read_text().strip()
# Golden streams derive from the frozen C implementation, once per cell.
golden = {}
for model in a.models:
    tokenizer = Path(os.environ.get('TOKS_TOKENIZER_CACHE', Path.home()/'.cache/toks/tokenizers')) / model
    if not tokenizer.is_file(): raise SystemExit(f'missing tokenizer: {tokenizer}')
    for corpus in a.corpora:
        text = ROOT / 'build/rust-bench/corpora' / f'{corpus}.txt'
        for chunk in a.chunks:
            key = f'{model}-{corpus}-{chunk}'
            cell = {'key': key, 'model_sha256': sha(tokenizer), 'corpus_sha256': sha(text), 'bytes': text.stat().st_size, 'runs': []}
            for block in range(a.rounds):
                # Alternating ABBA/BAAB preserves pairing while balancing starts.
                order = ('c','rust','rust','c') if block % 2 == 0 else ('rust','c','c','rust')
                for slot, side in enumerate(order):
                    tag = f'{key}-{block}-{slot}-{side}'
                    ids = out/'ids.tmp'
                    env = dict(os.environ, E2E_IDS_OUT=str(ids))
                    cmd = [a.c if side == 'c' else a.rust, str(tokenizer), str(chunk), str(a.reps), str(text)]
                    if a.cpu is not None: cmd = ['taskset', '-c', str(a.cpu)] + cmd
                    before = os.getloadavg(); start = time.monotonic()
                    cp = subprocess.run(cmd, text=True, stdout=subprocess.PIPE, stderr=subprocess.STDOUT, env=env, check=True)
                    elapsed = time.monotonic()-start
                    (out/f'{tag}.log').write_text(cp.stdout)
                    digest = sha(ids); ids.unlink()
                    expected = golden.setdefault(key, {'sha256': digest, 'from': tag, 'binary_sha256': meta['binaries'][side]['sha256']})
                    # The first block always starts with C; a golden is never generated from Rust.
                    assert digest == expected['sha256'], f'token stream mismatch: {tag}'
                    line = next(l for l in cp.stdout.splitlines() if l.startswith('E2E '))
                    fields = dict(x.split('=',1) for x in line.split()[1:])
                    run = {'side': side, 'block': block, 'slot': slot, 'seconds': elapsed, 'load_before': before, 'load_after': os.getloadavg(), 'ids_sha256': digest, 'metrics': fields}
                    cell['runs'].append(run)
            ratios = {}
            for state in ('cold','pass','warm'):
                times = {side: [statistics.median(float(t) for t in r['metrics'][state+'_reps_s'].split(',')) for r in cell['runs'] if r['side']==side] for side in ('c','rust')}
                ratios[state] = statistics.median(times['c']) / statistics.median(times['rust'])
            cell['speedup'] = ratios
            meta['cells'].append(cell)
            (out/'results.json').write_text(json.dumps(meta, indent=2)+'\n')
            (out/'golden.json').write_text(json.dumps(golden, indent=2)+'\n')
            print(key, ' '.join(f'{k}={v:.3f}x' for k,v in ratios.items()), flush=True)
meta['geomean'] = {state: math.exp(statistics.mean(math.log(c['speedup'][state]) for c in meta['cells'])) for state in ('cold','pass','warm')}
meta['finished'] = time.time()
(out/'results.json').write_text(json.dumps(meta, indent=2)+'\n')
print('geomean:', meta['geomean'])
