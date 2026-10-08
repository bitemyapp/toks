#[repr(C)]
pub struct toks_uni { _opaque: [u8; 0] }
#[repr(C)]
pub struct toks_wp_tables { _opaque: [u8; 0] }
#[repr(C)]
pub struct toks_spm_op { _opaque: [u8; 0] }
#[repr(C)]
pub struct toks_spm { _opaque: [u8; 0] }
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
    static TOKS_PATTERNS: [toks_pattern; 0];
    fn toks_round(
        ctx: *const toks_ctx,
        h: *mut toks_scratch,
        seg: *const uint8_t,
        len: uint64_t,
        pos: uint64_t,
        ends: *const uint32_t,
        n: uint64_t,
        base: uint64_t,
        e: *mut toks_emit,
        ids: ::core::ffi::c_int,
    );
    static toks_ucd_stage1: [uint16_t; 4352];
    static toks_ucd_stage2: [uint16_t; 39680];
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
pub struct toks_gen {
    pub n_steps: uint32_t,
    pub n_ins: uint32_t,
    pub n_cls: uint32_t,
    pub n_rng: uint32_t,
    pub max_ins: uint32_t,
    pub rsv: uint32_t,
    pub step: [gstep; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gstep {
    pub kind: uint8_t,
    pub beh: uint8_t,
    pub inv: uint8_t,
    pub bytes: uint8_t,
    pub pmask: uint32_t,
    pub pc0: uint32_t,
    pub len: uint32_t,
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
pub struct toks_emit {
    pub out: *mut uint32_t,
    pub cap: uint64_t,
    pub n: uint64_t,
    pub lim: uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct toks_scratch {
    pub magic: uint64_t,
    pub identity: uint64_t,
    pub base: uint64_t,
    pub bytes: uint64_t,
    pub max_len: uint64_t,
    pub off_cache: uint64_t,
    pub off_work: uint64_t,
    pub off_bounce: uint64_t,
    pub hits_static: uint64_t,
    pub hits_cache: uint64_t,
    pub misses: uint64_t,
    pub epoch: uint64_t,
    pub cache_mib: uint64_t,
    pub off_long: uint64_t,
    pub long_pos: uint64_t,
    pub long_gen: uint64_t,
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
pub struct gcls {
    pub ascii: [uint32_t; 4],
    pub r0: uint32_t,
    pub nr: uint32_t,
    pub mask: uint16_t,
    pub neg: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gins {
    pub op: uint8_t,
    pub a: uint8_t,
    pub rsv: uint16_t,
    pub x: int32_t,
    pub y: int32_t,
}
pub type C2RustUnnamed = ::core::ffi::c_uint;
pub const TOKS_GS_BYTELEVEL: C2RustUnnamed = 3;
pub const TOKS_GS_PUNCT: C2RustUnnamed = 2;
pub const TOKS_GS_DIGITS: C2RustUnnamed = 1;
pub const TOKS_GS_SPLIT: C2RustUnnamed = 0;
pub type C2RustUnnamed_0 = ::core::ffi::c_uint;
pub const TOKS_GB_CONTIGUOUS: C2RustUnnamed_0 = 4;
pub const TOKS_GB_NEXT: C2RustUnnamed_0 = 3;
pub const TOKS_GB_PREV: C2RustUnnamed_0 = 2;
pub const TOKS_GB_REMOVED: C2RustUnnamed_0 = 1;
pub const TOKS_GB_ISOLATED: C2RustUnnamed_0 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct toks_gen_spec {
    pub kind: uint32_t,
    pub beh: uint32_t,
    pub inv: uint32_t,
    pub lit: uint32_t,
    pub s: *const uint8_t,
    pub n: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gwork {
    pub ins: [gins; 4096],
    pub cls: [gcls; 512],
    pub rng: [uint32_t; 16384],
    pub stk: [uint32_t; 1026],
    pub seen: [uint8_t; 520],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rx {
    pub s: *const uint8_t,
    pub n: uint32_t,
    pub i: uint32_t,
    pub ni: uint32_t,
    pub nc: uint32_t,
    pub nr: uint32_t,
    pub icase: uint32_t,
    pub inlook: uint32_t,
    pub depth: uint32_t,
    pub ins: *mut gins,
    pub cls: *mut gcls,
    pub rng: *mut uint32_t,
    pub bad: *const ::core::ffi::c_char,
}
pub const G_LOOK: C2RustUnnamed_1 = 4;
pub const G_ASSERT: C2RustUnnamed_1 = 3;
pub const G_JMP: C2RustUnnamed_1 = 2;
pub const G_SPLIT: C2RustUnnamed_1 = 1;
pub const G_MATCH: C2RustUnnamed_1 = 5;
pub const G_CHAR: C2RustUnnamed_1 = 0;
pub const A_NWB: C2RustUnnamed_2 = 6;
pub const A_WB: C2RustUnnamed_2 = 5;
pub const A_EOSNL: C2RustUnnamed_2 = 4;
pub const A_EOS: C2RustUnnamed_2 = 3;
pub const A_BOS: C2RustUnnamed_2 = 2;
pub const A_EOL: C2RustUnnamed_2 = 1;
pub const A_BOL: C2RustUnnamed_2 = 0;
pub const GS_PRED: C2RustUnnamed_3 = 9;
pub const GS_RX: C2RustUnnamed_3 = 8;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gvm {
    pub ctx: *const toks_ctx,
    pub h: *mut toks_scratch,
    pub e: *mut toks_emit,
    pub t: *const uint8_t,
    pub len: uint64_t,
    pub base: uint64_t,
    pub a: uint64_t,
    pub b: uint64_t,
    pub pos: uint64_t,
    pub n: uint64_t,
    pub ids: ::core::ffi::c_int,
    pub bytes: uint32_t,
    pub pc0: uint32_t,
    pub stamp: [uint32_t; 2],
    pub g: *const toks_gen,
    pub ins: *const gins,
    pub cls: *const gcls,
    pub rng: *const uint32_t,
    pub mark: [*mut uint32_t; 2],
    pub list: [[*mut uint32_t; 2]; 2],
    pub stk: *mut uint32_t,
    pub ends: *mut uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gpend {
    pub s: uint64_t,
    pub e: uint64_t,
    pub on: uint32_t,
    pub m: uint32_t,
    pub prev: uint32_t,
}
pub type C2RustUnnamed_1 = ::core::ffi::c_uint;
pub type C2RustUnnamed_2 = ::core::ffi::c_uint;
pub type C2RustUnnamed_3 = ::core::ffi::c_uint;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
pub const TOKS_E_UNSUPPORTED: ::core::ffi::c_int = -(3 as ::core::ffi::c_int);
pub const TOKS_E_NOMEM: ::core::ffi::c_int = -(11 as ::core::ffi::c_int);
pub const TOKS_CHUNK_PIECES: ::core::ffi::c_int = 256 as ::core::ffi::c_int;
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
pub const TOKS_SCR_HDR: ::core::ffi::c_uint = 128 as ::core::ffi::c_uint;
#[inline]
unsafe extern "C" fn toks_align64(mut v: uint64_t) -> uint64_t {
    return v.wrapping_add(63 as uint64_t) & !(63 as ::core::ffi::c_uint as uint64_t);
}
#[inline]
unsafe extern "C" fn toks_scr_p(
    mut h: *const toks_scratch,
    mut p: *mut uint8_t,
) -> *mut uint8_t {
    #[cfg(feature = "test-guard")]
    { return crate::guard::toks_guard_scr_at(h.cast(), (p as u64).wrapping_sub((*h).base)); }
    #[cfg(not(feature = "test-guard"))]
    {
    return p;

    }
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
pub const TOKS_UCD_LU: ::core::ffi::c_uint = 0x1 as ::core::ffi::c_uint;
pub const TOKS_UCD_LL: ::core::ffi::c_uint = 0x2 as ::core::ffi::c_uint;
pub const TOKS_UCD_LT: ::core::ffi::c_uint = 0x4 as ::core::ffi::c_uint;
pub const TOKS_UCD_LM: ::core::ffi::c_uint = 0x8 as ::core::ffi::c_uint;
pub const TOKS_UCD_LO: ::core::ffi::c_uint = 0x10 as ::core::ffi::c_uint;
pub const TOKS_UCD_M: ::core::ffi::c_uint = 0x20 as ::core::ffi::c_uint;
pub const TOKS_UCD_N: ::core::ffi::c_uint = 0x40 as ::core::ffi::c_uint;
pub const TOKS_UCD_WS: ::core::ffi::c_uint = 0x80 as ::core::ffi::c_uint;
pub const TOKS_UCD_P: ::core::ffi::c_uint = 0x200 as ::core::ffi::c_uint;
pub const TOKS_UCD_S: ::core::ffi::c_uint = 0x400 as ::core::ffi::c_uint;
pub const TOKS_UCD_RNUM: ::core::ffi::c_uint = 0x800 as ::core::ffi::c_uint;
pub const TOKS_UCD_ND: ::core::ffi::c_uint = 0x1000 as ::core::ffi::c_uint;
pub const TOKS_UCD_WORD: ::core::ffi::c_uint = 0x2000 as ::core::ffi::c_uint;
pub const TOKS_UCD_PUNC: ::core::ffi::c_uint = 0x4000 as ::core::ffi::c_uint;
pub const TOKS_UCD_LETTERS: ::core::ffi::c_uint = TOKS_UCD_LU | TOKS_UCD_LL | TOKS_UCD_LT
    | TOKS_UCD_LM | TOKS_UCD_LO;
#[inline]
unsafe extern "C" fn toks_ucd_flags(mut cp: uint32_t) -> uint16_t {
    if cp > 0x10ffff as uint32_t {
        return 0 as uint16_t;
    }
    return toks_ucd_stage2[((toks_ucd_stage1[(cp >> 8 as ::core::ffi::c_int) as usize]
        as uint32_t) << 8 as ::core::ffi::c_int | cp & 0xff as uint32_t) as usize];
}
pub const G_MAX_STEP: ::core::ffi::c_uint = 512 as ::core::ffi::c_uint;
pub const G_INF: ::core::ffi::c_uint = 0xffffffff as ::core::ffi::c_uint;
pub const RX_INS: ::core::ffi::c_uint = 4096 as ::core::ffi::c_uint;
pub const RX_CLS: ::core::ffi::c_uint = 512 as ::core::ffi::c_uint;
pub const RX_RNG: ::core::ffi::c_uint = 8192 as ::core::ffi::c_uint;
unsafe extern "C" fn rx_fail(
    mut r: *mut rx,
    mut why: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if (*r).bad.is_null() {
        (*r).bad = why;
    }
    return -(1 as ::core::ffi::c_int);
}
unsafe extern "C" fn peek(mut r: *const rx) -> uint32_t {
    return if (*r).i < (*r).n {
        *(*r).s.offset((*r).i as isize) as uint32_t
    } else {
        0 as uint32_t
    };
}
unsafe extern "C" fn next_cp(mut r: *mut rx) -> uint32_t {
    let mut k: uint32_t = toks_utf8_len(
        (*r).s.offset((*r).i as isize),
        (*r).n.wrapping_sub((*r).i) as uint64_t,
    );
    let mut c: uint32_t = if k <= 1 as uint32_t {
        *(*r).s.offset((*r).i as isize) as uint32_t
    } else {
        toks_cp_decode((*r).s.offset((*r).i as isize), k)
    };
    (*r).i = ((*r).i as ::core::ffi::c_uint)
        .wrapping_add(
            (if k != 0 as uint32_t { k } else { 1 as uint32_t }) as ::core::ffi::c_uint,
        ) as uint32_t as uint32_t;
    return c;
}
unsafe extern "C" fn emit(
    mut r: *mut rx,
    mut op: uint32_t,
    mut a: uint32_t,
    mut x: int32_t,
    mut y: int32_t,
) -> ::core::ffi::c_int {
    if (*r).ni >= RX_INS as uint32_t {
        return rx_fail(
            r,
            b"pre_tokenizer Split regex: too large (generic engine)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    let fresh3 = (*r).ni;
    (*r).ni = (*r).ni.wrapping_add(1);
    *(*r).ins.offset(fresh3 as isize) = gins {
        op: op as uint8_t,
        a: a as uint8_t,
        rsv: 0 as uint16_t,
        x: x,
        y: y,
    };
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn insert(
    mut r: *mut rx,
    mut s: uint32_t,
    mut op: uint32_t,
    mut x: int32_t,
    mut y: int32_t,
) -> ::core::ffi::c_int {
    if emit(r, 0 as uint32_t, 0 as uint32_t, 0 as int32_t, 0 as int32_t)
        != 0 as ::core::ffi::c_int
    {
        return -(1 as ::core::ffi::c_int);
    }
    let mut j: uint32_t = (*r).ni.wrapping_sub(1 as uint32_t);
    while j > s {
        *(*r).ins.offset(j as isize) = *(*r)
            .ins
            .offset(j.wrapping_sub(1 as uint32_t) as isize);
        j = j.wrapping_sub(1);
    }
    *(*r).ins.offset(s as isize) = gins {
        op: op as uint8_t,
        a: 0 as uint8_t,
        rsv: 0 as uint16_t,
        x: x,
        y: y,
    };
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn add_range(
    mut r: *mut rx,
    mut lo: uint32_t,
    mut hi: uint32_t,
) -> ::core::ffi::c_int {
    if (*r).nr >= RX_RNG as uint32_t {
        return rx_fail(
            r,
            b"pre_tokenizer Split regex: too many class ranges (generic engine)\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
    }
    *(*r).rng.offset((2 as uint32_t).wrapping_mul((*r).nr) as isize) = lo;
    let fresh6 = (*r).nr;
    (*r).nr = (*r).nr.wrapping_add(1);
    *(*r)
        .rng
        .offset(
            (2 as uint32_t).wrapping_mul(fresh6).wrapping_add(1 as uint32_t) as isize,
        ) = hi;
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn esc(
    mut r: *mut rx,
    mut c: uint32_t,
    mut mask: *mut uint16_t,
    mut neg: *mut uint16_t,
    mut lit: *mut uint32_t,
) -> ::core::ffi::c_int {
    static mut PN: [*const ::core::ffi::c_char; 11] = [
        b"L\0" as *const u8 as *const ::core::ffi::c_char,
        b"Lu\0" as *const u8 as *const ::core::ffi::c_char,
        b"Ll\0" as *const u8 as *const ::core::ffi::c_char,
        b"Lt\0" as *const u8 as *const ::core::ffi::c_char,
        b"Lm\0" as *const u8 as *const ::core::ffi::c_char,
        b"Lo\0" as *const u8 as *const ::core::ffi::c_char,
        b"M\0" as *const u8 as *const ::core::ffi::c_char,
        b"N\0" as *const u8 as *const ::core::ffi::c_char,
        b"P\0" as *const u8 as *const ::core::ffi::c_char,
        b"S\0" as *const u8 as *const ::core::ffi::c_char,
        b"Nd\0" as *const u8 as *const ::core::ffi::c_char,
    ];
    static mut PM: [uint16_t; 11] = [
        TOKS_UCD_LETTERS as uint16_t,
        TOKS_UCD_LU as uint16_t,
        TOKS_UCD_LL as uint16_t,
        TOKS_UCD_LT as uint16_t,
        TOKS_UCD_LM as uint16_t,
        TOKS_UCD_LO as uint16_t,
        TOKS_UCD_M as uint16_t,
        TOKS_UCD_N as uint16_t,
        TOKS_UCD_P as uint16_t,
        TOKS_UCD_S as uint16_t,
        TOKS_UCD_ND as uint16_t,
    ];
    static mut CTL: [::core::ffi::c_char; 15] = unsafe {
        ::core::mem::transmute::<
            [u8; 15],
            [::core::ffi::c_char; 15],
        >(*b"r\rn\nt\tf\x0Cv\x0Ba\x07e\x1B\0")
    };
    let mut u: uint32_t = c | 0x20 as uint32_t;
    let mut e: uint32_t = (*r).i.wrapping_add(1 as uint32_t);
    *neg = (c >= 'A' as i32 as uint32_t && c <= 'Z' as i32 as uint32_t)
        as ::core::ffi::c_int as uint16_t;
    if u == 's' as i32 as uint32_t || u == 'd' as i32 as uint32_t
        || u == 'w' as i32 as uint32_t
    {
        *mask = (if u == 's' as i32 as uint32_t {
            TOKS_UCD_WS
        } else if u == 'd' as i32 as uint32_t {
            TOKS_UCD_ND
        } else {
            TOKS_UCD_WORD
        }) as uint16_t;
        return 1 as ::core::ffi::c_int;
    }
    let mut k: uint32_t = 0 as uint32_t;
    while CTL[k as usize] as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
        if CTL[k as usize] as uint32_t == c {
            *lit = CTL[k.wrapping_add(1 as uint32_t) as usize] as uint8_t as uint32_t;
            return 0 as ::core::ffi::c_int;
        }
        k = (k as ::core::ffi::c_uint).wrapping_add(2 as ::core::ffi::c_uint) as uint32_t
            as uint32_t;
    }
    while u == 'p' as i32 as uint32_t && peek(r) == '{' as i32 as uint32_t && e < (*r).n
        && *(*r).s.offset(e as isize) as ::core::ffi::c_int != '}' as i32
    {
        e = e.wrapping_add(1);
    }
    let mut k_0: uint32_t = 0 as uint32_t;
    while u == 'p' as i32 as uint32_t && peek(r) == '{' as i32 as uint32_t && e < (*r).n
        && k_0 < 11 as uint32_t
    {
        let mut l: uint32_t = if k_0 >= 1 as uint32_t && k_0 <= 5 as uint32_t
            || k_0 == 10 as uint32_t
        {
            2 as uint32_t
        } else {
            1 as uint32_t
        };
        if l == e.wrapping_sub((*r).i).wrapping_sub(1 as uint32_t)
            && memcmp(
                (*r).s.offset((*r).i as isize).offset(1 as ::core::ffi::c_uint as isize)
                    as *const ::core::ffi::c_void,
                PN[k_0 as usize] as *const ::core::ffi::c_void,
                l as size_t,
            ) == 0 as ::core::ffi::c_int
        {
            (*r).i = e.wrapping_add(1 as uint32_t);
            *mask = PM[k_0 as usize];
            return 1 as ::core::ffi::c_int;
        }
        k_0 = k_0.wrapping_add(1);
    }
    if c >= '0' as i32 as uint32_t && c <= '9' as i32 as uint32_t
        || u >= 'a' as i32 as uint32_t && u <= 'z' as i32 as uint32_t
    {
        return rx_fail(
            r,
            b"pre_tokenizer Split regex: an escape or property outside the census's set (generic engine)\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
    }
    *lit = c;
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn fold(
    mut r: *mut rx,
    mut lo: uint32_t,
    mut hi: uint32_t,
) -> ::core::ffi::c_int {
    if (*r).icase != 0 && hi >= 0x80 as uint32_t {
        return rx_fail(
            r,
            b"pre_tokenizer Split regex: (?i) beyond ascii letters (generic engine)\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
    }
    let mut c: uint32_t = lo;
    while (*r).icase != 0 && c <= hi {
        let mut u: uint32_t = c | 0x20 as uint32_t;
        if !(u < 'a' as i32 as uint32_t || u > 'z' as i32 as uint32_t) {
            if add_range(r, c ^ 0x20 as uint32_t, c ^ 0x20 as uint32_t)
                != 0 as ::core::ffi::c_int
                || u == 's' as i32 as uint32_t
                    && add_range(r, 0x17f as uint32_t, 0x17f as uint32_t)
                        != 0 as ::core::ffi::c_int
                || u == 'k' as i32 as uint32_t
                    && add_range(r, 0x212a as uint32_t, 0x212a as uint32_t)
                        != 0 as ::core::ffi::c_int
            {
                return -(1 as ::core::ffi::c_int);
            }
        }
        c = c.wrapping_add(1);
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn cls_items(mut r: *mut rx, mut k: *mut gcls) -> ::core::ffi::c_int {
    let mut first: uint32_t = 1 as uint32_t;
    loop {
        if (*r).i >= (*r).n {
            return rx_fail(
                r,
                b"pre_tokenizer Split regex: an unterminated class (generic engine)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
        let mut c: uint32_t = next_cp(r);
        let mut lo: uint32_t = c;
        let mut hi: uint32_t = 0;
        let mut m: uint16_t = 0 as uint16_t;
        let mut ng: uint16_t = 0 as uint16_t;
        if c == ']' as i32 as uint32_t && first == 0 {
            return 0 as ::core::ffi::c_int;
        }
        if c == '[' as i32 as uint32_t {
            if peek(r) == '^' as i32 as uint32_t || peek(r) == ':' as i32 as uint32_t
                || {
                    (*r).depth = (*r).depth.wrapping_add(1);
                    (*r).depth > 8 as uint32_t
                }
            {
                return rx_fail(
                    r,
                    b"pre_tokenizer Split regex: a negated, POSIX or deep class in a class (generic engine)\0"
                        as *const u8 as *const ::core::ffi::c_char,
                );
            }
            if cls_items(r, k) != 0 as ::core::ffi::c_int {
                return -(1 as ::core::ffi::c_int);
            }
            (*r).depth = (*r).depth.wrapping_sub(1);
        } else {
            if c == '&' as i32 as uint32_t && peek(r) == '&' as i32 as uint32_t {
                return rx_fail(
                    r,
                    b"pre_tokenizer Split regex: class intersection (generic engine)\0"
                        as *const u8 as *const ::core::ffi::c_char,
                );
            }
            let mut t: ::core::ffi::c_int = if c == '\\' as i32 as uint32_t {
                if (*r).i < (*r).n {
                    esc(r, next_cp(r), &raw mut m, &raw mut ng, &raw mut lo)
                } else {
                    rx_fail(
                        r,
                        b"pre_tokenizer Split regex: a trailing backslash (generic engine)\0"
                            as *const u8 as *const ::core::ffi::c_char,
                    )
                }
            } else {
                0 as ::core::ffi::c_int
            };
            if t < 0 as ::core::ffi::c_int
                || t == 1 as ::core::ffi::c_int
                    && (ng as ::core::ffi::c_int != 0 || (*r).icase != 0)
            {
                return rx_fail(
                    r,
                    b"pre_tokenizer Split regex: a negated or (?i) class escape in a class (generic engine)\0"
                        as *const u8 as *const ::core::ffi::c_char,
                );
            }
            if t == 1 as ::core::ffi::c_int {
                (*k).mask = ((*k).mask as ::core::ffi::c_int | m as ::core::ffi::c_int)
                    as uint16_t;
            } else {
                hi = lo;
                if peek(r) == '-' as i32 as uint32_t
                    && (*r).i.wrapping_add(1 as uint32_t) < (*r).n
                    && *(*r).s.offset((*r).i.wrapping_add(1 as uint32_t) as isize)
                        as ::core::ffi::c_int != ']' as i32
                {
                    (*r).i = (*r).i.wrapping_add(1);
                    hi = next_cp(r);
                    if hi == '[' as i32 as uint32_t
                        || hi == '\\' as i32 as uint32_t
                            && ((*r).i >= (*r).n
                                || esc(r, next_cp(r), &raw mut m, &raw mut ng, &raw mut hi)
                                    != 0 as ::core::ffi::c_int) || hi < lo
                    {
                        return rx_fail(
                            r,
                            b"pre_tokenizer Split regex: a class range that is not two characters in order (generic engine)\0"
                                as *const u8 as *const ::core::ffi::c_char,
                        );
                    }
                }
                if add_range(r, lo, hi) != 0 as ::core::ffi::c_int
                    || fold(r, lo, hi) != 0 as ::core::ffi::c_int
                {
                    return -(1 as ::core::ffi::c_int);
                }
            }
        }
        first = 0 as ::core::ffi::c_uint as uint32_t;
    };
}
unsafe extern "C" fn emit_cls(mut r: *mut rx, mut k: *mut gcls) -> ::core::ffi::c_int {
    let mut g: *mut uint32_t = (*r)
        .rng
        .offset((2 as uint32_t).wrapping_mul((*k).r0) as isize);
    let mut n: uint32_t = (*r).nr.wrapping_sub((*k).r0);
    let mut m: uint32_t = 0 as uint32_t;
    let mut i: uint32_t = 1 as uint32_t;
    while i < n {
        let mut lo: uint32_t = *g.offset((2 as uint32_t).wrapping_mul(i) as isize);
        let mut hi: uint32_t = *g
            .offset(
                (2 as uint32_t).wrapping_mul(i).wrapping_add(1 as uint32_t) as isize,
            );
        let mut j: uint32_t = i;
        while j > 0 as uint32_t
            && *g
                .offset(
                    (2 as uint32_t).wrapping_mul(j).wrapping_sub(2 as uint32_t) as isize,
                ) > lo
        {
            *g.offset((2 as uint32_t).wrapping_mul(j) as isize) = *g
                .offset(
                    (2 as uint32_t).wrapping_mul(j).wrapping_sub(2 as uint32_t) as isize,
                );
            *g
                .offset(
                    (2 as uint32_t).wrapping_mul(j).wrapping_add(1 as uint32_t) as isize,
                ) = *g
                .offset(
                    (2 as uint32_t).wrapping_mul(j).wrapping_sub(1 as uint32_t) as isize,
                );
            j = j.wrapping_sub(1);
        }
        *g.offset((2 as uint32_t).wrapping_mul(j) as isize) = lo;
        *g
            .offset(
                (2 as uint32_t).wrapping_mul(j).wrapping_add(1 as uint32_t) as isize,
            ) = hi;
        i = i.wrapping_add(1);
    }
    let mut i_0: uint32_t = 0 as uint32_t;
    while i_0 < n {
        if m > 0 as uint32_t
            && *g.offset((2 as uint32_t).wrapping_mul(i_0) as isize)
                <= (*g
                    .offset(
                        (2 as uint32_t).wrapping_mul(m).wrapping_sub(1 as uint32_t)
                            as isize,
                    ))
                    .wrapping_add(1 as uint32_t)
        {
            *g
                .offset(
                    (2 as uint32_t).wrapping_mul(m).wrapping_sub(1 as uint32_t) as isize,
                ) = if *g
                .offset(
                    (2 as uint32_t).wrapping_mul(i_0).wrapping_add(1 as uint32_t)
                        as isize,
                )
                > *g
                    .offset(
                        (2 as uint32_t).wrapping_mul(m).wrapping_sub(1 as uint32_t)
                            as isize,
                    )
            {
                *g
                    .offset(
                        (2 as uint32_t).wrapping_mul(i_0).wrapping_add(1 as uint32_t)
                            as isize,
                    )
            } else {
                *g
                    .offset(
                        (2 as uint32_t).wrapping_mul(m).wrapping_sub(1 as uint32_t)
                            as isize,
                    )
            };
        } else {
            *g.offset((2 as uint32_t).wrapping_mul(m) as isize) = *g
                .offset((2 as uint32_t).wrapping_mul(i_0) as isize);
            let fresh4 = m;
            m = m.wrapping_add(1);
            *g
                .offset(
                    (2 as uint32_t).wrapping_mul(fresh4).wrapping_add(1 as uint32_t)
                        as isize,
                ) = *g
                .offset(
                    (2 as uint32_t).wrapping_mul(i_0).wrapping_add(1 as uint32_t)
                        as isize,
                );
        }
        i_0 = i_0.wrapping_add(1);
    }
    (*r).nr = (*k).r0.wrapping_add(m);
    (*k).nr = m;
    let mut c: uint32_t = 0 as uint32_t;
    while c < 128 as uint32_t {
        let mut in_0: uint32_t = ((toks_ucd_flags(c) as ::core::ffi::c_int
            & (*k).mask as ::core::ffi::c_int) as ::core::ffi::c_uint
            != 0 as ::core::ffi::c_uint) as ::core::ffi::c_int as uint32_t;
        let mut i_1: uint32_t = 0 as uint32_t;
        while i_1 < m && in_0 == 0 {
            in_0 = (*g.offset((2 as uint32_t).wrapping_mul(i_1) as isize) <= c
                && c
                    <= *g
                        .offset(
                            (2 as uint32_t).wrapping_mul(i_1).wrapping_add(1 as uint32_t)
                                as isize,
                        )) as ::core::ffi::c_int as uint32_t;
            i_1 = i_1.wrapping_add(1);
        }
        (*k).ascii[(c >> 5 as ::core::ffi::c_int) as usize]
            |= (in_0 ^ (*k).neg as uint32_t) << (c & 31 as uint32_t);
        c = c.wrapping_add(1);
    }
    if (*r).nc >= RX_CLS as uint32_t {
        return rx_fail(
            r,
            b"pre_tokenizer Split regex: too many classes (generic engine)\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
    }
    *(*r).cls.offset((*r).nc as isize) = *k;
    let fresh5 = (*r).nc;
    (*r).nc = (*r).nc.wrapping_add(1);
    return emit(
        r,
        G_CHAR as ::core::ffi::c_int as uint32_t,
        0 as uint32_t,
        fresh5 as int32_t,
        0 as int32_t,
    );
}
unsafe extern "C" fn emit_one(
    mut r: *mut rx,
    mut c: uint32_t,
    mut mask: uint16_t,
    mut neg: uint16_t,
) -> ::core::ffi::c_int {
    let mut k: gcls = gcls {
        ascii: [
            0 as ::core::ffi::c_uint,
            0 as ::core::ffi::c_uint,
            0 as ::core::ffi::c_uint,
            0 as ::core::ffi::c_uint,
        ],
        r0: (*r).nr,
        nr: 0 as uint32_t,
        mask: mask,
        neg: neg,
    };
    if mask as ::core::ffi::c_uint != 0 as ::core::ffi::c_uint && (*r).icase != 0 {
        return rx_fail(
            r,
            b"pre_tokenizer Split regex: (?i) on a class escape (generic engine)\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if mask as ::core::ffi::c_uint == 0 as ::core::ffi::c_uint
        && (add_range(r, c, c) != 0 as ::core::ffi::c_int
            || fold(r, c, c) != 0 as ::core::ffi::c_int)
    {
        return -(1 as ::core::ffi::c_int);
    }
    return emit_cls(r, &raw mut k);
}
unsafe extern "C" fn interval(
    mut r: *mut rx,
    mut lo: *mut uint32_t,
    mut hi: *mut uint32_t,
) -> ::core::ffi::c_int {
    let mut i: uint32_t = (*r).i.wrapping_add(1 as uint32_t);
    let mut v: [uint32_t; 2] = [0 as ::core::ffi::c_uint, 0 as ::core::ffi::c_uint];
    let mut nd: [uint32_t; 2] = [0 as ::core::ffi::c_uint, 0 as ::core::ffi::c_uint];
    let mut part: uint32_t = 0 as uint32_t;
    while i < (*r).n && *(*r).s.offset(i as isize) as ::core::ffi::c_int != '}' as i32 {
        let mut c: uint8_t = *(*r).s.offset(i as isize);
        if c as ::core::ffi::c_int == ',' as i32 && part == 0 as uint32_t {
            part = 1 as ::core::ffi::c_uint as uint32_t;
        } else {
            if (c as ::core::ffi::c_int) < '0' as i32
                || c as ::core::ffi::c_int > '9' as i32
                || nd[part as usize] > 4 as uint32_t
            {
                return 0 as ::core::ffi::c_int;
            }
            v[part as usize] = v[part as usize]
                .wrapping_mul(10 as uint32_t)
                .wrapping_add((c as ::core::ffi::c_int - '0' as i32) as uint32_t);
            nd[part as usize] = nd[part as usize].wrapping_add(1);
        }
        i = i.wrapping_add(1);
    }
    if i >= (*r).n
        || nd[0 as ::core::ffi::c_int as usize] == 0 as uint32_t
            && (part == 0 as uint32_t
                || nd[1 as ::core::ffi::c_int as usize] == 0 as uint32_t)
    {
        return 0 as ::core::ffi::c_int;
    }
    *lo = v[0 as ::core::ffi::c_int as usize];
    *hi = if part == 0 as uint32_t {
        v[0 as ::core::ffi::c_int as usize]
    } else if nd[1 as ::core::ffi::c_int as usize] == 0 as uint32_t {
        G_INF as uint32_t
    } else {
        v[1 as ::core::ffi::c_int as usize]
    };
    (*r).i = i.wrapping_add(1 as uint32_t);
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn quant(
    mut r: *mut rx,
    mut s: uint32_t,
    mut lo: uint32_t,
    mut hi: uint32_t,
    mut lazy: uint32_t,
    mut poss: uint32_t,
) -> ::core::ffi::c_int {
    let mut L: uint32_t = (*r).ni.wrapping_sub(s);
    if hi < lo || hi == 0 as uint32_t {
        return rx_fail(
            r,
            b"pre_tokenizer Split regex: an empty or reversed repetition (generic engine)\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if poss != 0 {
        let mut c: gins = *(*r).ins.offset(s as isize);
        if L != 1 as uint32_t
            || c.op as ::core::ffi::c_int != G_CHAR as ::core::ffi::c_int
            || lo > 1 as uint32_t || hi != 1 as uint32_t && hi != G_INF as uint32_t
            || hi == lo
        {
            return rx_fail(
                r,
                b"pre_tokenizer Split regex: a possessive quantifier beyond one class (generic engine)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
        let mut bad: ::core::ffi::c_int = if lo == 0 as uint32_t {
            (insert(
                r,
                s,
                G_SPLIT as ::core::ffi::c_int as uint32_t,
                1 as int32_t,
                3 as int32_t,
            ) != 0
                || emit(
                    r,
                    G_JMP as ::core::ffi::c_int as uint32_t,
                    0 as uint32_t,
                    (if hi == 1 as uint32_t { 4 as int32_t } else { -(2 as int32_t) }),
                    0 as int32_t,
                ) != 0) as ::core::ffi::c_int
        } else {
            emit(
                r,
                G_SPLIT as ::core::ffi::c_int as uint32_t,
                0 as uint32_t,
                -(1 as int32_t),
                1 as int32_t,
            )
        };
        return if bad != 0
            || emit(
                r,
                G_LOOK as ::core::ffi::c_int as uint32_t,
                1 as uint32_t,
                2 as int32_t,
                0 as int32_t,
            ) != 0
            || emit(
                r,
                G_CHAR as ::core::ffi::c_int as uint32_t,
                0 as uint32_t,
                c.x,
                0 as int32_t,
            ) != 0
        {
            -(1 as ::core::ffi::c_int)
        } else {
            emit(
                r,
                G_MATCH as ::core::ffi::c_int as uint32_t,
                0 as uint32_t,
                0 as int32_t,
                0 as int32_t,
            )
        };
    }
    if lo == 1 as uint32_t && hi == 1 as uint32_t {
        return 0 as ::core::ffi::c_int;
    }
    if lo == 1 as uint32_t && hi == G_INF as uint32_t {
        return emit(
            r,
            G_SPLIT as ::core::ffi::c_int as uint32_t,
            0 as uint32_t,
            if lazy != 0 { 1 as int32_t } else { -(L as int32_t) },
            if lazy != 0 { -(L as int32_t) } else { 1 as int32_t },
        );
    }
    if lo == 0 as uint32_t && (hi == 1 as uint32_t || hi == G_INF as uint32_t) {
        let mut out: int32_t = L as int32_t
            + (if hi == 1 as uint32_t { 1 as int32_t } else { 2 as int32_t });
        if insert(
            r,
            s,
            G_SPLIT as ::core::ffi::c_int as uint32_t,
            (if lazy != 0 { out } else { 1 as int32_t }),
            (if lazy != 0 { 1 as int32_t } else { out }),
        ) != 0 as ::core::ffi::c_int
        {
            return -(1 as ::core::ffi::c_int);
        }
        return if hi == 1 as uint32_t {
            0 as ::core::ffi::c_int
        } else {
            emit(
                r,
                G_JMP as ::core::ffi::c_int as uint32_t,
                0 as uint32_t,
                -(L.wrapping_add(1 as uint32_t) as int32_t),
                0 as int32_t,
            )
        };
    }
    let mut total: uint64_t = (lo as uint64_t)
        .wrapping_mul(L as uint64_t)
        .wrapping_add(
            (if hi == G_INF as uint32_t {
                L.wrapping_add(2 as uint32_t) as uint64_t
            } else {
                (hi.wrapping_sub(lo) as uint64_t)
                    .wrapping_mul(L.wrapping_add(1 as uint32_t) as uint64_t)
            }),
        );
    if (s as uint64_t).wrapping_add(total).wrapping_add(L as uint64_t)
        > RX_INS as uint64_t
    {
        return rx_fail(
            r,
            b"pre_tokenizer Split regex: too large (generic engine)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    let mut tmp: *mut gins = (*r).ins.offset(RX_INS as isize).offset(-(L as isize));
    memcpy(
        tmp as *mut ::core::ffi::c_void,
        (*r).ins.offset(s as isize) as *const ::core::ffi::c_void,
        (L as size_t).wrapping_mul(::core::mem::size_of::<gins>() as size_t),
    );
    (*r).ni = s;
    let mut k: uint32_t = 0 as uint32_t;
    while k
        < lo
            .wrapping_add(
                (if hi == G_INF as uint32_t {
                    1 as uint32_t
                } else {
                    hi.wrapping_sub(lo)
                }),
            )
    {
        let mut skip: int32_t = if hi == G_INF as uint32_t {
            L as int32_t + 2 as int32_t
        } else {
            hi.wrapping_sub(k).wrapping_mul(L.wrapping_add(1 as uint32_t)) as int32_t
        };
        if k >= lo
            && emit(
                r,
                G_SPLIT as ::core::ffi::c_int as uint32_t,
                0 as uint32_t,
                (if lazy != 0 { skip } else { 1 as int32_t }),
                (if lazy != 0 { 1 as int32_t } else { skip }),
            ) != 0 as ::core::ffi::c_int
        {
            return -(1 as ::core::ffi::c_int);
        }
        memcpy(
            (*r).ins.offset((*r).ni as isize) as *mut ::core::ffi::c_void,
            tmp as *const ::core::ffi::c_void,
            (L as size_t).wrapping_mul(::core::mem::size_of::<gins>() as size_t),
        );
        (*r).ni = (*r).ni.wrapping_add(L);
        k = k.wrapping_add(1);
    }
    return if hi == G_INF as uint32_t {
        emit(
            r,
            G_JMP as ::core::ffi::c_int as uint32_t,
            0 as uint32_t,
            -(L.wrapping_add(1 as uint32_t) as int32_t),
            0 as int32_t,
        )
    } else {
        0 as ::core::ffi::c_int
    };
}
unsafe extern "C" fn group(
    mut r: *mut rx,
    mut zero: *mut uint32_t,
) -> ::core::ffi::c_int {
    let mut s: uint32_t = (*r).ni;
    let mut icase: uint32_t = (*r).icase;
    let mut look: uint32_t = 2 as uint32_t;
    let mut c: uint32_t = ':' as i32 as uint32_t;
    if peek(r) == '?' as i32 as uint32_t {
        (*r).i = (*r).i.wrapping_add(1);
        c = next_cp(r);
    }
    if c == '=' as i32 as uint32_t || c == '!' as i32 as uint32_t {
        if (*r).inlook != 0 {
            return rx_fail(
                r,
                b"pre_tokenizer Split regex: a lookahead inside a lookahead (generic engine)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
        look = (c == '!' as i32 as uint32_t) as ::core::ffi::c_int as uint32_t;
        *zero = 1 as ::core::ffi::c_uint as uint32_t;
        (*r).inlook = 1 as ::core::ffi::c_uint as uint32_t;
        if emit(
            r,
            G_LOOK as ::core::ffi::c_int as uint32_t,
            look,
            0 as int32_t,
            0 as int32_t,
        ) != 0 as ::core::ffi::c_int
        {
            return -(1 as ::core::ffi::c_int);
        }
    } else if c == 'i' as i32 as uint32_t && peek(r) == ':' as i32 as uint32_t {
        (*r).i = (*r).i.wrapping_add(1);
        (*r).icase = 1 as ::core::ffi::c_uint as uint32_t;
    } else if c != ':' as i32 as uint32_t {
        return rx_fail(
            r,
            b"pre_tokenizer Split regex: a group other than (?: (?i: (?= (?! (generic engine)\0"
                as *const u8 as *const ::core::ffi::c_char,
        )
    }
    (*r).depth = (*r).depth.wrapping_add(1);
    if (*r).depth > 32 as uint32_t || parse_alt(r) != 0 as ::core::ffi::c_int
        || peek(r) != ')' as i32 as uint32_t
    {
        return rx_fail(
            r,
            b"pre_tokenizer Split regex: an unbalanced or deep group (generic engine)\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
    }
    (*r).i = (*r).i.wrapping_add(1);
    (*r).depth = (*r).depth.wrapping_sub(1);
    (*r).icase = icase;
    if look < 2 as uint32_t {
        (*r).inlook = 0 as ::core::ffi::c_uint as uint32_t;
        (*(*r).ins.offset(s as isize)).x = (*r).ni.wrapping_sub(s) as int32_t;
        return emit(
            r,
            G_MATCH as ::core::ffi::c_int as uint32_t,
            0 as uint32_t,
            0 as int32_t,
            0 as int32_t,
        );
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn parse_atom(
    mut r: *mut rx,
    mut zero: *mut uint32_t,
) -> ::core::ffi::c_int {
    let mut c: uint32_t = next_cp(r);
    let mut lit: uint32_t = 0 as uint32_t;
    let mut a: uint32_t = 0;
    let mut m: uint16_t = 0 as uint16_t;
    let mut ng: uint16_t = 0 as uint16_t;
    let mut k: gcls = gcls {
        ascii: [
            0 as ::core::ffi::c_uint,
            0 as ::core::ffi::c_uint,
            0 as ::core::ffi::c_uint,
            0 as ::core::ffi::c_uint,
        ],
        r0: (*r).nr,
        nr: 0 as uint32_t,
        mask: 0 as uint16_t,
        neg: 0 as uint16_t,
    };
    match c {
        40 => return group(r, zero),
        91 => {
            if peek(r) == '^' as i32 as uint32_t {
                k.neg = 1 as uint16_t;
                (*r).i = (*r).i.wrapping_add(1);
            }
            return if cls_items(r, &raw mut k) != 0 as ::core::ffi::c_int {
                -(1 as ::core::ffi::c_int)
            } else {
                emit_cls(r, &raw mut k)
            };
        }
        46 => return emit_one(r, '\n' as i32 as uint32_t, 0 as uint16_t, 1 as uint16_t),
        94 => {
            *zero = 1 as ::core::ffi::c_uint as uint32_t;
            return emit(
                r,
                G_ASSERT as ::core::ffi::c_int as uint32_t,
                A_BOL as ::core::ffi::c_int as uint32_t,
                0 as int32_t,
                0 as int32_t,
            );
        }
        36 => {
            *zero = 1 as ::core::ffi::c_uint as uint32_t;
            return emit(
                r,
                G_ASSERT as ::core::ffi::c_int as uint32_t,
                A_EOL as ::core::ffi::c_int as uint32_t,
                0 as int32_t,
                0 as int32_t,
            );
        }
        42 | 43 | 63 => {
            return rx_fail(
                r,
                b"pre_tokenizer Split regex: a quantifier with nothing before it (generic engine)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
        92 => {}
        _ => return emit_one(r, c, 0 as uint16_t, 0 as uint16_t),
    }
    if (*r).i >= (*r).n {
        return rx_fail(
            r,
            b"pre_tokenizer Split regex: a trailing backslash (generic engine)\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
    }
    c = next_cp(r);
    a = (if c == 'A' as i32 as uint32_t {
        A_BOS as ::core::ffi::c_int as ::core::ffi::c_uint
    } else if c == 'z' as i32 as uint32_t {
        A_EOS as ::core::ffi::c_int as ::core::ffi::c_uint
    } else if c == 'Z' as i32 as uint32_t {
        A_EOSNL as ::core::ffi::c_int as ::core::ffi::c_uint
    } else if c == 'b' as i32 as uint32_t {
        A_WB as ::core::ffi::c_int as ::core::ffi::c_uint
    } else if c == 'B' as i32 as uint32_t {
        A_NWB as ::core::ffi::c_int as ::core::ffi::c_uint
    } else {
        99 as ::core::ffi::c_uint
    }) as uint32_t;
    if a != 99 as uint32_t {
        *zero = 1 as ::core::ffi::c_uint as uint32_t;
        return emit(
            r,
            G_ASSERT as ::core::ffi::c_int as uint32_t,
            a,
            0 as int32_t,
            0 as int32_t,
        );
    }
    let mut t: ::core::ffi::c_int = esc(r, c, &raw mut m, &raw mut ng, &raw mut lit);
    return if t < 0 as ::core::ffi::c_int {
        -(1 as ::core::ffi::c_int)
    } else {
        emit_one(
            r,
            lit,
            (if t == 1 as ::core::ffi::c_int {
                m as ::core::ffi::c_uint
            } else {
                0 as ::core::ffi::c_uint
            }) as uint16_t,
            (if t == 1 as ::core::ffi::c_int {
                ng as ::core::ffi::c_uint
            } else {
                0 as ::core::ffi::c_uint
            }) as uint16_t,
        )
    };
}
unsafe extern "C" fn parse_repeat(mut r: *mut rx) -> ::core::ffi::c_int {
    let mut s: uint32_t = (*r).ni;
    let mut zero: uint32_t = 0 as uint32_t;
    let mut lo: uint32_t = 0 as uint32_t;
    let mut hi: uint32_t = 1 as uint32_t;
    let mut brace: uint32_t = 0 as uint32_t;
    let mut c: uint32_t = 0;
    if parse_atom(r, &raw mut zero) != 0 as ::core::ffi::c_int {
        return -(1 as ::core::ffi::c_int);
    }
    c = peek(r);
    if c == '*' as i32 as uint32_t || c == '+' as i32 as uint32_t {
        lo = (c == '+' as i32 as uint32_t) as ::core::ffi::c_int as uint32_t;
        hi = G_INF as uint32_t;
    } else if c == '{' as i32 as uint32_t {
        brace = interval(r, &raw mut lo, &raw mut hi) as uint32_t;
        if brace == 0 {
            return 0 as ::core::ffi::c_int;
        }
    } else if c != '?' as i32 as uint32_t {
        return 0 as ::core::ffi::c_int
    }
    (*r).i = ((*r).i as ::core::ffi::c_uint)
        .wrapping_add((1 as uint32_t).wrapping_sub(brace) as ::core::ffi::c_uint)
        as uint32_t as uint32_t;
    let mut q: uint32_t = peek(r);
    let mut lazy: uint32_t = (q == '?' as i32 as uint32_t) as ::core::ffi::c_int
        as uint32_t;
    let mut poss: uint32_t = (q == '+' as i32 as uint32_t && brace == 0)
        as ::core::ffi::c_int as uint32_t;
    (*r).i = (*r).i.wrapping_add(lazy.wrapping_add(poss));
    c = peek(r);
    if zero != 0 || c == '?' as i32 as uint32_t || c == '*' as i32 as uint32_t
        || c == '+' as i32 as uint32_t
        || c == '{' as i32 as uint32_t && (*r).i.wrapping_add(1 as uint32_t) < (*r).n
            && *(*r).s.offset((*r).i.wrapping_add(1 as uint32_t) as isize)
                as ::core::ffi::c_int >= '0' as i32
            && *(*r).s.offset((*r).i.wrapping_add(1 as uint32_t) as isize)
                as ::core::ffi::c_int <= '9' as i32
    {
        return rx_fail(
            r,
            b"pre_tokenizer Split regex: a quantifier on an assertion or on a quantifier (generic engine)\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
    }
    return quant(r, s, lo, hi, lazy, poss);
}
unsafe extern "C" fn parse_alt(mut r: *mut rx) -> ::core::ffi::c_int {
    let mut s: uint32_t = (*r).ni;
    while (*r).i < (*r).n && peek(r) != '|' as i32 as uint32_t
        && peek(r) != ')' as i32 as uint32_t
    {
        if parse_repeat(r) != 0 as ::core::ffi::c_int {
            return -(1 as ::core::ffi::c_int);
        }
    }
    if peek(r) != '|' as i32 as uint32_t {
        return 0 as ::core::ffi::c_int;
    }
    (*r).i = (*r).i.wrapping_add(1);
    if insert(
        r,
        s,
        G_SPLIT as ::core::ffi::c_int as uint32_t,
        1 as int32_t,
        0 as int32_t,
    ) != 0 as ::core::ffi::c_int
    {
        return -(1 as ::core::ffi::c_int);
    }
    let mut j: uint32_t = (*r).ni;
    if emit(
        r,
        G_JMP as ::core::ffi::c_int as uint32_t,
        0 as uint32_t,
        0 as int32_t,
        0 as int32_t,
    ) != 0 as ::core::ffi::c_int || parse_alt(r) != 0 as ::core::ffi::c_int
    {
        return -(1 as ::core::ffi::c_int);
    }
    (*(*r).ins.offset(s as isize)).y = j.wrapping_add(1 as uint32_t).wrapping_sub(s)
        as int32_t;
    (*(*r).ins.offset(j as isize)).x = (*r).ni.wrapping_sub(j) as int32_t;
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn nullable(
    mut p: *const gins,
    mut n: uint32_t,
    mut seen: *mut uint8_t,
    mut stk: *mut uint32_t,
) -> ::core::ffi::c_int {
    let mut sp: uint32_t = 1 as uint32_t;
    memset(seen as *mut ::core::ffi::c_void, 0 as ::core::ffi::c_int, n as size_t);
    *stk.offset(0 as ::core::ffi::c_int as isize) = 0 as ::core::ffi::c_uint as uint32_t;
    while sp > 0 as uint32_t {
        sp = sp.wrapping_sub(1);
        let mut pc: uint32_t = *stk.offset(sp as isize);
        if pc >= n || *seen.offset(pc as isize) as ::core::ffi::c_int != 0 {
            continue;
        }
        let mut in_0: *const gins = p.offset(pc as isize) as *const gins;
        *seen.offset(pc as isize) = 1 as uint8_t;
        if (*in_0).op as ::core::ffi::c_int == G_MATCH as ::core::ffi::c_int {
            return 1 as ::core::ffi::c_int;
        }
        if (*in_0).op as ::core::ffi::c_int == G_SPLIT as ::core::ffi::c_int {
            let fresh0 = sp;
            sp = sp.wrapping_add(1);
            *stk.offset(fresh0 as isize) = (pc as int32_t + (*in_0).y) as uint32_t;
        }
        if (*in_0).op as ::core::ffi::c_int == G_SPLIT as ::core::ffi::c_int
            || (*in_0).op as ::core::ffi::c_int == G_JMP as ::core::ffi::c_int
        {
            let fresh1 = sp;
            sp = sp.wrapping_add(1);
            *stk.offset(fresh1 as isize) = (pc as int32_t + (*in_0).x) as uint32_t;
        }
        if (*in_0).op as ::core::ffi::c_int == G_ASSERT as ::core::ffi::c_int
            || (*in_0).op as ::core::ffi::c_int == G_LOOK as ::core::ffi::c_int
        {
            let fresh2 = sp;
            sp = sp.wrapping_add(1);
            *stk.offset(fresh2 as isize) = pc
                .wrapping_add(1 as uint32_t)
                .wrapping_add(
                    (if (*in_0).op as ::core::ffi::c_int == G_LOOK as ::core::ffi::c_int
                    {
                        (*in_0).x as uint32_t
                    } else {
                        0 as uint32_t
                    }),
                );
        }
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn toks_gen_max_bytes() -> uint64_t {
    return (::core::mem::size_of::<toks_gen>() as uint64_t)
        .wrapping_add(
            (RX_INS as uint64_t).wrapping_mul(::core::mem::size_of::<gins>() as uint64_t),
        )
        .wrapping_add(
            (RX_CLS as uint64_t).wrapping_mul(::core::mem::size_of::<gcls>() as uint64_t),
        )
        .wrapping_add((8 as ::core::ffi::c_uint).wrapping_mul(RX_RNG) as uint64_t);
}
#[no_mangle]
pub unsafe extern "C" fn toks_gen_compile(
    mut sp: *const toks_gen_spec,
    mut n: uint32_t,
    mut ar: *mut toks_arena,
    mut out: *mut *const toks_gen,
    mut bytes: *mut uint64_t,
    mut err: *mut toks_err,
) -> int64_t {
    let mut r: rx = rx {
        s: ::core::ptr::null::<uint8_t>(),
        n: 0,
        i: 0,
        ni: 0,
        nc: 0,
        nr: 0,
        icase: 0,
        inlook: 0,
        depth: 0,
        ins: ::core::ptr::null_mut::<gins>(),
        cls: ::core::ptr::null_mut::<gcls>(),
        rng: ::core::ptr::null_mut::<uint32_t>(),
        bad: ::core::ptr::null::<::core::ffi::c_char>(),
    };
    let mut h: toks_gen = toks_gen {
        n_steps: 0,
        n_ins: 0,
        n_cls: 0,
        n_rng: 0,
        max_ins: 0,
        rsv: 0,
        step: [gstep {
            kind: 0,
            beh: 0,
            inv: 0,
            bytes: 0,
            pmask: 0,
            pc0: 0,
            len: 0,
        }; 16],
    };
    let mut ret: int64_t = 0 as int64_t;
    memset(
        &raw mut r as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<rx>() as size_t,
    );
    memset(
        &raw mut h as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<toks_gen>() as size_t,
    );
    let mut w: *mut gwork = toks_plat_alloc(::core::mem::size_of::<gwork>() as uint64_t)
        as *mut gwork;
    if w.is_null() {
        return toks_fail(
            err,
            TOKS_E_NOMEM as int64_t,
            b"generic pre-tokenizer\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    r.ins = &raw mut (*w).ins as *mut gins;
    r.cls = &raw mut (*w).cls as *mut gcls;
    r.rng = &raw mut (*w).rng as *mut uint32_t;
    let mut current_block_39: u64;
    let mut i: uint32_t = 0 as uint32_t;
    let mut dom: uint32_t = 0 as uint32_t;
    while ret == 0 as int64_t && i < n {
        let mut q: *mut gstep = (&raw mut h.step as *mut gstep)
            .offset(h.n_steps as isize) as *mut gstep;
        let mut lit: uint32_t = (*sp.offset(i as isize)).lit;
        let mut pl: uint32_t = (*sp.offset(i as isize)).n;
        let mut pat: *const uint8_t = (*sp.offset(i as isize)).s;
        *q = gstep {
            kind: GS_RX as ::core::ffi::c_int as uint8_t,
            beh: (*sp.offset(i as isize)).beh as uint8_t,
            inv: (*sp.offset(i as isize)).inv as uint8_t,
            bytes: dom as uint8_t,
            pmask: 0 as uint32_t,
            pc0: r.ni,
            len: 0 as uint32_t,
        };
        if (*sp.offset(i as isize)).kind
            == TOKS_GS_DIGITS as ::core::ffi::c_int as uint32_t
            || (*sp.offset(i as isize)).kind
                == TOKS_GS_PUNCT as ::core::ffi::c_int as uint32_t
        {
            (*q).kind = GS_PRED as ::core::ffi::c_int as uint8_t;
            (*q).pmask = (if (*sp.offset(i as isize)).kind
                == TOKS_GS_DIGITS as ::core::ffi::c_int as uint32_t
            {
                TOKS_UCD_RNUM
            } else {
                TOKS_UCD_PUNC
            }) as uint32_t;
            h.n_steps = h.n_steps.wrapping_add(1);
        } else {
            if (*sp.offset(i as isize)).kind
                == TOKS_GS_BYTELEVEL as ::core::ffi::c_int as uint32_t
            {
                if dom != 0 {
                    ret = toks_fail(
                        err,
                        TOKS_E_UNSUPPORTED as int64_t,
                        b"pre_tokenizer ByteLevel twice (generic engine)\0" as *const u8
                            as *const ::core::ffi::c_char,
                    );
                    break;
                } else {
                    dom = 1 as ::core::ffi::c_uint as uint32_t;
                    if lit == 0 {
                        current_block_39 = 11006700562992250127;
                    } else {
                        pat = (*(&raw const TOKS_PATTERNS as *const toks_pattern)
                            .offset(0 as ::core::ffi::c_int as isize))
                            .regex as *const uint8_t;
                        pl = 0 as ::core::ffi::c_uint as uint32_t;
                        while *pat.offset(pl as isize) as ::core::ffi::c_int
                            != 0 as ::core::ffi::c_int
                        {
                            pl = pl.wrapping_add(1);
                        }
                        lit = 0 as ::core::ffi::c_uint as uint32_t;
                        (*q).beh = TOKS_GB_ISOLATED as ::core::ffi::c_int as uint8_t;
                        (*q).inv = 0 as uint8_t;
                        current_block_39 = 10652014663920648156;
                    }
                }
            } else {
                current_block_39 = 10652014663920648156;
            }
            match current_block_39 {
                11006700562992250127 => {}
                _ => {
                    r.s = pat;
                    r.n = pl;
                    r.i = 0 as ::core::ffi::c_uint as uint32_t;
                    while lit != 0 && r.i < pl {
                        emit_one(
                            &raw mut r,
                            next_cp(&raw mut r),
                            0 as uint16_t,
                            0 as uint16_t,
                        );
                    }
                    if lit == 0 && parse_alt(&raw mut r) == 0 as ::core::ffi::c_int
                        && r.i < r.n
                    {
                        rx_fail(
                            &raw mut r,
                            b"pre_tokenizer Split regex: an unbalanced group (generic engine)\0"
                                as *const u8 as *const ::core::ffi::c_char,
                        );
                    }
                    if r.bad.is_null()
                        && emit(
                            &raw mut r,
                            G_MATCH as ::core::ffi::c_int as uint32_t,
                            0 as uint32_t,
                            0 as int32_t,
                            0 as int32_t,
                        ) == 0 as ::core::ffi::c_int
                        && r.ni.wrapping_sub((*q).pc0) > G_MAX_STEP as uint32_t
                    {
                        rx_fail(
                            &raw mut r,
                            b"pre_tokenizer Split regex: over 512 instructions a step (generic engine)\0"
                                as *const u8 as *const ::core::ffi::c_char,
                        );
                    }
                    if r.bad.is_null()
                        && (pl == 0 as uint32_t
                            || nullable(
                                r.ins.offset((*q).pc0 as isize),
                                r.ni.wrapping_sub((*q).pc0),
                                &raw mut (*w).seen as *mut uint8_t,
                                &raw mut (*w).stk as *mut uint32_t,
                            ) != 0)
                    {
                        rx_fail(
                            &raw mut r,
                            b"pre_tokenizer Split regex: one that matches the empty string (generic engine)\0"
                                as *const u8 as *const ::core::ffi::c_char,
                        );
                    }
                    if !r.bad.is_null() {
                        ret = toks_fail(err, TOKS_E_UNSUPPORTED as int64_t, r.bad);
                        break;
                    } else {
                        (*q).len = r.ni.wrapping_sub((*q).pc0);
                        h.max_ins = if (*q).len > h.max_ins {
                            (*q).len
                        } else {
                            h.max_ins
                        };
                        h.n_steps = h.n_steps.wrapping_add(1);
                    }
                }
            }
        }
        i = i.wrapping_add(1);
    }
    h.n_ins = r.ni;
    h.n_cls = r.nc;
    h.n_rng = r.nr;
    let mut ci: uint64_t = (::core::mem::size_of::<toks_gen>() as uint64_t)
        .wrapping_add(
            (r.ni as uint64_t).wrapping_mul(::core::mem::size_of::<gins>() as uint64_t),
        );
    let mut cr: uint64_t = ci
        .wrapping_add(
            (r.nc as uint64_t).wrapping_mul(::core::mem::size_of::<gcls>() as uint64_t),
        );
    let mut total: uint64_t = cr
        .wrapping_add((8 as uint64_t).wrapping_mul(r.nr as uint64_t));
    let mut b: *mut uint8_t = if ret == 0 as int64_t {
        toks_ar_alloc(ar, total, 8 as uint64_t) as *mut uint8_t
    } else {
        ::core::ptr::null_mut::<uint8_t>()
    };
    if ret == 0 as int64_t && b.is_null() {
        ret = toks_fail(
            err,
            TOKS_E_NOMEM as int64_t,
            b"generic pre-tokenizer\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if ret == 0 as int64_t {
        memcpy(
            b as *mut ::core::ffi::c_void,
            &raw mut h as *const ::core::ffi::c_void,
            ::core::mem::size_of::<toks_gen>() as size_t,
        );
        memcpy(
            b.offset(::core::mem::size_of::<toks_gen>() as usize as isize)
                as *mut ::core::ffi::c_void,
            r.ins as *const ::core::ffi::c_void,
            ci.wrapping_sub(::core::mem::size_of::<toks_gen>() as uint64_t) as size_t,
        );
        memcpy(
            b.offset(ci as isize) as *mut ::core::ffi::c_void,
            r.cls as *const ::core::ffi::c_void,
            cr.wrapping_sub(ci) as size_t,
        );
        memcpy(
            b.offset(cr as isize) as *mut ::core::ffi::c_void,
            r.rng as *const ::core::ffi::c_void,
            total.wrapping_sub(cr) as size_t,
        );
        *out = b as *const ::core::ffi::c_void as *const toks_gen;
        *bytes = total;
    }
    toks_plat_free(
        w as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<gwork>() as uint64_t,
    );
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn toks_gen_scr(mut g: *const toks_gen) -> uint64_t {
    return (64 as uint64_t)
        .wrapping_add(
            (4 as uint64_t)
                .wrapping_mul(
                    (14 as uint64_t)
                        .wrapping_mul((*g).max_ins as uint64_t)
                        .wrapping_add(4 as uint64_t),
                ),
        );
}
unsafe extern "C" fn byte_char(mut b: uint32_t) -> uint32_t {
    return if b <= 0x20 as uint32_t {
        (256 as uint32_t).wrapping_add(b)
    } else if b >= 0x7f as uint32_t && b <= 0xa0 as uint32_t {
        b.wrapping_add(210 as uint32_t)
    } else if b == 0xad as uint32_t {
        323 as uint32_t
    } else {
        b
    };
}
unsafe extern "C" fn atom(
    mut v: *const gvm,
    mut p: uint64_t,
    mut k: *mut uint32_t,
) -> uint32_t {
    let mut c: uint32_t = *(*v).t.offset(p as isize) as uint32_t;
    let mut n: uint32_t = if (*v).bytes != 0 {
        1 as uint32_t
    } else {
        toks_utf8_len((*v).t.offset(p as isize), (*v).b.wrapping_sub(p))
    };
    *k = if n != 0 as uint32_t { n } else { 1 as uint32_t };
    return if (*v).bytes != 0 {
        byte_char(c)
    } else if n == 0 as uint32_t {
        (0x110000 as uint32_t).wrapping_add(c)
    } else if n == 1 as uint32_t {
        c
    } else {
        toks_cp_decode((*v).t.offset(p as isize), n)
    };
}
unsafe extern "C" fn atom_before(mut v: *const gvm, mut p: uint64_t) -> uint32_t {
    let mut s: uint64_t = p.wrapping_sub(1 as uint64_t);
    let mut k: uint32_t = 0;
    while (*v).bytes == 0 && s > (*v).a && s.wrapping_add(3 as uint64_t) >= p
        && *(*v).t.offset(s as isize) as ::core::ffi::c_uint
            & 0xc0 as ::core::ffi::c_uint == 0x80 as ::core::ffi::c_uint
    {
        s = s.wrapping_sub(1);
    }
    let mut c: uint32_t = atom(v, s, &raw mut k);
    return if s.wrapping_add(k as uint64_t) == p {
        c
    } else {
        (0x110000 as uint32_t)
            .wrapping_add(
                *(*v).t.offset(p.wrapping_sub(1 as uint64_t) as isize) as uint32_t,
            )
    };
}
unsafe extern "C" fn has(
    mut v: *const gvm,
    mut k: *const gcls,
    mut c: uint32_t,
) -> ::core::ffi::c_int {
    if c < 128 as uint32_t {
        return ((*k).ascii[(c >> 5 as ::core::ffi::c_int) as usize]
            >> (c & 31 as uint32_t) & 1 as uint32_t) as ::core::ffi::c_int;
    }
    let mut in_0: uint32_t = (if c < 0x110000 as uint32_t {
        toks_ucd_flags(c) as uint32_t
    } else {
        TOKS_UCD_P as uint32_t
    }) & (*k).mask as uint32_t;
    let mut lo: uint32_t = 0 as uint32_t;
    let mut hi: uint32_t = if c < 0x110000 as uint32_t {
        (*k).nr
    } else {
        0 as uint32_t
    };
    let mut r: *const uint32_t = (*v)
        .rng
        .offset((2 as uint32_t).wrapping_mul((*k).r0) as isize);
    while in_0 == 0 as uint32_t && lo < hi {
        let mut m: uint32_t = lo.wrapping_add(hi) >> 1 as ::core::ffi::c_int;
        if *r
            .offset((2 as uint32_t).wrapping_mul(m).wrapping_add(1 as uint32_t) as isize)
            < c
        {
            lo = m.wrapping_add(1 as uint32_t);
        } else if *r.offset((2 as uint32_t).wrapping_mul(m) as isize) > c {
            hi = m;
        } else {
            in_0 = 1 as ::core::ffi::c_uint as uint32_t;
        }
    }
    return (in_0 != 0 as uint32_t) as ::core::ffi::c_int
        ^ (*k).neg as ::core::ffi::c_int;
}
unsafe extern "C" fn word(mut c: uint32_t) -> ::core::ffi::c_int {
    return (c < 0x110000 as uint32_t
        && toks_ucd_flags(c) as ::core::ffi::c_uint & TOKS_UCD_WORD
            != 0 as ::core::ffi::c_uint) as ::core::ffi::c_int;
}
unsafe extern "C" fn assert_ok(
    mut v: *const gvm,
    mut kind: uint32_t,
    mut p: uint64_t,
) -> ::core::ffi::c_int {
    let mut k: uint32_t = 0;
    match kind {
        0 => {
            return (p == (*v).a || atom_before(v, p) == 0xa as uint32_t)
                as ::core::ffi::c_int;
        }
        1 => {
            return (p == (*v).b || atom(v, p, &raw mut k) == 0xa as uint32_t)
                as ::core::ffi::c_int;
        }
        2 => return (p == (*v).a) as ::core::ffi::c_int,
        3 => return (p == (*v).b) as ::core::ffi::c_int,
        4 => {
            return (p == (*v).b
                || atom(v, p, &raw mut k) == 0xa as uint32_t
                    && p.wrapping_add(k as uint64_t) == (*v).b) as ::core::ffi::c_int;
        }
        _ => {
            return (((p > (*v).a && word(atom_before(v, p)) != 0) as ::core::ffi::c_int
                != (p < (*v).b && word(atom(v, p, &raw mut k)) != 0)
                    as ::core::ffi::c_int) as ::core::ffi::c_int
                == (kind == A_WB as ::core::ffi::c_int as uint32_t)
                    as ::core::ffi::c_int) as ::core::ffi::c_int;
        }
    };
}
unsafe extern "C" fn add(
    mut v: *mut gvm,
    mut L: *mut uint32_t,
    mut n: uint32_t,
    mut pc: uint32_t,
    mut st: uint32_t,
    mut p: uint64_t,
    mut lk: uint32_t,
) -> uint32_t {
    let mut sp: uint32_t = 0 as uint32_t;
    let mut mark: *mut uint32_t = (*v).mark[lk as usize];
    let mut stamp: uint32_t = (*v).stamp[lk as usize];
    let mut d0: uint64_t = 0;
    let mut d1: uint64_t = 0;
    let fresh8 = sp;
    sp = sp.wrapping_add(1);
    *(*v).stk.offset(fresh8 as isize) = pc;
    while sp > 0 as uint32_t {
        sp = sp.wrapping_sub(1);
        pc = *(*v).stk.offset(sp as isize);
        let mut in_0: *const gins = (*v).ins.offset(pc as isize) as *const gins;
        if *mark.offset(pc.wrapping_sub((*v).pc0) as isize) == stamp {
            continue;
        }
        *mark.offset(pc.wrapping_sub((*v).pc0) as isize) = stamp;
        if (*in_0).op as ::core::ffi::c_int == G_SPLIT as ::core::ffi::c_int {
            let fresh9 = sp;
            sp = sp.wrapping_add(1);
            *(*v).stk.offset(fresh9 as isize) = (pc as int32_t + (*in_0).y) as uint32_t;
        }
        if (*in_0).op as ::core::ffi::c_int == G_SPLIT as ::core::ffi::c_int
            || (*in_0).op as ::core::ffi::c_int == G_JMP as ::core::ffi::c_int
        {
            let fresh10 = sp;
            sp = sp.wrapping_add(1);
            *(*v).stk.offset(fresh10 as isize) = (pc as int32_t + (*in_0).x) as uint32_t;
        } else if (*in_0).op as ::core::ffi::c_int == G_ASSERT as ::core::ffi::c_int {
            if assert_ok(v, (*in_0).a as uint32_t, p) != 0 {
                let fresh11 = sp;
                sp = sp.wrapping_add(1);
                *(*v).stk.offset(fresh11 as isize) = pc.wrapping_add(1 as uint32_t);
            }
        } else if (*in_0).op as ::core::ffi::c_int == G_LOOK as ::core::ffi::c_int {
            if search(
                v,
                pc.wrapping_add(1 as uint32_t),
                p,
                1 as uint32_t,
                &raw mut d0,
                &raw mut d1,
            ) as uint32_t != (*in_0).a as uint32_t
            {
                let fresh12 = sp;
                sp = sp.wrapping_add(1);
                *(*v).stk.offset(fresh12 as isize) = pc
                    .wrapping_add(1 as uint32_t)
                    .wrapping_add((*in_0).x as uint32_t);
            }
        } else {
            *L.offset((2 as uint32_t).wrapping_mul(n) as isize) = pc;
            let fresh13 = n;
            n = n.wrapping_add(1);
            *L
                .offset(
                    (2 as uint32_t).wrapping_mul(fresh13).wrapping_add(1 as uint32_t)
                        as isize,
                ) = st;
        }
    }
    return n;
}
unsafe extern "C" fn next_stamp(mut v: *mut gvm, mut lk: uint32_t) {
    (*v).stamp[lk as usize] = (*v).stamp[lk as usize].wrapping_add(1);
    if (*v).stamp[lk as usize] == 0 as uint32_t {
        memset(
            (*v).mark[lk as usize] as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ((*(*v).g).max_ins as size_t).wrapping_mul(4 as size_t),
        );
        (*v).stamp[lk as usize] = 1 as ::core::ffi::c_uint as uint32_t;
    }
}
unsafe extern "C" fn search(
    mut v: *mut gvm,
    mut pc0: uint32_t,
    mut p: uint64_t,
    mut lk: uint32_t,
    mut ms: *mut uint64_t,
    mut me: *mut uint64_t,
) -> ::core::ffi::c_int {
    let mut cl: *mut uint32_t = (*v).list[lk as usize][0 as ::core::ffi::c_int as usize];
    let mut nl: *mut uint32_t = (*v).list[lk as usize][1 as ::core::ffi::c_int as usize];
    let mut t: *mut uint32_t = ::core::ptr::null_mut::<uint32_t>();
    let mut stk: *mut uint32_t = (*v).stk;
    let mut nc: uint32_t = 0 as uint32_t;
    let mut nn: uint32_t = 0;
    let mut k: uint32_t = 0 as uint32_t;
    let mut cp: uint32_t = 0 as uint32_t;
    let mut found: uint32_t = 0 as uint32_t;
    (*v).stk = (*v)
        .stk
        .offset(
            lk
                .wrapping_mul(
                    (2 as uint32_t)
                        .wrapping_mul((*(*v).g).max_ins)
                        .wrapping_add(2 as uint32_t),
                ) as isize,
        );
    next_stamp(v, lk);
    let mut first: uint32_t = 1 as uint32_t;
    loop {
        if found == 0 && (first != 0 || lk == 0) {
            nc = add(v, cl, nc, pc0, p as uint32_t, p, lk);
        }
        if p < (*v).b {
            cp = atom(v, p, &raw mut k);
        }
        next_stamp(v, lk);
        nn = 0 as ::core::ffi::c_uint as uint32_t;
        let mut i: uint32_t = 0 as uint32_t;
        while i < nc {
            let mut in_0: *const gins = (*v)
                .ins
                .offset(*cl.offset((2 as uint32_t).wrapping_mul(i) as isize) as isize)
                as *const gins;
            if (*in_0).op as ::core::ffi::c_int == G_MATCH as ::core::ffi::c_int {
                *ms = *cl
                    .offset(
                        (2 as uint32_t).wrapping_mul(i).wrapping_add(1 as uint32_t)
                            as isize,
                    ) as uint64_t;
                *me = p;
                found = 1 as ::core::ffi::c_uint as uint32_t;
                break;
            } else {
                if p < (*v).b
                    && has(v, (*v).cls.offset((*in_0).x as isize) as *const gcls, cp)
                        != 0
                {
                    nn = add(
                        v,
                        nl,
                        nn,
                        (*cl.offset((2 as uint32_t).wrapping_mul(i) as isize))
                            .wrapping_add(1 as uint32_t),
                        *cl
                            .offset(
                                (2 as uint32_t).wrapping_mul(i).wrapping_add(1 as uint32_t)
                                    as isize,
                            ),
                        p.wrapping_add(k as uint64_t),
                        lk,
                    );
                }
                i = i.wrapping_add(1);
            }
        }
        if p >= (*v).b || found != 0 && (lk != 0 || nn == 0 as uint32_t)
            || lk != 0 && nn == 0 as uint32_t
        {
            break;
        }
        t = cl;
        cl = nl;
        nl = t;
        nc = nn;
        p = p.wrapping_add(k as uint64_t);
        first = 0 as ::core::ffi::c_uint as uint32_t;
    }
    (*v).stk = stk;
    return found as ::core::ffi::c_int;
}
unsafe extern "C" fn flush(mut v: *mut gvm) {
    if (*v).n != 0 as uint64_t {
        toks_round(
            (*v).ctx as *const toks_ctx,
            (*v).h,
            (*v).t,
            (*v).len,
            (*v).pos,
            (*v).ends,
            (*v).n,
            (*v).base,
            (*v).e,
            (*v).ids,
        );
    }
    (*v).n = 0 as uint64_t;
}
unsafe extern "C" fn char_end(mut v: *const gvm, mut e: uint64_t) -> uint64_t {
    let mut s: uint64_t = e.wrapping_sub(1 as uint64_t);
    if e >= (*v).len
        || *(*v).t.offset(e as isize) as ::core::ffi::c_uint
            & 0xc0 as ::core::ffi::c_uint != 0x80 as ::core::ffi::c_uint
    {
        return e;
    }
    while s > 0 as uint64_t && e.wrapping_sub(s) < 3 as uint64_t
        && *(*v).t.offset(s as isize) as ::core::ffi::c_uint
            & 0xc0 as ::core::ffi::c_uint == 0x80 as ::core::ffi::c_uint
    {
        s = s.wrapping_sub(1);
    }
    let mut k: uint32_t = toks_utf8_len(
        (*v).t.offset(s as isize),
        (*v).len.wrapping_sub(s),
    );
    return if k != 0 as uint32_t && s.wrapping_add(k as uint64_t) > e {
        s.wrapping_add(k as uint64_t)
    } else {
        e
    };
}
unsafe extern "C" fn pend_out(mut v: *mut gvm, mut i: uint32_t, mut q: *mut gpend) {
    if (*q).on != 0 {
        piece(v, i.wrapping_add(1 as uint32_t), (*q).s, (*q).e);
    }
    (*q).on = 0 as ::core::ffi::c_uint as uint32_t;
}
unsafe extern "C" fn elem(
    mut v: *mut gvm,
    mut i: uint32_t,
    mut q: *mut gpend,
    mut s: uint64_t,
    mut e: uint64_t,
    mut m: uint32_t,
) {
    let mut beh: uint32_t = (*(*v).g).step[i as usize].beh as uint32_t;
    m ^= (*(*v).g).step[i as usize].inv as uint32_t;
    if beh == TOKS_GB_ISOLATED as ::core::ffi::c_int as uint32_t
        || beh == TOKS_GB_REMOVED as ::core::ffi::c_int as uint32_t && m == 0
    {
        piece(v, i.wrapping_add(1 as uint32_t), s, e);
    } else if beh == TOKS_GB_PREV as ::core::ffi::c_int as uint32_t {
        if m != 0 && (*q).prev == 0 && (*q).on != 0 {
            (*q).e = e;
        } else {
            pend_out(v, i, q);
            *q = gpend {
                s: s,
                e: e,
                on: 1 as uint32_t,
                m: m,
                prev: 0 as uint32_t,
            };
        }
        (*q).prev = m;
    } else if beh == TOKS_GB_NEXT as ::core::ffi::c_int as uint32_t {
        if (*q).on != 0 && m == 0 {
            piece(v, i.wrapping_add(1 as uint32_t), (*q).s, e);
            (*q).on = 0 as ::core::ffi::c_uint as uint32_t;
            return;
        }
        pend_out(v, i, q);
        if m != 0 {
            *q = gpend {
                s: s,
                e: e,
                on: 1 as uint32_t,
                m: 1 as uint32_t,
                prev: 0 as uint32_t,
            };
        } else {
            piece(v, i.wrapping_add(1 as uint32_t), s, e);
        }
    } else if beh == TOKS_GB_CONTIGUOUS as ::core::ffi::c_int as uint32_t {
        if (*q).on != 0 && (*q).m == m {
            (*q).e = e;
        } else {
            pend_out(v, i, q);
            *q = gpend {
                s: s,
                e: e,
                on: 1 as uint32_t,
                m: m,
                prev: 0 as uint32_t,
            };
        }
    }
}
unsafe extern "C" fn piece(
    mut v: *mut gvm,
    mut i: uint32_t,
    mut s: uint64_t,
    mut e: uint64_t,
) {
    if s >= e {
        return;
    }
    if i == (*(*v).g).n_steps {
        if (*v).n != 0 as uint64_t
            && s
                != *(*v).ends.offset((*v).n.wrapping_sub(1 as uint64_t) as isize)
                    as uint64_t
        {
            flush(v);
        }
        if (*v).n == 0 as uint64_t {
            (*v).pos = s;
        }
        let fresh7 = (*v).n;
        (*v).n = (*v).n.wrapping_add(1);
        *(*v).ends.offset(fresh7 as isize) = (if (*v).ids != 0 {
            e
        } else {
            char_end(v, e)
        }) as uint32_t;
        if (*v).n == TOKS_CHUNK_PIECES as uint64_t {
            flush(v);
        }
        return;
    }
    let mut st: *const gstep = (&raw const (*(*v).g).step as *const gstep)
        .offset(i as isize) as *const gstep;
    let mut q: gpend = gpend {
        s: 0 as uint64_t,
        e: 0 as uint64_t,
        on: 0 as uint32_t,
        m: 0 as uint32_t,
        prev: 0 as uint32_t,
    };
    let mut p: uint64_t = s;
    let mut g: uint64_t = s;
    let mut ms: uint64_t = 0 as uint64_t;
    let mut me: uint64_t = 0 as uint64_t;
    let mut k: uint32_t = 0 as uint32_t;
    let mut c: uint32_t = 0;
    loop {
        (*v).a = s;
        (*v).b = e;
        (*v).bytes = (*st).bytes as uint32_t;
        (*v).pc0 = (*st).pc0;
        if (*st).kind as ::core::ffi::c_int == GS_RX as ::core::ffi::c_int
            && (p >= e
                || search(v, (*st).pc0, p, 0 as uint32_t, &raw mut ms, &raw mut me) == 0)
        {
            break;
        }
        while (*st).kind as ::core::ffi::c_int == GS_PRED as ::core::ffi::c_int && p < e
        {
            c = atom(v, p, &raw mut k);
            if if c < 0x110000 as uint32_t {
                (toks_ucd_flags(c) as uint32_t & (*st).pmask != 0 as uint32_t)
                    as ::core::ffi::c_int
            } else {
                ((*st).pmask == TOKS_UCD_PUNC as uint32_t) as ::core::ffi::c_int
            } != 0
            {
                break;
            }
            p = p.wrapping_add(k as uint64_t);
        }
        if (*st).kind as ::core::ffi::c_int == GS_PRED as ::core::ffi::c_int && p >= e {
            break;
        }
        if (*st).kind as ::core::ffi::c_int == GS_PRED as ::core::ffi::c_int {
            ms = p;
            me = p.wrapping_add(k as uint64_t);
        }
        p = me;
        if ms > g {
            elem(v, i, &raw mut q, g, ms, 0 as uint32_t);
        }
        elem(v, i, &raw mut q, ms, me, 1 as uint32_t);
        g = me;
    }
    if g < e {
        elem(v, i, &raw mut q, g, e, 0 as uint32_t);
    }
    pend_out(v, i, &raw mut q);
}
#[no_mangle]
pub unsafe extern "C" fn toks_gen_run(
    mut ctx: *const toks_ctx,
    mut h: *mut toks_scratch,
    mut seg: *const uint8_t,
    mut len: uint64_t,
    mut base: uint64_t,
    mut e: *mut toks_emit,
    mut ids: ::core::ffi::c_int,
) {
    let mut g: *const toks_gen = (*ctx).gen as *const toks_gen;
    let mut M: uint32_t = (*g).max_ins;
    let mut w: *mut uint32_t = toks_align64(
        toks_scr_p(
            h,
            (*h).base.wrapping_add((*h).off_bounce).wrapping_sub((*ctx).scr_extra)
                as uintptr_t as *mut uint8_t,
        ) as uintptr_t as uint64_t,
    ) as uintptr_t as *mut uint32_t;
    let mut v: gvm = gvm {
        ctx: ctx,
        h: h,
        e: e,
        t: seg,
        len: len,
        base: base,
        a: 0 as uint64_t,
        b: 0 as uint64_t,
        pos: 0 as uint64_t,
        n: 0 as uint64_t,
        ids: ids,
        bytes: 0 as uint32_t,
        pc0: 0 as uint32_t,
        stamp: [0 as ::core::ffi::c_uint, 0 as ::core::ffi::c_uint],
        g: g,
        ins: g.offset(1 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_void
            as *const gins,
        cls: (g.offset(1 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_void
            as *const gins)
            .offset((*g).n_ins as isize) as *const ::core::ffi::c_void as *const gcls,
        rng: ((g.offset(1 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_void
            as *const gins)
            .offset((*g).n_ins as isize) as *const ::core::ffi::c_void as *const gcls)
            .offset((*g).n_cls as isize) as *const ::core::ffi::c_void
            as *const uint32_t,
        mark: [w, w.offset(M as isize)],
        list: [
            [
                w.offset((2 as uint32_t).wrapping_mul(M) as isize),
                w.offset((4 as uint32_t).wrapping_mul(M) as isize),
            ],
            [
                w.offset((6 as uint32_t).wrapping_mul(M) as isize),
                w.offset((8 as uint32_t).wrapping_mul(M) as isize),
            ],
        ],
        stk: w.offset((10 as uint32_t).wrapping_mul(M) as isize),
        ends: toks_scr_p(h, (h as *mut uint8_t).offset(TOKS_SCR_HDR as isize))
            as *mut ::core::ffi::c_void as *mut uint32_t,
    };
    memset(
        w as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        (M as size_t).wrapping_mul(8 as size_t),
    );
    piece(&raw mut v, 0 as uint32_t, 0 as uint64_t, len);
    flush(&raw mut v);
}
