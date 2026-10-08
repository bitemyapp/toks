#[repr(C)]
pub struct toks_gen { _opaque: [u8; 0] }
#[repr(C)]
pub struct toks_uni_src { _opaque: [u8; 0] }
#[repr(C)]
pub struct toks_spm_config { _opaque: [u8; 0] }
extern "C" {
    fn memcpy(
        dst: *mut ::core::ffi::c_void,
        src: *const ::core::ffi::c_void,
        n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memset(
        dst: *mut ::core::ffi::c_void,
        c: ::core::ffi::c_int,
        n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memcmp(
        a: *const ::core::ffi::c_void,
        b: *const ::core::ffi::c_void,
        n: size_t,
    ) -> ::core::ffi::c_int;
    fn toks_plat_alloc(n: uint64_t) -> *mut ::core::ffi::c_void;
    fn toks_plat_free(p: *mut ::core::ffi::c_void, n: uint64_t);
    fn toks_plat_arena(n: uint64_t) -> *mut uint8_t;
    static TOKS_CRC32C_TAB: [uint32_t; 256];
    fn toks_wp_ascii_class(f: uint32_t, b: uint32_t) -> uint8_t;
}
pub type size_t = usize;
pub type __uint8_t = u8;
pub type __uint16_t = u16;
pub type __int32_t = i32;
pub type __uint32_t = u32;
pub type __int64_t = i64;
pub type __uint64_t = u64;
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct toks_opts {
    pub trunc_on: uint32_t,
    pub trunc_max: uint32_t,
    pub trunc_stride: uint32_t,
    pub pad_on: uint32_t,
    pub pad_fixed: uint32_t,
    pub pad_len: uint32_t,
    pub pad_multiple: uint32_t,
    pub pad_id: uint32_t,
    pub pad_left: uint32_t,
    pub wp_win: uint32_t,
    pub dec_wordpiece: uint32_t,
    pub dec_cleanup: uint32_t,
    pub dec_prefix_len: uint32_t,
    pub dec_prefix: [uint8_t; 16],
    pub pad_file: uint32_t,
    pub pad_type_id: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct toks_wp_tables {
    pub flags: uint32_t,
    pub max_chars: uint32_t,
    pub unk_id: uint32_t,
    pub word_maxlen: uint32_t,
    pub cont_maxlen: uint32_t,
    pub prefix_len: uint32_t,
    pub ascii_cls: [uint8_t; 128],
    pub ascii_cls0: [uint8_t; 128],
    pub word: *const toks_wp_entry,
    pub word_mask: uint64_t,
    pub cont: *const toks_wp_entry,
    pub cont_mask: uint64_t,
    pub keys: *const uint8_t,
    pub wtab: *const uint8_t,
    pub wtab_mask: uint64_t,
    pub wcell: *const toks_wp_cell,
    pub ccell: *const toks_wp_cell,
    pub wterm: *const int32_t,
    pub cterm: *const int32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct toks_wp_cell {
    pub base: uint32_t,
    pub check: int32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct toks_wp_entry {
    pub h: uint32_t,
    pub id: uint32_t,
    pub off: uint32_t,
    pub len: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct toks_ext {
    pub pad: uint32_t,
    pub align: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct toks_err {
    pub code: int64_t,
    pub what: *const ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct toks_arena {
    pub base: *mut uint8_t,
    pub len: uint64_t,
    pub pos: uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct toks_config {
    pub vocab: *const *const uint8_t,
    pub vocab_len: *const uint32_t,
    pub n_vocab: uint32_t,
    pub n_vocab_raw: uint32_t,
    pub drop: [uint8_t; 32],
    pub has_drop: uint32_t,
    pub drop_unk: uint32_t,
    pub drop_fuse: uint32_t,
    pub m_left_id: *const uint32_t,
    pub m_right_id: *const uint32_t,
    pub m_out_id: *const uint32_t,
    pub n_merges: uint32_t,
    pub ignore_merges: uint8_t,
    pub dec_byte_level: uint8_t,
    pub nfc: uint8_t,
    pub ids_as_rank: uint8_t,
    pub rsv: uint8_t,
    pub pattern: *const toks_pattern,
    pub gen: *const toks_gen,
    pub gen_bytes: uint64_t,
    pub cut_chunk: uint32_t,
    pub cut_run: uint32_t,
    pub added: *mut toks_cfg_added,
    pub n_added: uint32_t,
    pub n_ids: uint32_t,
    pub pp_single: *mut toks_pp_piece,
    pub n_pp_single: uint32_t,
    pub name: *const ::core::ffi::c_char,
    pub algo: uint32_t,
    pub n_strings: uint32_t,
    pub spm: *const toks_spm_config,
    pub wp_flags: uint32_t,
    pub wp_max_chars: uint32_t,
    pub wp_unk_id: uint32_t,
    pub wp_unk: *const uint8_t,
    pub wp_unk_len: uint32_t,
    pub wp_prefix_len: uint32_t,
    pub wp_prefix: *const uint8_t,
    pub o: toks_opts,
    pub uni: *const toks_uni_src,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct toks_pp_piece {
    pub kind: uint32_t,
    pub id: uint32_t,
    pub type_0: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct toks_cfg_added {
    pub content: *const uint8_t,
    pub len: uint32_t,
    pub id: uint32_t,
    pub special: uint8_t,
    pub normalized: uint8_t,
    pub lstrip: uint8_t,
    pub rstrip: uint8_t,
    pub single_word: uint8_t,
    pub pfx: uint8_t,
    pub attr: uint8_t,
    pub rsv: uint8_t,
    pub form: *const uint8_t,
    pub form_len: uint32_t,
    pub last: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct toks_pattern {
    pub regex: *const ::core::ffi::c_char,
    pub params: uint32_t,
    pub tmpl: uint32_t,
    pub class_flags: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct toks_wp_piece {
    pub off: uint32_t,
    pub len: uint32_t,
    pub end: uint32_t,
    pub flags: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct toks_wp_encode_args {
    pub text: *const uint8_t,
    pub mat: *const uint8_t,
    pub pieces: *const toks_wp_piece,
    pub n: uint64_t,
    pub out: *mut uint32_t,
    pub room: uint64_t,
    pub n_out: uint64_t,
    pub hits: uint64_t,
    pub misses: uint64_t,
    pub probes: uint64_t,
    pub text_len: uint64_t,
    pub mat_len: uint64_t,
    pub cache: *mut uint8_t,
    pub cache_mask: uint64_t,
    pub tw: uint64_t,
    pub rsv: [uint64_t; 1],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bpe_key {
    pub lo: uint64_t,
    pub hi: uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct toks_wp_vocab {
    pub str_0: *const *const uint8_t,
    pub len: *const uint32_t,
    pub id: *const uint32_t,
    pub n: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct toks_wp_params {
    pub flags: uint32_t,
    pub max_chars: uint32_t,
    pub unk: *const uint8_t,
    pub unk_len: uint32_t,
    pub prefix: *const uint8_t,
    pub prefix_len: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ukey {
    pub off: uint64_t,
    pub len: uint32_t,
    pub id: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct qent {
    pub slot: uint32_t,
    pub lo: uint32_t,
    pub hi: uint32_t,
    pub depth: uint32_t,
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
pub const UINT64_MAX: ::core::ffi::c_ulong = 18446744073709551615
    as ::core::ffi::c_ulong;
pub const TOKS_E_UNSUPPORTED: ::core::ffi::c_int = -(3 as ::core::ffi::c_int);
pub const TOKS_E_LIMIT: ::core::ffi::c_int = -(9 as ::core::ffi::c_int);
pub const TOKS_E_NOMEM: ::core::ffi::c_int = -(11 as ::core::ffi::c_int);
pub const TOKS_ID_MASK: ::core::ffi::c_uint = 0x1fffff as ::core::ffi::c_uint;
pub const TOKS_KEY_MAXLEN: ::core::ffi::c_int = 15 as ::core::ffi::c_int;
pub const TOKS_VAL_COUNT_SHIFT: ::core::ffi::c_int = 29 as ::core::ffi::c_int;
pub const TOKS_BUCKET: ::core::ffi::c_int = 64 as ::core::ffi::c_int;
pub const TOKS_TAG_MASK64: ::core::ffi::c_ulonglong = 0xffe00000ffe00000
    as ::core::ffi::c_ulonglong;
pub const TOKS_FIB64: ::core::ffi::c_ulonglong = 0x9e3779b97f4a7c15
    as ::core::ffi::c_ulonglong;
pub const TOKS_X_WORDS: toks_ext = toks_ext {
    pad: 0 as uint32_t,
    align: 64 as uint32_t,
};
pub const TOKS_X_WP: toks_ext = toks_ext {
    pad: 0 as uint32_t,
    align: 8 as uint32_t,
};
pub const TOKS_X_WP_KEYS: toks_ext = toks_ext {
    pad: 0 as uint32_t,
    align: 1 as uint32_t,
};
pub const TOKS_X_WP_ENTRIES: toks_ext = toks_ext {
    pad: 0 as uint32_t,
    align: 4 as uint32_t,
};
pub const TOKS_X_WP_CELLS: toks_ext = toks_ext {
    pad: 0 as uint32_t,
    align: 4 as uint32_t,
};
pub const TOKS_X_WP_TERM: toks_ext = toks_ext {
    pad: 0 as uint32_t,
    align: 4 as uint32_t,
};
#[inline]
unsafe extern "C" fn toks_ld32(mut p: *const ::core::ffi::c_void) -> uint32_t {
    let mut v: uint32_t = 0;
    memcpy(&raw mut v as *mut ::core::ffi::c_void, p, 4 as size_t);
    return v;
}
#[inline]
unsafe extern "C" fn toks_st32(mut p: *mut ::core::ffi::c_void, mut v: uint32_t) {
    memcpy(p, &raw mut v as *const ::core::ffi::c_void, 4 as size_t);
}
#[inline]
unsafe extern "C" fn toks_ar_alloc(
    mut a: *mut toks_arena,
    mut n: uint64_t,
    mut align: uint64_t,
) -> *mut ::core::ffi::c_void {
    let mut p: uint64_t = (*a).pos.wrapping_add(align.wrapping_sub(1 as uint64_t))
        & !align.wrapping_sub(1 as uint64_t);
    if p < (*a).pos || p > (*a).len || n > (*a).len.wrapping_sub(p) {
        return NULL;
    }
    (*a).pos = p.wrapping_add(n);
    return (*a).base.offset(p as isize) as *mut ::core::ffi::c_void;
}
#[inline]
unsafe extern "C" fn toks_tab_ar(
    mut a: *mut toks_arena,
    mut n: uint64_t,
    mut align: uint64_t,
    mut x: toks_ext,
) -> *mut ::core::ffi::c_void {
    let mut p: *mut ::core::ffi::c_void = toks_ar_alloc(
        a,
        n.wrapping_add(x.pad as uint64_t),
        align,
    );
    return p;
}
#[inline]
unsafe extern "C" fn toks_tab_seal(mut b: *mut ::core::ffi::c_void, mut n: uint64_t) {}
#[inline]
unsafe extern "C" fn toks_crc32c_u32(mut crc: uint32_t, mut v: uint32_t) -> uint32_t {
    crc = TOKS_CRC32C_TAB[((crc ^ v) & 0xff as uint32_t) as usize]
        ^ crc >> 8 as ::core::ffi::c_int;
    crc = TOKS_CRC32C_TAB[((crc ^ v >> 8 as ::core::ffi::c_int) & 0xff as uint32_t)
        as usize] ^ crc >> 8 as ::core::ffi::c_int;
    crc = TOKS_CRC32C_TAB[((crc ^ v >> 16 as ::core::ffi::c_int) & 0xff as uint32_t)
        as usize] ^ crc >> 8 as ::core::ffi::c_int;
    crc = TOKS_CRC32C_TAB[((crc ^ v >> 24 as ::core::ffi::c_int) & 0xff as uint32_t)
        as usize] ^ crc >> 8 as ::core::ffi::c_int;
    return crc;
}
#[inline]
unsafe extern "C" fn toks_crc32c_u64(mut crc: uint32_t, mut v: uint64_t) -> uint32_t {
    crc = toks_crc32c_u32(crc, v as uint32_t);
    crc = toks_crc32c_u32(crc, (v >> 32 as ::core::ffi::c_int) as uint32_t);
    return crc;
}
#[inline]
unsafe extern "C" fn toks_fail(
    mut err: *mut toks_err,
    mut code: int64_t,
    mut what: *const ::core::ffi::c_char,
) -> int64_t {
    (*err).code = code;
    (*err).what = what;
    return code;
}
pub const TOKS_NS_LOWER: ::core::ffi::c_uint = 0x8 as ::core::ffi::c_uint;
pub const TOKS_WPF_LOWER: ::core::ffi::c_uint = TOKS_NS_LOWER;
pub const TOKS_WPP_MAT: ::core::ffi::c_uint = 0x1 as ::core::ffi::c_uint;
pub const TOKS_WPP_OVER: ::core::ffi::c_uint = 0x2 as ::core::ffi::c_uint;
pub const TOKS_WPP_INVALID: ::core::ffi::c_uint = 0x4 as ::core::ffi::c_uint;
pub const TOKS_WP_HSEED: ::core::ffi::c_uint = 0x7f4a7c15 as ::core::ffi::c_uint;
pub const TOKS_WP_MAX_KEY: ::core::ffi::c_uint = 65535 as ::core::ffi::c_uint;
pub const TOKS_WP_TERM: ::core::ffi::c_uint = 0x80000000 as ::core::ffi::c_uint;
pub const TOKS_WP_BASE: ::core::ffi::c_uint = 0x7fffffff as ::core::ffi::c_uint;
pub const TOKS_WP_MAX_CHARS: ::core::ffi::c_uint = 1024 as ::core::ffi::c_uint;
#[inline]
unsafe extern "C" fn wp_fold(mut w: uint64_t) -> uint64_t {
    let mut x: uint64_t = (w as ::core::ffi::c_ulonglong
        & 0x7f7f7f7f7f7f7f7f as ::core::ffi::c_ulonglong) as uint64_t;
    let mut m: uint64_t = ((x as ::core::ffi::c_ulonglong)
        .wrapping_add(0x3f3f3f3f3f3f3f3f as ::core::ffi::c_ulonglong)
        & !(x as ::core::ffi::c_ulonglong)
            .wrapping_add(0x2525252525252525 as ::core::ffi::c_ulonglong)
        & !w as ::core::ffi::c_ulonglong
        & 0x8080808080808080 as ::core::ffi::c_ulonglong) as uint64_t;
    return w | m >> 2 as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn wp_load(mut p: *const uint8_t, mut n: uint64_t) -> uint64_t {
    let mut w: uint64_t = 0 as uint64_t;
    if n >= 8 as uint64_t {
        memcpy(
            &raw mut w as *mut ::core::ffi::c_void,
            p as *const ::core::ffi::c_void,
            8 as size_t,
        );
        return w;
    }
    let mut k: uint64_t = 0 as uint64_t;
    while k < n {
        w |= (*p.offset(k as isize) as uint64_t) << (8 as uint64_t).wrapping_mul(k);
        k = k.wrapping_add(1);
    }
    return w;
}
#[inline]
unsafe extern "C" fn wp_hash_end(mut h: uint32_t, mut len: uint64_t) -> uint32_t {
    h = toks_crc32c_u32(h, len as uint32_t);
    h ^= h >> 16 as ::core::ffi::c_int;
    h = (h as ::core::ffi::c_uint).wrapping_mul(0x85ebca6b as ::core::ffi::c_uint)
        as uint32_t as uint32_t;
    h ^= h >> 13 as ::core::ffi::c_int;
    h = (h as ::core::ffi::c_uint).wrapping_mul(0xc2b2ae35 as ::core::ffi::c_uint)
        as uint32_t as uint32_t;
    return h ^ h >> 16 as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn wp_hash(
    mut p: *const uint8_t,
    mut len: uint64_t,
    mut fold: ::core::ffi::c_int,
) -> uint32_t {
    let mut h: uint32_t = TOKS_WP_HSEED as uint32_t;
    let mut i: uint64_t = 0 as uint64_t;
    while len.wrapping_sub(i) >= 8 as uint64_t {
        let mut w: uint64_t = wp_load(p.offset(i as isize), 8 as uint64_t);
        h = toks_crc32c_u64(h, if fold != 0 { wp_fold(w) } else { w });
        i = i.wrapping_add(8 as uint64_t);
    }
    if i < len {
        let mut w_0: uint64_t = wp_load(p.offset(i as isize), len.wrapping_sub(i));
        h = toks_crc32c_u64(h, if fold != 0 { wp_fold(w_0) } else { w_0 });
    }
    return wp_hash_end(h, len);
}
#[inline]
unsafe extern "C" fn bpe_pow2(mut n: uint64_t) -> uint64_t {
    let mut p: uint64_t = 1 as uint64_t;
    while p < n {
        p <<= 1 as ::core::ffi::c_int;
    }
    return p;
}
#[inline]
unsafe extern "C" fn bpe_load_le(mut p: *const uint8_t, mut n: uint64_t) -> uint64_t {
    if n >= 4 as uint64_t {
        let mut x: uint32_t = 0;
        let mut y: uint32_t = 0;
        memcpy(
            &raw mut x as *mut ::core::ffi::c_void,
            p as *const ::core::ffi::c_void,
            4 as size_t,
        );
        memcpy(
            &raw mut y as *mut ::core::ffi::c_void,
            p.offset(n as isize).offset(-(4 as ::core::ffi::c_uint as isize))
                as *const ::core::ffi::c_void,
            4 as size_t,
        );
        return x as uint64_t
            | (y as uint64_t)
                << (8 as uint64_t).wrapping_mul(n.wrapping_sub(4 as uint64_t));
    }
    if n >= 2 as uint64_t {
        let mut x_0: uint16_t = 0;
        let mut y_0: uint16_t = 0;
        memcpy(
            &raw mut x_0 as *mut ::core::ffi::c_void,
            p as *const ::core::ffi::c_void,
            2 as size_t,
        );
        memcpy(
            &raw mut y_0 as *mut ::core::ffi::c_void,
            p.offset(n as isize).offset(-(2 as ::core::ffi::c_uint as isize))
                as *const ::core::ffi::c_void,
            2 as size_t,
        );
        return x_0 as uint64_t
            | (y_0 as uint64_t)
                << (8 as uint64_t).wrapping_mul(n.wrapping_sub(2 as uint64_t));
    }
    return *p.offset(0 as ::core::ffi::c_int as isize) as uint64_t;
}
#[inline]
unsafe extern "C" fn bpe_key_make(
    mut key: *mut uint8_t,
    mut p: *const uint8_t,
    mut len: uint32_t,
) {
    memset(key as *mut ::core::ffi::c_void, 0 as ::core::ffi::c_int, 16 as size_t);
    memcpy(
        key as *mut ::core::ffi::c_void,
        p as *const ::core::ffi::c_void,
        len as size_t,
    );
    *key.offset(15 as ::core::ffi::c_int as isize) = len as uint8_t;
}
#[inline]
unsafe extern "C" fn bpe_key_at(
    mut text: *const uint8_t,
    mut tlen: uint64_t,
    mut s: uint64_t,
    mut n: uint64_t,
) -> bpe_key {
    let mut k: bpe_key = bpe_key { lo: 0, hi: 0 };
    let mut p: *const uint8_t = text.offset(s as isize);
    if s.wrapping_add(16 as uint64_t) <= tlen {
        memcpy(
            &raw mut k.lo as *mut ::core::ffi::c_void,
            p as *const ::core::ffi::c_void,
            8 as size_t,
        );
        memcpy(
            &raw mut k.hi as *mut ::core::ffi::c_void,
            p.offset(8 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_void,
            8 as size_t,
        );
        k.lo = (k.lo as ::core::ffi::c_ulong
            & if n >= 8 as uint64_t {
                UINT64_MAX
            } else {
                ((1 as ::core::ffi::c_ulong) << (8 as uint64_t).wrapping_mul(n))
                    .wrapping_sub(1 as ::core::ffi::c_ulong)
            }) as uint64_t;
        k.hi = (k.hi as ::core::ffi::c_ulong
            & if n > 8 as uint64_t {
                ((1 as ::core::ffi::c_ulong)
                    << (8 as uint64_t).wrapping_mul(n.wrapping_sub(8 as uint64_t)))
                    .wrapping_sub(1 as ::core::ffi::c_ulong)
            } else {
                0 as ::core::ffi::c_ulong
            }) as uint64_t;
    } else if n > 8 as uint64_t {
        memcpy(
            &raw mut k.lo as *mut ::core::ffi::c_void,
            p as *const ::core::ffi::c_void,
            8 as size_t,
        );
        k.hi = bpe_load_le(
            p.offset(8 as ::core::ffi::c_int as isize),
            n.wrapping_sub(8 as uint64_t),
        );
    } else {
        k.lo = bpe_load_le(p, n);
        k.hi = 0 as uint64_t;
    }
    k.hi |= n << 56 as ::core::ffi::c_int;
    return k;
}
#[inline]
unsafe extern "C" fn bpe_bucket_get(
    mut b: *const uint8_t,
    mut k: bpe_key,
) -> *const uint8_t {
    let mut w: [uint64_t; 4] = [0; 4];
    memcpy(
        &raw mut w as *mut uint64_t as *mut ::core::ffi::c_void,
        b as *const ::core::ffi::c_void,
        32 as size_t,
    );
    if w[0 as ::core::ffi::c_int as usize] == k.lo
        && w[1 as ::core::ffi::c_int as usize] == k.hi
    {
        return b.offset(32 as ::core::ffi::c_int as isize);
    }
    if w[2 as ::core::ffi::c_int as usize] == k.lo
        && w[3 as ::core::ffi::c_int as usize] == k.hi
    {
        return b.offset(48 as ::core::ffi::c_int as isize);
    }
    return ::core::ptr::null::<uint8_t>();
}
#[inline]
unsafe extern "C" fn bpe_cache_get(
    mut b: *const uint8_t,
    mut k: bpe_key,
    mut tw: uint64_t,
) -> *const uint8_t {
    let mut v: *const uint8_t = bpe_bucket_get(b, k);
    if v.is_null() {
        return ::core::ptr::null::<uint8_t>();
    }
    let mut g: uint64_t = 0;
    memcpy(
        &raw mut g as *mut ::core::ffi::c_void,
        v.offset(8 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_void,
        8 as size_t,
    );
    return if g as ::core::ffi::c_ulonglong & TOKS_TAG_MASK64
        == tw as ::core::ffi::c_ulonglong
    {
        v
    } else {
        ::core::ptr::null::<uint8_t>()
    };
}
#[inline]
unsafe extern "C" fn bpe_val_pack_tag(
    mut val: *mut uint32_t,
    mut idv: *const uint32_t,
    mut n: uint32_t,
    mut tw: uint64_t,
) {
    *val.offset(0 as ::core::ffi::c_int as isize) = n << TOKS_VAL_COUNT_SHIFT
        | toks_ld32(idv as *const ::core::ffi::c_void) & TOKS_ID_MASK as uint32_t;
    *val.offset(1 as ::core::ffi::c_int as isize) = if n > 1 as uint32_t {
        toks_ld32(
            idv.offset(1 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_void,
        ) & TOKS_ID_MASK as uint32_t
    } else {
        0 as uint32_t
    };
    *val.offset(2 as ::core::ffi::c_int as isize) = (if n > 2 as uint32_t {
        toks_ld32(
            idv.offset(2 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_void,
        ) & TOKS_ID_MASK as uint32_t
    } else {
        0 as uint32_t
    }) | tw as uint32_t;
    *val.offset(3 as ::core::ffi::c_int as isize) = (if n > 3 as uint32_t {
        toks_ld32(
            idv.offset(3 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_void,
        ) & TOKS_ID_MASK as uint32_t
    } else {
        0 as uint32_t
    }) | (tw >> 32 as ::core::ffi::c_int) as uint32_t;
}
#[inline]
unsafe extern "C" fn bpe_cache_fill(
    mut bucket: *mut uint8_t,
    mut k: bpe_key,
    mut val: *const uint32_t,
) {
    memcpy(
        bucket.offset(16 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_void,
        bucket as *const ::core::ffi::c_void,
        16 as size_t,
    );
    memcpy(
        bucket.offset(48 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_void,
        bucket.offset(32 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_void,
        16 as size_t,
    );
    memcpy(
        bucket as *mut ::core::ffi::c_void,
        &raw mut k.lo as *const ::core::ffi::c_void,
        8 as size_t,
    );
    memcpy(
        bucket.offset(8 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_void,
        &raw mut k.hi as *const ::core::ffi::c_void,
        8 as size_t,
    );
    memcpy(
        bucket.offset(32 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_void,
        val as *const ::core::ffi::c_void,
        16 as size_t,
    );
}
#[inline]
unsafe extern "C" fn bpe_words_put(
    mut words: *mut uint8_t,
    mut mask: uint64_t,
    mut h: uint32_t,
    mut k: bpe_key,
    mut val: *const uint32_t,
) -> ::core::ffi::c_int {
    let mut which: uint32_t = 0 as uint32_t;
    while which < 2 as uint32_t {
        let mut b: *mut uint8_t = words
            .offset(
                ((if which == 0 as uint32_t {
                    h
                } else {
                    h >> 16 as ::core::ffi::c_int | h << 16 as ::core::ffi::c_int
                }) as uint64_t & mask)
                    .wrapping_mul(TOKS_BUCKET as uint64_t) as isize,
            );
        let mut way: uint32_t = 0 as uint32_t;
        while way < 2 as uint32_t {
            if *b.offset(15 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint
                != 0 as ::core::ffi::c_uint
            {
                way = way.wrapping_add(1);
                b = b.offset(16 as ::core::ffi::c_int as isize);
            } else {
                memcpy(
                    b as *mut ::core::ffi::c_void,
                    &raw mut k.lo as *const ::core::ffi::c_void,
                    8 as size_t,
                );
                memcpy(
                    b.offset(8 as ::core::ffi::c_int as isize)
                        as *mut ::core::ffi::c_void,
                    &raw mut k.hi as *const ::core::ffi::c_void,
                    8 as size_t,
                );
                memcpy(
                    b.offset(32 as ::core::ffi::c_int as isize)
                        as *mut ::core::ffi::c_void,
                    val as *const ::core::ffi::c_void,
                    16 as size_t,
                );
                return 1 as ::core::ffi::c_int;
            }
        }
        which = which.wrapping_add(1);
    }
    return 0 as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn bpe_words_probe(
    mut words: *const uint8_t,
    mut mask: uint64_t,
    mut h: uint32_t,
    mut k: bpe_key,
) -> *const uint8_t {
    let mut v: *const uint8_t = bpe_bucket_get(
        words
            .offset(
                (h as uint64_t & mask).wrapping_mul(TOKS_BUCKET as uint64_t) as isize,
            ),
        k,
    );
    if v.is_null() {
        v = bpe_bucket_get(
            words
                .offset(
                    ((h >> 16 as ::core::ffi::c_int | h << 16 as ::core::ffi::c_int)
                        as uint64_t & mask)
                        .wrapping_mul(TOKS_BUCKET as uint64_t) as isize,
                ),
            k,
        );
    }
    return v;
}
pub const TOKS_DA_FREE: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
#[inline]
unsafe extern "C" fn key_less(
    mut kb: *const uint8_t,
    mut a: *const ukey,
    mut b: *const ukey,
) -> ::core::ffi::c_int {
    let mut m: uint32_t = if (*a).len < (*b).len { (*a).len } else { (*b).len };
    let mut c: ::core::ffi::c_int = if m != 0 as uint32_t {
        memcmp(
            kb.offset((*a).off as isize) as *const ::core::ffi::c_void,
            kb.offset((*b).off as isize) as *const ::core::ffi::c_void,
            m as size_t,
        )
    } else {
        0 as ::core::ffi::c_int
    };
    if c != 0 as ::core::ffi::c_int {
        return (c < 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
    }
    if (*a).len != (*b).len {
        return ((*a).len < (*b).len) as ::core::ffi::c_int;
    }
    return ((*a).id < (*b).id) as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn sift(
    mut kb: *const uint8_t,
    mut k: *mut ukey,
    mut root: uint64_t,
    mut end: uint64_t,
) {
    while (2 as uint64_t).wrapping_mul(root).wrapping_add(1 as uint64_t) < end {
        let mut c: uint64_t = (2 as uint64_t)
            .wrapping_mul(root)
            .wrapping_add(1 as uint64_t);
        if c.wrapping_add(1 as uint64_t) < end
            && key_less(
                kb,
                k.offset(c as isize) as *mut ukey,
                k.offset(c.wrapping_add(1 as uint64_t) as isize) as *mut ukey,
            ) != 0
        {
            c = c.wrapping_add(1);
        }
        if key_less(
            kb,
            k.offset(root as isize) as *mut ukey,
            k.offset(c as isize) as *mut ukey,
        ) == 0
        {
            return;
        }
        let mut t: ukey = *k.offset(root as isize);
        *k.offset(root as isize) = *k.offset(c as isize);
        *k.offset(c as isize) = t;
        root = c;
    }
}
#[inline]
unsafe extern "C" fn heap_sort(
    mut kb: *const uint8_t,
    mut k: *mut ukey,
    mut n: uint64_t,
) {
    if n < 2 as uint64_t {
        return;
    }
    let mut i: uint64_t = n.wrapping_div(2 as uint64_t);
    while i > 0 as uint64_t {
        sift(kb, k, i.wrapping_sub(1 as uint64_t), n);
        i = i.wrapping_sub(1);
    }
    let mut e: uint64_t = n.wrapping_sub(1 as uint64_t);
    while e > 0 as uint64_t {
        let mut t: ukey = *k.offset(0 as ::core::ffi::c_int as isize);
        *k.offset(0 as ::core::ffi::c_int as isize) = *k.offset(e as isize);
        *k.offset(e as isize) = t;
        sift(kb, k, 0 as uint64_t, e);
        e = e.wrapping_sub(1);
    }
}
#[inline]
unsafe extern "C" fn da_init(
    mut arr: *mut int32_t,
    mut cap: uint64_t,
    mut from: uint64_t,
) {
    let mut i: uint64_t = from;
    while i < cap {
        *arr.offset(i as isize) = 0 as ::core::ffi::c_int as int32_t;
        *arr.offset(cap.wrapping_add(i) as isize) = TOKS_DA_FREE as int32_t;
        *arr.offset((2 as uint64_t).wrapping_mul(cap).wrapping_add(i) as isize) = -(1
            as ::core::ffi::c_int) as int32_t;
        i = i.wrapping_add(1);
    }
}
#[inline]
unsafe extern "C" fn da_grow(
    mut arr: *mut *mut int32_t,
    mut cap: *mut uint64_t,
    mut need: uint64_t,
) -> ::core::ffi::c_int {
    let mut oc: uint64_t = *cap;
    let mut nc: uint64_t = oc;
    while nc < need {
        nc = nc.wrapping_mul(2 as uint64_t);
    }
    if nc == oc {
        return 0 as ::core::ffi::c_int;
    }
    if nc > 0x7fffffff as uint64_t {
        return -(1 as ::core::ffi::c_int);
    }
    let mut na: *mut int32_t = toks_plat_alloc(nc.wrapping_mul(12 as uint64_t))
        as *mut int32_t;
    if na.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    let mut c: uint32_t = 0 as uint32_t;
    while c < 3 as uint32_t {
        memcpy(
            na.offset((c as uint64_t).wrapping_mul(nc) as isize)
                as *mut ::core::ffi::c_void,
            (*arr).offset((c as uint64_t).wrapping_mul(oc) as isize)
                as *const ::core::ffi::c_void,
            (oc as size_t).wrapping_mul(4 as size_t),
        );
        c = c.wrapping_add(1);
    }
    da_init(na, nc, oc);
    toks_plat_free(*arr as *mut ::core::ffi::c_void, oc.wrapping_mul(12 as uint64_t));
    *arr = na;
    *cap = nc;
    return 0 as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn toks_da_build(
    mut kb: *const uint8_t,
    mut keys: *mut ukey,
    mut nkp: *mut uint64_t,
    mut arrp: *mut *mut int32_t,
    mut capp: *mut uint64_t,
    mut da_lenp: *mut uint64_t,
    mut why: *mut *const ::core::ffi::c_char,
) -> int64_t {
    let mut nk: uint64_t = *nkp;
    heap_sort(kb, keys, nk);
    let mut w: uint64_t = 0 as uint64_t;
    let mut i: uint64_t = 0 as uint64_t;
    while i < nk {
        if w > 0 as uint64_t
            && (*keys.offset(w.wrapping_sub(1 as uint64_t) as isize)).len
                == (*keys.offset(i as isize)).len
            && memcmp(
                kb
                    .offset(
                        (*keys.offset(w.wrapping_sub(1 as uint64_t) as isize)).off
                            as isize,
                    ) as *const ::core::ffi::c_void,
                kb.offset((*keys.offset(i as isize)).off as isize)
                    as *const ::core::ffi::c_void,
                (*keys.offset(i as isize)).len as size_t,
            ) == 0 as ::core::ffi::c_int
        {
            *keys.offset(w.wrapping_sub(1 as uint64_t) as isize) = *keys
                .offset(i as isize);
        } else {
            let fresh2 = w;
            w = w.wrapping_add(1);
            *keys.offset(fresh2 as isize) = *keys.offset(i as isize);
        }
        i = i.wrapping_add(1);
    }
    nk = w;
    *nkp = nk;
    let mut nodes: uint64_t = 1 as uint64_t;
    let mut i_0: uint64_t = 0 as uint64_t;
    while i_0 < nk {
        let mut l: uint32_t = 0 as uint32_t;
        if i_0 > 0 as uint64_t {
            let mut m: uint32_t = if (*keys.offset(i_0 as isize)).len
                < (*keys.offset(i_0.wrapping_sub(1 as uint64_t) as isize)).len
            {
                (*keys.offset(i_0 as isize)).len
            } else {
                (*keys.offset(i_0.wrapping_sub(1 as uint64_t) as isize)).len
            };
            while l < m
                && *kb
                    .offset(
                        (*keys.offset(i_0 as isize)).off.wrapping_add(l as uint64_t)
                            as isize,
                    ) as ::core::ffi::c_int
                    == *kb
                        .offset(
                            (*keys.offset(i_0.wrapping_sub(1 as uint64_t) as isize))
                                .off
                                .wrapping_add(l as uint64_t) as isize,
                        ) as ::core::ffi::c_int
            {
                l = l.wrapping_add(1);
            }
        }
        nodes = nodes
            .wrapping_add((*keys.offset(i_0 as isize)).len.wrapping_sub(l) as uint64_t);
        i_0 = i_0.wrapping_add(1);
    }
    let mut cap: uint64_t = (2 as uint64_t)
        .wrapping_mul(nodes)
        .wrapping_add(512 as uint64_t);
    if cap > 0x7fffffff as uint64_t {
        *why = b"trie too large\0" as *const u8 as *const ::core::ffi::c_char;
        return TOKS_E_LIMIT as int64_t;
    }
    let mut q_bytes: uint64_t = nodes
        .wrapping_mul(::core::mem::size_of::<qent>() as uint64_t)
        .wrapping_add(64 as uint64_t);
    let mut q: *mut qent = toks_plat_alloc(q_bytes) as *mut qent;
    let mut arr: *mut int32_t = toks_plat_alloc(cap.wrapping_mul(12 as uint64_t))
        as *mut int32_t;
    if q.is_null() || arr.is_null() {
        if !q.is_null() {
            toks_plat_free(q as *mut ::core::ffi::c_void, q_bytes);
        }
        if !arr.is_null() {
            toks_plat_free(
                arr as *mut ::core::ffi::c_void,
                cap.wrapping_mul(12 as uint64_t),
            );
        }
        *why = b"trie memory\0" as *const u8 as *const ::core::ffi::c_char;
        return TOKS_E_NOMEM as int64_t;
    }
    da_init(arr, cap, 0 as uint64_t);
    *arr.offset(cap as isize) = -(2 as ::core::ffi::c_int) as int32_t;
    let mut qt: uint64_t = 0 as uint64_t;
    let mut ncp: uint64_t = 1 as uint64_t;
    let mut da_len: uint64_t = 1 as uint64_t;
    (*q.offset(qt as isize)).slot = 0 as ::core::ffi::c_uint as uint32_t;
    (*q.offset(qt as isize)).lo = 0 as ::core::ffi::c_uint as uint32_t;
    (*q.offset(qt as isize)).hi = nk as uint32_t;
    (*q.offset(qt as isize)).depth = 0 as ::core::ffi::c_uint as uint32_t;
    qt = qt.wrapping_add(1);
    while qt > 0 as uint64_t {
        qt = qt.wrapping_sub(1);
        let mut e: qent = *q.offset(qt as isize);
        let mut lo: uint32_t = e.lo;
        let mut hi: uint32_t = e.hi;
        let mut d: uint32_t = e.depth;
        if lo < hi && (*keys.offset(lo as isize)).len == d {
            *arr
                .offset(
                    (2 as uint64_t).wrapping_mul(cap).wrapping_add(e.slot as uint64_t)
                        as isize,
                ) = (*keys.offset(lo as isize)).id as int32_t;
            lo = lo.wrapping_add(1);
        }
        let mut lab: [uint32_t; 256] = [0; 256];
        let mut llo: [uint32_t; 256] = [0; 256];
        let mut lhi: [uint32_t; 256] = [0; 256];
        let mut nl: uint32_t = 0 as uint32_t;
        let mut j: uint32_t = lo;
        while j < hi {
            let mut c: uint8_t = *kb
                .offset(
                    (*keys.offset(j as isize)).off.wrapping_add(d as uint64_t) as isize,
                );
            let mut k: uint32_t = j.wrapping_add(1 as uint32_t);
            while k < hi
                && *kb
                    .offset(
                        (*keys.offset(k as isize)).off.wrapping_add(d as uint64_t)
                            as isize,
                    ) as ::core::ffi::c_int == c as ::core::ffi::c_int
            {
                k = k.wrapping_add(1);
            }
            lab[nl as usize] = c as uint32_t;
            llo[nl as usize] = j;
            lhi[nl as usize] = k;
            nl = nl.wrapping_add(1);
            j = k;
        }
        if nl == 0 as uint32_t {
            continue;
        }
        let mut pos: uint64_t = (if (lab[0 as ::core::ffi::c_int as usize] as uint64_t)
            .wrapping_add(1 as uint64_t) > ncp
        {
            (lab[0 as ::core::ffi::c_int as usize] as uint64_t)
                .wrapping_add(1 as uint64_t)
        } else {
            ncp
        })
            .wrapping_sub(1 as uint64_t);
        let mut nonzero: uint64_t = 0 as uint64_t;
        let mut b: uint64_t = 0 as uint64_t;
        let mut first: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
        loop {
            pos = pos.wrapping_add(1);
            if pos.wrapping_add(257 as uint64_t) >= cap {
                if da_grow(&raw mut arr, &raw mut cap, pos.wrapping_add(258 as uint64_t))
                    != 0 as ::core::ffi::c_int
                {
                    toks_plat_free(q as *mut ::core::ffi::c_void, q_bytes);
                    toks_plat_free(
                        arr as *mut ::core::ffi::c_void,
                        cap.wrapping_mul(12 as uint64_t),
                    );
                    *why = b"trie over 2^31 cells\0" as *const u8
                        as *const ::core::ffi::c_char;
                    return TOKS_E_LIMIT as int64_t;
                }
            }
            if *arr.offset(cap.wrapping_add(pos) as isize) != TOKS_DA_FREE as int32_t {
                nonzero = nonzero.wrapping_add(1);
            } else {
                if first != 0 {
                    ncp = pos;
                    first = 0 as ::core::ffi::c_int;
                }
                b = pos.wrapping_sub(lab[0 as ::core::ffi::c_int as usize] as uint64_t);
                let mut ok: uint32_t = 1 as uint32_t;
                let mut x: uint32_t = 1 as uint32_t;
                while x < nl {
                    if *arr
                        .offset(
                            cap.wrapping_add(b).wrapping_add(lab[x as usize] as uint64_t)
                                as isize,
                        ) != TOKS_DA_FREE as int32_t
                    {
                        ok = 0 as ::core::ffi::c_uint as uint32_t;
                        break;
                    } else {
                        x = x.wrapping_add(1);
                    }
                }
                if ok != 0 {
                    break;
                }
            }
        }
        if nonzero.wrapping_mul(20 as uint64_t)
            >= pos
                .wrapping_sub(ncp)
                .wrapping_add(1 as uint64_t)
                .wrapping_mul(19 as uint64_t)
        {
            ncp = pos;
        }
        *arr.offset(e.slot as isize) = b as int32_t;
        let mut x_0: uint32_t = 0 as uint32_t;
        while x_0 < nl {
            let mut sl: uint64_t = b.wrapping_add(lab[x_0 as usize] as uint64_t);
            *arr.offset(cap.wrapping_add(sl) as isize) = e.slot as int32_t;
            (*q.offset(qt as isize)).slot = sl as uint32_t;
            (*q.offset(qt as isize)).lo = llo[x_0 as usize];
            (*q.offset(qt as isize)).hi = lhi[x_0 as usize];
            (*q.offset(qt as isize)).depth = d.wrapping_add(1 as uint32_t);
            qt = qt.wrapping_add(1);
            if sl.wrapping_add(1 as uint64_t) > da_len {
                da_len = sl.wrapping_add(1 as uint64_t);
            }
            x_0 = x_0.wrapping_add(1);
        }
    }
    toks_plat_free(q as *mut ::core::ffi::c_void, q_bytes);
    *arrp = arr;
    *capp = cap;
    *da_lenp = da_len;
    return 0 as int64_t;
}
pub const WP_SMAX: ::core::ffi::c_uint = 64 as ::core::ffi::c_uint;
unsafe extern "C" fn wp_eq(
    mut key: *const uint8_t,
    mut p: *const uint8_t,
    mut n: uint64_t,
    mut fold: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut i: uint64_t = 0 as uint64_t;
    while n.wrapping_sub(i) >= 8 as uint64_t {
        let mut a: uint64_t = wp_load(key.offset(i as isize), 8 as uint64_t);
        let mut b: uint64_t = wp_load(p.offset(i as isize), 8 as uint64_t);
        if (if fold != 0 { wp_fold(b) } else { b }) != a {
            return 0 as ::core::ffi::c_int;
        }
        i = i.wrapping_add(8 as uint64_t);
    }
    if i < n {
        let mut a_0: uint64_t = wp_load(key.offset(i as isize), n.wrapping_sub(i));
        let mut b_0: uint64_t = wp_load(p.offset(i as isize), n.wrapping_sub(i));
        if (if fold != 0 { wp_fold(b_0) } else { b_0 }) != a_0 {
            return 0 as ::core::ffi::c_int;
        }
    }
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn wp_find(
    mut t: *const toks_wp_tables,
    mut tab: *const toks_wp_entry,
    mut mask: uint64_t,
    mut h: uint32_t,
    mut p: *const uint8_t,
    mut n: uint64_t,
    mut fold: ::core::ffi::c_int,
) -> int64_t {
    if tab.is_null() {
        return -(1 as ::core::ffi::c_int) as int64_t;
    }
    let mut i: uint64_t = h as uint64_t & mask;
    let mut k: uint64_t = 0 as uint64_t;
    while k <= mask {
        let mut e: *const toks_wp_entry = tab.offset(i as isize) as *const toks_wp_entry;
        if (*e).len == 0 as uint32_t {
            return -(1 as ::core::ffi::c_int) as int64_t;
        }
        if (*e).h == h && (*e).len as uint64_t == n
            && wp_eq((*t).keys.offset((*e).off as isize), p, n, fold) != 0
        {
            return (*e).id as int64_t;
        }
        i = i.wrapping_add(1 as uint64_t) & mask;
        k = k.wrapping_add(1);
    }
    return -(1 as ::core::ffi::c_int) as int64_t;
}
#[no_mangle]
pub unsafe extern "C" fn toks_wp_piece_ids(
    mut t: *const toks_wp_tables,
    mut p: *const uint8_t,
    mut len: uint64_t,
    mut flags: uint32_t,
    mut out: *mut uint32_t,
    mut probes: *mut uint64_t,
) -> uint64_t {
    if flags & (TOKS_WPP_OVER as uint32_t | TOKS_WPP_INVALID as uint32_t) != 0
        || len == 0 as uint64_t
    {
        toks_st32(out as *mut ::core::ffi::c_void, (*t).unk_id);
        return 1 as uint64_t;
    }
    let mut fold: ::core::ffi::c_int = ((*t).flags & TOKS_WPF_LOWER as uint32_t
        != 0 as uint32_t) as ::core::ffi::c_int;
    if !(*t).wcell.is_null() {
        let mut n: uint64_t = 0 as uint64_t;
        let mut s: uint64_t = 0 as uint64_t;
        while s < len {
            let mut cell: *const toks_wp_cell = if s == 0 as uint64_t {
                (*t).wcell as *const toks_wp_cell
            } else {
                (*t).ccell as *const toks_wp_cell
            };
            let mut term: *const int32_t = if s == 0 as uint64_t {
                (*t).wterm
            } else {
                (*t).cterm
            };
            let mut node: uint32_t = 0 as uint32_t;
            let mut nb: uint32_t = (*cell.offset(0 as ::core::ffi::c_int as isize)).base;
            let mut hit: int32_t = -(1 as int32_t);
            let mut e: uint64_t = s;
            let mut i: uint64_t = s;
            while i < len {
                let mut c: uint32_t = *p.offset(i as isize) as uint32_t;
                if fold != 0 && c.wrapping_sub(0x41 as uint32_t) < 26 as uint32_t {
                    c = (c as ::core::ffi::c_uint | 0x20 as ::core::ffi::c_uint)
                        as uint32_t;
                }
                let mut x: uint32_t = (nb & TOKS_WP_BASE as uint32_t).wrapping_add(c);
                if (*cell.offset(x as isize)).check != node as int32_t {
                    break;
                }
                node = x;
                nb = (*cell.offset(x as isize)).base;
                if nb & TOKS_WP_TERM as uint32_t != 0 {
                    hit = *term.offset(x as isize);
                    e = i.wrapping_add(1 as uint64_t);
                }
                i = i.wrapping_add(1);
            }
            *probes = (*probes).wrapping_add(1 as uint64_t);
            if hit < 0 as int32_t {
                toks_st32(out as *mut ::core::ffi::c_void, (*t).unk_id);
                return 1 as uint64_t;
            }
            let fresh0 = n;
            n = n.wrapping_add(1);
            toks_st32(
                out.offset(fresh0 as isize) as *mut ::core::ffi::c_void,
                hit as uint32_t,
            );
            s = e;
        }
        return n;
    }
    if len <= (*t).word_maxlen as uint64_t {
        let mut id: int64_t = wp_find(
            t,
            (*t).word,
            (*t).word_mask,
            wp_hash(p, len, fold),
            p,
            len,
            fold,
        );
        *probes = (*probes).wrapping_add(1 as uint64_t);
        if id >= 0 as int64_t {
            toks_st32(out as *mut ::core::ffi::c_void, id as uint32_t);
            return 1 as uint64_t;
        }
    }
    let mut n_0: uint64_t = 0 as uint64_t;
    let mut s_0: uint64_t = 0 as uint64_t;
    let mut st: [uint32_t; 65] = [0; 65];
    while s_0 < len {
        let mut tab: *const toks_wp_entry = if s_0 == 0 as uint64_t {
            (*t).word
        } else {
            (*t).cont
        };
        let mut mask: uint64_t = if s_0 == 0 as uint64_t {
            (*t).word_mask
        } else {
            (*t).cont_mask
        };
        let mut maxlen: uint64_t = (if s_0 == 0 as uint64_t {
            (*t).word_maxlen
        } else {
            (*t).cont_maxlen
        }) as uint64_t;
        let mut hi: uint64_t = if len.wrapping_sub(s_0) < maxlen {
            len.wrapping_sub(s_0)
        } else {
            maxlen
        };
        let mut nst: uint64_t = if hi.wrapping_div(8 as uint64_t) < WP_SMAX as uint64_t {
            hi.wrapping_div(8 as uint64_t)
        } else {
            WP_SMAX as uint64_t
        };
        st[0 as ::core::ffi::c_int as usize] = TOKS_WP_HSEED as uint32_t;
        let mut m: uint64_t = 0 as uint64_t;
        while m < nst {
            let mut w: uint64_t = wp_load(
                p.offset(s_0 as isize).offset((8 as uint64_t).wrapping_mul(m) as isize),
                8 as uint64_t,
            );
            st[m.wrapping_add(1 as uint64_t) as usize] = toks_crc32c_u64(
                st[m as usize],
                if fold != 0 { wp_fold(w) } else { w },
            );
            m = m.wrapping_add(1);
        }
        let mut hit_0: int64_t = -(1 as ::core::ffi::c_int) as int64_t;
        let mut e_0: uint64_t = s_0.wrapping_add(hi);
        while e_0 > s_0 {
            if !(e_0 < len
                && *p.offset(e_0 as isize) as ::core::ffi::c_uint
                    & 0xc0 as ::core::ffi::c_uint == 0x80 as ::core::ffi::c_uint)
            {
                if !(s_0 == 0 as uint64_t && e_0 == len) {
                    let mut k: uint64_t = e_0.wrapping_sub(s_0);
                    let mut h: uint32_t = 0;
                    if k.wrapping_div(8 as uint64_t) <= nst {
                        let mut m_0: uint64_t = k.wrapping_div(8 as uint64_t);
                        h = st[m_0 as usize];
                        if k.wrapping_rem(8 as uint64_t) != 0 {
                            let mut w_0: uint64_t = wp_load(
                                p
                                    .offset(s_0 as isize)
                                    .offset((8 as uint64_t).wrapping_mul(m_0) as isize),
                                k.wrapping_rem(8 as uint64_t),
                            );
                            h = toks_crc32c_u64(
                                h,
                                if fold != 0 { wp_fold(w_0) } else { w_0 },
                            );
                        }
                        h = wp_hash_end(h, k);
                    } else {
                        h = wp_hash(p.offset(s_0 as isize), k, fold);
                    }
                    *probes = (*probes).wrapping_add(1 as uint64_t);
                    hit_0 = wp_find(t, tab, mask, h, p.offset(s_0 as isize), k, fold);
                    if hit_0 >= 0 as int64_t {
                        break;
                    }
                }
            }
            e_0 = e_0.wrapping_sub(1);
        }
        if hit_0 < 0 as int64_t {
            toks_st32(out as *mut ::core::ffi::c_void, (*t).unk_id);
            return 1 as uint64_t;
        }
        let fresh1 = n_0;
        n_0 = n_0.wrapping_add(1);
        toks_st32(
            out.offset(fresh1 as isize) as *mut ::core::ffi::c_void,
            hit_0 as uint32_t,
        );
        s_0 = e_0;
    }
    return n_0;
}
#[inline]
unsafe extern "C" fn wp_key(
    mut p: *const uint8_t,
    mut len: uint64_t,
    mut avail: uint64_t,
    mut fold: ::core::ffi::c_int,
) -> bpe_key {
    let mut k: bpe_key = bpe_key_at(p, avail, 0 as uint64_t, len);
    if fold != 0 {
        k.lo = wp_fold(k.lo);
        k.hi = wp_fold(k.hi);
    }
    return k;
}
#[inline]
unsafe extern "C" fn wp_whash(mut k: bpe_key) -> uint32_t {
    return ((k.lo as ::core::ffi::c_ulonglong
        ^ (k.hi as ::core::ffi::c_ulonglong).wrapping_mul(TOKS_FIB64))
        .wrapping_mul(0xd6e8feb86659fd93 as ::core::ffi::c_ulonglong)
        >> 32 as ::core::ffi::c_int) as uint32_t;
}
pub const WP_BATCH: ::core::ffi::c_uint = 64 as ::core::ffi::c_uint;
#[no_mangle]
pub unsafe extern "C" fn toks_wp_encode_c(
    mut t: *const toks_wp_tables,
    mut a: *mut toks_wp_encode_args,
) -> uint64_t {
    let mut n: uint64_t = 0 as uint64_t;
    let mut hits: uint64_t = 0 as uint64_t;
    let mut misses: uint64_t = 0 as uint64_t;
    let mut probes: uint64_t = 0 as uint64_t;
    let mut fold: ::core::ffi::c_int = ((*t).flags & TOKS_WPF_LOWER as uint32_t
        != 0 as uint32_t) as ::core::ffi::c_int;
    let mut kq: [bpe_key; 64] = [bpe_key { lo: 0, hi: 0 }; 64];
    let mut hq: [uint32_t; 64] = [0; 64];
    let mut i: uint64_t = 0 as uint64_t;
    while i < (*a).n {
        if !(*t).wtab.is_null() && i.wrapping_rem(WP_BATCH as uint64_t) == 0 as uint64_t
        {
            let mut j: uint64_t = i;
            while j < (*a).n && j < i.wrapping_add(WP_BATCH as uint64_t) {
                let mut pj: *const toks_wp_piece = (*a).pieces.offset(j as isize)
                    as *const toks_wp_piece;
                if !((*pj).len.wrapping_sub(1 as uint32_t) as uint64_t
                    >= TOKS_KEY_MAXLEN as uint64_t
                    || (*pj).flags
                        & (TOKS_WPP_OVER as uint32_t | TOKS_WPP_INVALID as uint32_t)
                        != 0)
                {
                    let mut mj: ::core::ffi::c_int = ((*pj).flags
                        & TOKS_WPP_MAT as uint32_t != 0 as uint32_t)
                        as ::core::ffi::c_int;
                    let mut lim: uint64_t = if mj != 0 {
                        (*a).mat_len
                    } else {
                        (*a).text_len
                    };
                    kq[j.wrapping_sub(i) as usize] = wp_key(
                        (if mj != 0 { (*a).mat } else { (*a).text })
                            .offset((*pj).off as isize),
                        (*pj).len as uint64_t,
                        if lim > (*pj).off.wrapping_add((*pj).len) as uint64_t {
                            lim.wrapping_sub((*pj).off as uint64_t)
                        } else {
                            (*pj).len as uint64_t
                        },
                        fold,
                    );
                    hq[j.wrapping_sub(i) as usize] = wp_whash(
                        kq[j.wrapping_sub(i) as usize],
                    );
                    crate::prefetch_read((*t).wtab.add(((hq[j.wrapping_sub(i) as usize] as u64 & (*t).wtab_mask) * TOKS_BUCKET as u64) as usize));
                }
                j = j.wrapping_add(1);
            }
        }
        let mut pc: *const toks_wp_piece = (*a).pieces.offset(i as isize)
            as *const toks_wp_piece;
        let mut mat: ::core::ffi::c_int = ((*pc).flags & TOKS_WPP_MAT as uint32_t
            != 0 as uint32_t) as ::core::ffi::c_int;
        let mut p: *const uint8_t = (if mat != 0 { (*a).mat } else { (*a).text })
            .offset((*pc).off as isize);
        let mut len: uint64_t = (*pc).len as uint64_t;
        if !(*t).wtab.is_null()
            && len.wrapping_sub(1 as uint64_t) < TOKS_KEY_MAXLEN as uint64_t
            && (*pc).flags & (TOKS_WPP_OVER as uint32_t | TOKS_WPP_INVALID as uint32_t)
                == 0
        {
            let mut k: bpe_key = kq[i.wrapping_rem(WP_BATCH as uint64_t) as usize];
            let mut h: uint32_t = hq[i.wrapping_rem(WP_BATCH as uint64_t) as usize];
            let mut v: *const uint8_t = bpe_words_probe((*t).wtab, (*t).wtab_mask, h, k);
            if !v.is_null() {
                let mut id: uint32_t = 0;
                memcpy(
                    &raw mut id as *mut ::core::ffi::c_void,
                    v as *const ::core::ffi::c_void,
                    4 as size_t,
                );
                toks_st32(
                    (*a).out.offset(n as isize) as *mut ::core::ffi::c_void,
                    id & TOKS_ID_MASK as uint32_t,
                );
                n = n.wrapping_add(1);
                hits = hits.wrapping_add(1);
                probes = probes.wrapping_add(1);
            } else {
                let mut bucket: *mut uint8_t = if !(*a).cache.is_null() {
                    (*a)
                        .cache
                        .offset(
                            (h as uint64_t & (*a).cache_mask)
                                .wrapping_mul(TOKS_BUCKET as uint64_t) as isize,
                        )
                } else {
                    ::core::ptr::null_mut::<uint8_t>()
                };
                if !bucket.is_null()
                    && {
                        v = bpe_cache_get(bucket, k, (*a).tw);
                        !v.is_null()
                    }
                {
                    let mut w: [uint32_t; 4] = [0; 4];
                    memcpy(
                        &raw mut w as *mut uint32_t as *mut ::core::ffi::c_void,
                        v as *const ::core::ffi::c_void,
                        16 as size_t,
                    );
                    let mut m: uint32_t = w[0 as ::core::ffi::c_int as usize]
                        >> TOKS_VAL_COUNT_SHIFT;
                    let mut j_0: uint32_t = 0 as uint32_t;
                    while j_0 < m {
                        toks_st32(
                            (*a).out.offset(n as isize).offset(j_0 as isize)
                                as *mut ::core::ffi::c_void,
                            w[j_0 as usize] & TOKS_ID_MASK as uint32_t,
                        );
                        j_0 = j_0.wrapping_add(1);
                    }
                    n = n.wrapping_add(m as uint64_t);
                    misses = misses.wrapping_add(1);
                } else {
                    let mut m_0: uint64_t = toks_wp_piece_ids(
                        t,
                        p,
                        len,
                        (*pc).flags,
                        (*a).out.offset(n as isize),
                        &raw mut probes,
                    );
                    if !bucket.is_null() && m_0 <= 4 as uint64_t {
                        let mut val: [uint32_t; 4] = [0; 4];
                        bpe_val_pack_tag(
                            &raw mut val as *mut uint32_t,
                            (*a).out.offset(n as isize),
                            m_0 as uint32_t,
                            (*a).tw,
                        );
                        bpe_cache_fill(
                            bucket,
                            k,
                            &raw mut val as *mut uint32_t as *const uint32_t,
                        );
                    }
                    n = n.wrapping_add(m_0);
                    misses = misses.wrapping_add(1);
                }
            }
        } else {
            let mut before: uint64_t = probes;
            let mut k_0: uint64_t = toks_wp_piece_ids(
                t,
                p,
                len,
                (*pc).flags,
                (*a).out.offset(n as isize),
                &raw mut probes,
            );
            if probes.wrapping_sub(before) <= 1 as uint64_t && k_0 == 1 as uint64_t {
                hits = hits.wrapping_add(1);
            } else {
                misses = misses.wrapping_add(1);
            }
            n = n.wrapping_add(k_0);
        }
        i = i.wrapping_add(1);
    }
    (*a).n_out = n;
    (*a).hits = hits;
    (*a).misses = misses;
    (*a).probes = probes;
    return n;
}
unsafe extern "C" fn pow2_at_least(mut x: uint64_t) -> uint64_t {
    let mut p: uint64_t = 16 as uint64_t;
    while p < x {
        p <<= 1 as ::core::ffi::c_int;
    }
    return p;
}
unsafe extern "C" fn wtab_buckets(mut v: *const toks_wp_vocab) -> uint64_t {
    let mut n: uint64_t = 0 as uint64_t;
    let mut i: uint32_t = 0 as uint32_t;
    while i < (*v).n {
        n = n
            .wrapping_add(
                ((*(*v).len.offset(i as isize)).wrapping_sub(1 as uint32_t)
                    < TOKS_KEY_MAXLEN as uint32_t) as ::core::ffi::c_int as uint64_t,
            );
        i = i.wrapping_add(1);
    }
    return if n != 0 as uint64_t {
        bpe_pow2(n.wrapping_div(2 as uint64_t).wrapping_add(1 as uint64_t))
    } else {
        0 as uint64_t
    };
}
#[no_mangle]
pub unsafe extern "C" fn toks_wp_tables_bytes(mut v: *const toks_wp_vocab) -> uint64_t {
    let mut keys: uint64_t = 0 as uint64_t;
    let mut i: uint32_t = 0 as uint32_t;
    while i < (*v).n {
        keys = keys.wrapping_add(*(*v).len.offset(i as isize) as uint64_t);
        i = i.wrapping_add(1);
    }
    let mut slots: uint64_t = pow2_at_least(
        (2 as uint64_t).wrapping_mul((*v).n as uint64_t).wrapping_add(2 as uint64_t),
    );
    return (2 as uint64_t)
        .wrapping_mul(slots)
        .wrapping_mul(::core::mem::size_of::<toks_wp_entry>() as uint64_t)
        .wrapping_add(keys)
        .wrapping_add(wtab_buckets(v).wrapping_mul(TOKS_BUCKET as uint64_t))
        .wrapping_add(
            (4 as ::core::ffi::c_uint).wrapping_mul(64 as ::core::ffi::c_uint)
                as uint64_t,
        );
}
unsafe extern "C" fn wp_insert(
    mut tab: *mut toks_wp_entry,
    mut mask: uint64_t,
    mut keys: *const uint8_t,
    mut h: uint32_t,
    mut off: uint32_t,
    mut len: uint32_t,
    mut id: uint32_t,
) {
    let mut i: uint64_t = h as uint64_t & mask;
    let mut k: uint64_t = 0 as uint64_t;
    while k <= mask {
        let mut e: *mut toks_wp_entry = tab.offset(i as isize) as *mut toks_wp_entry;
        if (*e).len == 0 as uint32_t {
            (*e).h = h;
            (*e).id = id;
            (*e).off = off;
            (*e).len = len;
            return;
        }
        if (*e).h == h && (*e).len == len
            && memcmp(
                keys.offset((*e).off as isize) as *const ::core::ffi::c_void,
                keys.offset(off as isize) as *const ::core::ffi::c_void,
                len as size_t,
            ) == 0 as ::core::ffi::c_int
        {
            (*e).id = id;
            return;
        }
        i = i.wrapping_add(1 as uint64_t) & mask;
        k = k.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn toks_wp_build(
    mut t: *mut toks_wp_tables,
    mut ar: *mut toks_arena,
    mut v: *const toks_wp_vocab,
    mut p: *const toks_wp_params,
    mut err: *mut toks_err,
) -> int64_t {
    memset(
        t as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<toks_wp_tables>() as size_t,
    );
    (*err).code = 0 as int64_t;
    (*err).what = ::core::ptr::null::<::core::ffi::c_char>();
    if (*p).max_chars > TOKS_WP_MAX_CHARS as uint32_t {
        (*err).code = TOKS_E_UNSUPPORTED as int64_t;
        (*err).what = b"WordPiece max_input_chars_per_word above 1024\0" as *const u8
            as *const ::core::ffi::c_char;
        return TOKS_E_UNSUPPORTED as int64_t;
    }
    let mut total: uint64_t = 0 as uint64_t;
    let mut i: uint32_t = 0 as uint32_t;
    while i < (*v).n {
        if *(*v).len.offset(i as isize) > TOKS_WP_MAX_KEY as uint32_t {
            (*err).code = TOKS_E_LIMIT as int64_t;
            (*err).what = b"WordPiece vocab string over 65535 bytes\0" as *const u8
                as *const ::core::ffi::c_char;
            return TOKS_E_LIMIT as int64_t;
        }
        total = total.wrapping_add(*(*v).len.offset(i as isize) as uint64_t);
        i = i.wrapping_add(1);
    }
    if total as ::core::ffi::c_ulonglong
        >= (1 as ::core::ffi::c_ulonglong) << 32 as ::core::ffi::c_int
    {
        (*err).code = TOKS_E_LIMIT as int64_t;
        (*err).what = b"WordPiece vocab over 4 GiB\0" as *const u8
            as *const ::core::ffi::c_char;
        return TOKS_E_LIMIT as int64_t;
    }
    let mut keys: *mut uint8_t = toks_tab_ar(ar, total, 64 as uint64_t, TOKS_X_WP_KEYS)
        as *mut uint8_t;
    toks_ar_alloc(ar, 8 as uint64_t, 1 as uint64_t);
    let mut slots: uint64_t = pow2_at_least(
        (2 as uint64_t).wrapping_mul((*v).n as uint64_t).wrapping_add(2 as uint64_t),
    );
    let mut word: *mut toks_wp_entry = toks_tab_ar(
        ar,
        slots.wrapping_mul(::core::mem::size_of::<toks_wp_entry>() as uint64_t),
        64 as uint64_t,
        TOKS_X_WP_ENTRIES,
    ) as *mut toks_wp_entry;
    let mut cont: *mut toks_wp_entry = toks_tab_ar(
        ar,
        slots.wrapping_mul(::core::mem::size_of::<toks_wp_entry>() as uint64_t),
        64 as uint64_t,
        TOKS_X_WP_ENTRIES,
    ) as *mut toks_wp_entry;
    if keys.is_null() || word.is_null() || cont.is_null() {
        (*err).code = TOKS_E_NOMEM as int64_t;
        (*err).what = b"WordPiece tables\0" as *const u8 as *const ::core::ffi::c_char;
        return TOKS_E_NOMEM as int64_t;
    }
    memset(
        word as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        (slots as size_t).wrapping_mul(::core::mem::size_of::<toks_wp_entry>() as size_t),
    );
    memset(
        cont as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        (slots as size_t).wrapping_mul(::core::mem::size_of::<toks_wp_entry>() as size_t),
    );
    let mut off: uint64_t = 0 as uint64_t;
    let mut wmax: uint32_t = 0 as uint32_t;
    let mut cmax: uint32_t = 0 as uint32_t;
    let mut unk_found: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut unk_id: uint32_t = 0 as uint32_t;
    let mut i_0: uint32_t = 0 as uint32_t;
    while i_0 < (*v).n {
        let mut n: uint32_t = *(*v).len.offset(i_0 as isize);
        if !(n == 0 as uint32_t) {
            memcpy(
                keys.offset(off as isize) as *mut ::core::ffi::c_void,
                *(*v).str_0.offset(i_0 as isize) as *const ::core::ffi::c_void,
                n as size_t,
            );
            let mut ko: uint32_t = off as uint32_t;
            off = off.wrapping_add(n as uint64_t);
            wp_insert(
                word,
                slots.wrapping_sub(1 as uint64_t),
                keys,
                wp_hash(
                    keys.offset(ko as isize),
                    n as uint64_t,
                    0 as ::core::ffi::c_int,
                ),
                ko,
                n,
                *(*v).id.offset(i_0 as isize),
            );
            if n > wmax {
                wmax = n;
            }
            if n == (*p).unk_len
                && memcmp(
                    keys.offset(ko as isize) as *const ::core::ffi::c_void,
                    (*p).unk as *const ::core::ffi::c_void,
                    n as size_t,
                ) == 0 as ::core::ffi::c_int
            {
                unk_found = 1 as ::core::ffi::c_int;
                unk_id = *(*v).id.offset(i_0 as isize);
            }
            if n > (*p).prefix_len
                && memcmp(
                    keys.offset(ko as isize) as *const ::core::ffi::c_void,
                    (*p).prefix as *const ::core::ffi::c_void,
                    (*p).prefix_len as size_t,
                ) == 0 as ::core::ffi::c_int
            {
                let mut cn: uint32_t = n.wrapping_sub((*p).prefix_len);
                wp_insert(
                    cont,
                    slots.wrapping_sub(1 as uint64_t),
                    keys,
                    wp_hash(
                        keys.offset(ko as isize).offset((*p).prefix_len as isize),
                        cn as uint64_t,
                        0 as ::core::ffi::c_int,
                    ),
                    ko.wrapping_add((*p).prefix_len),
                    cn,
                    *(*v).id.offset(i_0 as isize),
                );
                if cn > cmax {
                    cmax = cn;
                }
            }
        }
        i_0 = i_0.wrapping_add(1);
    }
    if unk_found == 0 {
        (*err).code = TOKS_E_UNSUPPORTED as int64_t;
        (*err).what = b"WordPiece unk_token not in vocab\0" as *const u8
            as *const ::core::ffi::c_char;
        return TOKS_E_UNSUPPORTED as int64_t;
    }
    (*t).flags = (*p).flags;
    (*t).max_chars = (*p).max_chars;
    (*t).unk_id = unk_id;
    (*t).word_maxlen = wmax;
    (*t).cont_maxlen = cmax;
    (*t).prefix_len = (*p).prefix_len;
    let mut b: uint32_t = 0 as uint32_t;
    while b < 128 as uint32_t {
        (*t).ascii_cls[b as usize] = toks_wp_ascii_class((*p).flags, b);
        (*t).ascii_cls0[b as usize] = toks_wp_ascii_class(0 as uint32_t, b);
        b = b.wrapping_add(1);
    }
    (*t).word = word;
    (*t).word_mask = slots.wrapping_sub(1 as uint64_t);
    (*t).cont = cont;
    (*t).cont_mask = slots.wrapping_sub(1 as uint64_t);
    (*t).keys = keys;
    let mut wb: uint64_t = wtab_buckets(v);
    let mut wt: *mut uint8_t = (if wb != 0 as uint64_t {
        toks_tab_ar(
            ar,
            wb.wrapping_mul(TOKS_BUCKET as uint64_t),
            64 as uint64_t,
            TOKS_X_WORDS,
        )
    } else {
        NULL
    }) as *mut uint8_t;
    if wb != 0 as uint64_t && wt.is_null() {
        (*err).code = TOKS_E_NOMEM as int64_t;
        (*err).what = b"WordPiece tables\0" as *const u8 as *const ::core::ffi::c_char;
        return TOKS_E_NOMEM as int64_t;
    }
    if !wt.is_null() {
        memset(
            wt as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            (wb as size_t).wrapping_mul(TOKS_BUCKET as size_t),
        );
        let mut fold: ::core::ffi::c_int = ((*p).flags & TOKS_WPF_LOWER as uint32_t
            != 0 as uint32_t) as ::core::ffi::c_int;
        let mut i_1: uint32_t = 0 as uint32_t;
        while i_1 < (*v).n {
            let mut n_0: uint32_t = *(*v).len.offset(i_1 as isize);
            if !(n_0.wrapping_sub(1 as uint32_t) >= TOKS_KEY_MAXLEN as uint32_t) {
                let mut upper: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                let mut j: uint32_t = 0 as uint32_t;
                while j < n_0 {
                    upper
                        |= ((*(*(*v).str_0.offset(i_1 as isize)).offset(j as isize)
                            as ::core::ffi::c_uint)
                            .wrapping_sub(0x41 as ::core::ffi::c_uint)
                            < 26 as ::core::ffi::c_uint) as ::core::ffi::c_int;
                    j = j.wrapping_add(1);
                }
                if !(fold != 0 && upper != 0) {
                    let mut kb: [uint8_t; 16] = [0; 16];
                    bpe_key_make(
                        &raw mut kb as *mut uint8_t,
                        *(*v).str_0.offset(i_1 as isize),
                        n_0,
                    );
                    let mut k: bpe_key = bpe_key { lo: 0, hi: 0 };
                    memcpy(
                        &raw mut k.lo as *mut ::core::ffi::c_void,
                        &raw mut kb as *mut uint8_t as *const ::core::ffi::c_void,
                        8 as size_t,
                    );
                    memcpy(
                        &raw mut k.hi as *mut ::core::ffi::c_void,
                        (&raw mut kb as *mut uint8_t)
                            .offset(8 as ::core::ffi::c_int as isize)
                            as *const ::core::ffi::c_void,
                        8 as size_t,
                    );
                    let mut h: uint32_t = wp_whash(k);
                    if bpe_words_probe(wt, wb.wrapping_sub(1 as uint64_t), h, k)
                        .is_null()
                    {
                        let mut id: int64_t = wp_find(
                            t,
                            word,
                            slots.wrapping_sub(1 as uint64_t),
                            wp_hash(
                                *(*v).str_0.offset(i_1 as isize),
                                n_0 as uint64_t,
                                0 as ::core::ffi::c_int,
                            ),
                            *(*v).str_0.offset(i_1 as isize),
                            n_0 as uint64_t,
                            0 as ::core::ffi::c_int,
                        );
                        let mut val: [uint32_t; 4] = [
                            (1 as uint32_t) << TOKS_VAL_COUNT_SHIFT
                                | id as uint32_t & TOKS_ID_MASK as uint32_t,
                            0 as ::core::ffi::c_uint,
                            0 as ::core::ffi::c_uint,
                            0 as ::core::ffi::c_uint,
                        ];
                        bpe_words_put(
                            wt,
                            wb.wrapping_sub(1 as uint64_t),
                            h,
                            k,
                            &raw mut val as *mut uint32_t as *const uint32_t,
                        );
                    }
                }
            }
            i_1 = i_1.wrapping_add(1);
        }
        (*t).wtab = wt;
        (*t).wtab_mask = wb.wrapping_sub(1 as uint64_t);
    }
    return 0 as int64_t;
}
unsafe extern "C" fn wp_trie(
    mut v: *const toks_wp_vocab,
    mut p: *const toks_wp_params,
    mut which: ::core::ffi::c_int,
    mut arr: *mut *mut int32_t,
    mut cap: *mut uint64_t,
    mut da_len: *mut uint64_t,
) -> int64_t {
    let mut kbytes: uint64_t = 0 as uint64_t;
    let mut i: uint32_t = 0 as uint32_t;
    while i < (*v).n {
        kbytes = kbytes.wrapping_add(*(*v).len.offset(i as isize) as uint64_t);
        i = i.wrapping_add(1);
    }
    let mut kb_len: uint64_t = kbytes
        .wrapping_add(64 as uint64_t)
        .wrapping_add(63 as uint64_t) & !(63 as ::core::ffi::c_uint as uint64_t);
    let mut keys_len: uint64_t = ((*v).n as uint64_t)
        .wrapping_add(1 as uint64_t)
        .wrapping_mul(::core::mem::size_of::<ukey>() as uint64_t);
    let mut kb: *mut uint8_t = toks_plat_alloc(kb_len.wrapping_add(keys_len))
        as *mut uint8_t;
    if kb.is_null() {
        return TOKS_E_NOMEM as int64_t;
    }
    let mut keys: *mut ukey = kb.offset(kb_len as isize) as *mut ::core::ffi::c_void
        as *mut ukey;
    let mut nk: uint64_t = 0 as uint64_t;
    let mut off: uint64_t = 0 as uint64_t;
    let mut current_block_12: u64;
    let mut i_0: uint32_t = 0 as uint32_t;
    while i_0 < (*v).n {
        let mut n: uint32_t = *(*v).len.offset(i_0 as isize);
        let mut skip: uint32_t = 0 as uint32_t;
        if which == 1 as ::core::ffi::c_int {
            if n <= (*p).prefix_len
                || memcmp(
                    *(*v).str_0.offset(i_0 as isize) as *const ::core::ffi::c_void,
                    (*p).prefix as *const ::core::ffi::c_void,
                    (*p).prefix_len as size_t,
                ) != 0 as ::core::ffi::c_int
            {
                current_block_12 = 10886091980245723256;
            } else {
                skip = (*p).prefix_len;
                current_block_12 = 1856101646708284338;
            }
        } else {
            current_block_12 = 1856101646708284338;
        }
        match current_block_12 {
            1856101646708284338 => {
                if !(n.wrapping_sub(skip) == 0 as uint32_t) {
                    memcpy(
                        kb.offset(off as isize) as *mut ::core::ffi::c_void,
                        (*(*v).str_0.offset(i_0 as isize)).offset(skip as isize)
                            as *const ::core::ffi::c_void,
                        n.wrapping_sub(skip) as size_t,
                    );
                    (*keys.offset(nk as isize)).off = off;
                    (*keys.offset(nk as isize)).len = n.wrapping_sub(skip);
                    (*keys.offset(nk as isize)).id = i_0;
                    nk = nk.wrapping_add(1);
                    off = off.wrapping_add(n.wrapping_sub(skip) as uint64_t);
                }
            }
            _ => {}
        }
        i_0 = i_0.wrapping_add(1);
    }
    let mut why: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut r: int64_t = if nk != 0 as uint64_t {
        toks_da_build(kb, keys, &raw mut nk, arr, cap, da_len, &raw mut why)
    } else {
        0 as int64_t
    };
    if nk == 0 as uint64_t {
        *arr = ::core::ptr::null_mut::<int32_t>();
        *cap = 0 as uint64_t;
        *da_len = 0 as uint64_t;
    }
    toks_plat_free(kb as *mut ::core::ffi::c_void, kb_len.wrapping_add(keys_len));
    return r;
}
#[no_mangle]
pub unsafe extern "C" fn toks_wp_tries(
    mut t: *mut toks_wp_tables,
    mut ar: *mut toks_arena,
    mut v: *const toks_wp_vocab,
    mut p: *const toks_wp_params,
    mut bytes: *mut uint64_t,
) -> int64_t {
    *bytes = 0 as uint64_t;
    let mut arr: [*mut int32_t; 2] = [
        ::core::ptr::null_mut::<int32_t>(),
        ::core::ptr::null_mut::<int32_t>(),
    ];
    let mut cap: [uint64_t; 2] = [
        0 as ::core::ffi::c_uint as uint64_t,
        0 as ::core::ffi::c_uint as uint64_t,
    ];
    let mut dl: [uint64_t; 2] = [
        0 as ::core::ffi::c_uint as uint64_t,
        0 as ::core::ffi::c_uint as uint64_t,
    ];
    let mut r: int64_t = 0 as int64_t;
    let mut w: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while w < 2 as ::core::ffi::c_int && r == 0 as int64_t {
        r = wp_trie(
            v,
            p,
            w,
            (&raw mut arr as *mut *mut int32_t).offset(w as isize) as *mut *mut int32_t,
            (&raw mut cap as *mut uint64_t).offset(w as isize) as *mut uint64_t,
            (&raw mut dl as *mut uint64_t).offset(w as isize) as *mut uint64_t,
        );
        w += 1;
    }
    let mut w_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while w_0 < 2 as ::core::ffi::c_int {
        *bytes = (*bytes as ::core::ffi::c_ulong)
            .wrapping_add(
                dl[w_0 as usize]
                    .wrapping_add(256 as uint64_t)
                    .wrapping_mul(::core::mem::size_of::<toks_wp_cell>() as uint64_t)
                    .wrapping_add(dl[w_0 as usize].wrapping_mul(4 as uint64_t))
                    .wrapping_add(128 as uint64_t) as ::core::ffi::c_ulong,
            ) as uint64_t as uint64_t;
        w_0 += 1;
    }
    if r == 0 as int64_t && !ar.is_null()
        && dl[0 as ::core::ffi::c_int as usize] != 0 as uint64_t
        && dl[1 as ::core::ffi::c_int as usize] != 0 as uint64_t
    {
        let mut cl: [*mut toks_wp_cell; 2] = [::core::ptr::null_mut::<
            toks_wp_cell,
        >(); 2];
        let mut tm: [*mut int32_t; 2] = [::core::ptr::null_mut::<int32_t>(); 2];
        let mut w_1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while w_1 < 2 as ::core::ffi::c_int {
            cl[w_1 as usize] = toks_tab_ar(
                ar,
                dl[w_1 as usize]
                    .wrapping_add(256 as uint64_t)
                    .wrapping_mul(::core::mem::size_of::<toks_wp_cell>() as uint64_t),
                64 as uint64_t,
                TOKS_X_WP_CELLS,
            ) as *mut toks_wp_cell;
            tm[w_1 as usize] = toks_tab_ar(
                ar,
                dl[w_1 as usize].wrapping_mul(4 as uint64_t),
                64 as uint64_t,
                TOKS_X_WP_TERM,
            ) as *mut int32_t;
            if cl[w_1 as usize].is_null() || tm[w_1 as usize].is_null() {
                r = TOKS_E_NOMEM as int64_t;
                break;
            } else {
                let mut base: *const int32_t = arr[w_1 as usize];
                let mut check: *const int32_t = arr[w_1 as usize]
                    .offset(cap[w_1 as usize] as isize);
                let mut term: *const int32_t = arr[w_1 as usize]
                    .offset((2 as uint64_t).wrapping_mul(cap[w_1 as usize]) as isize);
                let mut x: uint64_t = 0 as uint64_t;
                while x < dl[w_1 as usize].wrapping_add(256 as uint64_t) {
                    let mut in_0: ::core::ffi::c_int = (x < dl[w_1 as usize])
                        as ::core::ffi::c_int;
                    let mut id: int32_t = if in_0 != 0
                        && *term.offset(x as isize) >= 0 as int32_t
                    {
                        *(*v).id.offset(*term.offset(x as isize) as isize) as int32_t
                    } else {
                        -(1 as int32_t)
                    };
                    (*cl[w_1 as usize].offset(x as isize)).base = if in_0 != 0 {
                        *base.offset(x as isize) as uint32_t
                            | (if id >= 0 as int32_t {
                                TOKS_WP_TERM as uint32_t
                            } else {
                                0 as uint32_t
                            })
                    } else {
                        0 as uint32_t
                    };
                    (*cl[w_1 as usize].offset(x as isize)).check = if in_0 != 0 {
                        *check.offset(x as isize)
                    } else {
                        TOKS_DA_FREE as int32_t
                    };
                    if in_0 != 0 {
                        *tm[w_1 as usize].offset(x as isize) = id;
                    }
                    x = x.wrapping_add(1);
                }
                w_1 += 1;
            }
        }
        if r == 0 as int64_t {
            (*t).wcell = cl[0 as ::core::ffi::c_int as usize];
            (*t).ccell = cl[1 as ::core::ffi::c_int as usize];
            (*t).wterm = tm[0 as ::core::ffi::c_int as usize];
            (*t).cterm = tm[1 as ::core::ffi::c_int as usize];
        }
    }
    let mut w_2: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while w_2 < 2 as ::core::ffi::c_int {
        if !arr[w_2 as usize].is_null() {
            toks_plat_free(
                arr[w_2 as usize] as *mut ::core::ffi::c_void,
                cap[w_2 as usize].wrapping_mul(12 as uint64_t),
            );
        }
        w_2 += 1;
    }
    return r;
}
#[no_mangle]
pub unsafe extern "C" fn toks_wp_ctx_build(
    mut cfg: *const toks_config,
    mut par: *mut toks_arena,
    mut mem: *mut *mut uint8_t,
    mut mem_len: *mut uint64_t,
    mut out: *mut *const toks_wp_tables,
    mut err: *mut toks_err,
) -> int64_t {
    let mut ids: *mut uint32_t = toks_ar_alloc(
        par,
        (4 as uint64_t)
            .wrapping_mul((*cfg).n_vocab as uint64_t)
            .wrapping_add(8 as uint64_t),
        8 as uint64_t,
    ) as *mut uint32_t;
    let mut t: *mut toks_wp_tables = toks_ar_alloc(
        par,
        ::core::mem::size_of::<toks_wp_tables>() as uint64_t,
        64 as uint64_t,
    ) as *mut toks_wp_tables;
    if ids.is_null() || t.is_null() {
        return toks_fail(
            err,
            TOKS_E_NOMEM as int64_t,
            b"wordpiece vocab ids\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    let mut i: uint32_t = 0 as uint32_t;
    while i < (*cfg).n_vocab {
        *ids.offset(i as isize) = i;
        i = i.wrapping_add(1);
    }
    let mut v: toks_wp_vocab = toks_wp_vocab {
        str_0: (*cfg).vocab,
        len: (*cfg).vocab_len,
        id: ids,
        n: (*cfg).n_vocab,
    };
    let mut p: toks_wp_params = toks_wp_params {
        flags: (*cfg).wp_flags,
        max_chars: (*cfg).wp_max_chars,
        unk: (*cfg).wp_unk,
        unk_len: (*cfg).wp_unk_len,
        prefix: (*cfg).wp_prefix,
        prefix_len: (*cfg).wp_prefix_len,
    };
    let mut tb: uint64_t = 0 as uint64_t;
    let mut tr: int64_t = toks_wp_tries(
        ::core::ptr::null_mut::<toks_wp_tables>(),
        ::core::ptr::null_mut::<toks_arena>(),
        &raw mut v,
        &raw mut p,
        &raw mut tb,
    );
    if tr != 0 as int64_t {
        tb = 0 as uint64_t;
    }
    let mut nb: uint64_t = toks_wp_tables_bytes(&raw mut v)
        .wrapping_add(tb)
        .wrapping_add(::core::mem::size_of::<toks_wp_tables>() as uint64_t)
        .wrapping_add(128 as uint64_t);
    let mut m: *mut uint8_t = toks_plat_arena(nb);
    if m.is_null() {
        return toks_fail(
            err,
            TOKS_E_NOMEM as int64_t,
            b"wordpiece tables\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    *mem = m;
    *mem_len = nb;
    let mut ar: toks_arena = toks_arena {
        base: m,
        len: nb,
        pos: 0 as uint64_t,
    };
    let mut dst: *mut toks_wp_tables = toks_tab_ar(
        &raw mut ar,
        ::core::mem::size_of::<toks_wp_tables>() as uint64_t,
        64 as uint64_t,
        TOKS_X_WP,
    ) as *mut toks_wp_tables;
    if dst.is_null() {
        return toks_fail(
            err,
            TOKS_E_NOMEM as int64_t,
            b"wordpiece tables\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    let mut r: int64_t = toks_wp_build(t, &raw mut ar, &raw mut v, &raw mut p, err);
    if r != 0 as int64_t {
        return r;
    }
    if tb != 0 as uint64_t
        && toks_wp_tries(t, &raw mut ar, &raw mut v, &raw mut p, &raw mut tb)
            != 0 as int64_t
    {
        (*t).ccell = ::core::ptr::null::<toks_wp_cell>();
        (*t).wcell = (*t).ccell;
        (*t).cterm = ::core::ptr::null::<int32_t>();
        (*t).wterm = (*t).cterm;
    }
    memcpy(
        dst as *mut ::core::ffi::c_void,
        t as *const ::core::ffi::c_void,
        ::core::mem::size_of::<toks_wp_tables>() as size_t,
    );
    toks_tab_seal(m as *mut ::core::ffi::c_void, nb);
    *out = dst;
    return 0 as int64_t;
}
