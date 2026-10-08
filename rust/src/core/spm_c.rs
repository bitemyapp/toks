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
    static TOKS_CRC32C_TAB: [uint32_t; 256];
    #[cfg_attr(target_arch = "x86_64", link_name = "toks_k7_spm_avx2")]
    fn toks_k7_spm_neon(s: *const toks_spm, a: *mut toks_k7_args) -> uint64_t;
    fn toks_k6_merge(
        t: *const toks_tables,
        work: *mut uint8_t,
        cap: uint64_t,
        n: uint64_t,
        out: *mut uint32_t,
    ) -> uint64_t;
    #[cfg_attr(target_arch = "x86_64", link_name = "toks_k6_merge_avx2")]
    fn toks_k6_merge_neon(t: *const toks_tables, a: *mut toks_k6_args) -> uint64_t;
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
pub type uintptr_t = usize;
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
pub struct toks_k6_args {
    pub piece: *const uint8_t,
    pub len: uint64_t,
    pub out: *mut uint32_t,
    pub work: *mut uint8_t,
    pub work_bytes: uint64_t,
    pub n_out: uint64_t,
    pub merges: uint64_t,
    pub rsv: uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct toks_k7_args {
    pub text: *const uint8_t,
    pub len: uint64_t,
    pub pos: uint64_t,
    pub prev: uint64_t,
    pub one: *mut uint32_t,
    pub mb: uint64_t,
    pub flags: uint64_t,
    pub rsv: uint64_t,
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
pub struct wkey {
    pub q: [uint64_t; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dec {
    pub out: *mut uint8_t,
    pub cap: uint64_t,
    pub total: uint64_t,
    pub strip_left: uint32_t,
    pub strip: toks_spm_str,
    pub raw: ::core::ffi::c_int,
}
pub const UINT32_MAX: ::core::ffi::c_uint = 4294967295 as ::core::ffi::c_uint;
pub const UINT64_MAX: ::core::ffi::c_ulong = 18446744073709551615
    as ::core::ffi::c_ulong;
pub const TOKS_TIER_NEON: ::core::ffi::c_uint = 2 as ::core::ffi::c_uint;
pub const TOKS_SKIP_SPECIAL: ::core::ffi::c_uint = 1 as ::core::ffi::c_uint;
pub const TOKS_DECODE_RAW: ::core::ffi::c_uint = 2 as ::core::ffi::c_uint;
pub const TOKS_ID_MASK: ::core::ffi::c_uint = 0x1fffff as ::core::ffi::c_uint;
pub const TOKS_TF_IGNORE_MERGES: ::core::ffi::c_uint = 0x1 as ::core::ffi::c_uint;
pub const TOKS_TF_ASM_MERGE: ::core::ffi::c_uint = 0x40 as ::core::ffi::c_uint;
pub const TOKS_KEY_MAXLEN: ::core::ffi::c_int = 15 as ::core::ffi::c_int;
pub const TOKS_VAL_COUNT_SHIFT: ::core::ffi::c_int = 29 as ::core::ffi::c_int;
pub const TOKS_BUCKET: ::core::ffi::c_int = 64 as ::core::ffi::c_int;
pub const TOKS_TAG_MASK64: ::core::ffi::c_ulonglong = 0xffe00000ffe00000
    as ::core::ffi::c_ulonglong;
pub const TOKS_FIB64: ::core::ffi::c_ulonglong = 0x9e3779b97f4a7c15
    as ::core::ffi::c_ulonglong;
pub const TOKS_APM_PAIRS: ::core::ffi::c_int = 256 as ::core::ffi::c_int;
pub const TOKS_VSEED: ::core::ffi::c_uint = 0x85ebca6b as ::core::ffi::c_uint;
#[inline]
unsafe extern "C" fn toks_tag_word(mut tag: uint64_t) -> uint64_t {
    return (tag & 0x7ff as uint64_t) << 21 as ::core::ffi::c_int
        | (tag >> 11 as ::core::ffi::c_int & 0x7ff as uint64_t)
            << 53 as ::core::ffi::c_int;
}
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
unsafe extern "C" fn toks_cpy(
    mut dst: *mut ::core::ffi::c_void,
    mut src: *const ::core::ffi::c_void,
    mut n: uint64_t,
) {
    memcpy(dst, src, n as size_t);
}
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
pub const TOKS_SPM_FUSE_UNK: ::core::ffi::c_uint = 0x2 as ::core::ffi::c_uint;
pub const TOKS_SPM_E_ID: ::core::ffi::c_uint = 0x1fffff as ::core::ffi::c_uint;
pub const TOKS_SPM_E_NOID: ::core::ffi::c_uint = 0x1fffff as ::core::ffi::c_uint;
pub const TOKS_SPM_E_SI_SHIFT: ::core::ffi::c_uint = 21 as ::core::ffi::c_uint;
pub const TOKS_SPM_E_SI_NONE: ::core::ffi::c_uint = 255 as ::core::ffi::c_uint;
pub const TOKS_SPM_E_PAIRED: ::core::ffi::c_uint = 0x40000000 as ::core::ffi::c_uint;
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
pub const TOKS_SPM_NOCUTS: ::core::ffi::c_uint = 1 as ::core::ffi::c_uint;
pub const TOKS_SPM_NOCACHE: ::core::ffi::c_uint = 2 as ::core::ffi::c_uint;
pub const TOKS_SPM_NOPFX: ::core::ffi::c_uint = 4 as ::core::ffi::c_uint;
#[inline]
unsafe extern "C" fn toks_spm_whash(mut lo: uint64_t, mut hi: uint64_t) -> uint32_t {
    return ((lo as ::core::ffi::c_ulonglong
        ^ (hi as ::core::ffi::c_ulonglong).wrapping_mul(TOKS_FIB64))
        .wrapping_mul(0xd6e8feb86659fd93 as ::core::ffi::c_ulonglong)
        >> 32 as ::core::ffi::c_int) as uint32_t;
}
pub const BPE_EMPTY_SLOT: ::core::ffi::c_ulong = UINT64_MAX;
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
unsafe extern "C" fn bpe_vhash_find(
    mut t: *const toks_tables,
    mut p: *const uint8_t,
    mut len: uint64_t,
    mut id_out: *mut uint32_t,
) -> ::core::ffi::c_int {
    if (*t).vhash.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    let mut h: uint32_t = bpe_vhash_h(p, len);
    let mut i: uint64_t = h as uint64_t & (*t).vhash_mask;
    let mut k: uint64_t = 0 as uint64_t;
    while k <= (*t).vhash_mask {
        let mut slot: uint64_t = *(*t).vhash.offset(i as isize);
        if slot == BPE_EMPTY_SLOT as uint64_t {
            return 0 as ::core::ffi::c_int;
        }
        if slot as uint32_t == h {
            let mut id: uint32_t = (slot >> 32 as ::core::ffi::c_int) as uint32_t;
            let mut o0: uint32_t = *(*t).tok_off.offset(id as isize);
            if (*(*t).tok_off.offset(id.wrapping_add(1 as uint32_t) as isize)
                as uint64_t)
                .wrapping_sub(o0 as uint64_t) == len
                && memcmp(
                    (*t).tok_bytes.offset(o0 as isize) as *const ::core::ffi::c_void,
                    p as *const ::core::ffi::c_void,
                    len as size_t,
                ) == 0 as ::core::ffi::c_int
            {
                *id_out = id;
                return 1 as ::core::ffi::c_int;
            }
        }
        i = i.wrapping_add(1 as uint64_t) & (*t).vhash_mask;
        k = k.wrapping_add(1);
    }
    return 0 as ::core::ffi::c_int;
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
unsafe extern "C" fn bpe_val_count(mut v: *const uint32_t) -> uint32_t {
    return *v.offset(0 as ::core::ffi::c_int as isize) >> TOKS_VAL_COUNT_SHIFT;
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
pub const NOENTRY: ::core::ffi::c_uint = TOKS_SPM_E_NOID
    | TOKS_SPM_E_SI_NONE << TOKS_SPM_E_SI_SHIFT;
#[inline]
unsafe extern "C" fn entry_at(
    mut s: *const toks_spm,
    mut p: *const uint8_t,
    mut avail: uint64_t,
    mut k: *mut uint32_t,
) -> uint32_t {
    let mut b: uint8_t = *p.offset(0 as ::core::ffi::c_int as isize);
    if (b as ::core::ffi::c_uint) < 0x80 as ::core::ffi::c_uint {
        *k = 1 as ::core::ffi::c_uint as uint32_t;
        return (*s).ascii[b as usize];
    }
    let mut n: uint32_t = toks_utf8_len(p, avail);
    if n == 0 as uint32_t {
        *k = 1 as ::core::ffi::c_uint as uint32_t;
        return NOENTRY as uint32_t;
    }
    *k = n;
    let mut cp: uint32_t = toks_cp_decode(p, n);
    return *(*s)
        .stage2
        .offset(
            (*(*s).stage1.offset((cp >> 8 as ::core::ffi::c_int) as isize) as uint32_t)
                .wrapping_mul(256 as uint32_t)
                .wrapping_add(cp & 0xff as uint32_t) as isize,
        );
}
#[inline]
unsafe extern "C" fn cut8(mut ab: *const uint32_t, mut p: *const uint8_t) -> uint64_t {
    let mut m: uint64_t = 0 as uint64_t;
    let mut j: uint32_t = 0 as uint32_t;
    while j < 8 as uint32_t {
        let mut x: uint32_t = *p.offset(j as isize) as uint32_t
            | (*p.offset(j.wrapping_add(1 as uint32_t) as isize) as uint32_t)
                << 8 as ::core::ffi::c_int;
        m
            |= ((*ab.offset((x >> 5 as ::core::ffi::c_int) as isize)
                >> (x & 31 as uint32_t) & 1 as uint32_t) as uint64_t) << j;
        j = j.wrapping_add(1);
    }
    return m;
}
#[no_mangle]
pub unsafe extern "C" fn toks_spm_cut(
    mut s: *const toks_spm,
    mut text: *const uint8_t,
    mut len: uint64_t,
    mut a: uint64_t,
    mut c: uint64_t,
) -> ::core::ffi::c_int {
    let mut k: uint32_t = 0;
    let mut ea: uint32_t = entry_at(
        s,
        text.offset(a as isize),
        len.wrapping_sub(a),
        &raw mut k,
    );
    let mut eb: uint32_t = entry_at(
        s,
        text.offset(c as isize),
        len.wrapping_sub(c),
        &raw mut k,
    );
    let mut repl: ::core::ffi::c_int = (eb & TOKS_SPM_E_ID as uint32_t == (*s).id_repl
        && (*s).id_repl != TOKS_SPM_NONE as uint32_t) as ::core::ffi::c_int;
    if (*s).pfx_mode == TOKS_SPM_PFX_GAP as ::core::ffi::c_int as uint32_t
        || (*s).pfx_mode == TOKS_SPM_PFX_ALWAYS as ::core::ffi::c_int as uint32_t
            && repl == 0
    {
        return 0 as ::core::ffi::c_int;
    }
    if (*s).ms_split != 0 && repl != 0 {
        return 1 as ::core::ffi::c_int;
    }
    return (!(*s).pairs.is_null() && toks_spm_cut_between(s, ea, eb) != 0)
        as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn has_prefix(
    mut s: *const toks_spm,
    mut first: uint32_t,
    mut at_start: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut m: ::core::ffi::c_int = (*s).pfx_mode as ::core::ffi::c_int;
    let mut repl: ::core::ffi::c_int = (first & TOKS_SPM_E_ID as uint32_t
        == (*s).id_repl) as ::core::ffi::c_int;
    return (m == TOKS_SPM_PFX_GAP as ::core::ffi::c_int
        || repl == 0
            && (m == TOKS_SPM_PFX_ALWAYS as ::core::ffi::c_int
                || m == TOKS_SPM_PFX_FIRST as ::core::ffi::c_int && at_start != 0))
        as ::core::ffi::c_int;
}
unsafe extern "C" fn symbols(
    mut t: *const toks_tables,
    mut s: *const toks_spm,
    mut p: *const uint8_t,
    mut len: uint64_t,
    mut wp: ::core::ffi::c_int,
    mut sym: *mut uint32_t,
) -> uint64_t {
    let mut n: uint64_t = 0 as uint64_t;
    let mut i: uint64_t = 0 as uint64_t;
    let mut pending: uint32_t = TOKS_SPM_NONE as uint32_t;
    if wp != 0 {
        let fresh0 = n;
        n = n.wrapping_add(1);
        *sym.offset(fresh0 as isize) = (*s).pfx_entry & TOKS_SPM_E_ID as uint32_t;
    }
    while i < len {
        let mut k: uint32_t = 0;
        let mut w: [uint32_t; 3] = [0; 3];
        if !(*t).apm.is_null() && i.wrapping_add(1 as uint64_t) < len
            && ((*p.offset(i as isize) as ::core::ffi::c_int
                | *p.offset(i.wrapping_add(1 as uint64_t) as isize)
                    as ::core::ffi::c_int) as ::core::ffi::c_uint)
                < 0x80 as ::core::ffi::c_uint && (i > 0 as uint64_t || wp == 0)
        {
            memcpy(
                &raw mut w as *mut uint32_t as *mut ::core::ffi::c_void,
                (*t)
                    .apm
                    .offset(TOKS_APM_PAIRS as isize)
                    .offset(
                        (16 as uint32_t)
                            .wrapping_mul(
                                (*p.offset(i as isize) as uint32_t)
                                    << 7 as ::core::ffi::c_int
                                    | *p.offset(i.wrapping_add(1 as uint64_t) as isize)
                                        as uint32_t,
                            ) as isize,
                    ) as *const ::core::ffi::c_void,
                12 as size_t,
            );
            let mut bf: uint32_t = if i > 0 as uint64_t {
                (1 as uint32_t)
                    << *(*t)
                        .apm
                        .offset(
                            *p.offset(i.wrapping_sub(1 as uint64_t) as isize) as isize,
                        ) as ::core::ffi::c_int
            } else {
                0 as uint32_t
            };
            let mut af: uint32_t = if i.wrapping_add(2 as uint64_t) < len {
                (1 as uint32_t)
                    << *(*t)
                        .apm
                        .offset(
                            *p.offset(i.wrapping_add(2 as uint64_t) as isize) as isize,
                        ) as ::core::ffi::c_int
            } else {
                0 as uint32_t
            };
            if w[0 as ::core::ffi::c_int as usize] != UINT32_MAX as uint32_t
                && w[1 as ::core::ffi::c_int as usize] & af
                    | w[2 as ::core::ffi::c_int as usize] & bf == 0 as uint32_t
            {
                if pending != TOKS_SPM_NONE as uint32_t {
                    let fresh1 = n;
                    n = n.wrapping_add(1);
                    *sym.offset(fresh1 as isize) = pending;
                    pending = TOKS_SPM_NONE as uint32_t;
                }
                let fresh2 = n;
                n = n.wrapping_add(1);
                *sym.offset(fresh2 as isize) = w[0 as ::core::ffi::c_int as usize];
                i = i.wrapping_add(2 as uint64_t);
                continue;
            }
        }
        let mut id: uint32_t = entry_at(
            s,
            p.offset(i as isize),
            len.wrapping_sub(i),
            &raw mut k,
        ) & TOKS_SPM_E_ID as uint32_t;
        if id != TOKS_SPM_E_NOID as uint32_t {
            if pending != TOKS_SPM_NONE as uint32_t {
                let fresh3 = n;
                n = n.wrapping_add(1);
                *sym.offset(fresh3 as isize) = pending;
                pending = TOKS_SPM_NONE as uint32_t;
            }
            let fresh4 = n;
            n = n.wrapping_add(1);
            *sym.offset(fresh4 as isize) = id;
            i = i.wrapping_add(k as uint64_t);
        } else {
            if (*s).sflags & TOKS_SPM_BYTE_FALLBACK as uint32_t != 0 {
                let mut all: uint32_t = 1 as uint32_t;
                let mut j: uint32_t = 0 as uint32_t;
                while j < k {
                    if *(*t)
                        .byte2id
                        .offset(
                            *p.offset(i.wrapping_add(j as uint64_t) as isize) as isize,
                        ) == TOKS_SPM_NONE as uint32_t
                    {
                        all = 0 as uint32_t;
                    }
                    j = j.wrapping_add(1);
                }
                if all != 0 {
                    let mut j_0: uint32_t = 0 as uint32_t;
                    while j_0 < k {
                        let fresh5 = n;
                        n = n.wrapping_add(1);
                        *sym.offset(fresh5 as isize) = *(*t)
                            .byte2id
                            .offset(
                                *p.offset(i.wrapping_add(j_0 as uint64_t) as isize) as isize,
                            );
                        j_0 = j_0.wrapping_add(1);
                    }
                    i = i.wrapping_add(k as uint64_t);
                    continue;
                }
            }
            if (*s).unk_id != TOKS_SPM_NONE as uint32_t {
                if pending == TOKS_SPM_NONE as uint32_t
                    || (*s).sflags & TOKS_SPM_FUSE_UNK as uint32_t == 0
                {
                    if pending != TOKS_SPM_NONE as uint32_t {
                        let fresh6 = n;
                        n = n.wrapping_add(1);
                        *sym.offset(fresh6 as isize) = pending;
                    }
                    pending = (*s).unk_id;
                }
            }
            i = i.wrapping_add(k as uint64_t);
        }
    }
    if pending != TOKS_SPM_NONE as uint32_t {
        let fresh7 = n;
        n = n.wrapping_add(1);
        *sym.offset(fresh7 as isize) = pending;
    }
    return n;
}
unsafe extern "C" fn model(
    mut t: *const toks_tables,
    mut s: *const toks_spm,
    mut p: *const uint8_t,
    mut len: uint64_t,
    mut wp: ::core::ffi::c_int,
    mut work: *mut uint8_t,
) -> uint64_t {
    let mut id: uint32_t = 0;
    let mut ids: *mut uint32_t = work as *mut ::core::ffi::c_void as *mut uint32_t;
    if (*t).flags & TOKS_TF_IGNORE_MERGES as uint32_t != 0 && wp == 0
        && len != 0 as uint64_t && bpe_vhash_find(t, p, len, &raw mut id) != 0
    {
        *ids.offset(0 as ::core::ffi::c_int as isize) = id;
        return 1 as uint64_t;
    }
    if (*t).flags & TOKS_TF_ASM_MERGE as uint32_t != 0 as uint32_t
        && len < 128 as uint64_t
    {
        // symbols writes exactly n entries; K6 initializes its own work region.
        // Keep the assembly's 64-byte alignment without clearing either array.
        let mut sym = crate::Aligned([std::mem::MaybeUninit::<u32>::uninit(); 192]);
        let mut kw = crate::Aligned(std::mem::MaybeUninit::<[u32; 1088]>::uninit());
        let n = symbols(t, s, p, len, wp, sym.as_mut_ptr().cast());
        if n == 0 {
            return 0;
        }
        if n == 1 {
            ids.write(sym[0].assume_init());
            return 1;
        }
        let mut a = toks_k6_args {
            piece: sym.as_ptr().cast(),
            len: n,
            out: ids,
            work: kw.as_mut_ptr().cast(),
            work_bytes: std::mem::size_of::<[u32; 1088]>() as u64,
            n_out: 0,
            merges: 0,
            rsv: 0,
        };
        return toks_k6_merge_neon(t, &raw mut a);
    }
    let mut n_0: uint64_t = symbols(
        t,
        s,
        p,
        len,
        wp,
        work
            .offset(
                (16 as uint64_t).wrapping_mul(len.wrapping_add(1 as uint64_t)) as isize,
            ) as *mut ::core::ffi::c_void as *mut uint32_t,
    );
    return if n_0 == 0 as uint64_t {
        0 as uint64_t
    } else {
        toks_k6_merge(t, work, len.wrapping_add(1 as uint64_t), n_0, ids)
    };
}
#[no_mangle]
pub unsafe extern "C" fn toks_spm_model(
    mut t: *const toks_tables,
    mut s: *const toks_spm,
    mut p: *const uint8_t,
    mut len: uint64_t,
    mut out: *mut uint32_t,
    mut work: *mut uint8_t,
) -> uint64_t {
    let mut a: *mut uint8_t = work
        .offset(
            ((8 as ::core::ffi::c_uint as uintptr_t)
                .wrapping_sub(work as uintptr_t & 7 as ::core::ffi::c_uint as uintptr_t)
                & 7 as ::core::ffi::c_uint as uintptr_t) as isize,
        );
    let mut m: uint64_t = model(t, s, p, len, 0 as ::core::ffi::c_int, a);
    memcpy(
        out as *mut ::core::ffi::c_void,
        a as *const ::core::ffi::c_void,
        m.wrapping_mul(4 as uint64_t) as size_t,
    );
    return m;
}
pub const SPM_WIDE_LEN: ::core::ffi::c_uint = 30 as ::core::ffi::c_uint;
pub const SPM_WIDE_IDS: ::core::ffi::c_uint = 7 as ::core::ffi::c_uint;
#[inline]
unsafe extern "C" fn spm_bucket(mut k: bpe_key, mut cm: uint64_t) -> uint64_t {
    return toks_spm_whash(k.lo, k.hi) as uint64_t & cm;
}
unsafe extern "C" fn wide_key(
    mut p: *const uint8_t,
    mut len: uint64_t,
    mut end: *const uint8_t,
    mut wp: ::core::ffi::c_int,
    mut sk: bpe_key,
) -> wkey {
    let mut w: wkey = wkey { q: [0; 4] };
    let mut mark: uint64_t = ((0x40 as ::core::ffi::c_uint
        | (if wp != 0 { 0x20 as ::core::ffi::c_uint } else { 0 as ::core::ffi::c_uint }))
        as uint64_t) << 56 as ::core::ffi::c_int;
    let mut tail: uint64_t = 0 as uint64_t;
    w.q[2 as ::core::ffi::c_int as usize] = 0 as uint64_t;
    if len > TOKS_KEY_MAXLEN as uint64_t {
        let mut avail: uint64_t = end.offset_from(p) as ::core::ffi::c_long as uint64_t;
        let mut b: bpe_key = bpe_key_at(
            p.offset(15 as ::core::ffi::c_int as isize),
            avail.wrapping_sub(15 as uint64_t),
            0 as uint64_t,
            len.wrapping_sub(15 as uint64_t),
        );
        sk = bpe_key_at(p, avail, 0 as uint64_t, 15 as uint64_t);
        w.q[2 as ::core::ffi::c_int as usize] = b.lo;
        tail = b.hi & 0xffffffffffffff as uint64_t;
    }
    w.q[0 as ::core::ffi::c_int as usize] = sk.lo;
    w.q[1 as ::core::ffi::c_int as usize] = sk.hi & 0xffffffffffffff as uint64_t | mark;
    w.q[3 as ::core::ffi::c_int as usize] = tail
        | (0x40 as uint64_t | len) << 56 as ::core::ffi::c_int;
    return w;
}
#[inline]
unsafe extern "C" fn wide_bucket(mut w: *const wkey, mut cm: uint64_t) -> uint64_t {
    let mut x: uint64_t = ((*w).q[0 as ::core::ffi::c_int as usize]
        as ::core::ffi::c_ulonglong
        ^ ((*w).q[1 as ::core::ffi::c_int as usize] as ::core::ffi::c_ulonglong)
            .wrapping_mul(TOKS_FIB64))
        .wrapping_mul(0xd6e8feb86659fd93 as ::core::ffi::c_ulonglong) as uint64_t;
    let mut y: uint64_t = ((*w).q[2 as ::core::ffi::c_int as usize]
        as ::core::ffi::c_ulonglong
        ^ ((*w).q[3 as ::core::ffi::c_int as usize] as ::core::ffi::c_ulonglong)
            .wrapping_mul(TOKS_FIB64))
        .wrapping_mul(0x9e3779b97f4a7c15 as ::core::ffi::c_ulonglong) as uint64_t;
    return (x ^ y) >> 32 as ::core::ffi::c_int & cm;
}
#[inline]
unsafe extern "C" fn put_ids(
    mut out: *mut uint32_t,
    mut cap: uint64_t,
    mut n: uint64_t,
    mut ids: *const uint32_t,
    mut m: uint64_t,
) -> uint64_t {
    let mut j: uint64_t = 0 as uint64_t;
    while j < m {
        if n.wrapping_add(j) < cap {
            toks_st32(
                out.offset(n as isize).offset(j as isize) as *mut ::core::ffi::c_void,
                *ids.offset(j as isize),
            );
        }
        j = j.wrapping_add(1);
    }
    return n.wrapping_add(m);
}
#[inline]
unsafe extern "C" fn put_wide(
    mut v: *const uint8_t,
    mut out: *mut uint32_t,
    mut cap: uint64_t,
    mut n: uint64_t,
) -> uint64_t {
    let mut val: [uint32_t; 8] = [0; 8];
    memcpy(
        &raw mut val as *mut uint32_t as *mut ::core::ffi::c_void,
        v as *const ::core::ffi::c_void,
        32 as size_t,
    );
    let mut m: uint32_t = bpe_val_count(&raw mut val as *mut uint32_t);
    let mut j: uint32_t = 0 as uint32_t;
    while j < 8 as uint32_t {
        val[j as usize] = (val[j as usize] as ::core::ffi::c_uint & TOKS_ID_MASK)
            as uint32_t;
        j = j.wrapping_add(1);
    }
    if n.wrapping_add(8 as uint64_t) <= cap {
        toks_cpy(
            out.offset(n as isize) as *mut ::core::ffi::c_void,
            &raw mut val as *mut uint32_t as *const ::core::ffi::c_void,
            32 as uint64_t,
        );
        return n.wrapping_add(m as uint64_t);
    }
    let mut j_0: uint32_t = 0 as uint32_t;
    while j_0 < m {
        if n.wrapping_add(j_0 as uint64_t) < cap {
            toks_st32(
                out.offset(n as isize).offset(j_0 as isize) as *mut ::core::ffi::c_void,
                val[j_0 as usize],
            );
        }
        j_0 = j_0.wrapping_add(1);
    }
    return n.wrapping_add(m as uint64_t);
}
#[inline(never)]
unsafe extern "C" fn word_slow(
    mut t: *const toks_tables,
    mut s: *const toks_spm,
    mut p: *const uint8_t,
    mut len: uint64_t,
    mut end: *const uint8_t,
    mut wp: ::core::ffi::c_int,
    mut out: *mut uint32_t,
    mut cap: uint64_t,
    mut n: uint64_t,
    mut cache: *mut uint8_t,
    mut cm: uint64_t,
    mut bucket: *mut uint8_t,
    mut k: bpe_key,
    mut tw: uint64_t,
    mut work: *mut uint8_t,
) -> uint64_t {
    let mut W: wkey = wkey { q: [0; 4] };
    let mut wb: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    if !cache.is_null() && len <= SPM_WIDE_LEN as uint64_t && len != 0 as uint64_t {
        W = wide_key(p, len, end, wp, k);
        wb = if !bucket.is_null() {
            bucket
        } else {
            cache
                .offset(
                    wide_bucket(&raw mut W, cm).wrapping_mul(TOKS_BUCKET as uint64_t)
                        as isize,
                )
        };
        let mut q: [uint64_t; 4] = [0; 4];
        let mut g: uint64_t = 0;
        memcpy(
            &raw mut q as *mut uint64_t as *mut ::core::ffi::c_void,
            wb as *const ::core::ffi::c_void,
            32 as size_t,
        );
        memcpy(
            &raw mut g as *mut ::core::ffi::c_void,
            wb.offset(40 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_void,
            8 as size_t,
        );
        if q[0 as ::core::ffi::c_int as usize] == W.q[0 as ::core::ffi::c_int as usize]
            && q[1 as ::core::ffi::c_int as usize]
                == W.q[1 as ::core::ffi::c_int as usize]
            && q[2 as ::core::ffi::c_int as usize]
                == W.q[2 as ::core::ffi::c_int as usize]
            && q[3 as ::core::ffi::c_int as usize]
                == W.q[3 as ::core::ffi::c_int as usize]
            && g as ::core::ffi::c_ulonglong & TOKS_TAG_MASK64
                == tw as ::core::ffi::c_ulonglong
        {
            return put_wide(wb.offset(32 as ::core::ffi::c_int as isize), out, cap, n);
        }
    }
    if !bucket.is_null() && !(*t).words.is_null() {
        let mut v: *const uint8_t = bpe_words_probe(
            (*t).words,
            (*t).words_mask,
            toks_spm_whash(k.lo, k.hi),
            k,
        );
        if !v.is_null() {
            let mut val: [uint32_t; 4] = [0; 4];
            let mut sid: [uint32_t; 4] = [0; 4];
            memcpy(
                &raw mut val as *mut uint32_t as *mut ::core::ffi::c_void,
                v as *const ::core::ffi::c_void,
                16 as size_t,
            );
            let mut j: uint32_t = 0 as uint32_t;
            while j < 4 as uint32_t {
                sid[j as usize] = val[j as usize] & TOKS_ID_MASK as uint32_t;
                j = j.wrapping_add(1);
            }
            bpe_val_pack_tag(
                &raw mut val as *mut uint32_t,
                &raw mut sid as *mut uint32_t,
                bpe_val_count(&raw mut val as *mut uint32_t),
                tw,
            );
            bpe_cache_fill(bucket, k, &raw mut val as *mut uint32_t as *const uint32_t);
            return put_ids(
                out,
                cap,
                n,
                &raw mut sid as *mut uint32_t,
                bpe_val_count(&raw mut val as *mut uint32_t) as uint64_t,
            ) | (1 as uint64_t) << 63 as ::core::ffi::c_int;
        }
    }
    let mut m: uint64_t = model(t, s, p, len, wp, work);
    let mut ids: *const uint32_t = work as *const ::core::ffi::c_void as *const uint32_t;
    if !bucket.is_null() && m >= 1 as uint64_t && m <= 4 as uint64_t {
        let mut val_0: [uint32_t; 4] = [0; 4];
        bpe_val_pack_tag(&raw mut val_0 as *mut uint32_t, ids, m as uint32_t, tw);
        bpe_cache_fill(bucket, k, &raw mut val_0 as *mut uint32_t as *const uint32_t);
    } else if !wb.is_null() && m >= 1 as uint64_t && m <= SPM_WIDE_IDS as uint64_t {
        let mut val_1: [uint32_t; 8] = [
            0 as ::core::ffi::c_uint,
            0 as ::core::ffi::c_uint,
            0 as ::core::ffi::c_uint,
            0 as ::core::ffi::c_uint,
            0 as ::core::ffi::c_uint,
            0 as ::core::ffi::c_uint,
            0 as ::core::ffi::c_uint,
            0 as ::core::ffi::c_uint,
        ];
        let mut j_0: uint64_t = 0 as uint64_t;
        while j_0 < m {
            val_1[j_0 as usize] = *ids.offset(j_0 as isize) & TOKS_ID_MASK as uint32_t;
            j_0 = j_0.wrapping_add(1);
        }
        val_1[0 as ::core::ffi::c_int as usize]
            |= (m as uint32_t) << TOKS_VAL_COUNT_SHIFT;
        val_1[2 as ::core::ffi::c_int as usize] |= tw as uint32_t;
        val_1[3 as ::core::ffi::c_int as usize]
            |= (tw >> 32 as ::core::ffi::c_int) as uint32_t;
        memcpy(
            wb as *mut ::core::ffi::c_void,
            &raw mut W.q as *mut uint64_t as *const ::core::ffi::c_void,
            32 as size_t,
        );
        memcpy(
            wb.offset(32 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_void,
            &raw mut val_1 as *mut uint32_t as *const ::core::ffi::c_void,
            32 as size_t,
        );
    }
    return put_ids(out, cap, n, ids, m);
}
#[inline]
unsafe extern "C" fn skey(
    mut p: *const uint8_t,
    mut len: uint64_t,
    mut end: *const uint8_t,
    mut wp: ::core::ffi::c_int,
) -> bpe_key {
    let mut k: bpe_key = bpe_key {
        lo: 0 as uint64_t,
        hi: 0 as uint64_t,
    };
    if len != 0 as uint64_t {
        k = bpe_key_at(
            p,
            end.offset_from(p) as ::core::ffi::c_long as uint64_t,
            0 as uint64_t,
            len,
        );
    }
    if wp != 0 {
        k.hi = (k.hi as ::core::ffi::c_ulong
            | (0x80 as ::core::ffi::c_ulong) << 56 as ::core::ffi::c_int) as uint64_t;
    }
    return k;
}
#[inline(always)]
unsafe extern "C" fn word_k(
    mut t: *const toks_tables,
    mut s: *const toks_spm,
    mut p: *const uint8_t,
    mut len: uint64_t,
    mut end: *const uint8_t,
    mut wp: ::core::ffi::c_int,
    mut out: *mut uint32_t,
    mut cap: uint64_t,
    mut n: uint64_t,
    mut cache: *mut uint8_t,
    mut cm: uint64_t,
    mut tw: uint64_t,
    mut work: *mut uint8_t,
    mut miss: *mut uint32_t,
    mut k: bpe_key,
) -> uint64_t {
    let mut bucket: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    if !cache.is_null() && len <= TOKS_KEY_MAXLEN as uint64_t {
        bucket = cache
            .offset(spm_bucket(k, cm).wrapping_mul(TOKS_BUCKET as uint64_t) as isize);
        let mut v: *const uint8_t = bpe_cache_get(bucket, k, tw);
        if !v.is_null() {
            if n.wrapping_add(4 as uint64_t) <= cap {
                return n
                    .wrapping_add(
                        bpe_val_put(
                            v,
                            out.offset(n as isize) as *mut ::core::ffi::c_void,
                        ),
                    );
            }
            let mut val: [uint32_t; 4] = [0; 4];
            memcpy(
                &raw mut val as *mut uint32_t as *mut ::core::ffi::c_void,
                v as *const ::core::ffi::c_void,
                16 as size_t,
            );
            let mut m: uint32_t = bpe_val_count(&raw mut val as *mut uint32_t);
            let mut j: uint32_t = 0 as uint32_t;
            while j < m {
                if n.wrapping_add(j as uint64_t) < cap {
                    toks_st32(
                        out.offset(n as isize).offset(j as isize)
                            as *mut ::core::ffi::c_void,
                        val[j as usize] & TOKS_ID_MASK as uint32_t,
                    );
                }
                j = j.wrapping_add(1);
            }
            return n.wrapping_add(m as uint64_t);
        }
    }
    let mut r: uint64_t = word_slow(
        t,
        s,
        p,
        len,
        end,
        wp,
        out,
        cap,
        n,
        cache,
        cm,
        bucket,
        k,
        tw,
        work,
    );
    *miss = (*miss).wrapping_add((r >> 63 as ::core::ffi::c_int) as uint32_t);
    return r & !((1 as uint64_t) << 63 as ::core::ffi::c_int);
}
#[inline(always)]
unsafe extern "C" fn word(
    mut t: *const toks_tables,
    mut s: *const toks_spm,
    mut p: *const uint8_t,
    mut len: uint64_t,
    mut end: *const uint8_t,
    mut wp: ::core::ffi::c_int,
    mut out: *mut uint32_t,
    mut cap: uint64_t,
    mut n: uint64_t,
    mut cache: *mut uint8_t,
    mut cm: uint64_t,
    mut tw: uint64_t,
    mut work: *mut uint8_t,
    mut miss: *mut uint32_t,
) -> uint64_t {
    let mut k: bpe_key = bpe_key {
        lo: 0 as uint64_t,
        hi: 0 as uint64_t,
    };
    if !cache.is_null() && len <= TOKS_KEY_MAXLEN as uint64_t {
        k = skey(p, len, end, wp);
    }
    return word_k(t, s, p, len, end, wp, out, cap, n, cache, cm, tw, work, miss, k);
}
#[inline(never)]
unsafe extern "C" fn window_batch(
    mut t: *const toks_tables,
    mut s: *const toks_spm,
    mut text: *const uint8_t,
    mut len: uint64_t,
    mut base: uint64_t,
    mut m: uint64_t,
    mut x: uint64_t,
    mut one: *const uint32_t,
    mut wsp: *mut uint64_t,
    mut wpp: *mut ::core::ffi::c_int,
    mut out: *mut uint32_t,
    mut cap: uint64_t,
    mut n: uint64_t,
    mut c: *mut uint8_t,
    mut cm: uint64_t,
    mut tw: uint64_t,
    mut w: *mut uint8_t,
    mut miss: *mut uint32_t,
) -> uint64_t {
    let mut ws: uint64_t = *wsp;
    let mut wp: ::core::ffi::c_int = *wpp;
    let mut ms: uint32_t = 0 as uint32_t;
    while m != 0 as uint64_t {
        let mut wst: [uint64_t; 32] = [0; 32];
        let mut wk: [bpe_key; 32] = [bpe_key { lo: 0, hi: 0 }; 32];
        let mut nw: uint32_t = 0 as uint32_t;
        let mut one_w: uint32_t = 0 as uint32_t;
        while m != 0 as uint64_t && nw < 32 as uint32_t {
            let mut at: uint64_t = base
                .wrapping_add(
                    (m as ::core::ffi::c_ulonglong).trailing_zeros() as i32 as uint64_t,
                );
            m &= m.wrapping_sub(1 as uint64_t);
            let mut wl: uint64_t = at.wrapping_sub(ws);
            wst[nw as usize] = ws;
            let mut o: uint32_t = if x != 0 as uint64_t
                && *text.offset(ws as isize) as ::core::ffi::c_uint
                    >= 0xc0 as ::core::ffi::c_uint && ws >= base
            {
                *one.offset(ws.wrapping_sub(base) as isize)
            } else {
                0 as uint32_t
            };
            if (o >> 24 as ::core::ffi::c_int) as uint64_t == wl
                && !(wp != 0 && nw == 0 as uint32_t)
                && o & TOKS_SPM_E_ID as uint32_t != TOKS_SPM_E_NOID as uint32_t
            {
                one_w = (one_w as ::core::ffi::c_uint | (1 as ::core::ffi::c_uint) << nw)
                    as uint32_t;
                wk[nw as usize].lo = (o & TOKS_SPM_E_ID as uint32_t) as uint64_t;
            } else if wl <= TOKS_KEY_MAXLEN as uint64_t {
                wk[nw as usize] = skey(
                    text.offset(ws as isize),
                    wl,
                    text.offset(len as isize),
                    (wp != 0 && nw == 0 as uint32_t) as ::core::ffi::c_int,
                );
                let mut h: uint32_t = toks_spm_whash(
                    wk[nw as usize].lo,
                    wk[nw as usize].hi,
                );
                crate::prefetch_read(c.add(((h as u64 & cm) * TOKS_BUCKET as u64) as usize));
                if !(*t).words.is_null() {
                    crate::prefetch_read((*t).words.add(((h as u64 & (*t).words_mask) * TOKS_BUCKET as u64) as usize));
                }
            }
            nw = nw.wrapping_add(1);
            ws = at;
        }
        let mut j: uint32_t = 0 as uint32_t;
        while j < nw {
            let mut a0: uint64_t = wst[j as usize];
            let mut e: uint64_t = if j.wrapping_add(1 as uint32_t) < nw {
                wst[j.wrapping_add(1 as uint32_t) as usize]
            } else {
                ws
            };
            if one_w >> j & 1 as uint32_t != 0 {
                if n < cap {
                    toks_st32(
                        out.offset(n as isize) as *mut ::core::ffi::c_void,
                        wk[j as usize].lo as uint32_t,
                    );
                }
                n = n.wrapping_add(1);
            } else {
                n = word_k(
                    t,
                    s,
                    text.offset(a0 as isize),
                    e.wrapping_sub(a0),
                    text.offset(len as isize),
                    wp,
                    out,
                    cap,
                    n,
                    c,
                    cm,
                    tw,
                    w,
                    &raw mut ms,
                    wk[j as usize],
                );
            }
            wp = 0 as ::core::ffi::c_int;
            j = j.wrapping_add(1);
        }
    }
    *wsp = ws;
    *wpp = wp;
    *miss = ms;
    return n;
}
#[inline(always)]
#[no_mangle]
pub unsafe extern "C" fn toks_k7_spm_c(
    mut s: *const toks_spm,
    mut a: *mut toks_k7_args,
) -> uint64_t {
    let mut text: *const uint8_t = (*a).text;
    let mut len: uint64_t = (*a).len;
    let mut i: uint64_t = (*a).pos;
    let mut base: uint64_t = i;
    let mut m: uint64_t = 0 as uint64_t;
    let mut cuts: uint64_t = (*a).flags & 1 as uint64_t;
    let mut prev: uint32_t = (*a).prev as uint32_t;
    let mut mb: uint32_t = 0 as uint32_t;
    let mut k: uint32_t = 0;
    let mut one: *mut uint32_t = (*a).one;
    while i < len && i.wrapping_sub(base) <= 56 as uint64_t {
        if cuts != 0 && i != 0 as uint64_t && len.wrapping_sub(i) >= 8 as uint64_t
            && (*text.offset(i.wrapping_sub(1 as uint64_t) as isize)
                as ::core::ffi::c_uint) < 0x80 as ::core::ffi::c_uint
        {
            let mut v: uint64_t = 0;
            memcpy(
                &raw mut v as *mut ::core::ffi::c_void,
                text.offset(i as isize) as *const ::core::ffi::c_void,
                8 as size_t,
            );
            if v & 0x8080808080808080 as uint64_t == 0 as uint64_t {
                m
                    |= cut8(
                        &raw const (*s).cut_ab as *const uint32_t,
                        text
                            .offset(i as isize)
                            .offset(-(1 as ::core::ffi::c_uint as isize)),
                    ) << i.wrapping_sub(base);
                i = i.wrapping_add(8 as uint64_t);
                continue;
            }
        }
        if i != 0 as uint64_t
            && (*text.offset(i.wrapping_sub(1 as uint64_t) as isize)
                as ::core::ffi::c_uint) < 0x80 as ::core::ffi::c_uint
        {
            prev = (*s)
                .ascii[*text.offset(i.wrapping_sub(1 as uint64_t) as isize) as usize];
        }
        let mut e: uint32_t = entry_at(
            s,
            text.offset(i as isize),
            len.wrapping_sub(i),
            &raw mut k,
        );
        *one.offset(i.wrapping_sub(base) as isize) = e & TOKS_SPM_E_ID as uint32_t
            | k << 24 as ::core::ffi::c_int;
        mb |= k >> 1 as ::core::ffi::c_int;
        let mut cut: ::core::ffi::c_int = ((*s).ms_split != 0
            && e & TOKS_SPM_E_ID as uint32_t == (*s).id_repl
            || cuts != 0 && toks_spm_cut_between(s, prev, e) != 0) as ::core::ffi::c_int;
        m |= (cut as uint64_t) << i.wrapping_sub(base);
        prev = e;
        i = i.wrapping_add(k as uint64_t);
    }
    (*a).pos = i;
    (*a).prev = prev as uint64_t;
    (*a).mb = mb as uint64_t;
    return m;
}
#[no_mangle]
pub unsafe extern "C" fn toks_spm_encode(
    mut t: *const toks_tables,
    mut s: *const toks_spm,
    mut text: *const uint8_t,
    mut len: uint64_t,
    mut at_start: ::core::ffi::c_int,
    mut out: *mut uint32_t,
    mut cap: uint64_t,
    mut n: uint64_t,
    mut cache: *mut uint8_t,
    mut cm: uint64_t,
    mut tag: uint64_t,
    mut work: *mut uint8_t,
    mut flags: uint32_t,
) -> uint64_t {
    if len == 0 as uint64_t {
        return n;
    }
    let mut w: *mut uint8_t = work
        .offset(
            ((8 as ::core::ffi::c_uint as uintptr_t)
                .wrapping_sub(work as uintptr_t & 7 as ::core::ffi::c_uint as uintptr_t)
                & 7 as ::core::ffi::c_uint as uintptr_t) as isize,
        );
    let mut c: *mut uint8_t = if flags & TOKS_SPM_NOCACHE as uint32_t != 0 {
        ::core::ptr::null_mut::<uint8_t>()
    } else {
        cache
    };
    let mut tw: uint64_t = toks_tag_word(tag);
    let mut k: uint32_t = 0;
    let mut pre: ::core::ffi::c_int = (has_prefix(
        s,
        entry_at(s, text, len, &raw mut k),
        at_start,
    ) != 0 && flags & TOKS_SPM_NOPFX as uint32_t == 0) as ::core::ffi::c_int;
    let mut ws: uint64_t = 0 as uint64_t;
    let mut wp: ::core::ffi::c_int = pre;
    let mut one: [uint32_t; 64] = [0; 64];
    let mut miss: uint32_t = 0 as uint32_t;
    let mut ka: toks_k7_args = toks_k7_args {
        text: text,
        len: len,
        pos: 0 as uint64_t,
        prev: (if pre != 0 { (*s).pfx_entry } else { NOENTRY as uint32_t }) as uint64_t,
        one: &raw mut one as *mut uint32_t,
        mb: 0 as uint64_t,
        flags: (!(*s).pairs.is_null() && flags & TOKS_SPM_NOCUTS as uint32_t == 0)
            as ::core::ffi::c_int as uint64_t,
        rsv: 0 as uint64_t,
    };
    while ka.pos < len {
        let mut base: uint64_t = ka.pos;
        let mut m: uint64_t = if flags >> 8 as ::core::ffi::c_int
            == TOKS_TIER_NEON as uint32_t
        {
            toks_k7_spm_neon(s, &raw mut ka)
        } else {
            0 as uint64_t
        };
        let mut x: uint64_t = 0;
        if ka.pos == base {
            m = toks_k7_spm_c(s, &raw mut ka);
        }
        if base == 0 as uint64_t && pre == 0 {
            m = (m as ::core::ffi::c_ulong & !(1 as ::core::ffi::c_ulong)) as uint64_t;
        }
        x = ka.mb;
        if miss >= 3 as uint32_t && !c.is_null() {
            n = window_batch(
                t,
                s,
                text,
                len,
                base,
                m,
                x,
                &raw mut one as *mut uint32_t,
                &raw mut ws,
                &raw mut wp,
                out,
                cap,
                n,
                c,
                cm,
                tw,
                w,
                &raw mut miss,
            );
        } else {
            miss = 0 as ::core::ffi::c_uint as uint32_t;
            while m != 0 as uint64_t && (x == 0 as uint64_t || c.is_null()) {
                let mut at: uint64_t = base
                    .wrapping_add(
                        (m as ::core::ffi::c_ulonglong).trailing_zeros() as i32
                            as uint64_t,
                    );
                m &= m.wrapping_sub(1 as uint64_t);
                n = word(
                    t,
                    s,
                    text.offset(ws as isize),
                    at.wrapping_sub(ws),
                    text.offset(len as isize),
                    wp,
                    out,
                    cap,
                    n,
                    c,
                    cm,
                    tw,
                    w,
                    &raw mut miss,
                );
                ws = at;
                wp = 0 as ::core::ffi::c_int;
            }
            while m != 0 as uint64_t {
                let mut at_0: uint64_t = base
                    .wrapping_add(
                        (m as ::core::ffi::c_ulonglong).trailing_zeros() as i32
                            as uint64_t,
                    );
                m &= m.wrapping_sub(1 as uint64_t);
                let mut o: uint32_t = if *text.offset(ws as isize) as ::core::ffi::c_uint
                    >= 0xc0 as ::core::ffi::c_uint && ws >= base
                {
                    one[ws.wrapping_sub(base) as usize]
                } else {
                    0 as uint32_t
                };
                if (o >> 24 as ::core::ffi::c_int) as uint64_t == at_0.wrapping_sub(ws)
                    && wp == 0
                    && o & TOKS_SPM_E_ID as uint32_t != TOKS_SPM_E_NOID as uint32_t
                {
                    if n < cap {
                        toks_st32(
                            out.offset(n as isize) as *mut ::core::ffi::c_void,
                            o & TOKS_SPM_E_ID as uint32_t,
                        );
                    }
                    n = n.wrapping_add(1);
                } else {
                    n = word(
                        t,
                        s,
                        text.offset(ws as isize),
                        at_0.wrapping_sub(ws),
                        text.offset(len as isize),
                        wp,
                        out,
                        cap,
                        n,
                        c,
                        cm,
                        tw,
                        w,
                        &raw mut miss,
                    );
                }
                ws = at_0;
                wp = 0 as ::core::ffi::c_int;
            }
        }
    }
    return word(
        t,
        s,
        text.offset(ws as isize),
        len.wrapping_sub(ws),
        text.offset(len as isize),
        wp,
        out,
        cap,
        n,
        c,
        cm,
        tw,
        w,
        &raw mut miss,
    );
}
#[no_mangle]
pub unsafe extern "C" fn toks_spm_pieces(
    mut s: *const toks_spm,
    mut text: *const uint8_t,
    mut len: uint64_t,
    mut at_start: ::core::ffi::c_int,
    mut base: uint64_t,
    mut out: *mut uint32_t,
    mut cap: uint64_t,
    mut n: uint64_t,
) -> uint64_t {
    if len == 0 as uint64_t {
        return n;
    }
    if (*s).ms_split != 0 {
        let mut k: uint32_t = 0;
        let mut e: uint32_t = entry_at(s, text, len, &raw mut k);
        let mut pre: ::core::ffi::c_int = has_prefix(s, e, at_start);
        let mut i: uint64_t = 0 as uint64_t;
        while i < len {
            if i != 0 as uint64_t {
                e = entry_at(
                    s,
                    text.offset(i as isize),
                    len.wrapping_sub(i),
                    &raw mut k,
                );
            }
            if e & TOKS_SPM_E_ID as uint32_t == (*s).id_repl
                && (i != 0 as uint64_t || pre != 0)
            {
                if n < cap {
                    toks_st32(
                        out.offset(n as isize) as *mut ::core::ffi::c_void,
                        base.wrapping_add(i) as uint32_t,
                    );
                }
                n = n.wrapping_add(1);
            }
            i = i.wrapping_add(k as uint64_t);
        }
    }
    if n < cap {
        toks_st32(
            out.offset(n as isize) as *mut ::core::ffi::c_void,
            base.wrapping_add(len) as uint32_t,
        );
    }
    return n.wrapping_add(1 as uint64_t);
}
static mut FFFD: [uint8_t; 3] = [
    0xef as ::core::ffi::c_uint as uint8_t,
    0xbf as ::core::ffi::c_uint as uint8_t,
    0xbd as ::core::ffi::c_uint as uint8_t,
];
unsafe extern "C" fn put_raw(mut d: *mut dec, mut p: *const uint8_t, mut k: uint64_t) {
    if k == 0 as uint64_t {
        return;
    }
    if (*d).total < (*d).cap {
        let mut room: uint64_t = (*d).cap.wrapping_sub((*d).total);
        memcpy(
            (*d).out.offset((*d).total as isize) as *mut ::core::ffi::c_void,
            p as *const ::core::ffi::c_void,
            (if room < k { room } else { k }) as size_t,
        );
    }
    (*d).total = (*d).total.wrapping_add(k);
}
unsafe extern "C" fn put(mut d: *mut dec, mut p: *const uint8_t, mut k: uint64_t) {
    while (*d).strip_left != 0 as uint32_t && k != 0 as uint64_t {
        if k >= (*d).strip.n as uint64_t
            && memcmp(
                p as *const ::core::ffi::c_void,
                &raw mut (*d).strip.b as *mut uint8_t as *const ::core::ffi::c_void,
                (*d).strip.n as size_t,
            ) == 0 as ::core::ffi::c_int
        {
            p = p.offset((*d).strip.n as isize);
            k = k.wrapping_sub((*d).strip.n as uint64_t);
            (*d).strip_left = (*d).strip_left.wrapping_sub(1);
        } else {
            (*d).strip_left = 0 as uint32_t;
        }
    }
    put_raw(d, p, k);
}
unsafe extern "C" fn put_bad(mut d: *mut dec, mut b: uint8_t) {
    if (*d).raw == 0 {
        put(d, &raw const FFFD as *const uint8_t, 3 as uint64_t);
        return;
    }
    if (*d).strip_left != 0 as uint32_t {
        if (*d).strip.n == 3 as uint32_t
            && memcmp(
                &raw mut (*d).strip.b as *mut uint8_t as *const ::core::ffi::c_void,
                &raw const FFFD as *const uint8_t as *const ::core::ffi::c_void,
                3 as size_t,
            ) == 0 as ::core::ffi::c_int
        {
            (*d).strip_left = (*d).strip_left.wrapping_sub(1);
            return;
        }
        (*d).strip_left = 0 as ::core::ffi::c_uint as uint32_t;
    }
    put_raw(d, &raw mut b, 1 as uint64_t);
}
unsafe extern "C" fn put_token(
    mut d: *mut dec,
    mut op: *const toks_spm_op,
    mut p: *const uint8_t,
    mut k: uint64_t,
    mut index: uint64_t,
) {
    if op.is_null() {
        put(d, p, k);
        return;
    }
    static mut SP: uint8_t = ' ' as i32 as uint8_t;
    let mut to: *const uint8_t = &raw const (*op).b.b as *const uint8_t;
    let mut to_n: uint64_t = (*op).b.n as uint64_t;
    if (*op).kind == TOKS_SPM_D_METASPACE as ::core::ffi::c_int as uint32_t {
        to = &raw const SP;
        to_n = (if index == 0 as uint64_t
            && (*op).scheme != TOKS_SPM_PS_NEVER as ::core::ffi::c_int as uint32_t
        {
            0 as ::core::ffi::c_uint
        } else {
            1 as ::core::ffi::c_uint
        }) as uint64_t;
    }
    let mut i: uint64_t = 0 as uint64_t;
    let mut from: uint64_t = 0 as uint64_t;
    while i.wrapping_add((*op).a.n as uint64_t) <= k {
        if *p.offset(i as isize) as ::core::ffi::c_int
            == (*op).a.b[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
            && memcmp(
                p.offset(i as isize) as *const ::core::ffi::c_void,
                &raw const (*op).a.b as *const uint8_t as *const ::core::ffi::c_void,
                (*op).a.n as size_t,
            ) == 0 as ::core::ffi::c_int
        {
            put(d, p.offset(from as isize), i.wrapping_sub(from));
            put(d, to, to_n);
            i = i.wrapping_add((*op).a.n as uint64_t);
            from = i;
        } else {
            i = i.wrapping_add(1);
        }
    }
    put(d, p.offset(from as isize), k.wrapping_sub(from));
}
#[inline]
unsafe extern "C" fn hexd(mut c: uint8_t) -> ::core::ffi::c_int {
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
#[inline]
unsafe extern "C" fn byte_token(
    mut p: *const uint8_t,
    mut k: uint64_t,
) -> ::core::ffi::c_int {
    if k != 6 as uint64_t
        || *p.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            != '<' as i32
        || *p.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            != '0' as i32
        || *p.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            != 'x' as i32
        || *p.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            != '>' as i32
    {
        return -(1 as ::core::ffi::c_int);
    }
    if *p.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '+' as i32 {
        return hexd(*p.offset(4 as ::core::ffi::c_int as isize));
    }
    let mut hi: ::core::ffi::c_int = hexd(*p.offset(3 as ::core::ffi::c_int as isize));
    let mut lo: ::core::ffi::c_int = hexd(*p.offset(4 as ::core::ffi::c_int as isize));
    return if hi < 0 as ::core::ffi::c_int || lo < 0 as ::core::ffi::c_int {
        -(1 as ::core::ffi::c_int)
    } else {
        hi * 16 as ::core::ffi::c_int + lo
    };
}
#[inline]
unsafe extern "C" fn kept(
    mut s: *const toks_spm,
    mut special: *const uint32_t,
    mut skip: ::core::ffi::c_int,
    mut id: uint32_t,
) -> ::core::ffi::c_int {
    if skip != 0 && !special.is_null()
        && *special.offset((id >> 5 as ::core::ffi::c_int) as isize)
            >> (id & 31 as uint32_t) & 1 as uint32_t != 0
    {
        return 0 as ::core::ffi::c_int;
    }
    if !(*s).holes.is_null()
        && *(*s).holes.offset((id >> 5 as ::core::ffi::c_int) as isize)
            >> (id & 31 as uint32_t) & 1 as uint32_t != 0
    {
        return 0 as ::core::ffi::c_int;
    }
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn flush_run(
    mut d: *mut dec,
    mut t: *const toks_tables,
    mut s: *const toks_spm,
    mut special: *const uint32_t,
    mut skip: ::core::ffi::c_int,
    mut ids: *const uint32_t,
    mut from: uint64_t,
    mut to: uint64_t,
    mut nbytes: uint64_t,
    mut valid: ::core::ffi::c_int,
) {
    if valid == 0 && (*d).raw == 0 {
        let mut j: uint64_t = 0 as uint64_t;
        while j < nbytes {
            put(d, &raw const FFFD as *const uint8_t, 3 as uint64_t);
            j = j.wrapping_add(1);
        }
        return;
    }
    let mut ch: [uint8_t; 4] = [0; 4];
    let mut have: uint32_t = 0 as uint32_t;
    let mut need: uint32_t = 0 as uint32_t;
    let mut j_0: uint64_t = from;
    while j_0 < to {
        let mut id: uint32_t = toks_ld32(
            ids.offset(j_0 as isize) as *const ::core::ffi::c_void,
        );
        if !(kept(s, special, skip, id) == 0) {
            let mut o: uint32_t = *(*t).tok_off.offset(id as isize);
            let mut b: uint8_t = byte_token(
                (*t).tok_bytes.offset(o as isize),
                (*(*t).tok_off.offset(id.wrapping_add(1 as uint32_t) as isize))
                    .wrapping_sub(o) as uint64_t,
            ) as uint8_t;
            if valid == 0 {
                put_bad(d, b);
            } else {
                if have == 0 as uint32_t {
                    need = (if (b as ::core::ffi::c_uint) < 0x80 as ::core::ffi::c_uint {
                        1 as ::core::ffi::c_uint
                    } else if (b as ::core::ffi::c_uint) < 0xe0 as ::core::ffi::c_uint {
                        2 as ::core::ffi::c_uint
                    } else if (b as ::core::ffi::c_uint) < 0xf0 as ::core::ffi::c_uint {
                        3 as ::core::ffi::c_uint
                    } else {
                        4 as ::core::ffi::c_uint
                    }) as uint32_t;
                }
                let fresh8 = have;
                have = have.wrapping_add(1);
                ch[fresh8 as usize] = b;
                if have == need {
                    put(d, &raw mut ch as *mut uint8_t, have as uint64_t);
                    have = 0 as uint32_t;
                }
            }
        }
        j_0 = j_0.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn toks_spm_decode(
    mut t: *const toks_tables,
    mut s: *const toks_spm,
    mut special: *const uint32_t,
    mut flags: uint32_t,
    mut ids: *const uint32_t,
    mut n: uint64_t,
    mut out: *mut uint8_t,
    mut cap: uint64_t,
) -> int64_t {
    let mut d: dec = dec {
        out: ::core::ptr::null_mut::<uint8_t>(),
        cap: 0,
        total: 0,
        strip_left: 0,
        strip: toks_spm_str { b: [0; 16], n: 0 },
        raw: 0,
    };
    memset(
        &raw mut d as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<dec>() as size_t,
    );
    d.out = out;
    d.cap = cap;
    d.raw = (flags & TOKS_DECODE_RAW as uint32_t != 0 as uint32_t) as ::core::ffi::c_int;
    let mut skip: ::core::ffi::c_int = (flags & TOKS_SKIP_SPECIAL as uint32_t
        != 0 as uint32_t) as ::core::ffi::c_int;
    let mut per_token: *const toks_spm_op = ::core::ptr::null::<toks_spm_op>();
    let mut bf: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut i: uint32_t = 0 as uint32_t;
    while i < (*s).n_dec {
        let mut op: *const toks_spm_op = (&raw const (*s).dec as *const toks_spm_op)
            .offset(i as isize) as *const toks_spm_op;
        if (*op).kind == TOKS_SPM_D_REPLACE as ::core::ffi::c_int as uint32_t
            || (*op).kind == TOKS_SPM_D_METASPACE as ::core::ffi::c_int as uint32_t
        {
            per_token = op;
        } else if (*op).kind
            == TOKS_SPM_D_BYTE_FALLBACK as ::core::ffi::c_int as uint32_t
        {
            bf = 1 as ::core::ffi::c_int;
        } else if (*op).kind == TOKS_SPM_D_STRIP as ::core::ffi::c_int as uint32_t {
            d.strip_left = (*op).start;
            d.strip = (*op).a;
        }
        i = i.wrapping_add(1);
    }
    let mut index: uint64_t = 0 as uint64_t;
    let mut run_from: uint64_t = 0 as uint64_t;
    let mut run_n: uint64_t = 0 as uint64_t;
    let mut need: uint32_t = 0 as uint32_t;
    let mut got: uint32_t = 0 as uint32_t;
    let mut lo: uint8_t = 0x80 as uint8_t;
    let mut hi: uint8_t = 0xbf as uint8_t;
    let mut valid: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut i_0: uint64_t = 0 as uint64_t;
    while i_0 < n {
        let mut id: uint32_t = toks_ld32(
            ids.offset(i_0 as isize) as *const ::core::ffi::c_void,
        );
        if !(kept(s, special, skip, id) == 0) {
            let mut o: uint32_t = *(*t).tok_off.offset(id as isize);
            let mut p: *const uint8_t = (*t).tok_bytes.offset(o as isize);
            let mut k: uint64_t = (*(*t)
                .tok_off
                .offset(id.wrapping_add(1 as uint32_t) as isize))
                .wrapping_sub(o) as uint64_t;
            if (*s).has_decoder == 0 {
                if index != 0 as uint64_t {
                    put_raw(
                        &raw mut d,
                        b" \0" as *const u8 as *const ::core::ffi::c_char
                            as *const uint8_t,
                        1 as uint64_t,
                    );
                }
                put_raw(&raw mut d, p, k);
                index = index.wrapping_add(1);
            } else {
                let mut b: ::core::ffi::c_int = if bf != 0 {
                    byte_token(p, k)
                } else {
                    -(1 as ::core::ffi::c_int)
                };
                if b >= 0 as ::core::ffi::c_int {
                    if run_n == 0 as uint64_t {
                        run_from = i_0;
                        valid = 1 as ::core::ffi::c_int;
                        need = 0 as uint32_t;
                        got = 0 as uint32_t;
                    }
                    run_n = run_n.wrapping_add(1);
                    index = index.wrapping_add(1);
                    if !(valid == 0) {
                        let mut c: uint8_t = b as uint8_t;
                        if need == 0 as uint32_t {
                            if !((c as ::core::ffi::c_uint)
                                < 0x80 as ::core::ffi::c_uint)
                            {
                                if (c as ::core::ffi::c_uint) < 0xc2 as ::core::ffi::c_uint
                                    || c as ::core::ffi::c_uint > 0xf4 as ::core::ffi::c_uint
                                {
                                    valid = 0 as ::core::ffi::c_int;
                                } else {
                                    need = (if (c as ::core::ffi::c_uint)
                                        < 0xe0 as ::core::ffi::c_uint
                                    {
                                        1 as ::core::ffi::c_uint
                                    } else if (c as ::core::ffi::c_uint)
                                        < 0xf0 as ::core::ffi::c_uint
                                    {
                                        2 as ::core::ffi::c_uint
                                    } else {
                                        3 as ::core::ffi::c_uint
                                    }) as uint32_t;
                                    got = 0 as uint32_t;
                                    lo = (if c as ::core::ffi::c_uint
                                        == 0xe0 as ::core::ffi::c_uint
                                    {
                                        0xa0 as ::core::ffi::c_uint
                                    } else if c as ::core::ffi::c_uint
                                        == 0xf0 as ::core::ffi::c_uint
                                    {
                                        0x90 as ::core::ffi::c_uint
                                    } else {
                                        0x80 as ::core::ffi::c_uint
                                    }) as uint8_t;
                                    hi = (if c as ::core::ffi::c_uint
                                        == 0xed as ::core::ffi::c_uint
                                    {
                                        0x9f as ::core::ffi::c_uint
                                    } else if c as ::core::ffi::c_uint
                                        == 0xf4 as ::core::ffi::c_uint
                                    {
                                        0x8f as ::core::ffi::c_uint
                                    } else {
                                        0xbf as ::core::ffi::c_uint
                                    }) as uint8_t;
                                }
                            }
                        } else if (c as ::core::ffi::c_uint)
                            < (if got == 0 as uint32_t {
                                lo as ::core::ffi::c_uint
                            } else {
                                0x80 as ::core::ffi::c_uint
                            })
                            || c as ::core::ffi::c_uint
                                > (if got == 0 as uint32_t {
                                    hi as ::core::ffi::c_uint
                                } else {
                                    0xbf as ::core::ffi::c_uint
                                })
                        {
                            valid = 0 as ::core::ffi::c_int;
                        } else {
                            got = got.wrapping_add(1);
                            if got == need {
                                need = 0 as uint32_t;
                            }
                        }
                    }
                } else {
                    if run_n != 0 as uint64_t {
                        flush_run(
                            &raw mut d,
                            t,
                            s,
                            special,
                            skip,
                            ids,
                            run_from,
                            i_0,
                            run_n,
                            (valid != 0 && need == 0 as uint32_t) as ::core::ffi::c_int,
                        );
                        run_n = 0 as uint64_t;
                    }
                    put_token(&raw mut d, per_token, p, k, index);
                    index = index.wrapping_add(1);
                }
            }
        }
        i_0 = i_0.wrapping_add(1);
    }
    if run_n != 0 as uint64_t {
        flush_run(
            &raw mut d,
            t,
            s,
            special,
            skip,
            ids,
            run_from,
            n,
            run_n,
            (valid != 0 && need == 0 as uint32_t) as ::core::ffi::c_int,
        );
    }
    return d.total as int64_t;
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
