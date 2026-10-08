#!/usr/bin/env python3
"""Materialize public, recorded benchmark inputs (no synthetic repeated text)."""
import hashlib
import json
import urllib.request
from pathlib import Path
ROOT = Path(__file__).resolve().parents[2]
OUT = ROOT / 'build/rust-bench/corpora'
OUT.mkdir(parents=True, exist_ok=True)
BOOKS = {'en': [1342, 2701], 'ml': [2229, 2000, 17489], 'cjk': [24264]}
manifest = {}
for name, ids in BOOKS.items():
    sources = []
    texts = []
    for id in ids:
        url = f'https://www.gutenberg.org/cache/epub/{id}/pg{id}.txt'
        path = OUT / f'pg{id}.txt'
        if not path.exists():
            path.write_bytes(urllib.request.urlopen(url, timeout=120).read())
        text = path.read_bytes()
        text.decode('utf8')
        texts.append(text)
        sources.append({'url': url, 'sha256': hashlib.sha256(text).hexdigest()})
    text = b'\n'.join(texts)
    (OUT / f'{name}.txt').write_bytes(text)
    manifest[name] = {'sources': sources, 'bytes': len(text), 'sha256': hashlib.sha256(text).hexdigest()}
paths = sorted(ROOT.glob('src/core/*.[ch]')) + sorted(ROOT.glob('python/**/*.py'))
text = b'\n'.join(p.read_bytes() for p in paths)
(OUT / 'code.txt').write_bytes(text)
manifest['code'] = {'sources': [str(p.relative_to(ROOT)) for p in paths], 'bytes': len(text), 'sha256': hashlib.sha256(text).hexdigest()}
(OUT / 'manifest.json').write_text(json.dumps(manifest, indent=2) + '\n')
print(json.dumps(manifest, indent=2))
