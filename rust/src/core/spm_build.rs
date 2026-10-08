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
    fn toks_plat_alloc(n: uint64_t) -> *mut ::core::ffi::c_void;
    fn toks_plat_free(p: *mut ::core::ffi::c_void, n: uint64_t);
    fn toks_plat_arena(n: uint64_t) -> *mut uint8_t;
    static TOKS_CRC32C_TAB: [uint32_t; 256];
    fn toks_spm_model(
        t: *const toks_tables,
        s: *const toks_spm,
        p: *const uint8_t,
        len: uint64_t,
        out: *mut uint32_t,
        work: *mut uint8_t,
    ) -> uint64_t;
    fn toks_merge_slots(
        t: *mut toks_tables,
        slots: *mut uint64_t,
        pf: *mut uint64_t,
        nb: uint64_t,
        ml: *const uint32_t,
        mr: *const uint32_t,
        n: uint32_t,
        reach: *const uint32_t,
    ) -> uint64_t;
}
pub type size_t = usize;
pub type __uint8_t = u8;
pub type __uint16_t = u16;
pub type __uint32_t = u32;
pub type __int64_t = i64;
pub type __uint64_t = u64;
pub type int64_t = __int64_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
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
pub struct toks_spm {
    pub ascii: [uint32_t; 128],
    pub stage1: *const uint16_t,
    pub stage2: *const uint32_t,
    pub pairs: *const uint64_t,
    pub pairs_mask: uint64_t,
    pub n_pairs: uint64_t,
    pub holes: *const uint32_t,
    pub cut: [uint8_t; 2081],
    pub unk_id: uint32_t,
    pub sflags: uint32_t,
    pub pfx_mode: uint32_t,
    pub pfx_entry: uint32_t,
    pub id_repl: uint32_t,
    pub ms_split: uint32_t,
    pub n_blocks: uint32_t,
    pub n_dec: uint32_t,
    pub has_decoder: uint32_t,
    pub rsv: uint32_t,
    pub n_dropped: uint64_t,
    pub dec: [toks_spm_op; 8],
    pub cut_ab: [uint32_t; 1024],
    pub cut_ab8: *const uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct toks_tables {
    pub magic: uint32_t,
    pub version: uint32_t,
    pub algo: uint32_t,
    pub tmpl: uint32_t,
    pub tmpl_params: uint32_t,
    pub flags: uint32_t,
    pub n_ids: uint32_t,
    pub n_merges: uint32_t,
    pub cls_ascii: *const uint8_t,
    pub cls_stage1: *const uint16_t,
    pub cls_stage2: *const uint8_t,
    pub cls_nblocks: uint64_t,
    pub byte2id: *const uint32_t,
    pub bytepair: *const uint32_t,
    pub merge_slots: *const uint64_t,
    pub merge_mask: uint64_t,
    pub merge_shift: uint64_t,
    pub merge_maxprobe: uint64_t,
    pub rank2id: *const uint32_t,
    pub premerge: *const uint8_t,
    pub words: *const uint8_t,
    pub words_mask: uint64_t,
    pub vhash: *const uint64_t,
    pub vhash_mask: uint64_t,
    pub tok_off: *const uint32_t,
    pub tok_bytes: *const uint8_t,
    pub add_shufti: *const uint8_t,
    pub add_index: *const uint64_t,
    pub add_single: *const uint32_t,
    pub add_cand: *const uint32_t,
    pub add_entries: *const toks_added_entry,
    pub add_bytes: *const uint8_t,
    pub add_n: uint64_t,
    pub add_phases: uint64_t,
    pub apm: *const uint8_t,
    pub pairf: *const uint64_t,
    pub rsv: [uint64_t; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct toks_added_entry {
    pub off: uint32_t,
    pub len: uint16_t,
    pub flags: uint8_t,
    pub phase: uint8_t,
    pub id: uint32_t,
    pub rsv: uint32_t,
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
pub struct toks_spm_config {
    pub vocab: *const *const uint8_t,
    pub vocab_len: *const uint32_t,
    pub n_ids: uint32_t,
    pub n_strings: uint32_t,
    pub vslot: *const uint32_t,
    pub vmask: uint64_t,
    pub m_left: *const uint32_t,
    pub m_right: *const uint32_t,
    pub m_out: *const uint32_t,
    pub n_merges: uint32_t,
    pub unk_id: uint32_t,
    pub sflags: uint32_t,
    pub text: toks_spm_text,
    pub dec: [toks_spm_op; 8],
    pub n_dec: uint32_t,
    pub has_decoder: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct toks_spm_text {
    pub norm: [toks_spm_op; 8],
    pub n_norm: uint32_t,
    pub metaspace: uint32_t,
    pub ms_scheme: uint32_t,
    pub ms_split: uint32_t,
    pub ms_repl: toks_spm_str,
}
pub type C2RustUnnamed = ::core::ffi::c_uint;
pub const TOKS_SPM_N_REPLACE: C2RustUnnamed = 2;
pub const TOKS_SPM_N_PREPEND: C2RustUnnamed = 1;
pub type C2RustUnnamed_0 = ::core::ffi::c_uint;
pub const TOKS_SPM_PS_NEVER: C2RustUnnamed_0 = 2;
pub const TOKS_SPM_PS_FIRST: C2RustUnnamed_0 = 1;
pub const TOKS_SPM_PS_ALWAYS: C2RustUnnamed_0 = 0;
pub type C2RustUnnamed_1 = ::core::ffi::c_uint;
pub const TOKS_SPM_PFX_FIRST: C2RustUnnamed_1 = 3;
pub const TOKS_SPM_PFX_ALWAYS: C2RustUnnamed_1 = 2;
pub const TOKS_SPM_PFX_GAP: C2RustUnnamed_1 = 1;
pub const TOKS_SPM_PFX_NONE: C2RustUnnamed_1 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bpe_key {
    pub lo: uint64_t,
    pub hi: uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fold {
    pub src: [uint32_t; 9],
    pub dst: [uint32_t; 9],
    pub n: uint32_t,
    pub pfx: int64_t,
    pub repl: int64_t,
    pub mode: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bpe_mt {
    pub slots: *const uint64_t,
    pub shift: uint64_t,
    pub mask: uint64_t,
    pub maxprobe: uint64_t,
    pub r2i: *const uint32_t,
    pub pf: *const uint64_t,
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
pub const UINT32_MAX: ::core::ffi::c_uint = 4294967295 as ::core::ffi::c_uint;
pub const UINT64_MAX: ::core::ffi::c_ulong = 18446744073709551615
    as ::core::ffi::c_ulong;
pub const TOKS_E_FORMAT: ::core::ffi::c_int = -(2 as ::core::ffi::c_int);
pub const TOKS_E_UNSUPPORTED: ::core::ffi::c_int = -(3 as ::core::ffi::c_int);
pub const TOKS_E_LIMIT: ::core::ffi::c_int = -(9 as ::core::ffi::c_int);
pub const TOKS_E_NOMEM: ::core::ffi::c_int = -(11 as ::core::ffi::c_int);
pub const TOKS_MAX_IDS: ::core::ffi::c_uint = ((1 as ::core::ffi::c_uint)
    << 21 as ::core::ffi::c_int)
    .wrapping_sub(1 as ::core::ffi::c_uint);
pub const TOKS_ID_MASK: ::core::ffi::c_uint = 0x1fffff as ::core::ffi::c_uint;
pub const TOKS_TF_IGNORE_MERGES: ::core::ffi::c_uint = 0x1 as ::core::ffi::c_uint;
pub const TOKS_TF_IDS_AS_RANK: ::core::ffi::c_uint = 0x2 as ::core::ffi::c_uint;
pub const TOKS_VAL_COUNT_SHIFT: ::core::ffi::c_int = 29 as ::core::ffi::c_int;
pub const TOKS_BUCKET: ::core::ffi::c_int = 64 as ::core::ffi::c_int;
pub const TOKS_FIB64: ::core::ffi::c_ulonglong = 0x9e3779b97f4a7c15
    as ::core::ffi::c_ulonglong;
pub const TOKS_PRIO_BITS: ::core::ffi::c_int = 22 as ::core::ffi::c_int;
pub const TOKS_PRIO_NONE: ::core::ffi::c_uint = 0xffffffff as ::core::ffi::c_uint;
pub const TOKS_APM_PAIRS: ::core::ffi::c_int = 256 as ::core::ffi::c_int;
pub const TOKS_APM_BYTES: ::core::ffi::c_int = 256 as ::core::ffi::c_int
    + 16384 as ::core::ffi::c_int * 16 as ::core::ffi::c_int;
pub const TOKS_VSEED: ::core::ffi::c_uint = 0x85ebca6b as ::core::ffi::c_uint;
pub const TOKS_X_BYTE2ID: toks_ext = toks_ext {
    pad: 0 as uint32_t,
    align: 4 as uint32_t,
};
pub const TOKS_X_MERGE_SLOTS: toks_ext = toks_ext {
    pad: 0 as uint32_t,
    align: 64 as uint32_t,
};
pub const TOKS_X_PAIRF: toks_ext = toks_ext {
    pad: 0 as uint32_t,
    align: 8 as uint32_t,
};
pub const TOKS_X_RANK2ID: toks_ext = toks_ext {
    pad: 0 as uint32_t,
    align: 4 as uint32_t,
};
pub const TOKS_X_VHASH: toks_ext = toks_ext {
    pad: 0 as uint32_t,
    align: 8 as uint32_t,
};
pub const TOKS_X_WORDS: toks_ext = toks_ext {
    pad: 0 as uint32_t,
    align: 64 as uint32_t,
};
pub const TOKS_X_APM: toks_ext = toks_ext {
    pad: 0 as uint32_t,
    align: 4 as uint32_t,
};
pub const TOKS_X_SPM: toks_ext = toks_ext {
    pad: 0 as uint32_t,
    align: 8 as uint32_t,
};
pub const TOKS_X_SPM_STAGE1: toks_ext = toks_ext {
    pad: 0 as uint32_t,
    align: 2 as uint32_t,
};
pub const TOKS_X_SPM_STAGE2: toks_ext = toks_ext {
    pad: 0 as uint32_t,
    align: 4 as uint32_t,
};
pub const TOKS_X_SPM_PAIRS: toks_ext = toks_ext {
    pad: 0 as uint32_t,
    align: 8 as uint32_t,
};
pub const TOKS_X_SPM_HOLES: toks_ext = toks_ext {
    pad: 0 as uint32_t,
    align: 4 as uint32_t,
};
pub const TOKS_X_SPM_AB8: toks_ext = toks_ext {
    pad: 0 as uint32_t,
    align: 1 as uint32_t,
};
#[inline]
unsafe extern "C" fn toks_ld32(mut p: *const ::core::ffi::c_void) -> uint32_t {
    let mut v: uint32_t = 0;
    memcpy(&raw mut v as *mut ::core::ffi::c_void, p, 4 as size_t);
    return v;
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
pub const TOKS_SPM_NONE: ::core::ffi::c_uint = 0xffffffff as ::core::ffi::c_uint;
pub const TOKS_SPM_BYTE_FALLBACK: ::core::ffi::c_uint = 0x1 as ::core::ffi::c_uint;
pub const TOKS_SPM_UNK_ERROR: ::core::ffi::c_uint = 0x4 as ::core::ffi::c_uint;
pub const TOKS_SPM_IGNORE_MERGES: ::core::ffi::c_uint = 0x8 as ::core::ffi::c_uint;
pub const TOKS_SPM_E_ID: ::core::ffi::c_uint = 0x1fffff as ::core::ffi::c_uint;
pub const TOKS_SPM_E_NOID: ::core::ffi::c_uint = 0x1fffff as ::core::ffi::c_uint;
pub const TOKS_SPM_E_SI_SHIFT: ::core::ffi::c_uint = 21 as ::core::ffi::c_uint;
pub const TOKS_SPM_E_SI_NONE: ::core::ffi::c_uint = 255 as ::core::ffi::c_uint;
pub const TOKS_SPM_E_PAIRED: ::core::ffi::c_uint = 0x40000000 as ::core::ffi::c_uint;
pub const TOKS_SPM_E_PLAIN: ::core::ffi::c_uint = 0x80000000 as ::core::ffi::c_uint;
pub const TOKS_SPM_SMALL: ::core::ffi::c_uint = 129 as ::core::ffi::c_uint;
#[inline]
unsafe extern "C" fn toks_spm_spanned(
    mut s: *const toks_spm,
    mut key: uint64_t,
) -> ::core::ffi::c_int {
    let mut mask: uint64_t = (*s).pairs_mask;
    let mut i: uint64_t = ((key as ::core::ffi::c_ulonglong).wrapping_mul(TOKS_FIB64)
        >> 32 as ::core::ffi::c_int & mask as ::core::ffi::c_ulonglong) as uint64_t;
    let mut k: uint64_t = 0 as uint64_t;
    while k <= mask {
        let mut v: uint64_t = *(*s).pairs.offset(i as isize);
        if v == key || v == UINT64_MAX as uint64_t {
            return (v == key) as ::core::ffi::c_int;
        }
        i = i.wrapping_add(1 as uint64_t) & mask;
        k = k.wrapping_add(1);
    }
    return 0 as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn toks_spm_cut_between(
    mut s: *const toks_spm,
    mut a: uint32_t,
    mut b: uint32_t,
) -> ::core::ffi::c_int {
    let mut ia: uint32_t = a & TOKS_SPM_E_ID as uint32_t;
    let mut ib: uint32_t = b & TOKS_SPM_E_ID as uint32_t;
    if ia == TOKS_SPM_E_NOID as uint32_t || ib == TOKS_SPM_E_NOID as uint32_t {
        return 0 as ::core::ffi::c_int;
    }
    let mut xa: uint32_t = a >> TOKS_SPM_E_SI_SHIFT & 0xff as uint32_t;
    let mut xb: uint32_t = b >> TOKS_SPM_E_SI_SHIFT & 0xff as uint32_t;
    if xa != TOKS_SPM_E_SI_NONE as uint32_t && xb != TOKS_SPM_E_SI_NONE as uint32_t {
        let mut bit: uint32_t = xa
            .wrapping_mul(TOKS_SPM_SMALL as uint32_t)
            .wrapping_add(xb);
        return (((*s).cut[(bit >> 3 as ::core::ffi::c_int) as usize]
            as ::core::ffi::c_int >> (bit & 7 as uint32_t)) as ::core::ffi::c_uint
            & 1 as ::core::ffi::c_uint) as ::core::ffi::c_int;
    }
    if a & TOKS_SPM_E_PAIRED as uint32_t == 0 {
        return 1 as ::core::ffi::c_int;
    }
    return (toks_spm_spanned(
        s,
        (ia as uint64_t) << 21 as ::core::ffi::c_int | ib as uint64_t,
    ) == 0) as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn toks_spm_whash(mut lo: uint64_t, mut hi: uint64_t) -> uint32_t {
    return ((lo as ::core::ffi::c_ulonglong
        ^ (hi as ::core::ffi::c_ulonglong).wrapping_mul(TOKS_FIB64))
        .wrapping_mul(0xd6e8feb86659fd93 as ::core::ffi::c_ulonglong)
        >> 32 as ::core::ffi::c_int) as uint32_t;
}
#[inline]
unsafe extern "C" fn bpe_pow2(mut n: uint64_t) -> uint64_t {
    let mut p: uint64_t = 1 as uint64_t;
    while p < n {
        p <<= 1 as ::core::ffi::c_int;
    }
    return p;
}
pub const BPE_PRIO_MASK: ::core::ffi::c_ulong = ((1 as ::core::ffi::c_ulong)
    << TOKS_PRIO_BITS)
    .wrapping_sub(1 as ::core::ffi::c_ulong);
pub const BPE_EMPTY_SLOT: ::core::ffi::c_ulong = UINT64_MAX;
#[inline]
unsafe extern "C" fn bpe_pair_key(mut left: uint32_t, mut right: uint32_t) -> uint64_t {
    return (left as uint64_t) << 21 as ::core::ffi::c_int | right as uint64_t;
}
#[inline]
unsafe extern "C" fn bpe_mt_of(mut t: *const toks_tables) -> bpe_mt {
    let mut mt: bpe_mt = bpe_mt {
        slots: (*t).merge_slots,
        shift: (*t).merge_shift,
        mask: (*t).merge_mask,
        maxprobe: (*t).merge_maxprobe,
        r2i: if (*t).flags & TOKS_TF_IDS_AS_RANK as uint32_t != 0 as uint32_t {
            ::core::ptr::null::<uint32_t>()
        } else {
            (*t).rank2id
        },
        pf: (*t).pairf,
    };
    return mt;
}
#[inline]
unsafe extern "C" fn bpe_pf_bits(
    mut key: uint64_t,
    mut shift: uint64_t,
    mut w: *mut uint64_t,
) -> uint64_t {
    let mut h: uint64_t = (key as ::core::ffi::c_ulonglong).wrapping_mul(TOKS_FIB64)
        as uint64_t;
    *w = h >> shift >> 1 as ::core::ffi::c_int;
    return (1 as uint64_t) << (h >> 20 as ::core::ffi::c_int & 63 as uint64_t)
        | (1 as uint64_t) << (h >> 26 as ::core::ffi::c_int & 63 as uint64_t);
}
#[inline]
unsafe extern "C" fn bpe_mt_find(mut mt: *const bpe_mt, mut key: uint64_t) -> uint32_t {
    let mut b: uint64_t = ((key as ::core::ffi::c_ulonglong).wrapping_mul(TOKS_FIB64)
        >> (*mt).shift & (*mt).mask as ::core::ffi::c_ulonglong) as uint64_t;
    let mut w: uint64_t = 0 as uint64_t;
    let mut m: uint64_t = if !(*mt).pf.is_null() {
        bpe_pf_bits(key, (*mt).shift, &raw mut w)
    } else {
        0 as uint64_t
    };
    if (if !(*mt).pf.is_null() { *(*mt).pf.offset(w as isize) & m } else { m }) != m {
        return TOKS_PRIO_NONE as uint32_t;
    }
    let mut i: uint64_t = 0 as uint64_t;
    while i < (*mt).maxprobe {
        let mut s: *const uint64_t = (*mt)
            .slots
            .offset(b.wrapping_mul(8 as uint64_t) as isize);
        let mut j: uint32_t = 0 as uint32_t;
        while j < 8 as uint32_t {
            if *s.offset(j as isize) == BPE_EMPTY_SLOT as uint64_t {
                return TOKS_PRIO_NONE as uint32_t;
            }
            if *s.offset(j as isize) >> TOKS_PRIO_BITS == key {
                return (*s.offset(j as isize) & BPE_PRIO_MASK as uint64_t) as uint32_t;
            }
            j = j.wrapping_add(1);
        }
        b = b.wrapping_add(1 as uint64_t) & (*mt).mask;
        i = i.wrapping_add(1);
    }
    return TOKS_PRIO_NONE as uint32_t;
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
unsafe extern "C" fn bpe_vhash_h(mut p: *const uint8_t, mut len: uint64_t) -> uint32_t {
    let mut h: uint32_t = TOKS_VSEED as uint32_t ^ len as uint32_t;
    let mut i: uint64_t = 0 as uint64_t;
    while len.wrapping_sub(i) >= 8 as uint64_t {
        h = toks_crc32c_u64(h, bpe_load_le(p.offset(i as isize), 8 as uint64_t));
        i = i.wrapping_add(8 as uint64_t);
    }
    if i < len {
        h = toks_crc32c_u64(h, bpe_load_le(p.offset(i as isize), len.wrapping_sub(i)));
    }
    return h;
}
#[inline]
unsafe extern "C" fn bpe_apm_class(mut b: uint32_t) -> uint32_t {
    let mut l: uint32_t = (b | 0x20 as uint32_t).wrapping_sub(0x61 as uint32_t);
    return if l < 26 as uint32_t {
        l
    } else if b == 0x20 as uint32_t {
        26 as uint32_t
    } else if b.wrapping_sub(0x30 as uint32_t) < 10 as uint32_t {
        27 as uint32_t
    } else if b < 0x80 as uint32_t {
        28 as uint32_t
    } else if b < 0xc0 as uint32_t {
        29 as uint32_t
    } else if b < 0xe0 as uint32_t {
        30 as uint32_t
    } else {
        31 as uint32_t
    };
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
unsafe extern "C" fn up64(mut v: uint64_t) -> uint64_t {
    return v.wrapping_add(63 as uint64_t) & !(63 as ::core::ffi::c_uint as uint64_t);
}
unsafe extern "C" fn hexval(mut c: uint8_t) -> ::core::ffi::c_int {
    if c as ::core::ffi::c_int >= '0' as i32 && c as ::core::ffi::c_int <= '9' as i32 {
        return c as ::core::ffi::c_int - '0' as i32;
    }
    if c as ::core::ffi::c_int >= 'a' as i32 && c as ::core::ffi::c_int <= 'f' as i32 {
        return c as ::core::ffi::c_int - 'a' as i32 + 10 as ::core::ffi::c_int;
    }
    if c as ::core::ffi::c_int >= 'A' as i32 && c as ::core::ffi::c_int <= 'F' as i32 {
        return c as ::core::ffi::c_int - 'A' as i32 + 10 as ::core::ffi::c_int;
    }
    return -(1 as ::core::ffi::c_int);
}
unsafe extern "C" fn fail(
    mut err: *mut toks_err,
    mut code: int64_t,
    mut what: *const ::core::ffi::c_char,
) -> int64_t {
    (*err).code = code;
    (*err).what = what;
    return code;
}
unsafe extern "C" fn one_cp(mut s: *const uint8_t, mut n: uint32_t) -> int64_t {
    if s.is_null() || n == 0 as uint32_t {
        return -(1 as ::core::ffi::c_int) as int64_t;
    }
    let mut k: uint32_t = toks_utf8_len(s, n as uint64_t);
    if k == 0 as uint32_t || k != n {
        return -(1 as ::core::ffi::c_int) as int64_t;
    }
    return if k == 1 as uint32_t {
        *s.offset(0 as ::core::ffi::c_int as isize) as int64_t
    } else {
        toks_cp_decode(s, k) as int64_t
    };
}
unsafe extern "C" fn phi(mut f: *const fold, mut c: uint32_t) -> uint32_t {
    let mut i: uint32_t = 0 as uint32_t;
    while i < (*f).n {
        if (*f).src[i as usize] == c {
            return (*f).dst[i as usize];
        }
        i = i.wrapping_add(1);
    }
    return c;
}
unsafe extern "C" fn fold_step(mut f: *mut fold, mut a: uint32_t, mut b: uint32_t) {
    let mut seen: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut i: uint32_t = 0 as uint32_t;
    while i < (*f).n {
        if (*f).dst[i as usize] == a {
            (*f).dst[i as usize] = b;
        }
        if (*f).src[i as usize] == a {
            seen = 1 as ::core::ffi::c_int;
        }
        i = i.wrapping_add(1);
    }
    if (*f).pfx == a as int64_t {
        (*f).pfx = b as int64_t;
    }
    if seen == 0 && a != b {
        (*f).src[(*f).n as usize] = a;
        (*f).dst[(*f).n as usize] = b;
        (*f).n = (*f).n.wrapping_add(1);
    }
}
unsafe extern "C" fn fold_text(
    mut tx: *const toks_spm_text,
    mut f: *mut fold,
    mut err: *mut toks_err,
) -> int64_t {
    memset(
        f as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<fold>() as size_t,
    );
    (*f).pfx = -(1 as ::core::ffi::c_int) as int64_t;
    (*f).repl = -(1 as ::core::ffi::c_int) as int64_t;
    (*f).mode = TOKS_SPM_PFX_NONE as ::core::ffi::c_int as uint32_t;
    let mut i: uint32_t = 0 as uint32_t;
    while i < (*tx).n_norm {
        let mut op: *const toks_spm_op = (&raw const (*tx).norm as *const toks_spm_op)
            .offset(i as isize) as *const toks_spm_op;
        if (*op).kind == TOKS_SPM_N_PREPEND as ::core::ffi::c_int as uint32_t {
            if !((*op).a.n == 0 as uint32_t) {
                let mut c: int64_t = one_cp(
                    &raw const (*op).a.b as *const uint8_t,
                    (*op).a.n,
                );
                if c < 0 as int64_t {
                    return fail(
                        err,
                        TOKS_E_UNSUPPORTED as int64_t,
                        b"normalizer Prepend of more than one char (the tables hold a one-symbol prefix)\0"
                            as *const u8 as *const ::core::ffi::c_char,
                    );
                }
                if (*f).pfx >= 0 as int64_t {
                    return fail(
                        err,
                        TOKS_E_UNSUPPORTED as int64_t,
                        b"normalizer: two Prepend steps\0" as *const u8
                            as *const ::core::ffi::c_char,
                    );
                }
                (*f).pfx = c;
                (*f).mode = TOKS_SPM_PFX_GAP as ::core::ffi::c_int as uint32_t;
            }
        } else {
            let mut a: int64_t = one_cp(
                &raw const (*op).a.b as *const uint8_t,
                (*op).a.n,
            );
            let mut b: int64_t = one_cp(
                &raw const (*op).b.b as *const uint8_t,
                (*op).b.n,
            );
            if a < 0 as int64_t || b < 0 as int64_t {
                return fail(
                    err,
                    TOKS_E_UNSUPPORTED as int64_t,
                    b"normalizer Replace other than one char -> one char (not absorbed by the tables)\0"
                        as *const u8 as *const ::core::ffi::c_char,
                );
            }
            fold_step(f, a as uint32_t, b as uint32_t);
        }
        i = i.wrapping_add(1);
    }
    if (*tx).metaspace != 0 {
        if (*tx).ms_split != 0 && (*f).pfx >= 0 as int64_t {
            return fail(
                err,
                TOKS_E_UNSUPPORTED as int64_t,
                b"Metaspace split after a Prepend prefix\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        let mut r: int64_t = one_cp(
            &raw const (*tx).ms_repl.b as *const uint8_t,
            (*tx).ms_repl.n,
        );
        if r < 0 as int64_t {
            return fail(
                err,
                TOKS_E_FORMAT as int64_t,
                b"Metaspace replacement\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        fold_step(f, ' ' as i32 as uint32_t, r as uint32_t);
        (*f).repl = r;
        if (*tx).ms_scheme != TOKS_SPM_PS_NEVER as ::core::ffi::c_int as uint32_t {
            if (*f).pfx >= 0 as int64_t {
                if (*f).pfx != r {
                    return fail(
                        err,
                        TOKS_E_UNSUPPORTED as int64_t,
                        b"Metaspace prefix after a Prepend prefix\0" as *const u8
                            as *const ::core::ffi::c_char,
                    );
                }
            } else {
                (*f).mode = (if (*tx).ms_scheme
                    == TOKS_SPM_PS_ALWAYS as ::core::ffi::c_int as uint32_t
                {
                    TOKS_SPM_PFX_ALWAYS as ::core::ffi::c_int
                } else {
                    TOKS_SPM_PFX_FIRST as ::core::ffi::c_int
                }) as uint32_t;
            }
        }
    }
    return 0 as int64_t;
}
unsafe extern "C" fn unreachable(
    mut f: *const fold,
    mut c: uint32_t,
) -> ::core::ffi::c_int {
    let mut is_src: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut i: uint32_t = 0 as uint32_t;
    while i < (*f).n {
        if (*f).dst[i as usize] == c {
            return 0 as ::core::ffi::c_int;
        }
        if (*f).src[i as usize] == c {
            is_src = 1 as ::core::ffi::c_int;
        }
        i = i.wrapping_add(1);
    }
    return (is_src != 0 && (*f).pfx != c as int64_t && (*f).repl != c as int64_t)
        as ::core::ffi::c_int;
}
unsafe extern "C" fn input_form(
    mut f: *const fold,
    mut v: *const uint8_t,
    mut n: uint32_t,
    mut out: *mut uint8_t,
) -> uint32_t {
    let mut o: uint32_t = 0 as uint32_t;
    let mut i: uint32_t = 0 as uint32_t;
    let mut k: uint32_t = 1 as uint32_t;
    while i < n {
        k = toks_utf8_len(v.offset(i as isize), n.wrapping_sub(i) as uint64_t);
        let mut cp: uint32_t = if k == 1 as uint32_t {
            *v.offset(i as isize) as uint32_t
        } else if k != 0 as uint32_t {
            toks_cp_decode(v.offset(i as isize), k)
        } else {
            0 as uint32_t
        };
        let mut src: uint32_t = cp;
        let mut j: uint32_t = 0 as uint32_t;
        while j < (*f).n && k > 1 as uint32_t {
            if (*f).dst[j as usize] == cp && (*f).src[j as usize] < 0x80 as uint32_t {
                src = (*f).src[j as usize];
            }
            j = j.wrapping_add(1);
        }
        let mut w: uint32_t = if src != cp { 1 as uint32_t } else { k };
        if k == 0 as uint32_t || o.wrapping_add(w) > 15 as uint32_t {
            return 0 as uint32_t;
        }
        if src != cp {
            *out.offset(o as isize) = src as uint8_t;
        } else {
            memcpy(
                out.offset(o as isize) as *mut ::core::ffi::c_void,
                v.offset(i as isize) as *const ::core::ffi::c_void,
                k as size_t,
            );
        }
        o = o.wrapping_add(w);
        i = i.wrapping_add(k);
    }
    return o;
}
unsafe extern "C" fn pre_classes(
    mut f: *const fold,
    mut cp: uint32_t,
    mut last: ::core::ffi::c_int,
) -> uint32_t {
    let mut m: uint32_t = 0 as uint32_t;
    let mut fixed: uint32_t = 1 as uint32_t;
    let mut i: uint32_t = 0 as uint32_t;
    while i <= (*f).n {
        let mut c: uint32_t = if i < (*f).n { (*f).src[i as usize] } else { cp };
        if i < (*f).n && c == cp {
            fixed = 0 as ::core::ffi::c_uint as uint32_t;
        }
        if !(if i < (*f).n {
            ((*f).dst[i as usize] != cp) as ::core::ffi::c_int
        } else {
            (fixed == 0) as ::core::ffi::c_int
        } != 0)
        {
            m = (m as ::core::ffi::c_uint
                | (1 as ::core::ffi::c_uint)
                    << bpe_apm_class(
                        (if c < 0x80 as uint32_t {
                            c
                        } else {
                            (if last != 0 {
                                0x80 as uint32_t
                            } else {
                                (if c < 0x800 as uint32_t {
                                    0xc0 as uint32_t
                                } else {
                                    0xe0 as uint32_t
                                })
                            })
                        }),
                    )) as uint32_t;
        }
        i = i.wrapping_add(1);
    }
    return m;
}
unsafe extern "C" fn spm_apm(
    mut t: *const toks_tables,
    mut s: *const toks_spm,
    mut cfg: *const toks_spm_config,
    mut f: *const fold,
    mut apm: *mut uint8_t,
) -> ::core::ffi::c_int {
    let mut nv: uint32_t = (*cfg).n_ids;
    let mut m: *mut uint32_t = if (*s).unk_id != TOKS_SPM_NONE as uint32_t {
        toks_plat_alloc((17 as uint64_t).wrapping_mul(nv as uint64_t)) as *mut uint32_t
    } else {
        ::core::ptr::null_mut::<uint32_t>()
    };
    if m.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    memset(
        m as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        (17 as size_t).wrapping_mul(nv as size_t),
    );
    let mut lm: *mut uint32_t = m;
    let mut fm: *mut uint32_t = m.offset(nv as isize);
    let mut rb: *mut uint32_t = m.offset((2 as uint32_t).wrapping_mul(nv) as isize);
    let mut ra: *mut uint32_t = m.offset((3 as uint32_t).wrapping_mul(nv) as isize);
    let mut fl: *mut uint8_t = m.offset((4 as uint32_t).wrapping_mul(nv) as isize)
        as *mut ::core::ffi::c_void as *mut uint8_t;
    let mut id: uint32_t = 0 as uint32_t;
    while id < nv {
        let mut v: *const uint8_t = *(*cfg).vocab.offset(id as isize);
        let mut n: uint32_t = *(*cfg).vocab_len.offset(id as isize);
        let mut k0: uint32_t = if !v.is_null() && n != 0 as uint32_t {
            toks_utf8_len(v, n as uint64_t)
        } else {
            0 as uint32_t
        };
        let mut j: uint32_t = n;
        if !(k0 == 0 as uint32_t) {
            while j > 1 as uint32_t
                && *v.offset(j.wrapping_sub(1 as uint32_t) as isize)
                    as ::core::ffi::c_uint & 0xc0 as ::core::ffi::c_uint
                    == 0x80 as ::core::ffi::c_uint
            {
                j = j.wrapping_sub(1);
            }
            *lm.offset(id as isize) = pre_classes(
                f,
                if n.wrapping_sub(j).wrapping_add(1 as uint32_t) > 1 as uint32_t {
                    toks_cp_decode(
                        v
                            .offset(j as isize)
                            .offset(-(1 as ::core::ffi::c_uint as isize)),
                        n.wrapping_sub(j).wrapping_add(1 as uint32_t),
                    )
                } else {
                    *v.offset(n.wrapping_sub(1 as uint32_t) as isize) as uint32_t
                },
                1 as ::core::ffi::c_int,
            );
            *fm.offset(id as isize) = pre_classes(
                f,
                if k0 > 1 as uint32_t {
                    toks_cp_decode(v, k0)
                } else {
                    *v.offset(0 as ::core::ffi::c_int as isize) as uint32_t
                },
                0 as ::core::ffi::c_int,
            );
        }
        id = id.wrapping_add(1);
    }
    let mut b: uint32_t = 0 as uint32_t;
    while b < 256 as uint32_t {
        let mut id_0: uint32_t = *(*t).byte2id.offset(b as isize);
        if id_0 < nv {
            let ref mut fresh6 = *fm.offset(id_0 as isize);
            *fresh6 = ((1 as ::core::ffi::c_uint) << bpe_apm_class(b)) as uint32_t;
            *lm.offset(id_0 as isize) = *fresh6;
            *fl.offset(id_0 as isize) = 1 as uint8_t;
        }
        if b < 128 as uint32_t
            && ((*s).ascii[b as usize] & TOKS_SPM_E_ID as uint32_t) < nv
        {
            let ref mut fresh7 = *fl
                .offset(((*s).ascii[b as usize] & TOKS_SPM_E_ID as uint32_t) as isize);
            *fresh7 = (*fresh7 as ::core::ffi::c_uint | 2 as ::core::ffi::c_uint)
                as uint8_t;
        }
        b = b.wrapping_add(1);
    }
    let ref mut fresh8 = *fm.offset((*s).unk_id as isize);
    *fresh8 = UINT32_MAX as uint32_t;
    *lm.offset((*s).unk_id as isize) = *fresh8;
    let ref mut fresh9 = *fl.offset((*s).unk_id as isize);
    *fresh9 = (*fresh9 as ::core::ffi::c_uint | 1 as ::core::ffi::c_uint) as uint8_t;
    let mut ok: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut r: uint32_t = 0 as uint32_t;
    while r < (*cfg).n_merges && ok != 0 {
        ok = (*fl.offset(*(*cfg).m_left.offset(r as isize) as isize)
            as ::core::ffi::c_uint & 1 as ::core::ffi::c_uint == 0
            && *fl.offset(*(*cfg).m_right.offset(r as isize) as isize)
                as ::core::ffi::c_uint & 1 as ::core::ffi::c_uint == 0)
            as ::core::ffi::c_int;
        r = r.wrapping_add(1);
    }
    memset(
        apm.offset(TOKS_APM_PAIRS as isize) as *mut ::core::ffi::c_void,
        0xff as ::core::ffi::c_int,
        (TOKS_APM_BYTES - TOKS_APM_PAIRS) as size_t,
    );
    let mut b_0: uint32_t = 0 as uint32_t;
    while b_0 < 256 as uint32_t {
        *apm.offset(b_0 as isize) = bpe_apm_class(b_0) as uint8_t;
        b_0 = b_0.wrapping_add(1);
    }
    let mut mt: bpe_mt = bpe_mt_of(t);
    let mut r_0: uint32_t = 0 as uint32_t;
    while r_0 < (*cfg).n_merges && ok != 0 {
        let mut a: uint32_t = *(*cfg).m_left.offset(r_0 as isize);
        let mut b_1: uint32_t = *(*cfg).m_right.offset(r_0 as isize);
        let mut o: uint32_t = *(*cfg).m_out.offset(r_0 as isize);
        *rb.offset(b_1 as isize) |= *lm.offset(a as isize);
        *ra.offset(a as isize) |= *fm.offset(b_1 as isize);
        if !((*fl.offset(a as isize) as ::core::ffi::c_int
            & *fl.offset(b_1 as isize) as ::core::ffi::c_int) as ::core::ffi::c_uint
            & 2 as ::core::ffi::c_uint == 0 as ::core::ffi::c_uint
            || bpe_mt_find(&raw mut mt, bpe_pair_key(a, b_1)) != r_0)
        {
            let mut v_0: [uint32_t; 4] = [
                o,
                *ra.offset(b_1 as isize) | *ra.offset(o as isize),
                *rb.offset(a as isize) | *rb.offset(o as isize),
                0 as ::core::ffi::c_uint,
            ];
            let mut x: uint32_t = 0 as uint32_t;
            while x < 128 as uint32_t {
                let mut y: uint32_t = 0 as uint32_t;
                while y < 128 as uint32_t
                    && (*s).ascii[x as usize] & TOKS_SPM_E_ID as uint32_t == a
                {
                    if (*s).ascii[y as usize] & TOKS_SPM_E_ID as uint32_t == b_1 {
                        memcpy(
                            apm
                                .offset(TOKS_APM_PAIRS as isize)
                                .offset(
                                    (16 as uint32_t)
                                        .wrapping_mul(x << 7 as ::core::ffi::c_int | y) as isize,
                                ) as *mut ::core::ffi::c_void,
                            &raw mut v_0 as *mut uint32_t as *const ::core::ffi::c_void,
                            16 as size_t,
                        );
                    }
                    y = y.wrapping_add(1);
                }
                x = x.wrapping_add(1);
            }
        }
        r_0 = r_0.wrapping_add(1);
    }
    toks_plat_free(
        m as *mut ::core::ffi::c_void,
        (17 as uint64_t).wrapping_mul(nv as uint64_t),
    );
    return ok;
}
unsafe extern "C" fn merge_buckets(mut n: uint64_t) -> uint64_t {
    let mut b: uint64_t = 2 as uint64_t;
    while b.wrapping_mul(4 as uint64_t) < n {
        b <<= 1 as ::core::ffi::c_int;
    }
    return b;
}
unsafe extern "C" fn pair_insert(
    mut slots: *mut uint64_t,
    mut mask: uint64_t,
    mut key: uint64_t,
) -> ::core::ffi::c_int {
    let mut i: uint64_t = ((key as ::core::ffi::c_ulonglong).wrapping_mul(TOKS_FIB64)
        >> 32 as ::core::ffi::c_int & mask as ::core::ffi::c_ulonglong) as uint64_t;
    let mut k: uint64_t = 0 as uint64_t;
    while k <= mask {
        if *slots.offset(i as isize) == key {
            return 0 as ::core::ffi::c_int;
        }
        if *slots.offset(i as isize) == UINT64_MAX as uint64_t {
            *slots.offset(i as isize) = key;
            return 1 as ::core::ffi::c_int;
        }
        i = i.wrapping_add(1 as uint64_t) & mask;
        k = k.wrapping_add(1);
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn pair_has(
    mut slots: *const uint64_t,
    mut mask: uint64_t,
    mut key: uint64_t,
) -> ::core::ffi::c_int {
    let mut i: uint64_t = ((key as ::core::ffi::c_ulonglong).wrapping_mul(TOKS_FIB64)
        >> 32 as ::core::ffi::c_int & mask as ::core::ffi::c_ulonglong) as uint64_t;
    let mut k: uint64_t = 0 as uint64_t;
    while k <= mask {
        if *slots.offset(i as isize) == UINT64_MAX as uint64_t {
            return 0 as ::core::ffi::c_int;
        }
        if *slots.offset(i as isize) == key {
            return 1 as ::core::ffi::c_int;
        }
        i = i.wrapping_add(1 as uint64_t) & mask;
        k = k.wrapping_add(1);
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn raw_id(
    mut st1: *const uint16_t,
    mut raw2: *const uint32_t,
    mut cp: uint32_t,
) -> uint32_t {
    if cp >= 0x110000 as uint32_t {
        return TOKS_SPM_E_NOID as uint32_t;
    }
    return *raw2
        .offset(
            (*st1.offset((cp >> 8 as ::core::ffi::c_int) as isize) as uint32_t)
                .wrapping_mul(256 as uint32_t)
                .wrapping_add(cp & 0xff as uint32_t) as isize,
        );
}
unsafe extern "C" fn small_index(mut img: uint32_t) -> uint32_t {
    return if img < 128 as uint32_t {
        img
    } else if img == 0x2581 as uint32_t {
        128 as uint32_t
    } else {
        TOKS_SPM_E_SI_NONE as uint32_t
    };
}
unsafe extern "C" fn add_pairs(
    mut slots: *mut uint64_t,
    mut mask: uint64_t,
    mut st1: *const uint16_t,
    mut raw2: *const uint32_t,
    mut s: *const uint8_t,
    mut n: uint32_t,
    mut left_bits: *mut uint32_t,
) -> uint64_t {
    let mut added: uint64_t = 0 as uint64_t;
    let mut i: uint32_t = 0 as uint32_t;
    let mut prev: uint32_t = TOKS_SPM_E_NOID as uint32_t;
    while i < n {
        let mut k: uint32_t = toks_utf8_len(
            s.offset(i as isize),
            n.wrapping_sub(i) as uint64_t,
        );
        if k == 0 as uint32_t {
            return added;
        }
        let mut cp: uint32_t = if k == 1 as uint32_t {
            *s.offset(i as isize) as uint32_t
        } else {
            toks_cp_decode(s.offset(i as isize), k)
        };
        let mut id: uint32_t = raw_id(st1, raw2, cp);
        if prev != TOKS_SPM_E_NOID as uint32_t && id != TOKS_SPM_E_NOID as uint32_t {
            if pair_insert(
                slots,
                mask,
                (prev as uint64_t) << 21 as ::core::ffi::c_int | id as uint64_t,
            ) != 0
            {
                added = added.wrapping_add(1);
                let ref mut fresh10 = *left_bits
                    .offset((prev >> 5 as ::core::ffi::c_int) as isize);
                *fresh10 = (*fresh10 as ::core::ffi::c_uint
                    | (1 as ::core::ffi::c_uint) << (prev & 31 as uint32_t)) as uint32_t;
            }
        }
        prev = id;
        i = i.wrapping_add(k);
    }
    return added;
}
#[no_mangle]
pub unsafe extern "C" fn toks_spm_build(
    mut t: *mut toks_tables,
    mut cfg: *const toks_spm_config,
    mut mem_out: *mut *mut uint8_t,
    mut mem_len_out: *mut uint64_t,
    mut out: *mut *const toks_spm,
    mut err: *mut toks_err,
) -> int64_t {
    *mem_out = ::core::ptr::null_mut::<uint8_t>();
    *mem_len_out = 0 as uint64_t;
    *out = ::core::ptr::null::<toks_spm>();
    let mut n_ids: uint32_t = (*t).n_ids;
    let mut nv: uint32_t = (*cfg).n_ids;
    if n_ids > TOKS_MAX_IDS as uint32_t || nv > n_ids
        || (*cfg).n_merges >= (1 as uint32_t) << TOKS_PRIO_BITS
    {
        return fail(
            err,
            TOKS_E_LIMIT as int64_t,
            b"ids or merges beyond the table widths\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    let mut f: fold = fold {
        src: [0; 9],
        dst: [0; 9],
        n: 0,
        pfx: 0,
        repl: 0,
        mode: 0,
    };
    let mut r: int64_t = fold_text(&raw const (*cfg).text, &raw mut f, err);
    if r != 0 as int64_t {
        return r;
    }
    let mut identity: ::core::ffi::c_int = (f.n == 0 as uint32_t
        && f.mode == TOKS_SPM_PFX_NONE as ::core::ffi::c_int as uint32_t)
        as ::core::ffi::c_int;
    let mut im: ::core::ffi::c_int = ((*cfg).sflags & TOKS_SPM_IGNORE_MERGES as uint32_t
        != 0 as uint32_t) as ::core::ffi::c_int;
    if im != 0 && identity == 0 {
        return fail(
            err,
            TOKS_E_UNSUPPORTED as int64_t,
            b"ignore_merges with a Replace / Prepend / Metaspace text model\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
    }
    let mut blk_used: [uint8_t; 4352] = [0; 4352];
    memset(
        &raw mut blk_used as *mut uint8_t as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<[uint8_t; 4352]>() as size_t,
    );
    let mut id: uint32_t = 0 as uint32_t;
    while id < nv {
        let mut cp: int64_t = one_cp(
            *(*cfg).vocab.offset(id as isize),
            *(*cfg).vocab_len.offset(id as isize),
        );
        if cp >= 0 as int64_t {
            blk_used[(cp >> 8 as ::core::ffi::c_int) as usize] = 1 as uint8_t;
        }
        id = id.wrapping_add(1);
    }
    let mut i: uint32_t = 0 as uint32_t;
    while i < f.n {
        blk_used[(f.src[i as usize] >> 8 as ::core::ffi::c_int) as usize] = 1 as uint8_t;
        i = i.wrapping_add(1);
    }
    blk_used[0 as ::core::ffi::c_int as usize] = 1 as uint8_t;
    let mut n_blocks: uint32_t = 1 as uint32_t;
    let mut b: uint32_t = 0 as uint32_t;
    while b < 0x1100 as uint32_t {
        n_blocks = n_blocks.wrapping_add(blk_used[b as usize] as uint32_t);
        b = b.wrapping_add(1);
    }
    let mut st2_bytes: uint64_t = (n_blocks as uint64_t)
        .wrapping_mul(256 as uint64_t)
        .wrapping_mul(4 as uint64_t);
    let mut bits: uint64_t = (n_ids as uint64_t)
        .wrapping_add(31 as uint64_t)
        .wrapping_div(32 as uint64_t)
        .wrapping_mul(4 as uint64_t);
    let mut ub: uint64_t = 0 as uint64_t;
    let mut id_0: uint32_t = 0 as uint32_t;
    while id_0 < nv {
        ub = ub.wrapping_add(*(*cfg).vocab_len.offset(id_0 as isize) as uint64_t);
        id_0 = id_0.wrapping_add(1);
    }
    let mut tps: uint64_t = if im != 0 {
        0 as uint64_t
    } else {
        bpe_pow2((2 as uint64_t).wrapping_mul(ub).wrapping_add(2 as uint64_t))
    };
    let mut a_st1: uint64_t = 0 as uint64_t;
    let mut a_st2: uint64_t = up64(
        (0x1100 as ::core::ffi::c_uint).wrapping_mul(2 as ::core::ffi::c_uint)
            as uint64_t,
    );
    let mut a_b2id: uint64_t = a_st2.wrapping_add(up64(st2_bytes));
    let mut a_reach: uint64_t = a_b2id
        .wrapping_add(
            up64(
                (256 as ::core::ffi::c_uint).wrapping_mul(4 as ::core::ffi::c_uint)
                    as uint64_t,
            ),
        );
    let mut a_left: uint64_t = a_reach.wrapping_add(up64(bits));
    let mut a_pairs: uint64_t = a_left.wrapping_add(up64(bits));
    let mut a_total: uint64_t = a_pairs.wrapping_add(tps.wrapping_mul(8 as uint64_t));
    let mut tmp: *mut uint8_t = toks_plat_alloc(a_total) as *mut uint8_t;
    if tmp.is_null() {
        return fail(
            err,
            TOKS_E_NOMEM as int64_t,
            b"spm build\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    memset(tmp as *mut ::core::ffi::c_void, 0 as ::core::ffi::c_int, a_total as size_t);
    let mut st1: *mut uint16_t = tmp.offset(a_st1 as isize) as *mut ::core::ffi::c_void
        as *mut uint16_t;
    let mut st2: *mut uint32_t = tmp.offset(a_st2 as isize) as *mut ::core::ffi::c_void
        as *mut uint32_t;
    let mut b2id: *mut uint32_t = tmp.offset(a_b2id as isize) as *mut ::core::ffi::c_void
        as *mut uint32_t;
    let mut reach: *mut uint32_t = tmp.offset(a_reach as isize)
        as *mut ::core::ffi::c_void as *mut uint32_t;
    let mut left: *mut uint32_t = tmp.offset(a_left as isize) as *mut ::core::ffi::c_void
        as *mut uint32_t;
    let mut tpairs: *mut uint64_t = tmp.offset(a_pairs as isize)
        as *mut ::core::ffi::c_void as *mut uint64_t;
    let mut why: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut b_0: uint32_t = 0 as uint32_t;
    while b_0 < 256 as uint32_t {
        *b2id.offset(b_0 as isize) = TOKS_SPM_NONE as uint32_t;
        b_0 = b_0.wrapping_add(1);
    }
    let mut n_bytes: uint32_t = 0 as uint32_t;
    let mut id_1: uint32_t = 0 as uint32_t;
    while id_1 < nv {
        let mut v: *const uint8_t = *(*cfg).vocab.offset(id_1 as isize);
        if !(v.is_null() || *(*cfg).vocab_len.offset(id_1 as isize) != 6 as uint32_t
            || *v.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                != '<' as i32
            || *v.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                != '0' as i32
            || *v.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                != 'x' as i32
            || *v.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                != '>' as i32)
        {
            let mut hi: ::core::ffi::c_int = hexval(
                *v.offset(3 as ::core::ffi::c_int as isize),
            );
            let mut lo: ::core::ffi::c_int = hexval(
                *v.offset(4 as ::core::ffi::c_int as isize),
            );
            if hi >= 0 as ::core::ffi::c_int && lo >= 0 as ::core::ffi::c_int
                && !(*v.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    >= 'a' as i32
                    && *v.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        <= 'f' as i32)
                && !(*v.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    >= 'a' as i32
                    && *v.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        <= 'f' as i32)
            {
                if *b2id.offset((hi * 16 as ::core::ffi::c_int + lo) as isize)
                    == TOKS_SPM_NONE as uint32_t
                {
                    n_bytes = n_bytes.wrapping_add(1);
                }
                *b2id.offset((hi * 16 as ::core::ffi::c_int + lo) as isize) = id_1;
            }
        }
        id_1 = id_1.wrapping_add(1);
    }
    if (*cfg).sflags & TOKS_SPM_UNK_ERROR as uint32_t != 0
        && !((*cfg).sflags & TOKS_SPM_BYTE_FALLBACK as uint32_t != 0
            && n_bytes == 256 as uint32_t)
    {
        why = b"model unk_token missing from the vocab (hf fails on a char without a token)\0"
            as *const u8 as *const ::core::ffi::c_char;
    }
    let mut i_0: uint64_t = 0 as uint64_t;
    while i_0 < (n_blocks as uint64_t).wrapping_mul(256 as uint64_t) {
        *st2.offset(i_0 as isize) = TOKS_SPM_E_NOID as uint32_t;
        i_0 = i_0.wrapping_add(1);
    }
    let mut next_blk: uint32_t = 1 as uint32_t;
    let mut b_1: uint32_t = 0 as uint32_t;
    while b_1 < 0x1100 as uint32_t {
        if blk_used[b_1 as usize] as ::core::ffi::c_uint != 0 as ::core::ffi::c_uint {
            let fresh0 = next_blk;
            next_blk = next_blk.wrapping_add(1);
            *st1.offset(b_1 as isize) = fresh0 as uint16_t;
        }
        b_1 = b_1.wrapping_add(1);
    }
    let mut id_2: uint32_t = 0 as uint32_t;
    while id_2 < nv {
        let mut cp_0: int64_t = one_cp(
            *(*cfg).vocab.offset(id_2 as isize),
            *(*cfg).vocab_len.offset(id_2 as isize),
        );
        if cp_0 >= 0 as int64_t {
            *st2
                .offset(
                    (*st1.offset((cp_0 >> 8 as ::core::ffi::c_int) as isize) as uint32_t)
                        .wrapping_mul(256 as uint32_t)
                        .wrapping_add(cp_0 as uint32_t & 0xff as uint32_t) as isize,
                ) = id_2;
        }
        id_2 = id_2.wrapping_add(1);
    }
    let mut i_1: uint32_t = 0 as uint32_t;
    while i_1 < f.n && why.is_null() {
        if raw_id(st1, st2, f.dst[i_1 as usize]) == TOKS_SPM_E_NOID as uint32_t {
            why = b"text model substitutes a char that is not a vocab char\0"
                as *const u8 as *const ::core::ffi::c_char;
        }
        i_1 = i_1.wrapping_add(1);
    }
    let mut pc: int64_t = if f.mode == TOKS_SPM_PFX_GAP as ::core::ffi::c_int as uint32_t
    {
        f.pfx
    } else if f.mode != TOKS_SPM_PFX_NONE as ::core::ffi::c_int as uint32_t {
        f.repl
    } else {
        -(1 as ::core::ffi::c_int) as int64_t
    };
    if why.is_null() && pc >= 0 as int64_t
        && raw_id(st1, st2, pc as uint32_t) == TOKS_SPM_E_NOID as uint32_t
    {
        why = b"text model prefix char is not a vocab char\0" as *const u8
            as *const ::core::ffi::c_char;
    }
    let mut id_repl: uint32_t = if f.repl >= 0 as int64_t {
        raw_id(st1, st2, f.repl as uint32_t)
    } else {
        TOKS_SPM_NONE as uint32_t
    };
    if why.is_null() && f.repl >= 0 as int64_t && id_repl == TOKS_SPM_E_NOID as uint32_t
    {
        why = b"Metaspace replacement is not a vocab char\0" as *const u8
            as *const ::core::ffi::c_char;
    }
    if !why.is_null() {
        toks_plat_free(tmp as *mut ::core::ffi::c_void, a_total);
        return fail(err, TOKS_E_UNSUPPORTED as int64_t, why);
    }
    let mut id_3: uint32_t = 0 as uint32_t;
    while id_3 < nv {
        let mut cp_1: int64_t = one_cp(
            *(*cfg).vocab.offset(id_3 as isize),
            *(*cfg).vocab_len.offset(id_3 as isize),
        );
        if cp_1 >= 0 as int64_t && unreachable(&raw mut f, cp_1 as uint32_t) == 0 {
            let ref mut fresh1 = *reach
                .offset((id_3 >> 5 as ::core::ffi::c_int) as isize);
            *fresh1 = (*fresh1 as ::core::ffi::c_uint
                | (1 as ::core::ffi::c_uint) << (id_3 & 31 as uint32_t)) as uint32_t;
        }
        id_3 = id_3.wrapping_add(1);
    }
    let mut b_2: uint32_t = 0 as uint32_t;
    while b_2 < 256 as uint32_t {
        if *b2id.offset(b_2 as isize) != TOKS_SPM_NONE as uint32_t {
            let ref mut fresh2 = *reach
                .offset(
                    (*b2id.offset(b_2 as isize) >> 5 as ::core::ffi::c_int) as isize,
                );
            *fresh2 = (*fresh2 as ::core::ffi::c_uint
                | (1 as ::core::ffi::c_uint)
                    << (*b2id.offset(b_2 as isize) & 31 as uint32_t)) as uint32_t;
        }
        b_2 = b_2.wrapping_add(1);
    }
    if (*cfg).unk_id != TOKS_SPM_NONE as uint32_t {
        let ref mut fresh3 = *reach
            .offset(((*cfg).unk_id >> 5 as ::core::ffi::c_int) as isize);
        *fresh3 = (*fresh3 as ::core::ffi::c_uint
            | (1 as ::core::ffi::c_uint) << ((*cfg).unk_id & 31 as uint32_t))
            as uint32_t;
    }
    let mut pass: uint32_t = 0 as uint32_t;
    while pass <= (*cfg).n_merges {
        let mut grew: uint32_t = 0 as uint32_t;
        let mut m: uint32_t = 0 as uint32_t;
        while m < (*cfg).n_merges {
            let mut l: uint32_t = *(*cfg).m_left.offset(m as isize);
            let mut rr: uint32_t = *(*cfg).m_right.offset(m as isize);
            let mut o: uint32_t = *(*cfg).m_out.offset(m as isize);
            if *reach.offset((l >> 5 as ::core::ffi::c_int) as isize)
                >> (l & 31 as uint32_t) & 1 as uint32_t != 0
                && *reach.offset((rr >> 5 as ::core::ffi::c_int) as isize)
                    >> (rr & 31 as uint32_t) & 1 as uint32_t != 0
                && *reach.offset((o >> 5 as ::core::ffi::c_int) as isize)
                    >> (o & 31 as uint32_t) & 1 as uint32_t == 0
            {
                let ref mut fresh4 = *reach
                    .offset((o >> 5 as ::core::ffi::c_int) as isize);
                *fresh4 = (*fresh4 as ::core::ffi::c_uint
                    | (1 as ::core::ffi::c_uint) << (o & 31 as uint32_t)) as uint32_t;
                grew = 1 as uint32_t;
            }
            m = m.wrapping_add(1);
        }
        if grew == 0 {
            break;
        }
        pass = pass.wrapping_add(1);
    }
    let mut np: uint64_t = 0 as uint64_t;
    if tps != 0 as uint64_t {
        memset(
            tpairs as *mut ::core::ffi::c_void,
            0xff as ::core::ffi::c_int,
            tps.wrapping_mul(8 as uint64_t) as size_t,
        );
        let mut id_4: uint32_t = 0 as uint32_t;
        while id_4 < nv {
            if !((*(*cfg).vocab.offset(id_4 as isize)).is_null()
                || *reach.offset((id_4 >> 5 as ::core::ffi::c_int) as isize)
                    >> (id_4 & 31 as uint32_t) & 1 as uint32_t == 0)
            {
                np = np
                    .wrapping_add(
                        add_pairs(
                            tpairs,
                            tps.wrapping_sub(1 as uint64_t),
                            st1,
                            st2,
                            *(*cfg).vocab.offset(id_4 as isize),
                            *(*cfg).vocab_len.offset(id_4 as isize),
                            left,
                        ),
                    );
            }
            id_4 = id_4.wrapping_add(1);
        }
    }
    let mut holes_any: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut id_5: uint32_t = 0 as uint32_t;
    while id_5 < n_ids && holes_any == 0 {
        let mut vocab: ::core::ffi::c_int = (id_5 < nv
            && !(*(*cfg).vocab.offset(id_5 as isize)).is_null()) as ::core::ffi::c_int;
        if vocab == 0
            && *(*t).tok_off.offset(id_5.wrapping_add(1 as uint32_t) as isize)
                == *(*t).tok_off.offset(id_5 as isize)
        {
            holes_any = 1 as ::core::ffi::c_int;
        }
        id_5 = id_5.wrapping_add(1);
    }
    let mut mb: uint64_t = merge_buckets((*cfg).n_merges as uint64_t);
    let mut vs: uint64_t = if im != 0 {
        bpe_pow2(
            (2 as uint64_t)
                .wrapping_mul((*cfg).n_strings as uint64_t)
                .wrapping_add(2 as uint64_t),
        )
    } else {
        0 as uint64_t
    };
    let mut ps: uint64_t = if im != 0 {
        0 as uint64_t
    } else {
        bpe_pow2((4 as uint64_t).wrapping_mul(np).wrapping_add(2 as uint64_t))
    };
    let mut o_b2id: uint64_t = up64(::core::mem::size_of::<toks_spm>() as uint64_t);
    let mut o_st1: uint64_t = o_b2id
        .wrapping_add(
            up64(
                (256 as ::core::ffi::c_uint).wrapping_mul(4 as ::core::ffi::c_uint)
                    as uint64_t,
            ),
        );
    let mut o_st2: uint64_t = o_st1
        .wrapping_add(
            up64(
                (0x1100 as ::core::ffi::c_uint).wrapping_mul(2 as ::core::ffi::c_uint)
                    as uint64_t,
            ),
        );
    let mut o_slots: uint64_t = o_st2.wrapping_add(up64(st2_bytes));
    let mut o_r2id: uint64_t = o_slots.wrapping_add(mb.wrapping_mul(64 as uint64_t));
    let mut o_vh: uint64_t = o_r2id
        .wrapping_add(
            up64(
                ((*cfg).n_merges as uint64_t)
                    .wrapping_mul(4 as uint64_t)
                    .wrapping_add(4 as uint64_t),
            ),
        );
    let mut o_pairs: uint64_t = o_vh.wrapping_add(vs.wrapping_mul(8 as uint64_t));
    let mut o_holes: uint64_t = o_pairs
        .wrapping_add(up64(ps.wrapping_add(1 as uint64_t).wrapping_mul(8 as uint64_t)));
    let mut o_pf: uint64_t = o_holes
        .wrapping_add((if holes_any != 0 { up64(bits) } else { 0 as uint64_t }));
    let mut n_words: uint64_t = 0 as uint64_t;
    let mut form: [uint8_t; 15] = [0; 15];
    let mut id_6: uint32_t = 0 as uint32_t;
    while id_6 < nv {
        n_words = n_words
            .wrapping_add(
                (!(*(*cfg).vocab.offset(id_6 as isize)).is_null()
                    && input_form(
                        &raw mut f,
                        *(*cfg).vocab.offset(id_6 as isize),
                        *(*cfg).vocab_len.offset(id_6 as isize),
                        &raw mut form as *mut uint8_t,
                    ) != 0 as uint32_t) as ::core::ffi::c_int as uint64_t,
            );
        id_6 = id_6.wrapping_add(1);
    }
    let mut wb: uint64_t = if n_words != 0 as uint64_t {
        bpe_pow2(n_words.wrapping_div(2 as uint64_t).wrapping_add(1 as uint64_t))
    } else {
        0 as uint64_t
    };
    let mut o_words: uint64_t = up64(o_pf.wrapping_add(mb.wrapping_mul(4 as uint64_t)));
    let mut o_apm: uint64_t = o_words
        .wrapping_add(wb.wrapping_mul(TOKS_BUCKET as uint64_t));
    let mut o_ab8: uint64_t = up64(o_apm.wrapping_add(TOKS_APM_BYTES as uint64_t));
    let mut total: uint64_t = o_ab8
        .wrapping_add(
            (if ps != 0 as uint64_t {
                65536 as ::core::ffi::c_uint
            } else {
                0 as ::core::ffi::c_uint
            }) as uint64_t,
        );
    let mut mem: *mut uint8_t = toks_plat_arena(total);
    if mem.is_null() {
        toks_plat_free(tmp as *mut ::core::ffi::c_void, a_total);
        return fail(
            err,
            TOKS_E_NOMEM as int64_t,
            b"spm tables\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    memset(mem as *mut ::core::ffi::c_void, 0 as ::core::ffi::c_int, total as size_t);
    let mut s: *mut toks_spm = toks_tab(
        mem,
        0 as uint64_t,
        ::core::mem::size_of::<toks_spm>() as uint64_t,
        TOKS_X_SPM,
    ) as *mut toks_spm;
    (*s).unk_id = (*cfg).unk_id;
    (*s).sflags = (*cfg).sflags;
    (*s).n_blocks = n_blocks;
    (*s).n_dec = (*cfg).n_dec;
    (*s).has_decoder = (*cfg).has_decoder;
    memcpy(
        &raw mut (*s).dec as *mut toks_spm_op as *mut ::core::ffi::c_void,
        &raw const (*cfg).dec as *const toks_spm_op as *const ::core::ffi::c_void,
        ::core::mem::size_of::<[toks_spm_op; 8]>() as size_t,
    );
    (*s).ms_split = if (*cfg).text.metaspace != 0 {
        (*cfg).text.ms_split
    } else {
        0 as uint32_t
    };
    (*s).pfx_mode = f.mode;
    (*s).id_repl = id_repl;
    let mut fb2: *mut uint32_t = toks_tab(
        mem,
        o_b2id,
        (256 as ::core::ffi::c_uint).wrapping_mul(4 as ::core::ffi::c_uint) as uint64_t,
        TOKS_X_BYTE2ID,
    ) as *mut uint32_t;
    memcpy(
        fb2 as *mut ::core::ffi::c_void,
        b2id as *const ::core::ffi::c_void,
        (256 as ::core::ffi::c_uint).wrapping_mul(4 as ::core::ffi::c_uint) as size_t,
    );
    (*t).byte2id = fb2;
    let mut r2id: *mut uint32_t = toks_tab(
        mem,
        o_r2id,
        ((*cfg).n_merges as uint64_t).wrapping_mul(4 as uint64_t),
        TOKS_X_RANK2ID,
    ) as *mut uint32_t;
    let mut m_0: uint32_t = 0 as uint32_t;
    while m_0 < (*cfg).n_merges {
        *r2id.offset(m_0 as isize) = *(*cfg).m_out.offset(m_0 as isize);
        m_0 = m_0.wrapping_add(1);
    }
    (*s).n_dropped = toks_merge_slots(
        t,
        toks_tab(mem, o_slots, mb.wrapping_mul(64 as uint64_t), TOKS_X_MERGE_SLOTS)
            as *mut uint64_t,
        toks_tab(mem, o_pf, mb.wrapping_mul(4 as uint64_t), TOKS_X_PAIRF)
            as *mut uint64_t,
        mb,
        (*cfg).m_left,
        (*cfg).m_right,
        (*cfg).n_merges,
        reach,
    );
    (*t).rank2id = r2id;
    (*t).flags = (if im != 0 { TOKS_TF_IGNORE_MERGES } else { 0 as ::core::ffi::c_uint })
        as uint32_t;
    if vs != 0 as uint64_t {
        let mut vh: *mut uint64_t = toks_tab(
            mem,
            o_vh,
            vs.wrapping_mul(8 as uint64_t),
            TOKS_X_VHASH,
        ) as *mut uint64_t;
        memset(
            vh as *mut ::core::ffi::c_void,
            0xff as ::core::ffi::c_int,
            vs.wrapping_mul(8 as uint64_t) as size_t,
        );
        let mut id_7: uint32_t = 0 as uint32_t;
        while id_7 < nv {
            if !(*(*cfg).vocab.offset(id_7 as isize)).is_null() {
                let mut h: uint32_t = bpe_vhash_h(
                    *(*cfg).vocab.offset(id_7 as isize),
                    *(*cfg).vocab_len.offset(id_7 as isize) as uint64_t,
                );
                let mut i_2: uint64_t = h as uint64_t & vs.wrapping_sub(1 as uint64_t);
                while *vh.offset(i_2 as isize) != UINT64_MAX as uint64_t {
                    i_2 = i_2.wrapping_add(1 as uint64_t)
                        & vs.wrapping_sub(1 as uint64_t);
                }
                *vh.offset(i_2 as isize) = (id_7 as uint64_t) << 32 as ::core::ffi::c_int
                    | h as uint64_t;
            }
            id_7 = id_7.wrapping_add(1);
        }
        (*t).vhash = vh;
        (*t).vhash_mask = vs.wrapping_sub(1 as uint64_t);
    }
    if ps != 0 as uint64_t {
        let mut pairs: *mut uint64_t = toks_tab(
            mem,
            o_pairs,
            ps.wrapping_add(1 as uint64_t).wrapping_mul(8 as uint64_t),
            TOKS_X_SPM_PAIRS,
        ) as *mut uint64_t;
        memset(
            pairs as *mut ::core::ffi::c_void,
            0xff as ::core::ffi::c_int,
            ps.wrapping_add(1 as uint64_t).wrapping_mul(8 as uint64_t) as size_t,
        );
        let mut i_3: uint64_t = 0 as uint64_t;
        while i_3 < tps {
            if *tpairs.offset(i_3 as isize) != UINT64_MAX as uint64_t {
                pair_insert(
                    pairs,
                    ps.wrapping_sub(1 as uint64_t),
                    *tpairs.offset(i_3 as isize),
                );
            }
            i_3 = i_3.wrapping_add(1);
        }
        *pairs.offset(ps as isize) = *pairs.offset(0 as ::core::ffi::c_int as isize);
        (*s).pairs = pairs;
        (*s).pairs_mask = ps.wrapping_sub(1 as uint64_t);
        (*s).n_pairs = np;
    }
    let mut fst1: *mut uint16_t = toks_tab(
        mem,
        o_st1,
        (0x1100 as ::core::ffi::c_uint).wrapping_mul(2 as ::core::ffi::c_uint)
            as uint64_t,
        TOKS_X_SPM_STAGE1,
    ) as *mut uint16_t;
    let mut fst2: *mut uint32_t = toks_tab(mem, o_st2, st2_bytes, TOKS_X_SPM_STAGE2)
        as *mut uint32_t;
    memcpy(
        fst1 as *mut ::core::ffi::c_void,
        st1 as *const ::core::ffi::c_void,
        (0x1100 as ::core::ffi::c_uint).wrapping_mul(2 as ::core::ffi::c_uint) as size_t,
    );
    let mut lo_0: uint32_t = 0 as uint32_t;
    while lo_0 < 256 as uint32_t {
        *fst2.offset(lo_0 as isize) = (TOKS_SPM_E_NOID
            | TOKS_SPM_E_SI_NONE << TOKS_SPM_E_SI_SHIFT) as uint32_t;
        lo_0 = lo_0.wrapping_add(1);
    }
    let mut b_3: uint32_t = 0 as uint32_t;
    while b_3 < 0x1100 as uint32_t {
        if !(*fst1.offset(b_3 as isize) as ::core::ffi::c_uint
            == 0 as ::core::ffi::c_uint)
        {
            let mut lo_1: uint32_t = 0 as uint32_t;
            while lo_1 < 256 as uint32_t {
                let mut cp_2: uint32_t = b_3 << 8 as ::core::ffi::c_int | lo_1;
                let mut img: uint32_t = phi(&raw mut f, cp_2);
                let mut id_8: uint32_t = raw_id(st1, st2, img);
                let mut e: *mut uint32_t = fst2
                    .offset(
                        (*fst1.offset(b_3 as isize) as uint32_t)
                            .wrapping_mul(256 as uint32_t)
                            .wrapping_add(lo_1) as isize,
                    ) as *mut uint32_t;
                if id_8 == TOKS_SPM_E_NOID as uint32_t {
                    *e = (TOKS_SPM_E_NOID | TOKS_SPM_E_SI_NONE << TOKS_SPM_E_SI_SHIFT)
                        as uint32_t;
                } else {
                    *e = id_8 | small_index(img) << TOKS_SPM_E_SI_SHIFT
                        | (if *left.offset((id_8 >> 5 as ::core::ffi::c_int) as isize)
                            >> (id_8 & 31 as uint32_t) & 1 as uint32_t != 0
                        {
                            TOKS_SPM_E_PAIRED as uint32_t
                        } else {
                            0 as uint32_t
                        })
                        | (if small_index(img) == TOKS_SPM_E_SI_NONE as uint32_t
                            && !((*s).ms_split != 0 && id_8 == id_repl)
                        {
                            TOKS_SPM_E_PLAIN as uint32_t
                        } else {
                            0 as uint32_t
                        });
                }
                lo_1 = lo_1.wrapping_add(1);
            }
        }
        b_3 = b_3.wrapping_add(1);
    }
    let mut c: uint32_t = 0 as uint32_t;
    while c < 128 as uint32_t {
        (*s).ascii[c as usize] = *fst2
            .offset(
                (*fst1.offset(0 as ::core::ffi::c_int as isize) as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add(c) as isize,
            );
        c = c.wrapping_add(1);
    }
    (*s).stage1 = fst1;
    (*s).stage2 = fst2;
    if pc >= 0 as int64_t {
        let mut id_9: uint32_t = raw_id(st1, st2, pc as uint32_t);
        (*s).pfx_entry = id_9 | small_index(pc as uint32_t) << TOKS_SPM_E_SI_SHIFT
            | (if *left.offset((id_9 >> 5 as ::core::ffi::c_int) as isize)
                >> (id_9 & 31 as uint32_t) & 1 as uint32_t != 0
            {
                TOKS_SPM_E_PAIRED as uint32_t
            } else {
                0 as uint32_t
            });
    }
    if !(*s).pairs.is_null() {
        let mut x: uint32_t = 0 as uint32_t;
        while x < TOKS_SPM_SMALL as uint32_t {
            let mut ix: uint32_t = raw_id(
                st1,
                st2,
                if x < 128 as uint32_t { x } else { 0x2581 as uint32_t },
            );
            let mut y: uint32_t = 0 as uint32_t;
            while y < TOKS_SPM_SMALL as uint32_t {
                let mut iy: uint32_t = raw_id(
                    st1,
                    st2,
                    if y < 128 as uint32_t { y } else { 0x2581 as uint32_t },
                );
                if !(ix == TOKS_SPM_E_NOID as uint32_t
                    || iy == TOKS_SPM_E_NOID as uint32_t)
                {
                    if !(pair_has(
                        (*s).pairs,
                        (*s).pairs_mask,
                        (ix as uint64_t) << 21 as ::core::ffi::c_int | iy as uint64_t,
                    ) != 0)
                    {
                        let mut bit: uint32_t = x
                            .wrapping_mul(TOKS_SPM_SMALL as uint32_t)
                            .wrapping_add(y);
                        (*s).cut[(bit >> 3 as ::core::ffi::c_int) as usize] = ((*s)
                            .cut[(bit >> 3 as ::core::ffi::c_int) as usize]
                            as ::core::ffi::c_int
                            | ((1 as ::core::ffi::c_uint) << (bit & 7 as uint32_t))
                                as uint8_t as ::core::ffi::c_int) as uint8_t;
                    }
                }
                y = y.wrapping_add(1);
            }
            x = x.wrapping_add(1);
        }
        let mut ab8: *mut uint8_t = toks_tab(
            mem,
            o_ab8,
            65536 as uint64_t,
            TOKS_X_SPM_AB8,
        ) as *mut uint8_t;
        let mut x_0: uint32_t = 0 as uint32_t;
        while x_0 < 128 as uint32_t {
            let mut y_0: uint32_t = 0 as uint32_t;
            while y_0 < 128 as uint32_t {
                let mut ey: uint32_t = (*s).ascii[y_0 as usize];
                let mut bit_0: uint32_t = x_0 | y_0 << 8 as ::core::ffi::c_int;
                if (*s).ms_split != 0 && ey & TOKS_SPM_E_ID as uint32_t == (*s).id_repl
                    || toks_spm_cut_between(s, (*s).ascii[x_0 as usize], ey) != 0
                {
                    (*s).cut_ab[(bit_0 >> 5 as ::core::ffi::c_int) as usize] = ((*s)
                        .cut_ab[(bit_0 >> 5 as ::core::ffi::c_int) as usize]
                        as ::core::ffi::c_uint
                        | (1 as ::core::ffi::c_uint) << (bit_0 & 31 as uint32_t))
                        as uint32_t;
                    *ab8.offset(bit_0 as isize) = 1 as uint8_t;
                }
                y_0 = y_0.wrapping_add(1);
            }
            x_0 = x_0.wrapping_add(1);
        }
        (*s).cut_ab8 = ab8;
    }
    if holes_any != 0 {
        let mut h_0: *mut uint32_t = toks_tab(mem, o_holes, bits, TOKS_X_SPM_HOLES)
            as *mut uint32_t;
        let mut id_10: uint32_t = 0 as uint32_t;
        while id_10 < n_ids {
            let mut vocab_0: ::core::ffi::c_int = (id_10 < nv
                && !(*(*cfg).vocab.offset(id_10 as isize)).is_null())
                as ::core::ffi::c_int;
            if vocab_0 == 0
                && *(*t).tok_off.offset(id_10.wrapping_add(1 as uint32_t) as isize)
                    == *(*t).tok_off.offset(id_10 as isize)
            {
                let ref mut fresh5 = *h_0
                    .offset((id_10 >> 5 as ::core::ffi::c_int) as isize);
                *fresh5 = (*fresh5 as ::core::ffi::c_uint
                    | (1 as ::core::ffi::c_uint) << (id_10 & 31 as uint32_t))
                    as uint32_t;
            }
            id_10 = id_10.wrapping_add(1);
        }
        (*s).holes = h_0;
    }
    let mut apm: *mut uint8_t = toks_tab(
        mem,
        o_apm,
        TOKS_APM_BYTES as uint64_t,
        TOKS_X_APM,
    ) as *mut uint8_t;
    (*t).apm = if spm_apm(t, s, cfg, &raw mut f, apm) != 0 {
        apm
    } else {
        ::core::ptr::null_mut::<uint8_t>()
    };
    let mut work: crate::Aligned<[uint8_t; 584]> = crate::Aligned([0; 584]);
    let mut ids: [uint32_t; 16] = [0; 16];
    let mut val: [uint32_t; 4] = [0; 4];
    let mut words: *mut uint8_t = if wb != 0 as uint64_t {
        toks_tab(mem, o_words, wb.wrapping_mul(TOKS_BUCKET as uint64_t), TOKS_X_WORDS)
            as *mut uint8_t
    } else {
        ::core::ptr::null_mut::<uint8_t>()
    };
    toks_tab_seal(mem as *mut ::core::ffi::c_void, total);
    (*t).words = words;
    (*t).words_mask = wb.wrapping_sub(1 as uint64_t);
    let mut id_11: uint32_t = 0 as uint32_t;
    let mut placed: uint32_t = 0 as uint32_t;
    while id_11 < nv
        && (placed.wrapping_mul(100 as uint32_t) as uint64_t)
            < wb.wrapping_mul(170 as uint64_t)
    {
        let mut l_0: uint32_t = if !(*(*cfg).vocab.offset(id_11 as isize)).is_null() {
            input_form(
                &raw mut f,
                *(*cfg).vocab.offset(id_11 as isize),
                *(*cfg).vocab_len.offset(id_11 as isize),
                &raw mut form as *mut uint8_t,
            )
        } else {
            0 as uint32_t
        };
        let mut m_1: uint64_t = if l_0 != 0 as uint32_t {
            toks_spm_model(
                t,
                s,
                &raw mut form as *mut uint8_t,
                l_0 as uint64_t,
                &raw mut ids as *mut uint32_t,
                &raw mut work as *mut uint8_t,
            )
        } else {
            0 as uint64_t
        };
        if !(m_1 == 0 as uint64_t || m_1 > 4 as uint64_t) {
            let mut k: bpe_key = bpe_key_at(
                &raw mut form as *mut uint8_t,
                l_0 as uint64_t,
                0 as uint64_t,
                l_0 as uint64_t,
            );
            bpe_val_pack_tag(
                &raw mut val as *mut uint32_t,
                &raw mut ids as *mut uint32_t,
                m_1 as uint32_t,
                0 as uint64_t,
            );
            placed = placed
                .wrapping_add(
                    bpe_words_put(
                        words,
                        wb.wrapping_sub(1 as uint64_t),
                        toks_spm_whash(k.lo, k.hi),
                        k,
                        &raw mut val as *mut uint32_t as *const uint32_t,
                    ) as uint32_t,
                );
        }
        id_11 = id_11.wrapping_add(1);
    }
    toks_plat_free(tmp as *mut ::core::ffi::c_void, a_total);
    *mem_out = mem;
    *mem_len_out = total;
    *out = s;
    return 0 as int64_t;
}
