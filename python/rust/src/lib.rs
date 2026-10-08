//! Rust implementation of the established CPython adapter.
use pyo3::{
    create_exception,
    exceptions::{PyException, PyMemoryError, PyTypeError},
    prelude::*,
    types::{PyBytes, PyDict, PyList, PyTuple},
};
use std::sync::{atomic::AtomicBool, Mutex, MutexGuard};
mod args;
mod buffers;
mod decode;
mod encode;
mod encoding;
mod tokenizer;
create_exception!(_toks, Error, PyException);
fn error_name(code: i64) -> &'static str {
    match code {
        -1 => "TOKS_E_OPEN",
        -2 => "TOKS_E_FORMAT",
        -3 => "TOKS_E_UNSUPPORTED",
        -5 => "TOKS_E_TIER",
        -6 => "TOKS_E_SCRATCH",
        -7 => "TOKS_E_ID",
        -8 => "TOKS_E_CAP",
        -9 => "TOKS_E_LIMIT",
        -10 => "TOKS_E_ARG",
        -11 => "TOKS_E_NOMEM",
        _ => "TOKS_E_UNKNOWN",
    }
}
#[pyfunction]
fn _error(py: Python<'_>, code: i64, message: &str) -> PyResult<Py<PyAny>> {
    let name = error_name(code);
    let err = Error::new_err(format!("{name}: {message}"));
    let value = err.value(py);
    value.setattr("code", code)?;
    value.setattr("name", name)?;
    Ok(value.clone().into_any().unbind())
}
fn native_error(py: Python<'_>, code: i64, message: &str) -> PyErr {
    if code == -11 {
        return PyMemoryError::new_err(message.to_owned());
    }
    match _error(py, code, message) {
        Ok(e) => PyErr::from_value(e.into_bound(py)),
        Err(e) => e,
    }
}
fn core_error(py: Python<'_>, e: toks::Error) -> PyErr {
    native_error(py, e.code, &e.message)
}
fn lock<T>(m: &Mutex<T>) -> MutexGuard<'_, T> {
    m.lock().unwrap_or_else(|p| p.into_inner())
}
#[derive(Default)]
struct Cache {
    vocab: Option<Py<PyTuple>>,
    tt: Option<Py<PyTuple>>,
    ints: Vec<Option<Py<PyAny>>>,
}
struct Slot {
    encoder: toks::Encoder,
    ids: Vec<u32>,
    bytes: Vec<u8>,
}
#[pyclass]
struct LoadTicket {
    value: Option<Tokenizer>,
}
#[pyclass(subclass, frozen, module = "toks")]
struct Tokenizer {
    core: toks::Tokenizer,
    source: Py<PyAny>,
    kind: u8,
    tier: toks::Tier,
    options: toks::ScratchOptions,
    template: toks::Template,
    encode_special: AtomicBool,
    cache: Mutex<Cache>,
    pool: Mutex<Vec<Slot>>,
}
impl Tokenizer {
    fn with_slot<T>(
        &self,
        py: Python<'_>,
        len: usize,
        f: impl FnOnce(&mut Slot) -> PyResult<T>,
    ) -> PyResult<T> {
        let idle = { lock(&self.pool).pop() };
        let mut slot = match idle {
            Some(slot) => slot,
            None => Slot {
                encoder: self
                    .core
                    .encoder(self.options)
                    .map_err(|e| core_error(py, e))?,
                ids: Vec::new(),
                bytes: Vec::new(),
            },
        };
        let result = f(&mut slot);
        if len > 1 << 20 {
            slot.encoder.clear_cache();
        }
        if slot.ids.capacity() > 1 << 20 {
            slot.ids = Vec::new();
        }
        if slot.bytes.capacity() > 4 << 20 {
            slot.bytes = Vec::new();
        }
        lock(&self.pool).push(slot);
        result
    }
    fn int_list<'py>(&self, py: Python<'py>, ids: &[u32]) -> PyResult<Bound<'py, PyList>> {
        let len = isize::try_from(ids.len()).map_err(|_| PyMemoryError::new_err("ID list"))?;
        // List allocation may run cyclic GC and reenter this tokenizer. Do it
        // before taking the cache lock. The new list is private until every
        // slot is initialized; CPython safely destroys NULL slots on error.
        let out = unsafe { Bound::from_owned_ptr_or_err(py, pyo3::ffi::PyList_New(len))? }
            .cast_into::<PyList>()?;
        {
            let mut cache = lock(&self.cache);
            if cache.ints.is_empty() {
                let n = self.core.info().n_ids as usize;
                cache
                    .ints
                    .try_reserve_exact(n)
                    .map_err(|_| PyMemoryError::new_err("ID cache"))?;
                cache.ints.resize_with(n, || None);
            }
            for (i, &id) in ids.iter().enumerate() {
                // Exact integer construction cannot invoke Python callbacks.
                let value = if let Some(entry) = cache.ints.get_mut(id as usize) {
                    if entry.is_none() {
                        *entry = Some(id.into_pyobject(py)?.into_any().unbind());
                    }
                    entry
                        .as_ref()
                        .map(|v| v.clone_ref(py))
                        .ok_or_else(|| PyTypeError::new_err("ID cache"))?
                } else {
                    id.into_pyobject(py)?.into_any().unbind()
                };
                // The list owns this reference; no temporary Rust vector or
                // second reference-count pass is needed.
                unsafe { pyo3::ffi::PyList_SET_ITEM(out.as_ptr(), i as isize, value.into_ptr()) };
            }
        }
        Ok(out)
    }
    fn vocab<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyTuple>> {
        if let Some(v) = lock(&self.cache).vocab.as_ref().map(|v| v.clone_ref(py)) {
            return Ok(v.into_bound(py));
        }
        let value = py
            .import("toks._vocab")?
            .call_method1(
                "load",
                (
                    self.kind,
                    self.source.bind(py),
                    PyBytes::new(py, &self.core.info().source_sha256),
                ),
            )?
            .cast_into::<PyTuple>()?;
        if value.len() != 6 {
            return Err(pyo3::exceptions::PyRuntimeError::new_err(
                "toks._vocab.load must return a 6-tuple",
            ));
        }
        let mut cache = lock(&self.cache);
        let saved = cache.vocab.get_or_insert_with(|| value.unbind());
        Ok(saved.clone_ref(py).into_bound(py))
    }
    fn tt<'py>(slf: &Bound<'py, Self>) -> PyResult<Bound<'py, PyTuple>> {
        let py = slf.py();
        let this = slf.get();
        if let Some(v) = lock(&this.cache).tt.as_ref().map(|v| v.clone_ref(py)) {
            return Ok(v.into_bound(py));
        }
        let entries = PyList::empty(py);
        let mut i = 0;
        while let Some(a) = this.core.added(i) {
            let content = std::str::from_utf8(a.content)
                .map_err(|_| native_error(py, -2, "added content is not UTF-8"))?;
            entries.append((
                a.id,
                content,
                a.flags,
                this.core.id_flags(a.id).map_err(|e| core_error(py, e))?,
            ))?;
            i += 1;
        }
        let value = py
            .import("toks._vocab")?
            .call_method1(
                "tiktoken_view",
                (slf, this.kind, this.source.bind(py), entries),
            )?
            .cast_into::<PyTuple>()?;
        if value.len() != 3 || !value.get_item(0)?.is_instance_of::<PyDict>() {
            return Err(pyo3::exceptions::PyRuntimeError::new_err(
                "invalid tiktoken view",
            ));
        }
        let mut cache = lock(&this.cache);
        let saved = cache.tt.get_or_insert_with(|| value.unbind());
        Ok(saved.clone_ref(py).into_bound(py))
    }
}
#[pymodule(gil_used = true)]
fn _toks(m: &Bound<'_, PyModule>) -> PyResult<()> {
    let py = m.py();
    let error = py.get_type::<Error>();
    error.setattr("__module__", "toks")?;
    m.add("Error", error)?;
    m.add_class::<Tokenizer>()?;
    m.add_class::<encoding::Encoding>()?;
    m.add_class::<decode::DecodeStream>()?;
    m.add_function(wrap_pyfunction!(_error, m)?)?;
    m.add_function(wrap_pyfunction!(tokenizer::_load, m)?)?;
    m.add("ABI", (0, 4))?;
    m.add("__version__", toks::Tokenizer::version())?;
    m.add("MAX_TEXT", 1u64 << 29)?;
    m.add("ID_ADDED", 1)?;
    m.add("ID_SPECIAL", 2)?;
    m.add("ID_BYTE", 4)?;
    Ok(())
}
