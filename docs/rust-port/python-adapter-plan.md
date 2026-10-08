# Replacing the Python C adapter with Rust

This plan maps `python/toks/_toks.c` as inspected on 2026-10-08. The file has
2,299 lines and remains separate from the Rust core under `rust/`. No adapter
implementation or benchmark was changed while preparing this document.

The recommended replacement is a separate PyO3 extension crate, retaining
the module name `toks._toks`, the existing Python package, and its installed
wheel tests. It should depend on the Rust core as an `rlib`. Preserve the
Python helper `python/toks/_vocab.py` initially: its 456 lines implement
substantial compatibility behavior that is independent of the native language.

## Public and private interfaces

The module exports `Tokenizer`, `Encoding`, `DecodeStream`, `Error`, `ABI`,
`__version__`, `MAX_TEXT`, `ID_ADDED`, `ID_SPECIAL` and `ID_BYTE`.
`toks.__init__` reexports these plus Python's `AddedToken`. Public native type
names are `toks.Tokenizer`, `toks.Encoding` and `toks.DecodeStream`, despite
their implementation module being `toks._toks`.

There are 26 public Tokenizer methods, including three factories, and seven
properties. `python/toks/_toks.pyi`, the C argument-spec table and the Python
tests together define signatures; the stub alone omits private interfaces and
some deliberately permissive runtime behavior.

| Methods or properties | Required behavior |
|---|---|
| `from_file(path, *, tier=None, cache_mib=0)` | Accept str, bytes and `os.PathLike`; preserve the `os.fspath` result as `source`. Release the GIL during loading. Validate cache budget before loading. Allocate the class the factory was called on. |
| `from_str(json, *, tier=None)` | Accept only str; retain that object as source; strict UTF-8 conversion. |
| `from_buffer(data, *, tier=None)` | Retain bytes directly; convert other accepted objects to immutable bytes before releasing the GIL. Mutable input cannot change the stored source afterward. |
| `encode`, `encode_ex`, `encode_into`, `encode_batch`, `encode_batch_ex`, `pieces` | Accept text str/bytes/contiguous buffer; preserve added-token modes, template flag, continuation, total-count and exact-prefix semantics. Batch accepts an iterable, not only a sequence. |
| `encode_ordinary` | Use the tiktoken view's ordinary mode with no template, truncation or padding; repair lone surrogates as tiktoken does. |
| `decode`, `decode_batch` | Default `skip_special_tokens=True`; repaired Unicode str; sequences, iterables and integer buffers accepted. Invalid in-range-u32 IDs raise `toks.Error`. |
| `decode_bytes`, `decode_bytes_batch` | Raw bytes, keeping specials; invalid table IDs raise `KeyError`. `num_threads` is accepted by the batch method but currently unused and not validated. |
| `decode_stream` | Default `skip_special_tokens=False`; return a new stream retaining this Tokenizer. |
| `token_bytes`, `token_byte_values` | Copy raw bytes; missing token returns None; the full values list excludes added IDs and is sorted. |
| `token_to_id` | str means Hugging Face's written vocabulary spelling; a buffer means the core's decoded-byte key. Return None if absent. These are different namespaces. |
| `id_to_token`, `get_vocab`, `get_vocab_size` | Use the lazy Python vocabulary view; `get_vocab` returns a new dict; unknown token strings can raise a deliberate unsupported error. |
| `id_flags`, `encode_bound` | Preserve u32 and u64 index conversion rules, stable errors and saturating bound. |
| `num_special_tokens_to_add`, `get_added_tokens_decoder` | Single-sequence template count; pairs explicitly unsupported; added-token objects returned in ID order with all six fields. |
| `info` | Preserve the nested dict schema, names, booleans, hex hashes/None, truncation, padding and template fields. |
| `encode_special_tokens` | Mutable truth-valued property; true changes the default added mode to nonspecial. Explicit `added_tokens` overrides it; deletion raises AttributeError. |
| `source`, `n_vocab`, `max_token_value`, `eot_token`, `special_tokens_set`, `name` | Read-only properties. `n_vocab` is maximum ID plus one, not dictionary length. `eot_token` raises KeyError if missing; every `special_tokens_set` access returns a new set. |

`Encoding` is created only by encode methods. Its `ids`, `type_ids`, `tokens`,
`attention_mask`, `special_tokens_mask` and `overflowing` properties return new
lists each time. `overflowing` is always empty, `n_sequences` is 1, and `len`
is the padded ID count. Preserve its repr and the distinction between a
special token found in the text and a template special: the former has a zero
special-token mask. `DecodeStream.push` accepts an int or ID sequence;
`flush` emits remaining bytes and resets decoding state for reuse. Neither
type is subclassable or directly constructible in the current extension.

The following private interfaces are real dependencies, not removable internals:

| Interface | Consumer and contract |
|---|---|
| `_toks._error(code, message)` | `_vocab.py`; constructs, rather than raises, an Error with `.code`, `.name` and prefixed message. |
| `_toks._load(kind, source, tier, sha256, encode_special_tokens)` | Existing pickles. Reconstruct the base Tokenizer and reject changed source hashes. Unknown tier values above the current maximum fall back to auto. |
| `Tokenizer._vocab()` | `_vocab.py`; returns `(tok2id, id2tok, model_vocab, n_model, n_total, extra)`. |
| `Encoding._layout` | `_vocab.py`; `(n_prefix, n_suffix, n_pad, pad_left, flags)`. |
| `Encoding._text` | `_vocab.py`; retained str/bytes corresponding to the encoded text. |
| `Tokenizer.__reduce__()` | Returns `_toks._load` plus the five reconstruction arguments above. The current pickle does not preserve subclass or `cache_mib`. |

Do not silently change existing pickle behavior during this port. Add any
future pickle version separately, retaining the old five-argument loader.

## Semantics implemented by the adapter

The adapter is more than an FFI call layer:

- It manages scratch and output storage, retries output sizing when a
  normalizer expands text, and caches Python integer objects for output lists.
- Batch encodes call the serial core repeatedly with one leased scratch.
  They do not use `toks_par`. Each call disables core padding, finds the
  longest encoded item, and applies Fixed/BatchLongest padding, rounding and
  side at the Python boundary. Continuation suppresses padding.
- `encode_ex` also disables core padding so it can distinguish pad IDs from
  text IDs when constructing masks. Template IDs and type IDs are cached at
  Tokenizer creation. Text sequence type IDs remain applicable when template
  insertion is disabled.
- Merely supplying `allowed_special` or `disallowed_special` selects the
  tiktoken interface. Combining either with any of the three Hugging Face
  arguments raises TypeError, even if a supplied value equals its default.
  A standard PyO3 `Option<T>` that conflates omitted and explicit None is not
  sufficient for this dispatch; use an omission sentinel or inspect keywords.
- The tiktoken view uses no post-processing, truncation or padding. It checks
  the leftmost disallowed string, including explicitly disallowed strings
  that are not special tokens. Partial allowed sets invoke the Python helper's
  leftmost/longest split and ordinary encoding between allowed specials.
- Only the tiktoken interface performs UTF-16 surrogatepass/replacement
  repair. Ordinary Hugging Face-style `encode` rejects lone surrogates.
- Batch decode remains sequential. Each sequence independently follows the
  decode GIL threshold; accepting `num_threads` does not enable a pool.

Error mapping also belongs to the adapter. Native `TOKS_E_NOMEM` becomes
MemoryError; other native failures normally become `toks.Error` with stable
code/name. Negative or above-u32 input IDs raise OverflowError, while IDs
inside u32 but outside the vocabulary raise the operation-specific error.
`decode_bytes` uses KeyError with the invalid ID; `decode` and stream push use
Error with the first bad ID's index. Missing vocabulary entries and
`token_bytes` return None. Invalid mode/tier/cache arguments use ValueError,
and Python conversion errors retain their Python type.

## Ownership, buffers and GIL policy

The current Tokenizer owns the loaded context, a cached `toks_info`, source
object, template arrays, lazy vocabulary/tiktoken views, special-ID bitmap,
lazy Python-int table and a list of idle scratch slots. A slot owns scratch,
ID/piece staging and decode-byte staging. Every active operation takes a slot
out of the pool before detaching, then returns it after reattaching; concurrent
calls never share mutable scratch. The pool and Python caches are touched only
with the GIL held. There is no tokenizer-wide lock held through core work.

Slot scratch initially covers at least 64 KiB and grows geometrically. On
return, scratch for texts over 1 MiB is freed; ID staging above 1 Mi IDs and
byte staging above 4 MiB are also freed. Scratch remains initialized between
ordinary calls so memo and piece-cache benefits survive. Preserve these
budgets initially and measure any policy change separately.

The actual source, rather than its more general docstrings, defines this table:

| Operation | Current detached interval |
|---|---|
| File or memory load | The native load call, always |
| Single encode/pieces/encode_into/encode_ex | Native call when input bytes are at least 2,048 |
| Encode batch and batch_ex | Whole native loop when aggregate bytes are at least 2,048 |
| Decode/decode_bytes | Native decode when there are at least 8,192 IDs |
| Decode batch | Per sequence, with the same 8,192-ID threshold |
| Stream push/flush, vocabulary and result construction | Remain attached |

Input preparation, Python conversions and result creation occur while
attached. Batches snapshot the outer iterable to a tuple before taking raw
text pointers; this keeps every item alive while detached. ID conversion
snapshots arbitrary iterables before invoking `__index__`, since a callback
could mutate its original list. Exact compact ints have a CPython 3.12+
fast path which executes no Python callbacks.

Accepted buffer details need explicit porting:

- Text uses a simple contiguous buffer export with no integer-format rule.
  Invalid UTF-8 bytes are passed to the core unchanged.
- ID input rejects str, bytes and bytearray as such, but accepts integer
  buffer views. Formats `bBhHiIlLqQnN` with item sizes 1, 2, 4 or 8 are read;
  `@`, `=` and `<` prefixes are accepted, big-endian formats are rejected.
  More than one dimension is rejected. Unsigned four-byte data is read in
  place; other types are converted and range checked. Bool in an iterable is
  accepted through Python's index protocol.
- `encode_into` requires writable C-contiguous four-byte `I/i/L/l` data,
  after the same prefix removal. Unlike ID input it does not reject extra
  dimensions; capacity is total byte length divided by four. It accepts a
  zero-capacity output and rejects overlap with nonempty text.
- The core allows unaligned ID/output pointers. A PyO3 typed buffer's
  alignment restrictions must not silently narrow that API. Use an owned
  untyped export guard and explicit checked format parsing; avoid creating
  `&[u32]` or `&mut [u32]` from an unaligned pointer.
- All acquired exports must be released on success and every error path,
  after reattachment. A held export pins storage against resize; it does not
  guarantee that another thread cannot change its contents.

PyO3's typed buffer accessors check alignment and return cell-based views
because Python calls can mutate buffer contents. This supports using a
dedicated buffer boundary instead of blanket slice extraction.
[PyO3 buffer documentation](https://docs.rs/pyo3/latest/pyo3/buffer/struct.PyBuffer.html).

Detached access to arbitrary mutable Python buffers deserves its own design
gate. Immutable str/bytes can retain the zero-copy fast path with owned Python
references kept alive. For other inputs, an immutable snapshot taken before
detach provides a sound default. For arbitrary writable `encode_into`
exports, holding the GIL during direct writes or staging then copying back
while attached avoids assuming Python-level exclusive access. These choices
preserve values and buffer acceptance but affect the published zero-copy/GIL
performance behavior, so they must be measured and documented before shipping.
Do not claim full preservation of that performance contract until the chosen
buffer strategy is verified. A read-only memoryview can still alias mutable
storage; its flag alone is not an immutability proof. Access by external native
writers requires an explicit synchronization contract in any implementation.

An Encoding owns a strong Tokenizer reference, padded ID list, immutable text
snapshot and optional cached token-string list. A stream owns a strong
Tokenizer reference, native 64-byte state and an optional external hold.
To grow its hold, allocate a new block, call `toks_stream_hold` while the old
block is still alive, then free the old block. Reallocating/freeing first would
invalidate the source the core must copy. `TOKS_E_LIMIT` from an oversized
byte-fallback run triggers one growth and retry of the same IDs. Flush retains
the hold for reuse. Preserve native atomic error behavior.

## Reuse the Python vocabulary layer

`_vocab.py` reads the original JSON on first demand and verifies SHA-256
against the loaded context. It reconstructs the written vocabulary, added
token precedence and normalized forms; core decoded bytes cannot recover
those spellings. Keeping it preserves changed-file detection and the current
normalizer/refusal semantics.

Retain these helpers unchanged during native migration:

| Helpers | Behavior retained |
|---|---|
| `load`, `_read`, `_normalize`, `_templates` | Source verification, vocabulary duplicates/added rules, selected normalizers, ambiguous template readings |
| `AddedToken`, `added_tokens_decoder` | Defaults, six attributes, repr/str/equality/hash/pickle and ID order |
| `tokens`, `_text_tokens`, `_strip_tokens` | Template and pad spelling, matched strip whitespace, model-vs-added ambiguity, explicit unsupported cases |
| `source_is_json`, `tiktoken_view` | JSON specials versus all tiktoken-model specials; nonspecial versus none as ordinary mode |
| `disallowed`, `encode_subset` | Exact tiktoken error wording and subset splitting |

The deliberate unsupported cases in `Encoding.tokens` are part of the
current API: unknown Unigram text, ambiguous model/added IDs, unrecoverable
strip matches and multiple template interpretations must not be guessed.
Python's own `unicodedata` is already used in this layer; replacing it with
different Rust Unicode tables is a separate semantic change.

These helpers reenter the native object: token construction calls `_vocab`
and sometimes `pieces`, while tiktoken subset encoding calls
`encode_ordinary`. Lazy initialization also calls Python file IO/imports,
which may switch threads. The C code therefore checks the cache again after
computing a value. Preserve that double-check behavior without holding a
Rust mutex or exclusive pyclass borrow across Python calls.

## Concrete Rust design

1. Add `python/rust/Cargo.toml` and `python/rust/src/` as a separate workspace
   member. Package name `toks-python`, library name `_toks`, crate type
   `cdylib`, dependency on `toks = { path = "../../rust" }`. Pin a PyO3 release
   verified by the compatibility spike and commit the lockfile. Keep PyO3
   out of the core crate and its benchmark dependencies.
2. Divide the adapter into `lib` (registration/errors), `tokenizer`
   (factories/properties/pickle), `buffers`, `slots`, `encode`, `decode`,
   `encoding`, and `stream`. Initially call the existing Rust ABI functions
   through a small internal wrapper; do not duplicate tokenizer algorithms.
3. Create `OwnedContext` with exactly one unload in Drop and an immutable
   context pointer plus cached info/template. `Send`/`Sync` must be justified
   by the core's read-only post-load contract. An owned slot lease is the only
   mutable object sent into each detached core call. Pure Rust pool metadata
   can use a short mutex; release it before detaching and before Python calls.
4. Expose Tokenizer with `#[pyclass(subclass, module = "toks")]`. Use shared
   method receivers with short interior-mutation scopes, never `&mut self`
   across `Python::detach`. Cached Python references remain owned `Py<T>`
   handles and are bound only while attached. No borrowed Python reference or
   raw pointer may outlive its explicit owner/export guard.
5. Prototype subclass-preserving factories first. A private nonconstructible
   `LoadTicket` owns one completed context. A guarded `#[new]` accepts only
   that ticket and consumes it once. Factory methods call the base type's
   `Tokenizer.__new__(cls, ticket)` directly, so PyO3's generated constructor
   receives the requested subtype without invoking user-overridden
   `__new__`/`__init__`. Do not use `Py::new(Tokenizer { ... })`, which would
   return the base type, or `cls(...)`, which invokes subclass initialization.
   Test this strategy before committing the rest of the adapter. It is a
   proposed use of the public constructor machinery, not a tested result.
6. Give Encoding and DecodeStream no public constructors. Keep their
   Tokenizer owners alive, preserve copy-on-access lists, and implement GC
   traversal/clearing for Python references where cycles through subclasses
   or retained callbacks can occur. Stream methods stay attached initially;
   mutable stream state must not remain borrowed while input `__index__`
   callbacks can reenter Python.
7. Preserve the lazy int cache explicitly. Blind `Vec<u32>` to Python-list
   conversion allocates Python integers per result and would discard a major
   existing optimization. Cache exact Python int objects by value below
   `n_ids`, clone their references into fresh lists, and keep every output
   list independent. Avoid a cache lock across Python callbacks.
8. Centralize Python conversions and exception construction. Distinguish
   omitted keywords from None, call Python truth/index protocols where the
   old code does, and avoid automatic PyO3 extraction that rejects accepted
   iterables or changes error categories. Preserve private helper signatures
   and module/type names so `_vocab.py` and old pickles continue to work.

PyO3 class methods receive the actual subclass type, but low-level subtype
initialization in PyO3's implementation is private. The factory proposal above
deliberately uses the generated constructor rather than copying private layout
code. [PyO3 classes](https://pyo3.rs/main/class.html),
[initializer source](https://raw.githubusercontent.com/PyO3/pyo3/main/src/pyclass_init.rs).

Declare `#[pymodule(gil_used = true)]` explicitly for the initial port. Recent
PyO3 releases default to free-threaded support, whereas this adapter relies
on the GIL around Python caches and buffers. Use `Python::detach` only for
work that needs no Python API. Do not advertise free-threaded support before
auditing the entire buffer/cache/stream design. PyO3 warns that exclusive
pyclass borrows can fail during concurrent or reentrant access.
[PyO3 threading guidance](https://pyo3.rs/main/free-threading.html).

The workspace currently sets release `panic = "abort"`. That means a PyO3
panic boundary cannot prevent a process abort. Use fallible allocation and
normal `PyResult` paths throughout; eliminate adapter unwrap/expect paths.
Evaluate a dedicated profile inheriting release with `panic = "unwind"` for
the Python extension so unexpected Rust panics can be contained by its
boundary, without changing the standalone core benchmark profile. Validate
the extension under the actual selected profile, including its core dependency.

## Packaging plan

Keep setuptools for the first migration. `setup.py` already reads the version
from `include/toks.h`, copies four license/notice files and configures package
data. Replace its C `Extension` and Make-based `BuildExt` with a
`setuptools-rust` `RustExtension("toks._toks", ...)` targeting
`python/rust/Cargo.toml`. Add the pinned build dependency to `pyproject.toml`
and configure the selected Rust profile. Remove `_toks.c` from production
compilation; retain it only as an explicitly frozen comparison source if needed.

This preserves `uv build --wheel python`, the package layout and the existing
wheel build/test script. The main initial wheel matrix remains CPython
3.10–3.14, Linux arm64/x86-64 and macOS arm64. Use per-version wheels initially:
limited-API Python 3.10 omits the buffer API, and changing to abi3 during this
port adds unnecessary constraints. Current PyO3 packaging guidance supports
setuptools-rust and distinguishes extension builds from embedding builds.
[PyO3 distribution guide](https://pyo3.rs/main/building-and-distribution.html).

Build controls that must survive the migration:

- Header, core `toks_version`, Cargo package and wheel metadata versions must
  agree. Keep license files, `_toks.pyi`, `py.typed`, `_vocab.py` and
  `__init__.py` in the wheel; update obsolete C-only descriptive text.
- Ensure source distributions include the sibling core crate, generated
  Rust tables, assembly/includes and workspace files. A wheel built from the
  checkout alone does not prove a source distribution can rebuild it.
- Preserve the macOS deployment target and verify Linux glibc requirements
  with auditwheel; do not assume the Rust standard library keeps the old
  manylinux floor. Inspect the final wheel's dynamic dependencies.
- The old extension exports only `PyInit__toks`. Linking a Rust core with
  public unmangled ABI symbols may export extra `toks_*` symbols; inspect the
  final dynamic symbol table and apply suitable extension export controls.
- Keep baseline ISA compilation plus runtime dispatch for shipped wheels.
  Host-specific `target-cpu=native` flags belong in experiments, not portable
  wheel builds. The core's assembly build remains responsible for platform ABI.
- Keep the SHA-named wheel-test directories in `python/build.sh`: they prevent
  uv from testing an older wheel with the same name. Record the loaded
  `toks.__file__` and native module path in test receipts.

Maturin is a viable later simplification: mixed packages support
`module-name = "toks._toks"` and inclusion of the existing Python sources.
Changing package backend is not required to replace the C adapter.
[Maturin project layout](https://www.maturin.rs/project_layout.html).

## Verification and order of work

The existing tests import the installed wheel, with the repository's
`python/` appended only for the oracle. `python/build.sh` runs API/thread tests
on each supported CPython; by default 3.13 also runs the full suite. The
oracles are pinned to `tokenizers==0.23.2`, `tiktoken==0.14.0` and
`transformers==5.18.0`. Missing model files or optional oracle packages cause
skips, so count them explicitly rather than accepting a green summary alone.

| Existing suite | What it proves |
|---|---|
| `test_api.py` | Load forms/errors, tiers/cache budgets, input conversions, short/overlapping outputs, modes, limits, decoding, lookups, stream hold growth, Encoding masks/tokens, batch padding, tiktoken interface, pickle, subclass and Python-allocation leak checks |
| `test_threads.py` | Eight threads share one gpt2/llama3 Tokenizer across encode/batch/pieces/decode/into; four threads repeatedly load and unload |
| `test_parity.py` | Six primary models, all modes/postprocess combinations, pieces/decode/stream/vocab, sampled case sets and the target ledger; differences are tallied explicitly |
| `test_surface.py` | All loadable fixture/cache JSONs, field-by-field Encoding and batch results, AddedToken, known refusal cases, tiktoken features and Kimi/Qwen wrappers |

Useful targeted additions for a native-language replacement are:

- Factory subtype preservation with custom `__init__`, `__new__`, attributes
  and all three load forms; direct construction must still fail. Old C-wheel
  pickles must load in the Rust wheel and preserve source-hash checks.
- Reentrant `__index__`, `__bool__`, iterators and `_vocab` helper callbacks;
  mutation of the original list during ID conversion must not invalidate it.
- Concurrent first access to vocab/tiktoken/token-string caches, attached and
  detached operations overlapping, and Encoding/Stream objects surviving
  deletion of the user's last explicit Tokenizer reference.
- The complete buffer-format matrix, misalignment, strided/multidimensional
  buffers, zero-length views, mutable-source snapshots, rejected big-endian
  inputs and release-on-error. Validate the chosen mutable-buffer/GIL policy.
- Stream hold growth and failed push retry, repeated flush and reuse, plus
  failure cleanup with caller-owned hold addresses kept valid.
- Omitted-versus-None special arguments, Python truth/index objects and
  exception-category/message parity. Preserve explicit unsupported token
  string cases, not just successful IDs.
- Native allocation accounting or RSS checks in addition to tracemalloc:
  Rust's allocator is not necessarily visible to Python's tracer. Verify pool
  retention limits and destruction while outputs still hold owners.

Implement in this order: construction/module/error/pickle spike; buffer and
slot primitives; basic encode/decode/into; Encoding and batch padding;
existing Python helper bridges and tiktoken behavior; streams; then complete
packaging, CPython/ISA matrix and installed-wheel differential tests. A frozen
C-adapter wheel and Rust-adapter wheel should be compared in separate
environments against the same fixture bytes, including failure results.

Only after semantic and lifetime checks pass should adapter overhead be
measured: short strings, long strings, encode_into, batches, large ID decode,
integer-list warmup and repeated token metadata access. Those measurements
are separate from the core C-versus-Rust benchmark. Reused Python ints, UTF-8
pointers and scratch caches must not disappear unnoticed behind a faster
core. No benchmark result is claimed by this plan.
