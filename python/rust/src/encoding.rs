use crate::{lock, Tokenizer};
use pyo3::{
    class::gc::{PyTraverseError, PyVisit},
    prelude::*,
    types::PyList,
};
use std::sync::Mutex;

#[pyclass(frozen, module = "toks")]
pub struct Encoding {
    tok: Py<Tokenizer>,
    ids: Py<PyList>,
    text: Py<PyAny>,
    flags: u32,
    pre: usize,
    suf: usize,
    pad: usize,
    left: bool,
    tokens: Mutex<Option<Py<PyList>>>,
}
impl Encoding {
    pub fn create<'py>(
        tok: &Bound<'py, Tokenizer>,
        ids: &[u32],
        pad: usize,
        flags: u32,
        text: Py<PyAny>,
    ) -> PyResult<Bound<'py, Self>> {
        let this = tok.get();
        let post = flags & 4 == 0 && ids.len() >= this.template.ids.len();
        let values = this.padded_list(tok.py(), ids, pad)?;
        Bound::new(
            tok.py(),
            Self {
                tok: tok.clone().unbind(),
                ids: values.unbind(),
                text,
                flags,
                pre: if post { this.template.n_prefix } else { 0 },
                suf: if post {
                    this.template.ids.len() - this.template.n_prefix
                } else {
                    0
                },
                pad,
                left: this.core.info().pad_left != 0,
                tokens: Mutex::new(None),
            },
        )
    }
    fn mask<'py>(&self, py: Python<'py>, kind: u8) -> PyResult<Bound<'py, PyList>> {
        let t = self.tok.get();
        let info = t.core.info();
        let n = self.ids.bind(py).len();
        let b0 = if self.left { self.pad } else { 0 };
        let b1 = if self.left { n } else { n - self.pad };
        let s0 = b0 + self.pre;
        let s1 = b1 - self.suf;
        PyList::new(
            py,
            (0..n).map(|i| {
                if i < b0 || i >= b1 {
                    match kind {
                        0 => info.pad_type_id,
                        1 => 0,
                        _ => 1,
                    }
                } else if i >= s0 && i < s1 {
                    match kind {
                        0 => info.seq_type_id,
                        1 => 1,
                        _ => 0,
                    }
                } else if kind == 0 {
                    let k = if i < s0 {
                        i - b0
                    } else {
                        t.template.n_prefix + i - s1
                    };
                    t.template.type_ids[k]
                } else {
                    1
                }
            }),
        )
    }
}
#[pymethods]
impl Encoding {
    #[getter]
    fn ids<'py>(&self, py: Python<'py>) -> Bound<'py, PyList> {
        self.ids.bind(py).get_slice(0, self.ids.bind(py).len())
    }
    #[getter]
    fn type_ids<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyList>> {
        self.mask(py, 0)
    }
    #[getter]
    fn attention_mask<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyList>> {
        self.mask(py, 1)
    }
    #[getter]
    fn special_tokens_mask<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyList>> {
        self.mask(py, 2)
    }
    #[getter]
    fn overflowing<'py>(&self, py: Python<'py>) -> Bound<'py, PyList> {
        PyList::empty(py)
    }
    #[getter]
    fn n_sequences(&self) -> u8 {
        1
    }
    #[getter]
    fn _text(&self, py: Python<'_>) -> Py<PyAny> {
        self.text.clone_ref(py)
    }
    #[getter]
    fn _layout(&self) -> (usize, usize, usize, u8, u32) {
        (
            self.pre,
            self.suf,
            self.pad,
            u8::from(self.left),
            self.flags,
        )
    }
    #[getter]
    fn tokens<'py>(slf: &Bound<'py, Self>) -> PyResult<Bound<'py, PyList>> {
        let this = slf.get();
        let py = slf.py();
        if let Some(v) = lock(&this.tokens).as_ref().map(|v| v.clone_ref(py)) {
            return Ok(v.bind(py).get_slice(0, v.bind(py).len()));
        }
        let value = py
            .import("toks._vocab")?
            .call_method1("tokens", (this.tok.bind(py), slf))?
            .cast_into::<PyList>()?;
        if value.len() != this.ids.bind(py).len() {
            return Err(pyo3::exceptions::PyRuntimeError::new_err(
                "tokens must return one string per id",
            ));
        }
        let mut cache = lock(&this.tokens);
        let stored = cache.get_or_insert_with(|| value.unbind());
        let stored = stored.clone_ref(py);
        drop(cache);
        Ok(stored.bind(py).get_slice(0, stored.bind(py).len()))
    }
    fn __len__(&self, py: Python<'_>) -> usize {
        self.ids.bind(py).len()
    }
    fn __repr__(&self, py: Python<'_>) -> String {
        format!("Encoding(num_tokens={}, attributes=[ids, type_ids, tokens, attention_mask, special_tokens_mask, overflowing])",self.ids.bind(py).len())
    }
    fn __traverse__(&self, visit: PyVisit<'_>) -> Result<(), PyTraverseError> {
        visit.call(&self.tok)?;
        visit.call(&self.ids)?;
        visit.call(&self.text)?;
        if let Some(v) = &*lock(&self.tokens) {
            visit.call(v)?;
        }
        Ok(())
    }
    fn __clear__(&self) {
        let old = lock(&self.tokens).take();
        drop(old);
    }
}
