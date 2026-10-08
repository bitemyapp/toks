"""Ownership, reentrancy and buffer contracts shared by both native adapters."""
import array
from concurrent.futures import ThreadPoolExecutor
import gc
import pathlib
import threading

import pytest
import toks


def test_factory_preserves_subtype_without_constructors(gpt2_path):
    class Derived(toks.Tokenizer):
        def __new__(cls, *args, **kwargs):
            raise AssertionError("factory called the subclass constructor")

        def __init__(self, *args, **kwargs):
            raise AssertionError("factory called the subclass initializer")

    data = pathlib.Path(gpt2_path).read_bytes()
    for t in (Derived.from_file(gpt2_path), Derived.from_str(data.decode()), Derived.from_buffer(data)):
        assert type(t) is Derived
        t.note = "retained"
        assert t.note == "retained" and t.encode("Hello world") == [15496, 995]
    with pytest.raises(TypeError):
        toks.Tokenizer()


def test_reentrant_index_uses_snapshot(gpt2):
    values = []

    class Index:
        def __index__(self):
            values.clear()
            assert gpt2.encode("Hello") == [15496]
            return 15496

    values.extend([Index(), 995])
    assert gpt2.decode(values) == "Hello world"
    assert values == []


def test_omitted_and_explicit_none_arguments(gpt2):
    # Explicit None selects the supplied interface and is false for truth-valued
    # options. It must not collapse into the omitted/default case.
    special = "<|endoftext|>"
    assert gpt2.decode([50256]) == ""
    assert gpt2.decode([50256], None) == special
    assert gpt2.decode_batch([[50256]], skip_special_tokens=None) == [special]
    for name in ("add_special_tokens", "added_tokens", "continuation"):
        with pytest.raises(TypeError):
            gpt2.encode("Hello", allowed_special="all", **{name: None})
    for name in ("allowed_special", "disallowed_special"):
        with pytest.raises(TypeError):
            gpt2.encode("Hello", **{name: None})
    assert gpt2.encode(text="Hello", added_tokens=None) == [15496]
    assert gpt2.decode_bytes_batch(batch=[[15496]], num_threads=object()) == [b"Hello"]

    class Truth:
        def __bool__(self):
            assert gpt2.encode("world") == [6894]
            return False

    assert gpt2.decode([50256], skip_special_tokens=Truth()) == special


def test_integer_buffers_and_unaligned_output(gpt2):
    for format in "bBhHiIlLqQ":
        assert gpt2.decode(array.array(format, [64, 65])) == "ab"
    for format in "bhilq":
        with pytest.raises(OverflowError):
            gpt2.decode(array.array(format, [-1]))
    values = array.array("I", [15496, 995] * 5000)
    storage = bytearray(b"x" + values.tobytes())
    view = memoryview(storage)[1:].cast("I")
    assert gpt2.decode(view) == "Hello world" * 5000
    out = memoryview(bytearray(1 + 4 * 8))[1:].cast("I", shape=(2, 4))
    assert gpt2.encode_into("Hello world", out) == 2
    assert out[0, 0] == 15496 and out[0, 1] == 995
    with pytest.raises(TypeError):
        gpt2.decode(out)
    with pytest.raises((BufferError, TypeError)):
        gpt2.encode_into("x", out.toreadonly())
    # Failed export acquisition or format checks must not leave an export held.
    bad = bytearray(8)
    with pytest.raises(TypeError):
        gpt2.encode_into("x", bad)
    bad.extend(b"resize after failure")


def test_concurrent_lazy_vocab_and_callback(gpt2_path, monkeypatch):
    import toks._vocab as vocab

    t = toks.Tokenizer.from_file(gpt2_path)
    original = vocab.load
    barrier = threading.Barrier(2)

    def load(*args):
        assert t.encode("Hello world") == [15496, 995]
        barrier.wait(timeout=10)
        return original(*args)

    monkeypatch.setattr(vocab, "load", load)
    with ThreadPoolExecutor(2) as pool:
        futures = [pool.submit(t.get_vocab) for _ in range(2)]
        a, b = [f.result(timeout=15) for f in futures]
    assert a == b and a is not b and a["Hello"] == 15496
    a.clear()
    assert t.get_vocab()["Hello"] == 15496


def test_retained_stream_encoding_and_mutable_source(gpt2_path):
    source = bytearray(pathlib.Path(gpt2_path).read_bytes())
    t = toks.Tokenizer.from_buffer(source)
    source[:] = b"changed"
    e = t.encode_ex("Hello world")
    s = t.decode_stream()
    del t
    gc.collect()
    assert e.tokens == ["Hello", "Ġworld"]
    assert s.push(e.ids) + s.flush() == "Hello world"
