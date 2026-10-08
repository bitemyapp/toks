use pyo3::{
    exceptions::PyTypeError,
    prelude::*,
    types::{PyDict, PyTuple},
};

pub struct Args<'py>(Vec<Option<Bound<'py, PyAny>>>);
impl<'py> Args<'py> {
    pub fn get(&self, i: usize) -> Option<&Bound<'py, PyAny>> {
        self.0[i].as_ref()
    }
    pub fn required(&self, i: usize) -> PyResult<&Bound<'py, PyAny>> {
        self.get(i)
            .ok_or_else(|| PyTypeError::new_err("missing required argument"))
    }
}
pub fn parse<'py>(
    args: &Bound<'py, PyTuple>,
    kwargs: Option<&Bound<'py, PyDict>>,
    names: &[&str],
    positional: usize,
    required: usize,
) -> PyResult<Args<'py>> {
    if args.len() > positional {
        return Err(PyTypeError::new_err(format!(
            "takes at most {positional} positional arguments"
        )));
    }
    let mut out: Vec<_> = (0..names.len()).map(|_| None).collect();
    for (i, v) in args.iter().enumerate() {
        out[i] = Some(v);
    }
    if let Some(kwargs) = kwargs {
        for (key, value) in kwargs {
            let key: String = key.extract()?;
            let i = names.iter().position(|&n| n == key).ok_or_else(|| {
                PyTypeError::new_err(format!("unexpected keyword argument '{key}'"))
            })?;
            if out[i].is_some() {
                return Err(PyTypeError::new_err(format!(
                    "multiple values for argument '{key}'"
                )));
            }
            out[i] = Some(value);
        }
    }
    for i in 0..required {
        if out[i].is_none() {
            return Err(PyTypeError::new_err(format!(
                "missing required argument '{}'",
                names[i]
            )));
        }
    }
    Ok(Args(out))
}
pub fn truth(o: Option<&Bound<'_, PyAny>>, default: bool) -> PyResult<bool> {
    o.map_or(Ok(default), |v| v.is_truthy())
}
