"""Build the self-contained Rust extension and native assembly kernels."""
import os
import re
import shutil
import sys
import sysconfig
from pathlib import Path
from setuptools import setup
from setuptools.command.sdist import sdist
from setuptools_rust import RustExtension

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.join(HERE, "native") if os.path.isdir(os.path.join(HERE, "native")) else os.path.dirname(HERE)
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


class SourceDistribution(sdist):
    """Stage the Rust core and assembly beside the adapter in source archives."""

    def make_release_tree(self, base_dir, files):
        super().make_release_tree(base_dir, files)
        release = Path(base_dir)
        root = Path(ROOT)
        native = release / "native"
        if root == Path(HERE) / "native":
            # Repacking an already standalone source distribution.
            shutil.copytree(root, native, dirs_exist_ok=True)
            return
        shutil.copytree(root / "rust", native / "rust", ignore=shutil.ignore_patterns("tests"))
        shutil.copytree(root / "src" / "asm", native / "src" / "asm")
        (native / "src" / "core").mkdir(parents=True)
        shutil.copy2(root / "src" / "core" / "layout.h", native / "src" / "core" / "layout.h")
        shutil.copytree(root / "include", native / "include")
        shutil.copy2(root / "Cargo.lock", release / "Cargo.lock")
        profiles = (root / "Cargo.toml").read_text().split("[profile.release]", 1)[1]
        (release / "Cargo.toml").write_text(
            '[workspace]\nmembers = ["rust", "native/rust"]\nresolver = "2"\n\n'
            "[profile.release]" + profiles
        )
        manifest = release / "rust" / "Cargo.toml"
        # sdist may hard-link source files; unlink before rewriting so building
        # a distribution never changes the checkout's path dependency.
        source = manifest.read_text()
        if source.count('../../rust') != 1:
            raise RuntimeError("toks: unexpected core dependency in adapter manifest")
        manifest.unlink()
        manifest.write_text(source.replace('../../rust', '../native/rust'))


if sys.platform == "darwin":
    os.environ.setdefault("MACOSX_DEPLOYMENT_TARGET", sysconfig.get_config_var("MACOSX_DEPLOYMENT_TARGET") or "11.0")
setup(
    version=_version(),
    cmdclass={"sdist": SourceDistribution},
    rust_extensions=[RustExtension("toks._toks", path="rust/Cargo.toml", args=["--profile", "python"], cargo_manifest_args=["--locked"])],
)
