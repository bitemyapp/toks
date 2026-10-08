use crate::{
    args::{parse, truth},
    buffers::{Buffer, Text},
    core_error, lock, native_error, Cache, LoadTicket, Tokenizer,
};
use pyo3::{
    class::gc::{PyTraverseError, PyVisit},
    exceptions::{PyKeyError, PyTypeError, PyValueError},
    prelude::*,
    types::{PyBytes, PyDict, PyList, PySet, PyString, PyTuple, PyType},
};
use std::{
    os::unix::ffi::OsStrExt,
    sync::{
        atomic::{AtomicBool, Ordering},
        Mutex,
    },
};
fn tier(o: Option<&Bound<'_, PyAny>>) -> PyResult<toks::Tier> {
    let Some(o) = o.filter(|o| !o.is_none()) else {
        return Ok(toks::Tier::Auto);
    };
    match o.extract::<&str>().ok() {
        Some("auto") => Ok(toks::Tier::Auto),
        Some("scalar") => Ok(toks::Tier::Scalar),
        Some("neon") => Ok(toks::Tier::Neon),
        Some("avx2") => Ok(toks::Tier::Avx2),
        Some("avx512") => Ok(toks::Tier::Avx512),
        _ => Err(PyValueError::new_err(
            "tier must be None, 'auto', 'scalar', 'neon', 'avx2' or 'avx512'",
        )),
    }
}
pub fn tier_name(n: u32) -> &'static str {
    ["auto", "scalar", "neon", "avx2", "avx512"]
        .get(n as usize)
        .copied()
        .unwrap_or("unknown")
}
fn algo(n: u32) -> &'static str {
    [
        "unknown",
        "bpe_bytelevel",
        "bpe_spm",
        "unigram",
        "wordpiece",
    ]
    .get(n as usize)
    .copied()
    .unwrap_or("unknown")
}
fn hex(h: &[u8; 32]) -> Option<String> {
    h.iter()
        .any(|&b| b != 0)
        .then(|| h.iter().map(|b| format!("{b:02x}")).collect())
}
impl Tokenizer {
    fn allocate<'py>(
        cls: &Bound<'py, PyType>,
        core: toks::Tokenizer,
        source: Py<PyAny>,
        kind: u8,
        tier: toks::Tier,
        cache_mib: u8,
    ) -> PyResult<Bound<'py, PyAny>> {
        let py = cls.py();
        let template = core.template().map_err(|e| core_error(py, e))?;
        let ticket = Py::new(
            py,
            LoadTicket {
                value: Some(Self {
                    core,
                    source,
                    kind,
                    tier,
                    options: toks::ScratchOptions {
                        memo_mib: None,
                        cache_mib,
                    },
                    template,
                    encode_special: AtomicBool::new(false),
                    cache: Mutex::new(Cache::default()),
                    pool: Mutex::new(Vec::new()),
                }),
            },
        )?;
        // Preserve cls without invoking user-defined subtype constructors.
        py.get_type::<Self>()
            .getattr("__new__")?
            .call1((cls, ticket))
    }
    fn load_source<'py>(
        cls: &Bound<'py, PyType>,
        source: &Bound<'py, PyAny>,
        kind: u8,
        tier: toks::Tier,
        cache: u8,
    ) -> PyResult<Bound<'py, PyAny>> {
        let py = cls.py();
        if kind == 0 {
            let src = py.import("os")?.call_method1("fspath", (source,))?;
            let fs = py
                .import("os")?
                .call_method1("fsencode", (&src,))?
                .cast_into::<PyBytes>()?;
            let path = std::path::Path::new(std::ffi::OsStr::from_bytes(fs.as_bytes()));
            if fs.as_bytes().contains(&0) {
                return Err(PyValueError::new_err("embedded null byte"));
            }
            let core = py
                .detach(|| toks::Tokenizer::from_file(path, tier))
                .map_err(|e| {
                    native_error(
                        py,
                        e.code,
                        &format!(
                            "{} (loading {})",
                            e.message,
                            src.repr().map(|s| s.to_string()).unwrap_or_default()
                        ),
                    )
                })?;
            Self::allocate(cls, core, src.unbind(), kind, tier, cache)
        } else {
            let src = if kind == 2 {
                source.cast::<PyString>()?.clone().into_any()
            } else if source.is_instance_of::<PyBytes>() {
                source.clone()
            } else {
                unsafe {
                    Bound::from_owned_ptr_or_err(
                        py,
                        pyo3::ffi::PyBytes_FromObject(source.as_ptr()),
                    )?
                }
            };
            let tx = Text::get(&src)?;
            let core = py
                .detach(|| toks::Tokenizer::from_bytes(tx.bytes(), tier))
                .map_err(|e| core_error(py, e))?;
            Self::allocate(cls, core, src.unbind(), kind, tier, cache)
        }
    }
}
#[pymethods]
impl Tokenizer {
    #[new]
    #[pyo3(signature=(ticket=None))]
    fn new(ticket: Option<PyRefMut<'_, LoadTicket>>) -> PyResult<Self> {
        ticket
            .and_then(|mut t| t.value.take())
            .ok_or_else(|| PyTypeError::new_err("use Tokenizer.from_file, from_str or from_buffer"))
    }
    #[classmethod]
    #[pyo3(signature=(*args,**kwargs))]
    fn from_file<'py>(
        cls: &Bound<'py, PyType>,
        args: &Bound<'py, PyTuple>,
        kwargs: Option<&Bound<'py, PyDict>>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let a = parse(args, kwargs, &["path", "tier", "cache_mib"], 1, 1)?;
        let tier = tier(a.get(1))?;
        let cache = match a.get(2).filter(|o| !o.is_none()) {
            Some(v) => v.extract::<i64>()?,
            None => 0,
        };
        if cache != 0 && (!(4..=128).contains(&cache) || !(cache as u64).is_power_of_two()) {
            return Err(PyValueError::new_err(
                "cache_mib must be 0 or a power of two 4..128",
            ));
        }
        Self::load_source(cls, a.required(0)?, 0, tier, cache as u8)
    }
    #[classmethod]
    #[pyo3(signature=(*args,**kwargs))]
    fn from_str<'py>(
        cls: &Bound<'py, PyType>,
        args: &Bound<'py, PyTuple>,
        kwargs: Option<&Bound<'py, PyDict>>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let a = parse(args, kwargs, &["json", "tier"], 1, 1)?;
        Self::load_source(cls, a.required(0)?, 2, tier(a.get(1))?, 0)
    }
    #[classmethod]
    #[pyo3(signature=(*args,**kwargs))]
    fn from_buffer<'py>(
        cls: &Bound<'py, PyType>,
        args: &Bound<'py, PyTuple>,
        kwargs: Option<&Bound<'py, PyDict>>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let a = parse(args, kwargs, &["data", "tier"], 1, 1)?;
        Self::load_source(cls, a.required(0)?, 1, tier(a.get(1))?, 0)
    }
    #[getter]
    fn source(&self, py: Python<'_>) -> Py<PyAny> {
        self.source.clone_ref(py)
    }
    #[getter]
    fn n_vocab(&self) -> u32 {
        self.core.info().n_ids
    }
    #[getter]
    fn max_token_value(&self) -> i64 {
        i64::from(self.core.info().n_ids) - 1
    }
    #[getter]
    fn name(&self) -> String {
        self.core.name()
    }
    #[getter]
    fn encode_special_tokens(&self) -> bool {
        self.encode_special.load(Ordering::Relaxed)
    }
    #[setter]
    fn set_encode_special_tokens(&self, value: &Bound<'_, PyAny>) -> PyResult<()> {
        self.encode_special
            .store(value.is_truthy()?, Ordering::Relaxed);
        Ok(())
    }
    #[getter]
    fn eot_token(slf: &Bound<'_, Self>) -> PyResult<Py<PyAny>> {
        Self::tt(slf)?
            .get_item(1)?
            .cast::<PyDict>()?
            .get_item("<|endoftext|>")?
            .map(|v| v.unbind())
            .ok_or_else(|| PyKeyError::new_err("<|endoftext|>"))
    }
    #[getter]
    fn special_tokens_set<'py>(slf: &Bound<'py, Self>) -> PyResult<Bound<'py, PySet>> {
        let d = Self::tt(slf)?.get_item(1)?;
        PySet::new(slf.py(), d.try_iter()?.collect::<PyResult<Vec<_>>>()?)
    }
    fn __repr__(&self) -> String {
        let i = self.core.info();
        format!(
            "<toks.Tokenizer {}: {}, {} ids, tier {}>",
            self.core.name(),
            algo(i.algorithm),
            i.n_ids,
            tier_name(i.tier)
        )
    }
    fn __reduce__<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyTuple>> {
        (
            py.import("toks._toks")?.getattr("_load")?,
            (
                self.kind,
                self.source.bind(py),
                self.tier as u32,
                PyBytes::new(py, &self.core.info().source_sha256),
                self.encode_special.load(Ordering::Relaxed),
            ),
        )
            .into_pyobject(py)
    }
    fn _vocab<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyTuple>> {
        self.vocab(py)
    }
    #[pyo3(signature=(token,/))]
    fn token_to_id<'py>(&self, token: &Bound<'py, PyAny>) -> PyResult<Bound<'py, PyAny>> {
        let py = token.py();
        if token.is_instance_of::<PyString>() {
            return Ok(self
                .vocab(py)?
                .get_item(0)?
                .cast::<PyDict>()?
                .get_item(token)?
                .unwrap_or_else(|| py.None().into_bound(py)));
        }
        let b = Buffer::get(token, pyo3::ffi::PyBUF_SIMPLE)?;
        let bytes = if b.view.len == 0 {
            &[]
        } else {
            unsafe { std::slice::from_raw_parts(b.view.buf.cast::<u8>(), b.view.len as usize) }
        };
        Ok(self.core.token_to_id(bytes).into_pyobject(py)?.into_any())
    }
    #[pyo3(signature=(id,/))]
    fn token_bytes<'py>(&self, py: Python<'py>, id: u32) -> Option<Bound<'py, PyBytes>> {
        self.core.token(id).map(|v| PyBytes::new(py, v))
    }
    #[pyo3(signature=(id,/))]
    fn id_flags(&self, py: Python<'_>, id: u32) -> PyResult<u32> {
        self.core.id_flags(id).map_err(|e| core_error(py, e))
    }
    #[pyo3(signature=(n,/))]
    fn encode_bound(&self, n: u64) -> u64 {
        self.core.encode_bound(n)
    }
    #[pyo3(signature=(id,/))]
    fn id_to_token<'py>(&self, py: Python<'py>, id: u32) -> PyResult<Bound<'py, PyAny>> {
        let result = self
            .vocab(py)?
            .get_item(1)?
            .cast::<PyDict>()?
            .get_item(id)?
            .unwrap_or_else(|| py.None().into_bound(py));
        if unsafe { pyo3::ffi::PyExceptionInstance_Check(result.as_ptr()) } != 0 {
            return Err(PyErr::from_value(result));
        }
        Ok(result)
    }
    #[pyo3(signature=(*args,**kwargs))]
    fn get_vocab<'py>(
        &self,
        args: &Bound<'py, PyTuple>,
        kwargs: Option<&Bound<'py, PyDict>>,
    ) -> PyResult<Bound<'py, PyDict>> {
        let a = parse(args, kwargs, &["with_added_tokens"], 1, 0)?;
        let i = if truth(a.get(0), true)? { 0 } else { 2 };
        self.vocab(args.py())?.get_item(i)?.cast::<PyDict>()?.copy()
    }
    #[pyo3(signature=(*args,**kwargs))]
    fn get_vocab_size<'py>(
        &self,
        args: &Bound<'py, PyTuple>,
        kwargs: Option<&Bound<'py, PyDict>>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let a = parse(args, kwargs, &["with_added_tokens"], 1, 0)?;
        let i = if truth(a.get(0), true)? { 4 } else { 3 };
        self.vocab(args.py())?.get_item(i)
    }
    fn token_byte_values<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyList>> {
        let mut bytes = Vec::new();
        for id in 0..self.core.info().n_ids {
            if self.core.id_flags(id).map_err(|e| core_error(py, e))? & 1 == 0 {
                if let Some(b) = self.core.token(id) {
                    bytes.push(b);
                }
            }
        }
        bytes.sort_unstable();
        PyList::new(py, bytes.into_iter().map(|b| PyBytes::new(py, b)))
    }
    #[pyo3(signature=(*args,**kwargs))]
    fn num_special_tokens_to_add(
        &self,
        args: &Bound<'_, PyTuple>,
        kwargs: Option<&Bound<'_, PyDict>>,
    ) -> PyResult<usize> {
        let a = parse(args, kwargs, &["is_pair"], 1, 0)?;
        if truth(a.get(0), false)? {
            return Err(native_error(
                args.py(),
                -3,
                "pair templates are not supported",
            ));
        }
        Ok(self.template.ids.len())
    }
    fn get_added_tokens_decoder<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let entries = PyList::empty(py);
        let mut index = 0;
        while let Some(a) = self.core.added(index) {
            let content = std::str::from_utf8(a.content)
                .map_err(|_| native_error(py, -2, "added content"))?;
            entries.append((a.id, content, a.flags))?;
            index += 1;
        }
        py.import("toks._vocab")?
            .call_method1("added_tokens_decoder", (entries,))
    }
    fn __traverse__(&self, visit: PyVisit<'_>) -> Result<(), PyTraverseError> {
        visit.call(&self.source)?;
        let cache = lock(&self.cache);
        if let Some(v) = &cache.vocab {
            visit.call(v)?;
        }
        if let Some(v) = &cache.tt {
            visit.call(v)?;
        }
        for v in cache.ints.iter().flatten() {
            visit.call(v)?;
        }
        Ok(())
    }
    fn __clear__(&self) {
        let old = std::mem::take(&mut *lock(&self.cache));
        drop(old);
    }
    fn info<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
        let i = self.core.info();
        let d = PyDict::new(py);
        d.set_item("abi", (i.abi_major, i.abi_minor))?;
        d.set_item("algorithm", algo(i.algorithm))?;
        d.set_item("tier", tier_name(i.tier))?;
        d.set_item("n_ids", i.n_ids)?;
        d.set_item("n_added", i.n_added)?;
        let paths = PyDict::new(py);
        paths.set_item("scan", i.paths & 1 != 0)?;
        paths.set_item("normalize", i.paths & 2 != 0)?;
        d.set_item("paths", paths)?;
        d.set_item("cpu_features", i.cpu_features)?;
        d.set_item("max_text", i.max_text)?;
        d.set_item("control_isolation", i.control_isolation != 0)?;
        d.set_item("source_sha256", hex(&i.source_sha256))?;
        d.set_item("image_sha256", hex(&i.image_sha256))?;
        d.set_item("name", self.core.name())?;
        if i.trunc_on != 0 {
            let t = PyDict::new(py);
            t.set_item("max_length", i.trunc_max)?;
            t.set_item("stride", i.trunc_stride)?;
            d.set_item("truncation", t)?;
        } else {
            d.set_item("truncation", py.None())?;
        }
        if i.pad_on != 0 {
            let p = PyDict::new(py);
            p.set_item(
                "strategy",
                if i.pad_fixed != 0 {
                    "fixed"
                } else {
                    "batch_longest"
                },
            )?;
            p.set_item("length", i.pad_len)?;
            p.set_item("pad_to_multiple_of", i.pad_multiple)?;
            p.set_item("pad_id", i.pad_id)?;
            p.set_item("pad_type_id", i.pad_type_id)?;
            p.set_item("direction", if i.pad_left != 0 { "left" } else { "right" })?;
            d.set_item("padding", p)?;
        } else {
            d.set_item("padding", py.None())?;
        }
        let t = PyDict::new(py);
        t.set_item("prefix", i.n_template_prefix)?;
        t.set_item("suffix", i.n_template_suffix)?;
        t.set_item("seq_type_id", i.seq_type_id)?;
        d.set_item("template", t)?;
        Ok(d)
    }
}
#[pyfunction]
pub fn _load<'py>(
    py: Python<'py>,
    kind: u8,
    source: &Bound<'py, PyAny>,
    tier: u32,
    sha256: &Bound<'py, PyBytes>,
    encode_special_tokens: &Bound<'py, PyAny>,
) -> PyResult<Bound<'py, PyAny>> {
    if kind > 2
        || kind == 2 && !source.is_instance_of::<PyString>()
        || kind == 1 && !source.is_instance_of::<PyBytes>()
    {
        return Err(PyValueError::new_err("_load: unknown source kind"));
    }
    let tier = match tier {
        1 => toks::Tier::Scalar,
        2 => toks::Tier::Neon,
        3 => toks::Tier::Avx2,
        4 => toks::Tier::Avx512,
        _ => toks::Tier::Auto,
    };
    let result = Tokenizer::load_source(&py.get_type::<Tokenizer>(), source, kind, tier, 0)?;
    let t = result.cast::<Tokenizer>()?.get();
    if sha256.as_bytes() != t.core.info().source_sha256 {
        return Err(native_error(
            py,
            -2,
            "source changed since the tokenizer was pickled (source sha256 differs)",
        ));
    }
    t.encode_special
        .store(encode_special_tokens.is_truthy()?, Ordering::Relaxed);
    Ok(result)
}
