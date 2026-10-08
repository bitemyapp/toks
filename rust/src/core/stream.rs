#[repr(C)]
pub struct toks_gen { _opaque: [u8; 0] }
#[repr(C)]
pub struct toks_uni { _opaque: [u8; 0] }
#[repr(C)]
pub struct toks_wp_tables { _opaque: [u8; 0] }
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
    fn toks_plat_arena(n: uint64_t) -> *mut uint8_t;
    fn toks_plat_arena_free(p: *mut uint8_t, n: uint64_t);
    fn toks_wp_decode_k(
        ctx: *const toks_ctx,
        ids: *const uint32_t,
        n: uint64_t,
        flags: uint32_t,
        out: *mut uint8_t,
        cap: uint64_t,
        kept: *mut uint64_t,
    ) -> int64_t;
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
pub struct toks_ctx {
    pub t: toks_tables,
    pub tier: uint32_t,
    pub dec_byte_level: uint32_t,
    pub pp_ids: [uint32_t; 64],
    pub n_pp_prefix: uint32_t,
    pub n_pp_suffix: uint32_t,
    pub n_nonspecial: uint32_t,
    pub dec_max: uint32_t,
    pub identity: uint64_t,
    pub cpu_features: uint64_t,
    pub special_ids: *mut uint32_t,
    pub dec_slot: *mut uint8_t,
    pub dec_len: *mut uint8_t,
    pub voc_slots: *const uint32_t,
    pub voc_mask: uint64_t,
    pub voc_add: *const uint32_t,
    pub voc_pool: *const uint8_t,
    pub voc_added: *const uint32_t,
    pub voc_special: *const uint32_t,
    pub voc_n_add: uint32_t,
    pub voc_bf: uint32_t,
    pub mem_voc: *mut uint8_t,
    pub mem_voc_len: uint64_t,
    pub mem_tables: *mut uint8_t,
    pub mem_tables_len: uint64_t,
    pub mem_bpe: *mut uint8_t,
    pub mem_bpe_len: uint64_t,
    pub name: [::core::ffi::c_char; 64],
    pub source_sha256: [uint8_t; 32],
    pub nfc: uint32_t,
    pub spm: *const toks_spm,
    pub mem_spm: *mut uint8_t,
    pub mem_spm_len: uint64_t,
    pub dc: toks_dchain,
    pub cut_chunk: uint32_t,
    pub cut_run: uint32_t,
    pub has_drop: uint32_t,
    pub drop: [uint8_t; 32],
    pub drop_unk: uint32_t,
    pub drop_fuse: uint32_t,
    pub wp: *const toks_wp_tables,
    pub mem_wp: *mut uint8_t,
    pub mem_wp_len: uint64_t,
    pub scr_extra: uint64_t,
    pub wp_mat_cap: uint64_t,
    pub o: toks_opts,
    pub uni: *const toks_uni,
    pub mem_uni: *mut uint8_t,
    pub mem_uni_len: uint64_t,
    pub gen: *const toks_gen,
    pub mem_gen: *mut uint8_t,
    pub mem_gen_len: uint64_t,
    pub bound_num: uint32_t,
    pub bound_den: uint32_t,
    pub bound_g: uint32_t,
    pub bound_rsv: uint32_t,
    pub pp_type: [uint32_t; 64],
    pub pp_seq_type: uint32_t,
    pub voc_n_dec: uint32_t,
    pub voc_dec: *const uint32_t,
}
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
pub struct toks_dchain {
    pub dec: *const toks_spm_op,
    pub n_dec: uint32_t,
    pub has_decoder: uint32_t,
    pub bf_first: uint32_t,
    pub on: uint32_t,
    pub holes: *const uint32_t,
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
pub struct toks_stream {
    pub opaque: [uint64_t; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sst {
    pub tag: uint64_t,
    pub flags: uint32_t,
    pub strip_left: uint32_t,
    pub np: uint8_t,
    pub mode: uint8_t,
    pub rsv: [uint8_t; 2],
    pub buf: [uint8_t; 44],
}
pub const TOKS_SPM_D_STRIP: C2RustUnnamed = 4;
pub const TOKS_SPM_D_BYTE_FALLBACK: C2RustUnnamed = 2;
pub const TOKS_SPM_D_METASPACE: C2RustUnnamed = 5;
pub const TOKS_SPM_D_REPLACE: C2RustUnnamed = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sst_x {
    pub p: *mut uint8_t,
    pub cap: uint64_t,
    pub n: uint64_t,
    pub chk: uint64_t,
    pub need: uint8_t,
    pub lo: uint8_t,
    pub hi: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct toks_lossy {
    pub out: *mut uint8_t,
    pub cap: uint64_t,
    pub n: uint64_t,
    pub np: uint32_t,
    pub pend: [uint8_t; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sw {
    pub out: *mut uint8_t,
    pub cap: uint64_t,
    pub n: uint64_t,
    pub s: sst,
    pub bfop: *const toks_spm_op,
    pub t_from: uint64_t,
    pub t_k: uint64_t,
    pub raw: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct chr {
    pub b: [uint8_t; 4],
    pub have: uint32_t,
    pub need: uint32_t,
}
pub const TOKS_SPM_PS_NEVER: C2RustUnnamed_0 = 2;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct toks_u8 {
    pub need: uint32_t,
    pub lo: uint8_t,
    pub hi: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct toks_ext {
    pub pad: uint32_t,
    pub align: uint32_t,
}
pub type C2RustUnnamed = ::core::ffi::c_uint;
pub const TOKS_SPM_D_FUSE: C2RustUnnamed = 3;
pub type C2RustUnnamed_0 = ::core::ffi::c_uint;
pub const TOKS_SPM_PS_FIRST: C2RustUnnamed_0 = 1;
pub const TOKS_SPM_PS_ALWAYS: C2RustUnnamed_0 = 0;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
pub const UINT64_MAX: ::core::ffi::c_ulong = 18446744073709551615
    as ::core::ffi::c_ulong;
pub const TOKS_E_UNSUPPORTED: ::core::ffi::c_int = -(3 as ::core::ffi::c_int);
pub const TOKS_E_ID: ::core::ffi::c_int = -(7 as ::core::ffi::c_int);
pub const TOKS_E_CAP: ::core::ffi::c_int = -(8 as ::core::ffi::c_int);
pub const TOKS_E_LIMIT: ::core::ffi::c_int = -(9 as ::core::ffi::c_int);
pub const TOKS_E_ARG: ::core::ffi::c_int = -(10 as ::core::ffi::c_int);
pub const TOKS_E_NOMEM: ::core::ffi::c_int = -(11 as ::core::ffi::c_int);
pub const TOKS_MAX_TEXT: ::core::ffi::c_ulonglong = (1 as ::core::ffi::c_ulonglong)
    << 29 as ::core::ffi::c_int;
pub const TOKS_SKIP_SPECIAL: ::core::ffi::c_uint = 1 as ::core::ffi::c_uint;
pub const TOKS_DECODE_RAW: ::core::ffi::c_uint = 2 as ::core::ffi::c_uint;
pub const TOKS_X_DEC_SLOT: toks_ext = toks_ext {
    pad: 0 as uint32_t,
    align: 1 as uint32_t,
};
pub const TOKS_X_DEC_LEN: toks_ext = toks_ext {
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
    #[cfg(feature = "test-guard")]
    { return crate::guard::toks_guard_tab(block.cast(), block.offset(o as isize).cast(), n.wrapping_add(x.pad as u64), x.align as u64); }
    #[cfg(not(feature = "test-guard"))]
    {
    return block.offset(o as isize) as *mut ::core::ffi::c_void;

    }
}
#[inline]
unsafe extern "C" fn toks_tab_owner(mut p: *const ::core::ffi::c_void) -> *mut uint8_t {
    #[cfg(feature = "test-guard")]
    { return crate::guard::toks_guard_owner(p); }
    #[cfg(not(feature = "test-guard"))]
    {
    return p as uintptr_t as *mut uint8_t;

    }
}
#[inline]
unsafe extern "C" fn toks_tab_seal(mut b: *mut ::core::ffi::c_void, mut n: uint64_t) {
    #[cfg(feature = "test-guard")]
    { crate::guard::toks_guard_seal(b, n); }
    #[cfg(not(feature = "test-guard"))]
    {
    }
}
#[inline]
unsafe extern "C" fn toks_tab_unmapped(
    mut b: *const ::core::ffi::c_void,
    mut n: uint64_t,
) {
    #[cfg(feature = "test-guard")]
    { crate::guard::toks_guard_release(b, n); }
    #[cfg(not(feature = "test-guard"))]
    {
    }
}
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
unsafe extern "C" fn toks_u8_feed(
    mut v: *mut toks_u8,
    mut c: uint8_t,
) -> ::core::ffi::c_int {
    if (*v).need == 0 as uint32_t {
        if (c as ::core::ffi::c_uint) < 0x80 as ::core::ffi::c_uint {
            return 1 as ::core::ffi::c_int;
        }
        if (c as ::core::ffi::c_uint) < 0xc2 as ::core::ffi::c_uint
            || c as ::core::ffi::c_uint > 0xf4 as ::core::ffi::c_uint
        {
            return 0 as ::core::ffi::c_int;
        }
        (*v).need = (if (c as ::core::ffi::c_uint) < 0xe0 as ::core::ffi::c_uint {
            1 as ::core::ffi::c_uint
        } else if (c as ::core::ffi::c_uint) < 0xf0 as ::core::ffi::c_uint {
            2 as ::core::ffi::c_uint
        } else {
            3 as ::core::ffi::c_uint
        }) as uint32_t;
        (*v).lo = (if c as ::core::ffi::c_uint == 0xe0 as ::core::ffi::c_uint {
            0xa0 as ::core::ffi::c_uint
        } else if c as ::core::ffi::c_uint == 0xf0 as ::core::ffi::c_uint {
            0x90 as ::core::ffi::c_uint
        } else {
            0x80 as ::core::ffi::c_uint
        }) as uint8_t;
        (*v).hi = (if c as ::core::ffi::c_uint == 0xed as ::core::ffi::c_uint {
            0x9f as ::core::ffi::c_uint
        } else if c as ::core::ffi::c_uint == 0xf4 as ::core::ffi::c_uint {
            0x8f as ::core::ffi::c_uint
        } else {
            0xbf as ::core::ffi::c_uint
        }) as uint8_t;
        return 1 as ::core::ffi::c_int;
    }
    if (c as ::core::ffi::c_int) < (*v).lo as ::core::ffi::c_int
        || c as ::core::ffi::c_int > (*v).hi as ::core::ffi::c_int
    {
        return 0 as ::core::ffi::c_int;
    }
    (*v).lo = 0x80 as uint8_t;
    (*v).hi = 0xbf as uint8_t;
    (*v).need = (*v).need.wrapping_sub(1);
    return 1 as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn toks_hexd(mut c: uint8_t) -> ::core::ffi::c_int {
    return if c as ::core::ffi::c_int >= '0' as i32
        && c as ::core::ffi::c_int <= '9' as i32
    {
        c as ::core::ffi::c_int - '0' as i32
    } else if c as ::core::ffi::c_int >= 'a' as i32
        && c as ::core::ffi::c_int <= 'f' as i32
    {
        c as ::core::ffi::c_int - 'a' as i32 + 10 as ::core::ffi::c_int
    } else if c as ::core::ffi::c_int >= 'A' as i32
        && c as ::core::ffi::c_int <= 'F' as i32
    {
        c as ::core::ffi::c_int - 'A' as i32 + 10 as ::core::ffi::c_int
    } else {
        -(1 as ::core::ffi::c_int)
    };
}
#[inline]
unsafe extern "C" fn toks_byte_token(
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
        return toks_hexd(*p.offset(4 as ::core::ffi::c_int as isize));
    }
    let mut hi: ::core::ffi::c_int = toks_hexd(
        *p.offset(3 as ::core::ffi::c_int as isize),
    );
    let mut lo: ::core::ffi::c_int = toks_hexd(
        *p.offset(4 as ::core::ffi::c_int as isize),
    );
    return if hi < 0 as ::core::ffi::c_int || lo < 0 as ::core::ffi::c_int {
        -(1 as ::core::ffi::c_int)
    } else {
        hi * 16 as ::core::ffi::c_int + lo
    };
}
#[inline]
unsafe extern "C" fn toks_bit(mut bits: *const uint32_t, mut id: uint32_t) -> uint32_t {
    return *bits.offset((id >> 5 as ::core::ffi::c_int) as isize)
        >> (id & 31 as uint32_t) & 1 as uint32_t;
}
#[inline]
unsafe extern "C" fn toks_utf8_valid(
    mut p: *const uint8_t,
    mut n: uint64_t,
) -> ::core::ffi::c_int {
    let mut i: uint64_t = 0 as uint64_t;
    while i < n {
        let mut k: uint32_t = toks_utf8_len(p.offset(i as isize), n.wrapping_sub(i));
        if k == 0 as uint32_t {
            return 0 as ::core::ffi::c_int;
        }
        i = i.wrapping_add(k as uint64_t);
    }
    return 1 as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn toks_mix64(mut x: uint64_t, mut w: uint64_t) -> uint64_t {
    x = ((x ^ w) as ::core::ffi::c_ulonglong)
        .wrapping_mul(0xff51afd7ed558ccd as ::core::ffi::c_ulonglong) as uint64_t;
    return x ^ x >> 32 as ::core::ffi::c_int;
}
pub const TOKS_SPM_MAX_OPS: ::core::ffi::c_uint = 8 as ::core::ffi::c_uint;
pub const SST_RUN: ::core::ffi::c_uint = 44 as ::core::ffi::c_uint;
pub const SST_FIRST: ::core::ffi::c_uint = 1 as ::core::ffi::c_uint;
pub const SST_VALID: ::core::ffi::c_uint = 2 as ::core::ffi::c_uint;
pub const SST_INVALID: ::core::ffi::c_uint = 4 as ::core::ffi::c_uint;
pub const SST_HOLD: ::core::ffi::c_uint = 8 as ::core::ffi::c_uint;
#[inline]
unsafe extern "C" fn sst_x_get(mut s: *const sst) -> sst_x {
    let mut x: sst_x = sst_x {
        p: ::core::ptr::null_mut::<uint8_t>(),
        cap: 0,
        n: 0,
        chk: 0,
        need: 0,
        lo: 0,
        hi: 0,
    };
    memcpy(
        &raw mut x as *mut ::core::ffi::c_void,
        &raw const (*s).buf as *const uint8_t as *const ::core::ffi::c_void,
        ::core::mem::size_of::<sst_x>() as size_t,
    );
    return x;
}
#[inline]
unsafe extern "C" fn sst_x_put(mut s: *mut sst, mut x: *const sst_x) {
    memcpy(
        &raw mut (*s).buf as *mut uint8_t as *mut ::core::ffi::c_void,
        x as *const ::core::ffi::c_void,
        ::core::mem::size_of::<sst_x>() as size_t,
    );
}
#[inline]
unsafe extern "C" fn sst_chk(mut tag: uint64_t, mut x: *const sst_x) -> uint64_t {
    return toks_mix64(toks_mix64(tag, (*x).p as uintptr_t as uint64_t), (*x).cap);
}
#[inline]
unsafe extern "C" fn sst_bytes(mut s: *const sst) -> *const uint8_t {
    return if (*s).mode as ::core::ffi::c_uint & SST_HOLD != 0 as ::core::ffi::c_uint {
        sst_x_get(s).p as *const uint8_t
    } else {
        &raw const (*s).buf as *const uint8_t
    };
}
#[inline]
unsafe extern "C" fn sst_held(mut s: *const sst) -> uint64_t {
    return if (*s).mode as ::core::ffi::c_uint & SST_HOLD != 0 as ::core::ffi::c_uint {
        sst_x_get(s).n
    } else {
        (*s).np as uint64_t
    };
}
#[inline]
unsafe extern "C" fn sst_cap(mut s: *const sst) -> uint64_t {
    return if (*s).mode as ::core::ffi::c_uint & SST_HOLD != 0 as ::core::ffi::c_uint {
        sst_x_get(s).cap
    } else {
        SST_RUN as uint64_t
    };
}
#[inline]
unsafe extern "C" fn sst_set_held(mut s: *mut sst, mut n: uint64_t) {
    if (*s).mode as ::core::ffi::c_uint & SST_HOLD == 0 as ::core::ffi::c_uint {
        (*s).np = n as uint8_t;
        return;
    }
    let mut x: sst_x = sst_x_get(s);
    x.n = n;
    sst_x_put(s, &raw mut x);
}
pub const SST_MAGIC: ::core::ffi::c_ulonglong = 0x314d5254534b4f54
    as ::core::ffi::c_ulonglong;
pub const DL_LONG: ::core::ffi::c_uint = 0xfe as ::core::ffi::c_uint;
pub const DL_SLOW: ::core::ffi::c_uint = 0xff as ::core::ffi::c_uint;
unsafe extern "C" fn dec_block_bytes(mut n_ids: uint32_t) -> uint64_t {
    return (17 as uint64_t).wrapping_mul(n_ids as uint64_t);
}
static mut FFFD: [uint8_t; 3] = [
    0xef as ::core::ffi::c_uint as uint8_t,
    0xbf as ::core::ffi::c_uint as uint8_t,
    0xbd as ::core::ffi::c_uint as uint8_t,
];
unsafe extern "C" fn copy(mut d: *mut uint8_t, mut s: *const uint8_t, mut k: uint64_t) {
    if k >= 8 as uint64_t {
        let mut v: uint64_t = 0;
        let mut w: uint64_t = 0;
        let mut i: uint64_t = 0 as uint64_t;
        while i.wrapping_add(8 as uint64_t) < k {
            memcpy(
                &raw mut v as *mut ::core::ffi::c_void,
                s.offset(i as isize) as *const ::core::ffi::c_void,
                8 as size_t,
            );
            memcpy(
                d.offset(i as isize) as *mut ::core::ffi::c_void,
                &raw mut v as *const ::core::ffi::c_void,
                8 as size_t,
            );
            i = i.wrapping_add(8 as uint64_t);
        }
        memcpy(
            &raw mut w as *mut ::core::ffi::c_void,
            s.offset(k as isize).offset(-(8 as ::core::ffi::c_uint as isize))
                as *const ::core::ffi::c_void,
            8 as size_t,
        );
        memcpy(
            d.offset(k as isize).offset(-(8 as ::core::ffi::c_uint as isize))
                as *mut ::core::ffi::c_void,
            &raw mut w as *const ::core::ffi::c_void,
            8 as size_t,
        );
    } else if k >= 4 as uint64_t {
        let mut v_0: uint32_t = 0;
        let mut w_0: uint32_t = 0;
        memcpy(
            &raw mut v_0 as *mut ::core::ffi::c_void,
            s as *const ::core::ffi::c_void,
            4 as size_t,
        );
        memcpy(
            &raw mut w_0 as *mut ::core::ffi::c_void,
            s.offset(k as isize).offset(-(4 as ::core::ffi::c_uint as isize))
                as *const ::core::ffi::c_void,
            4 as size_t,
        );
        memcpy(
            d as *mut ::core::ffi::c_void,
            &raw mut v_0 as *const ::core::ffi::c_void,
            4 as size_t,
        );
        memcpy(
            d.offset(k as isize).offset(-(4 as ::core::ffi::c_uint as isize))
                as *mut ::core::ffi::c_void,
            &raw mut w_0 as *const ::core::ffi::c_void,
            4 as size_t,
        );
    } else if k >= 2 as uint64_t {
        let mut v_1: uint16_t = 0;
        let mut w_1: uint16_t = 0;
        memcpy(
            &raw mut v_1 as *mut ::core::ffi::c_void,
            s as *const ::core::ffi::c_void,
            2 as size_t,
        );
        memcpy(
            &raw mut w_1 as *mut ::core::ffi::c_void,
            s.offset(k as isize).offset(-(2 as ::core::ffi::c_uint as isize))
                as *const ::core::ffi::c_void,
            2 as size_t,
        );
        memcpy(
            d as *mut ::core::ffi::c_void,
            &raw mut v_1 as *const ::core::ffi::c_void,
            2 as size_t,
        );
        memcpy(
            d.offset(k as isize).offset(-(2 as ::core::ffi::c_uint as isize))
                as *mut ::core::ffi::c_void,
            &raw mut w_1 as *const ::core::ffi::c_void,
            2 as size_t,
        );
    } else if k == 1 as uint64_t {
        *d.offset(0 as ::core::ffi::c_int as isize) = *s
            .offset(0 as ::core::ffi::c_int as isize);
    }
}
#[inline]
unsafe extern "C" fn ld32(mut p: *const ::core::ffi::c_void) -> uint32_t {
    return toks_ld32(p);
}
unsafe extern "C" fn copy16(mut d: *mut uint8_t, mut s: *const uint8_t) {
    let mut v: [uint8_t; 16] = [0; 16];
    memcpy(
        &raw mut v as *mut uint8_t as *mut ::core::ffi::c_void,
        s as *const ::core::ffi::c_void,
        16 as size_t,
    );
    memcpy(
        d as *mut ::core::ffi::c_void,
        &raw mut v as *mut uint8_t as *const ::core::ffi::c_void,
        16 as size_t,
    );
}
unsafe extern "C" fn move16s(
    mut d: *mut uint8_t,
    mut s: *const uint8_t,
    mut k: uint64_t,
) {
    let mut j: uint64_t = 0 as uint64_t;
    while j < k {
        copy16(d.offset(j as isize), s.offset(j as isize));
        j = j.wrapping_add(16 as uint64_t);
    }
}
unsafe extern "C" fn round16(mut k: uint64_t) -> uint64_t {
    return k.wrapping_add(15 as uint64_t) & !(15 as ::core::ffi::c_uint as uint64_t);
}
unsafe extern "C" fn put(
    mut d: *mut toks_lossy,
    mut p: *const uint8_t,
    mut k: uint64_t,
) {
    let mut n: uint64_t = (*d).n;
    (*d).n = n.wrapping_add(k);
    if n < (*d).cap {
        copy(
            (*d).out.offset(n as isize),
            p,
            if (*d).cap.wrapping_sub(n) < k { (*d).cap.wrapping_sub(n) } else { k },
        );
    }
}
unsafe extern "C" fn seq_need(mut c: uint8_t) -> uint32_t {
    return if (c as ::core::ffi::c_uint) < 0xe0 as ::core::ffi::c_uint {
        2 as uint32_t
    } else if (c as ::core::ffi::c_uint) < 0xf0 as ::core::ffi::c_uint {
        3 as uint32_t
    } else {
        4 as uint32_t
    };
}
unsafe extern "C" fn seq_lo(mut c: uint8_t) -> uint8_t {
    return (if c as ::core::ffi::c_uint == 0xe0 as ::core::ffi::c_uint {
        0xa0 as ::core::ffi::c_uint
    } else if c as ::core::ffi::c_uint == 0xf0 as ::core::ffi::c_uint {
        0x90 as ::core::ffi::c_uint
    } else {
        0x80 as ::core::ffi::c_uint
    }) as uint8_t;
}
unsafe extern "C" fn seq_hi(mut c: uint8_t) -> uint8_t {
    return (if c as ::core::ffi::c_uint == 0xed as ::core::ffi::c_uint {
        0x9f as ::core::ffi::c_uint
    } else if c as ::core::ffi::c_uint == 0xf4 as ::core::ffi::c_uint {
        0x8f as ::core::ffi::c_uint
    } else {
        0xbf as ::core::ffi::c_uint
    }) as uint8_t;
}
#[inline(always)]
unsafe extern "C" fn lossy_bytes(
    mut d: *mut toks_lossy,
    mut p: *const uint8_t,
    mut k: uint64_t,
) {
    let mut j: uint64_t = 0 as uint64_t;
    while j < k {
        if (*d).np == 0 as uint32_t {
            let mut s: uint64_t = j;
            while j < k {
                if (*p.offset(j as isize) as ::core::ffi::c_uint)
                    < 0x80 as ::core::ffi::c_uint
                {
                    j = j.wrapping_add(1);
                } else {
                    let mut w: uint32_t = toks_utf8_len(
                        p.offset(j as isize),
                        k.wrapping_sub(j),
                    );
                    if w == 0 as uint32_t {
                        break;
                    }
                    j = j.wrapping_add(w as uint64_t);
                }
            }
            if j > s {
                put(d, p.offset(s as isize), j.wrapping_sub(s));
            }
            if j == k {
                break;
            }
            let mut c: uint8_t = *p.offset(j as isize);
            j = j.wrapping_add(1);
            if (c as ::core::ffi::c_uint) < 0xc2 as ::core::ffi::c_uint
                || c as ::core::ffi::c_uint > 0xf4 as ::core::ffi::c_uint
            {
                put(d, &raw const FFFD as *const uint8_t, 3 as uint64_t);
            } else {
                (*d).pend[0 as ::core::ffi::c_int as usize] = c;
                (*d).np = 1 as ::core::ffi::c_uint as uint32_t;
            }
        } else {
            let mut c_0: uint8_t = *p.offset(j as isize);
            let mut c0: uint8_t = (*d).pend[0 as ::core::ffi::c_int as usize];
            let mut lo: uint8_t = (if (*d).np == 1 as uint32_t {
                seq_lo(c0) as ::core::ffi::c_uint
            } else {
                0x80 as ::core::ffi::c_uint
            }) as uint8_t;
            let mut hi: uint8_t = (if (*d).np == 1 as uint32_t {
                seq_hi(c0) as ::core::ffi::c_uint
            } else {
                0xbf as ::core::ffi::c_uint
            }) as uint8_t;
            if (c_0 as ::core::ffi::c_int) < lo as ::core::ffi::c_int
                || c_0 as ::core::ffi::c_int > hi as ::core::ffi::c_int
            {
                put(d, &raw const FFFD as *const uint8_t, 3 as uint64_t);
                (*d).np = 0 as ::core::ffi::c_uint as uint32_t;
            } else {
                (*d).pend[(*d).np as usize] = c_0;
                (*d).np = (*d).np.wrapping_add(1);
                j = j.wrapping_add(1);
                if (*d).np == seq_need(c0) {
                    put(d, &raw mut (*d).pend as *mut uint8_t, (*d).np as uint64_t);
                    (*d).np = 0 as ::core::ffi::c_uint as uint32_t;
                }
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn toks_lossy_ids(
    mut ctx: *const toks_ctx,
    mut d: *mut toks_lossy,
    mut ids: *const uint32_t,
    mut n: uint64_t,
    mut flags: uint32_t,
) -> int64_t {
    let mut off: *const uint32_t = (*ctx).t.tok_off;
    let mut tb: *const uint8_t = (*ctx).t.tok_bytes;
    let mut slot: *const uint8_t = (*ctx).dec_slot;
    let mut dlen: *const uint8_t = (*ctx).dec_len;
    let mut spec: *const uint32_t = if flags & TOKS_SKIP_SPECIAL as uint32_t
        != 0 as uint32_t
    {
        (*ctx).special_ids
    } else {
        ::core::ptr::null_mut::<uint32_t>()
    };
    let mut nid: uint32_t = (*ctx).t.n_ids;
    let mut c: toks_lossy = *d;
    let mut r: int64_t = 0 as int64_t;
    let mut i: uint64_t = 0 as uint64_t;
    while i < n {
        if c.np == 0 as uint32_t && !dlen.is_null() && c.cap >= 16 as uint64_t {
            let mut lim: uint64_t = c.cap.wrapping_sub(16 as uint64_t);
            while i < n && c.n <= lim {
                let mut id: uint32_t = ld32(
                    ids.offset(i as isize) as *const ::core::ffi::c_void,
                );
                if id >= nid
                    || *dlen.offset(id as isize) as ::core::ffi::c_uint
                        > 16 as ::core::ffi::c_uint
                {
                    break;
                }
                i = i.wrapping_add(1);
                if !spec.is_null() && toks_bit(spec, id) != 0 as uint32_t {
                    continue;
                }
                copy16(
                    c.out.offset(c.n as isize),
                    slot.offset((16 as uint64_t).wrapping_mul(id as uint64_t) as isize),
                );
                c.n = c.n.wrapping_add(*dlen.offset(id as isize) as uint64_t);
            }
            if i == n {
                break;
            }
        }
        let mut id_0: uint32_t = ld32(
            ids.offset(i as isize) as *const ::core::ffi::c_void,
        );
        i = i.wrapping_add(1);
        if id_0 >= nid {
            r = TOKS_E_ID as int64_t;
            break;
        } else {
            if !spec.is_null() && toks_bit(spec, id_0) != 0 as uint32_t {
                continue;
            }
            let mut l: uint32_t = if !dlen.is_null() {
                *dlen.offset(id_0 as isize) as uint32_t
            } else {
                DL_SLOW as uint32_t
            };
            let mut room: uint64_t = if c.n < c.cap {
                c.cap.wrapping_sub(c.n)
            } else {
                0 as uint64_t
            };
            if c.np == 0 as uint32_t && l <= 16 as uint32_t {
                let mut q: *const uint8_t = slot
                    .offset((16 as uint64_t).wrapping_mul(id_0 as uint64_t) as isize);
                if room >= 16 as uint64_t {
                    copy16(c.out.offset(c.n as isize), q);
                } else if room != 0 as uint64_t {
                    copy(
                        c.out.offset(c.n as isize),
                        q,
                        if room < l as uint64_t { room } else { l as uint64_t },
                    );
                }
                c.n = c.n.wrapping_add(l as uint64_t);
            } else {
                let mut p: *const uint8_t = tb
                    .offset(*off.offset(id_0 as isize) as isize);
                let mut k: uint64_t = (*off
                    .offset(id_0.wrapping_add(1 as uint32_t) as isize))
                    .wrapping_sub(*off.offset(id_0 as isize)) as uint64_t;
                if c.np == 0 as uint32_t && l == DL_LONG as uint32_t {
                    if room >= round16(k) {
                        move16s(c.out.offset(c.n as isize), p, k);
                    } else if room != 0 as uint64_t {
                        copy(
                            c.out.offset(c.n as isize),
                            p,
                            if room < k { room } else { k },
                        );
                    }
                    c.n = c.n.wrapping_add(k);
                } else {
                    lossy_bytes(&raw mut c, p, k);
                }
            }
        }
    }
    *d = c;
    return r;
}
#[no_mangle]
pub unsafe extern "C" fn toks_lossy_end(mut d: *mut toks_lossy) {
    if (*d).np != 0 as uint32_t {
        put(d, &raw const FFFD as *const uint8_t, 3 as uint64_t);
        (*d).np = 0 as ::core::ffi::c_uint as uint32_t;
    }
}
unsafe extern "C" fn spm_ops(
    mut s: *const toks_dchain,
    mut bf: *mut ::core::ffi::c_int,
    mut strip: *mut *const toks_spm_op,
) -> *const toks_spm_op {
    let mut per_token: *const toks_spm_op = ::core::ptr::null::<toks_spm_op>();
    *bf = 0 as ::core::ffi::c_int;
    *strip = ::core::ptr::null::<toks_spm_op>();
    let mut i: uint32_t = 0 as uint32_t;
    while i < (*s).n_dec && i < TOKS_SPM_MAX_OPS as uint32_t {
        let mut op: *const toks_spm_op = (*s).dec.offset(i as isize)
            as *const toks_spm_op;
        if (*op).kind == TOKS_SPM_D_REPLACE as ::core::ffi::c_int as uint32_t
            || (*op).kind == TOKS_SPM_D_METASPACE as ::core::ffi::c_int as uint32_t
        {
            per_token = op;
        } else if (*op).kind
            == TOKS_SPM_D_BYTE_FALLBACK as ::core::ffi::c_int as uint32_t
        {
            *bf = 1 as ::core::ffi::c_int;
        } else if (*op).kind == TOKS_SPM_D_STRIP as ::core::ffi::c_int as uint32_t {
            *strip = op;
        }
        i = i.wrapping_add(1);
    }
    return per_token;
}
#[no_mangle]
pub unsafe extern "C" fn toks_dec_build(mut c: *mut toks_ctx) -> int64_t {
    let mut t: *const toks_tables = &raw mut (*c).t;
    (*c).dec_max = 0 as ::core::ffi::c_uint as uint32_t;
    (*c).dec_slot = ::core::ptr::null_mut::<uint8_t>();
    (*c).dec_len = ::core::ptr::null_mut::<uint8_t>();
    if (*t).n_ids == 0 as uint32_t {
        return 0 as int64_t;
    }
    if !(*c).wp.is_null() {
        let mut best: uint64_t = 0 as uint64_t;
        let mut id: uint32_t = 0 as uint32_t;
        while id < (*t).n_ids {
            let mut k: uint64_t = (*(*t)
                .tok_off
                .offset(id.wrapping_add(1 as uint32_t) as isize) as uint64_t)
                .wrapping_sub(*(*t).tok_off.offset(id as isize) as uint64_t);
            if k > best {
                best = k;
            }
            id = id.wrapping_add(1);
        }
        (*c).dec_max = best.wrapping_add(1 as uint64_t) as uint32_t;
        return 0 as int64_t;
    }
    if (*c).dc.on != 0 {
        let mut bf: ::core::ffi::c_int = 0;
        let mut strip: *const toks_spm_op = ::core::ptr::null::<toks_spm_op>();
        let mut op: *const toks_spm_op = spm_ops(
            &raw mut (*c).dc,
            &raw mut bf,
            &raw mut strip,
        );
        let mut mult: uint64_t = (if !op.is_null()
            && (*op).kind == TOKS_SPM_D_REPLACE as ::core::ffi::c_int as uint32_t
            && (*op).b.n > 1 as uint32_t
        {
            (*op).b.n
        } else {
            1 as uint32_t
        }) as uint64_t;
        let mut best_0: uint64_t = 0 as uint64_t;
        let mut id_0: uint32_t = 0 as uint32_t;
        while id_0 < (*t).n_ids {
            let mut o: uint64_t = *(*t).tok_off.offset(id_0 as isize) as uint64_t;
            let mut k_0: uint64_t = (*(*t)
                .tok_off
                .offset(id_0.wrapping_add(1 as uint32_t) as isize) as uint64_t)
                .wrapping_sub(o);
            let mut w: uint64_t = if bf != 0
                && toks_byte_token((*t).tok_bytes.offset(o as isize), k_0)
                    >= 0 as ::core::ffi::c_int
            {
                3 as uint64_t
            } else {
                k_0.wrapping_mul(mult)
                    .wrapping_add(
                        (if (*c).dc.has_decoder != 0 {
                            0 as ::core::ffi::c_uint
                        } else {
                            1 as ::core::ffi::c_uint
                        }) as uint64_t,
                    )
            };
            if w > best_0 {
                best_0 = w;
            }
            id_0 = id_0.wrapping_add(1);
        }
        (*c).dec_max = best_0 as uint32_t;
        return 0 as int64_t;
    }
    if (*c).dec_byte_level == 0 as uint32_t {
        return 0 as int64_t;
    }
    let mut bytes: uint64_t = dec_block_bytes((*t).n_ids);
    let mut blk: *mut uint8_t = toks_plat_arena(bytes);
    if blk.is_null() {
        return TOKS_E_NOMEM as int64_t;
    }
    memset(blk as *mut ::core::ffi::c_void, 0 as ::core::ffi::c_int, bytes as size_t);
    let mut slot: *mut uint8_t = toks_tab(
        blk,
        0 as uint64_t,
        (16 as uint64_t).wrapping_mul((*t).n_ids as uint64_t),
        TOKS_X_DEC_SLOT,
    ) as *mut uint8_t;
    let mut len: *mut uint8_t = toks_tab(
        blk,
        (16 as uint64_t).wrapping_mul((*t).n_ids as uint64_t),
        (*t).n_ids as uint64_t,
        TOKS_X_DEC_LEN,
    ) as *mut uint8_t;
    toks_tab_seal(blk as *mut ::core::ffi::c_void, bytes);
    let mut end: uint64_t = *(*t).tok_off.offset((*t).n_ids as isize) as uint64_t;
    let mut best_1: uint64_t = 0 as uint64_t;
    let mut id_1: uint32_t = 0 as uint32_t;
    while id_1 < (*t).n_ids {
        let mut o_0: uint64_t = *(*t).tok_off.offset(id_1 as isize) as uint64_t;
        let mut k_1: uint64_t = (*(*t)
            .tok_off
            .offset(id_1.wrapping_add(1 as uint32_t) as isize) as uint64_t)
            .wrapping_sub(o_0);
        let mut p: *const uint8_t = (*t).tok_bytes.offset(o_0 as isize);
        let mut d: toks_lossy = toks_lossy {
            out: ::core::ptr::null_mut::<uint8_t>(),
            cap: 0,
            n: 0,
            np: 0,
            pend: [0; 4],
        };
        memset(
            &raw mut d as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<toks_lossy>() as size_t,
        );
        lossy_bytes(&raw mut d, p, k_1);
        toks_lossy_end(&raw mut d);
        if d.n > best_1 {
            best_1 = d.n;
        }
        *len.offset(id_1 as isize) = DL_SLOW as uint8_t;
        if toks_utf8_valid(p, k_1) != 0 {
            if k_1 <= 16 as uint64_t {
                copy(
                    slot
                        .offset(
                            (16 as uint64_t).wrapping_mul(id_1 as uint64_t) as isize,
                        ),
                    p,
                    k_1,
                );
                *len.offset(id_1 as isize) = k_1 as uint8_t;
            } else if o_0.wrapping_add(round16(k_1)) <= end {
                *len.offset(id_1 as isize) = DL_LONG as uint8_t;
            }
        }
        id_1 = id_1.wrapping_add(1);
    }
    (*c).dec_max = best_1 as uint32_t;
    (*c).dec_slot = slot;
    (*c).dec_len = len;
    return 0 as int64_t;
}
#[no_mangle]
pub unsafe extern "C" fn toks_dec_free(mut c: *mut toks_ctx) {
    if !(*c).dec_slot.is_null() {
        toks_tab_free(
            toks_tab_owner((*c).dec_slot as *const ::core::ffi::c_void),
            dec_block_bytes((*c).t.n_ids),
        );
    }
    (*c).dec_slot = ::core::ptr::null_mut::<uint8_t>();
    (*c).dec_len = ::core::ptr::null_mut::<uint8_t>();
}
unsafe extern "C" fn u8_state(
    mut b: *const uint8_t,
    mut n: uint32_t,
    mut need: *mut uint32_t,
    mut lo: *mut uint8_t,
    mut hi: *mut uint8_t,
) -> ::core::ffi::c_int {
    let mut v: toks_u8 = toks_u8 {
        need: 0 as uint32_t,
        lo: 0x80 as uint8_t,
        hi: 0xbf as uint8_t,
    };
    let mut i: uint32_t = 0 as uint32_t;
    while i < n {
        if toks_u8_feed(&raw mut v, *b.offset(i as isize)) == 0 {
            return 0 as ::core::ffi::c_int;
        }
        i = i.wrapping_add(1);
    }
    *need = v.need;
    *lo = v.lo;
    *hi = v.hi;
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn sst_get(
    mut ctx: *const toks_ctx,
    mut st: *const toks_stream,
    mut s: *mut sst,
) -> ::core::ffi::c_int {
    memcpy(
        s as *mut ::core::ffi::c_void,
        st as *const ::core::ffi::c_void,
        ::core::mem::size_of::<sst>() as size_t,
    );
    if (*s).tag as ::core::ffi::c_ulonglong
        != SST_MAGIC ^ (*ctx).identity as ::core::ffi::c_ulonglong
        || (*s).flags & !(TOKS_SKIP_SPECIAL as uint32_t) != 0 as uint32_t
        || (*s).rsv[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_uint
            != 0 as ::core::ffi::c_uint
        || (*s).rsv[1 as ::core::ffi::c_int as usize] as ::core::ffi::c_uint
            != 0 as ::core::ffi::c_uint
    {
        return 0 as ::core::ffi::c_int;
    }
    let mut need: uint32_t = 0;
    let mut lo: uint8_t = 0;
    let mut hi: uint8_t = 0;
    if !(*ctx).wp.is_null() {
        return ((*s).mode as ::core::ffi::c_uint & !SST_FIRST == 0 as ::core::ffi::c_uint
            && (*s).np as ::core::ffi::c_uint == 0 as ::core::ffi::c_uint
            && (*s).strip_left == 0 as uint32_t) as ::core::ffi::c_int;
    }
    if (*ctx).dc.on == 0 {
        if (*s).mode as ::core::ffi::c_uint != 0 as ::core::ffi::c_uint
            || (*s).strip_left != 0 as uint32_t
            || (*s).np as ::core::ffi::c_uint > 3 as ::core::ffi::c_uint
        {
            return 0 as ::core::ffi::c_int;
        }
        return ((*s).np as ::core::ffi::c_uint == 0 as ::core::ffi::c_uint
            || (*s).buf[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_uint
                >= 0xc2 as ::core::ffi::c_uint
                && (*s).buf[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_uint
                    <= 0xf4 as ::core::ffi::c_uint
                && u8_state(
                    &raw mut (*s).buf as *mut uint8_t,
                    (*s).np as uint32_t,
                    &raw mut need,
                    &raw mut lo,
                    &raw mut hi,
                ) != 0 && need != 0 as uint32_t) as ::core::ffi::c_int;
    }
    let mut bf: ::core::ffi::c_int = 0;
    let mut strip: *const toks_spm_op = ::core::ptr::null::<toks_spm_op>();
    spm_ops(&raw const (*ctx).dc, &raw mut bf, &raw mut strip);
    let mut n: uint64_t = sst_held(s);
    if (*s).mode as ::core::ffi::c_uint
        & !(SST_FIRST | SST_VALID | SST_INVALID | SST_HOLD) != 0 as ::core::ffi::c_uint
        || (*s).np as ::core::ffi::c_uint > SST_RUN
        || (*s).mode as ::core::ffi::c_uint & (SST_VALID | SST_INVALID)
            == SST_VALID | SST_INVALID
        || (*s).mode as ::core::ffi::c_uint & SST_VALID == 0 as ::core::ffi::c_uint
            && n != 0 as uint64_t
        || (*s).mode as ::core::ffi::c_uint & SST_VALID != 0 as ::core::ffi::c_uint
            && n == 0 as uint64_t
        || (*s).strip_left
            > (if !strip.is_null() { (*strip).start } else { 0 as uint32_t })
    {
        return 0 as ::core::ffi::c_int;
    }
    if (*s).mode as ::core::ffi::c_uint & SST_HOLD == 0 as ::core::ffi::c_uint {
        return ((*s).mode as ::core::ffi::c_uint & SST_VALID == 0 as ::core::ffi::c_uint
            || u8_state(
                &raw mut (*s).buf as *mut uint8_t,
                (*s).np as uint32_t,
                &raw mut need,
                &raw mut lo,
                &raw mut hi,
            ) != 0) as ::core::ffi::c_int;
    }
    let mut x: sst_x = sst_x_get(s);
    let mut i: uint32_t = ::core::mem::size_of::<sst_x>() as uint32_t;
    while i < SST_RUN as uint32_t {
        if (*s).buf[i as usize] as ::core::ffi::c_uint != 0 as ::core::ffi::c_uint {
            return 0 as ::core::ffi::c_int;
        }
        i = i.wrapping_add(1);
    }
    if (*s).np as ::core::ffi::c_uint != 0 as ::core::ffi::c_uint || x.p.is_null()
        || x.cap == 0 as uint64_t || x.n > x.cap
        || x.chk != sst_chk((*s).tag, &raw mut x)
    {
        return 0 as ::core::ffi::c_int;
    }
    if (*s).mode as ::core::ffi::c_uint & SST_VALID == 0 as ::core::ffi::c_uint {
        return (x.need as ::core::ffi::c_uint == 0 as ::core::ffi::c_uint
            && x.lo as ::core::ffi::c_uint == 0 as ::core::ffi::c_uint
            && x.hi as ::core::ffi::c_uint == 0 as ::core::ffi::c_uint)
            as ::core::ffi::c_int;
    }
    return (x.need as ::core::ffi::c_uint <= 3 as ::core::ffi::c_uint
        && 0x80 as ::core::ffi::c_uint <= x.lo as ::core::ffi::c_uint
        && x.lo as ::core::ffi::c_int <= x.hi as ::core::ffi::c_int
        && x.hi as ::core::ffi::c_uint <= 0xbf as ::core::ffi::c_uint)
        as ::core::ffi::c_int;
}
unsafe extern "C" fn sst_u8(
    mut s: *const sst,
    mut need: *mut uint32_t,
    mut lo: *mut uint8_t,
    mut hi: *mut uint8_t,
) {
    *need = 0 as ::core::ffi::c_uint as uint32_t;
    *lo = 0x80 as uint8_t;
    *hi = 0xbf as uint8_t;
    if (*s).mode as ::core::ffi::c_uint & SST_VALID == 0 as ::core::ffi::c_uint {
        return;
    }
    if (*s).mode as ::core::ffi::c_uint & SST_HOLD != 0 as ::core::ffi::c_uint {
        let mut x: sst_x = sst_x_get(s);
        *need = x.need as uint32_t;
        *lo = x.lo;
        *hi = x.hi;
        return;
    }
    u8_state(&raw const (*s).buf as *const uint8_t, (*s).np as uint32_t, need, lo, hi);
}
unsafe extern "C" fn sst_reset(mut ctx: *const toks_ctx, mut s: *mut sst) {
    let mut tag: uint64_t = (*s).tag;
    let mut flags: uint32_t = (*s).flags;
    let mut hold: ::core::ffi::c_int = ((*s).mode as ::core::ffi::c_uint & SST_HOLD
        != 0 as ::core::ffi::c_uint) as ::core::ffi::c_int;
    let mut x: sst_x = sst_x_get(s);
    memset(
        s as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<sst>() as size_t,
    );
    (*s).tag = tag;
    (*s).flags = flags;
    if hold != 0 {
        (*s).mode = SST_HOLD as uint8_t;
        x.n = 0 as uint64_t;
        x.need = 0 as uint8_t;
        x.lo = 0 as uint8_t;
        x.hi = 0 as uint8_t;
        sst_x_put(s, &raw mut x);
    }
    if (*ctx).dc.on != 0 {
        let mut bf: ::core::ffi::c_int = 0;
        let mut strip: *const toks_spm_op = ::core::ptr::null::<toks_spm_op>();
        spm_ops(&raw const (*ctx).dc, &raw mut bf, &raw mut strip);
        (*s).strip_left = if !strip.is_null() { (*strip).start } else { 0 as uint32_t };
    }
}
#[no_mangle]
pub unsafe extern "C" fn toks_stream_init(
    mut ctx: *const toks_ctx,
    mut st: *mut toks_stream,
    mut flags: uint32_t,
) {
    if st.is_null() {
        return;
    }
    memset(
        st as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<toks_stream>() as size_t,
    );
    if ctx.is_null() || flags & !(TOKS_SKIP_SPECIAL as uint32_t) != 0 as uint32_t {
        return;
    }
    let mut s: sst = sst {
        tag: 0,
        flags: 0,
        strip_left: 0,
        np: 0,
        mode: 0,
        rsv: [0; 2],
        buf: [0; 44],
    };
    memset(
        &raw mut s as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<sst>() as size_t,
    );
    s.tag = (SST_MAGIC ^ (*ctx).identity as ::core::ffi::c_ulonglong) as uint64_t;
    s.flags = flags;
    sst_reset(ctx, &raw mut s);
    memcpy(
        st as *mut ::core::ffi::c_void,
        &raw mut s as *const ::core::ffi::c_void,
        ::core::mem::size_of::<sst>() as size_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn toks_stream_hold(
    mut ctx: *const toks_ctx,
    mut st: *mut toks_stream,
    mut hold: *mut ::core::ffi::c_void,
    mut cap: uint64_t,
) -> int64_t {
    if ctx.is_null() || st.is_null() || hold.is_null() && cap != 0 as uint64_t {
        return TOKS_E_ARG as int64_t;
    }
    let mut s: sst = sst {
        tag: 0,
        flags: 0,
        strip_left: 0,
        np: 0,
        mode: 0,
        rsv: [0; 2],
        buf: [0; 44],
    };
    if sst_get(ctx, st, &raw mut s) == 0 {
        return TOKS_E_ARG as int64_t;
    }
    if (*ctx).dc.on == 0 && (*ctx).wp.is_null() && (*ctx).dec_byte_level == 0 as uint32_t
    {
        return TOKS_E_UNSUPPORTED as int64_t;
    }
    if (*ctx).dc.on == 0 {
        return 0 as int64_t;
    }
    let mut n: uint64_t = sst_held(&raw mut s);
    if n > (if cap != 0 as uint64_t { cap } else { SST_RUN as uint64_t }) {
        return TOKS_E_LIMIT as int64_t;
    }
    let mut need: uint32_t = 0;
    let mut lo: uint8_t = 0;
    let mut hi: uint8_t = 0;
    sst_u8(&raw mut s, &raw mut need, &raw mut lo, &raw mut hi);
    let mut from: *const uint8_t = sst_bytes(&raw mut s);
    if cap == 0 as uint64_t {
        let mut tmp: [uint8_t; 44] = [0; 44];
        copy(&raw mut tmp as *mut uint8_t, from, n);
        memset(
            &raw mut s.buf as *mut uint8_t as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<[uint8_t; 44]>() as size_t,
        );
        copy(&raw mut s.buf as *mut uint8_t, &raw mut tmp as *mut uint8_t, n);
        s.np = n as uint8_t;
        s.mode = (s.mode as ::core::ffi::c_uint & !SST_HOLD) as uint8_t;
    } else {
        let mut to: *mut uint8_t = hold as *mut uint8_t;
        let mut dt: uintptr_t = to as uintptr_t;
        let mut df: uintptr_t = from as uintptr_t;
        if dt < df {
            let mut i: uint64_t = 0 as uint64_t;
            while i < n {
                *to.offset(i as isize) = *from.offset(i as isize);
                i = i.wrapping_add(1);
            }
        } else if dt > df {
            let mut i_0: uint64_t = n;
            while i_0 > 0 as uint64_t {
                *to.offset(i_0.wrapping_sub(1 as uint64_t) as isize) = *from
                    .offset(i_0.wrapping_sub(1 as uint64_t) as isize);
                i_0 = i_0.wrapping_sub(1);
            }
        }
        let mut x: sst_x = sst_x {
            p: ::core::ptr::null_mut::<uint8_t>(),
            cap: 0,
            n: 0,
            chk: 0,
            need: 0,
            lo: 0,
            hi: 0,
        };
        memset(
            &raw mut x as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<sst_x>() as size_t,
        );
        x.p = to;
        x.cap = cap;
        x.n = n;
        x.chk = sst_chk(s.tag, &raw mut x);
        if s.mode as ::core::ffi::c_uint & SST_VALID != 0 as ::core::ffi::c_uint {
            x.need = need as uint8_t;
            x.lo = lo;
            x.hi = hi;
        }
        memset(
            &raw mut s.buf as *mut uint8_t as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<[uint8_t; 44]>() as size_t,
        );
        sst_x_put(&raw mut s, &raw mut x);
        s.np = 0 as uint8_t;
        s.mode = (s.mode as ::core::ffi::c_uint | SST_HOLD) as uint8_t;
    }
    memcpy(
        st as *mut ::core::ffi::c_void,
        &raw mut s as *const ::core::ffi::c_void,
        ::core::mem::size_of::<sst>() as size_t,
    );
    return n as int64_t;
}
#[no_mangle]
pub unsafe extern "C" fn toks_stream_bound(
    mut ctx: *const toks_ctx,
    mut n_ids: uint64_t,
) -> uint64_t {
    if ctx.is_null() {
        return 0 as uint64_t;
    }
    let mut d: uint64_t = (*ctx).dec_max as uint64_t;
    let mut h: uint64_t = (if (*ctx).dc.on != 0 {
        (3 as ::core::ffi::c_uint).wrapping_mul(SST_RUN)
    } else {
        3 as ::core::ffi::c_uint
    }) as uint64_t;
    if d != 0 as uint64_t
        && n_ids > (UINT64_MAX as uint64_t).wrapping_sub(h).wrapping_div(d)
    {
        return UINT64_MAX as uint64_t;
    }
    return n_ids.wrapping_mul(d).wrapping_add(h);
}
unsafe extern "C" fn sw_raw(mut w: *mut sw, mut p: *const uint8_t, mut k: uint64_t) {
    let mut n: uint64_t = (*w).n;
    (*w).n = n.wrapping_add(k);
    if n < (*w).cap {
        copy(
            (*w).out.offset(n as isize),
            p,
            if (*w).cap.wrapping_sub(n) < k { (*w).cap.wrapping_sub(n) } else { k },
        );
    }
}
unsafe extern "C" fn sw_put(
    mut w: *mut sw,
    mut strip: *const toks_spm_op,
    mut p: *const uint8_t,
    mut k: uint64_t,
) {
    while (*w).s.strip_left != 0 as uint32_t && k != 0 as uint64_t {
        if k >= (*strip).a.n as uint64_t
            && memcmp(
                p as *const ::core::ffi::c_void,
                &raw const (*strip).a.b as *const uint8_t as *const ::core::ffi::c_void,
                (*strip).a.n as size_t,
            ) == 0 as ::core::ffi::c_int
        {
            p = p.offset((*strip).a.n as isize);
            k = k.wrapping_sub((*strip).a.n as uint64_t);
            (*w).s.strip_left = (*w).s.strip_left.wrapping_sub(1);
        } else {
            (*w).s.strip_left = 0 as ::core::ffi::c_uint as uint32_t;
        }
    }
    sw_raw(w, p, k);
}
unsafe extern "C" fn sw_fffd(
    mut w: *mut sw,
    mut strip: *const toks_spm_op,
    mut k: uint64_t,
) {
    let mut i: uint64_t = 0 as uint64_t;
    while i < k {
        sw_put(w, strip, &raw const FFFD as *const uint8_t, 3 as uint64_t);
        i = i.wrapping_add(1);
    }
}
unsafe extern "C" fn sw_bad(
    mut w: *mut sw,
    mut strip: *const toks_spm_op,
    mut b: uint8_t,
) {
    if (*w).raw == 0 {
        sw_put(w, strip, &raw const FFFD as *const uint8_t, 3 as uint64_t);
        return;
    }
    if (*w).s.strip_left != 0 as uint32_t {
        if (*strip).a.n == 3 as uint32_t
            && memcmp(
                &raw const (*strip).a.b as *const uint8_t as *const ::core::ffi::c_void,
                &raw const FFFD as *const uint8_t as *const ::core::ffi::c_void,
                3 as size_t,
            ) == 0 as ::core::ffi::c_int
        {
            (*w).s.strip_left = (*w).s.strip_left.wrapping_sub(1);
            return;
        }
        (*w).s.strip_left = 0 as ::core::ffi::c_uint as uint32_t;
    }
    sw_raw(w, &raw mut b, 1 as uint64_t);
}
unsafe extern "C" fn sw_token(
    mut w: *mut sw,
    mut op: *const toks_spm_op,
    mut strip: *const toks_spm_op,
    mut p: *const uint8_t,
    mut k: uint64_t,
    mut first: ::core::ffi::c_int,
) {
    if op.is_null() {
        sw_put(w, strip, p, k);
        return;
    }
    static mut SP: uint8_t = ' ' as i32 as uint8_t;
    let mut to: *const uint8_t = &raw const (*op).b.b as *const uint8_t;
    let mut to_n: uint64_t = (*op).b.n as uint64_t;
    if (*op).kind == TOKS_SPM_D_METASPACE as ::core::ffi::c_int as uint32_t {
        to = &raw const SP;
        to_n = (if first != 0
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
            sw_put(w, strip, p.offset(from as isize), i.wrapping_sub(from));
            sw_put(w, strip, to, to_n);
            i = i.wrapping_add((*op).a.n as uint64_t);
            from = i;
        } else {
            i = i.wrapping_add(1);
        }
    }
    sw_put(w, strip, p.offset(from as isize), k.wrapping_sub(from));
}
unsafe extern "C" fn sw_byte(
    mut w: *mut sw,
    mut strip: *const toks_spm_op,
    mut c: *mut chr,
    mut b: uint8_t,
) {
    if (*c).have == 0 as uint32_t {
        (*c).need = if (b as ::core::ffi::c_uint) < 0x80 as ::core::ffi::c_uint {
            1 as uint32_t
        } else {
            seq_need(b)
        };
    }
    let fresh1 = (*c).have;
    (*c).have = (*c).have.wrapping_add(1);
    (*c).b[fresh1 as usize] = b;
    if (*c).have != (*c).need {
        return;
    }
    if !(*w).bfop.is_null() {
        sw_token(
            w,
            (*w).bfop,
            strip,
            &raw mut (*c).b as *mut uint8_t,
            (*c).have as uint64_t,
            0 as ::core::ffi::c_int,
        );
    } else {
        sw_put(w, strip, &raw mut (*c).b as *mut uint8_t, (*c).have as uint64_t);
    }
    (*c).have = 0 as ::core::ffi::c_uint as uint32_t;
}
unsafe extern "C" fn run_bytes(
    mut ctx: *const toks_ctx,
    mut ids: *const uint32_t,
    mut from: uint64_t,
    mut to: uint64_t,
    mut skip: ::core::ffi::c_int,
    mut w: *mut sw,
    mut strip: *const toks_spm_op,
    mut c: *mut chr,
    mut buf: *mut uint8_t,
) {
    let mut j: uint64_t = from;
    while j < to {
        let mut id: uint32_t = ld32(
            ids.offset(j as isize) as *const ::core::ffi::c_void,
        );
        if !(skip != 0 && toks_bit((*ctx).special_ids, id) != 0 as uint32_t) {
            if !(!(*ctx).dc.holes.is_null()
                && toks_bit((*ctx).dc.holes, id) != 0 as uint32_t)
            {
                let mut o: uint32_t = *(*ctx).t.tok_off.offset(id as isize);
                let mut b: ::core::ffi::c_int = toks_byte_token(
                    (*ctx).t.tok_bytes.offset(o as isize),
                    (*(*ctx).t.tok_off.offset(id.wrapping_add(1 as uint32_t) as isize)
                        as uint64_t)
                        .wrapping_sub(o as uint64_t),
                );
                if !(b < 0 as ::core::ffi::c_int) {
                    if !buf.is_null() {
                        let fresh0 = buf;
                        buf = buf.offset(1);
                        *fresh0 = b as uint8_t;
                    } else if !c.is_null() {
                        sw_byte(w, strip, c, b as uint8_t);
                    } else {
                        sw_bad(w, strip, b as uint8_t);
                    }
                }
            }
        }
        j = j.wrapping_add(1);
    }
}
unsafe extern "C" fn run_bad(
    mut ctx: *const toks_ctx,
    mut w: *mut sw,
    mut ids: *const uint32_t,
    mut from: uint64_t,
    mut to: uint64_t,
    mut skip: ::core::ffi::c_int,
    mut strip: *const toks_spm_op,
    mut k_run: uint64_t,
) {
    let mut np: uint64_t = sst_held(&raw mut (*w).s);
    if (*w).raw == 0 {
        sw_fffd(w, strip, np.wrapping_add(k_run));
        return;
    }
    let mut hb: *const uint8_t = sst_bytes(&raw mut (*w).s);
    let mut j: uint64_t = 0 as uint64_t;
    while j < np {
        sw_bad(w, strip, *hb.offset(j as isize));
        j = j.wrapping_add(1);
    }
    run_bytes(
        ctx,
        ids,
        from,
        to,
        skip,
        w,
        strip,
        ::core::ptr::null_mut::<chr>(),
        ::core::ptr::null_mut::<uint8_t>(),
    );
}
unsafe extern "C" fn run_end(
    mut ctx: *const toks_ctx,
    mut w: *mut sw,
    mut ids: *const uint32_t,
    mut from: uint64_t,
    mut to: uint64_t,
    mut skip: ::core::ffi::c_int,
    mut strip: *const toks_spm_op,
    mut need: uint32_t,
    mut k_run: uint64_t,
) {
    let mut ch: chr = chr {
        b: [
            0 as ::core::ffi::c_uint as uint8_t,
            0 as ::core::ffi::c_uint as uint8_t,
            0 as ::core::ffi::c_uint as uint8_t,
            0 as ::core::ffi::c_uint as uint8_t,
        ],
        have: 0 as uint32_t,
        need: 0 as uint32_t,
    };
    let mut np: uint64_t = sst_held(&raw mut (*w).s);
    if need != 0 as uint32_t {
        run_bad(ctx, w, ids, from, to, skip, strip, k_run);
        return;
    }
    let mut hb: *const uint8_t = sst_bytes(&raw mut (*w).s);
    let mut j: uint64_t = 0 as uint64_t;
    while j < np {
        sw_byte(w, strip, &raw mut ch, *hb.offset(j as isize));
        j = j.wrapping_add(1);
    }
    run_bytes(
        ctx,
        ids,
        from,
        to,
        skip,
        w,
        strip,
        &raw mut ch,
        ::core::ptr::null_mut::<uint8_t>(),
    );
}
unsafe extern "C" fn spm_push(
    mut ctx: *const toks_ctx,
    mut w: *mut sw,
    mut ids: *const uint32_t,
    mut n: uint64_t,
    mut fin: ::core::ffi::c_int,
) -> int64_t {
    let mut sp: *const toks_dchain = &raw const (*ctx).dc;
    let mut bf: ::core::ffi::c_int = 0;
    let mut strip: *const toks_spm_op = ::core::ptr::null::<toks_spm_op>();
    let mut op: *const toks_spm_op = spm_ops(sp, &raw mut bf, &raw mut strip);
    (*w).bfop = if (*sp).bf_first != 0 {
        op
    } else {
        ::core::ptr::null::<toks_spm_op>()
    };
    let mut skip: ::core::ffi::c_int = ((*w).s.flags & TOKS_SKIP_SPECIAL as uint32_t
        != 0 as uint32_t && !(*ctx).special_ids.is_null()) as ::core::ffi::c_int;
    let mut need: uint32_t = 0;
    let mut lo: uint8_t = 0;
    let mut hi: uint8_t = 0;
    sst_u8(&raw mut (*w).s, &raw mut need, &raw mut lo, &raw mut hi);
    (*w).t_from = 0 as uint64_t;
    (*w).t_k = 0 as uint64_t;
    let mut from: uint64_t = 0 as uint64_t;
    let mut k_run: uint64_t = 0 as uint64_t;
    let mut i: uint64_t = 0 as uint64_t;
    while i < n {
        let mut id: uint32_t = ld32(
            ids.offset(i as isize) as *const ::core::ffi::c_void,
        );
        if id >= (*ctx).t.n_ids {
            return TOKS_E_ID as int64_t;
        }
        if !(skip != 0 && toks_bit((*ctx).special_ids, id) != 0 as uint32_t) {
            if !(!(*sp).holes.is_null()
                && *(*sp).holes.offset((id >> 5 as ::core::ffi::c_int) as isize)
                    >> (id & 31 as uint32_t) & 1 as uint32_t != 0 as uint32_t)
            {
                let mut o: uint32_t = *(*ctx).t.tok_off.offset(id as isize);
                let mut p: *const uint8_t = (*ctx).t.tok_bytes.offset(o as isize);
                let mut k: uint64_t = (*(*ctx)
                    .t
                    .tok_off
                    .offset(id.wrapping_add(1 as uint32_t) as isize) as uint64_t)
                    .wrapping_sub(o as uint64_t);
                let mut first: ::core::ffi::c_int = ((*w).s.mode as ::core::ffi::c_uint
                    & SST_FIRST == 0 as ::core::ffi::c_uint) as ::core::ffi::c_int;
                (*w).s.mode = ((*w).s.mode as ::core::ffi::c_uint | SST_FIRST)
                    as uint8_t;
                if (*sp).has_decoder == 0 {
                    if first == 0 {
                        sw_raw(
                            w,
                            b" \0" as *const u8 as *const ::core::ffi::c_char
                                as *const uint8_t,
                            1 as uint64_t,
                        );
                    }
                    sw_raw(w, p, k);
                } else {
                    let mut b: ::core::ffi::c_int = if bf != 0 {
                        toks_byte_token(p, k)
                    } else {
                        -(1 as ::core::ffi::c_int)
                    };
                    if b >= 0 as ::core::ffi::c_int {
                        if (*w).s.mode as ::core::ffi::c_uint & (SST_VALID | SST_INVALID)
                            == 0 as ::core::ffi::c_uint
                        {
                            (*w).s.mode = ((*w).s.mode as ::core::ffi::c_uint
                                | SST_VALID) as uint8_t;
                            need = 0 as ::core::ffi::c_uint as uint32_t;
                            lo = 0x80 as uint8_t;
                            hi = 0xbf as uint8_t;
                        }
                        if (*w).s.mode as ::core::ffi::c_uint & SST_INVALID
                            != 0 as ::core::ffi::c_uint
                        {
                            sw_bad(w, strip, b as uint8_t);
                        } else {
                            if k_run == 0 as uint64_t {
                                from = i;
                            }
                            let mut c: uint8_t = b as uint8_t;
                            let mut fits: ::core::ffi::c_int = if need == 0 as uint32_t {
                                ((c as ::core::ffi::c_uint) < 0x80 as ::core::ffi::c_uint
                                    || c as ::core::ffi::c_uint >= 0xc2 as ::core::ffi::c_uint
                                        && c as ::core::ffi::c_uint <= 0xf4 as ::core::ffi::c_uint)
                                    as ::core::ffi::c_int
                            } else {
                                (c as ::core::ffi::c_int >= lo as ::core::ffi::c_int
                                    && c as ::core::ffi::c_int <= hi as ::core::ffi::c_int)
                                    as ::core::ffi::c_int
                            };
                            if fits != 0 {
                                if need == 0 as uint32_t
                                    && c as ::core::ffi::c_uint >= 0x80 as ::core::ffi::c_uint
                                {
                                    need = seq_need(c).wrapping_sub(1 as uint32_t);
                                    lo = seq_lo(c);
                                    hi = seq_hi(c);
                                } else if need != 0 as uint32_t {
                                    need = need.wrapping_sub(1);
                                    lo = 0x80 as uint8_t;
                                    hi = 0xbf as uint8_t;
                                }
                                k_run = k_run.wrapping_add(1);
                            } else {
                                run_bad(ctx, w, ids, from, i, skip, strip, k_run);
                                sw_bad(w, strip, c);
                                sst_set_held(&raw mut (*w).s, 0 as uint64_t);
                                k_run = 0 as uint64_t;
                                (*w).s.mode = ((*w).s.mode as ::core::ffi::c_uint
                                    & !SST_VALID | SST_INVALID) as uint8_t;
                            }
                        }
                    } else {
                        if (*w).s.mode as ::core::ffi::c_uint & SST_VALID
                            != 0 as ::core::ffi::c_uint
                        {
                            run_end(ctx, w, ids, from, i, skip, strip, need, k_run);
                        }
                        (*w).s.mode = ((*w).s.mode as ::core::ffi::c_int
                            & !(SST_VALID | SST_INVALID) as uint8_t
                                as ::core::ffi::c_int) as uint8_t;
                        sst_set_held(&raw mut (*w).s, 0 as uint64_t);
                        k_run = 0 as uint64_t;
                        sw_token(w, op, strip, p, k, first);
                    }
                }
            }
        }
        i = i.wrapping_add(1);
    }
    if fin != 0
        && (*w).s.mode as ::core::ffi::c_uint & SST_VALID != 0 as ::core::ffi::c_uint
    {
        run_end(ctx, w, ids, from, n, skip, strip, need, k_run);
        sst_set_held(&raw mut (*w).s, 0 as uint64_t);
    } else if (*w).s.mode as ::core::ffi::c_uint & SST_VALID != 0 as ::core::ffi::c_uint
        && k_run != 0 as uint64_t
    {
        if sst_held(&raw mut (*w).s).wrapping_add(k_run) > sst_cap(&raw mut (*w).s) {
            return TOKS_E_LIMIT as int64_t;
        }
        (*w).t_from = from;
        (*w).t_k = k_run;
    }
    if (*w).s.mode as ::core::ffi::c_uint & SST_HOLD != 0 as ::core::ffi::c_uint {
        let mut open: ::core::ffi::c_int = ((*w).s.mode as ::core::ffi::c_uint
            & SST_VALID != 0 as ::core::ffi::c_uint) as ::core::ffi::c_int;
        let mut x: sst_x = sst_x_get(&raw mut (*w).s);
        x.need = (if open != 0 {
            need as uint8_t as ::core::ffi::c_uint
        } else {
            0 as ::core::ffi::c_uint
        }) as uint8_t;
        x.lo = (if open != 0 {
            lo as ::core::ffi::c_uint
        } else {
            0 as ::core::ffi::c_uint
        }) as uint8_t;
        x.hi = (if open != 0 {
            hi as ::core::ffi::c_uint
        } else {
            0 as ::core::ffi::c_uint
        }) as uint8_t;
        sst_x_put(&raw mut (*w).s, &raw mut x);
    }
    return 0 as int64_t;
}
unsafe extern "C" fn sw_commit(
    mut ctx: *const toks_ctx,
    mut w: *mut sw,
    mut ids: *const uint32_t,
    mut n: uint64_t,
) {
    if (*w).t_k == 0 as uint64_t {
        return;
    }
    let mut skip: ::core::ffi::c_int = ((*w).s.flags & TOKS_SKIP_SPECIAL as uint32_t
        != 0 as uint32_t && !(*ctx).special_ids.is_null()) as ::core::ffi::c_int;
    let mut np: uint64_t = sst_held(&raw mut (*w).s);
    let mut hb: *mut uint8_t = if (*w).s.mode as ::core::ffi::c_uint & SST_HOLD
        != 0 as ::core::ffi::c_uint
    {
        sst_x_get(&raw mut (*w).s).p
    } else {
        &raw mut (*w).s.buf as *mut uint8_t
    };
    run_bytes(
        ctx,
        ids,
        (*w).t_from,
        n,
        skip,
        w,
        ::core::ptr::null::<toks_spm_op>(),
        ::core::ptr::null_mut::<chr>(),
        hb.offset(np as isize),
    );
    sst_set_held(&raw mut (*w).s, np.wrapping_add((*w).t_k));
}
#[no_mangle]
pub unsafe extern "C" fn toks_stream_push(
    mut ctx: *const toks_ctx,
    mut st: *mut toks_stream,
    mut ids: *const uint32_t,
    mut n: uint64_t,
    mut out: *mut uint8_t,
    mut cap: uint64_t,
) -> int64_t {
    if ctx.is_null() || st.is_null() || ids.is_null() && n != 0 as uint64_t
        || out.is_null() && cap != 0 as uint64_t
    {
        return TOKS_E_ARG as int64_t;
    }
    let mut s: sst = sst {
        tag: 0,
        flags: 0,
        strip_left: 0,
        np: 0,
        mode: 0,
        rsv: [0; 2],
        buf: [0; 44],
    };
    if sst_get(ctx, st, &raw mut s) == 0 {
        return TOKS_E_ARG as int64_t;
    }
    if (*ctx).dc.on == 0 && (*ctx).wp.is_null() && (*ctx).dec_byte_level == 0 as uint32_t
    {
        return TOKS_E_UNSUPPORTED as int64_t;
    }
    if n as ::core::ffi::c_ulonglong > TOKS_MAX_TEXT {
        return TOKS_E_LIMIT as int64_t;
    }
    if !(*ctx).wp.is_null() {
        let mut i: uint64_t = 0 as uint64_t;
        while i < n {
            if ld32(ids.offset(i as isize) as *const ::core::ffi::c_void)
                >= (*ctx).t.n_ids
            {
                return TOKS_E_ID as int64_t;
            }
            i = i.wrapping_add(1);
        }
        let mut k: uint64_t = (if s.mode as ::core::ffi::c_uint & SST_FIRST
            != 0 as ::core::ffi::c_uint
        {
            1 as ::core::ffi::c_uint
        } else {
            0 as ::core::ffi::c_uint
        }) as uint64_t;
        let mut r: int64_t = toks_wp_decode_k(
            ctx as *const toks_ctx,
            ids,
            n,
            s.flags,
            out,
            cap,
            &raw mut k,
        );
        if r < 0 as int64_t {
            return r;
        }
        if r as uint64_t > cap {
            return TOKS_E_CAP as int64_t;
        }
        if k != 0 as uint64_t {
            s.mode = SST_FIRST as uint8_t;
        }
        memcpy(
            st as *mut ::core::ffi::c_void,
            &raw mut s as *const ::core::ffi::c_void,
            ::core::mem::size_of::<sst>() as size_t,
        );
        return r;
    }
    if (*ctx).dc.on != 0 {
        let mut w: sw = sw {
            out: out,
            cap: cap,
            n: 0 as uint64_t,
            s: s,
            bfop: ::core::ptr::null::<toks_spm_op>(),
            t_from: 0 as uint64_t,
            t_k: 0 as uint64_t,
            raw: 0 as ::core::ffi::c_int,
        };
        let mut r_0: int64_t = spm_push(
            ctx,
            &raw mut w,
            ids,
            n,
            0 as ::core::ffi::c_int,
        );
        if r_0 != 0 as int64_t {
            return r_0;
        }
        if w.n > cap {
            return TOKS_E_CAP as int64_t;
        }
        sw_commit(ctx, &raw mut w, ids, n);
        memcpy(
            st as *mut ::core::ffi::c_void,
            &raw mut w.s as *const ::core::ffi::c_void,
            ::core::mem::size_of::<sst>() as size_t,
        );
        return w.n as int64_t;
    }
    if n == 1 as uint64_t && s.np as ::core::ffi::c_uint == 0 as ::core::ffi::c_uint
        && !(*ctx).dec_len.is_null() && cap >= 16 as uint64_t
    {
        let mut id: uint32_t = ld32(ids as *const ::core::ffi::c_void);
        if id < (*ctx).t.n_ids
            && *(*ctx).dec_len.offset(id as isize) as ::core::ffi::c_uint
                <= 16 as ::core::ffi::c_uint
            && (s.flags & TOKS_SKIP_SPECIAL as uint32_t == 0 as uint32_t
                || (*ctx).special_ids.is_null()
                || toks_bit((*ctx).special_ids, id) == 0 as uint32_t)
        {
            copy16(
                out,
                (*ctx)
                    .dec_slot
                    .offset((16 as uint64_t).wrapping_mul(id as uint64_t) as isize),
            );
            return *(*ctx).dec_len.offset(id as isize) as int64_t;
        }
    }
    let mut d: toks_lossy = toks_lossy {
        out: ::core::ptr::null_mut::<uint8_t>(),
        cap: 0,
        n: 0,
        np: 0,
        pend: [0; 4],
    };
    d.out = out;
    d.cap = cap;
    d.n = 0 as uint64_t;
    d.np = s.np as uint32_t;
    memcpy(
        &raw mut d.pend as *mut uint8_t as *mut ::core::ffi::c_void,
        &raw mut s.buf as *mut uint8_t as *const ::core::ffi::c_void,
        4 as size_t,
    );
    let mut r_1: int64_t = toks_lossy_ids(ctx, &raw mut d, ids, n, s.flags);
    if r_1 != 0 as int64_t {
        return r_1;
    }
    if d.n > cap {
        return TOKS_E_CAP as int64_t;
    }
    s.np = d.np as uint8_t;
    memcpy(
        &raw mut s.buf as *mut uint8_t as *mut ::core::ffi::c_void,
        &raw mut d.pend as *mut uint8_t as *const ::core::ffi::c_void,
        4 as size_t,
    );
    memcpy(
        st as *mut ::core::ffi::c_void,
        &raw mut s as *const ::core::ffi::c_void,
        ::core::mem::size_of::<sst>() as size_t,
    );
    return d.n as int64_t;
}
#[no_mangle]
pub unsafe extern "C" fn toks_stream_flush(
    mut ctx: *const toks_ctx,
    mut st: *mut toks_stream,
    mut out: *mut uint8_t,
    mut cap: uint64_t,
) -> int64_t {
    if ctx.is_null() || st.is_null() || out.is_null() && cap != 0 as uint64_t {
        return TOKS_E_ARG as int64_t;
    }
    let mut s: sst = sst {
        tag: 0,
        flags: 0,
        strip_left: 0,
        np: 0,
        mode: 0,
        rsv: [0; 2],
        buf: [0; 44],
    };
    if sst_get(ctx, st, &raw mut s) == 0 {
        return TOKS_E_ARG as int64_t;
    }
    if (*ctx).dc.on == 0 && (*ctx).wp.is_null() && (*ctx).dec_byte_level == 0 as uint32_t
    {
        return TOKS_E_UNSUPPORTED as int64_t;
    }
    let mut w: sw = sw {
        out: out,
        cap: cap,
        n: 0 as uint64_t,
        s: s,
        bfop: ::core::ptr::null::<toks_spm_op>(),
        t_from: 0 as uint64_t,
        t_k: 0 as uint64_t,
        raw: 0 as ::core::ffi::c_int,
    };
    if (*ctx).dc.on != 0 {
        let mut bf: ::core::ffi::c_int = 0;
        let mut strip: *const toks_spm_op = ::core::ptr::null::<toks_spm_op>();
        let mut op: *const toks_spm_op = spm_ops(
            &raw const (*ctx).dc,
            &raw mut bf,
            &raw mut strip,
        );
        w.bfop = if (*ctx).dc.bf_first != 0 {
            op
        } else {
            ::core::ptr::null::<toks_spm_op>()
        };
        let mut need: uint32_t = 0;
        let mut lo: uint8_t = 0;
        let mut hi: uint8_t = 0;
        sst_u8(&raw mut s, &raw mut need, &raw mut lo, &raw mut hi);
        if s.mode as ::core::ffi::c_uint & SST_VALID != 0 as ::core::ffi::c_uint {
            run_end(
                ctx,
                &raw mut w,
                ::core::ptr::null::<uint32_t>(),
                0 as uint64_t,
                0 as uint64_t,
                0 as ::core::ffi::c_int,
                strip,
                need,
                0 as uint64_t,
            );
        }
    } else if s.np as ::core::ffi::c_uint != 0 as ::core::ffi::c_uint {
        sw_raw(&raw mut w, &raw const FFFD as *const uint8_t, 3 as uint64_t);
    }
    if w.n > cap {
        return TOKS_E_CAP as int64_t;
    }
    sst_reset(ctx, &raw mut s);
    memcpy(
        st as *mut ::core::ffi::c_void,
        &raw mut s as *const ::core::ffi::c_void,
        ::core::mem::size_of::<sst>() as size_t,
    );
    return w.n as int64_t;
}
#[no_mangle]
pub unsafe extern "C" fn toks_uni_dec(
    mut ctx: *const toks_ctx,
    mut ids: *const uint32_t,
    mut n: uint64_t,
    mut flags: uint32_t,
    mut out: *mut uint8_t,
    mut cap: uint64_t,
) -> int64_t {
    let mut w: sw = sw {
        out: out,
        cap: cap,
        n: 0 as uint64_t,
        s: sst {
            tag: 0 as uint64_t,
            flags: 0,
            strip_left: 0,
            np: 0,
            mode: 0,
            rsv: [0; 2],
            buf: [0; 44],
        },
        bfop: ::core::ptr::null::<toks_spm_op>(),
        t_from: 0 as uint64_t,
        t_k: 0 as uint64_t,
        raw: (flags & TOKS_DECODE_RAW as uint32_t != 0 as uint32_t) as ::core::ffi::c_int,
    };
    w.s.flags = flags & TOKS_SKIP_SPECIAL as uint32_t;
    sst_reset(ctx, &raw mut w.s);
    let mut r: int64_t = spm_push(ctx, &raw mut w, ids, n, 1 as ::core::ffi::c_int);
    return if r != 0 as int64_t { r } else { w.n as int64_t };
}
