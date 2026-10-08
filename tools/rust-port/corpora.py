#!/usr/bin/env python3
"""Materialize public, recorded benchmark inputs (no synthetic repeated text)."""
import hashlib
import json
import subprocess
import urllib.request
from pathlib import Path
ROOT = Path(__file__).resolve().parents[2]
OUT = ROOT / 'build/rust-bench/corpora'
OUT.mkdir(parents=True, exist_ok=True)
BOOKS = {'en': [1342, 2701], 'ml': [2229, 2000, 17489], 'cjk': [24264]}
SOURCE_COMMIT = '55a5230b08a75916a2b36f92c320f057376833ea'
PINNED = json.loads((ROOT / 'docs/rust-port/receipts/corpora.json').read_text())
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
        digest = hashlib.sha256(text).hexdigest()
        expected = next(s['sha256'] for s in PINNED[name]['sources'] if s['url'] == url)
        if digest != expected:
            raise SystemExit(f'corpus source changed: {url}; expected {expected}, got {digest}')
        text.decode('utf8')
        texts.append(text)
        sources.append({'url': url, 'sha256': hashlib.sha256(text).hexdigest()})
    text = b'\n'.join(texts)
    (OUT / f'{name}.txt').write_bytes(text)
    manifest[name] = {'sources': sources, 'bytes': len(text), 'sha256': hashlib.sha256(text).hexdigest()}
paths = PINNED['code']['sources']
# Keep the code corpus stable when the port or its tests change. Reading the
# frozen source through git also excludes subsequently added Python test files.
text = b'\n'.join(subprocess.check_output(['git', 'show', f'{SOURCE_COMMIT}:{p}'], cwd=ROOT) for p in paths)
(OUT / 'code.txt').write_bytes(text)
manifest['code'] = {'sources': paths, 'source_commit': SOURCE_COMMIT, 'bytes': len(text), 'sha256': hashlib.sha256(text).hexdigest()}
for name, entry in manifest.items():
    if entry['sha256'] != PINNED[name]['sha256']:
        raise SystemExit(f'combined corpus mismatch: {name}')
(OUT / 'manifest.json').write_text(json.dumps(manifest, indent=2) + '\n')
print(json.dumps(manifest, indent=2))
