//! Owned interfaces to the core. Contexts are immutable and shared; an encoder
//! owns its scratch, and a decoder owns its stream and any external hold.
use crate::src::core::{api, load, split, stream, vocab};
use std::{
    ffi::{CStr, CString},
    fmt,
    path::Path,
    ptr::{self, NonNull},
    sync::Arc,
};

pub use api::toks_info as Info;

/// A stable native error code and, for loading errors, the diagnostic reason.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Error {
    pub code: i64,
    pub message: String,
}
impl Error {
    fn new(code: i64) -> Self {
        let message = match code {
            -1 => "could not open or read tokenizer",
            -2 => "invalid tokenizer format",
            -3 => "unsupported tokenizer feature",
            -5 => "unavailable CPU tier",
            -6 => "invalid scratch",
            -7 => "invalid token ID",
            -8 => "insufficient output capacity",
            -9 => "input exceeds limit",
            -10 => "invalid argument",
            -11 => "allocation failed",
            _ => "tokenizer error",
        }
        .into();
        Self { code, message }
    }
}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({})", self.message, self.code)
    }
}
impl std::error::Error for Error {}
fn count(n: i64) -> Result<usize, Error> {
    if n < 0 {
        Err(Error::new(n))
    } else {
        usize::try_from(n).map_err(|_| Error::new(-9))
    }
}
fn capacity(n: u64) -> Result<usize, Error> {
    usize::try_from(n).map_err(|_| Error::new(-9))
}
fn reserve<T>(v: &mut Vec<T>, n: usize) -> Result<(), Error> {
    if n > v.capacity() {
        v.try_reserve_exact(n.saturating_sub(v.len()))
            .map_err(|_| Error::new(-11))?;
    }
    Ok(())
}

/// Runtime-selected kernel family. A forced unsupported tier returns an error.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[repr(u32)]
pub enum Tier {
    #[default]
    Auto = 0,
    Scalar = 1,
    Neon = 2,
    Avx2 = 3,
    Avx512 = 4,
}

macro_rules! flags {
    ($name:ident) => {
        #[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
        pub struct $name(u32);
        impl $name {
            pub const fn bits(self) -> u32 {
                self.0
            }
            /// Unknown or conflicting bits are rejected by the operation.
            pub const fn from_bits(bits: u32) -> Self {
                Self(bits)
            }
        }
        impl std::ops::BitOr for $name {
            type Output = Self;
            fn bitor(self, rhs: Self) -> Self {
                Self(self.0 | rhs.0)
            }
        }
    };
}
flags!(EncodeFlags);
impl EncodeFlags {
    pub const ALL: Self = Self(0);
    pub const NONSPECIAL: Self = Self(1);
    pub const NONE: Self = Self(2);
    pub const NO_POSTPROCESS: Self = Self(4);
    pub const CONTINUATION: Self = Self(8);
    pub const NO_TRUNCATE: Self = Self(16);
    pub const NO_PAD: Self = Self(32);
}
flags!(DecodeFlags);
impl DecodeFlags {
    pub const ALL: Self = Self(0);
    pub const SKIP_SPECIAL: Self = Self(1);
    pub const RAW: Self = Self(2);
}

/// Per-encoder cache budgets. `None` selects the default 4 MiB memo; zero
/// explicitly disables it. Piece-cache zero selects its 2 MiB default.
#[derive(Clone, Copy, Debug, Default)]
pub struct ScratchOptions {
    pub memo_mib: Option<u16>,
    pub cache_mib: u8,
}
impl ScratchOptions {
    fn bits(self) -> Result<u32, Error> {
        if self.memo_mib.is_some_and(|n| n >= 4096)
            || !(self.cache_mib == 0
                || (4..=128).contains(&self.cache_mib) && self.cache_mib.is_power_of_two())
        {
            return Err(Error::new(-10));
        }
        Ok(self.memo_mib.map_or(0, |n| (1 << 20) | u32::from(n))
            | (u32::from(self.cache_mib) << 12))
    }
}

struct Context {
    ptr: NonNull<load::toks_ctx>,
    info: Info,
}
// A successfully loaded context owns all referenced tables. The core's public
// contract makes it read-only after loading; every mutable cache is in scratch.
unsafe impl Send for Context {}
unsafe impl Sync for Context {}
impl Drop for Context {
    fn drop(&mut self) {
        unsafe { load::toks_unload(self.ptr.as_ptr()) }
    }
}

/// An immutable tokenizer. Clones share its tables; create one encoder per
/// concurrent caller to retain independent caches.
#[derive(Clone)]
pub struct Tokenizer(Arc<Context>);
impl Tokenizer {
    pub fn from_file(path: impl AsRef<Path>, tier: Tier) -> Result<Self, Error> {
        let path = CString::new(path.as_ref().as_os_str().as_encoded_bytes())
            .map_err(|_| Error::new(-10))?;
        Self::load(tier, |out, opts| unsafe {
            load::toks_load(out, path.as_ptr(), opts)
        })
    }

    /// Copies the tokenizer data; the returned object does not borrow it.
    pub fn from_bytes(data: &[u8], tier: Tier) -> Result<Self, Error> {
        Self::load(tier, |out, opts| unsafe {
            load::toks_load_mem_copy(out, data.as_ptr().cast(), data.len() as u64, opts)
        })
    }

    fn load(
        tier: Tier,
        f: impl FnOnce(*mut *mut load::toks_ctx, *const load::toks_load_opts) -> i64,
    ) -> Result<Self, Error> {
        let mut diag = load::toks_diag {
            code: 0,
            what: [0; 248],
        };
        let opts = load::toks_load_opts {
            size: std::mem::size_of::<load::toks_load_opts>() as u32,
            tier: tier as u32,
            flags: 0,
            rsv: 0,
            diag: &mut diag,
        };
        let mut raw = ptr::null_mut();
        let rc = f(&mut raw, &opts);
        if rc < 0 {
            let mut error = Error::new(rc);
            let bytes: Vec<u8> = diag
                .what
                .iter()
                .take_while(|&&c| c != 0)
                .map(|&c| c as u8)
                .collect();
            if !bytes.is_empty() {
                error.message = String::from_utf8_lossy(&bytes).into_owned();
            }
            return Err(error);
        }
        let ptr = NonNull::new(raw).ok_or_else(|| Error::new(-2))?;
        // Info contains only integers and arrays; all-zero is a valid value.
        let mut info: Info = unsafe { std::mem::zeroed() };
        info.size = std::mem::size_of::<Info>() as u32;
        let rc = unsafe { api::toks_get_info(ptr.as_ptr().cast(), &mut info) };
        if rc < 0 {
            unsafe { load::toks_unload(ptr.as_ptr()) };
            return Err(Error::new(rc));
        }
        Ok(Self(Arc::new(Context { ptr, info })))
    }

    fn ptr<T>(&self) -> *const T {
        self.0.ptr.as_ptr().cast()
    }
    pub fn info(&self) -> Info {
        self.0.info
    }
    pub fn name(&self) -> String {
        String::from_utf8_lossy(
            &self
                .0
                .info
                .name
                .iter()
                .take_while(|&&c| c != 0)
                .map(|&c| c as u8)
                .collect::<Vec<_>>(),
        )
        .into_owned()
    }
    pub fn version() -> &'static str {
        // The exported version is an immutable, NUL-terminated ASCII literal.
        unsafe {
            CStr::from_ptr(crate::src::core::version::toks_version())
                .to_str()
                .unwrap_or("unknown")
        }
    }
    pub fn encode_bound(&self, len: u64) -> u64 {
        unsafe { api::toks_encode_bound(self.ptr(), len) }
    }
    pub fn encoder(&self, options: ScratchOptions) -> Result<Encoder, Error> {
        let flags = options.bits()?;
        Ok(Encoder {
            tokenizer: self.clone(),
            scratch: None,
            max_len: 0,
            flags,
        })
    }
    pub fn decoder(&self, flags: DecodeFlags) -> Result<Decoder, Error> {
        if flags.0 & !1 != 0 {
            return Err(Error::new(-10));
        }
        let mut state = stream::toks_stream { opaque: [0; 8] };
        unsafe { stream::toks_stream_init(self.ptr(), &mut state, flags.0) };
        Ok(Decoder {
            tokenizer: self.clone(),
            state,
            hold: Vec::new(),
        })
    }

    /// Bytes in the core's decoded-byte vocabulary namespace, borrowed from the
    /// context. These can differ from a Hugging Face token's written spelling.
    pub fn token(&self, id: u32) -> Option<&[u8]> {
        let mut len = 0;
        let p = unsafe { api::toks_token(self.ptr(), id, &mut len) };
        if p.is_null() {
            None
        } else {
            // The core owns len initialized bytes until the last context drops.
            Some(unsafe { std::slice::from_raw_parts(p, len as usize) })
        }
    }
    pub fn token_to_id(&self, bytes: &[u8]) -> Option<u32> {
        let n = unsafe {
            vocab::toks_token_to_id(self.ptr(), bytes.as_ptr().cast(), bytes.len() as u64)
        };
        if n < 0 {
            None
        } else {
            Some(n as u32)
        }
    }
    pub fn id_flags(&self, id: u32) -> Result<u32, Error> {
        count(unsafe { vocab::toks_id_flags(self.ptr(), id) }).map(|n| n as u32)
    }
    pub fn added(&self, index: u32) -> Option<AddedToken<'_>> {
        let (mut p, mut len, mut id) = (ptr::null(), 0, 0);
        let flags = unsafe { vocab::toks_added(self.ptr(), index, &mut p, &mut len, &mut id) };
        if flags < 0 {
            None
        } else {
            let content = if len == 0 {
                &[]
            } else {
                unsafe { std::slice::from_raw_parts(p.cast(), len as usize) }
            };
            Some(AddedToken {
                id,
                content,
                flags: flags as u32,
            })
        }
    }
    pub fn template(&self) -> Result<Template, Error> {
        let mut n_prefix = 0;
        let n = count(unsafe {
            api::toks_template(
                self.ptr(),
                ptr::null_mut(),
                ptr::null_mut(),
                0,
                &mut n_prefix,
            )
        })?;
        let (mut ids, mut type_ids) = (Vec::new(), Vec::new());
        reserve(&mut ids, n)?;
        reserve(&mut type_ids, n)?;
        let written = count(unsafe {
            api::toks_template(
                self.ptr(),
                ids.as_mut_ptr(),
                type_ids.as_mut_ptr(),
                n as u64,
                &mut n_prefix,
            )
        })?;
        // Immutable context: both calls have the same count; the second writes
        // every ID and type ID before exposing the vectors.
        if written != n {
            return Err(Error::new(-2));
        }
        unsafe {
            ids.set_len(n);
            type_ids.set_len(n);
        }
        Ok(Template {
            ids,
            type_ids,
            n_prefix: n_prefix as usize,
        })
    }

    pub fn decode_into(
        &self,
        ids: &[u32],
        flags: DecodeFlags,
        out: &mut [u8],
    ) -> Result<usize, Error> {
        count(unsafe {
            api::toks_decode(
                self.ptr(),
                ids.as_ptr(),
                ids.len() as u64,
                flags.0,
                out.as_mut_ptr(),
                out.len() as u64,
            )
        })
    }
    pub fn decode(&self, ids: &[u32], flags: DecodeFlags) -> Result<Vec<u8>, Error> {
        let first = ids.len().saturating_mul(4);
        // Each successful core call initializes the exact prefix of its result.
        unsafe {
            collect_prefix(first, |out, cap| {
                api::toks_decode(
                    self.ptr(),
                    ids.as_ptr(),
                    ids.len() as u64,
                    flags.0,
                    out,
                    cap,
                )
            })
        }
    }
    pub fn split_points(
        &self,
        text: &[u8],
        flags: EncodeFlags,
        parts: u32,
    ) -> Result<Vec<u64>, Error> {
        let n = (parts.saturating_sub(1) as usize).min(text.len());
        let mut out = Vec::new();
        reserve(&mut out, n)?;
        let got = count(unsafe {
            split::toks_split_points(
                self.ptr(),
                text.as_ptr().cast(),
                text.len() as u64,
                flags.0,
                parts,
                out.as_mut_ptr(),
                n as u64,
                ptr::null_mut(),
            )
        })?;
        if got > n {
            return Err(Error::new(-2));
        }
        unsafe { out.set_len(got) };
        Ok(out)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct AddedToken<'a> {
    pub id: u32,
    pub content: &'a [u8],
    pub flags: u32,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Template {
    pub ids: Vec<u32>,
    pub type_ids: Vec<u32>,
    pub n_prefix: usize,
}

struct Scratch {
    ptr: NonNull<u8>,
    bytes: u64,
}
// Scratch has one owner; core access requires &mut Encoder and never escapes.
unsafe impl Send for Scratch {}
impl Drop for Scratch {
    fn drop(&mut self) {
        unsafe { crate::platform::toks_plat_arena_free(self.ptr.as_ptr(), self.bytes) }
    }
}

/// Reusable encoder scratch and caches, bound to an owned tokenizer. Moving it
/// to another thread is safe; simultaneous mutation requires external locking.
pub struct Encoder {
    tokenizer: Tokenizer,
    scratch: Option<Scratch>,
    max_len: usize,
    flags: u32,
}
impl Encoder {
    pub fn tokenizer(&self) -> &Tokenizer {
        &self.tokenizer
    }
    fn prepare(&mut self, len: usize) -> Result<*mut libc::c_void, Error> {
        if len as u64 > (1 << 29) {
            return Err(Error::new(-9));
        }
        if self.scratch.is_none() || len > self.max_len {
            let max_len = len.max(65536).next_power_of_two().min(1 << 29);
            let bytes = unsafe {
                api::toks_scratch_bytes(self.tokenizer.ptr(), max_len as u64, self.flags)
            };
            let p = NonNull::new(unsafe { crate::platform::toks_plat_arena(bytes) })
                .ok_or_else(|| Error::new(-11))?;
            let scratch = Scratch { ptr: p, bytes };
            count(unsafe {
                api::toks_scratch_init(self.tokenizer.ptr(), p.as_ptr().cast(), bytes, self.flags)
            })?;
            self.scratch = Some(scratch);
            self.max_len = max_len;
        }
        Ok(self
            .scratch
            .as_ref()
            .ok_or_else(|| Error::new(-6))?
            .ptr
            .as_ptr()
            .cast())
    }
    /// Writes the exact prefix that fits and returns the total ID count. A
    /// count larger than `out.len()` is a short output, not an error.
    pub fn encode_into(
        &mut self,
        text: &[u8],
        flags: EncodeFlags,
        out: &mut [u32],
    ) -> Result<usize, Error> {
        let scr = self.prepare(text.len())?;
        count(unsafe {
            api::toks_encode(
                self.tokenizer.ptr(),
                text.as_ptr().cast(),
                text.len() as u64,
                flags.0,
                out.as_mut_ptr(),
                out.len() as u64,
                scr,
            )
        })
    }
    pub fn encode(&mut self, text: &[u8], flags: EncodeFlags) -> Result<Vec<u32>, Error> {
        let mut ids = Vec::new();
        self.encode_to(text, flags, &mut ids)?;
        Ok(ids)
    }
    /// Reuses the vector's allocation. On failure its length is zero.
    pub fn encode_to(
        &mut self,
        text: &[u8],
        flags: EncodeFlags,
        ids: &mut Vec<u32>,
    ) -> Result<(), Error> {
        ids.clear();
        let scr = self.prepare(text.len())?;
        let first = capacity(self.tokenizer.encode_bound(text.len() as u64))?
            .min(text.len().saturating_add(64));
        unsafe {
            fill_prefix(ids, first, |out, cap| {
                api::toks_encode(
                    self.tokenizer.ptr(),
                    text.as_ptr().cast(),
                    text.len() as u64,
                    flags.0,
                    out,
                    cap,
                    scr,
                )
            })
        }
    }
    pub fn pieces(&mut self, text: &[u8], flags: EncodeFlags) -> Result<Vec<u32>, Error> {
        let scr = self.prepare(text.len())?;
        unsafe {
            collect_prefix(text.len().saturating_add(1), |out, cap| {
                api::toks_pieces(
                    self.tokenizer.ptr(),
                    text.as_ptr().cast(),
                    text.len() as u64,
                    flags.0,
                    out,
                    cap,
                    scr,
                )
            })
        }
    }
    /// Releases retained scratch; the next encode recreates it.
    pub fn clear_cache(&mut self) {
        self.scratch = None;
        self.max_len = 0;
    }
}

// Safety: f must write min(returned_count, capacity) initialized T elements on
// success and stay inside the allocation on every path. Only native prefix APIs
// with that documented contract are used here. No uninitialized slice is made.
unsafe fn fill_prefix<T>(
    out: &mut Vec<T>,
    first: usize,
    mut f: impl FnMut(*mut T, u64) -> i64,
) -> Result<(), Error> {
    out.clear();
    reserve(out, first)?;
    let n = count(f(out.as_mut_ptr(), out.capacity() as u64))?;
    if n > out.capacity() {
        reserve(out, n)?;
        let got = count(f(out.as_mut_ptr(), out.capacity() as u64))?;
        if got != n {
            return Err(Error::new(-2));
        }
    }
    out.set_len(n);
    Ok(())
}
unsafe fn collect_prefix<T>(
    first: usize,
    f: impl FnMut(*mut T, u64) -> i64,
) -> Result<Vec<T>, Error> {
    let mut out = Vec::new();
    fill_prefix(&mut out, first, f)?;
    Ok(out)
}

/// Incremental repaired-byte decoder. It retains its tokenizer and grows the
/// byte-fallback hold without invalidating the old hold before the copy.
pub struct Decoder {
    tokenizer: Tokenizer,
    state: stream::toks_stream,
    hold: Vec<u8>,
}
impl Decoder {
    fn hold_capacity(&self) -> usize {
        self.hold.len().max(44)
    }
    fn output_capacity(&self, n: usize) -> Result<usize, Error> {
        let bound = unsafe { stream::toks_stream_bound(self.tokenizer.ptr(), n as u64) };
        capacity(
            bound
                .checked_add(
                    (self.hold_capacity() as u64)
                        .checked_mul(3)
                        .ok_or_else(|| Error::new(-9))?,
                )
                .ok_or_else(|| Error::new(-9))?,
        )
    }
    fn grow_hold(&mut self, n_ids: usize) -> Result<(), Error> {
        let n = self
            .hold_capacity()
            .checked_add(n_ids)
            .ok_or_else(|| Error::new(-9))?;
        let mut next = Vec::new();
        reserve(&mut next, n)?;
        next.resize(n, 0);
        count(unsafe {
            stream::toks_stream_hold(
                self.tokenizer.ptr(),
                &mut self.state,
                next.as_mut_ptr().cast(),
                n as u64,
            )
        })?;
        // The call copied held bytes from self.hold; it can now be dropped.
        self.hold = next;
        Ok(())
    }
    pub fn push(&mut self, ids: &[u32]) -> Result<Vec<u8>, Error> {
        let mut out = Vec::new();
        reserve(&mut out, self.output_capacity(ids.len())?)?;
        let mut rc = unsafe {
            stream::toks_stream_push(
                self.tokenizer.ptr(),
                &mut self.state,
                ids.as_ptr(),
                ids.len() as u64,
                out.as_mut_ptr(),
                out.capacity() as u64,
            )
        };
        if rc == -9 {
            self.grow_hold(ids.len())?;
            reserve(&mut out, self.output_capacity(ids.len())?)?;
            rc = unsafe {
                stream::toks_stream_push(
                    self.tokenizer.ptr(),
                    &mut self.state,
                    ids.as_ptr(),
                    ids.len() as u64,
                    out.as_mut_ptr(),
                    out.capacity() as u64,
                )
            };
        }
        let n = count(rc)?;
        if n > out.capacity() {
            return Err(Error::new(-2));
        }
        unsafe { out.set_len(n) };
        Ok(out)
    }
    /// Emits pending bytes and resets decoding state, retaining the hold.
    pub fn flush(&mut self) -> Result<Vec<u8>, Error> {
        let mut out = Vec::new();
        reserve(&mut out, self.output_capacity(0)?)?;
        let n = count(unsafe {
            stream::toks_stream_flush(
                self.tokenizer.ptr(),
                &mut self.state,
                out.as_mut_ptr(),
                out.capacity() as u64,
            )
        })?;
        if n > out.capacity() {
            return Err(Error::new(-2));
        }
        unsafe { out.set_len(n) };
        Ok(out)
    }
}
