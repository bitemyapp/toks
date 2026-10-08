//! Python-owned memory never becomes an exclusive Rust slice. Immutable text
//! keeps its owner alive across detach; other inputs are snapshotted while
//! attached. Writable output is copied from Rust staging while attached.
use crate::native_error;
use pyo3::{
    exceptions::{PyOverflowError, PyTypeError, PyValueError},
    ffi,
    prelude::*,
    types::{PyByteArray, PyBytes, PyString, PyTuple},
};
use std::{ffi::CStr, ptr};

pub struct Buffer<'py> {
    pub view: ffi::Py_buffer,
    _py: Python<'py>,
}
impl<'py> Buffer<'py> {
    pub fn get(o: &Bound<'py, PyAny>, flags: i32) -> PyResult<Self> {
        let mut view = unsafe { std::mem::zeroed() };
        if unsafe { ffi::PyObject_GetBuffer(o.as_ptr(), &mut view, flags) } != 0 {
            return Err(PyErr::fetch(o.py()));
        }
        Ok(Self { view, _py: o.py() })
    }
    pub fn format(&self) -> &[u8] {
        let f = if self.view.format.is_null() {
            b"B".as_slice()
        } else {
            unsafe { CStr::from_ptr(self.view.format).to_bytes() }
        };
        if f.first().is_some_and(|c| b"@=<".contains(c)) {
            &f[1..]
        } else {
            f
        }
    }
    pub fn output(o: &Bound<'py, PyAny>) -> PyResult<Self> {
        let b = Self::get(
            o,
            ffi::PyBUF_WRITABLE | ffi::PyBUF_FORMAT | ffi::PyBUF_C_CONTIGUOUS,
        )?;
        if b.view.itemsize != 4 || b.format().len() != 1 || !b"IiLl".contains(&b.format()[0]) {
            return Err(PyTypeError::new_err(
                "out must be a writable buffer of 4-byte integers (uint32 / int32)",
            ));
        }
        Ok(b)
    }
    pub fn reject_overlap(&self, text: &Text) -> PyResult<()> {
        let a = self.view.buf as usize;
        let b = text.original.0;
        let n = self.view.len as usize;
        if n != 0
            && text.original.1 != 0
            && a < b.saturating_add(text.original.1)
            && b < a.saturating_add(n)
        {
            return Err(PyValueError::new_err("out overlaps text"));
        }
        Ok(())
    }
    pub fn write_ids(&mut self, ids: &[u32]) {
        let n = ids.len().min(self.view.len as usize / 4);
        // The exported destination may be unaligned. No Rust reference to it
        // is created, and staging cannot overlap the Python export.
        unsafe {
            ptr::copy_nonoverlapping(ids.as_ptr().cast::<u8>(), self.view.buf.cast::<u8>(), n * 4)
        };
    }
}
impl Drop for Buffer<'_> {
    fn drop(&mut self) {
        unsafe { ffi::PyBuffer_Release(&mut self.view) }
    }
}

pub struct Text {
    pub owner: Py<PyAny>,
    ptr: *const u8,
    pub len: usize,
    original: (usize, usize),
}
// ptr always points into an immutable Python str/bytes owner. Both remain alive
// until the detached call ends; reference-count operations happen attached.
unsafe impl Send for Text {}
unsafe impl Sync for Text {}
impl Text {
    pub fn get(o: &Bound<'_, PyAny>) -> PyResult<Self> {
        let (owner, original) = if let Ok(s) = o.cast::<PyString>() {
            let bytes = s.to_str()?.as_bytes();
            (o.clone(), (bytes.as_ptr() as usize, bytes.len()))
        } else if let Ok(b) = o.cast::<PyBytes>() {
            (
                o.clone(),
                (b.as_bytes().as_ptr() as usize, b.as_bytes().len()),
            )
        } else {
            let b = Buffer::get(o, ffi::PyBUF_SIMPLE)?;
            let n = b.view.len as usize;
            if n > 1 << 29 {
                return Err(native_error(o.py(), -9, "text exceeds 2^29 bytes"));
            }
            let bytes = if n == 0 {
                &[]
            } else {
                unsafe { std::slice::from_raw_parts(b.view.buf.cast::<u8>(), n) }
            };
            (
                PyBytes::new(o.py(), bytes).into_any(),
                (b.view.buf as usize, n),
            )
        };
        if original.1 > 1 << 29 {
            return Err(native_error(o.py(), -9, "text exceeds 2^29 bytes"));
        }
        let bytes = if let Ok(s) = owner.cast::<PyString>() {
            s.to_str()?.as_bytes()
        } else {
            owner.cast::<PyBytes>()?.as_bytes()
        };
        let (ptr, len) = (bytes.as_ptr(), bytes.len());
        Ok(Self {
            owner: owner.unbind(),
            ptr,
            len,
            original,
        })
    }
    pub fn bytes(&self) -> &[u8] {
        unsafe { std::slice::from_raw_parts(self.ptr, self.len) }
    }
}

pub fn tuple<'py>(o: &Bound<'py, PyAny>) -> PyResult<Bound<'py, PyTuple>> {
    // Snapshot before any element's __index__ callback can mutate its source.
    unsafe {
        Bound::from_owned_ptr_or_err(o.py(), ffi::PySequence_Tuple(o.as_ptr()))?
            .cast_into::<PyTuple>()
            .map_err(Into::into)
    }
}
pub fn ids(o: &Bound<'_, PyAny>) -> PyResult<Vec<u32>> {
    if o.is_instance_of::<PyString>()
        || o.is_instance_of::<PyBytes>()
        || o.is_instance_of::<PyByteArray>()
    {
        return Err(PyTypeError::new_err("ids must be a sequence of ints"));
    }
    if unsafe { ffi::PyObject_CheckBuffer(o.as_ptr()) } == 0 {
        return tuple(o)?.iter().map(|v| v.extract::<u32>()).collect();
    }
    let b = Buffer::get(o, ffi::PyBUF_FORMAT | ffi::PyBUF_C_CONTIGUOUS)?;
    let f = b.format();
    let size = b.view.itemsize as usize;
    if f.len() != 1
        || !b"bBhHiIlLqQnN".contains(&f[0])
        || ![1, 2, 4, 8].contains(&size)
        || b.view.ndim > 1
    {
        return Err(PyTypeError::new_err(
            "ids must be a one-dimensional buffer of integers",
        ));
    }
    let signed = b"bhilqn".contains(&f[0]);
    let n = b.view.len as usize / size;
    let mut out = Vec::new();
    out.try_reserve_exact(n)
        .map_err(|_| pyo3::exceptions::PyMemoryError::new_err("ids"))?;
    for i in 0..n {
        let mut bytes = [0u8; 8];
        unsafe {
            ptr::copy_nonoverlapping(
                b.view.buf.cast::<u8>().add(i * size),
                bytes.as_mut_ptr(),
                size,
            )
        };
        let value = u64::from_le_bytes(bytes);
        if signed && bytes[size - 1] & 0x80 != 0 {
            return Err(PyOverflowError::new_err(
                "can't convert negative int to an id",
            ));
        }
        out.push(
            u32::try_from(value)
                .map_err(|_| PyOverflowError::new_err("id does not fit in 32 bits"))?,
        );
    }
    Ok(out)
}

pub fn repaired_text<'py>(o: &Bound<'py, PyAny>) -> PyResult<Bound<'py, PyAny>> {
    if let Ok(s) = o.cast::<PyString>() {
        match s.to_str() {
            Ok(_) => {}
            Err(e) if e.is_instance_of::<pyo3::exceptions::PyUnicodeEncodeError>(o.py()) => {
                return o
                    .call_method1("encode", ("utf-16", "surrogatepass"))?
                    .call_method1("decode", ("utf-16", "replace"));
            }
            Err(e) => return Err(e),
        }
    }
    Ok(o.clone())
}
