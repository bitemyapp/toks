#[repr(C)]
pub struct toks_gen { _opaque: [u8; 0] }
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
    fn toks_nfc_boundary(
        f: uint32_t,
        text: *const uint8_t,
        len: uint64_t,
        i: uint64_t,
    ) -> ::core::ffi::c_int;
    fn toks_spm_cut(
        s: *const toks_spm,
        text: *const uint8_t,
        len: uint64_t,
        a: uint64_t,
        c: uint64_t,
    ) -> ::core::ffi::c_int;
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
pub struct toks_cuts {
    pub family: uint32_t,
    pub rsv: uint32_t,
    pub maxlen: uint64_t,
    pub win: uint64_t,
}
pub const TOKS_SPM_PFX_GAP: C2RustUnnamed = 1;
pub type C2RustUnnamed = ::core::ffi::c_uint;
pub const TOKS_SPM_PFX_FIRST: C2RustUnnamed = 3;
pub const TOKS_SPM_PFX_ALWAYS: C2RustUnnamed = 2;
pub const TOKS_SPM_PFX_NONE: C2RustUnnamed = 0;
pub const TOKS_E_LIMIT: ::core::ffi::c_int = -(9 as ::core::ffi::c_int);
pub const TOKS_E_ARG: ::core::ffi::c_int = -(10 as ::core::ffi::c_int);
pub const TOKS_MAX_TEXT: ::core::ffi::c_ulonglong = (1 as ::core::ffi::c_ulonglong)
    << 29 as ::core::ffi::c_int;
pub const TOKS_ADDED_NONSPECIAL: ::core::ffi::c_uint = 1 as ::core::ffi::c_uint;
pub const TOKS_ADDED_NONE: ::core::ffi::c_uint = 2 as ::core::ffi::c_uint;
pub const TOKS_ADDED_MASK: ::core::ffi::c_uint = 3 as ::core::ffi::c_uint;
pub const TOKS_NO_POSTPROCESS: ::core::ffi::c_uint = 4 as ::core::ffi::c_uint;
pub const TOKS_CONTINUATION: ::core::ffi::c_uint = 8 as ::core::ffi::c_uint;
pub const TOKS_NO_TRUNCATE: ::core::ffi::c_uint = 16 as ::core::ffi::c_uint;
pub const TOKS_NO_PAD: ::core::ffi::c_uint = 32 as ::core::ffi::c_uint;
pub const TOKS_C_BASE_MASK: ::core::ffi::c_uint = 0x7 as ::core::ffi::c_uint;
pub const TOKS_C_P: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
pub const TOKS_C_L: ::core::ffi::c_uint = 1 as ::core::ffi::c_uint;
pub const TOKS_C_N: ::core::ffi::c_uint = 2 as ::core::ffi::c_uint;
pub const TOKS_C_WS: ::core::ffi::c_uint = 3 as ::core::ffi::c_uint;
pub const TOKS_C_NL: ::core::ffi::c_uint = 4 as ::core::ffi::c_uint;
pub const TOKS_C_X: ::core::ffi::c_uint = 5 as ::core::ffi::c_uint;
pub const TOKS_C_UPPER: ::core::ffi::c_uint = 0x8 as ::core::ffi::c_uint;
pub const TOKS_C_LOWER: ::core::ffi::c_uint = 0x10 as ::core::ffi::c_uint;
pub const TOKS_C_HAN: ::core::ffi::c_uint = 0x80 as ::core::ffi::c_uint;
pub const TOKS_TMPL_CL100K: ::core::ffi::c_uint = 1 as ::core::ffi::c_uint;
pub const TOKS_TMPL_O200K: ::core::ffi::c_uint = 2 as ::core::ffi::c_uint;
pub const TOKS_TMPL_DSV3: ::core::ffi::c_uint = 3 as ::core::ffi::c_uint;
pub const TOKS_TP_CONTR_MASK: ::core::ffi::c_uint = 0x3 as ::core::ffi::c_uint;
pub const TOKS_TP_CONTR_NONE: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
pub const TOKS_TP_LPREFIX_ANY: ::core::ffi::c_uint = 0x4 as ::core::ffi::c_uint;
pub const TOKS_TP_DIGITS_MASK: ::core::ffi::c_uint = 0x18 as ::core::ffi::c_uint;
pub const TOKS_TP_DIGITS_1: ::core::ffi::c_uint = 0x8 as ::core::ffi::c_uint;
pub const TOKS_TP_DIGITS_SP_RUN: ::core::ffi::c_uint = 0x10 as ::core::ffi::c_uint;
pub const TOKS_TP_PUNCT_NL: ::core::ffi::c_uint = 0x20 as ::core::ffi::c_uint;
pub const TOKS_TP_WS_NL: ::core::ffi::c_uint = 0x40 as ::core::ffi::c_uint;
pub const TOKS_TP_DIGIT_CUT: ::core::ffi::c_uint = 0x200 as ::core::ffi::c_uint;
pub const TOKS_TP_NL_CUT: ::core::ffi::c_uint = 0x400 as ::core::ffi::c_uint;
pub const TOKS_TP_GB_SP: ::core::ffi::c_uint = 0x800 as ::core::ffi::c_uint;
pub const TOKS_TP_HAN: ::core::ffi::c_uint = 0x80 as ::core::ffi::c_uint;
pub const TOKS_TP_NO_SLASH: ::core::ffi::c_uint = 0x100 as ::core::ffi::c_uint;
#[inline]
unsafe extern "C" fn toks_st64(mut p: *mut ::core::ffi::c_void, mut v: uint64_t) {
    memcpy(p, &raw mut v as *const ::core::ffi::c_void, 8 as size_t);
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
unsafe extern "C" fn toks_cls_cp(
    mut t: *const toks_tables,
    mut cp: uint32_t,
) -> uint8_t {
    let mut b1: uint32_t = *(*t)
        .cls_stage1
        .offset((cp >> 8 as ::core::ffi::c_int) as isize) as uint32_t;
    return *(*t)
        .cls_stage2
        .offset(
            b1.wrapping_mul(256 as uint32_t).wrapping_add(cp & 0xff as uint32_t) as isize,
        );
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
unsafe extern "C" fn toks_k3_atom(
    mut t: *const toks_tables,
    mut p: *const uint8_t,
    mut avail: uint64_t,
    mut cp: *mut uint32_t,
    mut k: *mut uint32_t,
    mut strip: uint8_t,
) -> uint8_t {
    let mut n: uint32_t = toks_utf8_len(p, avail);
    let mut c: uint8_t = 0;
    *cp = *p.offset(0 as ::core::ffi::c_int as isize) as uint32_t;
    *k = if n != 0 as uint32_t { n } else { 1 as uint32_t };
    if n <= 1 as uint32_t {
        return (if n == 0 as uint32_t {
            TOKS_C_P as uint8_t as ::core::ffi::c_int
        } else {
            *(*t).cls_ascii.offset(*p.offset(0 as ::core::ffi::c_int as isize) as isize)
                as ::core::ffi::c_int
        }) as uint8_t;
    }
    *cp = toks_cp_decode(p, n);
    c = toks_cls_cp(t, *cp);
    return (if c as ::core::ffi::c_uint & TOKS_C_HAN != 0 as ::core::ffi::c_uint {
        (c as ::core::ffi::c_int & !(strip as ::core::ffi::c_int)) as uint8_t
            as ::core::ffi::c_int
    } else {
        c as ::core::ffi::c_int
    }) as uint8_t;
}
pub const TOKS_CUT_NONE: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
pub const TOKS_CUT_CL100K: ::core::ffi::c_uint = 1;
pub const TOKS_CUT_O200K: ::core::ffi::c_uint = 2;
pub const TOKS_CUT_DSV3: ::core::ffi::c_uint = 3;
pub const TOKS_CUT_SPM: ::core::ffi::c_uint = 4;
pub const TOKS_CUT_WP: ::core::ffi::c_uint = 5;
pub const TOKS_CUT_UNI: ::core::ffi::c_uint = 6;
pub const TOKS_WPA_WORD: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
pub const TOKS_WPA_FOLD: ::core::ffi::c_uint = 1 as ::core::ffi::c_uint;
pub const TOKS_WPA_SPLIT: ::core::ffi::c_uint = 2 as ::core::ffi::c_uint;
pub const TOKS_WPA_PUNCT: ::core::ffi::c_uint = 3 as ::core::ffi::c_uint;
pub const TOKS_SPLIT_D: ::core::ffi::c_uint = 4096 as ::core::ffi::c_uint;
unsafe extern "C" fn atom_holding(
    mut x: *const uint8_t,
    mut len: uint64_t,
    mut i: uint64_t,
    mut k: *mut uint32_t,
) -> uint64_t {
    let mut s: uint64_t = i;
    let mut back: uint32_t = 0 as uint32_t;
    while back < 3 as uint32_t && s > 0 as uint64_t
        && *x.offset(s as isize) as ::core::ffi::c_uint & 0xc0 as ::core::ffi::c_uint
            == 0x80 as ::core::ffi::c_uint
    {
        s = s.wrapping_sub(1);
        back = back.wrapping_add(1);
    }
    let mut n: uint32_t = toks_utf8_len(x.offset(s as isize), len.wrapping_sub(s));
    if n != 0 as uint32_t && s.wrapping_add(n as uint64_t) > i {
        *k = n;
        return s;
    }
    *k = 1 as ::core::ffi::c_uint as uint32_t;
    return i;
}
unsafe extern "C" fn atom_base(
    mut t: *const toks_tables,
    mut p: *const uint8_t,
    mut k: uint32_t,
    mut cp: *mut uint32_t,
) -> uint8_t {
    if k == 1 as uint32_t {
        *cp = *p.offset(0 as ::core::ffi::c_int as isize) as uint32_t;
        return (if (*p.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
            < 0x80 as ::core::ffi::c_uint
        {
            (*(*t).cls_ascii.offset(*p.offset(0 as ::core::ffi::c_int as isize) as isize)
                as ::core::ffi::c_uint & TOKS_C_BASE_MASK) as uint8_t
                as ::core::ffi::c_int
        } else {
            TOKS_C_P as uint8_t as ::core::ffi::c_int
        }) as uint8_t;
    }
    *cp = toks_cp_decode(p, k);
    return (toks_cls_cp(t, *cp) as ::core::ffi::c_uint & TOKS_C_BASE_MASK) as uint8_t;
}
unsafe extern "C" fn scan_ok(
    mut t: *const toks_tables,
    mut x: *const uint8_t,
    mut len: uint64_t,
    mut c: uint64_t,
    mut lo: *mut uint64_t,
    mut hi: *mut uint64_t,
) -> ::core::ffi::c_int {
    let mut ka: uint32_t = 0;
    let mut kb: uint32_t = 0;
    let mut kz: uint32_t = 0;
    let mut cpa: uint32_t = 0;
    let mut cpb: uint32_t = 0;
    let mut cpz: uint32_t = 0;
    let mut sa: uint64_t = atom_holding(
        x,
        len,
        c.wrapping_sub(1 as uint64_t),
        &raw mut ka,
    );
    if sa.wrapping_add(ka as uint64_t) != c {
        return 0 as ::core::ffi::c_int;
    }
    kb = toks_utf8_len(x.offset(c as isize), len.wrapping_sub(c));
    if kb == 0 as uint32_t {
        kb = 1 as ::core::ffi::c_uint as uint32_t;
    }
    let mut A: uint8_t = atom_base(t, x.offset(sa as isize), ka, &raw mut cpa);
    let mut B: uint8_t = atom_base(t, x.offset(c as isize), kb, &raw mut cpb);
    *lo = sa;
    *hi = c.wrapping_add(kb as uint64_t);
    let mut pr: uint32_t = (*t).tmpl_params;
    let mut lpany: ::core::ffi::c_int = (pr & TOKS_TP_LPREFIX_ANY as uint32_t
        != 0 as uint32_t) as ::core::ffi::c_int;
    let mut digits: uint32_t = pr & TOKS_TP_DIGITS_MASK as uint32_t;
    let mut ws_nl: ::core::ffi::c_int = (pr & TOKS_TP_WS_NL as uint32_t != 0 as uint32_t)
        as ::core::ffi::c_int;
    let mut sp: ::core::ffi::c_int = (cpa == 0x20 as uint32_t) as ::core::ffi::c_int;
    if A as ::core::ffi::c_uint == TOKS_C_L {
        return (B as ::core::ffi::c_uint != TOKS_C_L) as ::core::ffi::c_int;
    }
    if A as ::core::ffi::c_uint == TOKS_C_N {
        return (B as ::core::ffi::c_uint != TOKS_C_N
            || digits == TOKS_TP_DIGITS_1 as uint32_t) as ::core::ffi::c_int;
    }
    if A as ::core::ffi::c_uint == TOKS_C_P {
        if B as ::core::ffi::c_uint == TOKS_C_P {
            return 0 as ::core::ffi::c_int;
        }
        if B as ::core::ffi::c_uint == TOKS_C_NL {
            return (pr & TOKS_TP_PUNCT_NL as uint32_t == 0 as uint32_t)
                as ::core::ffi::c_int;
        }
        if B as ::core::ffi::c_uint != TOKS_C_L {
            return 1 as ::core::ffi::c_int;
        }
        let mut apos: ::core::ffi::c_int = (pr & TOKS_TP_CONTR_MASK as uint32_t
            != TOKS_TP_CONTR_NONE as uint32_t && cpa == 0x27 as uint32_t)
            as ::core::ffi::c_int;
        if lpany == 0 && apos == 0 {
            return 1 as ::core::ffi::c_int;
        }
        if sa == 0 as uint64_t {
            return 0 as ::core::ffi::c_int;
        }
        let mut sz: uint64_t = atom_holding(
            x,
            len,
            sa.wrapping_sub(1 as uint64_t),
            &raw mut kz,
        );
        *lo = sz;
        return (atom_base(t, x.offset(sz as isize), kz, &raw mut cpz)
            as ::core::ffi::c_uint == TOKS_C_P) as ::core::ffi::c_int;
    }
    if B as ::core::ffi::c_uint == TOKS_C_WS || B as ::core::ffi::c_uint == TOKS_C_NL {
        return 0 as ::core::ffi::c_int;
    }
    if A as ::core::ffi::c_uint == TOKS_C_NL && ws_nl != 0 {
        return 1 as ::core::ffi::c_int;
    }
    if B as ::core::ffi::c_uint == TOKS_C_L && (lpany != 0 || sp != 0) {
        return 0 as ::core::ffi::c_int;
    }
    if sp != 0
        && (B as ::core::ffi::c_uint == TOKS_C_P
            || B as ::core::ffi::c_uint == TOKS_C_N
                && digits == TOKS_TP_DIGITS_SP_RUN as uint32_t)
    {
        return 0 as ::core::ffi::c_int;
    }
    if sa == 0 as uint64_t {
        return 1 as ::core::ffi::c_int;
    }
    let mut sz_0: uint64_t = atom_holding(
        x,
        len,
        sa.wrapping_sub(1 as uint64_t),
        &raw mut kz,
    );
    *lo = sz_0;
    let mut Z: uint8_t = atom_base(t, x.offset(sz_0 as isize), kz, &raw mut cpz);
    return (Z as ::core::ffi::c_uint != TOKS_C_WS
        && Z as ::core::ffi::c_uint != TOKS_C_NL) as ::core::ffi::c_int;
}
unsafe extern "C" fn lone_ws(
    mut t: *const toks_tables,
    mut x: *const uint8_t,
    mut len: uint64_t,
    mut sa: uint64_t,
    mut lo: *mut uint64_t,
    mut no_x: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if sa == 0 as uint64_t {
        return 1 as ::core::ffi::c_int;
    }
    let mut kz: uint32_t = 0;
    let mut cpz: uint32_t = 0;
    let mut sz: uint64_t = atom_holding(
        x,
        len,
        sa.wrapping_sub(1 as uint64_t),
        &raw mut kz,
    );
    *lo = sz;
    let mut Z: uint8_t = (toks_k3_atom(
        t,
        x.offset(sz as isize),
        len.wrapping_sub(sz),
        &raw mut cpz,
        &raw mut kz,
        0 as uint8_t,
    ) as ::core::ffi::c_uint & TOKS_C_BASE_MASK) as uint8_t;
    return (Z as ::core::ffi::c_uint != TOKS_C_WS
        && Z as ::core::ffi::c_uint != TOKS_C_NL
        && !(no_x != 0 && Z as ::core::ffi::c_uint == TOKS_C_X)) as ::core::ffi::c_int;
}
unsafe extern "C" fn scan_o200k(
    mut t: *const toks_tables,
    mut x: *const uint8_t,
    mut len: uint64_t,
    mut c: uint64_t,
    mut lo: *mut uint64_t,
    mut hi: *mut uint64_t,
) -> ::core::ffi::c_int {
    let mut ka: uint32_t = 0;
    let mut kb: uint32_t = 0;
    let mut cpa: uint32_t = 0;
    let mut cpb: uint32_t = 0;
    let mut sa: uint64_t = atom_holding(
        x,
        len,
        c.wrapping_sub(1 as uint64_t),
        &raw mut ka,
    );
    if sa.wrapping_add(ka as uint64_t) != c {
        return 0 as ::core::ffi::c_int;
    }
    let mut ca: uint8_t = toks_k3_atom(
        t,
        x.offset(sa as isize),
        len.wrapping_sub(sa),
        &raw mut cpa,
        &raw mut ka,
        0 as uint8_t,
    );
    let mut cb: uint8_t = toks_k3_atom(
        t,
        x.offset(c as isize),
        len.wrapping_sub(c),
        &raw mut cpb,
        &raw mut kb,
        0 as uint8_t,
    );
    let mut A: uint8_t = (ca as ::core::ffi::c_uint & TOKS_C_BASE_MASK) as uint8_t;
    let mut B: uint8_t = (cb as ::core::ffi::c_uint & TOKS_C_BASE_MASK) as uint8_t;
    let mut cs: uint8_t = (TOKS_C_UPPER | TOKS_C_LOWER) as uint8_t;
    *lo = sa;
    *hi = c.wrapping_add(kb as uint64_t);
    if A as ::core::ffi::c_uint == TOKS_C_L {
        return ((cb as ::core::ffi::c_int & cs as ::core::ffi::c_int)
            as ::core::ffi::c_uint == 0 as ::core::ffi::c_uint
            && cpb != 0x27 as uint32_t) as ::core::ffi::c_int;
    }
    if A as ::core::ffi::c_uint == TOKS_C_N {
        return (B as ::core::ffi::c_uint != TOKS_C_N
            || (*t).tmpl_params & TOKS_TP_DIGITS_MASK as uint32_t
                == TOKS_TP_DIGITS_1 as uint32_t) as ::core::ffi::c_int;
    }
    if A as ::core::ffi::c_uint == TOKS_C_P {
        return ((ca as ::core::ffi::c_int & cs as ::core::ffi::c_int)
            as ::core::ffi::c_uint == 0 as ::core::ffi::c_uint
            && (B as ::core::ffi::c_uint == TOKS_C_N
                || B as ::core::ffi::c_uint == TOKS_C_WS)) as ::core::ffi::c_int;
    }
    let mut tail: uint32_t = if (*t).tmpl_params & TOKS_TP_NO_SLASH as uint32_t
        != 0 as uint32_t
    {
        0xd as uint32_t
    } else {
        0x2f as uint32_t
    };
    if A as ::core::ffi::c_uint == TOKS_C_NL {
        return (B as ::core::ffi::c_uint != TOKS_C_WS
            && B as ::core::ffi::c_uint != TOKS_C_NL && cpb != tail)
            as ::core::ffi::c_int;
    }
    if A as ::core::ffi::c_uint != TOKS_C_WS || B as ::core::ffi::c_uint == TOKS_C_WS
        || B as ::core::ffi::c_uint == TOKS_C_NL
        || (cb as ::core::ffi::c_int & cs as ::core::ffi::c_int) as ::core::ffi::c_uint
            != 0 as ::core::ffi::c_uint
    {
        return 0 as ::core::ffi::c_int;
    }
    if cpa == 0x20 as uint32_t && B as ::core::ffi::c_uint == TOKS_C_P {
        return 0 as ::core::ffi::c_int;
    }
    return lone_ws(t, x, len, sa, lo, 0 as ::core::ffi::c_int);
}
unsafe extern "C" fn scan_dsv3(
    mut t: *const toks_tables,
    mut x: *const uint8_t,
    mut len: uint64_t,
    mut c: uint64_t,
    mut lo: *mut uint64_t,
    mut hi: *mut uint64_t,
) -> ::core::ffi::c_int {
    let mut ka: uint32_t = 0;
    let mut kb: uint32_t = 0;
    let mut cpa: uint32_t = 0;
    let mut cpb: uint32_t = 0;
    let mut sa: uint64_t = atom_holding(
        x,
        len,
        c.wrapping_sub(1 as uint64_t),
        &raw mut ka,
    );
    if sa.wrapping_add(ka as uint64_t) != c {
        return 0 as ::core::ffi::c_int;
    }
    let mut A: uint8_t = (toks_k3_atom(
        t,
        x.offset(sa as isize),
        len.wrapping_sub(sa),
        &raw mut cpa,
        &raw mut ka,
        0 as uint8_t,
    ) as ::core::ffi::c_uint & TOKS_C_BASE_MASK) as uint8_t;
    let mut B: uint8_t = (toks_k3_atom(
        t,
        x.offset(c as isize),
        len.wrapping_sub(c),
        &raw mut cpb,
        &raw mut kb,
        0 as uint8_t,
    ) as ::core::ffi::c_uint & TOKS_C_BASE_MASK) as uint8_t;
    *lo = sa;
    *hi = c.wrapping_add(kb as uint64_t);
    if A as ::core::ffi::c_uint == TOKS_C_X || B as ::core::ffi::c_uint == TOKS_C_X {
        return 0 as ::core::ffi::c_int;
    }
    if A as ::core::ffi::c_uint == TOKS_C_L {
        return (B as ::core::ffi::c_uint != TOKS_C_L) as ::core::ffi::c_int;
    }
    if A as ::core::ffi::c_uint == TOKS_C_N {
        return (B as ::core::ffi::c_uint != TOKS_C_N) as ::core::ffi::c_int;
    }
    if A as ::core::ffi::c_uint == TOKS_C_P {
        return (B as ::core::ffi::c_uint == TOKS_C_N
            || B as ::core::ffi::c_uint == TOKS_C_WS) as ::core::ffi::c_int;
    }
    if A as ::core::ffi::c_uint == TOKS_C_NL {
        return (B as ::core::ffi::c_uint != TOKS_C_WS
            && B as ::core::ffi::c_uint != TOKS_C_NL) as ::core::ffi::c_int;
    }
    if B as ::core::ffi::c_uint == TOKS_C_WS || B as ::core::ffi::c_uint == TOKS_C_NL
        || B as ::core::ffi::c_uint == TOKS_C_L
        || cpa == 0x20 as uint32_t && B as ::core::ffi::c_uint == TOKS_C_P
    {
        return 0 as ::core::ffi::c_int;
    }
    return lone_ws(t, x, len, sa, lo, 1 as ::core::ffi::c_int);
}
unsafe extern "C" fn cut_wp(
    mut w: *const toks_wp_tables,
    mut x: *const uint8_t,
    mut c: uint64_t,
    mut lo: *mut uint64_t,
    mut hi: *mut uint64_t,
) -> ::core::ffi::c_int {
    let mut p: uint8_t = *x.offset(c.wrapping_sub(1 as uint64_t) as isize);
    let mut q: uint8_t = *x.offset(c as isize);
    if p as ::core::ffi::c_uint >= 0x80 as ::core::ffi::c_uint
        || q as ::core::ffi::c_uint >= 0x80 as ::core::ffi::c_uint
        || (*w).ascii_cls[p as usize] as ::core::ffi::c_uint != TOKS_WPA_SPLIT
    {
        return 0 as ::core::ffi::c_int;
    }
    *lo = c.wrapping_sub(1 as uint64_t);
    *hi = c.wrapping_add(1 as uint64_t);
    return ((*w).ascii_cls[q as usize] as ::core::ffi::c_uint == TOKS_WPA_WORD
        || (*w).ascii_cls[q as usize] as ::core::ffi::c_uint == TOKS_WPA_FOLD
        || (*w).ascii_cls[q as usize] as ::core::ffi::c_uint == TOKS_WPA_PUNCT)
        as ::core::ffi::c_int;
}
unsafe extern "C" fn cut_uni(
    mut u: *const toks_uni,
    mut x: *const uint8_t,
    mut len: uint64_t,
    mut c: uint64_t,
    mut lo: *mut uint64_t,
    mut hi: *mut uint64_t,
) -> ::core::ffi::c_int {
    if c.wrapping_add(1 as uint64_t) >= len
        || *x.offset(c as isize) as ::core::ffi::c_uint != 0x20 as ::core::ffi::c_uint
        || *x.offset(c.wrapping_sub(1 as uint64_t) as isize) as ::core::ffi::c_uint
            >= 0x80 as ::core::ffi::c_uint
        || *x.offset(c.wrapping_add(1 as uint64_t) as isize) as ::core::ffi::c_uint
            >= 0x80 as ::core::ffi::c_uint
    {
        return 0 as ::core::ffi::c_int;
    }
    *lo = c.wrapping_sub(1 as uint64_t);
    *hi = c.wrapping_add(2 as uint64_t);
    return ((*u).simple[*x.offset(c.wrapping_sub(1 as uint64_t) as isize) as usize]
        as ::core::ffi::c_uint == 1 as ::core::ffi::c_uint
        && (*u).simple[*x.offset(c.wrapping_add(1 as uint64_t) as isize) as usize]
            as ::core::ffi::c_uint == 1 as ::core::ffi::c_uint) as ::core::ffi::c_int;
}
unsafe extern "C" fn tokens_clear(
    mut t: *const toks_tables,
    mut x: *const uint8_t,
    mut len: uint64_t,
    mut lo: uint64_t,
    mut hi: uint64_t,
    mut maxlen: uint64_t,
) -> ::core::ffi::c_int {
    let mut s: uint64_t = if lo.wrapping_add(1 as uint64_t) > maxlen {
        lo.wrapping_add(1 as uint64_t).wrapping_sub(maxlen)
    } else {
        0 as uint64_t
    };
    while s < hi {
        let mut b0: uint8_t = *x.offset(s as isize);
        let mut ph: uint64_t = 0 as uint64_t;
        while ph < 2 as uint64_t {
            if !((*t).add_phases as ::core::ffi::c_ulonglong
                & (1 as ::core::ffi::c_ulonglong) << ph == 0 as ::core::ffi::c_ulonglong)
            {
                let mut sh: *const uint8_t = (*t)
                    .add_shufti
                    .offset(ph.wrapping_mul(32 as uint64_t) as isize);
                if !((*sh
                    .offset(
                        (b0 as ::core::ffi::c_uint & 0xf as ::core::ffi::c_uint) as isize,
                    ) as ::core::ffi::c_int
                    & *sh
                        .offset(
                            (16 as ::core::ffi::c_uint)
                                .wrapping_add(
                                    (b0 as ::core::ffi::c_int >> 4 as ::core::ffi::c_int)
                                        as ::core::ffi::c_uint,
                                ) as isize,
                        ) as ::core::ffi::c_int) as ::core::ffi::c_uint
                    == 0 as ::core::ffi::c_uint)
                {
                    if s >= lo
                        && *(*t)
                            .add_single
                            .offset(
                                ph
                                    .wrapping_mul(256 as uint64_t)
                                    .wrapping_add(b0 as uint64_t) as isize,
                            ) != 0 as uint32_t
                    {
                        return 0 as ::core::ffi::c_int;
                    }
                    if !(s.wrapping_add(1 as uint64_t) >= len) {
                        let mut ent: uint64_t = *(*t)
                            .add_index
                            .offset(
                                ph
                                    .wrapping_mul(65536 as uint64_t)
                                    .wrapping_add(
                                        b0 as uint64_t
                                            | (*x.offset(s.wrapping_add(1 as uint64_t) as isize)
                                                as uint64_t) << 8 as ::core::ffi::c_int,
                                    ) as isize,
                            );
                        let mut cand: *const uint32_t = (*t)
                            .add_cand
                            .offset(
                                (ent as ::core::ffi::c_ulonglong
                                    & 0xffffffff as ::core::ffi::c_ulonglong) as isize,
                            );
                        let mut j: uint64_t = 0 as uint64_t;
                        while j < ent >> 32 as ::core::ffi::c_int {
                            let mut e: *const toks_added_entry = (*t)
                                .add_entries
                                .offset(*cand.offset(j as isize) as isize)
                                as *const toks_added_entry;
                            if s.wrapping_add((*e).len as uint64_t) <= lo {
                                break;
                            }
                            if !(s.wrapping_add((*e).len as uint64_t) > len) {
                                if memcmp(
                                    (*t).add_bytes.offset((*e).off as isize)
                                        as *const ::core::ffi::c_void,
                                    x.offset(s as isize) as *const ::core::ffi::c_void,
                                    (*e).len as size_t,
                                ) == 0 as ::core::ffi::c_int
                                {
                                    return 0 as ::core::ffi::c_int;
                                }
                            }
                            j = j.wrapping_add(1);
                        }
                    }
                }
            }
            ph = ph.wrapping_add(1);
        }
        s = s.wrapping_add(1);
    }
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn nfc_clear(
    mut f: uint32_t,
    mut x: *const uint8_t,
    mut len: uint64_t,
    mut lo: uint64_t,
    mut hi: uint64_t,
) -> ::core::ffi::c_int {
    let mut i: uint64_t = lo;
    while i <= hi && i < len {
        if *x.offset(i as isize) as ::core::ffi::c_uint >= 0x80 as ::core::ffi::c_uint
            && toks_nfc_boundary(f, x, len, i) == 0
        {
            return 0 as ::core::ffi::c_int;
        }
        let mut k: uint32_t = toks_utf8_len(x.offset(i as isize), len.wrapping_sub(i));
        i = i
            .wrapping_add(
                (if k != 0 as uint32_t { k } else { 1 as uint32_t }) as uint64_t,
            );
    }
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn uni_rules(mut u: *const toks_uni) -> ::core::ffi::c_int {
    let mut g: *const toks_uni_cfg = &raw const (*u).cfg;
    if (*g).meta_prefix as ::core::ffi::c_int != 0
        || (*g).meta_replace as ::core::ffi::c_int != 0
        || !((*g).ws_split as ::core::ffi::c_int != 0
            || (*g).metaspace as ::core::ffi::c_int != 0
                && (*g).meta_split as ::core::ffi::c_int != 0)
    {
        return 0 as ::core::ffi::c_int;
    }
    if (*g).has_charsmap as ::core::ffi::c_int != 0
        && (*u).aent[0x20 as ::core::ffi::c_int as usize] != 0 as uint32_t
    {
        return 0 as ::core::ffi::c_int;
    }
    let mut k: uint32_t = 0 as uint32_t;
    while k < (*g).rep_n as uint32_t {
        let mut i: uint32_t = 0 as uint32_t;
        while i < 4 as uint32_t {
            if i < (*g).rep_pl[k as usize] as uint32_t
                && (*g).rep_p[k as usize][i as usize] as ::core::ffi::c_uint
                    == 0x20 as ::core::ffi::c_uint
                || i < (*g).rep_cl[k as usize] as uint32_t
                    && (*g).rep_c[k as usize][i as usize] as ::core::ffi::c_uint
                        == 0x20 as ::core::ffi::c_uint
            {
                return 0 as ::core::ffi::c_int;
            }
            i = i.wrapping_add(1);
        }
        k = k.wrapping_add(1);
    }
    return 1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn toks_cuts_of(
    mut ctx: *const toks_ctx,
    mut flags: uint32_t,
    mut k: *mut toks_cuts,
) -> ::core::ffi::c_int {
    memset(
        k as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<toks_cuts>() as size_t,
    );
    if (*ctx).o.trunc_on != 0 && flags & TOKS_NO_TRUNCATE as uint32_t == 0 as uint32_t
        || (*ctx).o.pad_on != 0 && flags & TOKS_NO_PAD as uint32_t == 0 as uint32_t
        || (*ctx).cut_run != 0 as uint32_t || !(*ctx).gen.is_null()
    {
        return 0 as ::core::ffi::c_int;
    }
    let mut t: *const toks_tables = &raw const (*ctx).t;
    let mut mode: uint32_t = flags & TOKS_ADDED_MASK as uint32_t;
    let mut fam: uint32_t = TOKS_CUT_NONE as uint32_t;
    let mut tokens: ::core::ffi::c_int = (mode != TOKS_ADDED_NONE as uint32_t
        && (*t).add_n != 0 as uint64_t
        && !(mode == TOKS_ADDED_NONSPECIAL as uint32_t
            && (*ctx).n_nonspecial == 0 as uint32_t)) as ::core::ffi::c_int;
    let mut phase1: ::core::ffi::c_int = (tokens != 0
        && (*t).add_phases & 2 as uint64_t != 0 as uint64_t) as ::core::ffi::c_int;
    let mut sp: *const toks_spm = (*ctx).spm as *const toks_spm;
    if !(*ctx).wp.is_null() {
        fam = (if phase1 != 0 { TOKS_CUT_NONE } else { TOKS_CUT_WP }) as uint32_t;
    } else if !(*ctx).uni.is_null() {
        fam = (if phase1 != 0 || uni_rules((*ctx).uni as *const toks_uni) == 0 {
            TOKS_CUT_NONE
        } else {
            TOKS_CUT_UNI
        }) as uint32_t;
    } else if !sp.is_null() {
        fam = (if (*sp).pfx_mode == TOKS_SPM_PFX_GAP as ::core::ffi::c_int as uint32_t
            || (*sp).pairs.is_null() && (*sp).ms_split == 0
        {
            TOKS_CUT_NONE
        } else {
            TOKS_CUT_SPM
        }) as uint32_t;
    } else if (*t).tmpl == TOKS_TMPL_CL100K as uint32_t {
        fam = (if (*t).tmpl_params
            & (TOKS_TP_DIGIT_CUT as uint32_t | TOKS_TP_NL_CUT as uint32_t
                | TOKS_TP_GB_SP as uint32_t) != 0 as uint32_t
        {
            TOKS_CUT_NONE
        } else {
            TOKS_CUT_CL100K
        }) as uint32_t;
    } else if (*t).tmpl == TOKS_TMPL_O200K as uint32_t {
        fam = (if (*t).tmpl_params & TOKS_TP_HAN as uint32_t != 0 as uint32_t {
            TOKS_CUT_NONE
        } else {
            TOKS_CUT_O200K
        }) as uint32_t;
    } else if (*t).tmpl == TOKS_TMPL_DSV3 as uint32_t {
        fam = TOKS_CUT_DSV3 as uint32_t;
    }
    if fam == TOKS_CUT_NONE as uint32_t || phase1 != 0 && (*ctx).nfc != 0 as uint32_t {
        return 0 as ::core::ffi::c_int;
    }
    if tokens != 0 {
        let mut i: uint64_t = 0 as uint64_t;
        while i < (*t).add_n {
            if (*(*t).add_entries.offset(i as isize)).len as uint64_t > (*k).maxlen {
                (*k).maxlen = (*(*t).add_entries.offset(i as isize)).len as uint64_t;
            }
            i = i.wrapping_add(1);
        }
    }
    (*k).family = fam;
    (*k).win = (*k).maxlen.wrapping_add(16 as uint64_t);
    return 1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn toks_cut_ok(
    mut ctx: *const toks_ctx,
    mut k: *const toks_cuts,
    mut x: *const uint8_t,
    mut len: uint64_t,
    mut c: uint64_t,
) -> ::core::ffi::c_int {
    let mut lo: uint64_t = c;
    let mut hi: uint64_t = c;
    let mut ok: ::core::ffi::c_int = 0;
    match (*k).family {
        TOKS_CUT_SPM => {
            let mut ka: uint32_t = 0;
            lo = atom_holding(x, len, c.wrapping_sub(1 as uint64_t), &raw mut ka);
            ok = (lo.wrapping_add(ka as uint64_t) == c
                && toks_spm_cut((*ctx).spm as *const toks_spm, x, len, lo, c) != 0)
                as ::core::ffi::c_int;
            let mut kb: uint32_t = toks_utf8_len(
                x.offset(c as isize),
                len.wrapping_sub(c),
            );
            hi = c
                .wrapping_add(
                    (if kb != 0 as uint32_t { kb } else { 1 as uint32_t }) as uint64_t,
                );
        }
        TOKS_CUT_WP => {
            ok = cut_wp(
                (*ctx).wp as *const toks_wp_tables,
                x,
                c,
                &raw mut lo,
                &raw mut hi,
            );
        }
        TOKS_CUT_UNI => {
            ok = cut_uni(
                (*ctx).uni as *const toks_uni,
                x,
                len,
                c,
                &raw mut lo,
                &raw mut hi,
            );
        }
        TOKS_CUT_O200K => {
            ok = scan_o200k(&raw const (*ctx).t, x, len, c, &raw mut lo, &raw mut hi);
        }
        TOKS_CUT_DSV3 => {
            ok = scan_dsv3(&raw const (*ctx).t, x, len, c, &raw mut lo, &raw mut hi);
        }
        TOKS_CUT_CL100K => {
            ok = scan_ok(&raw const (*ctx).t, x, len, c, &raw mut lo, &raw mut hi);
        }
        _ => return 0 as ::core::ffi::c_int,
    }
    if ok == 0
        || (*ctx).nfc != 0 as uint32_t && nfc_clear((*ctx).nfc, x, len, lo, hi) == 0
    {
        return 0 as ::core::ffi::c_int;
    }
    return ((*k).maxlen == 0 as uint64_t
        || tokens_clear(&raw const (*ctx).t, x, len, lo, hi, (*k).maxlen) != 0)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn toks_split_points(
    mut ctx: *const toks_ctx,
    mut text: *const ::core::ffi::c_void,
    mut len: uint64_t,
    mut flags: uint32_t,
    mut n_want: uint32_t,
    mut offs: *mut uint64_t,
    mut cap: uint64_t,
    mut scr: *mut ::core::ffi::c_void,
) -> int64_t {
    if ctx.is_null() {
        return TOKS_E_ARG as int64_t;
    }
    if flags
        & !(TOKS_ADDED_MASK as uint32_t | TOKS_NO_POSTPROCESS as uint32_t
            | TOKS_CONTINUATION as uint32_t | TOKS_NO_TRUNCATE as uint32_t
            | TOKS_NO_PAD as uint32_t) != 0 as uint32_t
        || flags & TOKS_ADDED_MASK as uint32_t == TOKS_ADDED_MASK as uint32_t
    {
        return TOKS_E_ARG as int64_t;
    }
    if text.is_null() && len != 0 as uint64_t || offs.is_null() && cap != 0 as uint64_t {
        return TOKS_E_ARG as int64_t;
    }
    if len as ::core::ffi::c_ulonglong > TOKS_MAX_TEXT {
        return TOKS_E_LIMIT as int64_t;
    }
    let mut k: toks_cuts = toks_cuts {
        family: 0,
        rsv: 0,
        maxlen: 0,
        win: 0,
    };
    if n_want <= 1 as uint32_t || len < 2 as uint64_t || cap == 0 as uint64_t
        || toks_cuts_of(ctx, flags, &raw mut k) == 0
    {
        return 0 as int64_t;
    }
    let mut x: *const uint8_t = text as *const uint8_t;
    let mut nw: uint64_t = if n_want as uint64_t > len {
        len
    } else {
        n_want as uint64_t
    };
    let mut d_max: uint64_t = len.wrapping_div((4 as uint64_t).wrapping_mul(nw));
    if d_max > TOKS_SPLIT_D as uint64_t {
        d_max = TOKS_SPLIT_D as uint64_t;
    }
    let mut c: uint64_t = 0 as uint64_t;
    let mut prev: uint64_t = 0 as uint64_t;
    let mut i: uint64_t = 1 as uint64_t;
    while i < nw && c < cap {
        let mut tg: uint64_t = len.wrapping_mul(i).wrapping_div(nw);
        let mut best: uint64_t = 0 as uint64_t;
        let mut d: uint64_t = 0 as uint64_t;
        while d <= d_max && best == 0 as uint64_t {
            let mut side: uint32_t = 0 as uint32_t;
            while side < 2 as uint32_t {
                if side == 1 as uint32_t && d == 0 as uint64_t {
                    break;
                }
                if !(if side == 0 as uint32_t {
                    (d >= tg) as ::core::ffi::c_int
                } else {
                    (tg.wrapping_add(d) >= len) as ::core::ffi::c_int
                } != 0)
                {
                    let mut p: uint64_t = if side == 0 as uint32_t {
                        tg.wrapping_sub(d)
                    } else {
                        tg.wrapping_add(d)
                    };
                    if toks_cut_ok(ctx, &raw mut k, x, len, p) != 0 {
                        best = p;
                        break;
                    }
                }
                side = side.wrapping_add(1);
            }
            d = d.wrapping_add(1);
        }
        if best > prev {
            toks_st64(
                (offs as *mut ::core::ffi::c_void as *mut uint8_t)
                    .offset((8 as uint64_t).wrapping_mul(c) as isize)
                    as *mut ::core::ffi::c_void,
                best,
            );
            c = c.wrapping_add(1);
            prev = best;
        }
        i = i.wrapping_add(1);
    }
    return c as int64_t;
}
#[no_mangle]
pub unsafe extern "C" fn toks_pp_ids(
    mut ctx: *const toks_ctx,
    mut flags: uint32_t,
    mut pre: *mut *const uint32_t,
    mut n_pre: *mut uint32_t,
    mut suf: *mut *const uint32_t,
    mut n_suf: *mut uint32_t,
) {
    let mut on: ::core::ffi::c_int = (flags & TOKS_NO_POSTPROCESS as uint32_t
        == 0 as uint32_t) as ::core::ffi::c_int;
    *pre = &raw const (*ctx).pp_ids as *const uint32_t;
    *n_pre = if on != 0 { (*ctx).n_pp_prefix } else { 0 as uint32_t };
    *suf = (&raw const (*ctx).pp_ids as *const uint32_t)
        .offset((*ctx).n_pp_prefix as isize);
    *n_suf = if on != 0 { (*ctx).n_pp_suffix } else { 0 as uint32_t };
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
