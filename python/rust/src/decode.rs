use crate::{
    args::{parse, truth},
    buffers, core_error, lock, native_error, Tokenizer,
};
use pyo3::{
    class::gc::{PyTraverseError, PyVisit},
    exceptions::{PyKeyError, PyTypeError},
    prelude::*,
    types::{PyBytes, PyDict, PyInt, PyList, PyString, PyTuple},
};
use std::sync::Mutex;
use toks::DecodeFlags as D;
impl Tokenizer {
    fn ids_error(&self, py: Python<'_>, code: i64, message: &str, ids: &[u32], raw: bool) -> PyErr {
        if code == -7 {
            if let Some((at, id)) = ids
                .iter()
                .enumerate()
                .find(|(_, id)| **id >= self.core.info().n_ids)
            {
                if raw {
                    return PyKeyError::new_err(format!("Invalid token for decoding: {id}"));
                }
                return native_error(
                    py,
                    code,
                    &format!(
                        "decode: id {id} at index {at} (n_ids {})",
                        self.core.info().n_ids
                    ),
                );
            }
        }
        native_error(py, code, message)
    }
    fn decode_one<'py>(&self, ids: &Bound<'py, PyAny>, flags: D) -> PyResult<Bound<'py, PyAny>> {
        let py = ids.py();
        let ids = buffers::ids(ids)?;
        self.with_slot(py, 0, |s| {
            let mut run = || self.core.decode_to(&ids, flags, &mut s.bytes);
            let result = if ids.len() >= 8192 {
                py.detach(run)
            } else {
                run()
            };
            result
                .map_err(|e| self.ids_error(py, e.code, &e.message, &ids, flags.bits() & 2 != 0))?;
            Ok(if flags.bits() & 2 != 0 {
                PyBytes::new(py, &s.bytes).into_any()
            } else {
                PyString::new(py, &String::from_utf8_lossy(&s.bytes)).into_any()
            })
        })
    }
    fn decode_many<'py>(
        &self,
        sequences: &Bound<'py, PyAny>,
        flags: D,
    ) -> PyResult<Bound<'py, PyList>> {
        if sequences.is_instance_of::<PyString>() || sequences.is_instance_of::<PyBytes>() {
            return Err(PyTypeError::new_err(
                "decode_batch takes a sequence of ID sequences",
            ));
        }
        let out = PyList::empty(sequences.py());
        for seq in buffers::tuple(sequences)?.iter() {
            out.append(self.decode_one(&seq, flags)?)?;
        }
        Ok(out)
    }
}
#[pymethods]
impl Tokenizer {
    #[pyo3(signature=(*args,**kwargs))]
    fn decode<'py>(
        &self,
        args: &Bound<'py, PyTuple>,
        kwargs: Option<&Bound<'py, PyDict>>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let a = parse(args, kwargs, &["ids", "skip_special_tokens"], 2, 1)?;
        self.decode_one(
            a.required(0)?,
            if truth(a.get(1), true)? {
                D::SKIP_SPECIAL
            } else {
                D::ALL
            },
        )
    }
    #[pyo3(signature=(*args,**kwargs))]
    fn decode_batch<'py>(
        &self,
        args: &Bound<'py, PyTuple>,
        kwargs: Option<&Bound<'py, PyDict>>,
    ) -> PyResult<Bound<'py, PyList>> {
        let a = parse(args, kwargs, &["sequences", "skip_special_tokens"], 2, 1)?;
        self.decode_many(
            a.required(0)?,
            if truth(a.get(1), true)? {
                D::SKIP_SPECIAL
            } else {
                D::ALL
            },
        )
    }
    #[pyo3(signature=(ids,/))]
    fn decode_bytes<'py>(&self, ids: &Bound<'py, PyAny>) -> PyResult<Bound<'py, PyAny>> {
        self.decode_one(ids, D::RAW)
    }
    #[pyo3(signature=(*args,**kwargs))]
    fn decode_bytes_batch<'py>(
        &self,
        args: &Bound<'py, PyTuple>,
        kwargs: Option<&Bound<'py, PyDict>>,
    ) -> PyResult<Bound<'py, PyList>> {
        let a = parse(args, kwargs, &["batch", "num_threads"], 1, 1)?;
        self.decode_many(a.required(0)?, D::RAW)
    }
    #[pyo3(signature=(*args,**kwargs))]
    fn decode_stream<'py>(
        slf: &Bound<'py, Self>,
        args: &Bound<'py, PyTuple>,
        kwargs: Option<&Bound<'py, PyDict>>,
    ) -> PyResult<Bound<'py, DecodeStream>> {
        let a = parse(args, kwargs, &["skip_special_tokens"], 1, 0)?;
        let state = slf
            .get()
            .core
            .decoder(if truth(a.get(0), false)? {
                D::SKIP_SPECIAL
            } else {
                D::ALL
            })
            .map_err(|e| core_error(slf.py(), e))?;
        Bound::new(
            slf.py(),
            DecodeStream {
                tok: slf.clone().unbind(),
                state: Mutex::new(state),
            },
        )
    }
}
#[pyclass(frozen, module = "toks")]
pub struct DecodeStream {
    tok: Py<Tokenizer>,
    state: Mutex<toks::Decoder>,
}
#[pymethods]
impl DecodeStream {
    #[pyo3(signature=(ids,/))]
    fn push<'py>(&self, ids: &Bound<'py, PyAny>) -> PyResult<Bound<'py, PyString>> {
        let py = ids.py();
        // Conversions may reenter this stream. Finish them before borrowing its
        // state; the native operation itself makes no Python calls.
        let ids = if ids.is_instance_of::<PyInt>() {
            vec![ids.extract::<u32>()?]
        } else {
            buffers::ids(ids)?
        };
        let result = lock(&self.state).push(&ids);
        let bytes = result.map_err(|e| {
            self.tok
                .get()
                .ids_error(py, e.code, &e.message, &ids, false)
        })?;
        Ok(PyString::new(py, &String::from_utf8_lossy(&bytes)))
    }
    fn flush<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyString>> {
        let result = lock(&self.state).flush();
        let bytes = result.map_err(|e| core_error(py, e))?;
        Ok(PyString::new(py, &String::from_utf8_lossy(&bytes)))
    }
    fn __repr__(&self) -> String {
        format!("<toks.DecodeStream {}>", self.tok.get().core.name())
    }
    fn __traverse__(&self, visit: PyVisit<'_>) -> Result<(), PyTraverseError> {
        visit.call(&self.tok)
    }
}
