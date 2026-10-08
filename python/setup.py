"""Build the self-contained Rust extension and native assembly kernels."""
import os
import re
import sys
import sysconfig
from setuptools import setup
from setuptools_rust import RustExtension

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.dirname(HERE)
for name in ("LICENSE", "NOTICE", "LICENSING.md", "THIRD_PARTY_NOTICES.md"):
    src = os.path.join(ROOT, name)
    if os.path.isfile(src):
        with open(src, "rb") as f:
            data = f.read()
        with open(os.path.join(HERE, name), "wb") as f:
            f.write(data)

def _version():
    """TOKS_VERSION of include/toks.h: the wheel's version is the library's, never a second number"""
    with open(os.path.join(ROOT, "include", "toks.h"), encoding="utf-8") as f:
        m = re.search(r'^#define TOKS_VERSION +"([0-9]+\.[0-9]+\.[0-9]+)"', f.read(), re.M)
    if m is None:
        raise RuntimeError("toks: no TOKS_VERSION in include/toks.h")
    return m.group(1)


if sys.platform == "darwin":
    os.environ.setdefault("MACOSX_DEPLOYMENT_TARGET", sysconfig.get_config_var("MACOSX_DEPLOYMENT_TARGET") or "11.0")
setup(
    version=_version(),
    rust_extensions=[RustExtension("toks._toks", path="rust/Cargo.toml", args=["--profile", "python"], cargo_manifest_args=["--locked"])],
)
