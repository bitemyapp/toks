#!/usr/bin/env python3
"""Paired installed-wheel overhead benchmark, with complete ID/byte digests."""
import argparse
import array
import gc
import hashlib
import json
import math
import os
from pathlib import Path
import platform
import statistics
import subprocess
import sys
import time

p = argparse.ArgumentParser()
p.add_argument('--worker', action='store_true')
p.add_argument('--c-python', default='build/c-python-env/bin/python')
p.add_argument('--rust-python', default='build/rust-python-env/bin/python')
p.add_argument('--rounds', type=int, default=3)
p.add_argument('--out', default='build/rust-port/python-bench.json')
a = p.parse_args()

if a.worker:
    import toks
    import toks._toks as native
    rows = []
    for model in ('gpt2', 'uni_t5base'):
        path = Path.home()/'.cache/toks/tokenizers'/model
        tok = toks.Tokenizer.from_file(path)
        for size in (20, 4096):
            text = ('Hello world. Café, numbers 1234567, and source code: def foo(x): return x + 1.\n' * 100)[:size]
            ids = tok.encode(text)
            decoded = tok.decode(ids)
            gold = hashlib.sha256(array.array('I', ids).tobytes() + decoded.encode()).hexdigest()
            output = array.array('I', [0])*tok.encode_bound(len(text.encode()))
            methods = {
                'encode': lambda: tok.encode(text),
                'encode_into': lambda: tok.encode_into(text, output),
                'decode': lambda: tok.decode(ids),
                'batch4': lambda: tok.encode_batch([text]*4),
            }
            for name, run in methods.items():
                for _ in range(10): run()
                start = time.perf_counter_ns()
                for _ in range(20): run()
                elapsed = time.perf_counter_ns()-start
                n = max(10, min(20000, int(20_000_000 * 20 / max(1, elapsed))))
                gc.disable()
                samples = []
                try:
                    for _ in range(5):
                        start = time.perf_counter_ns()
                        for _ in range(n): run()
                        samples.append((time.perf_counter_ns()-start)/n)
                finally:
                    gc.enable()
                rows.append({'model': model, 'method': name, 'chars': size, 'bytes': len(text.encode()), 'gold': gold,
                             'iterations': n, 'ns_per_call': samples, 'median_ns': statistics.median(samples)})
    binary = Path(native.__file__)
    print(json.dumps({'python': sys.version, 'module': str(binary), 'module_sha256': hashlib.sha256(binary.read_bytes()).hexdigest(), 'rows': rows}))
    raise SystemExit()

runs = []
for block in range(a.rounds):
    for variant in ('c','rust','rust','c') if block%2==0 else ('rust','c','c','rust'):
        python = a.c_python if variant=='c' else a.rust_python
        run = json.loads(subprocess.check_output([python, __file__, '--worker'], text=True))
        run.update(block=block, variant=variant)
        runs.append(run)
    print(f'block {block+1}/{a.rounds}', flush=True)

summary = []
for i, first in enumerate(runs[0]['rows']):
    ratios = []
    for block in range(a.rounds):
        group = [r for r in runs if r['block']==block]
        assert all(r['rows'][i]['gold']==first['gold'] for r in group), first
        c = statistics.median(r['rows'][i]['median_ns'] for r in group if r['variant']=='c')
        rust = statistics.median(r['rows'][i]['median_ns'] for r in group if r['variant']=='rust')
        ratios.append(c/rust)
    summary.append({k:first[k] for k in ('model','method','chars','bytes')} | {'block_speedups':ratios, 'speedup':math.exp(statistics.mean(map(math.log,ratios)))})
report = {'host':platform.platform(), 'protocol':'whole-process alternating ABBA/BAAB; warm repeated calls; 5 inner repetitions are not independent samples', 'runs':runs, 'summary':summary}
Path(a.out).write_text(json.dumps(report,indent=2)+'\n')
for row in summary: print(json.dumps(row))
