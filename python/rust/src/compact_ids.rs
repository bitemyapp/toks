//! Version-gated binding to CPython's inline compact-integer API.
//!
//! PyO3 does not expose PyUnstable_Long_IsCompact/CompactValue because they are
//! header-only. Keep their known 3.12--3.14 layout isolated here. Other Python
//! implementations, ABIs, digit formats and future versions use the public
//! conversion function in buffers.rs.
use pyo3::{ffi, prelude::*};

#[cfg(all(
    Py_3_12,
    not(Py_3_15),
    not(PyPy),
    not(GraalPy),
    not(RustPython),
    not(Py_LIMITED_API),
    not(Py_GIL_DISABLED),
    not(py_sys_config = "Py_TRACE_REFS")
))]
mod known {
    use super::*;
    use std::sync::atomic::{AtomicBool, Ordering};

    static ENABLED: AtomicBool = AtomicBool::new(false);

    #[repr(C)]
    struct LongPrefix {
        object: ffi::PyObject,
        tag: usize,
        first_digit: u32,
    }

    pub fn init(py: Python<'_>) -> PyResult<()> {
        // Use the C API's fresh struct sequence, not the replaceable sys.int_info
        // attribute. CPython also permits 15-bit/u16 digits: fall back for those.
        let info = unsafe { Bound::from_owned_ptr_or_err(py, ffi::PyLong_GetInfo())? };
        let bits: usize = info.getattr("bits_per_digit")?.extract()?;
        let bytes: usize = info.getattr("sizeof_digit")?.extract()?;
        ENABLED.store(bits == 30 && bytes == 4, Ordering::Relaxed);
        Ok(())
    }

    #[inline]
    pub fn enabled() -> bool {
        ENABLED.load(Ordering::Relaxed)
    }

    /// The caller holds the GIL, has checked PyLong_CheckExact, and has checked
    /// enabled() once for this conversion. No Python callbacks occur in between.
    #[inline]
    pub unsafe fn unsigned(value: *mut ffi::PyObject) -> Option<u32> {
        let value = value.cast::<LongPrefix>();
        // Do not form a reference to the whole struct: its trailing Rust padding
        // need not be part of the Python allocation. Zero's digit is unspecified,
        // so it is never read. Bit 2 marks small immortal ints in Python 3.14.
        match std::ptr::addr_of!((*value).tag).read() & !4 {
            1 => Some(0),
            8 => Some(std::ptr::addr_of!((*value).first_digit).read()),
            _ => None,
        }
    }
}

#[cfg(not(all(
    Py_3_12,
    not(Py_3_15),
    not(PyPy),
    not(GraalPy),
    not(RustPython),
    not(Py_LIMITED_API),
    not(Py_GIL_DISABLED),
    not(py_sys_config = "Py_TRACE_REFS")
)))]
mod known {
    use super::*;
    pub fn init(_: Python<'_>) -> PyResult<()> {
        Ok(())
    }
    #[inline]
    pub fn enabled() -> bool {
        false
    }
    #[inline]
    pub unsafe fn unsigned(_: *mut ffi::PyObject) -> Option<u32> {
        None
    }
}

pub use known::{enabled, init, unsigned};
