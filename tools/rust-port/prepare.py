#!/usr/bin/env python3
"""Prepare the initial C2Rust migration database; never used by cargo builds."""
import json
from pathlib import Path
root = Path(__file__).resolve().parents[2]
flags = ['clang', '-std=c17', '-Iinclude', '-Isrc/core', '-Isrc/platform']
for isa in ('neon', 'avx2'):
    for kernel in ('K1', 'K3_CL100K', 'K3_O200K', 'K3_DSV3', 'K5', 'K6', 'K7_SPM'):
        flags.append(f'-DTOKS_HAVE_{kernel}_{isa.upper()}=1')
sources = sorted(root.glob('src/core/*.c')) + sorted(root.glob('src/gen/*.c'))
sources += [root / 'src/par/par.c']
database = [dict(directory=str(root), file=str(p), arguments=flags + ['-c', str(p)]) for p in sources]
out = root / 'build/rust-port'
out.mkdir(parents=True, exist_ok=True)
(out / 'compile_commands.json').write_text(json.dumps(database, indent=2) + '\n')
print(out / 'compile_commands.json')
