#!/usr/bin/env python3
"""One-time stable-Rust/dual-ISA bootstrap of C2Rust 0.22.1 output.

Run only immediately after translation; subsequent changes live in Rust sources.
"""
import re
from pathlib import Path
ROOT = Path(__file__).resolve().parents[2]
R = ROOT / 'rust'
for p in (R / 'src').rglob('*.rs'):
    s = p.read_text()
    opaque = re.findall(r'    pub type (\w+);\n', s)
    s = re.sub(r'    pub type \w+;\n', '', s)
    s = ''.join(f'#[repr(C)]\npub struct {name} {{ _opaque: [u8; 0] }}\n' for name in opaque) + s
    def alias(m):
        indent, name, isa = m.groups()
        other = 'neon' if isa == 'avx2' else 'avx2'
        arch = 'aarch64' if isa == 'avx2' else 'x86_64'
        return f'{indent}#[cfg_attr(target_arch = "{arch}", link_name = "{name}_{other}")]\n{indent}fn {name}_{isa}('
    s = re.sub(r'(    )fn (toks_\w+)_(avx2|neon)\(', alias, s)
    s = s.replace('::core::intrinsics::atomic_', 'crate::atomic::atomic_')
    p.write_text(s)
s = (R / 'lib.rs').read_text()
s = re.sub(r'^#!\[feature.*\]\n', '', s, flags=re.M)
s += '\nmod atomic;\nmod platform;\n'
(R / 'lib.rs').write_text(s)
(R / 'rust-toolchain.toml').unlink(missing_ok=True)
