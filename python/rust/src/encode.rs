use crate::{
    args::{truth, Arg},
    buffers::{self, Buffer, Text},
    core_error,
    encoding::Encoding,
    Tokenizer,
};
use pyo3::{
    exceptions::{PyTypeError, PyValueError},
    prelude::*,
    types::{PyBytes, PyDict, PyList, PySet, PyString},
};
use std::sync::atomic::Ordering;
use toks::EncodeFlags as F;
impl Tokenizer {
    fn flags(
        &self,
        ast: Option<&Bound<'_, PyAny>>,
        mode: Option<&Bound<'_, PyAny>>,
        cont: Option<&Bound<'_, PyAny>>,
    ) -> PyResult<u32> {
        let mut flags = u32::from(self.encode_special.load(Ordering::Relaxed));
        if let Some(mode) = mode.filter(|v| !v.is_none()) {
            flags = match mode.extract::<&str>().ok() {
                Some("all") => 0,
                Some("nonspecial") => 1,
                Some("none") => 2,
                _ => {
                    return Err(PyValueError::new_err(
                        "added_tokens must be 'all', 'nonspecial' or 'none'",
                    ))
                }
            };
        }
        if !truth(ast, true)? {
            flags |= 4;
        }
        if truth(cont, false)? {
            flags |= 8;
        }
        Ok(flags)
    }
    fn pad_count(&self, flags: u32, n: usize, longest: usize) -> usize {
        let i = self.core.info();
        if i.pad_on == 0 || flags & (32 | 8) != 0 {
            return 0;
        }
        let mut target = if i.pad_fixed != 0 {
            i.pad_len as usize
        } else {
            longest
        };
        if i.pad_multiple != 0 {
            let rem = target % i.pad_multiple as usize;
            if rem != 0 {
                target += i.pad_multiple as usize - rem;
            }
        }
        target.saturating_sub(n)
    }
    pub fn padded_list<'py>(
        &self,
        py: Python<'py>,
        ids: &[u32],
        pad: usize,
    ) -> PyResult<Bound<'py, PyList>> {
        if pad == 0 {
            return self.int_list(py, ids);
        }
        let mut values = Vec::new();
        values
            .try_reserve_exact(ids.len() + pad)
            .map_err(|_| pyo3::exceptions::PyMemoryError::new_err("padding"))?;
        if self.core.info().pad_left != 0 {
            values.resize(pad, self.core.info().pad_id);
            values.extend_from_slice(ids);
        } else {
            values.extend_from_slice(ids);
            values.resize(ids.len() + pad, self.core.info().pad_id);
        }
        self.int_list(py, &values)
    }
    fn one_out<'py>(
        slf: &Bound<'py, Self>,
        text: &Bound<'py, PyAny>,
        flags: u32,
        kind: u8,
    ) -> PyResult<Bound<'py, PyAny>> {
        let py = slf.py();
        let this = slf.get();
        let tx = Text::get(text)?;
        let cf = F::from_bits(if kind == 2 { flags | 32 } else { flags });
        this.with_slot(py, tx.len, |s| {
            let mut run = || {
                if kind == 1 {
                    s.encoder.pieces_to(tx.bytes(), cf, &mut s.ids)
                } else {
                    s.encoder.encode_to(tx.bytes(), cf, &mut s.ids)
                }
            };
            let result = if tx.len >= 2048 {
                py.detach(run)
            } else {
                run()
            };
            result.map_err(|e| core_error(py, e))?;
            if kind == 2 {
                Ok(Encoding::create(
                    slf,
                    &s.ids,
                    this.pad_count(flags, s.ids.len(), s.ids.len()),
                    flags,
                    tx.owner.clone_ref(py),
                )?
                .into_any())
            } else {
                Ok(this.int_list(py, &s.ids)?.into_any())
            }
        })
    }
    fn batch_out<'py>(
        slf: &Bound<'py, Self>,
        texts: &Bound<'py, PyAny>,
        flags: u32,
        extended: bool,
    ) -> PyResult<Bound<'py, PyList>> {
        let py = slf.py();
        let this = slf.get();
        if texts.is_instance_of::<PyString>() || texts.is_instance_of::<PyBytes>() {
            return Err(PyTypeError::new_err(
                "encode_batch takes a sequence of texts, not one text",
            ));
        }
        let texts = buffers::tuple(texts)?;
        let mut tx = Vec::new();
        let mut total = 0usize;
        let mut longest_text = 0;
        for (i, item) in texts.iter().enumerate() {
            if !item.is_instance_of::<PyString>()
                && !item.is_instance_of::<PyBytes>()
                && unsafe { pyo3::ffi::PyObject_CheckBuffer(item.as_ptr()) } == 0
            {
                return Err(PyTypeError::new_err(format!(
                    "encode_batch: text {i} must be str or a bytes-like object"
                )));
            }
            let text = Text::get(&item)?;
            total = total.saturating_add(text.len);
            longest_text = longest_text.max(text.len);
            tx.push(text);
        }
        let values = this.with_slot(py, longest_text, |s| {
            let mut run = || {
                tx.iter()
                    .map(|text| s.encoder.encode(text.bytes(), F::from_bits(flags | 32)))
                    .collect::<Result<Vec<_>, _>>()
            };
            if total >= 2048 { py.detach(run) } else { run() }.map_err(|e| core_error(py, e))
        })?;
        let longest = values.iter().map(Vec::len).max().unwrap_or(0);
        let out = PyList::empty(py);
        for (ids, text) in values.iter().zip(tx) {
            let pad = this.pad_count(flags, ids.len(), longest);
            if extended {
                out.append(Encoding::create(slf, ids, pad, flags, text.owner)?)?;
            } else {
                out.append(this.padded_list(py, ids, pad)?)?;
            }
        }
        Ok(out)
    }
    fn tt_encode<'py>(
        slf: &Bound<'py, Self>,
        text: &Bound<'py, PyAny>,
        allowed: Option<&Bound<'py, PyAny>>,
        disallowed: Option<&Bound<'py, PyAny>>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let py = slf.py();
        let tt = Self::tt(slf)?;
        let byid = tt.get_item(0)?.cast_into::<PyDict>()?;
        let bystr = tt.get_item(1)?.cast_into::<PyDict>()?;
        let ord = tt.get_item(2)?.extract::<u32>()?;
        let all = |v: Option<&Bound<'_, PyAny>>| {
            v.and_then(|v| v.extract::<&str>().ok())
                .is_some_and(|s| s == "all")
        };
        let a_all = all(allowed);
        let d_all = disallowed.is_none() || all(disallowed);
        let set = |v: Option<&Bound<'py, PyAny>>| -> PyResult<Bound<'py, PySet>> {
            match v {
                Some(v) => py
                    .get_type::<PySet>()
                    .call1((v,))?
                    .cast_into::<PySet>()
                    .map_err(Into::into),
                None => PySet::empty(py),
            }
        };
        let aset = if a_all {
            PySet::empty(py)?
        } else {
            set(allowed)?
        };
        let dset = if d_all { None } else { Some(set(disallowed)?) };
        let fixed = buffers::repaired_text(text)?;
        if !a_all && aset.is_empty() && dset.as_ref().is_some_and(|s| s.is_empty()) {
            return Self::one_out(slf, &fixed, ord | 4 | 16 | 32, 0);
        }
        let ids = Self::one_out(slf, &fixed, 4 | 16 | 32, 0)?.cast_into::<PyList>()?;
        if a_all && dset.is_none() {
            return Ok(ids.into_any());
        }
        let find = |needle: &Bound<'py, PyAny>| -> PyResult<isize> {
            if !needle.is_instance_of::<PyString>() {
                return Ok(-1);
            }
            if fixed.is_instance_of::<PyString>() {
                fixed.call_method1("find", (needle,))?.extract()
            } else {
                let text = Text::get(&fixed)?;
                PyBytes::new(py, text.bytes())
                    .call_method1("find", (needle.call_method1("encode", ("utf-8",))?,))?
                    .extract()
            }
        };
        let seen = PyDict::new(py);
        let mut mixed = false;
        let mut best = isize::MAX;
        let mut bad = None;
        for id in ids.iter() {
            if let Some(content) = byid.get_item(&id)? {
                let at = if let Some(v) = seen.get_item(&content)? {
                    v.extract::<isize>()?
                } else {
                    let at = find(&content)?;
                    seen.set_item(&content, at)?;
                    at
                };
                if at < 0 {
                    continue;
                }
                let in_a = a_all || aset.contains(&content)?;
                let in_d = match &dset {
                    Some(s) => s.contains(&content)?,
                    None => !in_a,
                };
                if in_d && at < best {
                    best = at;
                    bad = Some(content.clone());
                }
                if !in_d && !in_a {
                    mixed = true;
                }
            }
        }
        if let Some(dset) = dset {
            for content in dset.iter() {
                if !bystr.contains(&content)? {
                    let at = find(&content)?;
                    if at >= 0 && at < best {
                        best = at;
                        bad = Some(content);
                    }
                }
            }
        }
        if let Some(bad) = bad {
            return Err(PyErr::from_value(
                py.import("toks._vocab")?
                    .call_method1("disallowed", (bad,))?,
            ));
        }
        if !mixed {
            return Ok(ids.into_any());
        }
        let keep = PyDict::new(py);
        for (k, v) in bystr {
            if aset.contains(&k)? {
                keep.set_item(k, v)?;
            }
        }
        py.import("toks._vocab")?
            .call_method1("encode_subset", (slf, fixed, keep))
    }
}
#[pymethods]
impl Tokenizer {
    #[pyo3(signature=(text, *, add_special_tokens=Arg::MISSING, added_tokens=Arg::MISSING, continuation=Arg::MISSING, allowed_special=Arg::MISSING, disallowed_special=Arg::MISSING))]
    fn encode<'py>(
        slf: &Bound<'py, Self>,
        text: &Bound<'py, PyAny>,
        add_special_tokens: Arg<'py>,
        added_tokens: Arg<'py>,
        continuation: Arg<'py>,
        allowed_special: Arg<'py>,
        disallowed_special: Arg<'py>,
    ) -> PyResult<Bound<'py, PyAny>> {
        if allowed_special.get().is_some() || disallowed_special.get().is_some() {
            if add_special_tokens.get().is_some()
                || added_tokens.get().is_some()
                || continuation.get().is_some()
            {
                return Err(PyTypeError::new_err(
                    "cannot combine Hugging Face and tiktoken arguments",
                ));
            }
            return Self::tt_encode(slf, text, allowed_special.get(), disallowed_special.get());
        }
        Self::one_out(
            slf,
            text,
            slf.get().flags(
                add_special_tokens.get(),
                added_tokens.get(),
                continuation.get(),
            )?,
            0,
        )
    }
    #[pyo3(signature=(text,/))]
    fn encode_ordinary<'py>(
        slf: &Bound<'py, Self>,
        text: &Bound<'py, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let ord = Self::tt(slf)?.get_item(2)?.extract::<u32>()?;
        Self::one_out(slf, &buffers::repaired_text(text)?, ord | 4 | 16 | 32, 0)
    }
    #[pyo3(signature=(text, *, add_special_tokens=Arg::MISSING, added_tokens=Arg::MISSING, continuation=Arg::MISSING))]
    fn encode_ex<'py>(
        slf: &Bound<'py, Self>,
        text: &Bound<'py, PyAny>,
        add_special_tokens: Arg<'py>,
        added_tokens: Arg<'py>,
        continuation: Arg<'py>,
    ) -> PyResult<Bound<'py, PyAny>> {
        Self::one_out(
            slf,
            text,
            slf.get().flags(
                add_special_tokens.get(),
                added_tokens.get(),
                continuation.get(),
            )?,
            2,
        )
    }
    #[pyo3(signature=(text, *, added_tokens=Arg::MISSING, continuation=Arg::MISSING))]
    fn pieces<'py>(
        slf: &Bound<'py, Self>,
        text: &Bound<'py, PyAny>,
        added_tokens: Arg<'py>,
        continuation: Arg<'py>,
    ) -> PyResult<Bound<'py, PyAny>> {
        Self::one_out(
            slf,
            text,
            slf.get()
                .flags(None, added_tokens.get(), continuation.get())?,
            1,
        )
    }
    #[pyo3(signature=(text, out, *, add_special_tokens=Arg::MISSING, added_tokens=Arg::MISSING, continuation=Arg::MISSING))]
    fn encode_into<'py>(
        &self,
        text: &Bound<'py, PyAny>,
        out: &Bound<'py, PyAny>,
        add_special_tokens: Arg<'py>,
        added_tokens: Arg<'py>,
        continuation: Arg<'py>,
    ) -> PyResult<usize> {
        let py = text.py();
        let flags = F::from_bits(self.flags(
            add_special_tokens.get(),
            added_tokens.get(),
            continuation.get(),
        )?);
        let mut out = Buffer::output(out)?;
        let tx = Text::get(text)?;
        out.reject_overlap(&tx)?;
        self.with_slot(py, tx.len, |s| {
            let mut run = || s.encoder.encode_to(tx.bytes(), flags, &mut s.ids);
            if tx.len >= 2048 {
                py.detach(run)
            } else {
                run()
            }
            .map_err(|e| core_error(py, e))?;
            out.write_ids(&s.ids);
            Ok(s.ids.len())
        })
    }
    #[pyo3(signature=(texts, *, add_special_tokens=Arg::MISSING, added_tokens=Arg::MISSING, continuation=Arg::MISSING))]
    fn encode_batch<'py>(
        slf: &Bound<'py, Self>,
        texts: &Bound<'py, PyAny>,
        add_special_tokens: Arg<'py>,
        added_tokens: Arg<'py>,
        continuation: Arg<'py>,
    ) -> PyResult<Bound<'py, PyList>> {
        Self::batch_out(
            slf,
            texts,
            slf.get().flags(
                add_special_tokens.get(),
                added_tokens.get(),
                continuation.get(),
            )?,
            false,
        )
    }
    #[pyo3(signature=(texts, *, add_special_tokens=Arg::MISSING, added_tokens=Arg::MISSING, continuation=Arg::MISSING))]
    fn encode_batch_ex<'py>(
        slf: &Bound<'py, Self>,
        texts: &Bound<'py, PyAny>,
        add_special_tokens: Arg<'py>,
        added_tokens: Arg<'py>,
        continuation: Arg<'py>,
    ) -> PyResult<Bound<'py, PyList>> {
        Self::batch_out(
            slf,
            texts,
            slf.get().flags(
                add_special_tokens.get(),
                added_tokens.get(),
                continuation.get(),
            )?,
            true,
        )
    }
}
