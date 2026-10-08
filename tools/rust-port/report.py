#!/usr/bin/env python3
"""Summarize a paired run, bootstrapping whole ABBA blocks rather than inner reps.

Intervals are exploratory with few blocks. The median time within each process
is one observation; repeated timings from that process are never independent
samples. Report every cell and cache state, including losses.
"""
import argparse
import json
import math
from pathlib import Path
import random
import statistics as st

p = argparse.ArgumentParser()
p.add_argument('results', type=Path)
p.add_argument('--out', type=Path)
a = p.parse_args()
d = json.loads(a.results.read_text())
rng = random.Random(20261008)
rows = []
for cell in d['cells']:
    row = {'key': cell['key']}
    for state in ('cold', 'pass', 'warm'):
        blocks = {}
        for r in cell['runs']:
            t = st.median(float(x) for x in r['metrics'][state+'_reps_s'].split(','))
            blocks.setdefault(r['block'], {'c': [], 'rust': []})[r['side']].append(t)
        ratios = [math.log(st.median(b['c']) / st.median(b['rust'])) for b in blocks.values()]
        boot = sorted(math.exp(st.mean(rng.choices(ratios, k=len(ratios)))) for _ in range(10000))
        row[state] = {'speedup': math.exp(st.mean(ratios)), 'ci95': [boot[249], boot[9749]], 'blocks': len(ratios)}
    rows.append(row)
result = {'source': str(a.results), 'estimator': 'geometric mean of paired block median ratios',
          'interval': 'percentile bootstrap of whole ABBA blocks, 10000 resamples, seed 20261008', 'cells': rows}
result['geomean'] = {s: math.exp(st.mean(math.log(r[s]['speedup']) for r in rows)) for s in ('cold','pass','warm')}
for r in rows:
    print(r['key'], ' '.join(f"{s}={r[s]['speedup']:.3f}x [{r[s]['ci95'][0]:.3f}, {r[s]['ci95'][1]:.3f}]" for s in ('cold','pass','warm')))
print('geomean:', result['geomean'])
if a.out: a.out.write_text(json.dumps(result, indent=2)+'\n')
