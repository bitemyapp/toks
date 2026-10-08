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
    fn toks_plat_arena_free(p: *mut uint8_t, n: uint64_t);
    fn toks_is_regex_ws(cp: uint32_t) -> ::core::ffi::c_int;
    static TOKS_GC_STAGE1: [uint16_t; 4352];
    static TOKS_GC_STAGE2: [uint8_t; 0];
    fn toks_gc_step(s: *mut toks_gc_state, cls: uint8_t) -> ::core::ffi::c_int;
    fn toks_pc_bytes(
        blob: *const uint8_t,
        len: uint64_t,
        why: *mut *const ::core::ffi::c_char,
    ) -> uint64_t;
    fn toks_pc_build(
        pc: *mut toks_pc,
        blob: *const uint8_t,
        len: uint64_t,
        ar: *mut toks_arena,
        why: *mut *const ::core::ffi::c_char,
    ) -> int64_t;
    fn toks_pc_multi(pc: *const toks_pc, key: *const uint8_t, n: uint32_t) -> uint32_t;
    static toks_pow10: [::core::ffi::c_double; 309];
}
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
pub type size_t = usize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct toks_uni {
    pub cfg: toks_uni_cfg,
    pub pc: toks_pc,
    pub cell: *const toks_uni_cell,
    pub score: *const ::core::ffi::c_double,
    pub term: *const int32_t,
    pub da_len: uint32_t,
    pub n_vocab: uint32_t,
    pub unk_score: ::core::ffi::c_double,
    pub words: *const uint8_t,
    pub words_mask: uint64_t,
    pub simple: [uint8_t; 128],
    pub aent: [uint32_t; 128],
    pub acls: [uint8_t; 128],
    pub fast: uint8_t,
    pub rsv2: [uint8_t; 7],
    pub unk_id: uint32_t,
    pub max_piece: uint32_t,
    pub byte_id: [int32_t; 256],
    pub byte_fallback: uint8_t,
    pub remap: uint8_t,
    pub rsv: [uint8_t; 2],
    pub work_x: uint32_t,
    pub pre_x: uint32_t,
    pub n_dropped: uint32_t,
    pub n_dec: uint32_t,
    pub dec: [toks_spm_op; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct toks_spm_op {
    pub kind: uint32_t,
    pub start: uint32_t,
    pub stop: uint32_t,
    pub scheme: uint32_t,
    pub a: toks_spm_str,
    pub b: toks_spm_str,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct toks_spm_str {
    pub b: [uint8_t; 16],
    pub n: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct toks_uni_cell {
    pub base: uint32_t,
    pub check: int32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct toks_pc {
    pub stage1: *mut uint16_t,
    pub stage2: *mut uint32_t,
    pub n_blocks: uint32_t,
    pub mk_key: *mut uint64_t,
    pub mk_val: *mut uint32_t,
    pub mk_mask: uint32_t,
    pub pool: *mut uint8_t,
    pub pool_len: uint32_t,
    pub max_expand: uint32_t,
    pub n_keys: uint32_t,
    pub n_live_single: uint32_t,
    pub n_live_multi: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct toks_uni_cfg {
    pub strip_left: uint8_t,
    pub strip_right: uint8_t,
    pub has_charsmap: uint8_t,
    pub collapse: uint8_t,
    pub meta_prefix: uint8_t,
    pub meta_replace: uint8_t,
    pub ws_split: uint8_t,
    pub metaspace: uint8_t,
    pub meta_prepend: uint8_t,
    pub meta_split: uint8_t,
    pub dec: uint8_t,
    pub dec_prepend: uint8_t,
    pub rep_n: uint8_t,
    pub rep_pl: [uint8_t; 4],
    pub rep_cl: [uint8_t; 4],
    pub rep_p: [[uint8_t; 4]; 4],
    pub rep_c: [[uint8_t; 4]; 4],
    pub form: uint8_t,
    pub rsv: [uint8_t; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct toks_ext {
    pub pad: uint32_t,
    pub align: uint32_t,
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
pub struct toks_emit {
    pub out: *mut uint32_t,
    pub cap: uint64_t,
    pub n: uint64_t,
    pub lim: uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct toks_dsink {
    pub out: *mut uint8_t,
    pub cap: uint64_t,
    pub n: uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct toks_uni_src {
    pub n: uint32_t,
    pub piece: *const *const uint8_t,
    pub piece_len: *const uint32_t,
    pub score_txt: *const *const uint8_t,
    pub score_len: *const uint32_t,
    pub unk_id: int64_t,
    pub byte_fallback: uint32_t,
    pub charsmap: *const uint8_t,
    pub charsmap_len: uint64_t,
    pub cfg: toks_uni_cfg,
}
pub type C2RustUnnamed = ::core::ffi::c_uint;
pub const TOKS_SPM_D_METASPACE: C2RustUnnamed = 5;
pub const TOKS_SPM_D_STRIP: C2RustUnnamed = 4;
pub const TOKS_SPM_D_FUSE: C2RustUnnamed = 3;
pub const TOKS_SPM_D_BYTE_FALLBACK: C2RustUnnamed = 2;
pub const TOKS_SPM_D_REPLACE: C2RustUnnamed = 1;
pub type C2RustUnnamed_0 = ::core::ffi::c_uint;
pub const TOKS_SPM_PS_NEVER: C2RustUnnamed_0 = 2;
pub const TOKS_SPM_PS_FIRST: C2RustUnnamed_0 = 1;
pub const TOKS_SPM_PS_ALWAYS: C2RustUnnamed_0 = 0;
pub type C2RustUnnamed_1 = ::core::ffi::c_uint;
pub const TOKS_GC_ZWJ: C2RustUnnamed_1 = 15;
pub const TOKS_GC_V: C2RustUnnamed_1 = 14;
pub const TOKS_GC_T: C2RustUnnamed_1 = 13;
pub const TOKS_GC_SPACINGMARK: C2RustUnnamed_1 = 12;
pub const TOKS_GC_RI: C2RustUnnamed_1 = 11;
pub const TOKS_GC_PREPEND: C2RustUnnamed_1 = 10;
pub const TOKS_GC_LVT: C2RustUnnamed_1 = 9;
pub const TOKS_GC_LV: C2RustUnnamed_1 = 8;
pub const TOKS_GC_LF: C2RustUnnamed_1 = 7;
pub const TOKS_GC_L: C2RustUnnamed_1 = 6;
pub const TOKS_GC_INCB_CONSONANT: C2RustUnnamed_1 = 5;
pub const TOKS_GC_EXTPICT: C2RustUnnamed_1 = 4;
pub const TOKS_GC_EXTEND: C2RustUnnamed_1 = 3;
pub const TOKS_GC_CONTROL: C2RustUnnamed_1 = 2;
pub const TOKS_GC_CR: C2RustUnnamed_1 = 1;
pub const TOKS_GC_ANY: C2RustUnnamed_1 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct toks_gc_state {
    pub prev: uint8_t,
    pub ri_odd: uint8_t,
    pub ep_run: uint8_t,
    pub ep_zwj: uint8_t,
    pub incb_cons: uint8_t,
    pub incb_link: uint8_t,
    pub rsv: [uint8_t; 2],
}
pub type C2RustUnnamed_2 = ::core::ffi::c_uint;
pub const TOKS_UNI_DEC_BFRF: C2RustUnnamed_2 = 3;
pub const TOKS_UNI_DEC_RBF: C2RustUnnamed_2 = 2;
pub const TOKS_UNI_DEC_META: C2RustUnnamed_2 = 1;
pub const TOKS_UNI_DEC_NONE: C2RustUnnamed_2 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ukey {
    pub off: uint64_t,
    pub len: uint32_t,
    pub id: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uni_cand {
    pub score: ::core::ffi::c_double,
    pub key: uint32_t,
    pub rsv: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bpe_key {
    pub lo: uint64_t,
    pub hi: uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct qent {
    pub slot: uint32_t,
    pub lo: uint32_t,
    pub hi: uint32_t,
    pub depth: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct seg {
    pub u: *const toks_uni,
    pub text: *const uint8_t,
    pub tlen: uint64_t,
    pub c: *mut toks_uni_call,
    pub mbuf: *mut uint8_t,
    pub mcap: uint64_t,
    pub delta: *mut uint8_t,
    pub npos: uint64_t,
    pub p_nend: uint64_t,
    pub seen: ::core::ffi::c_int,
    pub prev_space: ::core::ffi::c_int,
    pub in_word: ::core::ffi::c_int,
    pub split_start: ::core::ffi::c_int,
    pub started: ::core::ffi::c_int,
    pub p_open: ::core::ffi::c_int,
    pub p_virt: ::core::ffi::c_int,
    pub p_mat: ::core::ffi::c_int,
    pub p_s: uint64_t,
    pub p_e: uint64_t,
    pub p_ml: uint64_t,
    pub err: ::core::ffi::c_int,
    pub sink: *mut toks_dsink,
    pub qn: uint32_t,
    pub q: [C2RustUnnamed_3; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub b: *const uint8_t,
    pub l: uint64_t,
    pub virt: ::core::ffi::c_int,
    pub h: uint32_t,
    pub k: bpe_key,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct toks_uni_call {
    pub e: toks_emit,
    pub nbase: uint64_t,
    pub pieces: ::core::ffi::c_int,
    pub rsv: ::core::ffi::c_int,
    pub work: *mut uint8_t,
    pub work_bytes: uint64_t,
    pub cache: *mut uint8_t,
    pub cache_mask: uint64_t,
    pub tw: uint64_t,
}
pub const UINT64_MAX: ::core::ffi::c_ulong = 18446744073709551615
    as ::core::ffi::c_ulong;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
pub const TOKS_E_FORMAT: ::core::ffi::c_int = -(2 as ::core::ffi::c_int);
pub const TOKS_E_UNSUPPORTED: ::core::ffi::c_int = -(3 as ::core::ffi::c_int);
pub const TOKS_E_SCRATCH: ::core::ffi::c_int = -(6 as ::core::ffi::c_int);
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
pub const TOKS_X_UNI: toks_ext = toks_ext {
    pad: 0 as uint32_t,
    align: 8 as uint32_t,
};
pub const TOKS_X_UNI_CELLS: toks_ext = toks_ext {
    pad: 0 as uint32_t,
    align: 4 as uint32_t,
};
pub const TOKS_X_UNI_SCORE: toks_ext = toks_ext {
    pad: 0 as uint32_t,
    align: 8 as uint32_t,
};
pub const TOKS_X_UNI_TERM: toks_ext = toks_ext {
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
unsafe extern "C" fn toks_tab(
    mut block: *mut uint8_t,
    mut o: uint64_t,
    mut n: uint64_t,
    mut x: toks_ext,
) -> *mut ::core::ffi::c_void {
    return block.offset(o as isize) as *mut ::core::ffi::c_void;
}
#[inline]
unsafe extern "C" fn toks_tab_seal(mut b: *mut ::core::ffi::c_void, mut n: uint64_t) {}
#[inline]
unsafe extern "C" fn toks_tab_unmapped(
    mut b: *const ::core::ffi::c_void,
    mut n: uint64_t,
) {}
#[inline]
unsafe extern "C" fn toks_tab_free(mut b: *mut uint8_t, mut n: uint64_t) {
    toks_tab_unmapped(b as *const ::core::ffi::c_void, n);
    toks_plat_arena_free(b, n);
}
#[inline]
unsafe extern "C" fn toks_utf8_len(
    mut p: *const uint8_t,
    mut avail: uint64_t,
) -> uint32_t {
    let mut b: uint8_t = *p.offset(0 as ::core::ffi::c_int as isize);
    if (b as ::core::ffi::c_uint) < 0x80 as ::core::ffi::c_uint {
        return 1 as uint32_t;
    }
    if (b as ::core::ffi::c_uint) < 0xc2 as ::core::ffi::c_uint {
        return 0 as uint32_t;
    }
    if (b as ::core::ffi::c_uint) < 0xe0 as ::core::ffi::c_uint {
        if avail < 2 as uint64_t
            || *p.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint
                & 0xc0 as ::core::ffi::c_uint != 0x80 as ::core::ffi::c_uint
        {
            return 0 as uint32_t;
        }
        return 2 as uint32_t;
    }
    if (b as ::core::ffi::c_uint) < 0xf0 as ::core::ffi::c_uint {
        if avail < 3 as uint64_t {
            return 0 as uint32_t;
        }
        if b as ::core::ffi::c_uint == 0xe0 as ::core::ffi::c_uint
            && (*p.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                < 0xa0 as ::core::ffi::c_uint
        {
            return 0 as uint32_t;
        }
        if b as ::core::ffi::c_uint == 0xed as ::core::ffi::c_uint
            && *p.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint
                >= 0xa0 as ::core::ffi::c_uint
        {
            return 0 as uint32_t;
        }
        if *p.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint
            & 0xc0 as ::core::ffi::c_uint != 0x80 as ::core::ffi::c_uint
            || *p.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint
                & 0xc0 as ::core::ffi::c_uint != 0x80 as ::core::ffi::c_uint
        {
            return 0 as uint32_t;
        }
        return 3 as uint32_t;
    }
    if b as ::core::ffi::c_uint == 0xf0 as ::core::ffi::c_uint {
        if avail < 4 as uint64_t
            || (*p.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                < 0x90 as ::core::ffi::c_uint
            || *p.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint
                & 0xc0 as ::core::ffi::c_uint != 0x80 as ::core::ffi::c_uint
        {
            return 0 as uint32_t;
        }
    } else if b as ::core::ffi::c_uint <= 0xf3 as ::core::ffi::c_uint {
        if avail < 4 as uint64_t
            || *p.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint
                & 0xc0 as ::core::ffi::c_uint != 0x80 as ::core::ffi::c_uint
        {
            return 0 as uint32_t;
        }
    } else if b as ::core::ffi::c_uint == 0xf4 as ::core::ffi::c_uint {
        if avail < 4 as uint64_t
            || *p.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint
                > 0x8f as ::core::ffi::c_uint
            || *p.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint
                & 0xc0 as ::core::ffi::c_uint != 0x80 as ::core::ffi::c_uint
        {
            return 0 as uint32_t;
        }
    } else {
        return 0 as uint32_t
    }
    if *p.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint
        & 0xc0 as ::core::ffi::c_uint != 0x80 as ::core::ffi::c_uint
        || *p.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint
            & 0xc0 as ::core::ffi::c_uint != 0x80 as ::core::ffi::c_uint
    {
        return 0 as uint32_t;
    }
    return 4 as uint32_t;
}
#[inline]
unsafe extern "C" fn toks_put(mut e: *mut toks_emit, mut v: uint32_t) {
    if (*e).n >= (*e).lim {
        return;
    }
    if (*e).n < (*e).cap {
        toks_st32((*e).out.offset((*e).n as isize) as *mut ::core::ffi::c_void, v);
    }
    (*e).n = (*e).n.wrapping_add(1);
}
#[inline]
unsafe extern "C" fn toks_dput(
    mut d: *mut toks_dsink,
    mut p: *const ::core::ffi::c_void,
    mut k: uint64_t,
) {
    if (*d).n < (*d).cap {
        memcpy(
            (*d).out.offset((*d).n as isize) as *mut ::core::ffi::c_void,
            p,
            (if (*d).cap.wrapping_sub((*d).n) < k {
                (*d).cap.wrapping_sub((*d).n)
            } else {
                k
            }) as size_t,
        );
    }
    (*d).n = (*d).n.wrapping_add(k);
}
pub const TOKS_NFC_X: ::core::ffi::c_uint = 3 as ::core::ffi::c_uint;
pub const TOKS_NFKC_X: ::core::ffi::c_uint = 11 as ::core::ffi::c_uint;
#[inline]
unsafe extern "C" fn toks_utf8_put(mut o: *mut uint8_t, mut cp: uint32_t) -> uint32_t {
    if cp < 0x80 as uint32_t {
        *o.offset(0 as ::core::ffi::c_int as isize) = cp as uint8_t;
        return 1 as uint32_t;
    }
    if cp < 0x800 as uint32_t {
        *o.offset(0 as ::core::ffi::c_int as isize) = (0xc0 as uint32_t
            | cp >> 6 as ::core::ffi::c_int) as uint8_t;
        *o.offset(1 as ::core::ffi::c_int as isize) = (0x80 as uint32_t
            | cp & 0x3f as uint32_t) as uint8_t;
        return 2 as uint32_t;
    }
    if cp < 0x10000 as uint32_t {
        *o.offset(0 as ::core::ffi::c_int as isize) = (0xe0 as uint32_t
            | cp >> 12 as ::core::ffi::c_int) as uint8_t;
        *o.offset(1 as ::core::ffi::c_int as isize) = (0x80 as uint32_t
            | cp >> 6 as ::core::ffi::c_int & 0x3f as uint32_t) as uint8_t;
        *o.offset(2 as ::core::ffi::c_int as isize) = (0x80 as uint32_t
            | cp & 0x3f as uint32_t) as uint8_t;
        return 3 as uint32_t;
    }
    *o.offset(0 as ::core::ffi::c_int as isize) = (0xf0 as uint32_t
        | cp >> 18 as ::core::ffi::c_int) as uint8_t;
    *o.offset(1 as ::core::ffi::c_int as isize) = (0x80 as uint32_t
        | cp >> 12 as ::core::ffi::c_int & 0x3f as uint32_t) as uint8_t;
    *o.offset(2 as ::core::ffi::c_int as isize) = (0x80 as uint32_t
        | cp >> 6 as ::core::ffi::c_int & 0x3f as uint32_t) as uint8_t;
    *o.offset(3 as ::core::ffi::c_int as isize) = (0x80 as uint32_t
        | cp & 0x3f as uint32_t) as uint8_t;
    return 4 as uint32_t;
}
#[inline]
unsafe extern "C" fn toks_cp_decode(mut p: *const uint8_t, mut k: uint32_t) -> uint32_t {
    if k == 2 as uint32_t {
        return (*p.offset(0 as ::core::ffi::c_int as isize) as uint32_t
            & 0x1f as uint32_t) << 6 as ::core::ffi::c_int
            | *p.offset(1 as ::core::ffi::c_int as isize) as uint32_t & 0x3f as uint32_t;
    }
    if k == 3 as uint32_t {
        return (*p.offset(0 as ::core::ffi::c_int as isize) as uint32_t
            & 0xf as uint32_t) << 12 as ::core::ffi::c_int
            | (*p.offset(1 as ::core::ffi::c_int as isize) as uint32_t
                & 0x3f as uint32_t) << 6 as ::core::ffi::c_int
            | *p.offset(2 as ::core::ffi::c_int as isize) as uint32_t & 0x3f as uint32_t;
    }
    return (*p.offset(0 as ::core::ffi::c_int as isize) as uint32_t & 0x7 as uint32_t)
        << 18 as ::core::ffi::c_int
        | (*p.offset(1 as ::core::ffi::c_int as isize) as uint32_t & 0x3f as uint32_t)
            << 12 as ::core::ffi::c_int
        | (*p.offset(2 as ::core::ffi::c_int as isize) as uint32_t & 0x3f as uint32_t)
            << 6 as ::core::ffi::c_int
        | *p.offset(3 as ::core::ffi::c_int as isize) as uint32_t & 0x3f as uint32_t;
}
pub const TOKS_NS_COMPAT: ::core::ffi::c_uint = 0x40 as ::core::ffi::c_uint;
#[inline]
unsafe extern "C" fn toks_spm_whash(mut lo: uint64_t, mut hi: uint64_t) -> uint32_t {
    return ((lo as ::core::ffi::c_ulonglong
        ^ (hi as ::core::ffi::c_ulonglong).wrapping_mul(TOKS_FIB64))
        .wrapping_mul(0xd6e8feb86659fd93 as ::core::ffi::c_ulonglong)
        >> 32 as ::core::ffi::c_int) as uint32_t;
}
#[inline]
unsafe extern "C" fn toks_gc_class(mut cp: uint32_t) -> uint8_t {
    return *(&raw const TOKS_GC_STAGE2 as *const uint8_t)
        .offset(
            (TOKS_GC_STAGE1[(cp >> 8 as ::core::ffi::c_int) as usize] as uint32_t)
                .wrapping_mul(256 as uint32_t)
                .wrapping_add(cp & 0xff as uint32_t) as isize,
        );
}
#[inline]
unsafe extern "C" fn toks_gc_init(mut s: *mut toks_gc_state) {
    memset(
        s as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<toks_gc_state>() as size_t,
    );
    (*s).prev = 0xff as uint8_t;
}
#[inline]
unsafe extern "C" fn toks_pc_char(mut pc: *const toks_pc, mut cp: uint32_t) -> uint32_t {
    if cp > 0x10ffff as uint32_t {
        return 0 as uint32_t;
    }
    return *(*pc)
        .stage2
        .offset(
            (*(*pc).stage1.offset((cp >> 8 as ::core::ffi::c_int) as isize) as uint32_t)
                .wrapping_mul(256 as uint32_t)
                .wrapping_add(cp & 0xff as uint32_t) as isize,
        );
}
pub const TOKS_UNI_TERM: ::core::ffi::c_uint = 0x80000000 as ::core::ffi::c_uint;
pub const TOKS_UNI_BASE: ::core::ffi::c_uint = 0x7fffffff as ::core::ffi::c_uint;
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
unsafe extern "C" fn bpe_val_put(
    mut v: *const uint8_t,
    mut out: *mut ::core::ffi::c_void,
) -> uint64_t {
    let mut w: [uint32_t; 4] = [0; 4];
    memcpy(
        &raw mut w as *mut uint32_t as *mut ::core::ffi::c_void,
        v as *const ::core::ffi::c_void,
        16 as size_t,
    );
    let mut count: uint64_t = (w[0 as ::core::ffi::c_int as usize]
        >> TOKS_VAL_COUNT_SHIFT) as uint64_t;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < 4 as ::core::ffi::c_int {
        w[i as usize] = (w[i as usize] as ::core::ffi::c_uint & TOKS_ID_MASK)
            as uint32_t;
        i += 1;
    }
    memcpy(out, &raw mut w as *mut uint32_t as *const ::core::ffi::c_void, 16 as size_t);
    return count;
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
            let fresh0 = w;
            w = w.wrapping_add(1);
            *keys.offset(fresh0 as isize) = *keys.offset(i as isize);
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
unsafe extern "C" fn is_digit(mut c: uint8_t) -> ::core::ffi::c_int {
    return (c as ::core::ffi::c_int >= '0' as i32
        && c as ::core::ffi::c_int <= '9' as i32) as ::core::ffi::c_int;
}
unsafe extern "C" fn parse_score(
    mut t: *const uint8_t,
    mut n: uint32_t,
    mut out: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    let mut i: uint32_t = 0 as uint32_t;
    let mut neg: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if i < n && *t.offset(i as isize) as ::core::ffi::c_int == '-' as i32 {
        neg = 1 as ::core::ffi::c_int;
        i = i.wrapping_add(1);
    }
    let mut sig: uint64_t = 0 as uint64_t;
    let mut exp: int64_t = 0 as int64_t;
    let mut ovf: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut is_int: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    while i < n && is_digit(*t.offset(i as isize)) != 0 {
        let mut d: uint64_t = (*t.offset(i as isize) as ::core::ffi::c_int - '0' as i32)
            as uint64_t;
        if ovf == 0
            && sig
                > (UINT64_MAX as uint64_t).wrapping_sub(d).wrapping_div(10 as uint64_t)
        {
            ovf = 1 as ::core::ffi::c_int;
        }
        if ovf != 0 {
            exp += 1;
        } else {
            sig = sig.wrapping_mul(10 as uint64_t).wrapping_add(d);
        }
        i = i.wrapping_add(1);
    }
    if i < n && *t.offset(i as isize) as ::core::ffi::c_int == '.' as i32 {
        is_int = 0 as ::core::ffi::c_int;
        i = i.wrapping_add(1);
        while i < n && is_digit(*t.offset(i as isize)) != 0 {
            let mut d_0: uint64_t = (*t.offset(i as isize) as ::core::ffi::c_int
                - '0' as i32) as uint64_t;
            if ovf == 0
                && sig
                    > (UINT64_MAX as uint64_t)
                        .wrapping_sub(d_0)
                        .wrapping_div(10 as uint64_t)
            {
                ovf = 1 as ::core::ffi::c_int;
            }
            if ovf == 0 {
                sig = sig.wrapping_mul(10 as uint64_t).wrapping_add(d_0);
                exp -= 1;
            }
            i = i.wrapping_add(1);
        }
    }
    if i < n
        && (*t.offset(i as isize) as ::core::ffi::c_int == 'e' as i32
            || *t.offset(i as isize) as ::core::ffi::c_int == 'E' as i32)
    {
        is_int = 0 as ::core::ffi::c_int;
        i = i.wrapping_add(1);
        let mut eneg: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        if i < n
            && (*t.offset(i as isize) as ::core::ffi::c_int == '+' as i32
                || *t.offset(i as isize) as ::core::ffi::c_int == '-' as i32)
        {
            eneg = (*t.offset(i as isize) as ::core::ffi::c_int == '-' as i32)
                as ::core::ffi::c_int;
            i = i.wrapping_add(1);
        }
        let mut e: int64_t = 0 as int64_t;
        while i < n && is_digit(*t.offset(i as isize)) != 0 {
            e = e * 10 as int64_t
                + (*t.offset(i as isize) as ::core::ffi::c_int - '0' as i32) as int64_t;
            if e > 2147483647 as int64_t {
                if sig != 0 as uint64_t && eneg == 0 {
                    return -(1 as ::core::ffi::c_int);
                }
                *out = if neg != 0 { -0.0f64 } else { 0.0f64 };
                return 0 as ::core::ffi::c_int;
            }
            i = i.wrapping_add(1);
        }
        exp = if eneg != 0 { exp - e } else { exp + e };
    }
    if is_int != 0 && ovf == 0 {
        let mut f: ::core::ffi::c_double = sig as ::core::ffi::c_double;
        *out = if neg != 0 { -f } else { f };
        return 0 as ::core::ffi::c_int;
    }
    let mut f_0: ::core::ffi::c_double = sig as ::core::ffi::c_double;
    let mut k: uint32_t = 0 as uint32_t;
    while k < 16 as uint32_t {
        let mut ae: int64_t = if exp < 0 as int64_t { -exp } else { exp };
        if ae <= 308 as int64_t {
            if exp >= 0 as int64_t {
                f_0 = f_0 * toks_pow10[ae as usize];
                if f_0 > 1.7976931348623157e308f64 {
                    return -(1 as ::core::ffi::c_int);
                }
            } else {
                f_0 = f_0 / toks_pow10[ae as usize];
            }
            break;
        } else {
            if f_0 == 0.0f64 {
                break;
            }
            if exp >= 0 as int64_t {
                return -(1 as ::core::ffi::c_int);
            }
            f_0 = f_0 / 1e308f64;
            exp += 308 as int64_t;
            k = k.wrapping_add(1);
        }
    }
    *out = if neg != 0 { -f_0 } else { f_0 };
    return 0 as ::core::ffi::c_int;
}
pub const UNI_MAX_PIECE: ::core::ffi::c_uint = 127 as ::core::ffi::c_uint;
pub const UNI_FREE: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
unsafe extern "C" fn remap_piece(
    mut s: *const uint8_t,
    mut n: uint32_t,
    mut remap: ::core::ffi::c_int,
    mut dst: *mut uint8_t,
) -> int64_t {
    let mut i: uint32_t = 0 as uint32_t;
    let mut m: int64_t = 0 as int64_t;
    while i < n {
        if remap != 0
            && *s.offset(i as isize) as ::core::ffi::c_uint
                == 0x20 as ::core::ffi::c_uint
        {
            return -(1 as ::core::ffi::c_int) as int64_t;
        }
        if remap != 0 && i.wrapping_add(2 as uint32_t) < n
            && *s.offset(i as isize) as ::core::ffi::c_uint
                == 0xe2 as ::core::ffi::c_uint
            && *s.offset(i.wrapping_add(1 as uint32_t) as isize) as ::core::ffi::c_uint
                == 0x96 as ::core::ffi::c_uint
            && *s.offset(i.wrapping_add(2 as uint32_t) as isize) as ::core::ffi::c_uint
                == 0x81 as ::core::ffi::c_uint
        {
            if !dst.is_null() {
                *dst.offset(m as isize) = 0x20 as uint8_t;
            }
            m += 1;
            i = (i as ::core::ffi::c_uint).wrapping_add(3 as ::core::ffi::c_uint)
                as uint32_t as uint32_t;
        } else {
            if !dst.is_null() {
                *dst.offset(m as isize) = *s.offset(i as isize);
            }
            m += 1;
            i = i.wrapping_add(1);
        }
    }
    return m;
}
unsafe extern "C" fn fail_why(
    mut why: *mut *const ::core::ffi::c_char,
    mut w: *const ::core::ffi::c_char,
    mut code: int64_t,
) -> int64_t {
    *why = w;
    return code;
}
unsafe extern "C" fn dec_ops(
    mut c: *const toks_uni_cfg,
    mut op: *mut toks_spm_op,
) -> uint32_t {
    memset(
        op as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        (4 as size_t).wrapping_mul(::core::mem::size_of::<toks_spm_op>() as size_t),
    );
    memcpy(
        &raw mut (*op.offset(0 as ::core::ffi::c_int as isize)).a.b as *mut uint8_t
            as *mut ::core::ffi::c_void,
        b"\xE2\x96\x81\0" as *const u8 as *const ::core::ffi::c_char
            as *const ::core::ffi::c_void,
        3 as size_t,
    );
    (*op.offset(0 as ::core::ffi::c_int as isize)).a.n = 3 as ::core::ffi::c_uint
        as uint32_t;
    (*op.offset(0 as ::core::ffi::c_int as isize)).kind = TOKS_SPM_D_METASPACE
        as ::core::ffi::c_int as uint32_t;
    (*op.offset(0 as ::core::ffi::c_int as isize)).scheme = (if (*c).dec_prepend
        as ::core::ffi::c_int != 0
    {
        TOKS_SPM_PS_ALWAYS as ::core::ffi::c_int
    } else {
        TOKS_SPM_PS_NEVER as ::core::ffi::c_int
    }) as uint32_t;
    if (*c).dec as ::core::ffi::c_int != TOKS_UNI_DEC_RBF as ::core::ffi::c_int
        && (*c).dec as ::core::ffi::c_int != TOKS_UNI_DEC_BFRF as ::core::ffi::c_int
    {
        return if (*c).dec as ::core::ffi::c_int
            == TOKS_UNI_DEC_META as ::core::ffi::c_int
        {
            1 as uint32_t
        } else {
            0 as uint32_t
        };
    }
    let mut r: uint32_t = if (*c).dec as ::core::ffi::c_int
        == TOKS_UNI_DEC_RBF as ::core::ffi::c_int
    {
        0 as uint32_t
    } else {
        1 as uint32_t
    };
    *op.offset(r as isize) = *op.offset(0 as ::core::ffi::c_int as isize);
    (*op.offset(r as isize)).kind = TOKS_SPM_D_REPLACE as ::core::ffi::c_int as uint32_t;
    (*op.offset(r as isize)).b.b[0 as ::core::ffi::c_int as usize] = ' ' as i32
        as uint8_t;
    (*op.offset(r as isize)).b.n = 1 as ::core::ffi::c_uint as uint32_t;
    (*op.offset((1 as uint32_t).wrapping_sub(r) as isize)).kind = TOKS_SPM_D_BYTE_FALLBACK
        as ::core::ffi::c_int as uint32_t;
    (*op.offset(2 as ::core::ffi::c_int as isize)).kind = TOKS_SPM_D_FUSE
        as ::core::ffi::c_int as uint32_t;
    (*op.offset(3 as ::core::ffi::c_int as isize)).kind = TOKS_SPM_D_STRIP
        as ::core::ffi::c_int as uint32_t;
    (*op.offset(3 as ::core::ffi::c_int as isize))
        .a
        .b[0 as ::core::ffi::c_int as usize] = ' ' as i32 as uint8_t;
    (*op.offset(3 as ::core::ffi::c_int as isize)).a.n = 1 as ::core::ffi::c_uint
        as uint32_t;
    (*op.offset(3 as ::core::ffi::c_int as isize)).start = 1 as ::core::ffi::c_uint
        as uint32_t;
    return (3 as uint32_t).wrapping_add(r);
}
#[inline]
unsafe extern "C" fn pbyte(
    mut virt: ::core::ffi::c_int,
    mut p: *const uint8_t,
    mut i: uint64_t,
) -> uint8_t {
    return (if virt != 0 && i == 0 as uint64_t {
        0x20 as ::core::ffi::c_uint as uint8_t as ::core::ffi::c_int
    } else {
        *p
            .offset(
                i
                    .wrapping_sub(
                        (virt != 0 as ::core::ffi::c_int) as ::core::ffi::c_int
                            as uint64_t,
                    ) as isize,
            ) as ::core::ffi::c_int
    }) as uint8_t;
}
unsafe extern "C" fn exact_id(
    mut u: *const toks_uni,
    mut virt: ::core::ffi::c_int,
    mut p: *const uint8_t,
    mut s: uint64_t,
    mut e: uint64_t,
) -> int32_t {
    let mut node: uint32_t = 0 as uint32_t;
    let mut j: uint64_t = s;
    while j < e {
        let mut t: uint32_t = ((*(*u).cell.offset(node as isize)).base
            & TOKS_UNI_BASE as uint32_t)
            .wrapping_add(pbyte(virt, p, j) as uint32_t);
        if (*(*u).cell.offset(t as isize)).check != node as int32_t {
            return -(1 as int32_t);
        }
        node = t;
        j = j.wrapping_add(1);
    }
    return *(*u).term.offset(node as isize);
}
unsafe extern "C" fn emit_unknown(
    mut u: *const toks_uni,
    mut virt: ::core::ffi::c_int,
    mut p: *const uint8_t,
    mut s: uint64_t,
    mut end: uint64_t,
    mut e: *mut toks_emit,
) {
    let mut id: int32_t = exact_id(u, virt, p, s, end);
    if id >= 0 as int32_t {
        toks_put(e, id as uint32_t);
        return;
    }
    if (*u).byte_fallback != 0 {
        let mut all: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
        let mut j: uint64_t = s;
        while j < end && all != 0 {
            let mut c: uint8_t = pbyte(virt, p, j);
            if (*u).remap as ::core::ffi::c_int != 0
                && c as ::core::ffi::c_uint == 0x20 as ::core::ffi::c_uint
            {
                all = ((*u).byte_id[0xe2 as ::core::ffi::c_int as usize] >= 0 as int32_t
                    && (*u).byte_id[0x96 as ::core::ffi::c_int as usize] >= 0 as int32_t
                    && (*u).byte_id[0x81 as ::core::ffi::c_int as usize] >= 0 as int32_t)
                    as ::core::ffi::c_int;
            } else {
                all = ((*u).byte_id[c as usize] >= 0 as int32_t) as ::core::ffi::c_int;
            }
            j = j.wrapping_add(1);
        }
        if all != 0 {
            let mut j_0: uint64_t = s;
            while j_0 < end {
                let mut c_0: uint8_t = pbyte(virt, p, j_0);
                if (*u).remap as ::core::ffi::c_int != 0
                    && c_0 as ::core::ffi::c_uint == 0x20 as ::core::ffi::c_uint
                {
                    toks_put(
                        e,
                        (*u).byte_id[0xe2 as ::core::ffi::c_int as usize] as uint32_t,
                    );
                    toks_put(
                        e,
                        (*u).byte_id[0x96 as ::core::ffi::c_int as usize] as uint32_t,
                    );
                    toks_put(
                        e,
                        (*u).byte_id[0x81 as ::core::ffi::c_int as usize] as uint32_t,
                    );
                } else {
                    toks_put(e, (*u).byte_id[c_0 as usize] as uint32_t);
                }
                j_0 = j_0.wrapping_add(1);
            }
            return;
        }
    }
    toks_put(e, (*u).unk_id);
}
pub const UNI_BID: ::core::ffi::c_uint = 512 as ::core::ffi::c_uint;
unsafe extern "C" fn uni_piece(
    mut u: *const toks_uni,
    mut virt: ::core::ffi::c_int,
    mut p: *const uint8_t,
    mut len: uint64_t,
    mut em: *mut toks_emit,
    mut delta: *mut uint8_t,
) {
    virt = (virt != 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
    let mut L: uint64_t = len.wrapping_add(virt as uint64_t);
    if L == 0 as uint64_t {
        return;
    }
    let mut best: [::core::ffi::c_double; 128] = [0.; 128];
    let mut bid: [int32_t; 512] = [0; 512];
    let mut cell: *const toks_uni_cell = (*u).cell;
    let mut score: *const ::core::ffi::c_double = (*u).score;
    let mut term: *const int32_t = (*u).term;
    let mut keep: ::core::ffi::c_int = (L < UNI_BID as uint64_t) as ::core::ffi::c_int;
    memset(
        delta as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        L.wrapping_add(1 as uint64_t) as size_t,
    );
    best[0 as ::core::ffi::c_int as usize] = 0.0f64;
    let mut s: uint64_t = 0 as uint64_t;
    while s < L {
        let mut mb: uint32_t = 1 as uint32_t;
        if !(virt != 0 && s == 0 as uint64_t) {
            let mut i: uint64_t = s.wrapping_sub(virt as uint64_t);
            mb = toks_utf8_len(p.offset(i as isize), len.wrapping_sub(i));
            if mb == 0 as uint32_t {
                mb = 1 as ::core::ffi::c_uint as uint32_t;
            }
        }
        let mut bs: ::core::ffi::c_double = best[(s & UNI_MAX_PIECE as uint64_t)
            as usize];
        let mut single: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut node: uint32_t = 0 as uint32_t;
        let mut nb: uint32_t = (*cell.offset(0 as ::core::ffi::c_int as isize)).base;
        let mut lim: uint64_t = if L.wrapping_sub(s) < (*u).max_piece as uint64_t {
            L.wrapping_sub(s)
        } else {
            (*u).max_piece as uint64_t
        };
        let mut v0: ::core::ffi::c_int = (virt != 0 && s == 0 as uint64_t)
            as ::core::ffi::c_int;
        let mut q: *const uint8_t = if v0 != 0 {
            p
        } else {
            p.offset(s.wrapping_sub(virt as uint64_t) as isize)
        };
        let mut k: uint64_t = 0 as uint64_t;
        while k < lim {
            let mut by: uint32_t = if v0 != 0 {
                if k == 0 as uint64_t {
                    0x20 as uint32_t
                } else {
                    *q.offset(k.wrapping_sub(1 as uint64_t) as isize) as uint32_t
                }
            } else {
                *q.offset(k as isize) as uint32_t
            };
            let mut t: uint32_t = (nb & TOKS_UNI_BASE as uint32_t).wrapping_add(by);
            let mut c: *const toks_uni_cell = cell.offset(t as isize)
                as *const toks_uni_cell;
            if (*c).check != node as int32_t {
                break;
            }
            node = t;
            nb = (*c).base;
            if !(nb & TOKS_UNI_TERM as uint32_t == 0) {
                let mut e: uint64_t = s.wrapping_add(k).wrapping_add(1 as uint64_t);
                let mut cand: ::core::ffi::c_double = *score.offset(t as isize) + bs;
                if *delta.offset(e as isize) as ::core::ffi::c_uint
                    == 0 as ::core::ffi::c_uint
                    || cand > best[(e & UNI_MAX_PIECE as uint64_t) as usize]
                {
                    best[(e & UNI_MAX_PIECE as uint64_t) as usize] = cand;
                    *delta.offset(e as isize) = k.wrapping_add(1 as uint64_t) as uint8_t;
                    if keep != 0 {
                        bid[e as usize] = *term.offset(t as isize);
                    }
                }
                if k.wrapping_add(1 as uint64_t) == mb as uint64_t {
                    single = 1 as ::core::ffi::c_int;
                }
            }
            k = k.wrapping_add(1);
        }
        if single == 0 {
            let mut e_0: uint64_t = s.wrapping_add(mb as uint64_t);
            let mut cand_0: ::core::ffi::c_double = (*u).unk_score + bs;
            if *delta.offset(e_0 as isize) as ::core::ffi::c_uint
                == 0 as ::core::ffi::c_uint
                || cand_0 > best[(e_0 & UNI_MAX_PIECE as uint64_t) as usize]
            {
                best[(e_0 & UNI_MAX_PIECE as uint64_t) as usize] = cand_0;
                *delta.offset(e_0 as isize) = mb as uint8_t;
                if keep != 0 {
                    bid[e_0 as usize] = -(1 as ::core::ffi::c_int) as int32_t;
                }
            }
        }
        s = s.wrapping_add(mb as uint64_t);
    }
    let mut e_1: uint64_t = L;
    while e_1 > 0 as uint64_t {
        let mut d: uint32_t = *delta.offset(e_1 as isize) as uint32_t & 0x7f as uint32_t;
        *delta.offset(e_1 as isize) = (*delta.offset(e_1 as isize) as ::core::ffi::c_uint
            | 0x80 as ::core::ffi::c_uint) as uint8_t;
        e_1 = e_1.wrapping_sub(d as uint64_t);
    }
    let mut ps: uint64_t = 0 as uint64_t;
    let mut us: uint64_t = 0 as uint64_t;
    let mut in_unk: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut x: uint64_t = 1 as uint64_t;
    while x <= L {
        if !(*delta.offset(x as isize) as ::core::ffi::c_uint
            & 0x80 as ::core::ffi::c_uint == 0 as ::core::ffi::c_uint)
        {
            let mut id: int32_t = if keep != 0 {
                bid[x as usize]
            } else {
                exact_id(u, virt, p, ps, x)
            };
            if id < 0 as int32_t || id as uint32_t == (*u).unk_id {
                if in_unk == 0 {
                    in_unk = 1 as ::core::ffi::c_int;
                    us = ps;
                }
            } else {
                if in_unk != 0 {
                    emit_unknown(u, virt, p, us, ps, em);
                    in_unk = 0 as ::core::ffi::c_int;
                }
                toks_put(em, id as uint32_t);
            }
            ps = x;
        }
        x = x.wrapping_add(1);
    }
    if in_unk != 0 {
        emit_unknown(u, virt, p, us, L, em);
    }
}
#[inline]
unsafe extern "C" fn uni_key(
    mut virt: ::core::ffi::c_int,
    mut b: *const uint8_t,
    mut l: uint64_t,
    mut avail: uint64_t,
) -> bpe_key {
    let mut k: bpe_key = bpe_key {
        lo: 0 as uint64_t,
        hi: 0 as uint64_t,
    };
    if l != 0 as uint64_t {
        k = bpe_key_at(b, avail, 0 as uint64_t, l);
    }
    if virt != 0 {
        k.hi = (k.hi as ::core::ffi::c_ulong
            | (0x80 as ::core::ffi::c_ulong) << 56 as ::core::ffi::c_int) as uint64_t;
    }
    return k;
}
#[inline]
unsafe extern "C" fn uni_val(
    mut val: *mut uint32_t,
    mut ids: *const uint32_t,
    mut n: uint64_t,
    mut tw: uint64_t,
) {
    bpe_val_pack_tag(val, ids, n as uint32_t, tw);
}
unsafe extern "C" fn cand_sift(
    mut c: *mut uni_cand,
    mut root: uint64_t,
    mut end: uint64_t,
) {
    while (2 as uint64_t).wrapping_mul(root).wrapping_add(1 as uint64_t) < end {
        let mut m: uint64_t = (2 as uint64_t)
            .wrapping_mul(root)
            .wrapping_add(1 as uint64_t);
        if m.wrapping_add(1 as uint64_t) < end
            && (*c.offset(m.wrapping_add(1 as uint64_t) as isize)).score
                < (*c.offset(m as isize)).score
        {
            m = m.wrapping_add(1);
        }
        if !((*c.offset(m as isize)).score < (*c.offset(root as isize)).score) {
            return;
        }
        let mut t: uni_cand = *c.offset(root as isize);
        *c.offset(root as isize) = *c.offset(m as isize);
        *c.offset(m as isize) = t;
        root = m;
    }
}
unsafe extern "C" fn uni_words(
    mut u: *mut toks_uni,
    mut words: *mut uint8_t,
    mut wb: uint64_t,
    mut kb: *const uint8_t,
    mut keys: *const ukey,
    mut nk: uint64_t,
    mut score: *const ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    let mut nc: uint64_t = 0 as uint64_t;
    let mut c: *mut uni_cand = toks_plat_alloc(
        nk
            .wrapping_add(1 as uint64_t)
            .wrapping_mul(::core::mem::size_of::<uni_cand>() as uint64_t),
    ) as *mut uni_cand;
    if c.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    let mut i: uint64_t = 0 as uint64_t;
    while i < nk {
        if !((*keys.offset(i as isize))
            .len
            .wrapping_sub(
                ((*u).remap as ::core::ffi::c_int != 0
                    && *kb.offset((*keys.offset(i as isize)).off as isize)
                        as ::core::ffi::c_uint == 0x20 as ::core::ffi::c_uint)
                    as ::core::ffi::c_int as uint32_t,
            ) > TOKS_KEY_MAXLEN as uint32_t)
        {
            (*c.offset(nc as isize)).score = *score
                .offset((*keys.offset(i as isize)).id as isize);
            (*c.offset(nc as isize)).key = i as uint32_t;
            nc = nc.wrapping_add(1);
        }
        i = i.wrapping_add(1);
    }
    let mut i_0: uint64_t = nc.wrapping_div(2 as uint64_t);
    while i_0 > 0 as uint64_t {
        cand_sift(c, i_0.wrapping_sub(1 as uint64_t), nc);
        i_0 = i_0.wrapping_sub(1);
    }
    let mut e: uint64_t = nc;
    while e > 1 as uint64_t {
        let mut t: uni_cand = *c.offset(0 as ::core::ffi::c_int as isize);
        *c.offset(0 as ::core::ffi::c_int as isize) = *c
            .offset(e.wrapping_sub(1 as uint64_t) as isize);
        *c.offset(e.wrapping_sub(1 as uint64_t) as isize) = t;
        cand_sift(c, 0 as uint64_t, e.wrapping_sub(1 as uint64_t));
        e = e.wrapping_sub(1);
    }
    memset(
        words as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        wb.wrapping_mul(TOKS_BUCKET as uint64_t) as size_t,
    );
    let mut delta: [uint8_t; 18] = [0; 18];
    let mut bytes: [uint8_t; 31] = [0; 31];
    let mut ids: [uint32_t; 32] = [0; 32];
    let mut val: [uint32_t; 4] = [0; 4];
    let mut j: uint64_t = 0 as uint64_t;
    while j < nc {
        let mut k: *const ukey = keys.offset((*c.offset(j as isize)).key as isize)
            as *const ukey;
        let mut virt: ::core::ffi::c_int = ((*u).remap as ::core::ffi::c_int != 0
            && *kb.offset((*k).off as isize) as ::core::ffi::c_uint
                == 0x20 as ::core::ffi::c_uint) as ::core::ffi::c_int;
        let mut l: uint64_t = (*k).len.wrapping_sub(virt as uint32_t) as uint64_t;
        memset(
            &raw mut bytes as *mut uint8_t as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<[uint8_t; 31]>() as size_t,
        );
        memcpy(
            &raw mut bytes as *mut uint8_t as *mut ::core::ffi::c_void,
            kb.offset((*k).off as isize).offset(virt as uint32_t as isize)
                as *const ::core::ffi::c_void,
            l as size_t,
        );
        let mut e_0: toks_emit = toks_emit {
            out: &raw mut ids as *mut uint32_t,
            cap: 32 as uint64_t,
            n: 0 as uint64_t,
            lim: UINT64_MAX as uint64_t,
        };
        uni_piece(
            u,
            virt,
            &raw mut bytes as *mut uint8_t,
            l,
            &raw mut e_0,
            &raw mut delta as *mut uint8_t,
        );
        if !(e_0.n == 0 as uint64_t || e_0.n > 4 as uint64_t) {
            let mut key: bpe_key = uni_key(
                virt,
                &raw mut bytes as *mut uint8_t,
                l,
                ::core::mem::size_of::<[uint8_t; 31]>() as uint64_t,
            );
            uni_val(
                &raw mut val as *mut uint32_t,
                &raw mut ids as *mut uint32_t,
                e_0.n,
                0 as uint64_t,
            );
            bpe_words_put(
                words,
                wb.wrapping_sub(1 as uint64_t),
                toks_spm_whash(key.lo, key.hi),
                key,
                &raw mut val as *mut uint32_t as *const uint32_t,
            );
        }
        j = j.wrapping_add(1);
    }
    toks_plat_free(
        c as *mut ::core::ffi::c_void,
        nk
            .wrapping_add(1 as uint64_t)
            .wrapping_mul(::core::mem::size_of::<uni_cand>() as uint64_t),
    );
    (*u).words = words;
    (*u).words_mask = wb.wrapping_sub(1 as uint64_t);
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn toks_uni_build(
    mut src: *const toks_uni_src,
    mut uo: *mut *const toks_uni,
    mut memo: *mut *mut uint8_t,
    mut mem_leno: *mut uint64_t,
    mut why: *mut *const ::core::ffi::c_char,
) -> int64_t {
    let mut arr: *mut int32_t = ::core::ptr::null_mut::<int32_t>();
    let mut cap: uint64_t = 0;
    let mut da_len: uint64_t = 0;
    let mut dwhy: *const ::core::ffi::c_char = ::core::ptr::null::<
        ::core::ffi::c_char,
    >();
    let mut dr: int64_t = 0;
    let mut base: *const int32_t = ::core::ptr::null::<int32_t>();
    let mut check: *const int32_t = ::core::ptr::null::<int32_t>();
    let mut term: *const int32_t = ::core::ptr::null::<int32_t>();
    let mut da_bytes: uint64_t = 0;
    let mut da: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    let mut pc_bytes: uint64_t = 0;
    let mut n_cand: uint64_t = 0;
    let mut wb: uint64_t = 0;
    let mut o_u: uint64_t = 0;
    let mut o_cell: uint64_t = 0;
    let mut o_score: uint64_t = 0;
    let mut o_term: uint64_t = 0;
    let mut o_pc: uint64_t = 0;
    let mut o_words: uint64_t = 0;
    let mut total: uint64_t = 0;
    let mut mem: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    let mut u: *mut toks_uni = ::core::ptr::null_mut::<toks_uni>();
    let mut cl: *mut toks_uni_cell = ::core::ptr::null_mut::<toks_uni_cell>();
    let mut sc: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<
        ::core::ffi::c_double,
    >();
    let mut tm: *mut int32_t = ::core::ptr::null_mut::<int32_t>();
    let mut cf: *const toks_uni_cfg = ::core::ptr::null::<toks_uni_cfg>();
    let mut current_block: u64;
    *uo = ::core::ptr::null::<toks_uni>();
    *memo = ::core::ptr::null_mut::<uint8_t>();
    *mem_leno = 0 as uint64_t;
    if (*src).n == 0 as uint32_t {
        return fail_why(
            why,
            b"Unigram vocab empty\0" as *const u8 as *const ::core::ffi::c_char,
            TOKS_E_FORMAT as int64_t,
        );
    }
    if (*src).unk_id < 0 as int64_t {
        return fail_why(
            why,
            b"Unigram without unk_id (hf: MissingUnkId on unknown chars)\0" as *const u8
                as *const ::core::ffi::c_char,
            TOKS_E_UNSUPPORTED as int64_t,
        );
    }
    if (*src).unk_id as uint64_t >= (*src).n as uint64_t {
        return fail_why(
            why,
            b"Unigram unk_id outside the vocab\0" as *const u8
                as *const ::core::ffi::c_char,
            TOKS_E_FORMAT as int64_t,
        );
    }
    let mut remap: ::core::ffi::c_int = ((*src).cfg.metaspace as ::core::ffi::c_uint
        != 0 as ::core::ffi::c_uint
        || (*src).cfg.meta_replace as ::core::ffi::c_uint != 0 as ::core::ffi::c_uint)
        as ::core::ffi::c_int;
    let mut kb_len: uint64_t = 0 as uint64_t;
    let mut i: uint32_t = 0 as uint32_t;
    while i < (*src).n {
        kb_len = kb_len.wrapping_add(*(*src).piece_len.offset(i as isize) as uint64_t);
        i = i.wrapping_add(1);
    }
    let mut tmp_len: uint64_t = kb_len
        .wrapping_add(64 as uint64_t)
        .wrapping_add(
            ((*src).n as uint64_t)
                .wrapping_mul(
                    (::core::mem::size_of::<ukey>() as uint64_t)
                        .wrapping_add(8 as uint64_t),
                ),
        )
        .wrapping_add(64 as uint64_t);
    let mut tmp: *mut uint8_t = toks_plat_alloc(tmp_len) as *mut uint8_t;
    if tmp.is_null() {
        return fail_why(
            why,
            b"Unigram build memory\0" as *const u8 as *const ::core::ffi::c_char,
            TOKS_E_NOMEM as int64_t,
        );
    }
    let mut kb: *mut uint8_t = tmp;
    let mut keys: *mut ukey = tmp
        .offset(
            (kb_len.wrapping_add(63 as uint64_t)
                & !(63 as ::core::ffi::c_uint as uint64_t)) as isize,
        ) as *mut ::core::ffi::c_void as *mut ukey;
    let mut score: *mut ::core::ffi::c_double = (keys as *mut uint8_t)
        .offset(
            ((*src).n as uint64_t)
                .wrapping_mul(::core::mem::size_of::<ukey>() as uint64_t) as isize,
        ) as *mut ::core::ffi::c_void as *mut ::core::ffi::c_double;
    let mut ret: int64_t = 0 as int64_t;
    let mut nk: uint64_t = 0 as uint64_t;
    let mut kpos: uint64_t = 0 as uint64_t;
    let mut n_dropped: uint64_t = 0 as uint64_t;
    let mut max_piece: uint32_t = 0 as uint32_t;
    let mut min_score: ::core::ffi::c_double = 0.0f64;
    let mut byte_id: [int32_t; 256] = [0; 256];
    let mut b: uint32_t = 0 as uint32_t;
    while b < 256 as uint32_t {
        byte_id[b as usize] = -(1 as ::core::ffi::c_int) as int32_t;
        b = b.wrapping_add(1);
    }
    let mut i_0: uint32_t = 0 as uint32_t;
    loop {
        if !(i_0 < (*src).n) {
            current_block = 9441801433784995173;
            break;
        }
        if parse_score(
            *(*src).score_txt.offset(i_0 as isize),
            *(*src).score_len.offset(i_0 as isize),
            score.offset(i_0 as isize) as *mut ::core::ffi::c_double,
        ) != 0 as ::core::ffi::c_int
        {
            ret = fail_why(
                why,
                b"Unigram score out of range (serde refuses it)\0" as *const u8
                    as *const ::core::ffi::c_char,
                TOKS_E_FORMAT as int64_t,
            );
            current_block = 8285098540978962305;
            break;
        } else {
            if i_0 == 0 as uint32_t || *score.offset(i_0 as isize) < min_score {
                min_score = *score.offset(i_0 as isize);
            }
            let mut s: *const uint8_t = *(*src).piece.offset(i_0 as isize);
            let mut l: uint32_t = *(*src).piece_len.offset(i_0 as isize);
            if l == 6 as uint32_t
                && *s.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == '<' as i32
                && *s.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == '0' as i32
                && *s.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == 'x' as i32
                && *s.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == '>' as i32
            {
                let mut hi: ::core::ffi::c_int = if *s
                    .offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    >= '0' as i32
                    && *s.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        <= '9' as i32
                {
                    *s.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        - '0' as i32
                } else if *s.offset(3 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int >= 'A' as i32
                    && *s.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        <= 'F' as i32
                {
                    *s.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        - 'A' as i32 + 10 as ::core::ffi::c_int
                } else {
                    -(1 as ::core::ffi::c_int)
                };
                let mut lo: ::core::ffi::c_int = if *s
                    .offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    >= '0' as i32
                    && *s.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        <= '9' as i32
                {
                    *s.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        - '0' as i32
                } else if *s.offset(4 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int >= 'A' as i32
                    && *s.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        <= 'F' as i32
                {
                    *s.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        - 'A' as i32 + 10 as ::core::ffi::c_int
                } else {
                    -(1 as ::core::ffi::c_int)
                };
                if hi >= 0 as ::core::ffi::c_int && lo >= 0 as ::core::ffi::c_int {
                    byte_id[(hi * 16 as ::core::ffi::c_int + lo) as usize] = i_0
                        as int32_t;
                }
            }
            let mut m: int64_t = remap_piece(s, l, remap, kb.offset(kpos as isize));
            if m < 0 as int64_t {
                n_dropped = n_dropped.wrapping_add(1);
            } else if !(m == 0 as int64_t) {
                if m as uint64_t > UNI_MAX_PIECE as uint64_t {
                    ret = fail_why(
                        why,
                        b"Unigram piece longer than 127 bytes\0" as *const u8
                            as *const ::core::ffi::c_char,
                        TOKS_E_UNSUPPORTED as int64_t,
                    );
                    current_block = 8285098540978962305;
                    break;
                } else {
                    if m as uint32_t > max_piece {
                        max_piece = m as uint32_t;
                    }
                    (*keys.offset(nk as isize)).off = kpos;
                    (*keys.offset(nk as isize)).len = m as uint32_t;
                    (*keys.offset(nk as isize)).id = i_0;
                    nk = nk.wrapping_add(1);
                    kpos = kpos.wrapping_add(m as uint64_t);
                }
            }
            i_0 = i_0.wrapping_add(1);
        }
    }
    match current_block {
        9441801433784995173 => {
            arr = ::core::ptr::null_mut::<int32_t>();
            cap = 0 as uint64_t;
            da_len = 0 as uint64_t;
            dwhy = ::core::ptr::null::<::core::ffi::c_char>();
            dr = toks_da_build(
                kb,
                keys,
                &raw mut nk,
                &raw mut arr,
                &raw mut cap,
                &raw mut da_len,
                &raw mut dwhy,
            );
            if dr != 0 as int64_t {
                ret = fail_why(
                    why,
                    if dr == TOKS_E_NOMEM as int64_t {
                        b"Unigram trie memory\0" as *const u8
                            as *const ::core::ffi::c_char
                    } else {
                        b"Unigram trie too large\0" as *const u8
                            as *const ::core::ffi::c_char
                    },
                    dr,
                );
            } else {
                base = arr;
                check = arr.offset(cap as isize);
                term = arr.offset((2 as uint64_t).wrapping_mul(cap) as isize);
                da_bytes = cap.wrapping_mul(12 as uint64_t);
                da = arr as *mut uint8_t;
                pc_bytes = 0 as uint64_t;
                if (*src).cfg.has_charsmap != 0 {
                    pc_bytes = toks_pc_bytes((*src).charsmap, (*src).charsmap_len, why);
                    if pc_bytes == 0 as uint64_t {
                        toks_plat_free(da as *mut ::core::ffi::c_void, da_bytes);
                        ret = TOKS_E_UNSUPPORTED as int64_t;
                        current_block = 8285098540978962305;
                    } else {
                        current_block = 10930818133215224067;
                    }
                } else {
                    current_block = 10930818133215224067;
                }
                match current_block {
                    8285098540978962305 => {}
                    _ => {
                        n_cand = 0 as uint64_t;
                        let mut i_1: uint64_t = 0 as uint64_t;
                        while i_1 < nk {
                            n_cand = n_cand
                                .wrapping_add(
                                    ((*keys.offset(i_1 as isize))
                                        .len
                                        .wrapping_sub(
                                            (remap != 0
                                                && *kb.offset((*keys.offset(i_1 as isize)).off as isize)
                                                    as ::core::ffi::c_uint == 0x20 as ::core::ffi::c_uint)
                                                as ::core::ffi::c_int as uint32_t,
                                        ) <= TOKS_KEY_MAXLEN as uint32_t) as ::core::ffi::c_int
                                        as uint64_t,
                                );
                            i_1 = i_1.wrapping_add(1);
                        }
                        wb = if n_cand != 0 as uint64_t {
                            bpe_pow2(
                                n_cand
                                    .wrapping_div(2 as uint64_t)
                                    .wrapping_add(1 as uint64_t),
                            )
                        } else {
                            0 as uint64_t
                        };
                        o_u = 0 as uint64_t;
                        o_cell = (::core::mem::size_of::<toks_uni>() as uint64_t)
                            .wrapping_add(63 as uint64_t)
                            & !(63 as ::core::ffi::c_uint as uint64_t);
                        o_score = o_cell
                            .wrapping_add(
                                da_len
                                    .wrapping_add(256 as uint64_t)
                                    .wrapping_mul(
                                        ::core::mem::size_of::<toks_uni_cell>() as uint64_t,
                                    )
                                    .wrapping_add(63 as uint64_t)
                                    & !(63 as ::core::ffi::c_uint as uint64_t),
                            );
                        o_term = o_score
                            .wrapping_add(
                                da_len
                                    .wrapping_mul(8 as uint64_t)
                                    .wrapping_add(63 as uint64_t)
                                    & !(63 as ::core::ffi::c_uint as uint64_t),
                            );
                        o_pc = o_term
                            .wrapping_add(
                                da_len
                                    .wrapping_mul(4 as uint64_t)
                                    .wrapping_add(63 as uint64_t)
                                    & !(63 as ::core::ffi::c_uint as uint64_t),
                            );
                        o_words = o_pc
                            .wrapping_add(
                                pc_bytes
                                    .wrapping_add(64 as uint64_t)
                                    .wrapping_add(63 as uint64_t)
                                    & !(63 as ::core::ffi::c_uint as uint64_t),
                            );
                        total = o_words
                            .wrapping_add(wb.wrapping_mul(TOKS_BUCKET as uint64_t))
                            .wrapping_add(64 as uint64_t);
                        mem = toks_plat_arena(total);
                        if mem.is_null() {
                            toks_plat_free(da as *mut ::core::ffi::c_void, da_bytes);
                            ret = fail_why(
                                why,
                                b"Unigram tables\0" as *const u8
                                    as *const ::core::ffi::c_char,
                                TOKS_E_NOMEM as int64_t,
                            );
                        } else {
                            u = toks_tab(
                                mem,
                                o_u,
                                ::core::mem::size_of::<toks_uni>() as uint64_t,
                                TOKS_X_UNI,
                            ) as *mut toks_uni;
                            memset(
                                u as *mut ::core::ffi::c_void,
                                0 as ::core::ffi::c_int,
                                ::core::mem::size_of::<toks_uni>() as size_t,
                            );
                            (*u).cfg = (*src).cfg;
                            (*u).n_dec = dec_ops(
                                &raw const (*src).cfg,
                                &raw mut (*u).dec as *mut toks_spm_op,
                            );
                            cl = toks_tab(
                                mem,
                                o_cell,
                                da_len
                                    .wrapping_add(256 as uint64_t)
                                    .wrapping_mul(
                                        ::core::mem::size_of::<toks_uni_cell>() as uint64_t,
                                    ),
                                TOKS_X_UNI_CELLS,
                            ) as *mut toks_uni_cell;
                            sc = toks_tab(
                                mem,
                                o_score,
                                da_len.wrapping_mul(8 as uint64_t),
                                TOKS_X_UNI_SCORE,
                            ) as *mut ::core::ffi::c_double;
                            tm = toks_tab(
                                mem,
                                o_term,
                                da_len.wrapping_mul(4 as uint64_t),
                                TOKS_X_UNI_TERM,
                            ) as *mut int32_t;
                            let mut x: uint64_t = 0 as uint64_t;
                            while x < da_len.wrapping_add(256 as uint64_t) {
                                let mut in_0: ::core::ffi::c_int = (x < da_len)
                                    as ::core::ffi::c_int;
                                let mut t: ::core::ffi::c_int = if in_0 != 0 {
                                    *term.offset(x as isize) as ::core::ffi::c_int
                                } else {
                                    -(1 as ::core::ffi::c_int)
                                };
                                (*cl.offset(x as isize)).base = if in_0 != 0 {
                                    *base.offset(x as isize) as uint32_t
                                        | (if t >= 0 as ::core::ffi::c_int {
                                            TOKS_UNI_TERM as uint32_t
                                        } else {
                                            0 as uint32_t
                                        })
                                } else {
                                    0 as uint32_t
                                };
                                (*cl.offset(x as isize)).check = if in_0 != 0 {
                                    *check.offset(x as isize)
                                } else {
                                    UNI_FREE as int32_t
                                };
                                if in_0 != 0 {
                                    *sc.offset(x as isize) = if t >= 0 as ::core::ffi::c_int {
                                        *score.offset(t as isize)
                                    } else {
                                        0.0f64
                                    };
                                    *tm.offset(x as isize) = t as int32_t;
                                }
                                x = x.wrapping_add(1);
                            }
                            toks_plat_free(da as *mut ::core::ffi::c_void, da_bytes);
                            (*u).cell = cl;
                            (*u).score = sc;
                            (*u).term = tm;
                            (*u).da_len = da_len as uint32_t;
                            (*u).n_vocab = (*src).n;
                            (*u).unk_score = min_score - 10.0f64;
                            (*u).unk_id = (*src).unk_id as uint32_t;
                            (*u).max_piece = max_piece;
                            let mut b_0: uint32_t = 0 as uint32_t;
                            while b_0 < 256 as uint32_t {
                                (*u).byte_id[b_0 as usize] = byte_id[b_0 as usize];
                                b_0 = b_0.wrapping_add(1);
                            }
                            (*u).byte_fallback = ((*src).byte_fallback != 0 as uint32_t)
                                as ::core::ffi::c_int as uint8_t;
                            (*u).remap = remap as uint8_t;
                            (*u).n_dropped = n_dropped as uint32_t;
                            (*u).work_x = 1 as ::core::ffi::c_uint as uint32_t;
                            (*u).pre_x = (if (*src).cfg.form as ::core::ffi::c_uint
                                != 0 as ::core::ffi::c_uint
                            {
                                if (*src).cfg.form as ::core::ffi::c_uint & TOKS_NS_COMPAT
                                    != 0 as ::core::ffi::c_uint
                                {
                                    TOKS_NFKC_X
                                } else {
                                    TOKS_NFC_X
                                }
                            } else {
                                1 as ::core::ffi::c_uint
                            }) as uint32_t;
                            if (*src).cfg.has_charsmap != 0 {
                                let mut par: toks_arena = toks_arena {
                                    base: mem.offset(o_pc as isize),
                                    len: pc_bytes.wrapping_add(64 as uint64_t),
                                    pos: 0 as uint64_t,
                                };
                                let mut r: int64_t = toks_pc_build(
                                    &raw mut (*u).pc,
                                    (*src).charsmap,
                                    (*src).charsmap_len,
                                    &raw mut par,
                                    why,
                                );
                                if r != 0 as int64_t {
                                    toks_tab_free(mem, total);
                                    ret = r;
                                    current_block = 8285098540978962305;
                                } else {
                                    (*u).work_x = (*u).pc.max_expand;
                                    if (*u).work_x > 11 as uint32_t {
                                        toks_tab_free(mem, total);
                                        ret = fail_why(
                                            why,
                                            b"precompiled charsmap expands a byte more than 11x (nmt_nfkc: 11)\0"
                                                as *const u8 as *const ::core::ffi::c_char,
                                            TOKS_E_UNSUPPORTED as int64_t,
                                        );
                                        current_block = 8285098540978962305;
                                    } else {
                                        current_block = 17995254032144898061;
                                    }
                                }
                            } else {
                                current_block = 17995254032144898061;
                            }
                            match current_block {
                                8285098540978962305 => {}
                                _ => {
                                    let mut b_1: uint32_t = 0 as uint32_t;
                                    while b_1 < 128 as uint32_t {
                                        (*u).aent[b_1 as usize] = if (*src).cfg.has_charsmap
                                            as ::core::ffi::c_int != 0
                                        {
                                            toks_pc_char(&raw mut (*u).pc, b_1)
                                        } else {
                                            0 as uint32_t
                                        };
                                        (*u).simple[b_1 as usize] = (b_1 != 0x20 as uint32_t
                                            && !((*src).cfg.ws_split as ::core::ffi::c_int != 0
                                                && toks_is_regex_ws(b_1) != 0)
                                            && !((*src).cfg.has_charsmap as ::core::ffi::c_int != 0
                                                && ((*u).aent[b_1 as usize] != 0 as uint32_t
                                                    || toks_gc_class(b_1) as ::core::ffi::c_uint
                                                        != 0 as ::core::ffi::c_uint))) as ::core::ffi::c_int
                                            as uint8_t;
                                        b_1 = b_1.wrapping_add(1);
                                    }
                                    cf = &raw const (*src).cfg;
                                    (*u).fast = ((*cf).has_charsmap as ::core::ffi::c_int != 0
                                        && (*cf).metaspace as ::core::ffi::c_int != 0
                                        && (*cf).meta_split as ::core::ffi::c_int != 0
                                        && (*cf).meta_prepend as ::core::ffi::c_int != 0
                                        && (*cf).meta_prefix == 0 && (*cf).meta_replace == 0
                                        && remap != 0) as ::core::ffi::c_int as uint8_t;
                                    let mut b_2: uint32_t = 0 as uint32_t;
                                    while b_2 < 128 as uint32_t
                                        && (*u).fast as ::core::ffi::c_int != 0
                                    {
                                        let mut e: uint32_t = (*u).aent[b_2 as usize];
                                        let mut sp: ::core::ffi::c_int = (b_2 == 0x20 as uint32_t
                                            && e == 0 as uint32_t
                                            || e != 0 as uint32_t
                                                && e & 0xfff as uint32_t == 1 as uint32_t
                                                && *(*u)
                                                    .pc
                                                    .pool
                                                    .offset(
                                                        (e >> 12 as ::core::ffi::c_int & 0x3ffff as uint32_t)
                                                            as isize,
                                                    ) as ::core::ffi::c_uint == 0x20 as ::core::ffi::c_uint)
                                            as ::core::ffi::c_int;
                                        (*u).acls[b_2 as usize] = (if (*u).simple[b_2 as usize]
                                            as ::core::ffi::c_int != 0
                                        {
                                            1 as ::core::ffi::c_uint
                                        } else if sp != 0 {
                                            2 as ::core::ffi::c_uint
                                        } else {
                                            0 as ::core::ffi::c_uint
                                        }) as uint8_t;
                                        b_2 = b_2.wrapping_add(1);
                                    }
                                    if wb != 0 as uint64_t
                                        && uni_words(
                                            u,
                                            toks_tab(
                                                mem,
                                                o_words,
                                                wb.wrapping_mul(TOKS_BUCKET as uint64_t),
                                                TOKS_X_WORDS,
                                            ) as *mut uint8_t,
                                            wb,
                                            kb,
                                            keys,
                                            nk,
                                            score,
                                        ) != 0 as ::core::ffi::c_int
                                    {
                                        toks_tab_free(mem, total);
                                        ret = fail_why(
                                            why,
                                            b"Unigram build memory\0" as *const u8
                                                as *const ::core::ffi::c_char,
                                            TOKS_E_NOMEM as int64_t,
                                        );
                                    } else {
                                        toks_tab_seal(mem as *mut ::core::ffi::c_void, total);
                                        *uo = u;
                                        *memo = mem;
                                        *mem_leno = total;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        _ => {}
    }
    toks_plat_free(tmp as *mut ::core::ffi::c_void, tmp_len);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn toks_uni_area(
    mut u: *const toks_uni,
    mut len: uint64_t,
) -> uint64_t {
    return ((*u).work_x as uint64_t)
        .wrapping_mul((*u).pre_x as uint64_t)
        .wrapping_mul(len)
        .wrapping_add(3 as uint64_t)
        .wrapping_add(63 as uint64_t) & !(63 as ::core::ffi::c_uint as uint64_t);
}
pub const NOSRC: ::core::ffi::c_ulong = UINT64_MAX;
pub const UNI_Q: ::core::ffi::c_uint = 16 as ::core::ffi::c_uint;
pub const UNI_RESOLVE_IDS: ::core::ffi::c_uint = (3 as ::core::ffi::c_uint)
    .wrapping_mul(
        (TOKS_KEY_MAXLEN as ::core::ffi::c_uint).wrapping_add(1 as ::core::ffi::c_uint),
    );
#[inline]
unsafe extern "C" fn emit_val(mut e: *mut toks_emit, mut v: *const uint8_t) {
    let mut w: [uint32_t; 4] = [0; 4];
    memcpy(
        &raw mut w as *mut uint32_t as *mut ::core::ffi::c_void,
        v as *const ::core::ffi::c_void,
        16 as size_t,
    );
    let mut m: uint64_t = (w[0 as ::core::ffi::c_int as usize] >> TOKS_VAL_COUNT_SHIFT)
        as uint64_t;
    if (*e).n.wrapping_add(4 as uint64_t) <= (*e).cap
        && (*e).n.wrapping_add(m) <= (*e).lim
    {
        (*e).n = (*e)
            .n
            .wrapping_add(
                bpe_val_put(
                    v,
                    (*e).out.offset((*e).n as isize) as *mut ::core::ffi::c_void,
                ),
            );
        return;
    }
    let mut j: uint64_t = 0 as uint64_t;
    while j < m {
        toks_put(e, w[j as usize] & TOKS_ID_MASK as uint32_t);
        j = j.wrapping_add(1);
    }
}
unsafe extern "C" fn uni_resolve(
    mut u: *const toks_uni,
    mut c: *mut toks_uni_call,
    mut virt: ::core::ffi::c_int,
    mut b: *const uint8_t,
    mut l: uint64_t,
    mut k: bpe_key,
    mut h: uint32_t,
    mut delta: *mut uint8_t,
) {
    let mut bucket: *mut uint8_t = if !(*c).cache.is_null() {
        (*c)
            .cache
            .offset(
                (h as uint64_t & (*c).cache_mask).wrapping_mul(TOKS_BUCKET as uint64_t)
                    as isize,
            )
    } else {
        ::core::ptr::null_mut::<uint8_t>()
    };
    if !bucket.is_null() {
        let mut v: *const uint8_t = bpe_cache_get(bucket, k, (*c).tw);
        if !v.is_null() {
            emit_val(&raw mut (*c).e, v);
            return;
        }
    }
    let mut val: [uint32_t; 4] = [0; 4];
    if !(*u).words.is_null() {
        let mut v_0: *const uint8_t = bpe_words_probe((*u).words, (*u).words_mask, h, k);
        if !v_0.is_null() {
            if !bucket.is_null() {
                memcpy(
                    &raw mut val as *mut uint32_t as *mut ::core::ffi::c_void,
                    v_0 as *const ::core::ffi::c_void,
                    16 as size_t,
                );
                val[2 as ::core::ffi::c_int as usize] = val[2 as ::core::ffi::c_int
                    as usize] & TOKS_ID_MASK as uint32_t | (*c).tw as uint32_t;
                val[3 as ::core::ffi::c_int as usize] = val[3 as ::core::ffi::c_int
                    as usize] & TOKS_ID_MASK as uint32_t
                    | ((*c).tw >> 32 as ::core::ffi::c_int) as uint32_t;
                bpe_cache_fill(
                    bucket,
                    k,
                    &raw mut val as *mut uint32_t as *const uint32_t,
                );
            }
            emit_val(&raw mut (*c).e, v_0);
            return;
        }
    }
    let mut ids: [uint32_t; 48] = [0; 48];
    let mut t: toks_emit = toks_emit {
        out: &raw mut ids as *mut uint32_t,
        cap: UNI_RESOLVE_IDS as uint64_t,
        n: 0 as uint64_t,
        lim: UINT64_MAX as uint64_t,
    };
    uni_piece(u, virt, b, l, &raw mut t, delta);
    if t.n > UNI_RESOLVE_IDS as uint64_t {
        uni_piece(u, virt, b, l, &raw mut (*c).e, delta);
        return;
    }
    if !bucket.is_null() && t.n >= 1 as uint64_t && t.n <= 4 as uint64_t {
        uni_val(
            &raw mut val as *mut uint32_t,
            &raw mut ids as *mut uint32_t,
            t.n,
            (*c).tw,
        );
        bpe_cache_fill(bucket, k, &raw mut val as *mut uint32_t as *const uint32_t);
    }
    let mut j: uint64_t = 0 as uint64_t;
    while j < t.n {
        toks_put(&raw mut (*c).e, ids[j as usize]);
        j = j.wrapping_add(1);
    }
}
unsafe extern "C" fn q_flush(mut g: *mut seg) {
    let mut j: uint32_t = 0 as uint32_t;
    while j < (*g).qn {
        uni_resolve(
            (*g).u,
            (*g).c,
            (*g).q[j as usize].virt,
            (*g).q[j as usize].b,
            (*g).q[j as usize].l,
            (*g).q[j as usize].k,
            (*g).q[j as usize].h,
            (*g).delta,
        );
        j = j.wrapping_add(1);
    }
    (*g).qn = 0 as ::core::ffi::c_uint as uint32_t;
}
unsafe extern "C" fn piece_close(mut g: *mut seg) {
    if (*g).p_open == 0 {
        return;
    }
    let mut l: uint64_t = if (*g).p_mat != 0 {
        (*g).p_ml
    } else {
        (*g).p_e.wrapping_sub((*g).p_s)
    };
    if (*g).p_virt != 0 || l > 0 as uint64_t {
        let mut c: *mut toks_uni_call = (*g).c;
        let mut u: *const toks_uni = (*g).u;
        if (*c).pieces != 0 {
            toks_put(&raw mut (*c).e, (*c).nbase.wrapping_add((*g).p_nend) as uint32_t);
        } else if l > TOKS_KEY_MAXLEN as uint64_t
            || (*c).cache.is_null() && (*u).words.is_null()
        {
            q_flush(g);
            uni_piece(
                u,
                (*g).p_virt,
                if (*g).p_mat != 0 {
                    (*g).mbuf as *const uint8_t
                } else {
                    (*g).text.offset((*g).p_s as isize)
                },
                l,
                &raw mut (*c).e,
                (*g).delta,
            );
        } else if (*g).p_mat != 0 {
            q_flush(g);
            let mut k: bpe_key = uni_key((*g).p_virt, (*g).mbuf, l, (*g).mcap);
            uni_resolve(
                u,
                c,
                (*g).p_virt,
                (*g).mbuf,
                l,
                k,
                toks_spm_whash(k.lo, k.hi),
                (*g).delta,
            );
        } else {
            let mut k_0: bpe_key = uni_key(
                (*g).p_virt,
                (*g).text.offset((*g).p_s as isize),
                l,
                (*g).tlen.wrapping_sub((*g).p_s),
            );
            let mut h: uint32_t = toks_spm_whash(k_0.lo, k_0.hi);
            !(*c).cache.is_null();
            !(*u).words.is_null();
            let fresh1 = (*g).qn;
            (*g).qn = (*g).qn.wrapping_add(1);
            let mut j: uint32_t = fresh1;
            (*g).q[j as usize].b = (*g).text.offset((*g).p_s as isize);
            (*g).q[j as usize].l = l;
            (*g).q[j as usize].virt = (*g).p_virt;
            (*g).q[j as usize].h = h;
            (*g).q[j as usize].k = k_0;
            if (*g).qn == UNI_Q as uint32_t {
                q_flush(g);
            }
        }
    }
    (*g).p_open = 0 as ::core::ffi::c_int;
    (*g).p_virt = 0 as ::core::ffi::c_int;
    (*g).p_mat = 0 as ::core::ffi::c_int;
    (*g).p_ml = 0 as uint64_t;
    (*g).p_e = 0 as uint64_t;
    (*g).p_s = (*g).p_e;
}
unsafe extern "C" fn piece_open(mut g: *mut seg, mut virt: ::core::ffi::c_int) {
    (*g).p_open = 1 as ::core::ffi::c_int;
    (*g).p_virt = virt;
    (*g).p_mat = 0 as ::core::ffi::c_int;
    (*g).p_ml = 0 as uint64_t;
    (*g).p_e = 0 as uint64_t;
    (*g).p_s = (*g).p_e;
}
unsafe extern "C" fn piece_add(
    mut g: *mut seg,
    mut b: *const uint8_t,
    mut k: uint32_t,
    mut src: uint64_t,
    mut in_place: ::core::ffi::c_int,
) {
    if (*g).p_open == 0 {
        piece_open(g, 0 as ::core::ffi::c_int);
    }
    (*g).p_nend = (*g).npos;
    if (*g).p_virt == 0 && (*g).p_mat == 0 && (*g).p_e == (*g).p_s && k == 1 as uint32_t
        && *b.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint
            == 0x20 as ::core::ffi::c_uint && (*(*g).u).remap as ::core::ffi::c_int != 0
    {
        (*g).p_virt = 1 as ::core::ffi::c_int;
        return;
    }
    if (*g).p_mat == 0 && in_place != 0 {
        if (*g).p_e == (*g).p_s {
            (*g).p_s = src;
            (*g).p_e = src.wrapping_add(k as uint64_t);
            return;
        }
        if src == (*g).p_e {
            (*g).p_e = (*g).p_e.wrapping_add(k as uint64_t);
            return;
        }
    }
    if (*g).p_mat == 0 {
        let mut l: uint64_t = (*g).p_e.wrapping_sub((*g).p_s);
        if l > (*g).mcap {
            (*g).err = 1 as ::core::ffi::c_int;
            return;
        }
        if l > 0 as uint64_t {
            memcpy(
                (*g).mbuf as *mut ::core::ffi::c_void,
                (*g).text.offset((*g).p_s as isize) as *const ::core::ffi::c_void,
                l as size_t,
            );
        }
        (*g).p_ml = l;
        (*g).p_mat = 1 as ::core::ffi::c_int;
    }
    if (*g).p_ml.wrapping_add(k as uint64_t) > (*g).mcap {
        (*g).err = 1 as ::core::ffi::c_int;
        return;
    }
    memcpy(
        (*g).mbuf.offset((*g).p_ml as isize) as *mut ::core::ffi::c_void,
        b as *const ::core::ffi::c_void,
        k as size_t,
    );
    (*g).p_ml = (*g).p_ml.wrapping_add(k as uint64_t);
}
static mut V_BYTE: [uint8_t; 1] = [0x20 as ::core::ffi::c_uint as uint8_t];
unsafe extern "C" fn meta_char(
    mut g: *mut seg,
    mut b: *const uint8_t,
    mut k: uint32_t,
    mut src: uint64_t,
    mut in_place: ::core::ffi::c_int,
    mut is_v: ::core::ffi::c_int,
) {
    let mut c: *const toks_uni_cfg = &raw const (*(*g).u).cfg;
    if (*g).split_start != 0 {
        (*g).split_start = 0 as ::core::ffi::c_int;
        if is_v != 0 {
            piece_open(g, 1 as ::core::ffi::c_int);
            (*g).p_nend = (*g).npos;
            return;
        }
        piece_open(
            g,
            if (*c).meta_prepend as ::core::ffi::c_int != 0 {
                1 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            },
        );
        piece_add(g, b, k, src, in_place);
        return;
    }
    if is_v != 0 && (*c).meta_split as ::core::ffi::c_int != 0 {
        piece_close(g);
        piece_open(g, 1 as ::core::ffi::c_int);
        (*g).p_nend = (*g).npos;
        return;
    }
    if is_v != 0 {
        piece_add(
            g,
            &raw const V_BYTE as *const uint8_t,
            1 as uint32_t,
            src,
            (in_place != 0 && k == 1 as uint32_t) as ::core::ffi::c_int,
        );
        return;
    }
    piece_add(g, b, k, src, in_place);
}
unsafe extern "C" fn pre_char(
    mut g: *mut seg,
    mut cp: uint32_t,
    mut b: *const uint8_t,
    mut k: uint32_t,
    mut src: uint64_t,
    mut in_place: ::core::ffi::c_int,
) {
    let mut c: *const toks_uni_cfg = &raw const (*(*g).u).cfg;
    let mut is_v: ::core::ffi::c_int = (cp == 0x20 as uint32_t
        || cp == 0x2581 as uint32_t) as ::core::ffi::c_int;
    if (*c).ws_split != 0 {
        if cp != 0xffffffff as uint32_t && toks_is_regex_ws(cp) != 0 {
            if (*g).in_word != 0 {
                piece_close(g);
                (*g).in_word = 0 as ::core::ffi::c_int;
            }
            return;
        }
        if (*g).in_word == 0 {
            (*g).in_word = 1 as ::core::ffi::c_int;
            (*g).split_start = 1 as ::core::ffi::c_int;
        }
        if (*c).metaspace != 0 {
            meta_char(g, b, k, src, in_place, is_v);
        } else {
            piece_add(g, b, k, src, in_place);
        }
        return;
    }
    if (*c).metaspace != 0 {
        if (*g).started == 0 {
            (*g).started = 1 as ::core::ffi::c_int;
            (*g).split_start = 1 as ::core::ffi::c_int;
        }
        meta_char(g, b, k, src, in_place, is_v);
        return;
    }
    if is_v != 0 && (*(*g).u).remap as ::core::ffi::c_int != 0 {
        piece_add(
            g,
            &raw const V_BYTE as *const uint8_t,
            1 as uint32_t,
            src,
            (in_place != 0 && k == 1 as uint32_t) as ::core::ffi::c_int,
        );
        return;
    }
    piece_add(g, b, k, src, in_place);
}
unsafe extern "C" fn norm_char(
    mut g: *mut seg,
    mut cp: uint32_t,
    mut b: *const uint8_t,
    mut k: uint32_t,
    mut nlen: uint32_t,
    mut src: uint64_t,
    mut in_place: ::core::ffi::c_int,
) {
    let mut c: *const toks_uni_cfg = &raw const (*(*g).u).cfg;
    if (*c).collapse != 0 {
        if cp == 0x20 as uint32_t {
            if (*g).prev_space != 0 {
                return;
            }
            (*g).prev_space = 1 as ::core::ffi::c_int;
        } else {
            (*g).prev_space = 0 as ::core::ffi::c_int;
        }
    }
    if !(*g).sink.is_null() {
        let mut u: [uint8_t; 4] = [0; 4];
        toks_dput(
            (*g).sink,
            (if cp == 0xffffffff as uint32_t {
                b
            } else {
                &raw mut u as *mut uint8_t as *const uint8_t
            }) as *const ::core::ffi::c_void,
            (if cp == 0xffffffff as uint32_t {
                1 as uint32_t
            } else {
                toks_utf8_put(&raw mut u as *mut uint8_t, cp)
            }) as uint64_t,
        );
        return;
    }
    if (*c).meta_prefix as ::core::ffi::c_int != 0 && (*g).seen == 0 {
        (*g).seen = 1 as ::core::ffi::c_int;
        (*g).npos = (*g).npos.wrapping_add(3 as uint64_t);
        pre_char(
            g,
            0x2581 as uint32_t,
            &raw const V_BYTE as *const uint8_t,
            1 as uint32_t,
            NOSRC as uint64_t,
            0 as ::core::ffi::c_int,
        );
    }
    (*g).seen = 1 as ::core::ffi::c_int;
    if (*c).meta_replace as ::core::ffi::c_int != 0 && cp == 0x20 as uint32_t {
        (*g).npos = (*g).npos.wrapping_add(3 as uint64_t);
        pre_char(g, 0x2581 as uint32_t, b, k, src, in_place);
        return;
    }
    (*g).npos = (*g).npos.wrapping_add(nlen as uint64_t);
    pre_char(g, cp, b, k, src, in_place);
}
unsafe extern "C" fn emit_value(mut g: *mut seg, mut ent: uint32_t) {
    let mut pc: *const toks_pc = &raw const (*(*g).u).pc;
    let mut v: *const uint8_t = (*pc)
        .pool
        .offset((ent >> 12 as ::core::ffi::c_int & 0x3ffff as uint32_t) as isize);
    let mut vl: uint32_t = ent & 0xfff as uint32_t;
    let mut i: uint32_t = 0 as uint32_t;
    while i < vl {
        let mut k: uint32_t = toks_utf8_len(
            v.offset(i as isize),
            vl.wrapping_sub(i) as uint64_t,
        );
        if k == 0 as uint32_t {
            k = 1 as ::core::ffi::c_uint as uint32_t;
        }
        let mut cp: uint32_t = if k == 1 as uint32_t {
            *v.offset(i as isize) as uint32_t
        } else {
            toks_cp_decode(v.offset(i as isize), k)
        };
        if cp == 0x2581 as uint32_t && (*(*g).u).remap as ::core::ffi::c_int != 0 {
            norm_char(
                g,
                cp,
                &raw const V_BYTE as *const uint8_t,
                1 as uint32_t,
                3 as uint32_t,
                NOSRC as uint64_t,
                0 as ::core::ffi::c_int,
            );
        } else {
            norm_char(
                g,
                cp,
                v.offset(i as isize),
                k,
                k,
                NOSRC as uint64_t,
                0 as ::core::ffi::c_int,
            );
        }
        i = i.wrapping_add(k);
    }
}
unsafe extern "C" fn atom(
    mut t: *const uint8_t,
    mut i: uint64_t,
    mut len: uint64_t,
    mut k: *mut uint32_t,
) -> uint32_t {
    let mut l: uint32_t = toks_utf8_len(t.offset(i as isize), len.wrapping_sub(i));
    if l == 0 as uint32_t {
        *k = 1 as ::core::ffi::c_uint as uint32_t;
        return 0xffffffff as uint32_t;
    }
    *k = l;
    return if l == 1 as uint32_t {
        *t.offset(i as isize) as uint32_t
    } else {
        toks_cp_decode(t.offset(i as isize), l)
    };
}
unsafe extern "C" fn ident_char(
    mut g: *mut seg,
    mut cp: uint32_t,
    mut i: uint64_t,
    mut k: uint32_t,
) {
    let mut b: *const uint8_t = (*g).text.offset(i as isize);
    if cp == 0x2581 as uint32_t && (*(*g).u).remap as ::core::ffi::c_int != 0 {
        norm_char(
            g,
            cp,
            &raw const V_BYTE as *const uint8_t,
            1 as uint32_t,
            3 as uint32_t,
            i,
            0 as ::core::ffi::c_int,
        );
        return;
    }
    norm_char(g, cp, b, k, k, i, 1 as ::core::ffi::c_int);
}
unsafe extern "C" fn flush_grapheme(
    mut g: *mut seg,
    mut gs: uint64_t,
    mut ge: uint64_t,
    mut nchars: uint32_t,
) {
    let mut pc: *const toks_pc = &raw const (*(*g).u).pc;
    let mut k1: uint32_t = 0 as uint32_t;
    let mut cp1: uint32_t = atom((*g).text, gs, ge, &raw mut k1);
    if ge.wrapping_sub(gs) < 6 as uint64_t {
        let mut e1: uint32_t = if cp1 != 0xffffffff as uint32_t {
            toks_pc_char(pc, cp1)
        } else {
            0 as uint32_t
        };
        if e1 != 0 as uint32_t {
            emit_value(g, e1);
            return;
        }
        let mut pe: uint64_t = gs.wrapping_add(k1 as uint64_t);
        let mut c: uint32_t = 1 as uint32_t;
        while c < nchars {
            let mut k: uint32_t = 0 as uint32_t;
            atom((*g).text, pe, ge, &raw mut k);
            pe = pe.wrapping_add(k as uint64_t);
            let mut em: uint32_t = toks_pc_multi(
                pc,
                (*g).text.offset(gs as isize),
                pe.wrapping_sub(gs) as uint32_t,
            );
            if em != 0 as uint32_t {
                emit_value(g, em);
                return;
            }
            c = c.wrapping_add(1);
        }
    }
    let mut i: uint64_t = gs;
    while i < ge {
        let mut k_0: uint32_t = 0 as uint32_t;
        let mut cp: uint32_t = atom((*g).text, i, ge, &raw mut k_0);
        let mut e: uint32_t = if cp != 0xffffffff as uint32_t {
            toks_pc_char(pc, cp)
        } else {
            0 as uint32_t
        };
        if e != 0 as uint32_t {
            emit_value(g, e);
        } else {
            ident_char(g, cp, i, k_0);
        }
        i = i.wrapping_add(k_0 as uint64_t);
    }
}
unsafe extern "C" fn atom_class(mut cp: uint32_t) -> uint8_t {
    return (if cp == 0xffffffff as uint32_t {
        TOKS_GC_CONTROL as ::core::ffi::c_int as uint8_t as ::core::ffi::c_int
    } else {
        toks_gc_class(cp) as ::core::ffi::c_int
    }) as uint8_t;
}
unsafe extern "C" fn strip_span(
    mut c: *const toks_uni_cfg,
    mut gap_start: ::core::ffi::c_int,
    mut t: *const uint8_t,
    mut a: *mut uint64_t,
    mut b: *mut uint64_t,
) {
    if (*c).strip_left as ::core::ffi::c_int != 0 && gap_start != 0 {
        while *a < *b {
            let mut k: uint32_t = 0 as uint32_t;
            let mut cp: uint32_t = atom(t, *a, *b, &raw mut k);
            if cp == 0xffffffff as uint32_t || toks_is_regex_ws(cp) == 0 {
                break;
            }
            *a = (*a).wrapping_add(k as uint64_t);
        }
    }
    if (*c).strip_right != 0 {
        while *b > *a {
            let mut s: uint64_t = (*b).wrapping_sub(1 as uint64_t);
            let mut back: uint32_t = 0 as uint32_t;
            while s > *a
                && *t.offset(s as isize) as ::core::ffi::c_uint
                    & 0xc0 as ::core::ffi::c_uint == 0x80 as ::core::ffi::c_uint
                && back < 3 as uint32_t
            {
                s = s.wrapping_sub(1);
                back = back.wrapping_add(1);
            }
            let mut k_0: uint32_t = 0 as uint32_t;
            let mut cp_0: uint32_t = atom(t, s, *b, &raw mut k_0);
            if cp_0 == 0xffffffff as uint32_t || s.wrapping_add(k_0 as uint64_t) != *b
                || toks_is_regex_ws(cp_0) == 0
            {
                break;
            }
            *b = s;
        }
    }
}
#[inline]
unsafe extern "C" fn simple(
    mut u: *const toks_uni,
    mut cp: uint32_t,
) -> ::core::ffi::c_int {
    if cp < 0x80 as uint32_t {
        return (*u).simple[cp as usize] as ::core::ffi::c_int;
    }
    if cp == 0xffffffff as uint32_t || cp == 0x2581 as uint32_t
        || (*u).cfg.ws_split as ::core::ffi::c_int != 0 && toks_is_regex_ws(cp) != 0
    {
        return 0 as ::core::ffi::c_int;
    }
    return ((*u).cfg.has_charsmap == 0
        || toks_pc_char(&raw const (*u).pc, cp) == 0 as uint32_t
            && toks_gc_class(cp) as ::core::ffi::c_uint == 0 as ::core::ffi::c_uint)
        as ::core::ffi::c_int;
}
unsafe extern "C" fn bulk(mut g: *mut seg, mut s: uint64_t, mut e: uint64_t) {
    if e <= s {
        return;
    }
    if !(*g).sink.is_null() {
        toks_dput(
            (*g).sink,
            (*g).text.offset(s as isize) as *const ::core::ffi::c_void,
            e.wrapping_sub(s),
        );
        return;
    }
    (*g).npos = (*g).npos.wrapping_add(e.wrapping_sub(s));
    if (*g).p_mat == 0 && (*g).p_e == s && (*g).p_e != (*g).p_s {
        (*g).p_e = e;
        (*g).p_nend = (*g).npos;
        return;
    }
    while s < e {
        let mut n: uint32_t = if e.wrapping_sub(s) > 0xffff as uint64_t {
            0xffff as uint32_t
        } else {
            e.wrapping_sub(s) as uint32_t
        };
        piece_add(g, (*g).text.offset(s as isize), n, s, 1 as ::core::ffi::c_int);
        s = s.wrapping_add(n as uint64_t);
    }
}
unsafe extern "C" fn ascii_run(
    mut g: *mut seg,
    mut i: uint64_t,
    mut b: uint64_t,
    mut cm: ::core::ffi::c_int,
) -> uint64_t {
    let mut u: *const toks_uni = (*g).u;
    let mut t: *const uint8_t = (*g).text;
    while i < b {
        let mut c: uint32_t = *t.offset(i as isize) as uint32_t;
        if c >= 0x80 as uint32_t
            || cm != 0
                && (c == 0xd as uint32_t
                    || i.wrapping_add(1 as uint64_t) < b
                        && *t.offset(i.wrapping_add(1 as uint64_t) as isize)
                            as ::core::ffi::c_uint >= 0x80 as ::core::ffi::c_uint)
        {
            break;
        }
        if (*u).simple[c as usize] != 0 {
            let mut j: uint64_t = i.wrapping_add(1 as uint64_t);
            while j < b
                && (*t.offset(j as isize) as ::core::ffi::c_uint)
                    < 0x80 as ::core::ffi::c_uint
                && (*u).simple[*t.offset(j as isize) as usize] as ::core::ffi::c_int != 0
            {
                j = j.wrapping_add(1);
            }
            let mut e: uint64_t = if cm != 0 && j < b
                && *t.offset(j as isize) as ::core::ffi::c_uint
                    >= 0x80 as ::core::ffi::c_uint
            {
                j.wrapping_sub(1 as uint64_t)
            } else {
                j
            };
            ident_char(g, c, i, 1 as uint32_t);
            bulk(g, i.wrapping_add(1 as uint64_t), e);
            i = e;
        } else {
            let mut ent: uint32_t = if cm != 0 {
                (*u).aent[c as usize]
            } else {
                0 as uint32_t
            };
            if ent != 0 as uint32_t {
                emit_value(g, ent);
            } else {
                ident_char(g, c, i, 1 as uint32_t);
            }
            i = i.wrapping_add(1);
        }
    }
    return i;
}
unsafe extern "C" fn ascii_fast(
    mut g: *mut seg,
    mut i: uint64_t,
    mut b: uint64_t,
) -> uint64_t {
    let mut u: *const toks_uni = (*g).u;
    let mut t: *const uint8_t = (*g).text;
    let collapse: ::core::ffi::c_int = (*u).cfg.collapse as ::core::ffi::c_int;
    let ws: ::core::ffi::c_int = (*u).cfg.ws_split as ::core::ffi::c_int;
    while i < b {
        let mut ch: uint32_t = *t.offset(i as isize) as uint32_t;
        if ch >= 0x80 as uint32_t {
            break;
        }
        let mut k: uint32_t = (*u).acls[ch as usize] as uint32_t;
        if k == 1 as uint32_t {
            let mut j: uint64_t = i.wrapping_add(1 as uint64_t);
            while j < b
                && (*t.offset(j as isize) as ::core::ffi::c_uint)
                    < 0x80 as ::core::ffi::c_uint
                && (*u).simple[*t.offset(j as isize) as usize] as ::core::ffi::c_int != 0
            {
                j = j.wrapping_add(1);
            }
            let mut e: uint64_t = if j < b
                && *t.offset(j as isize) as ::core::ffi::c_uint
                    >= 0x80 as ::core::ffi::c_uint
            {
                j.wrapping_sub(1 as uint64_t)
            } else {
                j
            };
            if e == i {
                break;
            }
            (*g).prev_space = 0 as ::core::ffi::c_int;
            (*g).seen = 1 as ::core::ffi::c_int;
            (*g).npos = (*g).npos.wrapping_add(e.wrapping_sub(i));
            if ws != 0 {
                if (*g).in_word == 0 {
                    (*g).in_word = 1 as ::core::ffi::c_int;
                    (*g).split_start = 1 as ::core::ffi::c_int;
                }
            } else if (*g).started == 0 {
                (*g).started = 1 as ::core::ffi::c_int;
                (*g).split_start = 1 as ::core::ffi::c_int;
            }
            if (*g).split_start != 0 {
                (*g).split_start = 0 as ::core::ffi::c_int;
                piece_open(g, 1 as ::core::ffi::c_int);
            }
            if (*g).p_open != 0 && (*g).p_mat == 0 && (*g).p_e == (*g).p_s {
                (*g).p_s = i;
                (*g).p_e = e;
                (*g).p_nend = (*g).npos;
            } else if (*g).p_open != 0 && (*g).p_mat == 0 && (*g).p_e == i {
                (*g).p_e = e;
                (*g).p_nend = (*g).npos;
            } else {
                (*g).npos = (*g).npos.wrapping_sub(e.wrapping_sub(i));
                bulk(g, i, e);
            }
            i = e;
        } else {
            if k != 2 as uint32_t {
                break;
            }
            let mut n: uint64_t = (if ch == 0xd as uint32_t
                && i.wrapping_add(1 as uint64_t) < b
                && *t.offset(i.wrapping_add(1 as uint64_t) as isize)
                    as ::core::ffi::c_uint == 0xa as ::core::ffi::c_uint
            {
                2 as ::core::ffi::c_uint
            } else {
                1 as ::core::ffi::c_uint
            }) as uint64_t;
            if i.wrapping_add(n) < b
                && *t.offset(i.wrapping_add(n) as isize) as ::core::ffi::c_uint
                    >= 0x80 as ::core::ffi::c_uint
            {
                break;
            }
            i = i.wrapping_add(n);
            if collapse != 0 && (*g).prev_space != 0 {
                continue;
            }
            (*g).prev_space = collapse;
            (*g).seen = 1 as ::core::ffi::c_int;
            (*g).npos = (*g).npos.wrapping_add(1 as uint64_t);
            if ws != 0 {
                if (*g).in_word != 0 {
                    piece_close(g);
                    (*g).in_word = 0 as ::core::ffi::c_int;
                }
            } else {
                if (*g).started == 0 {
                    (*g).started = 1 as ::core::ffi::c_int;
                }
                (*g).split_start = 0 as ::core::ffi::c_int;
                piece_close(g);
                piece_open(g, 1 as ::core::ffi::c_int);
                (*g).p_nend = (*g).npos;
            }
        }
    }
    return i;
}
unsafe extern "C" fn walk(
    mut g: *mut seg,
    mut len: uint64_t,
    mut gap_start: ::core::ffi::c_int,
) {
    let mut u: *const toks_uni = (*g).u;
    let mut gs: toks_gc_state = toks_gc_state {
        prev: 0,
        ri_odd: 0,
        ep_run: 0,
        ep_zwj: 0,
        incb_cons: 0,
        incb_link: 0,
        rsv: [0; 2],
    };
    toks_gc_init(&raw mut gs);
    let mut a: uint64_t = 0 as uint64_t;
    let mut b: uint64_t = len;
    strip_span(&raw const (*u).cfg, gap_start, (*g).text, &raw mut a, &raw mut b);
    let mut t: *const uint8_t = (*g).text;
    let mut cm: ::core::ffi::c_int = (*u).cfg.has_charsmap as ::core::ffi::c_int;
    let mut i: uint64_t = a;
    let mut g0: uint64_t = a;
    let mut nch: uint32_t = 0 as uint32_t;
    while i < b {
        if nch == 0 as uint32_t
            && (*t.offset(i as isize) as ::core::ffi::c_uint)
                < 0x80 as ::core::ffi::c_uint
        {
            let mut i0: uint64_t = i;
            i = if (*u).fast as ::core::ffi::c_int != 0 && (*g).sink.is_null()
                && (*(*g).c).pieces == 0
            {
                ascii_fast(g, i, b)
            } else {
                i
            };
            i = ascii_run(g, i, b, cm);
            if i != i0 {
                toks_gc_init(&raw mut gs);
                toks_gc_step(
                    &raw mut gs,
                    toks_gc_class(
                        *t.offset(i.wrapping_sub(1 as uint64_t) as isize) as uint32_t,
                    ),
                );
                g0 = i;
                if i >= b {
                    break;
                }
            }
        }
        let mut k: uint32_t = 0 as uint32_t;
        let mut cp: uint32_t = atom(t, i, b, &raw mut k);
        if cm != 0 && toks_gc_step(&raw mut gs, atom_class(cp)) != 0
            && nch > 0 as uint32_t
        {
            flush_grapheme(g, g0, i, nch);
            g0 = i;
            nch = 0 as ::core::ffi::c_uint as uint32_t;
        }
        if nch == 0 as uint32_t && simple(u, cp) != 0 {
            let mut j: uint64_t = i.wrapping_add(k as uint64_t);
            let mut last: uint64_t = i;
            while j < b {
                let mut kj: uint32_t = 0 as uint32_t;
                if simple(u, atom(t, j, b, &raw mut kj)) == 0 {
                    break;
                }
                last = j;
                j = j.wrapping_add(kj as uint64_t);
            }
            if cm == 0 {
                last = j;
            }
            if last > i {
                ident_char(g, cp, i, k);
                bulk(g, i.wrapping_add(k as uint64_t), last);
                i = last;
                g0 = i;
                continue;
            }
        }
        if cm == 0 {
            ident_char(g, cp, i, k);
        } else {
            nch = nch.wrapping_add(1);
        }
        i = i.wrapping_add(k as uint64_t);
    }
    if cm != 0 && nch > 0 as uint32_t {
        flush_grapheme(g, g0, b, nch);
    }
}
#[no_mangle]
pub unsafe extern "C" fn toks_uni_encode_segment(
    mut u: *const toks_uni,
    mut text: *const uint8_t,
    mut len: uint64_t,
    mut pre_normalized: ::core::ffi::c_int,
    mut gap_start: ::core::ffi::c_int,
    mut c: *mut toks_uni_call,
) -> int64_t {
    let mut one: uint64_t = ((if pre_normalized != 0 {
        1 as uint32_t
    } else {
        (*u).work_x
    }) as uint64_t)
        .wrapping_mul(len)
        .wrapping_add(3 as uint64_t)
        .wrapping_add(63 as uint64_t) & !(63 as ::core::ffi::c_uint as uint64_t);
    if (*c).work_bytes < (2 as uint64_t).wrapping_mul(one) {
        return TOKS_E_SCRATCH as int64_t;
    }
    let mut g: seg = seg {
        u: ::core::ptr::null::<toks_uni>(),
        text: ::core::ptr::null::<uint8_t>(),
        tlen: 0,
        c: ::core::ptr::null_mut::<toks_uni_call>(),
        mbuf: ::core::ptr::null_mut::<uint8_t>(),
        mcap: 0,
        delta: ::core::ptr::null_mut::<uint8_t>(),
        npos: 0,
        p_nend: 0,
        seen: 0,
        prev_space: 0,
        in_word: 0,
        split_start: 0,
        started: 0,
        p_open: 0,
        p_virt: 0,
        p_mat: 0,
        p_s: 0,
        p_e: 0,
        p_ml: 0,
        err: 0,
        sink: ::core::ptr::null_mut::<toks_dsink>(),
        qn: 0,
        q: [C2RustUnnamed_3 {
            b: ::core::ptr::null::<uint8_t>(),
            l: 0,
            virt: 0,
            h: 0,
            k: bpe_key { lo: 0, hi: 0 },
        }; 16],
    };
    memset(
        &raw mut g as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<seg>() as size_t,
    );
    g.u = u;
    g.text = text;
    g.tlen = len;
    g.c = c;
    g.mbuf = (*c).work;
    g.mcap = one;
    g.delta = (*c).work.offset(one as isize);
    g.seen = (gap_start == 0) as ::core::ffi::c_int;
    if pre_normalized != 0 {
        let mut i: uint64_t = 0 as uint64_t;
        while i < len {
            let mut k: uint32_t = 0 as uint32_t;
            let mut cp: uint32_t = atom(text, i, len, &raw mut k);
            g.npos = g.npos.wrapping_add(k as uint64_t);
            if cp == 0x2581 as uint32_t && (*u).remap as ::core::ffi::c_int != 0 {
                pre_char(
                    &raw mut g,
                    cp,
                    &raw const V_BYTE as *const uint8_t,
                    1 as uint32_t,
                    i,
                    0 as ::core::ffi::c_int,
                );
            } else {
                pre_char(
                    &raw mut g,
                    cp,
                    text.offset(i as isize),
                    k,
                    i,
                    1 as ::core::ffi::c_int,
                );
            }
            i = i.wrapping_add(k as uint64_t);
        }
    } else {
        walk(&raw mut g, len, gap_start);
    }
    piece_close(&raw mut g);
    q_flush(&raw mut g);
    return if g.err != 0 { TOKS_E_SCRATCH as int64_t } else { g.npos as int64_t };
}
#[no_mangle]
pub unsafe extern "C" fn toks_uni_normalize(
    mut u: *const toks_uni,
    mut text: *const uint8_t,
    mut len: uint64_t,
    mut gap_start: ::core::ffi::c_int,
    mut dst: *mut uint8_t,
    mut cap: uint64_t,
) -> int64_t {
    let mut d: toks_dsink = toks_dsink {
        out: dst,
        cap: cap,
        n: 0 as uint64_t,
    };
    let mut g: seg = seg {
        u: ::core::ptr::null::<toks_uni>(),
        text: ::core::ptr::null::<uint8_t>(),
        tlen: 0,
        c: ::core::ptr::null_mut::<toks_uni_call>(),
        mbuf: ::core::ptr::null_mut::<uint8_t>(),
        mcap: 0,
        delta: ::core::ptr::null_mut::<uint8_t>(),
        npos: 0,
        p_nend: 0,
        seen: 0,
        prev_space: 0,
        in_word: 0,
        split_start: 0,
        started: 0,
        p_open: 0,
        p_virt: 0,
        p_mat: 0,
        p_s: 0,
        p_e: 0,
        p_ml: 0,
        err: 0,
        sink: ::core::ptr::null_mut::<toks_dsink>(),
        qn: 0,
        q: [C2RustUnnamed_3 {
            b: ::core::ptr::null::<uint8_t>(),
            l: 0,
            virt: 0,
            h: 0,
            k: bpe_key { lo: 0, hi: 0 },
        }; 16],
    };
    memset(
        &raw mut g as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<seg>() as size_t,
    );
    g.u = u;
    g.text = text;
    g.sink = &raw mut d;
    walk(&raw mut g, len, gap_start);
    return if d.n > cap {
        -(1 as ::core::ffi::c_int) as int64_t
    } else {
        d.n as int64_t
    };
}
