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
    static TOKS_CRC32C_TAB: [uint32_t; 256];
    fn toks_b64_decode(
        p: *const uint8_t,
        n: uint64_t,
        out: *mut uint8_t,
        cap: uint64_t,
    ) -> int64_t;
    fn toks_is_regex_ws(cp: uint32_t) -> ::core::ffi::c_int;
    fn toks_json_parse(
        data: *const uint8_t,
        len: uint64_t,
        ar: *mut toks_arena,
        out: *mut *mut jv,
    ) -> int64_t;
    fn toks_jv_get(obj: *const jv, key: *const ::core::ffi::c_char) -> *mut jv;
    fn toks_gen_compile(
        sp: *const toks_gen_spec,
        n: uint32_t,
        ar: *mut toks_arena,
        out: *mut *const toks_gen,
        bytes: *mut uint64_t,
        err: *mut toks_err,
    ) -> int64_t;
    fn toks_char_byte(cp: uint32_t) -> int32_t;
    fn toks_norm(
        steps: uint32_t,
        text: *const uint8_t,
        len: uint64_t,
        out: *mut uint8_t,
        cap: uint64_t,
    ) -> int64_t;
    fn toks_nfc_scan(
        f: uint32_t,
        text: *const uint8_t,
        len: uint64_t,
        pos: uint64_t,
        run_end: *mut uint64_t,
    ) -> uint64_t;
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
pub type C2RustUnnamed = ::core::ffi::c_uint;
pub const JV_MEM: C2RustUnnamed = 6;
pub const JV_OBJ: C2RustUnnamed = 5;
pub const JV_ARR: C2RustUnnamed = 4;
pub const JV_STR: C2RustUnnamed = 3;
pub const JV_NUM: C2RustUnnamed = 2;
pub const JV_BOOL: C2RustUnnamed = 1;
pub const JV_NULL: C2RustUnnamed = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct jv {
    pub type_0: uint8_t,
    pub num_float: uint8_t,
    pub rsv: uint16_t,
    pub s_len: uint32_t,
    pub s: *const uint8_t,
    pub child: *mut jv,
    pub next: *mut jv,
    pub num: int64_t,
}
pub type C2RustUnnamed_0 = ::core::ffi::c_uint;
pub const TOKS_PPS_TOK: C2RustUnnamed_0 = 1;
pub const TOKS_PPS_SEQ: C2RustUnnamed_0 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct toks_sidx {
    pub s: *const *const uint8_t,
    pub len: *const uint32_t,
    pub slot: *mut uint32_t,
    pub mask: uint64_t,
}
pub const PP_CLS_SEP: C2RustUnnamed_7 = 1;
pub const PP_NONE: C2RustUnnamed_7 = 0;
pub const PP_SEQUENCE: C2RustUnnamed_7 = 4;
pub const PP_TEMPLATE: C2RustUnnamed_7 = 3;
pub const PP_BYTELEVEL: C2RustUnnamed_7 = 2;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct steps {
    pub o: [*const jv; 16],
    pub k: [uint8_t; 16],
    pub n: uint32_t,
    pub present: uint32_t,
}
pub const D_BYTELEVEL: C2RustUnnamed_8 = 24;
pub const C_DEC: C2RustUnnamed_9 = 2;
pub const STEP_N: C2RustUnnamed_8 = 33;
pub const P_BYTELEVEL: C2RustUnnamed_8 = 13;
pub const N_NFC: C2RustUnnamed_8 = 0;
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
pub const TOKS_GS_BYTELEVEL: C2RustUnnamed_1 = 3;
pub const TOKS_GB_REMOVED: C2RustUnnamed_2 = 1;
pub const TOKS_GB_ISOLATED: C2RustUnnamed_2 = 0;
pub const TOKS_GS_DIGITS: C2RustUnnamed_1 = 1;
pub const TOKS_GB_NEXT: C2RustUnnamed_2 = 3;
pub const P_PUNCT: C2RustUnnamed_8 = 20;
pub const P_DIGITS: C2RustUnnamed_8 = 19;
pub const P_SPLIT: C2RustUnnamed_8 = 14;
pub const C_PRE: C2RustUnnamed_9 = 1;
pub const TOKS_GS_PUNCT: C2RustUnnamed_1 = 2;
pub const TOKS_GB_CONTIGUOUS: C2RustUnnamed_2 = 4;
pub const TOKS_GS_SPLIT: C2RustUnnamed_1 = 0;
pub const N_NFKC: C2RustUnnamed_8 = 2;
pub const C_NORM: C2RustUnnamed_9 = 0;
pub const TOKS_SPM_N_PREPEND: C2RustUnnamed_3 = 1;
pub const N_REPLACE: C2RustUnnamed_8 = 7;
pub const N_PREPEND: C2RustUnnamed_8 = 8;
pub const TOKS_SPM_D_STRIP: C2RustUnnamed_4 = 4;
pub const TOKS_SPM_D_FUSE: C2RustUnnamed_4 = 3;
pub const TOKS_SPM_D_BYTE_FALLBACK: C2RustUnnamed_4 = 2;
pub const TOKS_SPM_D_METASPACE: C2RustUnnamed_4 = 5;
pub const TOKS_SPM_PS_ALWAYS: C2RustUnnamed_5 = 0;
pub const TOKS_SPM_PS_NEVER: C2RustUnnamed_5 = 2;
pub const TOKS_SPM_PS_FIRST: C2RustUnnamed_5 = 1;
pub const D_METASPACE: C2RustUnnamed_8 = 25;
pub const D_STRIP: C2RustUnnamed_8 = 29;
pub const TOKS_SPM_D_REPLACE: C2RustUnnamed_4 = 1;
pub const D_REPLACE: C2RustUnnamed_8 = 26;
pub const D_FUSE: C2RustUnnamed_8 = 28;
pub const D_BYTEFALLBACK: C2RustUnnamed_8 = 27;
pub const TOKS_SPM_N_REPLACE: C2RustUnnamed_3 = 2;
pub const P_METASPACE: C2RustUnnamed_8 = 15;
pub const N_LOWER: C2RustUnnamed_8 = 4;
pub const N_STRIPACC: C2RustUnnamed_8 = 5;
pub const N_NFKD: C2RustUnnamed_8 = 3;
pub const N_NFD: C2RustUnnamed_8 = 1;
pub const N_PRECOMPILED: C2RustUnnamed_8 = 9;
pub const N_STRIP: C2RustUnnamed_8 = 6;
pub const TOKS_UNI_DEC_BFRF: C2RustUnnamed_6 = 3;
pub const TOKS_UNI_DEC_RBF: C2RustUnnamed_6 = 2;
pub const TOKS_UNI_DEC_META: C2RustUnnamed_6 = 1;
pub const TOKS_UNI_DEC_NONE: C2RustUnnamed_6 = 0;
pub const P_WSSPLIT: C2RustUnnamed_8 = 16;
pub const ST_REPLACE: C2RustUnnamed_10 = 9;
pub const ST_REP: C2RustUnnamed_10 = 1;
pub const ST_PREFIX: C2RustUnnamed_10 = 8;
pub const ST_COLLAPSE: C2RustUnnamed_10 = 7;
pub const ST_PC: C2RustUnnamed_10 = 6;
pub const ST_STRIP: C2RustUnnamed_10 = 5;
pub const ST_SA: C2RustUnnamed_10 = 3;
pub const ST_LOW: C2RustUnnamed_10 = 4;
pub const ST_NF: C2RustUnnamed_10 = 2;
pub const N_BERT: C2RustUnnamed_8 = 10;
pub const D_WORDPIECE: C2RustUnnamed_8 = 30;
pub const P_BERT: C2RustUnnamed_8 = 17;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hf_key {
    pub key: *const ::core::ffi::c_char,
    pub twice: *const ::core::ffi::c_char,
}
pub type C2RustUnnamed_1 = ::core::ffi::c_uint;
pub type C2RustUnnamed_2 = ::core::ffi::c_uint;
pub const TOKS_GB_PREV: C2RustUnnamed_2 = 2;
pub type C2RustUnnamed_3 = ::core::ffi::c_uint;
pub type C2RustUnnamed_4 = ::core::ffi::c_uint;
pub type C2RustUnnamed_5 = ::core::ffi::c_uint;
pub type C2RustUnnamed_6 = ::core::ffi::c_uint;
pub type C2RustUnnamed_7 = ::core::ffi::c_uint;
pub type C2RustUnnamed_8 = ::core::ffi::c_uint;
pub const D_CTC: C2RustUnnamed_8 = 32;
pub const D_BPE: C2RustUnnamed_8 = 31;
pub const P_FIXED: C2RustUnnamed_8 = 23;
pub const P_CHARDELIM: C2RustUnnamed_8 = 22;
pub const P_SCRIPTS: C2RustUnnamed_8 = 21;
pub const P_WS: C2RustUnnamed_8 = 18;
pub const N_BYTELEVEL: C2RustUnnamed_8 = 12;
pub const N_NMT: C2RustUnnamed_8 = 11;
pub type C2RustUnnamed_9 = ::core::ffi::c_uint;
pub type C2RustUnnamed_10 = ::core::ffi::c_uint;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
pub const INT64_MAX: ::core::ffi::c_long = 9223372036854775807 as ::core::ffi::c_long;
pub const UINT64_MAX: ::core::ffi::c_ulong = 18446744073709551615
    as ::core::ffi::c_ulong;
pub const TOKS_E_FORMAT: ::core::ffi::c_int = -(2 as ::core::ffi::c_int);
pub const TOKS_E_UNSUPPORTED: ::core::ffi::c_int = -(3 as ::core::ffi::c_int);
pub const TOKS_E_LIMIT: ::core::ffi::c_int = -(9 as ::core::ffi::c_int);
pub const TOKS_E_NOMEM: ::core::ffi::c_int = -(11 as ::core::ffi::c_int);
pub const TOKS_MAX_TEXT: ::core::ffi::c_ulonglong = (1 as ::core::ffi::c_ulonglong)
    << 29 as ::core::ffi::c_int;
pub const TOKS_MAX_TOKEN_BYTES: ::core::ffi::c_uint = 65535 as ::core::ffi::c_uint;
pub const TOKS_MAX_ADDED_BYTES: ::core::ffi::c_uint = 255 as ::core::ffi::c_uint;
pub const TOKS_MAX_IDS: ::core::ffi::c_uint = ((1 as ::core::ffi::c_uint)
    << 21 as ::core::ffi::c_int)
    .wrapping_sub(1 as ::core::ffi::c_uint);
pub const TOKS_ID_SPECIAL: ::core::ffi::c_uint = 2 as ::core::ffi::c_uint;
pub const TOKS_ID_LSTRIP: ::core::ffi::c_uint = 8 as ::core::ffi::c_uint;
pub const TOKS_ID_RSTRIP: ::core::ffi::c_uint = 16 as ::core::ffi::c_uint;
pub const TOKS_ID_SINGLE_WORD: ::core::ffi::c_uint = 32 as ::core::ffi::c_uint;
pub const TOKS_ID_NORMALIZED: ::core::ffi::c_uint = 64 as ::core::ffi::c_uint;
pub const TOKS_ALGO_BPE_BYTELEVEL: ::core::ffi::c_uint = 1 as ::core::ffi::c_uint;
pub const TOKS_ALGO_BPE_SPM: ::core::ffi::c_uint = 2 as ::core::ffi::c_uint;
pub const TOKS_ALGO_UNIGRAM: ::core::ffi::c_uint = 3 as ::core::ffi::c_uint;
pub const TOKS_ALGO_WORDPIECE: ::core::ffi::c_uint = 4 as ::core::ffi::c_uint;
pub const TOKS_TMPL_CL100K: ::core::ffi::c_uint = 1 as ::core::ffi::c_uint;
pub const TOKS_TMPL_O200K: ::core::ffi::c_uint = 2 as ::core::ffi::c_uint;
pub const TOKS_TMPL_DSV3: ::core::ffi::c_uint = 3 as ::core::ffi::c_uint;
pub const TOKS_TP_CONTR_NONE: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
pub const TOKS_TP_CONTR_CS: ::core::ffi::c_uint = 0x1 as ::core::ffi::c_uint;
pub const TOKS_TP_CONTR_CI: ::core::ffi::c_uint = 0x2 as ::core::ffi::c_uint;
pub const TOKS_TP_LPREFIX_ANY: ::core::ffi::c_uint = 0x4 as ::core::ffi::c_uint;
pub const TOKS_TP_DIGITS_1_3: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
pub const TOKS_TP_DIGITS_1: ::core::ffi::c_uint = 0x8 as ::core::ffi::c_uint;
pub const TOKS_TP_DIGITS_SP_RUN: ::core::ffi::c_uint = 0x10 as ::core::ffi::c_uint;
pub const TOKS_TP_PUNCT_NL: ::core::ffi::c_uint = 0x20 as ::core::ffi::c_uint;
pub const TOKS_TP_WS_NL: ::core::ffi::c_uint = 0x40 as ::core::ffi::c_uint;
pub const TOKS_TP_DIGIT_CUT: ::core::ffi::c_uint = 0x200 as ::core::ffi::c_uint;
pub const TOKS_TP_NL_CUT: ::core::ffi::c_uint = 0x400 as ::core::ffi::c_uint;
pub const TOKS_TP_GB_SP: ::core::ffi::c_uint = 0x800 as ::core::ffi::c_uint;
pub const TOKS_TP_HAN: ::core::ffi::c_uint = 0x80 as ::core::ffi::c_uint;
pub const TOKS_TP_NO_SLASH: ::core::ffi::c_uint = 0x100 as ::core::ffi::c_uint;
pub const TOKS_HSEED: ::core::ffi::c_uint = 0x9e3779b9 as ::core::ffi::c_uint;
pub const TOKS_PRIO_BITS: ::core::ffi::c_int = 22 as ::core::ffi::c_int;
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
#[inline]
unsafe extern "C" fn toks_meta_at(
    mut p: *const uint8_t,
    mut n: uint64_t,
    mut i: uint64_t,
) -> ::core::ffi::c_int {
    return (n.wrapping_sub(i) >= 3 as uint64_t && i < n
        && *p.offset(i as isize) as ::core::ffi::c_uint == 0xe2 as ::core::ffi::c_uint
        && *p.offset(i.wrapping_add(1 as uint64_t) as isize) as ::core::ffi::c_uint
            == 0x96 as ::core::ffi::c_uint
        && *p.offset(i.wrapping_add(2 as uint64_t) as isize) as ::core::ffi::c_uint
            == 0x81 as ::core::ffi::c_uint) as ::core::ffi::c_int;
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
unsafe extern "C" fn toks_jstr(
    mut v: *const jv,
    mut s: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut n: uint64_t = 0 as uint64_t;
    while *s.offset(n as isize) as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
        n = n.wrapping_add(1);
    }
    return (!v.is_null()
        && (*v).type_0 as ::core::ffi::c_int == JV_STR as ::core::ffi::c_int
        && (*v).s_len as uint64_t == n
        && (n == 0 as uint64_t
            || memcmp(
                (*v).s as *const ::core::ffi::c_void,
                s as *const ::core::ffi::c_void,
                n as size_t,
            ) == 0 as ::core::ffi::c_int)) as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn toks_jnull(mut v: *const jv) -> ::core::ffi::c_int {
    return (v.is_null()
        || (*v).type_0 as ::core::ffi::c_int == JV_NULL as ::core::ffi::c_int)
        as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn toks_jbool(mut v: *const jv) -> ::core::ffi::c_int {
    return (!v.is_null()
        && (*v).type_0 as ::core::ffi::c_int == JV_BOOL as ::core::ffi::c_int)
        as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn toks_jtrue(mut v: *const jv) -> ::core::ffi::c_int {
    return (toks_jbool(v) != 0 && (*v).num != 0 as int64_t) as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn toks_jtype(
    mut o: *const jv,
    mut t: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    return (!o.is_null()
        && (*o).type_0 as ::core::ffi::c_int == JV_OBJ as ::core::ffi::c_int
        && toks_jstr(
            toks_jv_get(o, b"type\0" as *const u8 as *const ::core::ffi::c_char),
            t,
        ) != 0) as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn toks_juint(
    mut v: *const jv,
    mut max: uint64_t,
) -> ::core::ffi::c_int {
    return (!v.is_null()
        && (*v).type_0 as ::core::ffi::c_int == JV_NUM as ::core::ffi::c_int
        && (*v).num_float as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        && (*v).num >= 0 as int64_t && (*v).num as uint64_t <= max)
        as ::core::ffi::c_int;
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
pub const TOKS_CLASSES_MARKS_ARE_LETTERS: ::core::ffi::c_uint = 0x1
    as ::core::ffi::c_uint;
pub const TOKS_CLASSES_HAN: ::core::ffi::c_uint = 0x2 as ::core::ffi::c_uint;
pub const TOKS_CLASSES_DSV3: ::core::ffi::c_uint = 0x8 as ::core::ffi::c_uint;
pub const TOKS_CLASSES_DIGITS: ::core::ffi::c_uint = 0x20 as ::core::ffi::c_uint;
pub const TOKS_CLASSES_BLOOM: ::core::ffi::c_uint = 0x40 as ::core::ffi::c_uint;
pub const TOKS_NS_CLEAN: ::core::ffi::c_uint = 0x1 as ::core::ffi::c_uint;
pub const TOKS_NS_CJK: ::core::ffi::c_uint = 0x2 as ::core::ffi::c_uint;
pub const TOKS_NS_STRIP_MN: ::core::ffi::c_uint = 0x4 as ::core::ffi::c_uint;
pub const TOKS_NS_LOWER: ::core::ffi::c_uint = 0x8 as ::core::ffi::c_uint;
pub const TOKS_NS_CANON: ::core::ffi::c_uint = 0x10 as ::core::ffi::c_uint;
pub const TOKS_NS_COMPOSE: ::core::ffi::c_uint = 0x20 as ::core::ffi::c_uint;
pub const TOKS_NS_COMPAT: ::core::ffi::c_uint = 0x40 as ::core::ffi::c_uint;
pub const TOKS_NS_STRIP_M: ::core::ffi::c_uint = 0x80 as ::core::ffi::c_uint;
pub const TOKS_NS_NFC: ::core::ffi::c_uint = TOKS_NS_CANON | TOKS_NS_COMPOSE;
pub const TOKS_NS_NFKC: ::core::ffi::c_uint = TOKS_NS_NFC | TOKS_NS_COMPAT;
pub const TOKS_NS_NFKD: ::core::ffi::c_uint = TOKS_NS_CANON | TOKS_NS_COMPAT;
pub const TOKS_SPM_NONE: ::core::ffi::c_uint = 0xffffffff as ::core::ffi::c_uint;
pub const TOKS_SPM_MAX_OPS: ::core::ffi::c_uint = 8 as ::core::ffi::c_uint;
pub const TOKS_SPM_MAX_STR: ::core::ffi::c_uint = 16 as ::core::ffi::c_uint;
pub const TOKS_SPM_BYTE_FALLBACK: ::core::ffi::c_uint = 0x1 as ::core::ffi::c_uint;
pub const TOKS_SPM_FUSE_UNK: ::core::ffi::c_uint = 0x2 as ::core::ffi::c_uint;
pub const TOKS_SPM_UNK_ERROR: ::core::ffi::c_uint = 0x4 as ::core::ffi::c_uint;
pub const TOKS_SPM_IGNORE_MERGES: ::core::ffi::c_uint = 0x8 as ::core::ffi::c_uint;
pub const TOKS_WPF_CLEAN: ::core::ffi::c_uint = TOKS_NS_CLEAN;
pub const TOKS_WPF_CHINESE: ::core::ffi::c_uint = TOKS_NS_CJK;
pub const TOKS_WPF_STRIP: ::core::ffi::c_uint = TOKS_NS_STRIP_MN;
pub const TOKS_WPF_LOWER: ::core::ffi::c_uint = TOKS_NS_LOWER;
pub const TOKS_WP_MAX_CHARS: ::core::ffi::c_uint = 1024 as ::core::ffi::c_uint;
pub const TOKS_WP_DEC_PREFIX: ::core::ffi::c_uint = 16 as ::core::ffi::c_uint;
#[no_mangle]
pub static mut TOKS_PATTERNS: [toks_pattern; 9] = [
    toks_pattern {
        regex: b"'s|'t|'re|'ve|'m|'ll|'d| ?\\p{L}+| ?\\p{N}+| ?[^\\s\\p{L}\\p{N}]+|\\s+(?!\\S)|\\s+\0"
            as *const u8 as *const ::core::ffi::c_char,
        params: TOKS_TP_CONTR_CS as uint32_t | TOKS_TP_DIGITS_SP_RUN as uint32_t,
        tmpl: TOKS_TMPL_CL100K as uint32_t,
        class_flags: 0 as uint32_t,
    },
    toks_pattern {
        regex: b"(?i:'s|'t|'re|'ve|'m|'ll|'d)|[^\\r\\n\\p{L}\\p{N}]?\\p{L}+|\\p{N}{1,3}| ?[^\\s\\p{L}\\p{N}]+[\\r\\n]*|\\s*[\\r\\n]+|\\s+(?!\\S)|\\s+\0"
            as *const u8 as *const ::core::ffi::c_char,
        params: TOKS_TP_CONTR_CI as uint32_t | TOKS_TP_LPREFIX_ANY as uint32_t
            | TOKS_TP_DIGITS_1_3 as uint32_t | TOKS_TP_PUNCT_NL as uint32_t
            | TOKS_TP_WS_NL as uint32_t,
        tmpl: TOKS_TMPL_CL100K as uint32_t,
        class_flags: 0 as uint32_t,
    },
    toks_pattern {
        regex: b"(?i:'s|'t|'re|'ve|'m|'ll|'d)|[^\\r\\n\\p{L}\\p{N}]?\\p{L}+|\\p{N}| ?[^\\s\\p{L}\\p{N}]+[\\r\\n]*|\\s*[\\r\\n]+|\\s+(?!\\S)|\\s+\0"
            as *const u8 as *const ::core::ffi::c_char,
        params: TOKS_TP_CONTR_CI as uint32_t | TOKS_TP_LPREFIX_ANY as uint32_t
            | TOKS_TP_DIGITS_1 as uint32_t | TOKS_TP_PUNCT_NL as uint32_t
            | TOKS_TP_WS_NL as uint32_t,
        tmpl: TOKS_TMPL_CL100K as uint32_t,
        class_flags: 0 as uint32_t,
    },
    toks_pattern {
        regex: b"(?i:'s|'t|'re|'ve|'m|'ll|'d)|[^\\r\\n\\p{L}\\p{N}]?[\\p{L}\\p{M}]+|\\p{N}| ?[^\\s\\p{L}\\p{M}\\p{N}]+[\\r\\n]*|\\s*[\\r\\n]+|\\s+(?!\\S)|\\s+\0"
            as *const u8 as *const ::core::ffi::c_char,
        params: TOKS_TP_CONTR_CI as uint32_t | TOKS_TP_LPREFIX_ANY as uint32_t
            | TOKS_TP_DIGITS_1 as uint32_t | TOKS_TP_PUNCT_NL as uint32_t
            | TOKS_TP_WS_NL as uint32_t,
        tmpl: TOKS_TMPL_CL100K as uint32_t,
        class_flags: TOKS_CLASSES_MARKS_ARE_LETTERS as uint32_t,
    },
    toks_pattern {
        regex: b"[^\\r\\n\\p{L}\\p{N}]?[\\p{Lu}\\p{Lt}\\p{Lm}\\p{Lo}\\p{M}]*[\\p{Ll}\\p{Lm}\\p{Lo}\\p{M}]+(?i:'s|'t|'re|'ve|'m|'ll|'d)?|[^\\r\\n\\p{L}\\p{N}]?[\\p{Lu}\\p{Lt}\\p{Lm}\\p{Lo}\\p{M}]+[\\p{Ll}\\p{Lm}\\p{Lo}\\p{M}]*(?i:'s|'t|'re|'ve|'m|'ll|'d)?|\\p{N}{1,3}| ?[^\\s\\p{L}\\p{N}]+[\\r\\n/]*|\\s*[\\r\\n]+|\\s+(?!\\S)|\\s+\0"
            as *const u8 as *const ::core::ffi::c_char,
        params: TOKS_TP_CONTR_CI as uint32_t | TOKS_TP_DIGITS_1_3 as uint32_t,
        tmpl: TOKS_TMPL_O200K as uint32_t,
        class_flags: 0 as uint32_t,
    },
    toks_pattern {
        regex: b"[^\\r\\n\\p{L}\\p{N}]?[\\p{Lu}\\p{Lt}\\p{Lm}\\p{Lo}\\p{M}]*[\\p{Ll}\\p{Lm}\\p{Lo}\\p{M}]+|[^\\r\\n\\p{L}\\p{N}]?[\\p{Lu}\\p{Lt}\\p{Lm}\\p{Lo}\\p{M}]+[\\p{Ll}\\p{Lm}\\p{Lo}\\p{M}]*|\\p{N}| ?[^\\s\\p{L}\\p{N}]+[\\r\\n/]*|\\s*[\\r\\n]+|\\s+(?!\\S)|\\s+\0"
            as *const u8 as *const ::core::ffi::c_char,
        params: TOKS_TP_CONTR_NONE as uint32_t | TOKS_TP_DIGITS_1 as uint32_t,
        tmpl: TOKS_TMPL_O200K as uint32_t,
        class_flags: 0 as uint32_t,
    },
    toks_pattern {
        regex: b"'(?i:[sdmt]|ll|ve|re)|[^\\r\\n\\p{L}\\p{N}]?+\\p{L}+|\\p{N}| ?[^\\s\\p{L}\\p{N}]++[\\r\\n]*|\\s*[\\r\\n]|\\s+(?!\\S)|\\s+\0"
            as *const u8 as *const ::core::ffi::c_char,
        params: TOKS_TP_CONTR_CI as uint32_t | TOKS_TP_LPREFIX_ANY as uint32_t
            | TOKS_TP_DIGITS_1 as uint32_t | TOKS_TP_PUNCT_NL as uint32_t
            | TOKS_TP_WS_NL as uint32_t,
        tmpl: TOKS_TMPL_CL100K as uint32_t,
        class_flags: 0 as uint32_t,
    },
    toks_pattern {
        regex: b"(?i:'s|'t|'re|'ve|'m|'ll|'d)|[^\r\n\\p{L}\\p{N}]?\\p{L}+|\\p{N}{1}| ?[^\\s\\p{L}\\p{N}\r\n]+|\\s*[\r\n]+|\\s+(?!\\S)|\\s+\0"
            as *const u8 as *const ::core::ffi::c_char,
        params: TOKS_TP_CONTR_CI as uint32_t | TOKS_TP_LPREFIX_ANY as uint32_t
            | TOKS_TP_DIGITS_1 as uint32_t | TOKS_TP_WS_NL as uint32_t,
        tmpl: TOKS_TMPL_CL100K as uint32_t,
        class_flags: 0 as uint32_t,
    },
    toks_pattern {
        regex: b" ?[^(\\s|[.,!?\xE2\x80\xA6\xE3\x80\x82\xEF\xBC\x8C\xE3\x80\x81\xE0\xA5\xA4\xDB\x94\xD8\x8C])]+\0"
            as *const u8 as *const ::core::ffi::c_char,
        params: TOKS_TP_GB_SP as uint32_t,
        tmpl: TOKS_TMPL_CL100K as uint32_t,
        class_flags: TOKS_CLASSES_BLOOM as uint32_t,
    },
];
#[no_mangle]
pub static mut TOKS_PATTERNS_N: uint32_t = 0;
#[no_mangle]
pub static mut TOKS_PATTERN_DIGITS: toks_pattern = toks_pattern {
    regex: b"'s|'t|'re|'ve|'m|'ll|'d| ?\\p{L}+| ?\\p{N}+| ?[^\\s\\p{L}\\p{N}]+|\\s+(?!\\S)|\\s+\0"
        as *const u8 as *const ::core::ffi::c_char,
    params: TOKS_TP_CONTR_CS as uint32_t | TOKS_TP_DIGITS_1 as uint32_t
        | TOKS_TP_DIGIT_CUT as uint32_t,
    tmpl: TOKS_TMPL_CL100K as uint32_t,
    class_flags: TOKS_CLASSES_DIGITS as uint32_t,
};
#[no_mangle]
pub static mut TOKS_PATTERN_P16: toks_pattern = toks_pattern {
    regex: b"(?i:'s|'t|'re|'ve|'m|'ll|'d)|[^\\r\\n\\p{L}\\p{N}]?\\p{L}+|\\p{N}+| ?[^\\s\\p{L}\\p{N}]+[\\r\\n]*|\\s*[\\r\\n]+|\\s+(?!\\S)|\\s+\0"
        as *const u8 as *const ::core::ffi::c_char,
    params: TOKS_TP_CONTR_CI as uint32_t | TOKS_TP_LPREFIX_ANY as uint32_t
        | TOKS_TP_DIGITS_1_3 as uint32_t | TOKS_TP_PUNCT_NL as uint32_t
        | TOKS_TP_WS_NL as uint32_t | TOKS_TP_DIGIT_CUT as uint32_t,
    tmpl: TOKS_TMPL_CL100K as uint32_t,
    class_flags: 0 as uint32_t,
};
#[no_mangle]
pub static mut TOKS_PATTERN_P20: toks_pattern = toks_pattern {
    regex: b"(?i:'s|'t|'re|'ve|'m|'ll|'d)|[^\\r\\n\\p{L}\\p{N}]?\\p{L}+|\\p{N}| ?[^\\s\\p{L}\\p{N}]+[\\r\\n]*|\\s*[\\r\\n]+|\\s+(?!\\S)|\\s+\0"
        as *const u8 as *const ::core::ffi::c_char,
    params: TOKS_TP_CONTR_CI as uint32_t | TOKS_TP_LPREFIX_ANY as uint32_t
        | TOKS_TP_DIGITS_1 as uint32_t | TOKS_TP_PUNCT_NL as uint32_t
        | TOKS_TP_WS_NL as uint32_t | TOKS_TP_NL_CUT as uint32_t,
    tmpl: TOKS_TMPL_CL100K as uint32_t,
    class_flags: 0 as uint32_t,
};
#[no_mangle]
pub static mut TOKS_PATTERN_KIMI: toks_pattern = toks_pattern {
    regex: b"[\\p{Han}]+|[^\\r\\n\\p{L}\\p{N}]?[\\p{Lu}\\p{Lt}\\p{Lm}\\p{Lo}\\p{M}&&[^\\p{Han}]]*[\\p{Ll}\\p{Lm}\\p{Lo}\\p{M}&&[^\\p{Han}]]+(?i:'s|'t|'re|'ve|'m|'ll|'d)?|[^\\r\\n\\p{L}\\p{N}]?[\\p{Lu}\\p{Lt}\\p{Lm}\\p{Lo}\\p{M}&&[^\\p{Han}]]+[\\p{Ll}\\p{Lm}\\p{Lo}\\p{M}&&[^\\p{Han}]]*(?i:'s|'t|'re|'ve|'m|'ll|'d)?|\\p{N}{1,3}| ?[^\\s\\p{L}\\p{N}]+[\\r\\n]*|\\s*[\\r\\n]+|\\s+(?!\\S)|\\s+\0"
        as *const u8 as *const ::core::ffi::c_char,
    params: TOKS_TP_CONTR_CI as uint32_t | TOKS_TP_DIGITS_1_3 as uint32_t
        | TOKS_TP_HAN as uint32_t | TOKS_TP_NO_SLASH as uint32_t,
    tmpl: TOKS_TMPL_O200K as uint32_t,
    class_flags: TOKS_CLASSES_HAN as uint32_t,
};
#[no_mangle]
pub static mut TOKS_DSV3_SPLITS: [*const ::core::ffi::c_char; 3] = [
    b"\\p{N}{1,3}\0" as *const u8 as *const ::core::ffi::c_char,
    b"[\xE4\xB8\x80-\xE9\xBE\xA5\xE3\x81\x80-\xE3\x82\x9F\xE3\x82\xA0-\xE3\x83\xBF]+\0"
        as *const u8 as *const ::core::ffi::c_char,
    b"[!\"#$%&'()*+,\\-./:;<=>?@\\[\\\\\\]^_`{|}~][A-Za-z]+|[^\r\n\\p{L}\\p{P}\\p{S}]?[\\p{L}\\p{M}]+| ?[\\p{P}\\p{S}]+[\r\n]*|\\s*[\r\n]+|\\s+(?!\\S)|\\s+\0"
        as *const u8 as *const ::core::ffi::c_char,
];
#[no_mangle]
pub static mut TOKS_PATTERN_DSV3: toks_pattern = toks_pattern {
    regex: b"[!\"#$%&'()*+,\\-./:;<=>?@\\[\\\\\\]^_`{|}~][A-Za-z]+|[^\r\n\\p{L}\\p{P}\\p{S}]?[\\p{L}\\p{M}]+| ?[\\p{P}\\p{S}]+[\r\n]*|\\s*[\r\n]+|\\s+(?!\\S)|\\s+\0"
        as *const u8 as *const ::core::ffi::c_char,
    params: 0 as uint32_t,
    tmpl: TOKS_TMPL_DSV3 as uint32_t,
    class_flags: TOKS_CLASSES_DSV3 as uint32_t,
};
pub const META_UTF8: [::core::ffi::c_char; 4] = unsafe {
    ::core::mem::transmute::<[u8; 4], [::core::ffi::c_char; 4]>(*b"\xE2\x96\x81\0")
};
unsafe extern "C" fn jv_num_zero(mut v: *const jv) -> ::core::ffi::c_int {
    if v.is_null() || (*v).type_0 as ::core::ffi::c_int != JV_NUM as ::core::ffi::c_int
        || (*v).s.is_null()
    {
        return 0 as ::core::ffi::c_int;
    }
    let mut i: uint32_t = 0 as uint32_t;
    while i < (*v).s_len {
        let mut c: uint8_t = *(*v).s.offset(i as isize);
        if c as ::core::ffi::c_int == 'e' as i32 || c as ::core::ffi::c_int == 'E' as i32
        {
            break;
        }
        if c as ::core::ffi::c_int >= '1' as i32 && c as ::core::ffi::c_int <= '9' as i32
        {
            return 0 as ::core::ffi::c_int;
        }
        i = i.wrapping_add(1);
    }
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn jflag(
    mut o: *const jv,
    mut key: *const ::core::ffi::c_char,
    mut dflt: ::core::ffi::c_int,
    mut out: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut v: *const jv = toks_jv_get(o, key);
    if toks_jnull(v) != 0 {
        *out = dflt;
        return 0 as ::core::ffi::c_int;
    }
    if (*v).type_0 as ::core::ffi::c_int != JV_BOOL as ::core::ffi::c_int {
        return -(1 as ::core::ffi::c_int);
    }
    *out = ((*v).num != 0 as int64_t) as ::core::ffi::c_int;
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn has_type(
    mut v: *const jv,
    mut list_key: *const ::core::ffi::c_char,
    mut type_0: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if toks_jtype(v, type_0) != 0 {
        return 1 as ::core::ffi::c_int;
    }
    let mut list: *const jv = if toks_jtype(
        v,
        b"Sequence\0" as *const u8 as *const ::core::ffi::c_char,
    ) != 0
    {
        toks_jv_get(v, list_key)
    } else {
        ::core::ptr::null_mut::<jv>()
    };
    let mut e: *const jv = if !list.is_null()
        && (*list).type_0 as ::core::ffi::c_int == JV_ARR as ::core::ffi::c_int
    {
        (*list).child
    } else {
        ::core::ptr::null_mut::<jv>()
    };
    while !e.is_null() {
        if toks_jtype(e, type_0) != 0 {
            return 1 as ::core::ffi::c_int;
        }
        e = (*e).next;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn replace_of(
    mut n: *const jv,
    mut str: *mut *const jv,
    mut rx: *mut *const jv,
    mut content: *mut *const jv,
) -> ::core::ffi::c_int {
    let mut pat: *const jv = if toks_jtype(
        n,
        b"Replace\0" as *const u8 as *const ::core::ffi::c_char,
    ) != 0
    {
        toks_jv_get(n, b"pattern\0" as *const u8 as *const ::core::ffi::c_char)
    } else {
        ::core::ptr::null_mut::<jv>()
    };
    if pat.is_null()
        || (*pat).type_0 as ::core::ffi::c_int != JV_OBJ as ::core::ffi::c_int
    {
        return 0 as ::core::ffi::c_int;
    }
    *str = toks_jv_get(pat, b"String\0" as *const u8 as *const ::core::ffi::c_char);
    *rx = toks_jv_get(pat, b"Regex\0" as *const u8 as *const ::core::ffi::c_char);
    *content = toks_jv_get(n, b"content\0" as *const u8 as *const ::core::ffi::c_char);
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn str_hash(
    mut h: uint32_t,
    mut p: *const uint8_t,
    mut n: uint32_t,
) -> uint32_t {
    let mut i: uint32_t = 0 as uint32_t;
    while i < n {
        h = TOKS_CRC32C_TAB[((h ^ *p.offset(i as isize) as uint32_t) & 0xff as uint32_t)
            as usize] ^ h >> 8 as ::core::ffi::c_int;
        i = i.wrapping_add(1);
    }
    return h;
}
#[no_mangle]
pub unsafe extern "C" fn toks_sidx_init(
    mut x: *mut toks_sidx,
    mut ar: *mut toks_arena,
    mut s: *const *const uint8_t,
    mut len: *const uint32_t,
    mut n: uint64_t,
) -> int64_t {
    let mut slots: uint64_t = 2 as uint64_t;
    while slots < (2 as uint64_t).wrapping_mul(n) {
        slots <<= 1 as ::core::ffi::c_int;
    }
    (*x).s = s;
    (*x).len = len;
    (*x).mask = slots.wrapping_sub(1 as uint64_t);
    (*x).slot = toks_ar_alloc(ar, slots.wrapping_mul(4 as uint64_t), 8 as uint64_t)
        as *mut uint32_t;
    if (*x).slot.is_null() {
        return TOKS_E_NOMEM as int64_t;
    }
    memset(
        (*x).slot as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        slots.wrapping_mul(4 as uint64_t) as size_t,
    );
    return 0 as int64_t;
}
#[no_mangle]
pub unsafe extern "C" fn toks_sidx_find(
    mut x: *const toks_sidx,
    mut a: *const uint8_t,
    mut al: uint32_t,
    mut b: *const uint8_t,
    mut bl: uint32_t,
) -> int64_t {
    let mut i: uint64_t = str_hash(str_hash(TOKS_HSEED as uint32_t, a, al), b, bl)
        as uint64_t & (*x).mask;
    let mut k: uint64_t = 0 as uint64_t;
    while k <= (*x).mask {
        let mut e: uint32_t = *(*x).slot.offset(i as isize);
        if e == 0 as uint32_t {
            return -(1 as ::core::ffi::c_int) as int64_t;
        }
        let mut t: *const uint8_t = *(*x)
            .s
            .offset(e.wrapping_sub(1 as uint32_t) as isize);
        if *(*x).len.offset(e.wrapping_sub(1 as uint32_t) as isize) as uint64_t
            == (al as uint64_t).wrapping_add(bl as uint64_t)
            && (al == 0 as uint32_t
                || memcmp(
                    t as *const ::core::ffi::c_void,
                    a as *const ::core::ffi::c_void,
                    al as size_t,
                ) == 0 as ::core::ffi::c_int)
            && (bl == 0 as uint32_t
                || memcmp(
                    t.offset(al as isize) as *const ::core::ffi::c_void,
                    b as *const ::core::ffi::c_void,
                    bl as size_t,
                ) == 0 as ::core::ffi::c_int)
        {
            return e.wrapping_sub(1 as uint32_t) as int64_t;
        }
        i = i.wrapping_add(1 as uint64_t) & (*x).mask;
        k = k.wrapping_add(1);
    }
    return -(1 as ::core::ffi::c_int) as int64_t;
}
#[no_mangle]
pub unsafe extern "C" fn toks_sidx_add(
    mut x: *mut toks_sidx,
    mut e: uint32_t,
) -> ::core::ffi::c_int {
    if toks_sidx_find(
        x,
        *(*x).s.offset(e as isize),
        *(*x).len.offset(e as isize),
        ::core::ptr::null::<uint8_t>(),
        0 as uint32_t,
    ) >= 0 as int64_t
    {
        return 1 as ::core::ffi::c_int;
    }
    let mut i: uint64_t = str_hash(
        TOKS_HSEED as uint32_t,
        *(*x).s.offset(e as isize),
        *(*x).len.offset(e as isize),
    ) as uint64_t & (*x).mask;
    while *(*x).slot.offset(i as isize) != 0 as uint32_t {
        i = i.wrapping_add(1 as uint64_t) & (*x).mask;
    }
    *(*x).slot.offset(i as isize) = e.wrapping_add(1 as uint32_t);
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn spm_vocab_find(
    mut c: *const toks_spm_config,
    mut s: *const uint8_t,
    mut n: uint32_t,
) -> uint32_t {
    let mut x: toks_sidx = toks_sidx {
        s: (*c).vocab,
        len: (*c).vocab_len,
        slot: (*c).vslot as uintptr_t as *mut uint32_t,
        mask: (*c).vmask,
    };
    let mut id: int64_t = toks_sidx_find(
        &raw mut x,
        s,
        n,
        ::core::ptr::null::<uint8_t>(),
        0 as uint32_t,
    );
    return if id < 0 as int64_t { TOKS_SPM_NONE as uint32_t } else { id as uint32_t };
}
#[no_mangle]
pub unsafe extern "C" fn toks_alpha_bytes(
    mut s: *const uint8_t,
    mut n: uint32_t,
    mut out: *mut uint8_t,
) -> int64_t {
    let mut i: uint32_t = 0 as uint32_t;
    let mut m: int64_t = 0 as int64_t;
    while i < n {
        let mut k: uint32_t = toks_utf8_len(
            s.offset(i as isize),
            n.wrapping_sub(i) as uint64_t,
        );
        if k == 0 as uint32_t {
            return -(1 as ::core::ffi::c_int) as int64_t;
        }
        let mut b: int32_t = toks_char_byte(
            if k == 1 as uint32_t {
                *s.offset(i as isize) as uint32_t
            } else {
                toks_cp_decode(s.offset(i as isize), k)
            },
        );
        if b < 0 as int32_t {
            return -(1 as ::core::ffi::c_int) as int64_t;
        }
        if !out.is_null() {
            *out.offset(m as isize) = b as uint8_t;
        }
        m += 1;
        i = i.wrapping_add(k);
    }
    return m;
}
#[no_mangle]
pub unsafe extern "C" fn toks_token_bytes(
    mut s: *const uint8_t,
    mut n: uint32_t,
    mut out: *mut uint8_t,
) -> uint32_t {
    let mut m: int64_t = toks_alpha_bytes(s, n, out);
    if m >= 0 as int64_t {
        return m as uint32_t;
    }
    if !out.is_null() && n != 0 as uint32_t {
        memcpy(
            out as *mut ::core::ffi::c_void,
            s as *const ::core::ffi::c_void,
            n as size_t,
        );
    }
    return n;
}
unsafe extern "C" fn read_vocab(
    mut model: *const jv,
    mut ar: *mut toks_arena,
    mut holes: ::core::ffi::c_int,
    mut vx: *mut toks_sidx,
    mut n_ids: *mut uint32_t,
    mut n_strings: *mut uint32_t,
    mut err: *mut toks_err,
) -> int64_t {
    let mut vocab: *const jv = toks_jv_get(
        model,
        b"vocab\0" as *const u8 as *const ::core::ffi::c_char,
    );
    if vocab.is_null()
        || (*vocab).type_0 as ::core::ffi::c_int != JV_OBJ as ::core::ffi::c_int
    {
        return toks_fail(
            err,
            TOKS_E_FORMAT as int64_t,
            b"model.vocab missing\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    let mut n: uint64_t = 0 as uint64_t;
    let mut max_id: uint64_t = 0 as uint64_t;
    let mut m: *const jv = (*vocab).child;
    while !m.is_null() {
        let mut v: *const jv = (*m).child;
        if v.is_null()
            || (*v).type_0 as ::core::ffi::c_int != JV_NUM as ::core::ffi::c_int
            || (*v).num_float as ::core::ffi::c_int != 0 as ::core::ffi::c_int
            || (*v).num < 0 as int64_t
        {
            return toks_fail(
                err,
                TOKS_E_FORMAT as int64_t,
                b"model.vocab entry not an id\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        if (*v).num >= TOKS_MAX_IDS as int64_t {
            return toks_fail(
                err,
                TOKS_E_LIMIT as int64_t,
                b"model.vocab id >= TOKS_MAX_IDS\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        if (*m).s_len > TOKS_MAX_TOKEN_BYTES as uint32_t {
            return toks_fail(
                err,
                TOKS_E_LIMIT as int64_t,
                b"model.vocab token > TOKS_MAX_TOKEN_BYTES\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        if (*v).num as uint64_t > max_id {
            max_id = (*v).num as uint64_t;
        }
        n = n.wrapping_add(1);
        m = (*m).next;
    }
    if n == 0 as uint64_t {
        return toks_fail(
            err,
            TOKS_E_FORMAT as int64_t,
            b"model.vocab empty\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if holes == 0 && n != max_id.wrapping_add(1 as uint64_t) {
        return toks_fail(
            err,
            TOKS_E_UNSUPPORTED as int64_t,
            b"model.vocab ids not dense\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    let mut ni: uint64_t = max_id.wrapping_add(1 as uint64_t);
    let mut vs: *mut *const uint8_t = toks_ar_alloc(
        ar,
        ni.wrapping_mul(::core::mem::size_of::<*mut uint8_t>() as uint64_t),
        8 as uint64_t,
    ) as *mut *const uint8_t;
    let mut vl: *mut uint32_t = toks_ar_alloc(
        ar,
        ni.wrapping_mul(4 as uint64_t).wrapping_add(4 as uint64_t),
        8 as uint64_t,
    ) as *mut uint32_t;
    if vs.is_null() || vl.is_null() || toks_sidx_init(vx, ar, vs, vl, n) != 0 as int64_t
    {
        return toks_fail(
            err,
            (if ni > n { TOKS_E_LIMIT } else { TOKS_E_NOMEM }) as int64_t,
            if ni > n {
                b"model.vocab: id holes past what the file's size gives the parse arena (32 B a source byte)\0"
                    as *const u8 as *const ::core::ffi::c_char
            } else {
                b"model.vocab arrays\0" as *const u8 as *const ::core::ffi::c_char
            },
        );
    }
    memset(
        vs as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ni.wrapping_mul(::core::mem::size_of::<*mut uint8_t>() as uint64_t) as size_t,
    );
    memset(
        vl as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ni.wrapping_mul(4 as uint64_t) as size_t,
    );
    let mut m_0: *const jv = (*vocab).child;
    while !m_0.is_null() {
        let mut id: uint64_t = (*(*m_0).child).num as uint64_t;
        if !(*vs.offset(id as isize)).is_null() {
            return toks_fail(
                err,
                TOKS_E_UNSUPPORTED as int64_t,
                if holes != 0 {
                    b"model.vocab: two strings share an id\0" as *const u8
                        as *const ::core::ffi::c_char
                } else {
                    b"model.vocab ids not dense\0" as *const u8
                        as *const ::core::ffi::c_char
                },
            );
        }
        let ref mut fresh3 = *vs.offset(id as isize);
        *fresh3 = if !(*m_0).s.is_null() {
            (*m_0).s
        } else {
            b"\0" as *const u8 as *const ::core::ffi::c_char as *const uint8_t
        };
        *vl.offset(id as isize) = (*m_0).s_len;
        if toks_sidx_add(vx, id as uint32_t) != 0 as ::core::ffi::c_int {
            return toks_fail(
                err,
                (if holes != 0 { TOKS_E_UNSUPPORTED } else { TOKS_E_FORMAT }) as int64_t,
                b"model.vocab repeats a token\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        m_0 = (*m_0).next;
    }
    *n_ids = ni as uint32_t;
    *n_strings = n as uint32_t;
    return 0 as int64_t;
}
unsafe extern "C" fn read_merges(
    mut model: *const jv,
    mut ar: *mut toks_arena,
    mut vx: *const toks_sidx,
    mut lro: *mut *const uint32_t,
    mut n_out: *mut uint32_t,
    mut err: *mut toks_err,
) -> int64_t {
    let mut merges: *const jv = toks_jv_get(
        model,
        b"merges\0" as *const u8 as *const ::core::ffi::c_char,
    );
    if merges.is_null()
        || (*merges).type_0 as ::core::ffi::c_int != JV_ARR as ::core::ffi::c_int
    {
        return toks_fail(
            err,
            TOKS_E_FORMAT as int64_t,
            b"model.merges missing\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    let mut n: uint64_t = 0 as uint64_t;
    let mut e: *const jv = (*merges).child;
    while !e.is_null() {
        n = n.wrapping_add(1);
        e = (*e).next;
    }
    if n >= ((1 as ::core::ffi::c_uint) << TOKS_PRIO_BITS) as uint64_t {
        return toks_fail(
            err,
            TOKS_E_LIMIT as int64_t,
            b"model.merges > 2^22\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    let mut ids: [*mut uint32_t; 3] = [::core::ptr::null_mut::<uint32_t>(); 3];
    let mut j: uint32_t = 0 as uint32_t;
    while j < 3 as uint32_t {
        ids[j as usize] = toks_ar_alloc(
            ar,
            n.wrapping_mul(4 as uint64_t).wrapping_add(4 as uint64_t),
            8 as uint64_t,
        ) as *mut uint32_t;
        if ids[j as usize].is_null() {
            return toks_fail(
                err,
                TOKS_E_NOMEM as int64_t,
                b"model.merges arrays\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        let ref mut fresh2 = *lro.offset(j as isize);
        *fresh2 = ids[j as usize];
        j = j.wrapping_add(1);
    }
    static mut VER: [uint8_t; 8] = [
        '#' as i32 as uint8_t,
        'v' as i32 as uint8_t,
        'e' as i32 as uint8_t,
        'r' as i32 as uint8_t,
        's' as i32 as uint8_t,
        'i' as i32 as uint8_t,
        'o' as i32 as uint8_t,
        'n' as i32 as uint8_t,
    ];
    let mut k: uint32_t = 0 as uint32_t;
    let mut current_block_43: u64;
    let mut e_0: *const jv = (*merges).child;
    while !e_0.is_null() {
        let mut ls: *const uint8_t = ::core::ptr::null::<uint8_t>();
        let mut rs: *const uint8_t = ::core::ptr::null::<uint8_t>();
        let mut ll: uint32_t = 0;
        let mut rl: uint32_t = 0;
        if (*e_0).type_0 as ::core::ffi::c_int == JV_STR as ::core::ffi::c_int {
            if (*e_0).s_len >= 8 as uint32_t
                && memcmp(
                    (*e_0).s as *const ::core::ffi::c_void,
                    &raw const VER as *const uint8_t as *const ::core::ffi::c_void,
                    8 as size_t,
                ) == 0 as ::core::ffi::c_int
            {
                current_block_43 = 1054647088692577877;
            } else {
                let mut sp: uint32_t = 0 as uint32_t;
                let mut spaces: uint32_t = 0 as uint32_t;
                let mut i: uint32_t = 0 as uint32_t;
                while i < (*e_0).s_len {
                    if *(*e_0).s.offset(i as isize) as ::core::ffi::c_int == ' ' as i32 {
                        spaces = spaces.wrapping_add(1);
                        sp = i;
                    }
                    i = i.wrapping_add(1);
                }
                if spaces != 1 as uint32_t {
                    return toks_fail(
                        err,
                        TOKS_E_FORMAT as int64_t,
                        b"model.merges entry is not 'left right'\0" as *const u8
                            as *const ::core::ffi::c_char,
                    );
                }
                ls = (*e_0).s;
                ll = sp;
                rs = (*e_0)
                    .s
                    .offset(sp as isize)
                    .offset(1 as ::core::ffi::c_uint as isize);
                rl = (*e_0).s_len.wrapping_sub(sp).wrapping_sub(1 as uint32_t);
                current_block_43 = 6450636197030046351;
            }
        } else {
            if (*e_0).type_0 as ::core::ffi::c_int == JV_ARR as ::core::ffi::c_int
                && !(*e_0).child.is_null() && !(*(*e_0).child).next.is_null()
                && (*(*(*e_0).child).next).next.is_null()
                && (*(*e_0).child).type_0 as ::core::ffi::c_int
                    == JV_STR as ::core::ffi::c_int
                && (*(*(*e_0).child).next).type_0 as ::core::ffi::c_int
                    == JV_STR as ::core::ffi::c_int
            {
                ls = (*(*e_0).child).s;
                ll = (*(*e_0).child).s_len;
                rs = (*(*(*e_0).child).next).s;
                rl = (*(*(*e_0).child).next).s_len;
            } else {
                return toks_fail(
                    err,
                    TOKS_E_FORMAT as int64_t,
                    b"model.merges entry is not a pair\0" as *const u8
                        as *const ::core::ffi::c_char,
                )
            }
            current_block_43 = 6450636197030046351;
        }
        match current_block_43 {
            6450636197030046351 => {
                let mut l: int64_t = toks_sidx_find(
                    vx,
                    ls,
                    ll,
                    ::core::ptr::null::<uint8_t>(),
                    0 as uint32_t,
                );
                let mut r: int64_t = toks_sidx_find(
                    vx,
                    rs,
                    rl,
                    ::core::ptr::null::<uint8_t>(),
                    0 as uint32_t,
                );
                let mut o: int64_t = toks_sidx_find(vx, ls, ll, rs, rl);
                if l < 0 as int64_t || r < 0 as int64_t || o < 0 as int64_t {
                    return toks_fail(
                        err,
                        TOKS_E_FORMAT as int64_t,
                        b"model.merges token out of vocabulary\0" as *const u8
                            as *const ::core::ffi::c_char,
                    );
                }
                *ids[0 as ::core::ffi::c_int as usize].offset(k as isize) = l
                    as uint32_t;
                *ids[1 as ::core::ffi::c_int as usize].offset(k as isize) = r
                    as uint32_t;
                *ids[2 as ::core::ffi::c_int as usize].offset(k as isize) = o
                    as uint32_t;
                k = k.wrapping_add(1);
            }
            _ => {}
        }
        e_0 = (*e_0).next;
    }
    *n_out = k;
    return 0 as int64_t;
}
unsafe extern "C" fn read_bpe(
    mut model: *const jv,
    mut flags: *mut uint32_t,
    mut err: *mut toks_err,
) -> int64_t {
    let mut type_0: *const jv = toks_jv_get(
        model,
        b"type\0" as *const u8 as *const ::core::ffi::c_char,
    );
    if !type_0.is_null()
        && toks_jstr(type_0, b"BPE\0" as *const u8 as *const ::core::ffi::c_char) == 0
    {
        return toks_fail(
            err,
            TOKS_E_UNSUPPORTED as int64_t,
            b"model type (not BPE, WordPiece or Unigram)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    let mut dr: *const jv = toks_jv_get(
        model,
        b"dropout\0" as *const u8 as *const ::core::ffi::c_char,
    );
    if toks_jnull(dr) == 0
        && (*dr).type_0 as ::core::ffi::c_int != JV_NUM as ::core::ffi::c_int
    {
        return toks_fail(
            err,
            TOKS_E_FORMAT as int64_t,
            b"model dropout\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if toks_jnull(dr) == 0 && jv_num_zero(dr) == 0 {
        return toks_fail(
            err,
            TOKS_E_UNSUPPORTED as int64_t,
            b"model dropout (not 0.0)\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    let mut csp: *const jv = toks_jv_get(
        model,
        b"continuing_subword_prefix\0" as *const u8 as *const ::core::ffi::c_char,
    );
    let mut ews: *const jv = toks_jv_get(
        model,
        b"end_of_word_suffix\0" as *const u8 as *const ::core::ffi::c_char,
    );
    if toks_jnull(csp) == 0
        && toks_jstr(csp, b"\0" as *const u8 as *const ::core::ffi::c_char) == 0
    {
        return toks_fail(
            err,
            TOKS_E_UNSUPPORTED as int64_t,
            b"model continuing_subword_prefix\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    if toks_jnull(ews) == 0
        && toks_jstr(ews, b"\0" as *const u8 as *const ::core::ffi::c_char) == 0
    {
        return toks_fail(
            err,
            TOKS_E_UNSUPPORTED as int64_t,
            b"model end_of_word_suffix\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    let mut bf: ::core::ffi::c_int = 0;
    let mut fu: ::core::ffi::c_int = 0;
    let mut im: ::core::ffi::c_int = 0;
    if jflag(
        model,
        b"byte_fallback\0" as *const u8 as *const ::core::ffi::c_char,
        0 as ::core::ffi::c_int,
        &raw mut bf,
    ) != 0 as ::core::ffi::c_int
        || jflag(
            model,
            b"fuse_unk\0" as *const u8 as *const ::core::ffi::c_char,
            0 as ::core::ffi::c_int,
            &raw mut fu,
        ) != 0 as ::core::ffi::c_int
        || jflag(
            model,
            b"ignore_merges\0" as *const u8 as *const ::core::ffi::c_char,
            0 as ::core::ffi::c_int,
            &raw mut im,
        ) != 0 as ::core::ffi::c_int
    {
        return toks_fail(
            err,
            TOKS_E_FORMAT as int64_t,
            b"model flags\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    *flags = ((if bf != 0 { TOKS_SPM_BYTE_FALLBACK } else { 0 as ::core::ffi::c_uint })
        | (if fu != 0 { TOKS_SPM_FUSE_UNK } else { 0 as ::core::ffi::c_uint })
        | (if im != 0 { TOKS_SPM_IGNORE_MERGES } else { 0 as ::core::ffi::c_uint }))
        as uint32_t;
    return 0 as int64_t;
}
unsafe extern "C" fn meta_scheme(
    mut o: *const jv,
    mut scheme: *mut uint32_t,
    mut err: *mut toks_err,
) -> int64_t {
    let mut ps: *const jv = toks_jv_get(
        o,
        b"prepend_scheme\0" as *const u8 as *const ::core::ffi::c_char,
    );
    let mut s: uint32_t = TOKS_SPM_PS_ALWAYS as ::core::ffi::c_int as uint32_t;
    if toks_jnull(ps) == 0 {
        if toks_jstr(ps, b"always\0" as *const u8 as *const ::core::ffi::c_char) != 0 {
            s = TOKS_SPM_PS_ALWAYS as ::core::ffi::c_int as uint32_t;
        } else if toks_jstr(ps, b"first\0" as *const u8 as *const ::core::ffi::c_char)
            != 0
        {
            s = TOKS_SPM_PS_FIRST as ::core::ffi::c_int as uint32_t;
        } else if toks_jstr(ps, b"never\0" as *const u8 as *const ::core::ffi::c_char)
            != 0
        {
            s = TOKS_SPM_PS_NEVER as ::core::ffi::c_int as uint32_t;
        } else {
            return toks_fail(
                err,
                TOKS_E_FORMAT as int64_t,
                b"Metaspace prepend_scheme\0" as *const u8 as *const ::core::ffi::c_char,
            )
        }
    }
    let mut aps: ::core::ffi::c_int = 0;
    if jflag(
        o,
        b"add_prefix_space\0" as *const u8 as *const ::core::ffi::c_char,
        1 as ::core::ffi::c_int,
        &raw mut aps,
    ) != 0 as ::core::ffi::c_int
    {
        return toks_fail(
            err,
            TOKS_E_FORMAT as int64_t,
            b"Metaspace add_prefix_space\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if aps == 0 && s != TOKS_SPM_PS_NEVER as ::core::ffi::c_int as uint32_t {
        return toks_fail(
            err,
            TOKS_E_FORMAT as int64_t,
            b"Metaspace add_prefix_space does not match prepend_scheme\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    *scheme = s;
    return 0 as int64_t;
}
unsafe extern "C" fn read_added(
    mut root: *const jv,
    mut ar: *mut toks_arena,
    mut cfg: *mut toks_config,
    mut vx: *const toks_sidx,
    mut err: *mut toks_err,
) -> int64_t {
    let mut list: *const jv = toks_jv_get(
        root,
        b"added_tokens\0" as *const u8 as *const ::core::ffi::c_char,
    );
    (*cfg).added = ::core::ptr::null_mut::<toks_cfg_added>();
    (*cfg).n_added = 0 as uint32_t;
    (*cfg).n_ids = (*cfg).n_vocab;
    if toks_jnull(list) != 0 {
        return 0 as int64_t;
    }
    if (*list).type_0 as ::core::ffi::c_int != JV_ARR as ::core::ffi::c_int {
        return toks_fail(
            err,
            TOKS_E_FORMAT as int64_t,
            b"added_tokens not an array\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    let mut n: uint64_t = 0 as uint64_t;
    let mut e: *const jv = (*list).child;
    while !e.is_null() {
        n = n.wrapping_add(1);
        e = (*e).next;
    }
    if n > 0xffff as uint64_t {
        return toks_fail(
            err,
            TOKS_E_LIMIT as int64_t,
            b"added_tokens > 65535\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if n == 0 as uint64_t {
        return 0 as int64_t;
    }
    let mut ad: *mut toks_cfg_added = toks_ar_alloc(
        ar,
        n.wrapping_mul(::core::mem::size_of::<toks_cfg_added>() as uint64_t),
        8 as uint64_t,
    ) as *mut toks_cfg_added;
    let mut cs: *mut *const uint8_t = toks_ar_alloc(
        ar,
        n.wrapping_mul(::core::mem::size_of::<*mut uint8_t>() as uint64_t),
        8 as uint64_t,
    ) as *mut *const uint8_t;
    let mut cl: *mut uint32_t = toks_ar_alloc(
        ar,
        n.wrapping_mul(4 as uint64_t),
        8 as uint64_t,
    ) as *mut uint32_t;
    let mut ax: toks_sidx = toks_sidx {
        s: ::core::ptr::null::<*const uint8_t>(),
        len: ::core::ptr::null::<uint32_t>(),
        slot: ::core::ptr::null_mut::<uint32_t>(),
        mask: 0,
    };
    if ad.is_null() || cs.is_null() || cl.is_null()
        || toks_sidx_init(&raw mut ax, ar, cs, cl, n) != 0 as int64_t
    {
        return toks_fail(
            err,
            TOKS_E_NOMEM as int64_t,
            b"added_tokens arrays\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    let mut k: uint32_t = 0 as uint32_t;
    let mut j: uint32_t = 0 as uint32_t;
    let mut next_id: uint64_t = (*cfg).n_strings as uint64_t;
    let mut e_0: *const jv = (*list).child;
    while !e_0.is_null() {
        if (*e_0).type_0 as ::core::ffi::c_int != JV_OBJ as ::core::ffi::c_int {
            return toks_fail(
                err,
                TOKS_E_FORMAT as int64_t,
                b"added_tokens entry\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        let mut id: *const jv = toks_jv_get(
            e_0,
            b"id\0" as *const u8 as *const ::core::ffi::c_char,
        );
        let mut content: *const jv = toks_jv_get(
            e_0,
            b"content\0" as *const u8 as *const ::core::ffi::c_char,
        );
        let mut special: *const jv = toks_jv_get(
            e_0,
            b"special\0" as *const u8 as *const ::core::ffi::c_char,
        );
        let mut norm: *const jv = toks_jv_get(
            e_0,
            b"normalized\0" as *const u8 as *const ::core::ffi::c_char,
        );
        let mut sw: *const jv = toks_jv_get(
            e_0,
            b"single_word\0" as *const u8 as *const ::core::ffi::c_char,
        );
        let mut ls: *const jv = toks_jv_get(
            e_0,
            b"lstrip\0" as *const u8 as *const ::core::ffi::c_char,
        );
        let mut rs: *const jv = toks_jv_get(
            e_0,
            b"rstrip\0" as *const u8 as *const ::core::ffi::c_char,
        );
        if toks_juint(id, 0xffffffff as uint64_t) == 0 || content.is_null()
            || (*content).type_0 as ::core::ffi::c_int != JV_STR as ::core::ffi::c_int
            || toks_jbool(special) == 0 || toks_jbool(norm) == 0 || toks_jbool(sw) == 0
            || toks_jbool(ls) == 0 || toks_jbool(rs) == 0
        {
            return toks_fail(
                err,
                TOKS_E_FORMAT as int64_t,
                b"added_tokens entry fields\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        if !((*content).s_len == 0 as uint32_t) {
            if (*content).s_len > TOKS_MAX_ADDED_BYTES as uint32_t {
                return toks_fail(
                    err,
                    TOKS_E_LIMIT as int64_t,
                    b"added token > TOKS_MAX_ADDED_BYTES\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
            }
            let ref mut fresh0 = *cs.offset(k as isize);
            *fresh0 = (*content).s;
            *cl.offset(k as isize) = (*content).s_len;
            let mut prev: int64_t = toks_sidx_find(
                &raw mut ax,
                (*content).s,
                (*content).s_len,
                ::core::ptr::null::<uint8_t>(),
                0 as uint32_t,
            );
            let mut a: *mut toks_cfg_added = if prev >= 0 as int64_t {
                ad.offset(prev as isize) as *mut toks_cfg_added
            } else {
                ad.offset(k as isize) as *mut toks_cfg_added
            };
            if prev < 0 as int64_t {
                toks_sidx_add(&raw mut ax, k);
                let mut vid: int64_t = if !(*cfg).spm.is_null() {
                    spm_vocab_find(
                        (*cfg).spm as *const toks_spm_config,
                        (*content).s,
                        (*content).s_len,
                    ) as int64_t
                } else {
                    toks_sidx_find(
                        vx,
                        (*content).s,
                        (*content).s_len,
                        ::core::ptr::null::<uint8_t>(),
                        0 as uint32_t,
                    )
                };
                if !(*cfg).spm.is_null() && vid == TOKS_SPM_NONE as int64_t {
                    vid = -(1 as ::core::ffi::c_int) as int64_t;
                }
                let mut tid: uint64_t = if vid >= 0 as int64_t {
                    vid as uint64_t
                } else {
                    let fresh1 = next_id;
                    next_id = next_id.wrapping_add(1);
                    fresh1
                };
                if tid >= TOKS_MAX_IDS as uint64_t {
                    return toks_fail(
                        err,
                        TOKS_E_LIMIT as int64_t,
                        b"added token id >= TOKS_MAX_IDS\0" as *const u8
                            as *const ::core::ffi::c_char,
                    );
                }
                memset(
                    a as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    ::core::mem::size_of::<toks_cfg_added>() as size_t,
                );
                (*a).content = (*content).s;
                (*a).len = (*content).s_len;
                (*a).id = tid as uint32_t;
                if tid.wrapping_add(1 as uint64_t) > (*cfg).n_ids as uint64_t {
                    (*cfg).n_ids = tid.wrapping_add(1 as uint64_t) as uint32_t;
                }
                k = k.wrapping_add(1);
            }
            (*a).special = ((*a).special as ::core::ffi::c_int
                | ((*special).num != 0 as int64_t) as ::core::ffi::c_int) as uint8_t;
            (*a).normalized = ((*norm).num != 0 as int64_t) as ::core::ffi::c_int
                as uint8_t;
            (*a).lstrip = ((*ls).num != 0 as int64_t) as ::core::ffi::c_int as uint8_t;
            (*a).rstrip = ((*rs).num != 0 as int64_t) as ::core::ffi::c_int as uint8_t;
            (*a).single_word = ((*sw).num != 0 as int64_t) as ::core::ffi::c_int
                as uint8_t;
            (*a).attr = ((if (*special).num != 0 as int64_t {
                TOKS_ID_SPECIAL
            } else {
                0 as ::core::ffi::c_uint
            })
                | (if (*ls).num != 0 as int64_t {
                    TOKS_ID_LSTRIP
                } else {
                    0 as ::core::ffi::c_uint
                })
                | (if (*rs).num != 0 as int64_t {
                    TOKS_ID_RSTRIP
                } else {
                    0 as ::core::ffi::c_uint
                })
                | (if (*sw).num != 0 as int64_t {
                    TOKS_ID_SINGLE_WORD
                } else {
                    0 as ::core::ffi::c_uint
                })
                | (if (*norm).num != 0 as int64_t {
                    TOKS_ID_NORMALIZED
                } else {
                    0 as ::core::ffi::c_uint
                })) as uint8_t;
            (*a).last = j;
        }
        e_0 = (*e_0).next;
        j = j.wrapping_add(1);
    }
    let mut rstrip_in: [uint32_t; 2] = [
        0 as ::core::ffi::c_uint,
        0 as ::core::ffi::c_uint,
    ];
    let mut i: uint32_t = 0 as uint32_t;
    while i < k {
        if (*ad.offset(i as isize)).rstrip as ::core::ffi::c_uint
            != 0 as ::core::ffi::c_uint
        {
            rstrip_in[(*ad.offset(i as isize)).normalized as usize] = 1
                as ::core::ffi::c_uint as uint32_t;
        }
        i = i.wrapping_add(1);
    }
    let mut i_0: uint32_t = 0 as uint32_t;
    while i_0 < k {
        if !((*ad.offset(i_0 as isize)).lstrip as ::core::ffi::c_uint
            == 0 as ::core::ffi::c_uint
            || (*ad.offset(i_0 as isize)).rstrip as ::core::ffi::c_uint
                != 0 as ::core::ffi::c_uint
            || rstrip_in[(*ad.offset(i_0 as isize)).normalized as usize]
                == 0 as uint32_t)
        {
            let mut p: uint32_t = 0 as uint32_t;
            let mut all_ws: uint32_t = 1 as uint32_t;
            while p < (*ad.offset(i_0 as isize)).len && all_ws != 0 as uint32_t {
                let mut q: uint32_t = toks_utf8_len(
                    (*ad.offset(i_0 as isize)).content.offset(p as isize),
                    (*ad.offset(i_0 as isize)).len.wrapping_sub(p) as uint64_t,
                );
                all_ws = toks_is_regex_ws(
                    if q == 1 as uint32_t {
                        *(*ad.offset(i_0 as isize)).content.offset(p as isize)
                            as uint32_t
                    } else {
                        toks_cp_decode(
                            (*ad.offset(i_0 as isize)).content.offset(p as isize),
                            q,
                        )
                    },
                ) as uint32_t;
                p = p.wrapping_add(q);
            }
            if all_ws != 0 as uint32_t {
                return toks_fail(
                    err,
                    TOKS_E_UNSUPPORTED as int64_t,
                    b"added token: all-whitespace lstrip token beside rstrip tokens (hf panics)\0"
                        as *const u8 as *const ::core::ffi::c_char,
                );
            }
        }
        i_0 = i_0.wrapping_add(1);
    }
    (*cfg).added = ad;
    (*cfg).n_added = k;
    return 0 as int64_t;
}
unsafe extern "C" fn read_template(
    mut tp: *const jv,
    mut cfg: *const toks_config,
    mut flat: *mut toks_pp_piece,
    mut nf_out: *mut uint32_t,
    mut err: *mut toks_err,
) -> int64_t {
    let mut nf: uint32_t = 0 as uint32_t;
    let mut seen_a: uint32_t = 0 as uint32_t;
    let mut single: *const jv = toks_jv_get(
        tp,
        b"single\0" as *const u8 as *const ::core::ffi::c_char,
    );
    let mut specials: *const jv = toks_jv_get(
        tp,
        b"special_tokens\0" as *const u8 as *const ::core::ffi::c_char,
    );
    if single.is_null()
        || (*single).type_0 as ::core::ffi::c_int != JV_ARR as ::core::ffi::c_int
        || specials.is_null()
        || (*specials).type_0 as ::core::ffi::c_int != JV_OBJ as ::core::ffi::c_int
    {
        return toks_fail(
            err,
            TOKS_E_FORMAT as int64_t,
            b"post_processor TemplateProcessing fields\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    let mut pc: *const jv = (*single).child;
    while !pc.is_null() {
        let mut sq: *const jv = if (*pc).type_0 as ::core::ffi::c_int
            == JV_OBJ as ::core::ffi::c_int
        {
            toks_jv_get(pc, b"Sequence\0" as *const u8 as *const ::core::ffi::c_char)
        } else {
            ::core::ptr::null_mut::<jv>()
        };
        let mut st: *const jv = if (*pc).type_0 as ::core::ffi::c_int
            == JV_OBJ as ::core::ffi::c_int
        {
            toks_jv_get(pc, b"SpecialToken\0" as *const u8 as *const ::core::ffi::c_char)
        } else {
            ::core::ptr::null_mut::<jv>()
        };
        let mut pid: *const jv = toks_jv_get(
            if !sq.is_null() { sq } else { st },
            b"id\0" as *const u8 as *const ::core::ffi::c_char,
        );
        let mut ty: *const jv = toks_jv_get(
            if !sq.is_null() { sq } else { st },
            b"type_id\0" as *const u8 as *const ::core::ffi::c_char,
        );
        if (sq == NULL as *const jv) as ::core::ffi::c_int
            == (st == NULL as *const jv) as ::core::ffi::c_int || pid.is_null()
            || (*pid).type_0 as ::core::ffi::c_int != JV_STR as ::core::ffi::c_int
        {
            return toks_fail(
                err,
                TOKS_E_FORMAT as int64_t,
                b"post_processor template piece\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        let mut type_0: uint32_t = if toks_juint(ty, 0xffffffff as uint64_t) != 0 {
            (*ty).num as uint32_t
        } else {
            0 as uint32_t
        };
        if !sq.is_null() {
            if toks_jstr(pid, b"A\0" as *const u8 as *const ::core::ffi::c_char) == 0 {
                return toks_fail(
                    err,
                    TOKS_E_UNSUPPORTED as int64_t,
                    b"post_processor single template uses $B\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
            }
            if seen_a != 0 as uint32_t {
                return toks_fail(
                    err,
                    TOKS_E_UNSUPPORTED as int64_t,
                    b"post_processor template: $A twice\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
            }
            if nf >= 64 as uint32_t {
                return toks_fail(
                    err,
                    TOKS_E_UNSUPPORTED as int64_t,
                    b"post_processor template > 64 pieces\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
            }
            seen_a = 1 as ::core::ffi::c_uint as uint32_t;
            (*flat.offset(nf as isize)).kind = TOKS_PPS_SEQ as ::core::ffi::c_int
                as uint32_t;
            (*flat.offset(nf as isize)).id = 0 as uint32_t;
            (*flat.offset(nf as isize)).type_0 = type_0;
            nf = nf.wrapping_add(1);
        } else {
            let mut sp: *const jv = ::core::ptr::null::<jv>();
            let mut m: *const jv = (*specials).child;
            while !m.is_null() {
                if (*m).s_len == (*pid).s_len
                    && ((*m).s_len == 0 as uint32_t
                        || memcmp(
                            (*m).s as *const ::core::ffi::c_void,
                            (*pid).s as *const ::core::ffi::c_void,
                            (*m).s_len as size_t,
                        ) == 0 as ::core::ffi::c_int)
                {
                    sp = (*m).child;
                }
                m = (*m).next;
            }
            let mut ids: *const jv = if !sp.is_null()
                && (*sp).type_0 as ::core::ffi::c_int == JV_OBJ as ::core::ffi::c_int
            {
                toks_jv_get(sp, b"ids\0" as *const u8 as *const ::core::ffi::c_char)
            } else {
                ::core::ptr::null_mut::<jv>()
            };
            if sp.is_null() {
                return toks_fail(
                    err,
                    TOKS_E_FORMAT as int64_t,
                    b"post_processor SpecialToken not in special_tokens (hf loads the file and panics on every encode that adds special tokens)\0"
                        as *const u8 as *const ::core::ffi::c_char,
                );
            }
            if ids.is_null()
                || (*ids).type_0 as ::core::ffi::c_int != JV_ARR as ::core::ffi::c_int
            {
                return toks_fail(
                    err,
                    TOKS_E_FORMAT as int64_t,
                    b"post_processor special_tokens entry without ids\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
            }
            let mut v: *const jv = (*ids).child;
            while !v.is_null() {
                if (*v).type_0 as ::core::ffi::c_int != JV_NUM as ::core::ffi::c_int
                    || (*v).num_float as ::core::ffi::c_int != 0 as ::core::ffi::c_int
                    || (*v).num < 0 as int64_t
                {
                    return toks_fail(
                        err,
                        TOKS_E_FORMAT as int64_t,
                        b"post_processor special ids\0" as *const u8
                            as *const ::core::ffi::c_char,
                    );
                }
                if (*v).num >= (*cfg).n_ids as int64_t {
                    return toks_fail(
                        err,
                        TOKS_E_UNSUPPORTED as int64_t,
                        b"post_processor id beyond the vocabulary\0" as *const u8
                            as *const ::core::ffi::c_char,
                    );
                }
                if nf >= 64 as uint32_t {
                    return toks_fail(
                        err,
                        TOKS_E_UNSUPPORTED as int64_t,
                        b"post_processor template > 64 pieces\0" as *const u8
                            as *const ::core::ffi::c_char,
                    );
                }
                (*flat.offset(nf as isize)).kind = TOKS_PPS_TOK as ::core::ffi::c_int
                    as uint32_t;
                (*flat.offset(nf as isize)).id = (*v).num as uint32_t;
                (*flat.offset(nf as isize)).type_0 = type_0;
                nf = nf.wrapping_add(1);
                v = (*v).next;
            }
        }
        pc = (*pc).next;
    }
    if seen_a == 0 as uint32_t {
        return toks_fail(
            err,
            TOKS_E_UNSUPPORTED as int64_t,
            b"post_processor template without $A\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    *nf_out = nf;
    return 0 as int64_t;
}
unsafe extern "C" fn pp_pair(
    mut v: *const jv,
    mut id: *mut uint32_t,
) -> ::core::ffi::c_int {
    if v.is_null() || (*v).type_0 as ::core::ffi::c_int != JV_ARR as ::core::ffi::c_int
        || (*v).child.is_null()
        || (*(*v).child).type_0 as ::core::ffi::c_int != JV_STR as ::core::ffi::c_int
    {
        return 0 as ::core::ffi::c_int;
    }
    let mut n: *const jv = (*(*v).child).next;
    if toks_juint(n, 0xffffffff as uint64_t) == 0 || !(*n).next.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    *id = (*n).num as uint32_t;
    return 1 as ::core::ffi::c_int;
}
static mut HF_TOP: [*const ::core::ffi::c_char; 10] = [
    b"version\0" as *const u8 as *const ::core::ffi::c_char,
    b"truncation\0" as *const u8 as *const ::core::ffi::c_char,
    b"padding\0" as *const u8 as *const ::core::ffi::c_char,
    b"added_tokens\0" as *const u8 as *const ::core::ffi::c_char,
    b"normalizer\0" as *const u8 as *const ::core::ffi::c_char,
    b"pre_tokenizer\0" as *const u8 as *const ::core::ffi::c_char,
    b"post_processor\0" as *const u8 as *const ::core::ffi::c_char,
    b"decoder\0" as *const u8 as *const ::core::ffi::c_char,
    b"model\0" as *const u8 as *const ::core::ffi::c_char,
    ::core::ptr::null::<::core::ffi::c_char>(),
];
static mut HF_ADDED: [hf_key; 8] = [
    hf_key {
        key: b"id\0" as *const u8 as *const ::core::ffi::c_char,
        twice: b"an added_tokens entry gives 'id' twice (hf refuses the file)\0"
            as *const u8 as *const ::core::ffi::c_char,
    },
    hf_key {
        key: b"content\0" as *const u8 as *const ::core::ffi::c_char,
        twice: b"an added_tokens entry gives 'content' twice (hf refuses the file)\0"
            as *const u8 as *const ::core::ffi::c_char,
    },
    hf_key {
        key: b"single_word\0" as *const u8 as *const ::core::ffi::c_char,
        twice: b"an added_tokens entry gives 'single_word' twice (hf refuses the file)\0"
            as *const u8 as *const ::core::ffi::c_char,
    },
    hf_key {
        key: b"lstrip\0" as *const u8 as *const ::core::ffi::c_char,
        twice: b"an added_tokens entry gives 'lstrip' twice (hf refuses the file)\0"
            as *const u8 as *const ::core::ffi::c_char,
    },
    hf_key {
        key: b"rstrip\0" as *const u8 as *const ::core::ffi::c_char,
        twice: b"an added_tokens entry gives 'rstrip' twice (hf refuses the file)\0"
            as *const u8 as *const ::core::ffi::c_char,
    },
    hf_key {
        key: b"normalized\0" as *const u8 as *const ::core::ffi::c_char,
        twice: b"an added_tokens entry gives 'normalized' twice (hf refuses the file)\0"
            as *const u8 as *const ::core::ffi::c_char,
    },
    hf_key {
        key: b"special\0" as *const u8 as *const ::core::ffi::c_char,
        twice: b"an added_tokens entry gives 'special' twice (hf refuses the file)\0"
            as *const u8 as *const ::core::ffi::c_char,
    },
    hf_key {
        key: ::core::ptr::null::<::core::ffi::c_char>(),
        twice: ::core::ptr::null::<::core::ffi::c_char>(),
    },
];
static mut HF_TRUNC: [hf_key; 5] = [
    hf_key {
        key: b"direction\0" as *const u8 as *const ::core::ffi::c_char,
        twice: b"truncation gives 'direction' twice (hf refuses the file)\0" as *const u8
            as *const ::core::ffi::c_char,
    },
    hf_key {
        key: b"max_length\0" as *const u8 as *const ::core::ffi::c_char,
        twice: b"truncation gives 'max_length' twice (hf refuses the file)\0"
            as *const u8 as *const ::core::ffi::c_char,
    },
    hf_key {
        key: b"strategy\0" as *const u8 as *const ::core::ffi::c_char,
        twice: b"truncation gives 'strategy' twice (hf refuses the file)\0" as *const u8
            as *const ::core::ffi::c_char,
    },
    hf_key {
        key: b"stride\0" as *const u8 as *const ::core::ffi::c_char,
        twice: b"truncation gives 'stride' twice (hf refuses the file)\0" as *const u8
            as *const ::core::ffi::c_char,
    },
    hf_key {
        key: ::core::ptr::null::<::core::ffi::c_char>(),
        twice: ::core::ptr::null::<::core::ffi::c_char>(),
    },
];
static mut HF_PAD: [hf_key; 7] = [
    hf_key {
        key: b"strategy\0" as *const u8 as *const ::core::ffi::c_char,
        twice: b"padding gives 'strategy' twice (hf refuses the file)\0" as *const u8
            as *const ::core::ffi::c_char,
    },
    hf_key {
        key: b"direction\0" as *const u8 as *const ::core::ffi::c_char,
        twice: b"padding gives 'direction' twice (hf refuses the file)\0" as *const u8
            as *const ::core::ffi::c_char,
    },
    hf_key {
        key: b"pad_to_multiple_of\0" as *const u8 as *const ::core::ffi::c_char,
        twice: b"padding gives 'pad_to_multiple_of' twice (hf refuses the file)\0"
            as *const u8 as *const ::core::ffi::c_char,
    },
    hf_key {
        key: b"pad_id\0" as *const u8 as *const ::core::ffi::c_char,
        twice: b"padding gives 'pad_id' twice (hf refuses the file)\0" as *const u8
            as *const ::core::ffi::c_char,
    },
    hf_key {
        key: b"pad_type_id\0" as *const u8 as *const ::core::ffi::c_char,
        twice: b"padding gives 'pad_type_id' twice (hf refuses the file)\0" as *const u8
            as *const ::core::ffi::c_char,
    },
    hf_key {
        key: b"pad_token\0" as *const u8 as *const ::core::ffi::c_char,
        twice: b"padding gives 'pad_token' twice (hf refuses the file)\0" as *const u8
            as *const ::core::ffi::c_char,
    },
    hf_key {
        key: ::core::ptr::null::<::core::ffi::c_char>(),
        twice: ::core::ptr::null::<::core::ffi::c_char>(),
    },
];
static mut HFPP_BERT: [*const ::core::ffi::c_char; 3] = [
    b"sep\0" as *const u8 as *const ::core::ffi::c_char,
    b"cls\0" as *const u8 as *const ::core::ffi::c_char,
    ::core::ptr::null::<::core::ffi::c_char>(),
];
static mut HFPP_BYTELEVEL: [*const ::core::ffi::c_char; 4] = [
    b"type\0" as *const u8 as *const ::core::ffi::c_char,
    b"add_prefix_space\0" as *const u8 as *const ::core::ffi::c_char,
    b"trim_offsets\0" as *const u8 as *const ::core::ffi::c_char,
    ::core::ptr::null::<::core::ffi::c_char>(),
];
static mut HFPP_TEMPLATE: [*const ::core::ffi::c_char; 4] = [
    b"single\0" as *const u8 as *const ::core::ffi::c_char,
    b"pair\0" as *const u8 as *const ::core::ffi::c_char,
    b"special_tokens\0" as *const u8 as *const ::core::ffi::c_char,
    ::core::ptr::null::<::core::ffi::c_char>(),
];
static mut HFPP_SEQUENCE: [*const ::core::ffi::c_char; 3] = [
    b"type\0" as *const u8 as *const ::core::ffi::c_char,
    b"processors\0" as *const u8 as *const ::core::ffi::c_char,
    ::core::ptr::null::<::core::ffi::c_char>(),
];
static mut HFPP_PIECE: [*const ::core::ffi::c_char; 3] = [
    b"id\0" as *const u8 as *const ::core::ffi::c_char,
    b"type_id\0" as *const u8 as *const ::core::ffi::c_char,
    ::core::ptr::null::<::core::ffi::c_char>(),
];
static mut HFPP_SPECIAL: [*const ::core::ffi::c_char; 4] = [
    b"id\0" as *const u8 as *const ::core::ffi::c_char,
    b"ids\0" as *const u8 as *const ::core::ffi::c_char,
    b"tokens\0" as *const u8 as *const ::core::ffi::c_char,
    ::core::ptr::null::<::core::ffi::c_char>(),
];
unsafe extern "C" fn key_is(
    mut m: *const jv,
    mut k: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut n: uint64_t = 0 as uint64_t;
    while *k.offset(n as isize) as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
        n = n.wrapping_add(1);
    }
    return ((*m).type_0 as ::core::ffi::c_int == JV_MEM as ::core::ffi::c_int
        && (*m).s_len as uint64_t == n
        && (n == 0 as uint64_t
            || memcmp(
                (*m).s as *const ::core::ffi::c_void,
                k as *const ::core::ffi::c_void,
                n as size_t,
            ) == 0 as ::core::ffi::c_int)) as ::core::ffi::c_int;
}
unsafe extern "C" fn count(
    mut o: *const jv,
    mut k: *const ::core::ffi::c_char,
) -> uint32_t {
    let mut c: uint32_t = 0 as uint32_t;
    let mut m: *const jv = (*o).child;
    while !m.is_null() {
        c = c.wrapping_add(key_is(m, k) as uint32_t);
        m = (*m).next;
    }
    return c;
}
unsafe extern "C" fn twice(mut o: *const jv, mut f: *const hf_key) -> *const hf_key {
    if o.is_null() || (*o).type_0 as ::core::ffi::c_int != JV_OBJ as ::core::ffi::c_int {
        return ::core::ptr::null::<hf_key>();
    }
    while !(*f).key.is_null() {
        if count(o, (*f).key) > 1 as uint32_t {
            return f;
        }
        f = f.offset(1);
    }
    return ::core::ptr::null::<hf_key>();
}
unsafe extern "C" fn not_once(
    mut o: *const jv,
    mut f: *const *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    while !(*f).is_null() {
        if count(o, *f) != 1 as uint32_t {
            return 1 as ::core::ffi::c_int;
        }
        f = f.offset(1);
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn not_one_key(mut o: *const jv) -> ::core::ffi::c_int {
    return (!o.is_null()
        && (*o).type_0 as ::core::ffi::c_int == JV_OBJ as ::core::ffi::c_int
        && ((*o).child.is_null() || !(*(*o).child).next.is_null()))
        as ::core::ffi::c_int;
}
unsafe extern "C" fn pieces_refused(mut t: *const jv) -> ::core::ffi::c_int {
    let mut e: *const jv = if !t.is_null()
        && (*t).type_0 as ::core::ffi::c_int == JV_ARR as ::core::ffi::c_int
    {
        (*t).child
    } else {
        ::core::ptr::null_mut::<jv>()
    };
    while !e.is_null() {
        if not_one_key(e) != 0 {
            return 1 as ::core::ffi::c_int;
        }
        let mut in_0: *const jv = if (*e).type_0 as ::core::ffi::c_int
            == JV_OBJ as ::core::ffi::c_int
        {
            (*(*e).child).child
        } else {
            ::core::ptr::null_mut::<jv>()
        };
        if !in_0.is_null()
            && (*in_0).type_0 as ::core::ffi::c_int == JV_OBJ as ::core::ffi::c_int
            && not_once(in_0, &raw const HFPP_PIECE as *const *const ::core::ffi::c_char)
                != 0
        {
            return 1 as ::core::ffi::c_int;
        }
        e = (*e).next;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn bert_refuses(mut p: *const jv) -> ::core::ffi::c_int {
    return not_once(p, &raw const HFPP_BERT as *const *const ::core::ffi::c_char);
}
unsafe extern "C" fn bytelevel_refuses(mut p: *const jv) -> ::core::ffi::c_int {
    return (not_once(p, &raw const HFPP_BYTELEVEL as *const *const ::core::ffi::c_char)
        != 0
        || count(p, b"use_regex\0" as *const u8 as *const ::core::ffi::c_char)
            > 1 as uint32_t
        || toks_jtype(p, b"ByteLevel\0" as *const u8 as *const ::core::ffi::c_char) == 0)
        as ::core::ffi::c_int;
}
unsafe extern "C" fn template_refuses(mut p: *const jv) -> ::core::ffi::c_int {
    if not_once(p, &raw const HFPP_TEMPLATE as *const *const ::core::ffi::c_char) != 0
        || pieces_refused(
            toks_jv_get(p, b"single\0" as *const u8 as *const ::core::ffi::c_char),
        ) != 0
        || pieces_refused(
            toks_jv_get(p, b"pair\0" as *const u8 as *const ::core::ffi::c_char),
        ) != 0
    {
        return 1 as ::core::ffi::c_int;
    }
    let mut sp: *const jv = toks_jv_get(
        p,
        b"special_tokens\0" as *const u8 as *const ::core::ffi::c_char,
    );
    let mut m: *const jv = if !sp.is_null()
        && (*sp).type_0 as ::core::ffi::c_int == JV_OBJ as ::core::ffi::c_int
    {
        (*sp).child
    } else {
        ::core::ptr::null_mut::<jv>()
    };
    while !m.is_null() {
        if !(*m).child.is_null()
            && (*(*m).child).type_0 as ::core::ffi::c_int == JV_OBJ as ::core::ffi::c_int
            && not_once(
                (*m).child,
                &raw const HFPP_SPECIAL as *const *const ::core::ffi::c_char,
            ) != 0
        {
            return 1 as ::core::ffi::c_int;
        }
        m = (*m).next;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn seq_refuses(mut p: *const jv) -> ::core::ffi::c_int {
    return (not_once(p, &raw const HFPP_SEQUENCE as *const *const ::core::ffi::c_char)
        != 0
        || toks_jtype(p, b"Sequence\0" as *const u8 as *const ::core::ffi::c_char) == 0)
        as ::core::ffi::c_int;
}
unsafe extern "C" fn others_refuse(mut p: *const jv) -> ::core::ffi::c_int {
    return (bert_refuses(p) != 0 && bytelevel_refuses(p) != 0
        && template_refuses(p) != 0) as ::core::ffi::c_int;
}
unsafe extern "C" fn pp_refused(mut p: *const jv) -> ::core::ffi::c_int {
    if p.is_null() || (*p).type_0 as ::core::ffi::c_int != JV_OBJ as ::core::ffi::c_int
        || others_refuse(p) == 0
    {
        return 0 as ::core::ffi::c_int;
    }
    if seq_refuses(p) != 0 {
        return 1 as ::core::ffi::c_int;
    }
    let mut l: *const jv = toks_jv_get(
        p,
        b"processors\0" as *const u8 as *const ::core::ffi::c_char,
    );
    let mut e: *const jv = if !l.is_null()
        && (*l).type_0 as ::core::ffi::c_int == JV_ARR as ::core::ffi::c_int
    {
        (*l).child
    } else {
        ::core::ptr::null_mut::<jv>()
    };
    while !e.is_null() {
        if (*e).type_0 as ::core::ffi::c_int == JV_OBJ as ::core::ffi::c_int
            && others_refuse(e) != 0 && seq_refuses(e) != 0
        {
            return 1 as ::core::ffi::c_int;
        }
        e = (*e).next;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn hf_member(mut m: *const jv, mut err: *mut toks_err) -> int64_t {
    let mut k: uint32_t = 0 as uint32_t;
    while !HF_TOP[k as usize].is_null() && key_is(m, HF_TOP[k as usize]) == 0 {
        k = k.wrapping_add(1);
    }
    let mut v: *const jv = (*m).child;
    let mut f: *const hf_key = ::core::ptr::null::<hf_key>();
    if k == 0 as uint32_t {
        return if toks_jstr(v, b"1.0\0" as *const u8 as *const ::core::ffi::c_char) != 0
        {
            0 as int64_t
        } else {
            toks_fail(
                err,
                TOKS_E_FORMAT as int64_t,
                b"version is not the string \"1.0\" (hf refuses the file)\0" as *const u8
                    as *const ::core::ffi::c_char,
            )
        };
    }
    if k == 9 as uint32_t {
        return toks_fail(
            err,
            TOKS_E_FORMAT as int64_t,
            b"a top-level key other than version, truncation, padding, added_tokens, normalizer, pre_tokenizer, post_processor, decoder and model (hf refuses the file)\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if k == 1 as uint32_t {
        f = twice(v, &raw const HF_TRUNC as *const hf_key);
    }
    if k == 2 as uint32_t {
        f = twice(v, &raw const HF_PAD as *const hf_key);
        if f.is_null() && !v.is_null()
            && (*v).type_0 as ::core::ffi::c_int == JV_OBJ as ::core::ffi::c_int
            && not_one_key(
                toks_jv_get(v, b"strategy\0" as *const u8 as *const ::core::ffi::c_char),
            ) != 0
        {
            return toks_fail(
                err,
                TOKS_E_FORMAT as int64_t,
                b"padding: strategy is an object of other than one key (hf refuses the file)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    }
    let mut e: *const jv = if k == 3 as uint32_t && !v.is_null()
        && (*v).type_0 as ::core::ffi::c_int == JV_ARR as ::core::ffi::c_int
    {
        (*v).child
    } else {
        ::core::ptr::null_mut::<jv>()
    };
    while !e.is_null() && f.is_null() {
        f = twice(e, &raw const HF_ADDED as *const hf_key);
        e = (*e).next;
    }
    if k == 6 as uint32_t && pp_refused(v) != 0 {
        if seq_refuses(v) == 0 {
            return toks_fail(
                err,
                TOKS_E_FORMAT as int64_t,
                b"post_processor.processors: an element every variant hf tries refuses (hf refuses the file)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
        return toks_fail(
            err,
            TOKS_E_FORMAT as int64_t,
            b"post_processor: every variant hf tries refuses it (Roberta, Bert: sep and cls once each; ByteLevel, Template, Sequence: their fields once, ByteLevel and Sequence their own type; hf refuses the file)\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
    }
    return if !f.is_null() {
        toks_fail(err, TOKS_E_FORMAT as int64_t, (*f).twice)
    } else {
        0 as int64_t
    };
}
unsafe extern "C" fn hf_refuses(mut root: *const jv, mut err: *mut toks_err) -> int64_t {
    let mut m: *const jv = (*root).child;
    while !m.is_null() {
        let mut r: int64_t = hf_member(m, err);
        if r != 0 as int64_t {
            return r;
        }
        m = (*m).next;
    }
    return 0 as int64_t;
}
unsafe extern "C" fn pp_kind(
    mut e: *const jv,
    mut cls: *mut uint32_t,
    mut sep: *mut uint32_t,
) -> uint32_t {
    if (*e).type_0 as ::core::ffi::c_int != JV_OBJ as ::core::ffi::c_int {
        return PP_NONE as ::core::ffi::c_int as uint32_t;
    }
    if bert_refuses(e) == 0
        && pp_pair(
            toks_jv_get(e, b"sep\0" as *const u8 as *const ::core::ffi::c_char),
            sep,
        ) != 0
        && pp_pair(
            toks_jv_get(e, b"cls\0" as *const u8 as *const ::core::ffi::c_char),
            cls,
        ) != 0
    {
        return PP_CLS_SEP as ::core::ffi::c_int as uint32_t;
    }
    let mut ur: *const jv = toks_jv_get(
        e,
        b"use_regex\0" as *const u8 as *const ::core::ffi::c_char,
    );
    if bytelevel_refuses(e) == 0
        && toks_jbool(
            toks_jv_get(
                e,
                b"add_prefix_space\0" as *const u8 as *const ::core::ffi::c_char,
            ),
        ) != 0
        && toks_jbool(
            toks_jv_get(e, b"trim_offsets\0" as *const u8 as *const ::core::ffi::c_char),
        ) != 0
        && (ur.is_null()
            || (*ur).type_0 as ::core::ffi::c_int == JV_BOOL as ::core::ffi::c_int)
    {
        return PP_BYTELEVEL as ::core::ffi::c_int as uint32_t;
    }
    let mut single: *const jv = toks_jv_get(
        e,
        b"single\0" as *const u8 as *const ::core::ffi::c_char,
    );
    let mut pair: *const jv = toks_jv_get(
        e,
        b"pair\0" as *const u8 as *const ::core::ffi::c_char,
    );
    let mut sp: *const jv = toks_jv_get(
        e,
        b"special_tokens\0" as *const u8 as *const ::core::ffi::c_char,
    );
    if template_refuses(e) == 0 && !single.is_null()
        && (*single).type_0 as ::core::ffi::c_int == JV_ARR as ::core::ffi::c_int
        && !pair.is_null()
        && (*pair).type_0 as ::core::ffi::c_int == JV_ARR as ::core::ffi::c_int
        && !sp.is_null()
        && (*sp).type_0 as ::core::ffi::c_int == JV_OBJ as ::core::ffi::c_int
    {
        return PP_TEMPLATE as ::core::ffi::c_int as uint32_t;
    }
    let mut list: *const jv = toks_jv_get(
        e,
        b"processors\0" as *const u8 as *const ::core::ffi::c_char,
    );
    if toks_jtype(e, b"Sequence\0" as *const u8 as *const ::core::ffi::c_char) != 0
        && !list.is_null()
        && (*list).type_0 as ::core::ffi::c_int == JV_ARR as ::core::ffi::c_int
    {
        return PP_SEQUENCE as ::core::ffi::c_int as uint32_t;
    }
    return PP_NONE as ::core::ffi::c_int as uint32_t;
}
unsafe extern "C" fn read_post_processor(
    mut root: *const jv,
    mut ar: *mut toks_arena,
    mut cfg: *mut toks_config,
    mut err: *mut toks_err,
) -> int64_t {
    let mut pp: *const jv = toks_jv_get(
        root,
        b"post_processor\0" as *const u8 as *const ::core::ffi::c_char,
    );
    (*cfg).pp_single = ::core::ptr::null_mut::<toks_pp_piece>();
    (*cfg).n_pp_single = 0 as uint32_t;
    if toks_jnull(pp) != 0 {
        return 0 as int64_t;
    }
    if (*pp).type_0 as ::core::ffi::c_int != JV_OBJ as ::core::ffi::c_int {
        return toks_fail(
            err,
            TOKS_E_FORMAT as int64_t,
            b"post_processor\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    let mut cls: uint32_t = 0 as uint32_t;
    let mut sep: uint32_t = 0 as uint32_t;
    let mut tp_kind: uint32_t = PP_NONE as ::core::ffi::c_int as uint32_t;
    let mut tp_cls: uint32_t = 0 as uint32_t;
    let mut tp_sep: uint32_t = 0 as uint32_t;
    let mut seq: ::core::ffi::c_int = (pp_kind(pp, &raw mut cls, &raw mut sep)
        == PP_SEQUENCE as ::core::ffi::c_int as uint32_t) as ::core::ffi::c_int;
    let mut tp: *const jv = ::core::ptr::null::<jv>();
    let mut e: *const jv = if seq != 0 {
        (*toks_jv_get(pp, b"processors\0" as *const u8 as *const ::core::ffi::c_char))
            .child as *const jv
    } else {
        pp
    };
    while !e.is_null() {
        let mut k: uint32_t = pp_kind(e, &raw mut cls, &raw mut sep);
        if !(k == PP_BYTELEVEL as ::core::ffi::c_int as uint32_t) {
            if k == PP_NONE as ::core::ffi::c_int as uint32_t {
                return toks_fail(
                    err,
                    TOKS_E_UNSUPPORTED as int64_t,
                    b"post_processor type (ByteLevel, TemplateProcessing, RobertaProcessing, BertProcessing)\0"
                        as *const u8 as *const ::core::ffi::c_char,
                );
            }
            if k == PP_SEQUENCE as ::core::ffi::c_int as uint32_t {
                return toks_fail(
                    err,
                    TOKS_E_UNSUPPORTED as int64_t,
                    b"post_processor: Sequence inside a Sequence\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
            }
            if !tp.is_null() {
                return toks_fail(
                    err,
                    TOKS_E_UNSUPPORTED as int64_t,
                    b"post_processor: two that add ids\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
            }
            tp = e;
            tp_kind = k;
            tp_cls = cls;
            tp_sep = sep;
        }
        e = if seq != 0 { (*e).next } else { ::core::ptr::null_mut::<jv>() };
    }
    if tp.is_null() {
        return 0 as int64_t;
    }
    let mut flat: [toks_pp_piece; 64] = [toks_pp_piece {
        kind: 0,
        id: 0,
        type_0: 0,
    }; 64];
    let mut nf: uint32_t = 3 as uint32_t;
    if tp_kind == PP_CLS_SEP as ::core::ffi::c_int as uint32_t {
        if tp_cls >= (*cfg).n_ids || tp_sep >= (*cfg).n_ids {
            return toks_fail(
                err,
                TOKS_E_UNSUPPORTED as int64_t,
                b"post_processor id beyond the vocabulary\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        memset(
            &raw mut flat as *mut toks_pp_piece as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            (3 as size_t).wrapping_mul(::core::mem::size_of::<toks_pp_piece>() as size_t),
        );
        flat[0 as ::core::ffi::c_int as usize].kind = TOKS_PPS_TOK as ::core::ffi::c_int
            as uint32_t;
        flat[0 as ::core::ffi::c_int as usize].id = tp_cls;
        flat[1 as ::core::ffi::c_int as usize].kind = TOKS_PPS_SEQ as ::core::ffi::c_int
            as uint32_t;
        flat[1 as ::core::ffi::c_int as usize].id = 0 as uint32_t;
        flat[2 as ::core::ffi::c_int as usize].kind = TOKS_PPS_TOK as ::core::ffi::c_int
            as uint32_t;
        flat[2 as ::core::ffi::c_int as usize].id = tp_sep;
    } else {
        let mut r: int64_t = read_template(
            tp,
            cfg,
            &raw mut flat as *mut toks_pp_piece,
            &raw mut nf,
            err,
        );
        if r != 0 as int64_t {
            return r;
        }
    }
    let mut out: *mut toks_pp_piece = toks_ar_alloc(
        ar,
        (nf as uint64_t)
            .wrapping_mul(::core::mem::size_of::<toks_pp_piece>() as uint64_t),
        8 as uint64_t,
    ) as *mut toks_pp_piece;
    if out.is_null() {
        return toks_fail(
            err,
            TOKS_E_NOMEM as int64_t,
            b"post_processor pieces\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    memcpy(
        out as *mut ::core::ffi::c_void,
        &raw mut flat as *mut toks_pp_piece as *const ::core::ffi::c_void,
        (nf as size_t).wrapping_mul(::core::mem::size_of::<toks_pp_piece>() as size_t),
    );
    (*cfg).pp_single = out;
    (*cfg).n_pp_single = nf;
    return 0 as int64_t;
}
unsafe extern "C" fn read_trunc_pad(
    mut root: *const jv,
    mut cfg: *mut toks_config,
    mut err: *mut toks_err,
) -> int64_t {
    let mut tr: *const jv = toks_jv_get(
        root,
        b"truncation\0" as *const u8 as *const ::core::ffi::c_char,
    );
    if toks_jnull(tr) == 0 {
        if (*tr).type_0 as ::core::ffi::c_int != JV_OBJ as ::core::ffi::c_int {
            return toks_fail(
                err,
                TOKS_E_FORMAT as int64_t,
                b"truncation\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        let mut dir: *const jv = toks_jv_get(
            tr,
            b"direction\0" as *const u8 as *const ::core::ffi::c_char,
        );
        let mut st: *const jv = toks_jv_get(
            tr,
            b"strategy\0" as *const u8 as *const ::core::ffi::c_char,
        );
        let mut ml: *const jv = toks_jv_get(
            tr,
            b"max_length\0" as *const u8 as *const ::core::ffi::c_char,
        );
        let mut sd: *const jv = toks_jv_get(
            tr,
            b"stride\0" as *const u8 as *const ::core::ffi::c_char,
        );
        if toks_juint(ml, UINT64_MAX as uint64_t >> 1 as ::core::ffi::c_int) == 0
            || toks_juint(sd, UINT64_MAX as uint64_t >> 1 as ::core::ffi::c_int) == 0
            || st.is_null()
            || (*st).type_0 as ::core::ffi::c_int != JV_STR as ::core::ffi::c_int
            || !dir.is_null()
                && (*dir).type_0 as ::core::ffi::c_int != JV_STR as ::core::ffi::c_int
        {
            return toks_fail(
                err,
                TOKS_E_FORMAT as int64_t,
                b"truncation fields\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        if !dir.is_null()
            && toks_jstr(dir, b"Right\0" as *const u8 as *const ::core::ffi::c_char) == 0
        {
            return toks_fail(
                err,
                (if toks_jstr(dir, b"Left\0" as *const u8 as *const ::core::ffi::c_char)
                    != 0
                {
                    TOKS_E_UNSUPPORTED
                } else {
                    TOKS_E_FORMAT
                }) as int64_t,
                if toks_jstr(dir, b"Left\0" as *const u8 as *const ::core::ffi::c_char)
                    != 0
                {
                    b"truncation direction Left\0" as *const u8
                        as *const ::core::ffi::c_char
                } else {
                    b"truncation direction\0" as *const u8 as *const ::core::ffi::c_char
                },
            );
        }
        if toks_jstr(st, b"OnlySecond\0" as *const u8 as *const ::core::ffi::c_char) != 0
        {
            return toks_fail(
                err,
                TOKS_E_UNSUPPORTED as int64_t,
                b"truncation OnlySecond (hf fails every long single-sequence encode)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
        if toks_jstr(st, b"LongestFirst\0" as *const u8 as *const ::core::ffi::c_char)
            == 0
            && toks_jstr(st, b"OnlyFirst\0" as *const u8 as *const ::core::ffi::c_char)
                == 0
        {
            return toks_fail(
                err,
                TOKS_E_FORMAT as int64_t,
                b"truncation strategy\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        (*cfg).o.trunc_on = 1 as uint32_t;
        (*cfg).o.trunc_max = if (*ml).num as uint64_t as ::core::ffi::c_ulonglong
            > TOKS_MAX_TEXT
        {
            TOKS_MAX_TEXT as uint32_t
        } else {
            (*ml).num as uint32_t
        };
        (*cfg).o.trunc_stride = if (*sd).num as uint64_t > 0xffffffff as uint64_t {
            0xffffffff as uint32_t
        } else {
            (*sd).num as uint32_t
        };
    }
    let mut pd: *const jv = toks_jv_get(
        root,
        b"padding\0" as *const u8 as *const ::core::ffi::c_char,
    );
    if toks_jnull(pd) != 0 {
        return 0 as int64_t;
    }
    if (*pd).type_0 as ::core::ffi::c_int != JV_OBJ as ::core::ffi::c_int {
        return toks_fail(
            err,
            TOKS_E_FORMAT as int64_t,
            b"padding\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    let mut st_0: *const jv = toks_jv_get(
        pd,
        b"strategy\0" as *const u8 as *const ::core::ffi::c_char,
    );
    let mut dir_0: *const jv = toks_jv_get(
        pd,
        b"direction\0" as *const u8 as *const ::core::ffi::c_char,
    );
    let mut mul: *const jv = toks_jv_get(
        pd,
        b"pad_to_multiple_of\0" as *const u8 as *const ::core::ffi::c_char,
    );
    let mut ptok: *const jv = toks_jv_get(
        pd,
        b"pad_token\0" as *const u8 as *const ::core::ffi::c_char,
    );
    let mut pid: *const jv = toks_jv_get(
        pd,
        b"pad_id\0" as *const u8 as *const ::core::ffi::c_char,
    );
    let mut fx: *const jv = if !st_0.is_null()
        && (*st_0).type_0 as ::core::ffi::c_int == JV_OBJ as ::core::ffi::c_int
    {
        toks_jv_get(st_0, b"Fixed\0" as *const u8 as *const ::core::ffi::c_char)
    } else {
        ::core::ptr::null_mut::<jv>()
    };
    if toks_juint(pid, 0xffffffff as uint64_t) == 0
        || toks_juint(
            toks_jv_get(pd, b"pad_type_id\0" as *const u8 as *const ::core::ffi::c_char),
            0xffffffff as uint64_t,
        ) == 0 || ptok.is_null()
        || (*ptok).type_0 as ::core::ffi::c_int != JV_STR as ::core::ffi::c_int
        || dir_0.is_null()
        || (*dir_0).type_0 as ::core::ffi::c_int != JV_STR as ::core::ffi::c_int
        || st_0.is_null()
        || toks_jnull(mul) == 0
            && toks_juint(mul, UINT64_MAX as uint64_t >> 1 as ::core::ffi::c_int) == 0
    {
        return toks_fail(
            err,
            TOKS_E_FORMAT as int64_t,
            b"padding fields\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if toks_juint(fx, UINT64_MAX as uint64_t >> 1 as ::core::ffi::c_int) != 0 {
        if (*fx).num as uint64_t as ::core::ffi::c_ulonglong > TOKS_MAX_TEXT {
            return toks_fail(
                err,
                TOKS_E_UNSUPPORTED as int64_t,
                b"padding Fixed above 2^29\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        (*cfg).o.pad_fixed = 1 as uint32_t;
        (*cfg).o.pad_len = (*fx).num as uint32_t;
    } else if toks_jstr(
        st_0,
        b"BatchLongest\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        return toks_fail(
            err,
            TOKS_E_FORMAT as int64_t,
            b"padding strategy\0" as *const u8 as *const ::core::ffi::c_char,
        )
    }
    if toks_jstr(dir_0, b"Left\0" as *const u8 as *const ::core::ffi::c_char) == 0
        && toks_jstr(dir_0, b"Right\0" as *const u8 as *const ::core::ffi::c_char) == 0
    {
        return toks_fail(
            err,
            TOKS_E_FORMAT as int64_t,
            b"padding direction\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    let mut m: uint64_t = if toks_jnull(mul) != 0 {
        0 as uint64_t
    } else {
        (*mul).num as uint64_t
    };
    if m > ((1 as ::core::ffi::c_uint) << 20 as ::core::ffi::c_int) as uint64_t {
        return toks_fail(
            err,
            TOKS_E_UNSUPPORTED as int64_t,
            b"padding pad_to_multiple_of above 2^20\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    (*cfg).o.pad_left = toks_jstr(
        dir_0,
        b"Left\0" as *const u8 as *const ::core::ffi::c_char,
    ) as uint32_t;
    (*cfg).o.pad_multiple = m as uint32_t;
    (*cfg).o.pad_id = (*pid).num as uint32_t;
    (*cfg).o.pad_type_id = (*toks_jv_get(
        pd,
        b"pad_type_id\0" as *const u8 as *const ::core::ffi::c_char,
    ))
        .num as uint32_t;
    (*cfg).o.pad_file = 1 as ::core::ffi::c_uint as uint32_t;
    (*cfg).o.pad_on = ((*cfg).o.pad_fixed != 0 as uint32_t || m > 0 as uint64_t)
        as ::core::ffi::c_int as uint8_t as uint32_t;
    return 0 as int64_t;
}
unsafe extern "C" fn trunc_check(
    mut cfg: *const toks_config,
    mut err: *mut toks_err,
) -> int64_t {
    if (*cfg).o.pad_on != 0 && (*cfg).o.pad_id >= (*cfg).n_ids {
        return toks_fail(
            err,
            TOKS_E_UNSUPPORTED as int64_t,
            b"padding pad_id beyond the vocabulary\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    if (*cfg).o.trunc_on == 0 {
        return 0 as int64_t;
    }
    let mut n_added: uint32_t = if (*cfg).n_pp_single > 0 as uint32_t {
        (*cfg).n_pp_single.wrapping_sub(1 as uint32_t)
    } else {
        0 as uint32_t
    };
    if (*cfg).o.trunc_max < n_added {
        return toks_fail(
            err,
            TOKS_E_UNSUPPORTED as int64_t,
            b"truncation max_length below the post-processor's ids\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    let mut l_pp: uint32_t = (*cfg).o.trunc_max.wrapping_sub(n_added);
    let mut l_raw: uint32_t = (*cfg).o.trunc_max;
    if l_pp > 0 as uint32_t && (*cfg).o.trunc_stride >= l_pp
        || l_raw > 0 as uint32_t && (*cfg).o.trunc_stride >= l_raw
    {
        return toks_fail(
            err,
            TOKS_E_UNSUPPORTED as int64_t,
            b"truncation stride >= max_length (hf panics)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    return 0 as int64_t;
}
static mut STEP: [*const ::core::ffi::c_char; 33] = [
    b"normalizer NFC\0" as *const u8 as *const ::core::ffi::c_char,
    b"normalizer NFD\0" as *const u8 as *const ::core::ffi::c_char,
    b"normalizer NFKC\0" as *const u8 as *const ::core::ffi::c_char,
    b"normalizer NFKD\0" as *const u8 as *const ::core::ffi::c_char,
    b"normalizer Lowercase\0" as *const u8 as *const ::core::ffi::c_char,
    b"normalizer StripAccents\0" as *const u8 as *const ::core::ffi::c_char,
    b"normalizer Strip\0" as *const u8 as *const ::core::ffi::c_char,
    b"normalizer Replace\0" as *const u8 as *const ::core::ffi::c_char,
    b"normalizer Prepend\0" as *const u8 as *const ::core::ffi::c_char,
    b"normalizer Precompiled\0" as *const u8 as *const ::core::ffi::c_char,
    b"normalizer BertNormalizer\0" as *const u8 as *const ::core::ffi::c_char,
    b"normalizer Nmt\0" as *const u8 as *const ::core::ffi::c_char,
    b"normalizer ByteLevel\0" as *const u8 as *const ::core::ffi::c_char,
    b"pre_tokenizer ByteLevel\0" as *const u8 as *const ::core::ffi::c_char,
    b"pre_tokenizer Split\0" as *const u8 as *const ::core::ffi::c_char,
    b"pre_tokenizer Metaspace\0" as *const u8 as *const ::core::ffi::c_char,
    b"pre_tokenizer WhitespaceSplit\0" as *const u8 as *const ::core::ffi::c_char,
    b"pre_tokenizer BertPreTokenizer\0" as *const u8 as *const ::core::ffi::c_char,
    b"pre_tokenizer Whitespace\0" as *const u8 as *const ::core::ffi::c_char,
    b"pre_tokenizer Digits\0" as *const u8 as *const ::core::ffi::c_char,
    b"pre_tokenizer Punctuation\0" as *const u8 as *const ::core::ffi::c_char,
    b"pre_tokenizer UnicodeScripts\0" as *const u8 as *const ::core::ffi::c_char,
    b"pre_tokenizer CharDelimiterSplit\0" as *const u8 as *const ::core::ffi::c_char,
    b"pre_tokenizer FixedLength\0" as *const u8 as *const ::core::ffi::c_char,
    b"decoder ByteLevel\0" as *const u8 as *const ::core::ffi::c_char,
    b"decoder Metaspace\0" as *const u8 as *const ::core::ffi::c_char,
    b"decoder Replace\0" as *const u8 as *const ::core::ffi::c_char,
    b"decoder ByteFallback\0" as *const u8 as *const ::core::ffi::c_char,
    b"decoder Fuse\0" as *const u8 as *const ::core::ffi::c_char,
    b"decoder Strip\0" as *const u8 as *const ::core::ffi::c_char,
    b"decoder WordPiece\0" as *const u8 as *const ::core::ffi::c_char,
    b"decoder BPEDecoder\0" as *const u8 as *const ::core::ffi::c_char,
    b"decoder CTC\0" as *const u8 as *const ::core::ffi::c_char,
];
unsafe extern "C" fn read_steps(
    mut root: *const jv,
    mut c: uint32_t,
    mut accept: uint64_t,
    mut s: *mut steps,
    mut err: *mut toks_err,
) -> int64_t {
    static mut KEY: [*const ::core::ffi::c_char; 3] = [
        b"normalizer\0" as *const u8 as *const ::core::ffi::c_char,
        b"pre_tokenizer\0" as *const u8 as *const ::core::ffi::c_char,
        b"decoder\0" as *const u8 as *const ::core::ffi::c_char,
    ];
    static mut LIST: [*const ::core::ffi::c_char; 3] = [
        b"normalizers\0" as *const u8 as *const ::core::ffi::c_char,
        b"pretokenizers\0" as *const u8 as *const ::core::ffi::c_char,
        b"decoders\0" as *const u8 as *const ::core::ffi::c_char,
    ];
    static mut BAD: [*const ::core::ffi::c_char; 3] = [
        b"normalizer Sequence list\0" as *const u8 as *const ::core::ffi::c_char,
        b"pre_tokenizer Sequence list\0" as *const u8 as *const ::core::ffi::c_char,
        b"decoder Sequence list\0" as *const u8 as *const ::core::ffi::c_char,
    ];
    static mut UNKNOWN: [*const ::core::ffi::c_char; 3] = [
        b"normalizer (unknown type)\0" as *const u8 as *const ::core::ffi::c_char,
        b"pre_tokenizer (unknown type)\0" as *const u8 as *const ::core::ffi::c_char,
        b"decoder (unknown type)\0" as *const u8 as *const ::core::ffi::c_char,
    ];
    static mut FIRST: [uint8_t; 4] = [
        N_NFC as ::core::ffi::c_int as uint8_t,
        P_BYTELEVEL as ::core::ffi::c_int as uint8_t,
        D_BYTELEVEL as ::core::ffi::c_int as uint8_t,
        STEP_N as ::core::ffi::c_int as uint8_t,
    ];
    static mut SKIP: [uint8_t; 3] = [
        11 as ::core::ffi::c_int as uint8_t,
        14 as ::core::ffi::c_int as uint8_t,
        8 as ::core::ffi::c_int as uint8_t,
    ];
    let mut stk: [*const jv; 4] = [::core::ptr::null::<jv>(); 4];
    let mut e: *const jv = toks_jv_get(root, KEY[c as usize]);
    let mut d: uint32_t = 0 as uint32_t;
    let mut top: uint32_t = 1 as uint32_t;
    (*s).n = 0 as uint32_t;
    (*s).present = (toks_jnull(e) == 0) as ::core::ffi::c_int as uint32_t;
    while (*s).present != 0 && (!e.is_null() || d > 0 as uint32_t) {
        if e.is_null() {
            d = d.wrapping_sub(1);
            e = stk[d as usize];
        } else {
            let mut next: *const jv = if top != 0 {
                ::core::ptr::null_mut::<jv>()
            } else {
                (*e).next
            };
            top = 0 as uint32_t;
            if (*e).type_0 as ::core::ffi::c_int != JV_OBJ as ::core::ffi::c_int {
                return toks_fail(err, TOKS_E_FORMAT as int64_t, KEY[c as usize]);
            }
            if toks_jtype(e, b"Sequence\0" as *const u8 as *const ::core::ffi::c_char)
                != 0
            {
                let mut l: *const jv = toks_jv_get(e, LIST[c as usize]);
                if l.is_null()
                    || (*l).type_0 as ::core::ffi::c_int != JV_ARR as ::core::ffi::c_int
                {
                    return toks_fail(err, TOKS_E_FORMAT as int64_t, BAD[c as usize]);
                }
                if d == 4 as uint32_t {
                    return toks_fail(
                        err,
                        TOKS_E_UNSUPPORTED as int64_t,
                        b"component Sequences nested deeper than 4\0" as *const u8
                            as *const ::core::ffi::c_char,
                    );
                }
                let fresh4 = d;
                d = d.wrapping_add(1);
                stk[fresh4 as usize] = next;
                e = (*l).child;
            } else {
                let mut k: uint32_t = FIRST[c as usize] as uint32_t;
                while k < FIRST[c.wrapping_add(1 as uint32_t) as usize] as uint32_t
                    && toks_jtype(
                        e,
                        STEP[k as usize]
                            .offset(SKIP[c as usize] as ::core::ffi::c_int as isize),
                    ) == 0
                {
                    k = k.wrapping_add(1);
                }
                if k == FIRST[c.wrapping_add(1 as uint32_t) as usize] as uint32_t {
                    return toks_fail(
                        err,
                        TOKS_E_UNSUPPORTED as int64_t,
                        UNKNOWN[c as usize],
                    );
                }
                if accept as ::core::ffi::c_ulonglong
                    & (1 as ::core::ffi::c_ulonglong) << k
                    == 0 as ::core::ffi::c_ulonglong
                {
                    return toks_fail(
                        err,
                        TOKS_E_UNSUPPORTED as int64_t,
                        STEP[k as usize],
                    );
                }
                if (*s).n == 16 as uint32_t {
                    return toks_fail(
                        err,
                        TOKS_E_UNSUPPORTED as int64_t,
                        b"component: more than 16 steps\0" as *const u8
                            as *const ::core::ffi::c_char,
                    );
                }
                (*s).o[(*s).n as usize] = e;
                let fresh5 = (*s).n;
                (*s).n = (*s).n.wrapping_add(1);
                (*s).k[fresh5 as usize] = k as uint8_t;
                e = next;
            }
        }
    }
    return 0 as int64_t;
}
unsafe extern "C" fn bl_normalizer(
    mut root: *const jv,
    mut cfg: *mut toks_config,
    mut err: *mut toks_err,
) -> int64_t {
    let mut s: steps = steps {
        o: [::core::ptr::null::<jv>(); 16],
        k: [0; 16],
        n: 0,
        present: 0,
    };
    let mut r: int64_t = read_steps(
        root,
        C_NORM as ::core::ffi::c_int as uint32_t,
        ((1 as ::core::ffi::c_ulonglong) << N_NFC as ::core::ffi::c_int
            | (1 as ::core::ffi::c_ulonglong) << N_NFKC as ::core::ffi::c_int)
            as uint64_t,
        &raw mut s,
        err,
    );
    let mut k: uint32_t = 0 as uint32_t;
    while k < s.n {
        (*cfg).nfc = ((*cfg).nfc as ::core::ffi::c_int
            | (if s.k[k as usize] as ::core::ffi::c_int == N_NFKC as ::core::ffi::c_int {
                TOKS_NS_NFKC
            } else {
                TOKS_NS_NFC
            }) as uint8_t as ::core::ffi::c_int) as uint8_t;
        k = k.wrapping_add(1);
    }
    return r;
}
unsafe extern "C" fn bl_bytelevel(
    mut bl: *const jv,
    mut use_regex: *mut ::core::ffi::c_int,
    mut err: *mut toks_err,
) -> int64_t {
    let mut aps: *const jv = toks_jv_get(
        bl,
        b"add_prefix_space\0" as *const u8 as *const ::core::ffi::c_char,
    );
    let mut ur: *const jv = toks_jv_get(
        bl,
        b"use_regex\0" as *const u8 as *const ::core::ffi::c_char,
    );
    if toks_jbool(aps) == 0
        || !ur.is_null()
            && (*ur).type_0 as ::core::ffi::c_int != JV_BOOL as ::core::ffi::c_int
    {
        return toks_fail(
            err,
            TOKS_E_FORMAT as int64_t,
            b"pre_tokenizer ByteLevel fields\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    if (*aps).num != 0 as int64_t {
        return toks_fail(
            err,
            TOKS_E_UNSUPPORTED as int64_t,
            b"pre_tokenizer ByteLevel add_prefix_space=true\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    *use_regex = (ur.is_null() || (*ur).num != 0 as int64_t) as ::core::ffi::c_int;
    return 0 as int64_t;
}
unsafe extern "C" fn beh_of(mut b: *const jv) -> uint32_t {
    static mut BEH: [*const ::core::ffi::c_char; 5] = [
        b"Isolated\0" as *const u8 as *const ::core::ffi::c_char,
        b"Removed\0" as *const u8 as *const ::core::ffi::c_char,
        b"MergedWithPrevious\0" as *const u8 as *const ::core::ffi::c_char,
        b"MergedWithNext\0" as *const u8 as *const ::core::ffi::c_char,
        b"Contiguous\0" as *const u8 as *const ::core::ffi::c_char,
    ];
    let mut k: uint32_t = 0 as uint32_t;
    while k < 5 as uint32_t && toks_jstr(b, BEH[k as usize]) == 0 {
        k = k.wrapping_add(1);
    }
    return k;
}
unsafe extern "C" fn bl_split(
    mut s: *const jv,
    mut g: *mut toks_gen_spec,
    mut re: *mut *const jv,
    mut err: *mut toks_err,
) -> int64_t {
    let mut inv: *const jv = toks_jv_get(
        s,
        b"invert\0" as *const u8 as *const ::core::ffi::c_char,
    );
    let mut pat: *const jv = toks_jv_get(
        s,
        b"pattern\0" as *const u8 as *const ::core::ffi::c_char,
    );
    let mut str: *const jv = if !pat.is_null()
        && (*pat).type_0 as ::core::ffi::c_int == JV_OBJ as ::core::ffi::c_int
    {
        toks_jv_get(pat, b"String\0" as *const u8 as *const ::core::ffi::c_char)
    } else {
        ::core::ptr::null_mut::<jv>()
    };
    let mut p: *const jv = ::core::ptr::null::<jv>();
    *re = if !pat.is_null()
        && (*pat).type_0 as ::core::ffi::c_int == JV_OBJ as ::core::ffi::c_int
    {
        toks_jv_get(pat, b"Regex\0" as *const u8 as *const ::core::ffi::c_char)
    } else {
        ::core::ptr::null_mut::<jv>()
    };
    p = if !(*re).is_null() { *re } else { str };
    (*g).beh = beh_of(
        toks_jv_get(s, b"behavior\0" as *const u8 as *const ::core::ffi::c_char),
    );
    if (*g).beh == 5 as uint32_t || toks_jbool(inv) == 0 || p.is_null()
        || (*p).type_0 as ::core::ffi::c_int != JV_STR as ::core::ffi::c_int
    {
        return toks_fail(
            err,
            TOKS_E_FORMAT as int64_t,
            b"pre_tokenizer Split fields\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    (*g).inv = ((*inv).num != 0 as int64_t) as ::core::ffi::c_int as uint32_t;
    (*g).lit = (*re == NULL as *const jv) as ::core::ffi::c_int as uint32_t;
    (*g).s = (*p).s;
    (*g).n = (*p).s_len;
    return 0 as int64_t;
}
unsafe extern "C" fn bl_pretok(
    mut root: *const jv,
    mut cfg: *mut toks_config,
    mut ar: *mut toks_arena,
    mut err: *mut toks_err,
) -> int64_t {
    let mut s: steps = steps {
        o: [::core::ptr::null::<jv>(); 16],
        k: [0; 16],
        n: 0,
        present: 0,
    };
    let mut g: [toks_gen_spec; 16] = [toks_gen_spec {
        kind: 0,
        beh: 0,
        inv: 0,
        lit: 0,
        s: ::core::ptr::null::<uint8_t>(),
        n: 0,
    }; 16];
    let mut re: [*const jv; 16] = [::core::ptr::null::<jv>(); 16];
    let mut v: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut bl: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut r: int64_t = read_steps(
        root,
        C_PRE as ::core::ffi::c_int as uint32_t,
        ((1 as ::core::ffi::c_ulonglong) << P_BYTELEVEL as ::core::ffi::c_int
            | (1 as ::core::ffi::c_ulonglong) << P_SPLIT as ::core::ffi::c_int
            | (1 as ::core::ffi::c_ulonglong) << P_DIGITS as ::core::ffi::c_int
            | (1 as ::core::ffi::c_ulonglong) << P_PUNCT as ::core::ffi::c_int)
            as uint64_t,
        &raw mut s,
        err,
    );
    let mut n: uint32_t = s.n;
    let mut k: uint32_t = 0 as uint32_t;
    let mut i: uint32_t = 0 as uint32_t;
    while r == 0 as int64_t && i < n {
        memset(
            (&raw mut g as *mut toks_gen_spec).offset(i as isize) as *mut toks_gen_spec
                as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<toks_gen_spec>() as size_t,
        );
        re[i as usize] = ::core::ptr::null::<jv>();
        if s.k[i as usize] as ::core::ffi::c_int == P_SPLIT as ::core::ffi::c_int {
            g[i as usize].kind = TOKS_GS_SPLIT as ::core::ffi::c_int as uint32_t;
            r = bl_split(
                s.o[i as usize],
                (&raw mut g as *mut toks_gen_spec).offset(i as isize)
                    as *mut toks_gen_spec,
                (&raw mut re as *mut *const jv).offset(i as isize) as *mut *const jv,
                err,
            );
        } else if s.k[i as usize] as ::core::ffi::c_int
            == P_BYTELEVEL as ::core::ffi::c_int
        {
            g[i as usize].kind = TOKS_GS_BYTELEVEL as ::core::ffi::c_int as uint32_t;
            r = bl_bytelevel(s.o[i as usize], &raw mut v, err);
            g[i as usize].lit = v as uint32_t;
            bl = 1 as ::core::ffi::c_int;
        } else if s.k[i as usize] as ::core::ffi::c_int == P_DIGITS as ::core::ffi::c_int
        {
            g[i as usize].kind = TOKS_GS_DIGITS as ::core::ffi::c_int as uint32_t;
            r = if jflag(
                s.o[i as usize],
                b"individual_digits\0" as *const u8 as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int,
                &raw mut v,
            ) != 0 as ::core::ffi::c_int
            {
                toks_fail(
                    err,
                    TOKS_E_FORMAT as int64_t,
                    b"pre_tokenizer Digits\0" as *const u8 as *const ::core::ffi::c_char,
                )
            } else {
                0 as int64_t
            };
            g[i as usize].beh = (if v != 0 {
                TOKS_GB_ISOLATED as ::core::ffi::c_int
            } else {
                TOKS_GB_CONTIGUOUS as ::core::ffi::c_int
            }) as uint32_t;
        } else {
            let mut b: *const jv = toks_jv_get(
                s.o[i as usize],
                b"behavior\0" as *const u8 as *const ::core::ffi::c_char,
            );
            g[i as usize].kind = TOKS_GS_PUNCT as ::core::ffi::c_int as uint32_t;
            g[i as usize].beh = if toks_jnull(b) != 0 {
                TOKS_GB_ISOLATED as ::core::ffi::c_int as uint32_t
            } else {
                beh_of(b)
            };
            if g[i as usize].beh == 5 as uint32_t {
                r = toks_fail(
                    err,
                    TOKS_E_FORMAT as int64_t,
                    b"pre_tokenizer Punctuation behavior\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
            }
        }
        i = i.wrapping_add(1);
    }
    if r == 0 as int64_t && bl == 0 {
        r = toks_fail(
            err,
            TOKS_E_UNSUPPORTED as int64_t,
            if n == 0 as uint32_t {
                b"pre_tokenizer absent (byte-level bpe needs ByteLevel)\0" as *const u8
                    as *const ::core::ffi::c_char
            } else {
                b"pre_tokenizer without ByteLevel (byte-level bpe)\0" as *const u8
                    as *const ::core::ffi::c_char
            },
        );
    }
    if r != 0 as int64_t {
        return r;
    }
    let mut z: *const toks_gen_spec = (&raw mut g as *mut toks_gen_spec)
        .offset(n.wrapping_sub(1 as uint32_t) as isize) as *mut toks_gen_spec;
    if n == 1 as uint32_t
        && (*z).kind == TOKS_GS_BYTELEVEL as ::core::ffi::c_int as uint32_t
    {
        (*cfg).pattern = if (*z).lit != 0 {
            (&raw const TOKS_PATTERNS as *const toks_pattern)
                .offset(0 as ::core::ffi::c_int as isize) as *const toks_pattern
        } else {
            ::core::ptr::null::<toks_pattern>()
        };
        return 0 as int64_t;
    }
    if n == 3 as uint32_t && !re[0 as ::core::ffi::c_int as usize].is_null()
        && !re[1 as ::core::ffi::c_int as usize].is_null()
        && g[0 as ::core::ffi::c_int as usize].beh
            == TOKS_GB_NEXT as ::core::ffi::c_int as uint32_t
        && g[0 as ::core::ffi::c_int as usize].inv == 0
        && g[1 as ::core::ffi::c_int as usize].beh
            == TOKS_GB_ISOLATED as ::core::ffi::c_int as uint32_t
        && (*z).kind == TOKS_GS_BYTELEVEL as ::core::ffi::c_int as uint32_t
        && (*z).lit == 0
        && toks_jstr(
            re[0 as ::core::ffi::c_int as usize],
            b"(?:\\r?\\n)+(?!\\r?\\n)\0" as *const u8 as *const ::core::ffi::c_char,
        ) != 0
        && toks_jstr(re[1 as ::core::ffi::c_int as usize], TOKS_PATTERN_P20.regex) != 0
    {
        (*cfg).pattern = &raw const TOKS_PATTERN_P20;
        return 0 as int64_t;
    }
    if n == 2 as uint32_t
        && g[0 as ::core::ffi::c_int as usize].kind
            == TOKS_GS_DIGITS as ::core::ffi::c_int as uint32_t
        && g[0 as ::core::ffi::c_int as usize].beh
            == TOKS_GB_ISOLATED as ::core::ffi::c_int as uint32_t
        && (*z).kind == TOKS_GS_BYTELEVEL as ::core::ffi::c_int as uint32_t
        && (*z).lit != 0
    {
        (*cfg).pattern = &raw const TOKS_PATTERN_DIGITS;
        return 0 as int64_t;
    }
    while k.wrapping_add(1 as uint32_t) < n && !re[k as usize].is_null()
        && (g[k as usize].beh == TOKS_GB_ISOLATED as ::core::ffi::c_int as uint32_t
            || n == 2 as uint32_t
                && g[k as usize].beh == TOKS_GB_REMOVED as ::core::ffi::c_int as uint32_t
                && g[k as usize].inv != 0)
        && (n != 4 as uint32_t
            || toks_jstr(re[k as usize], TOKS_DSV3_SPLITS[k as usize]) != 0)
    {
        k = k.wrapping_add(1);
    }
    if k.wrapping_add(1 as uint32_t) == n && n >= 2 as uint32_t && n <= 4 as uint32_t
        && (*z).kind == TOKS_GS_BYTELEVEL as ::core::ffi::c_int as uint32_t
        && (*z).lit == 0
    {
        if n == 4 as uint32_t {
            (*cfg).pattern = &raw const TOKS_PATTERN_DSV3;
            return 0 as int64_t;
        }
        if n == 3 as uint32_t
            && toks_jstr(
                re[0 as ::core::ffi::c_int as usize],
                TOKS_DSV3_SPLITS[0 as ::core::ffi::c_int as usize],
            ) != 0
            && toks_jstr(re[1 as ::core::ffi::c_int as usize], TOKS_PATTERN_P16.regex)
                != 0
        {
            (*cfg).pattern = &raw const TOKS_PATTERN_P16;
            return 0 as int64_t;
        }
        let mut i_0: uint32_t = 0 as uint32_t;
        while n == 2 as uint32_t && i_0 < TOKS_PATTERNS_N {
            if toks_jstr(
                re[0 as ::core::ffi::c_int as usize],
                TOKS_PATTERNS[i_0 as usize].regex,
            ) != 0
            {
                (*cfg).pattern = (&raw const TOKS_PATTERNS as *const toks_pattern)
                    .offset(i_0 as isize) as *const toks_pattern;
                return 0 as int64_t;
            }
            i_0 = i_0.wrapping_add(1);
        }
    }
    return toks_gen_compile(
        &raw mut g as *mut toks_gen_spec,
        n,
        ar,
        &raw mut (*cfg).gen,
        &raw mut (*cfg).gen_bytes,
        err,
    );
}
unsafe extern "C" fn bl_decoder(
    mut root: *const jv,
    mut cfg: *mut toks_config,
    mut err: *mut toks_err,
) -> int64_t {
    let mut s: steps = steps {
        o: [::core::ptr::null::<jv>(); 16],
        k: [0; 16],
        n: 0,
        present: 0,
    };
    let mut r: int64_t = read_steps(
        root,
        C_DEC as ::core::ffi::c_int as uint32_t,
        ((1 as ::core::ffi::c_ulonglong) << D_BYTELEVEL as ::core::ffi::c_int)
            as uint64_t,
        &raw mut s,
        err,
    );
    if r == 0 as int64_t && s.n != 1 as uint32_t {
        r = toks_fail(
            err,
            TOKS_E_UNSUPPORTED as int64_t,
            if s.n == 0 as uint32_t {
                b"decoder absent (byte-level BPE needs ByteLevel)\0" as *const u8
                    as *const ::core::ffi::c_char
            } else {
                b"decoder Sequence (byte-level BPE needs [ByteLevel])\0" as *const u8
                    as *const ::core::ffi::c_char
            },
        );
    }
    (*cfg).dec_byte_level = (r == 0 as int64_t) as ::core::ffi::c_int as uint8_t;
    return r;
}
unsafe extern "C" fn bl_alphabet(
    mut model: *const jv,
    mut cfg: *mut toks_config,
    mut vx: *const toks_sidx,
    mut err: *mut toks_err,
) -> int64_t {
    let mut seen: [uint8_t; 256] = [0; 256];
    let mut n_seen: uint32_t = 0 as uint32_t;
    let mut n_raw: uint32_t = 0 as uint32_t;
    memset(
        &raw mut seen as *mut uint8_t as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<[uint8_t; 256]>() as size_t,
    );
    let mut id: uint32_t = 0 as uint32_t;
    while id < (*cfg).n_vocab {
        let mut b: uint8_t = 0 as uint8_t;
        let mut m: int64_t = toks_alpha_bytes(
            *(*cfg).vocab.offset(id as isize),
            *(*cfg).vocab_len.offset(id as isize),
            ::core::ptr::null_mut::<uint8_t>(),
        );
        if m < 0 as int64_t {
            n_raw = n_raw.wrapping_add(1);
        } else if m == 1 as int64_t
            && toks_alpha_bytes(
                *(*cfg).vocab.offset(id as isize),
                *(*cfg).vocab_len.offset(id as isize),
                &raw mut b,
            ) == 1 as int64_t
            && seen[b as usize] as ::core::ffi::c_uint == 0 as ::core::ffi::c_uint
        {
            seen[b as usize] = 1 as uint8_t;
            n_seen = n_seen.wrapping_add(1);
        }
        id = id.wrapping_add(1);
    }
    (*cfg).n_vocab_raw = n_raw;
    if n_seen == 256 as uint32_t {
        return 0 as int64_t;
    }
    let mut unk: *const jv = toks_jv_get(
        model,
        b"unk_token\0" as *const u8 as *const ::core::ffi::c_char,
    );
    if toks_jtrue(
        toks_jv_get(model, b"byte_fallback\0" as *const u8 as *const ::core::ffi::c_char),
    ) != 0
    {
        return toks_fail(
            err,
            TOKS_E_UNSUPPORTED as int64_t,
            b"model.vocab lacks byte-level alphabet chars (with byte_fallback)\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if toks_jnull(unk) == 0 {
        let mut id_0: int64_t = if (*unk).type_0 as ::core::ffi::c_int
            == JV_STR as ::core::ffi::c_int
        {
            toks_sidx_find(
                vx,
                (*unk).s,
                (*unk).s_len,
                ::core::ptr::null::<uint8_t>(),
                0 as uint32_t,
            )
        } else {
            -(1 as ::core::ffi::c_int) as int64_t
        };
        if id_0 < 0 as int64_t {
            return toks_fail(
                err,
                TOKS_E_UNSUPPORTED as int64_t,
                b"model.unk_token not in model.vocab (a byte-level vocab lacking alphabet chars)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
        (*cfg).drop_unk = (id_0 as uint32_t).wrapping_add(1 as uint32_t);
        (*cfg).drop_fuse = toks_jtrue(
            toks_jv_get(model, b"fuse_unk\0" as *const u8 as *const ::core::ffi::c_char),
        ) as uint32_t;
    }
    if (*cfg).ignore_merges as ::core::ffi::c_uint != 0 as ::core::ffi::c_uint {
        return toks_fail(
            err,
            TOKS_E_UNSUPPORTED as int64_t,
            b"model.vocab lacks byte-level alphabet chars (under ignore_merges)\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
    }
    let mut b_0: uint32_t = 0 as uint32_t;
    while b_0 < 256 as uint32_t {
        if seen[b_0 as usize] as ::core::ffi::c_uint == 0 as ::core::ffi::c_uint {
            (*cfg).drop[(b_0 >> 3 as ::core::ffi::c_int) as usize] = ((*cfg)
                .drop[(b_0 >> 3 as ::core::ffi::c_int) as usize] as ::core::ffi::c_uint
                | (1 as ::core::ffi::c_uint) << (b_0 & 7 as uint32_t)) as uint8_t;
        }
        b_0 = b_0.wrapping_add(1);
    }
    (*cfg).has_drop = 1 as ::core::ffi::c_uint as uint32_t;
    return 0 as int64_t;
}
unsafe extern "C" fn parse_bytelevel(
    mut root: *const jv,
    mut model: *const jv,
    mut ar: *mut toks_arena,
    mut cfg: *mut toks_config,
    mut err: *mut toks_err,
) -> int64_t {
    let mut fl: uint32_t = 0 as uint32_t;
    let mut r: int64_t = read_bpe(model, &raw mut fl, err);
    if r == 0 as int64_t {
        r = bl_normalizer(root, cfg, err);
    }
    if r == 0 as int64_t {
        r = bl_pretok(root, cfg, ar, err);
    }
    if r == 0 as int64_t {
        r = bl_decoder(root, cfg, err);
    }
    if r != 0 as int64_t {
        return r;
    }
    (*cfg).ignore_merges = (fl & TOKS_SPM_IGNORE_MERGES as uint32_t != 0 as uint32_t)
        as ::core::ffi::c_int as uint8_t;
    let mut vx: toks_sidx = toks_sidx {
        s: ::core::ptr::null::<*const uint8_t>(),
        len: ::core::ptr::null::<uint32_t>(),
        slot: ::core::ptr::null_mut::<uint32_t>(),
        mask: 0,
    };
    let mut lro: [*const uint32_t; 3] = [::core::ptr::null::<uint32_t>(); 3];
    r = read_vocab(
        model,
        ar,
        0 as ::core::ffi::c_int,
        &raw mut vx,
        &raw mut (*cfg).n_vocab,
        &raw mut (*cfg).n_strings,
        err,
    );
    if r == 0 as int64_t {
        (*cfg).vocab = vx.s;
        (*cfg).vocab_len = vx.len;
        r = bl_alphabet(model, cfg, &raw mut vx, err);
    }
    if r == 0 as int64_t {
        r = read_merges(
            model,
            ar,
            &raw mut vx,
            &raw mut lro as *mut *const uint32_t,
            &raw mut (*cfg).n_merges,
            err,
        );
    }
    let mut i: uint32_t = 0 as uint32_t;
    while r == 0 as int64_t && (*cfg).drop_unk != 0 as uint32_t && i < (*cfg).n_merges {
        if (*lro[0 as ::core::ffi::c_int as usize].offset(i as isize))
            .wrapping_add(1 as uint32_t) == (*cfg).drop_unk
            || (*lro[1 as ::core::ffi::c_int as usize].offset(i as isize))
                .wrapping_add(1 as uint32_t) == (*cfg).drop_unk
        {
            r = toks_fail(
                err,
                TOKS_E_UNSUPPORTED as int64_t,
                b"a merge of model.unk_token (a byte-level vocab lacking alphabet chars)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
        i = i.wrapping_add(1);
    }
    if r == 0 as int64_t {
        r = read_added(root, ar, cfg, &raw mut vx, err);
    }
    if r != 0 as int64_t {
        return r;
    }
    (*cfg).m_left_id = lro[0 as ::core::ffi::c_int as usize];
    (*cfg).m_right_id = lro[1 as ::core::ffi::c_int as usize];
    (*cfg).m_out_id = lro[2 as ::core::ffi::c_int as usize];
    let mut i_0: uint32_t = 0 as uint32_t;
    while i_0 < (*cfg).n_added
        && (*cfg).nfc as ::core::ffi::c_uint != 0 as ::core::ffi::c_uint
    {
        let mut run_end: uint64_t = 0 as uint64_t;
        if (*(*cfg).added.offset(i_0 as isize)).normalized as ::core::ffi::c_uint
            != 0 as ::core::ffi::c_uint
            && toks_nfc_scan(
                (*cfg).nfc as uint32_t,
                (*(*cfg).added.offset(i_0 as isize)).content,
                (*(*cfg).added.offset(i_0 as isize)).len as uint64_t,
                0 as uint64_t,
                &raw mut run_end,
            ) != (*(*cfg).added.offset(i_0 as isize)).len as uint64_t
        {
            return toks_fail(
                err,
                TOKS_E_UNSUPPORTED as int64_t,
                b"added token normalized=true whose content the normalizer changes\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
        i_0 = i_0.wrapping_add(1);
    }
    return read_post_processor(root, ar, cfg, err);
}
unsafe extern "C" fn set_str(
    mut d: *mut toks_spm_str,
    mut v: *const jv,
) -> ::core::ffi::c_int {
    if v.is_null() || (*v).type_0 as ::core::ffi::c_int != JV_STR as ::core::ffi::c_int
        || (*v).s_len > TOKS_SPM_MAX_STR as uint32_t
    {
        return -(1 as ::core::ffi::c_int);
    }
    memset(
        d as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<toks_spm_str>() as size_t,
    );
    if (*v).s_len != 0 as uint32_t {
        memcpy(
            &raw mut (*d).b as *mut uint8_t as *mut ::core::ffi::c_void,
            (*v).s as *const ::core::ffi::c_void,
            (*v).s_len as size_t,
        );
    }
    (*d).n = (*v).s_len;
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn one_char(mut s: *const toks_spm_str) -> ::core::ffi::c_int {
    return ((*s).n >= 1 as uint32_t
        && toks_utf8_len(&raw const (*s).b as *const uint8_t, (*s).n as uint64_t)
            == (*s).n) as ::core::ffi::c_int;
}
unsafe extern "C" fn spm_norm_step(
    mut n: *const jv,
    mut k: uint32_t,
    mut tx: *mut toks_spm_text,
    mut err: *mut toks_err,
) -> int64_t {
    let mut ps: *const jv = ::core::ptr::null::<jv>();
    let mut rx: *const jv = ::core::ptr::null::<jv>();
    let mut con: *const jv = ::core::ptr::null::<jv>();
    if (*tx).n_norm >= TOKS_SPM_MAX_OPS as uint32_t {
        return toks_fail(
            err,
            TOKS_E_UNSUPPORTED as int64_t,
            b"normalizer: more than 8 steps\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    let fresh7 = (*tx).n_norm;
    (*tx).n_norm = (*tx).n_norm.wrapping_add(1);
    let mut op: *mut toks_spm_op = (&raw mut (*tx).norm as *mut toks_spm_op)
        .offset(fresh7 as isize) as *mut toks_spm_op;
    memset(
        op as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<toks_spm_op>() as size_t,
    );
    if k == N_PREPEND as ::core::ffi::c_int as uint32_t {
        (*op).kind = TOKS_SPM_N_PREPEND as ::core::ffi::c_int as uint32_t;
        return if set_str(
            &raw mut (*op).a,
            toks_jv_get(n, b"prepend\0" as *const u8 as *const ::core::ffi::c_char),
        ) != 0 as ::core::ffi::c_int
        {
            toks_fail(
                err,
                TOKS_E_UNSUPPORTED as int64_t,
                b"normalizer Prepend string\0" as *const u8 as *const ::core::ffi::c_char,
            )
        } else {
            0 as int64_t
        };
    }
    replace_of(n, &raw mut ps, &raw mut rx, &raw mut con);
    if !rx.is_null() {
        return toks_fail(
            err,
            TOKS_E_UNSUPPORTED as int64_t,
            b"normalizer Replace Regex\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if set_str(&raw mut (*op).a, ps) != 0 as ::core::ffi::c_int
        || (*op).a.n == 0 as uint32_t
    {
        return toks_fail(
            err,
            TOKS_E_UNSUPPORTED as int64_t,
            b"normalizer Replace pattern\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if set_str(&raw mut (*op).b, con) != 0 as ::core::ffi::c_int {
        return toks_fail(
            err,
            TOKS_E_UNSUPPORTED as int64_t,
            b"normalizer Replace content\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    (*op).kind = TOKS_SPM_N_REPLACE as ::core::ffi::c_int as uint32_t;
    return 0 as int64_t;
}
unsafe extern "C" fn meta_op(mut o: *const toks_spm_op) -> uint32_t {
    if (*o).kind == TOKS_SPM_N_PREPEND as ::core::ffi::c_int as uint32_t {
        return if (*o).a.n == 3 as uint32_t
            && toks_meta_at(
                &raw const (*o).a.b as *const uint8_t,
                3 as uint64_t,
                0 as uint64_t,
            ) != 0
        {
            2 as uint32_t
        } else {
            0 as uint32_t
        };
    }
    return ((*o).a.n == 1 as uint32_t
        && (*o).a.b[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_uint
            == 0x20 as ::core::ffi::c_uint && (*o).b.n == 3 as uint32_t
        && toks_meta_at(
            &raw const (*o).b.b as *const uint8_t,
            3 as uint64_t,
            0 as uint64_t,
        ) != 0) as ::core::ffi::c_int as uint32_t;
}
unsafe extern "C" fn split_is_noop(
    mut sp: *const jv,
    mut tx: *const toks_spm_text,
) -> ::core::ffi::c_int {
    let mut pat: *const jv = toks_jv_get(
        sp,
        b"pattern\0" as *const u8 as *const ::core::ffi::c_char,
    );
    let mut inv: *const jv = toks_jv_get(
        sp,
        b"invert\0" as *const u8 as *const ::core::ffi::c_char,
    );
    let mut beh: *const jv = toks_jv_get(
        sp,
        b"behavior\0" as *const u8 as *const ::core::ffi::c_char,
    );
    let mut p: toks_spm_str = toks_spm_str { b: [0; 16], n: 0 };
    if set_str(
        &raw mut p,
        (if !pat.is_null()
            && (*pat).type_0 as ::core::ffi::c_int == JV_OBJ as ::core::ffi::c_int
        {
            toks_jv_get(pat, b"String\0" as *const u8 as *const ::core::ffi::c_char)
        } else {
            ::core::ptr::null_mut::<jv>()
        }),
    ) != 0 as ::core::ffi::c_int || one_char(&raw mut p) == 0 || beh.is_null()
        || (*beh).type_0 as ::core::ffi::c_int != JV_STR as ::core::ffi::c_int
        || (*tx).n_norm == 0 as uint32_t
        || toks_jnull(inv) == 0 && !(toks_jbool(inv) != 0 && (*inv).num == 0 as int64_t)
    {
        return 0 as ::core::ffi::c_int;
    }
    let mut last: *const toks_spm_op = (&raw const (*tx).norm as *const toks_spm_op)
        .offset((*tx).n_norm.wrapping_sub(1 as uint32_t) as isize) as *const toks_spm_op;
    if (*last).kind != TOKS_SPM_N_REPLACE as ::core::ffi::c_int as uint32_t
        || (*last).a.n != p.n
        || memcmp(
            &raw const (*last).a.b as *const uint8_t as *const ::core::ffi::c_void,
            &raw mut p.b as *mut uint8_t as *const ::core::ffi::c_void,
            p.n as size_t,
        ) != 0 as ::core::ffi::c_int
    {
        return 0 as ::core::ffi::c_int;
    }
    let mut i: uint32_t = 0 as uint32_t;
    while i.wrapping_add(p.n) <= (*last).b.n {
        if memcmp(
            (&raw const (*last).b.b as *const uint8_t).offset(i as isize)
                as *const ::core::ffi::c_void,
            &raw mut p.b as *mut uint8_t as *const ::core::ffi::c_void,
            p.n as size_t,
        ) == 0 as ::core::ffi::c_int
        {
            return 0 as ::core::ffi::c_int;
        }
        i = i.wrapping_add(1);
    }
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn read_meta(
    mut o: *const jv,
    mut repl: *mut toks_spm_str,
    mut scheme: *mut uint32_t,
    mut split: *mut uint32_t,
    mut err: *mut toks_err,
) -> int64_t {
    let mut sp: ::core::ffi::c_int = 0;
    if set_str(
        repl,
        toks_jv_get(o, b"replacement\0" as *const u8 as *const ::core::ffi::c_char),
    ) != 0 as ::core::ffi::c_int || one_char(repl) == 0
    {
        return toks_fail(
            err,
            TOKS_E_FORMAT as int64_t,
            b"Metaspace replacement\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if jflag(
        o,
        b"split\0" as *const u8 as *const ::core::ffi::c_char,
        1 as ::core::ffi::c_int,
        &raw mut sp,
    ) != 0 as ::core::ffi::c_int
    {
        return toks_fail(
            err,
            TOKS_E_FORMAT as int64_t,
            b"Metaspace split\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    *split = sp as uint32_t;
    return meta_scheme(o, scheme, err);
}
unsafe extern "C" fn spm_pretok_step(
    mut pt: *const jv,
    mut k: uint32_t,
    mut tx: *mut toks_spm_text,
    mut err: *mut toks_err,
) -> int64_t {
    if k == P_SPLIT as ::core::ffi::c_int as uint32_t {
        return if (*tx).metaspace != 0 || split_is_noop(pt, tx) == 0 {
            toks_fail(
                err,
                TOKS_E_UNSUPPORTED as int64_t,
                b"pre_tokenizer Split (one that can cut)\0" as *const u8
                    as *const ::core::ffi::c_char,
            )
        } else {
            0 as int64_t
        };
    }
    if (*tx).metaspace != 0 {
        return toks_fail(
            err,
            TOKS_E_UNSUPPORTED as int64_t,
            b"pre_tokenizer: two Metaspace\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    let mut r: int64_t = read_meta(
        pt,
        &raw mut (*tx).ms_repl,
        &raw mut (*tx).ms_scheme,
        &raw mut (*tx).ms_split,
        err,
    );
    (*tx).metaspace = 1 as ::core::ffi::c_uint as uint32_t;
    if r == 0 as int64_t
        && (*tx).ms_scheme == TOKS_SPM_PS_FIRST as ::core::ffi::c_int as uint32_t
        && (*tx).n_norm != 0 as uint32_t
    {
        r = toks_fail(
            err,
            TOKS_E_UNSUPPORTED as int64_t,
            b"Metaspace prepend_scheme first after a normalizer\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    return r;
}
unsafe extern "C" fn clear_of_byte_tokens(
    mut x: *const toks_spm_str,
) -> ::core::ffi::c_int {
    let mut i: uint32_t = 0 as uint32_t;
    while i < (*x).n {
        let mut c: uint8_t = (*x).b[i as usize];
        if c as ::core::ffi::c_int == '<' as i32 || c as ::core::ffi::c_int == '>' as i32
            || c as ::core::ffi::c_int == 'x' as i32
            || c as ::core::ffi::c_int == '+' as i32
            || c as ::core::ffi::c_int >= '0' as i32
                && c as ::core::ffi::c_int <= '9' as i32
            || c as ::core::ffi::c_int >= 'A' as i32
                && c as ::core::ffi::c_int <= 'F' as i32
            || c as ::core::ffi::c_int >= 'a' as i32
                && c as ::core::ffi::c_int <= 'f' as i32
        {
            return 0 as ::core::ffi::c_int;
        }
        i = i.wrapping_add(1);
    }
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn spm_dec_step(
    mut d: *const jv,
    mut k: uint32_t,
    mut sc: *mut toks_spm_config,
    mut err: *mut toks_err,
) -> int64_t {
    let mut ps: *const jv = ::core::ptr::null::<jv>();
    let mut rx: *const jv = ::core::ptr::null::<jv>();
    let mut con: *const jv = ::core::ptr::null::<jv>();
    let mut split: uint32_t = 0;
    if (*sc).n_dec >= TOKS_SPM_MAX_OPS as uint32_t {
        return toks_fail(
            err,
            TOKS_E_UNSUPPORTED as int64_t,
            b"decoder: more than 8 steps\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    let fresh6 = (*sc).n_dec;
    (*sc).n_dec = (*sc).n_dec.wrapping_add(1);
    let mut op: *mut toks_spm_op = (&raw mut (*sc).dec as *mut toks_spm_op)
        .offset(fresh6 as isize) as *mut toks_spm_op;
    memset(
        op as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<toks_spm_op>() as size_t,
    );
    (*op).kind = (if k == D_FUSE as ::core::ffi::c_int as uint32_t {
        TOKS_SPM_D_FUSE as ::core::ffi::c_int
    } else {
        TOKS_SPM_D_BYTE_FALLBACK as ::core::ffi::c_int
    }) as uint32_t;
    if k == D_REPLACE as ::core::ffi::c_int as uint32_t {
        replace_of(d, &raw mut ps, &raw mut rx, &raw mut con);
        if set_str(&raw mut (*op).a, ps) != 0 as ::core::ffi::c_int
            || (*op).a.n == 0 as uint32_t
        {
            return toks_fail(
                err,
                TOKS_E_UNSUPPORTED as int64_t,
                b"decoder Replace pattern\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        if set_str(&raw mut (*op).b, con) != 0 as ::core::ffi::c_int {
            return toks_fail(
                err,
                TOKS_E_UNSUPPORTED as int64_t,
                b"decoder Replace content\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        if clear_of_byte_tokens(&raw mut (*op).a) == 0
            || clear_of_byte_tokens(&raw mut (*op).b) == 0
        {
            return toks_fail(
                err,
                TOKS_E_UNSUPPORTED as int64_t,
                b"decoder Replace touching byte-token chars\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        (*op).kind = TOKS_SPM_D_REPLACE as ::core::ffi::c_int as uint32_t;
    } else if k == D_STRIP as ::core::ffi::c_int as uint32_t {
        let mut st: *const jv = toks_jv_get(
            d,
            b"start\0" as *const u8 as *const ::core::ffi::c_char,
        );
        let mut sp: *const jv = toks_jv_get(
            d,
            b"stop\0" as *const u8 as *const ::core::ffi::c_char,
        );
        if set_str(
            &raw mut (*op).a,
            toks_jv_get(d, b"content\0" as *const u8 as *const ::core::ffi::c_char),
        ) != 0 as ::core::ffi::c_int || one_char(&raw mut (*op).a) == 0
            || toks_juint(st, INT64_MAX as uint64_t) == 0
            || toks_juint(sp, INT64_MAX as uint64_t) == 0
        {
            return toks_fail(
                err,
                TOKS_E_FORMAT as int64_t,
                b"decoder Strip fields\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        if (*sp).num != 0 as int64_t {
            return toks_fail(
                err,
                TOKS_E_UNSUPPORTED as int64_t,
                b"decoder Strip stop > 0\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        if (*st).num > 0xffff as int64_t {
            return toks_fail(
                err,
                TOKS_E_UNSUPPORTED as int64_t,
                b"decoder Strip start > 65535\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        (*op).start = (*st).num as uint32_t;
        (*op).kind = TOKS_SPM_D_STRIP as ::core::ffi::c_int as uint32_t;
    } else if k == D_METASPACE as ::core::ffi::c_int as uint32_t {
        let mut r: int64_t = read_meta(
            d,
            &raw mut (*op).a,
            &raw mut (*op).scheme,
            &raw mut split,
            err,
        );
        if r != 0 as int64_t {
            return r;
        }
        if clear_of_byte_tokens(&raw mut (*op).a) == 0 {
            return toks_fail(
                err,
                TOKS_E_UNSUPPORTED as int64_t,
                b"decoder Metaspace replacement is a byte-token char\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        (*op).kind = TOKS_SPM_D_METASPACE as ::core::ffi::c_int as uint32_t;
    }
    return 0 as int64_t;
}
unsafe extern "C" fn spm_dec_order(
    mut sc: *const toks_spm_config,
    mut err: *mut toks_err,
) -> int64_t {
    static mut WHY: *const ::core::ffi::c_char = b"decoder order (sentencepiece-style bpe: one Replace or Metaspace, ByteFallback, Fuse, Strip)\0"
        as *const u8 as *const ::core::ffi::c_char;
    let mut stage: uint32_t = 0 as uint32_t;
    let mut per_token: uint32_t = 0 as uint32_t;
    let mut i: uint32_t = 0 as uint32_t;
    while i < (*sc).n_dec {
        let mut k: uint32_t = (*sc).dec[i as usize].kind;
        let mut want: uint32_t = if k
            == TOKS_SPM_D_BYTE_FALLBACK as ::core::ffi::c_int as uint32_t
        {
            1 as uint32_t
        } else if k == TOKS_SPM_D_FUSE as ::core::ffi::c_int as uint32_t {
            2 as uint32_t
        } else if k == TOKS_SPM_D_STRIP as ::core::ffi::c_int as uint32_t {
            3 as uint32_t
        } else {
            0 as uint32_t
        };
        if if want == 0 as uint32_t {
            (stage != 0 as uint32_t || per_token != 0 as uint32_t) as ::core::ffi::c_int
        } else if want == 3 as uint32_t {
            (stage != 2 as uint32_t) as ::core::ffi::c_int
        } else {
            (stage >= want) as ::core::ffi::c_int
        } != 0
        {
            return toks_fail(err, TOKS_E_UNSUPPORTED as int64_t, WHY);
        }
        if want == 0 as uint32_t {
            per_token = 1 as uint32_t;
        } else {
            stage = want;
        }
        i = i.wrapping_add(1);
    }
    return 0 as int64_t;
}
unsafe extern "C" fn is_spm(
    mut root: *const jv,
    mut model: *const jv,
) -> ::core::ffi::c_int {
    let mut type_0: *const jv = toks_jv_get(
        model,
        b"type\0" as *const u8 as *const ::core::ffi::c_char,
    );
    if !type_0.is_null()
        && toks_jstr(type_0, b"BPE\0" as *const u8 as *const ::core::ffi::c_char) == 0
    {
        return 0 as ::core::ffi::c_int;
    }
    return (has_type(
        toks_jv_get(root, b"normalizer\0" as *const u8 as *const ::core::ffi::c_char),
        b"normalizers\0" as *const u8 as *const ::core::ffi::c_char,
        b"ByteLevel\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
        && has_type(
            toks_jv_get(
                root,
                b"pre_tokenizer\0" as *const u8 as *const ::core::ffi::c_char,
            ),
            b"pretokenizers\0" as *const u8 as *const ::core::ffi::c_char,
            b"ByteLevel\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0
        && has_type(
            toks_jv_get(root, b"decoder\0" as *const u8 as *const ::core::ffi::c_char),
            b"decoders\0" as *const u8 as *const ::core::ffi::c_char,
            b"ByteLevel\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0
        && has_type(
            toks_jv_get(
                root,
                b"post_processor\0" as *const u8 as *const ::core::ffi::c_char,
            ),
            b"processors\0" as *const u8 as *const ::core::ffi::c_char,
            b"ByteLevel\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0) as ::core::ffi::c_int;
}
unsafe extern "C" fn parse_spm(
    mut root: *const jv,
    mut model: *const jv,
    mut ar: *mut toks_arena,
    mut cfg: *mut toks_config,
    mut err: *mut toks_err,
) -> int64_t {
    let mut sc: *mut toks_spm_config = toks_ar_alloc(
        ar,
        ::core::mem::size_of::<toks_spm_config>() as uint64_t,
        8 as uint64_t,
    ) as *mut toks_spm_config;
    if sc.is_null() {
        return toks_fail(
            err,
            TOKS_E_NOMEM as int64_t,
            b"spm config\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    memset(
        sc as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<toks_spm_config>() as size_t,
    );
    let mut s: steps = steps {
        o: [::core::ptr::null::<jv>(); 16],
        k: [0; 16],
        n: 0,
        present: 0,
    };
    let mut r: int64_t = read_steps(
        root,
        C_NORM as ::core::ffi::c_int as uint32_t,
        ((1 as ::core::ffi::c_ulonglong) << N_PREPEND as ::core::ffi::c_int
            | (1 as ::core::ffi::c_ulonglong) << N_REPLACE as ::core::ffi::c_int)
            as uint64_t,
        &raw mut s,
        err,
    );
    let mut i: uint32_t = 0 as uint32_t;
    while r == 0 as int64_t && i < s.n {
        r = spm_norm_step(
            s.o[i as usize],
            s.k[i as usize] as uint32_t,
            &raw mut (*sc).text,
            err,
        );
        i = i.wrapping_add(1);
    }
    if r == 0 as int64_t {
        r = read_steps(
            root,
            C_PRE as ::core::ffi::c_int as uint32_t,
            ((1 as ::core::ffi::c_ulonglong) << P_METASPACE as ::core::ffi::c_int
                | (1 as ::core::ffi::c_ulonglong) << P_SPLIT as ::core::ffi::c_int)
                as uint64_t,
            &raw mut s,
            err,
        );
    }
    let mut i_0: uint32_t = 0 as uint32_t;
    while r == 0 as int64_t && i_0 < s.n {
        r = spm_pretok_step(
            s.o[i_0 as usize],
            s.k[i_0 as usize] as uint32_t,
            &raw mut (*sc).text,
            err,
        );
        i_0 = i_0.wrapping_add(1);
    }
    if r == 0 as int64_t {
        r = read_steps(
            root,
            C_DEC as ::core::ffi::c_int as uint32_t,
            ((1 as ::core::ffi::c_ulonglong) << D_REPLACE as ::core::ffi::c_int
                | (1 as ::core::ffi::c_ulonglong) << D_BYTEFALLBACK as ::core::ffi::c_int
                | (1 as ::core::ffi::c_ulonglong) << D_FUSE as ::core::ffi::c_int
                | (1 as ::core::ffi::c_ulonglong) << D_STRIP as ::core::ffi::c_int
                | (1 as ::core::ffi::c_ulonglong) << D_METASPACE as ::core::ffi::c_int)
                as uint64_t,
            &raw mut s,
            err,
        );
    }
    (*sc).has_decoder = s.present;
    let mut i_1: uint32_t = 0 as uint32_t;
    while r == 0 as int64_t && i_1 < s.n {
        r = spm_dec_step(s.o[i_1 as usize], s.k[i_1 as usize] as uint32_t, sc, err);
        i_1 = i_1.wrapping_add(1);
    }
    if r == 0 as int64_t {
        r = spm_dec_order(sc, err);
    }
    if r == 0 as int64_t {
        r = read_bpe(model, &raw mut (*sc).sflags, err);
    }
    let mut vx: toks_sidx = toks_sidx {
        s: ::core::ptr::null::<*const uint8_t>(),
        len: ::core::ptr::null::<uint32_t>(),
        slot: ::core::ptr::null_mut::<uint32_t>(),
        mask: 0,
    };
    let mut lro: [*const uint32_t; 3] = [::core::ptr::null::<uint32_t>(); 3];
    if r == 0 as int64_t {
        r = read_vocab(
            model,
            ar,
            1 as ::core::ffi::c_int,
            &raw mut vx,
            &raw mut (*sc).n_ids,
            &raw mut (*sc).n_strings,
            err,
        );
    }
    if r == 0 as int64_t {
        r = read_merges(
            model,
            ar,
            &raw mut vx,
            &raw mut lro as *mut *const uint32_t,
            &raw mut (*sc).n_merges,
            err,
        );
    }
    if r != 0 as int64_t {
        return r;
    }
    (*sc).vocab = vx.s;
    (*sc).vocab_len = vx.len;
    (*sc).vslot = vx.slot;
    (*sc).vmask = vx.mask;
    (*sc).m_left = lro[0 as ::core::ffi::c_int as usize];
    (*sc).m_right = lro[1 as ::core::ffi::c_int as usize];
    (*sc).m_out = lro[2 as ::core::ffi::c_int as usize];
    (*sc).unk_id = TOKS_SPM_NONE as uint32_t;
    let mut unk: *const jv = toks_jv_get(
        model,
        b"unk_token\0" as *const u8 as *const ::core::ffi::c_char,
    );
    if toks_jnull(unk) == 0 {
        if (*unk).type_0 as ::core::ffi::c_int != JV_STR as ::core::ffi::c_int {
            return toks_fail(
                err,
                TOKS_E_FORMAT as int64_t,
                b"model unk_token\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        (*sc).unk_id = spm_vocab_find(sc, (*unk).s, (*unk).s_len);
        if (*sc).unk_id == TOKS_SPM_NONE as uint32_t {
            (*sc).sflags = ((*sc).sflags as ::core::ffi::c_uint | TOKS_SPM_UNK_ERROR)
                as uint32_t;
        }
    }
    (*cfg).algo = TOKS_ALGO_BPE_SPM as uint32_t;
    (*cfg).spm = sc;
    (*cfg).vocab = (*sc).vocab;
    (*cfg).vocab_len = (*sc).vocab_len;
    (*cfg).n_vocab = (*sc).n_ids;
    (*cfg).n_strings = (*sc).n_strings;
    (*cfg).n_merges = (*sc).n_merges;
    (*cfg).ignore_merges = ((*sc).sflags & TOKS_SPM_IGNORE_MERGES as uint32_t
        != 0 as uint32_t) as ::core::ffi::c_int as uint8_t;
    r = read_added(root, ar, cfg, ::core::ptr::null::<toks_sidx>(), err);
    if r != 0 as int64_t {
        return r;
    }
    let mut nn: uint32_t = (*sc).text.n_norm;
    let mut sh: uint32_t = if nn == 0 as uint32_t {
        0 as uint32_t
    } else if nn == 1 as uint32_t {
        meta_op(
            (&raw mut (*sc).text.norm as *mut toks_spm_op)
                .offset(0 as ::core::ffi::c_int as isize) as *mut toks_spm_op,
        )
    } else if nn == 2 as uint32_t {
        meta_op(
                (&raw mut (*sc).text.norm as *mut toks_spm_op)
                    .offset(0 as ::core::ffi::c_int as isize) as *mut toks_spm_op,
            )
            .wrapping_mul(4 as uint32_t)
            .wrapping_add(
                meta_op(
                    (&raw mut (*sc).text.norm as *mut toks_spm_op)
                        .offset(1 as ::core::ffi::c_int as isize) as *mut toks_spm_op,
                ),
            )
    } else {
        15 as uint32_t
    };
    let mut i_2: uint32_t = 0 as uint32_t;
    while i_2 < (*cfg).n_added && nn != 0 as uint32_t {
        let mut a: *mut toks_cfg_added = (*cfg).added.offset(i_2 as isize)
            as *mut toks_cfg_added;
        let mut bad: ::core::ffi::c_int = (sh != 1 as uint32_t && sh != 6 as uint32_t
            && sh != 9 as uint32_t || (*sc).text.metaspace != 0 as uint32_t
            || (*a).lstrip as ::core::ffi::c_uint != 0 as ::core::ffi::c_uint
            || (*a).rstrip as ::core::ffi::c_uint != 0 as ::core::ffi::c_uint
            || (*a).single_word as ::core::ffi::c_uint != 0 as ::core::ffi::c_uint)
            as ::core::ffi::c_int;
        let mut j: uint32_t = 0 as uint32_t;
        while j < (*a).len {
            bad
                |= (*(*a).content.offset(j as isize) as ::core::ffi::c_uint
                    == 0x20 as ::core::ffi::c_uint
                    || toks_meta_at((*a).content, (*a).len as uint64_t, j as uint64_t)
                        != 0) as ::core::ffi::c_int;
            j = j.wrapping_add(1);
        }
        if (*a).normalized as ::core::ffi::c_uint != 0 as ::core::ffi::c_uint && bad != 0
        {
            return toks_fail(
                err,
                TOKS_E_UNSUPPORTED as int64_t,
                b"added token normalized=true under a normalizer (phase 1 on the folded text: m2)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
        (*a).pfx = ((*a).normalized as ::core::ffi::c_uint != 0 as ::core::ffi::c_uint
            && nn == 2 as uint32_t) as ::core::ffi::c_int as uint8_t;
        i_2 = i_2.wrapping_add(1);
    }
    return read_post_processor(root, ar, cfg, err);
}
unsafe extern "C" fn legacy_wordpiece(mut model: *const jv) -> ::core::ffi::c_int {
    let mut v: *const jv = toks_jv_get(
        model,
        b"vocab\0" as *const u8 as *const ::core::ffi::c_char,
    );
    return (toks_jv_get(model, b"type\0" as *const u8 as *const ::core::ffi::c_char)
        .is_null()
        && toks_jv_get(model, b"merges\0" as *const u8 as *const ::core::ffi::c_char)
            .is_null()
        && !toks_jv_get(model, b"unk_token\0" as *const u8 as *const ::core::ffi::c_char)
            .is_null()
        && !toks_jv_get(
                model,
                b"continuing_subword_prefix\0" as *const u8 as *const ::core::ffi::c_char,
            )
            .is_null()
        && !toks_jv_get(
                model,
                b"max_input_chars_per_word\0" as *const u8 as *const ::core::ffi::c_char,
            )
            .is_null() && !v.is_null()
        && (*v).type_0 as ::core::ffi::c_int == JV_OBJ as ::core::ffi::c_int)
        as ::core::ffi::c_int;
}
unsafe extern "C" fn wp_forms(
    mut cfg: *mut toks_config,
    mut ar: *mut toks_arena,
    mut err: *mut toks_err,
) -> int64_t {
    (*cfg).o.wp_win = 1 as uint32_t;
    let mut i: uint32_t = 0 as uint32_t;
    while i < (*cfg).n_added {
        let mut a: *mut toks_cfg_added = (*cfg).added.offset(i as isize)
            as *mut toks_cfg_added;
        (*a).form = (*a).content;
        (*a).form_len = (*a).len;
        if (*a).normalized as ::core::ffi::c_int != 0 && (*cfg).wp_flags != 0 as uint32_t
        {
            let mut room: uint64_t = (4 as uint64_t)
                .wrapping_mul((*a).len as uint64_t)
                .wrapping_add(16 as uint64_t);
            let mut f: *mut uint8_t = toks_ar_alloc(ar, room, 8 as uint64_t)
                as *mut uint8_t;
            if f.is_null() {
                return toks_fail(
                    err,
                    TOKS_E_NOMEM as int64_t,
                    b"added token forms\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            let mut n: int64_t = toks_norm(
                (*cfg).wp_flags,
                (*a).content,
                (*a).len as uint64_t,
                f,
                room,
            );
            if n < 0 as int64_t {
                return toks_fail(
                    err,
                    TOKS_E_UNSUPPORTED as int64_t,
                    b"added token form above its bound\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
            }
            if n == 0 as int64_t {
                return toks_fail(
                    err,
                    TOKS_E_UNSUPPORTED as int64_t,
                    b"added token whose normalized form is empty\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
            }
            if n > TOKS_MAX_ADDED_BYTES as int64_t {
                return toks_fail(
                    err,
                    TOKS_E_UNSUPPORTED as int64_t,
                    b"added token normalized form above 255 bytes\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
            }
            (*a).form = f;
            (*a).form_len = n as uint32_t;
        }
        let mut k: uint32_t = 0 as uint32_t;
        while (*a).normalized as ::core::ffi::c_int != 0 && k < (*a).form_len {
            let mut b: uint8_t = *(*a).form.offset(k as isize);
            if b as ::core::ffi::c_uint == 0x20 as ::core::ffi::c_uint
                || b as ::core::ffi::c_uint == 0x9 as ::core::ffi::c_uint
                || b as ::core::ffi::c_uint == 0xa as ::core::ffi::c_uint
                || b as ::core::ffi::c_uint == 0xd as ::core::ffi::c_uint
            {
                (*cfg).o.wp_win = 0 as uint32_t;
            }
            k = k.wrapping_add(1);
        }
        let mut j: uint32_t = 0 as uint32_t;
        while (*a).normalized as ::core::ffi::c_int != 0 && j < i {
            let mut b_0: *const toks_cfg_added = (*cfg).added.offset(j as isize)
                as *mut toks_cfg_added;
            if (*b_0).normalized as ::core::ffi::c_int != 0
                && (*b_0).form_len == (*a).form_len
                && memcmp(
                    (*b_0).form as *const ::core::ffi::c_void,
                    (*a).form as *const ::core::ffi::c_void,
                    (*a).form_len as size_t,
                ) == 0 as ::core::ffi::c_int
            {
                return toks_fail(
                    err,
                    TOKS_E_FORMAT as int64_t,
                    b"added tokens: a normalized form twice (hf refuses the file)\0"
                        as *const u8 as *const ::core::ffi::c_char,
                );
            }
            j = j.wrapping_add(1);
        }
        i = i.wrapping_add(1);
    }
    return 0 as int64_t;
}
unsafe extern "C" fn parse_wordpiece(
    mut root: *const jv,
    mut model: *const jv,
    mut ar: *mut toks_arena,
    mut cfg: *mut toks_config,
    mut err: *mut toks_err,
) -> int64_t {
    let mut unk: *const jv = toks_jv_get(
        model,
        b"unk_token\0" as *const u8 as *const ::core::ffi::c_char,
    );
    let mut pre: *const jv = toks_jv_get(
        model,
        b"continuing_subword_prefix\0" as *const u8 as *const ::core::ffi::c_char,
    );
    let mut maxc: *const jv = toks_jv_get(
        model,
        b"max_input_chars_per_word\0" as *const u8 as *const ::core::ffi::c_char,
    );
    if unk.is_null()
        || (*unk).type_0 as ::core::ffi::c_int != JV_STR as ::core::ffi::c_int
        || pre.is_null()
        || (*pre).type_0 as ::core::ffi::c_int != JV_STR as ::core::ffi::c_int
        || toks_juint(maxc, UINT64_MAX as uint64_t >> 1 as ::core::ffi::c_int) == 0
    {
        return toks_fail(
            err,
            TOKS_E_FORMAT as int64_t,
            b"WordPiece model fields\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if (*maxc).num as uint64_t > TOKS_WP_MAX_CHARS as uint64_t {
        return toks_fail(
            err,
            TOKS_E_UNSUPPORTED as int64_t,
            b"WordPiece max_input_chars_per_word above 1024\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    (*cfg).algo = TOKS_ALGO_WORDPIECE as uint32_t;
    (*cfg).wp_max_chars = (*maxc).num as uint32_t;
    (*cfg).wp_unk = (*unk).s;
    (*cfg).wp_unk_len = (*unk).s_len;
    (*cfg).wp_prefix = (*pre).s;
    (*cfg).wp_prefix_len = (*pre).s_len;
    let mut s: steps = steps {
        o: [::core::ptr::null::<jv>(); 16],
        k: [0; 16],
        n: 0,
        present: 0,
    };
    let mut r: int64_t = read_steps(
        root,
        C_NORM as ::core::ffi::c_int as uint32_t,
        ((1 as ::core::ffi::c_ulonglong) << N_BERT as ::core::ffi::c_int) as uint64_t,
        &raw mut s,
        err,
    );
    if r == 0 as int64_t && s.n > 1 as uint32_t {
        r = toks_fail(
            err,
            TOKS_E_UNSUPPORTED as int64_t,
            b"normalizer BertNormalizer twice\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    if r == 0 as int64_t && s.n == 1 as uint32_t {
        let mut ct: *const jv = toks_jv_get(
            s.o[0 as ::core::ffi::c_int as usize],
            b"clean_text\0" as *const u8 as *const ::core::ffi::c_char,
        );
        let mut hc: *const jv = toks_jv_get(
            s.o[0 as ::core::ffi::c_int as usize],
            b"handle_chinese_chars\0" as *const u8 as *const ::core::ffi::c_char,
        );
        let mut sa: *const jv = toks_jv_get(
            s.o[0 as ::core::ffi::c_int as usize],
            b"strip_accents\0" as *const u8 as *const ::core::ffi::c_char,
        );
        let mut lc: *const jv = toks_jv_get(
            s.o[0 as ::core::ffi::c_int as usize],
            b"lowercase\0" as *const u8 as *const ::core::ffi::c_char,
        );
        if toks_jbool(ct) == 0 || toks_jbool(hc) == 0 || toks_jbool(lc) == 0
            || !sa.is_null()
                && (*sa).type_0 as ::core::ffi::c_int != JV_BOOL as ::core::ffi::c_int
                && (*sa).type_0 as ::core::ffi::c_int != JV_NULL as ::core::ffi::c_int
        {
            return toks_fail(
                err,
                TOKS_E_FORMAT as int64_t,
                b"BertNormalizer fields\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        (*cfg).wp_flags = ((if (*ct).num != 0 {
            TOKS_WPF_CLEAN
        } else {
            0 as ::core::ffi::c_uint
        }) | (if (*hc).num != 0 { TOKS_WPF_CHINESE } else { 0 as ::core::ffi::c_uint })
            | (if (*lc).num != 0 { TOKS_WPF_LOWER } else { 0 as ::core::ffi::c_uint })
            | (if (if toks_jnull(sa) != 0 {
                ((*lc).num != 0 as int64_t) as ::core::ffi::c_int
            } else {
                ((*sa).num != 0 as int64_t) as ::core::ffi::c_int
            }) != 0
            {
                TOKS_WPF_STRIP
            } else {
                0 as ::core::ffi::c_uint
            })) as uint32_t;
    }
    if r == 0 as int64_t {
        r = read_steps(
            root,
            C_PRE as ::core::ffi::c_int as uint32_t,
            ((1 as ::core::ffi::c_ulonglong) << P_BERT as ::core::ffi::c_int)
                as uint64_t,
            &raw mut s,
            err,
        );
    }
    if r == 0 as int64_t && s.n != 1 as uint32_t {
        r = toks_fail(
            err,
            TOKS_E_UNSUPPORTED as int64_t,
            b"pre_tokenizer (wordpiece: BertPreTokenizer)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    if r == 0 as int64_t {
        r = read_steps(
            root,
            C_DEC as ::core::ffi::c_int as uint32_t,
            ((1 as ::core::ffi::c_ulonglong) << D_WORDPIECE as ::core::ffi::c_int)
                as uint64_t,
            &raw mut s,
            err,
        );
    }
    if r == 0 as int64_t
        && (s.n > 1 as uint32_t || s.n == 0 as uint32_t && s.present != 0)
    {
        r = toks_fail(
            err,
            TOKS_E_UNSUPPORTED as int64_t,
            b"decoder (wordpiece: WordPiece or none)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    if r == 0 as int64_t && s.n == 1 as uint32_t {
        let mut dp: *const jv = toks_jv_get(
            s.o[0 as ::core::ffi::c_int as usize],
            b"prefix\0" as *const u8 as *const ::core::ffi::c_char,
        );
        let mut cu: *const jv = toks_jv_get(
            s.o[0 as ::core::ffi::c_int as usize],
            b"cleanup\0" as *const u8 as *const ::core::ffi::c_char,
        );
        if dp.is_null()
            || (*dp).type_0 as ::core::ffi::c_int != JV_STR as ::core::ffi::c_int
            || toks_jbool(cu) == 0
        {
            return toks_fail(
                err,
                TOKS_E_FORMAT as int64_t,
                b"decoder WordPiece fields\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        if (*dp).s_len > TOKS_WP_DEC_PREFIX as uint32_t {
            return toks_fail(
                err,
                TOKS_E_UNSUPPORTED as int64_t,
                b"decoder WordPiece prefix above 16 bytes\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        (*cfg).o.dec_wordpiece = 1 as uint32_t;
        (*cfg).o.dec_cleanup = ((*cu).num != 0 as int64_t) as ::core::ffi::c_int
            as uint8_t as uint32_t;
        memcpy(
            &raw mut (*cfg).o.dec_prefix as *mut uint8_t as *mut ::core::ffi::c_void,
            (*dp).s as *const ::core::ffi::c_void,
            (*dp).s_len as size_t,
        );
        (*cfg).o.dec_prefix_len = (*dp).s_len;
    }
    if r == 0 as int64_t {
        r = read_trunc_pad(root, cfg, err);
    }
    let mut vx: toks_sidx = toks_sidx {
        s: ::core::ptr::null::<*const uint8_t>(),
        len: ::core::ptr::null::<uint32_t>(),
        slot: ::core::ptr::null_mut::<uint32_t>(),
        mask: 0,
    };
    if r == 0 as int64_t {
        r = read_vocab(
            model,
            ar,
            0 as ::core::ffi::c_int,
            &raw mut vx,
            &raw mut (*cfg).n_vocab,
            &raw mut (*cfg).n_strings,
            err,
        );
    }
    if r != 0 as int64_t {
        return r;
    }
    (*cfg).vocab = vx.s;
    (*cfg).vocab_len = vx.len;
    let mut u: int64_t = toks_sidx_find(
        &raw mut vx,
        (*cfg).wp_unk,
        (*cfg).wp_unk_len,
        ::core::ptr::null::<uint8_t>(),
        0 as uint32_t,
    );
    if u < 0 as int64_t {
        return toks_fail(
            err,
            TOKS_E_UNSUPPORTED as int64_t,
            b"WordPiece unk_token not in vocab\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    (*cfg).wp_unk_id = u as uint32_t;
    r = read_added(root, ar, cfg, &raw mut vx, err);
    if r == 0 as int64_t {
        r = wp_forms(cfg, ar, err);
    }
    if r == 0 as int64_t {
        r = read_post_processor(root, ar, cfg, err);
    }
    return if r != 0 as int64_t { r } else { trunc_check(cfg, err) };
}
unsafe extern "C" fn uni_model(mut model: *const jv) -> ::core::ffi::c_int {
    let mut t: *const jv = toks_jv_get(
        model,
        b"type\0" as *const u8 as *const ::core::ffi::c_char,
    );
    let mut v: *const jv = toks_jv_get(
        model,
        b"vocab\0" as *const u8 as *const ::core::ffi::c_char,
    );
    return if !t.is_null() {
        toks_jstr(t, b"Unigram\0" as *const u8 as *const ::core::ffi::c_char)
    } else {
        (!v.is_null()
            && (*v).type_0 as ::core::ffi::c_int == JV_ARR as ::core::ffi::c_int)
            as ::core::ffi::c_int
    };
}
unsafe extern "C" fn uni_norm_one(
    mut n: *const jv,
    mut k: uint32_t,
    mut src: *mut toks_uni_src,
    mut ar: *mut toks_arena,
    mut stage: *mut uint32_t,
    mut err: *mut toks_err,
) -> int64_t {
    let mut form: *mut uint8_t = &raw mut (*src).cfg.form;
    static mut NF: [uint8_t; 4] = [
        TOKS_NS_NFC as uint8_t,
        TOKS_NS_CANON as uint8_t,
        TOKS_NS_NFKC as uint8_t,
        TOKS_NS_NFKD as uint8_t,
    ];
    let mut st: uint32_t = ST_REPLACE as ::core::ffi::c_int as uint32_t;
    let mut str: *const jv = ::core::ptr::null::<jv>();
    let mut rx: *const jv = ::core::ptr::null::<jv>();
    let mut con: *const jv = ::core::ptr::null::<jv>();
    if k <= N_NFKD as ::core::ffi::c_int as uint32_t
        || k == N_STRIPACC as ::core::ffi::c_int as uint32_t
        || k == N_LOWER as ::core::ffi::c_int as uint32_t
    {
        *form = (*form as ::core::ffi::c_uint
            | (if k <= N_NFKD as ::core::ffi::c_int as uint32_t {
                NF[k as usize] as ::core::ffi::c_uint
            } else {
                (if k == N_LOWER as ::core::ffi::c_int as uint32_t {
                    TOKS_NS_LOWER
                } else {
                    TOKS_NS_STRIP_M
                })
            })) as uint8_t;
        st = (if k <= N_NFKD as ::core::ffi::c_int as uint32_t {
            ST_NF as ::core::ffi::c_int
        } else if k == N_LOWER as ::core::ffi::c_int as uint32_t {
            ST_LOW as ::core::ffi::c_int
        } else {
            ST_SA as ::core::ffi::c_int
        }) as uint32_t;
    } else if k == N_STRIP as ::core::ffi::c_int as uint32_t {
        let mut l: ::core::ffi::c_int = 0;
        let mut r: ::core::ffi::c_int = 0;
        if jflag(
            n,
            b"strip_left\0" as *const u8 as *const ::core::ffi::c_char,
            1 as ::core::ffi::c_int,
            &raw mut l,
        ) != 0 as ::core::ffi::c_int
            || jflag(
                n,
                b"strip_right\0" as *const u8 as *const ::core::ffi::c_char,
                1 as ::core::ffi::c_int,
                &raw mut r,
            ) != 0 as ::core::ffi::c_int
        {
            return toks_fail(
                err,
                TOKS_E_FORMAT as int64_t,
                b"normalizer Strip fields\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        (*src).cfg.strip_left = l as uint8_t;
        (*src).cfg.strip_right = r as uint8_t;
        st = ST_STRIP as ::core::ffi::c_int as uint32_t;
    } else if k == N_PRECOMPILED as ::core::ffi::c_int as uint32_t {
        let mut b: *const jv = toks_jv_get(
            n,
            b"precompiled_charsmap\0" as *const u8 as *const ::core::ffi::c_char,
        );
        if b.is_null()
            || (*b).type_0 as ::core::ffi::c_int != JV_STR as ::core::ffi::c_int
        {
            return toks_fail(
                err,
                TOKS_E_FORMAT as int64_t,
                b"normalizer Precompiled charsmap\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        let mut cap: uint64_t = ((*b).s_len as uint64_t)
            .wrapping_div(4 as uint64_t)
            .wrapping_mul(3 as uint64_t)
            .wrapping_add(3 as uint64_t);
        let mut blob: *mut uint8_t = toks_ar_alloc(
            ar,
            cap.wrapping_add(1 as uint64_t),
            8 as uint64_t,
        ) as *mut uint8_t;
        if blob.is_null() {
            return toks_fail(
                err,
                TOKS_E_NOMEM as int64_t,
                b"normalizer Precompiled charsmap\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        let mut m: int64_t = toks_b64_decode((*b).s, (*b).s_len as uint64_t, blob, cap);
        if m <= 0 as int64_t {
            return toks_fail(
                err,
                TOKS_E_FORMAT as int64_t,
                if m < 0 as int64_t {
                    b"normalizer Precompiled charsmap base64\0" as *const u8
                        as *const ::core::ffi::c_char
                } else {
                    b"normalizer Precompiled charsmap empty\0" as *const u8
                        as *const ::core::ffi::c_char
                },
            );
        }
        (*src).charsmap = blob;
        (*src).charsmap_len = m as uint64_t;
        (*src).cfg.has_charsmap = 1 as uint8_t;
        st = ST_PC as ::core::ffi::c_int as uint32_t;
    } else if replace_of(n, &raw mut str, &raw mut rx, &raw mut con) == 0
        || con.is_null()
        || (*con).type_0 as ::core::ffi::c_int != JV_STR as ::core::ffi::c_int
    {
        return toks_fail(
            err,
            TOKS_E_FORMAT as int64_t,
            b"normalizer Replace fields\0" as *const u8 as *const ::core::ffi::c_char,
        )
    } else if toks_jstr(rx, b" {2,}\0" as *const u8 as *const ::core::ffi::c_char) != 0
        && toks_jstr(con, b" \0" as *const u8 as *const ::core::ffi::c_char) != 0
    {
        (*src).cfg.collapse = 1 as uint8_t;
        st = ST_COLLAPSE as ::core::ffi::c_int as uint32_t;
    } else if toks_jstr(rx, b"(?<!\\n)^\0" as *const u8 as *const ::core::ffi::c_char)
        != 0 && toks_jstr(con, META_UTF8.as_ptr()) != 0
    {
        (*src).cfg.meta_prefix = 1 as uint8_t;
        st = ST_PREFIX as ::core::ffi::c_int as uint32_t;
    } else if (toks_jstr(rx, b" \0" as *const u8 as *const ::core::ffi::c_char) != 0
        || toks_jstr(str, b" \0" as *const u8 as *const ::core::ffi::c_char) != 0)
        && toks_jstr(con, META_UTF8.as_ptr()) != 0
    {
        (*src).cfg.meta_replace = 1 as uint8_t;
    } else if !str.is_null()
        && (*str).type_0 as ::core::ffi::c_int == JV_STR as ::core::ffi::c_int
        && (*str).s_len.wrapping_sub(1 as uint32_t) < 4 as uint32_t
        && (*con).s_len <= (*str).s_len
        && ((*src).cfg.rep_n as ::core::ffi::c_uint) < 4 as ::core::ffi::c_uint
    {
        let fresh10 = (*src).cfg.rep_n;
        (*src).cfg.rep_n = (*src).cfg.rep_n.wrapping_add(1);
        let mut i: uint8_t = fresh10;
        memcpy(
            &raw mut *(&raw mut (*src).cfg.rep_p as *mut [uint8_t; 4]).offset(i as isize)
                as *mut uint8_t as *mut ::core::ffi::c_void,
            (*str).s as *const ::core::ffi::c_void,
            (*str).s_len as size_t,
        );
        memcpy(
            &raw mut *(&raw mut (*src).cfg.rep_c as *mut [uint8_t; 4]).offset(i as isize)
                as *mut uint8_t as *mut ::core::ffi::c_void,
            (*con).s as *const ::core::ffi::c_void,
            (*con).s_len as size_t,
        );
        (*src).cfg.rep_pl[i as usize] = (*str).s_len as uint8_t;
        (*src).cfg.rep_cl[i as usize] = (*con).s_len as uint8_t;
        st = ST_REP as ::core::ffi::c_int as uint32_t;
    } else {
        return toks_fail(
            err,
            TOKS_E_UNSUPPORTED as int64_t,
            b"normalizer Replace (Unigram: ' {2,}' -> ' ', '(?<!\\n)^' -> '\xE2\x96\x81', ' ' -> '\xE2\x96\x81', String -> no longer)\0"
                as *const u8 as *const ::core::ffi::c_char,
        )
    }
    if st < *stage || st == *stage && st != ST_REP as ::core::ffi::c_int as uint32_t {
        return toks_fail(
            err,
            TOKS_E_UNSUPPORTED as int64_t,
            b"normalizer order (Unigram: Replace(String), NF*, StripAccents, Lowercase, Strip, Precompiled, Replace)\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
    }
    *stage = st;
    return 0 as int64_t;
}
unsafe extern "C" fn uni_meta(
    mut m: *const jv,
    mut always: *mut uint8_t,
    mut split: *mut uint8_t,
    mut err: *mut toks_err,
) -> int64_t {
    let mut rep: toks_spm_str = toks_spm_str { b: [0; 16], n: 0 };
    let mut scheme: uint32_t = 0;
    let mut sp: uint32_t = 0;
    let mut r: int64_t = read_meta(m, &raw mut rep, &raw mut scheme, &raw mut sp, err);
    if r != 0 as int64_t {
        return r;
    }
    if rep.n != 3 as uint32_t
        || memcmp(
            &raw mut rep.b as *mut uint8_t as *const ::core::ffi::c_void,
            META_UTF8.as_ptr() as *const ::core::ffi::c_void,
            3 as size_t,
        ) != 0 as ::core::ffi::c_int
    {
        return toks_fail(
            err,
            TOKS_E_UNSUPPORTED as int64_t,
            b"Metaspace replacement (toks: U+2581)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    if scheme == TOKS_SPM_PS_FIRST as ::core::ffi::c_int as uint32_t {
        return toks_fail(
            err,
            TOKS_E_UNSUPPORTED as int64_t,
            b"Metaspace prepend_scheme first (Unigram)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    *always = (scheme == TOKS_SPM_PS_ALWAYS as ::core::ffi::c_int as uint32_t)
        as ::core::ffi::c_int as uint8_t;
    *split = sp as uint8_t;
    return 0 as int64_t;
}
unsafe extern "C" fn uni_replace(
    mut d: *const jv,
    mut pat: *const ::core::ffi::c_char,
    mut content: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut str: *const jv = ::core::ptr::null::<jv>();
    let mut rx: *const jv = ::core::ptr::null::<jv>();
    let mut con: *const jv = ::core::ptr::null::<jv>();
    return (replace_of(d, &raw mut str, &raw mut rx, &raw mut con) != 0
        && (toks_jstr(rx, pat) != 0 || toks_jstr(str, pat) != 0)
        && toks_jstr(con, content) != 0) as ::core::ffi::c_int;
}
unsafe extern "C" fn parse_unigram(
    mut root: *const jv,
    mut model: *const jv,
    mut ar: *mut toks_arena,
    mut cfg: *mut toks_config,
    mut err: *mut toks_err,
) -> int64_t {
    let mut src: *mut toks_uni_src = toks_ar_alloc(
        ar,
        ::core::mem::size_of::<toks_uni_src>() as uint64_t,
        8 as uint64_t,
    ) as *mut toks_uni_src;
    let mut vocab: *const jv = toks_jv_get(
        model,
        b"vocab\0" as *const u8 as *const ::core::ffi::c_char,
    );
    if src.is_null() {
        return toks_fail(
            err,
            TOKS_E_NOMEM as int64_t,
            b"unigram config\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    memset(
        src as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<toks_uni_src>() as size_t,
    );
    if vocab.is_null()
        || (*vocab).type_0 as ::core::ffi::c_int != JV_ARR as ::core::ffi::c_int
    {
        return toks_fail(
            err,
            TOKS_E_FORMAT as int64_t,
            b"model.vocab missing\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    let mut n: uint64_t = 0 as uint64_t;
    let mut e: *const jv = (*vocab).child;
    while !e.is_null() {
        n = n.wrapping_add(1);
        e = (*e).next;
    }
    if n == 0 as uint64_t {
        return toks_fail(
            err,
            TOKS_E_FORMAT as int64_t,
            b"model.vocab empty\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if n > TOKS_MAX_IDS as uint64_t {
        return toks_fail(
            err,
            TOKS_E_LIMIT as int64_t,
            b"model.vocab > TOKS_MAX_IDS pieces\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    let mut ps: *mut *const uint8_t = toks_ar_alloc(
        ar,
        n.wrapping_mul(::core::mem::size_of::<*mut uint8_t>() as uint64_t),
        8 as uint64_t,
    ) as *mut *const uint8_t;
    let mut pl: *mut uint32_t = toks_ar_alloc(
        ar,
        n.wrapping_mul(4 as uint64_t),
        8 as uint64_t,
    ) as *mut uint32_t;
    let mut ss: *mut *const uint8_t = toks_ar_alloc(
        ar,
        n.wrapping_mul(::core::mem::size_of::<*mut uint8_t>() as uint64_t),
        8 as uint64_t,
    ) as *mut *const uint8_t;
    let mut sl: *mut uint32_t = toks_ar_alloc(
        ar,
        n.wrapping_mul(4 as uint64_t),
        8 as uint64_t,
    ) as *mut uint32_t;
    let mut ux: toks_sidx = toks_sidx {
        s: ::core::ptr::null::<*const uint8_t>(),
        len: ::core::ptr::null::<uint32_t>(),
        slot: ::core::ptr::null_mut::<uint32_t>(),
        mask: 0,
    };
    if ps.is_null() || pl.is_null() || ss.is_null() || sl.is_null()
        || toks_sidx_init(&raw mut ux, ar, ps, pl, n) != 0 as int64_t
    {
        return toks_fail(
            err,
            TOKS_E_NOMEM as int64_t,
            b"model.vocab arrays\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    let mut i: uint32_t = 0 as uint32_t;
    let mut e_0: *const jv = (*vocab).child;
    while !e_0.is_null() {
        let mut pc: *const jv = if (*e_0).type_0 as ::core::ffi::c_int
            == JV_ARR as ::core::ffi::c_int
        {
            (*e_0).child
        } else {
            ::core::ptr::null_mut::<jv>()
        };
        let mut sc: *const jv = if !pc.is_null() {
            (*pc).next
        } else {
            ::core::ptr::null_mut::<jv>()
        };
        if pc.is_null() || sc.is_null() || !(*sc).next.is_null()
            || (*pc).type_0 as ::core::ffi::c_int != JV_STR as ::core::ffi::c_int
            || (*sc).type_0 as ::core::ffi::c_int != JV_NUM as ::core::ffi::c_int
        {
            return toks_fail(
                err,
                TOKS_E_FORMAT as int64_t,
                b"model.vocab entry is not [piece, score]\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        if (*pc).s_len > TOKS_MAX_TOKEN_BYTES as uint32_t {
            return toks_fail(
                err,
                TOKS_E_LIMIT as int64_t,
                b"model.vocab piece > TOKS_MAX_TOKEN_BYTES\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        let ref mut fresh8 = *ps.offset(i as isize);
        *fresh8 = if !(*pc).s.is_null() {
            (*pc).s
        } else {
            b"\0" as *const u8 as *const ::core::ffi::c_char as *const uint8_t
        };
        *pl.offset(i as isize) = (*pc).s_len;
        let ref mut fresh9 = *ss.offset(i as isize);
        *fresh9 = (*sc).s;
        *sl.offset(i as isize) = (*sc).s_len;
        e_0 = (*e_0).next;
        i = i.wrapping_add(1);
    }
    let mut unk: *const jv = toks_jv_get(
        model,
        b"unk_id\0" as *const u8 as *const ::core::ffi::c_char,
    );
    let mut bf: *const jv = toks_jv_get(
        model,
        b"byte_fallback\0" as *const u8 as *const ::core::ffi::c_char,
    );
    if toks_jnull(unk) == 0 && toks_juint(unk, INT64_MAX as uint64_t) == 0 {
        return toks_fail(
            err,
            TOKS_E_FORMAT as int64_t,
            b"model.unk_id\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if !bf.is_null()
        && (*bf).type_0 as ::core::ffi::c_int != JV_BOOL as ::core::ffi::c_int
    {
        return toks_fail(
            err,
            TOKS_E_FORMAT as int64_t,
            b"model.byte_fallback\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    (*src).n = n as uint32_t;
    (*src).piece = ps;
    (*src).piece_len = pl;
    (*src).score_txt = ss;
    (*src).score_len = sl;
    (*src).unk_id = if toks_jnull(unk) != 0 {
        -(1 as ::core::ffi::c_int) as int64_t
    } else {
        (*unk).num
    };
    (*src).byte_fallback = toks_jtrue(bf) as uint32_t;
    let mut s: steps = steps {
        o: [::core::ptr::null::<jv>(); 16],
        k: [0; 16],
        n: 0,
        present: 0,
    };
    let mut stage: uint32_t = 0 as uint32_t;
    let mut r: int64_t = read_steps(
        root,
        C_NORM as ::core::ffi::c_int as uint32_t,
        ((1 as ::core::ffi::c_ulonglong) << N_STRIP as ::core::ffi::c_int
            | (1 as ::core::ffi::c_ulonglong) << N_PRECOMPILED as ::core::ffi::c_int
            | (1 as ::core::ffi::c_ulonglong) << N_REPLACE as ::core::ffi::c_int
            | (1 as ::core::ffi::c_ulonglong) << N_NFC as ::core::ffi::c_int
            | (1 as ::core::ffi::c_ulonglong) << N_NFD as ::core::ffi::c_int
            | (1 as ::core::ffi::c_ulonglong) << N_NFKC as ::core::ffi::c_int
            | (1 as ::core::ffi::c_ulonglong) << N_NFKD as ::core::ffi::c_int
            | (1 as ::core::ffi::c_ulonglong) << N_STRIPACC as ::core::ffi::c_int
            | (1 as ::core::ffi::c_ulonglong) << N_LOWER as ::core::ffi::c_int)
            as uint64_t,
        &raw mut s,
        err,
    );
    let mut k: uint32_t = 0 as uint32_t;
    while r == 0 as int64_t && k < s.n {
        r = uni_norm_one(
            s.o[k as usize],
            s.k[k as usize] as uint32_t,
            src,
            ar,
            &raw mut stage,
            err,
        );
        k = k.wrapping_add(1);
    }
    (*cfg).nfc = (*src).cfg.form;
    if r == 0 as int64_t
        && (*cfg).nfc as ::core::ffi::c_uint & TOKS_NS_COMPOSE
            != 0 as ::core::ffi::c_uint
        && (*cfg).nfc as ::core::ffi::c_uint & (TOKS_NS_STRIP_M | TOKS_NS_LOWER)
            != 0 as ::core::ffi::c_uint
    {
        r = toks_fail(
            err,
            TOKS_E_UNSUPPORTED as int64_t,
            b"normalizer NFC / NFKC then StripAccents or Lowercase (Unigram)\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if r == 0 as int64_t {
        r = read_steps(
            root,
            C_PRE as ::core::ffi::c_int as uint32_t,
            ((1 as ::core::ffi::c_ulonglong) << P_WSSPLIT as ::core::ffi::c_int
                | (1 as ::core::ffi::c_ulonglong) << P_METASPACE as ::core::ffi::c_int)
                as uint64_t,
            &raw mut s,
            err,
        );
    }
    if r == 0 as int64_t
        && (s.n > 2 as uint32_t
            || s.n == 2 as uint32_t
                && (s.k[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                    != P_WSSPLIT as ::core::ffi::c_int
                    || s.k[1 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                        != P_METASPACE as ::core::ffi::c_int))
    {
        r = toks_fail(
            err,
            TOKS_E_UNSUPPORTED as int64_t,
            b"pre_tokenizer Sequence (Unigram: [WhitespaceSplit, Metaspace])\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if r == 0 as int64_t && s.n != 0 as uint32_t {
        (*src).cfg.ws_split = (s.k[0 as ::core::ffi::c_int as usize]
            as ::core::ffi::c_int == P_WSSPLIT as ::core::ffi::c_int)
            as ::core::ffi::c_int as uint8_t;
        (*src).cfg.metaspace = (s.k[s.n.wrapping_sub(1 as uint32_t) as usize]
            as ::core::ffi::c_int == P_METASPACE as ::core::ffi::c_int)
            as ::core::ffi::c_int as uint8_t;
        if (*src).cfg.metaspace != 0 {
            r = uni_meta(
                s.o[s.n.wrapping_sub(1 as uint32_t) as usize],
                &raw mut (*src).cfg.meta_prepend,
                &raw mut (*src).cfg.meta_split,
                err,
            );
        }
    }
    if r == 0 as int64_t
        && (*src).cfg.meta_prefix as ::core::ffi::c_uint != 0 as ::core::ffi::c_uint
        && (*src).cfg.meta_replace as ::core::ffi::c_uint == 0 as ::core::ffi::c_uint
        && (*src).cfg.metaspace as ::core::ffi::c_uint == 0 as ::core::ffi::c_uint
    {
        r = toks_fail(
            err,
            TOKS_E_UNSUPPORTED as int64_t,
            b"normalizer Replace '(?<!\\n)^' -> '\xE2\x96\x81' without ' ' -> '\xE2\x96\x81' or Metaspace (Unigram)\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if r == 0 as int64_t {
        r = read_steps(
            root,
            C_DEC as ::core::ffi::c_int as uint32_t,
            ((1 as ::core::ffi::c_ulonglong) << D_METASPACE as ::core::ffi::c_int
                | (1 as ::core::ffi::c_ulonglong) << D_REPLACE as ::core::ffi::c_int
                | (1 as ::core::ffi::c_ulonglong) << D_BYTEFALLBACK as ::core::ffi::c_int
                | (1 as ::core::ffi::c_ulonglong) << D_FUSE as ::core::ffi::c_int)
                as uint64_t,
            &raw mut s,
            err,
        );
    }
    let mut dsplit: uint8_t = 1 as uint8_t;
    if r != 0 as int64_t || s.present == 0 {
        (*src).cfg.dec = TOKS_UNI_DEC_NONE as ::core::ffi::c_int as uint8_t;
    } else if s.n == 1 as uint32_t
        && s.k[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
            == D_METASPACE as ::core::ffi::c_int
    {
        (*src).cfg.dec = TOKS_UNI_DEC_META as ::core::ffi::c_int as uint8_t;
        r = uni_meta(
            s.o[0 as ::core::ffi::c_int as usize],
            &raw mut (*src).cfg.dec_prepend,
            &raw mut dsplit,
            err,
        );
    } else if s.n == 3 as uint32_t
        && uni_replace(
            s.o[0 as ::core::ffi::c_int as usize],
            META_UTF8.as_ptr(),
            b" \0" as *const u8 as *const ::core::ffi::c_char,
        ) != 0
        && s.k[1 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
            == D_BYTEFALLBACK as ::core::ffi::c_int
        && s.k[2 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
            == D_FUSE as ::core::ffi::c_int
    {
        (*src).cfg.dec = TOKS_UNI_DEC_RBF as ::core::ffi::c_int as uint8_t;
    } else if s.n == 4 as uint32_t
        && s.k[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
            == D_BYTEFALLBACK as ::core::ffi::c_int
        && uni_replace(
            s.o[1 as ::core::ffi::c_int as usize],
            META_UTF8.as_ptr(),
            b" \0" as *const u8 as *const ::core::ffi::c_char,
        ) != 0
        && s.k[2 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
            == D_FUSE as ::core::ffi::c_int
        && uni_replace(
            s.o[3 as ::core::ffi::c_int as usize],
            b"(?<!\\n)^ \0" as *const u8 as *const ::core::ffi::c_char,
            b"\0" as *const u8 as *const ::core::ffi::c_char,
        ) != 0
    {
        (*src).cfg.dec = TOKS_UNI_DEC_BFRF as ::core::ffi::c_int as uint8_t;
    } else {
        r = toks_fail(
            err,
            TOKS_E_UNSUPPORTED as int64_t,
            b"decoder (Unigram: Metaspace; Replace > ByteFallback > Fuse; ByteFallback > Replace > Fuse > Replace)\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if r == 0 as int64_t {
        r = read_trunc_pad(root, cfg, err);
    }
    if r != 0 as int64_t {
        return r;
    }
    if (*cfg).o.trunc_on != 0 && (*cfg).o.trunc_max == 0 as uint32_t {
        return toks_fail(
            err,
            TOKS_E_UNSUPPORTED as int64_t,
            b"truncation max_length\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if (*src).cfg.meta_replace as ::core::ffi::c_int != 0
        && (*src).cfg.metaspace as ::core::ffi::c_int != 0
    {
        return toks_fail(
            err,
            TOKS_E_UNSUPPORTED as int64_t,
            b"Replace ' ' -> '\xE2\x96\x81' before Metaspace\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    let mut id: uint32_t = 0 as uint32_t;
    while id < n as uint32_t {
        if toks_sidx_add(&raw mut ux, id) != 0 as ::core::ffi::c_int {
            return toks_fail(
                err,
                TOKS_E_UNSUPPORTED as int64_t,
                b"Unigram vocab repeats a piece\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        id = id.wrapping_add(1);
    }
    (*cfg).vocab = ps;
    (*cfg).vocab_len = pl;
    (*cfg).n_vocab = n as uint32_t;
    (*cfg).algo = TOKS_ALGO_UNIGRAM as uint32_t;
    (*cfg).n_strings = n as uint32_t;
    (*cfg).uni = src;
    r = read_added(root, ar, cfg, &raw mut ux, err);
    if r == 0 as int64_t {
        r = read_post_processor(root, ar, cfg, err);
    }
    return if r != 0 as int64_t { r } else { trunc_check(cfg, err) };
}
#[no_mangle]
pub unsafe extern "C" fn toks_config_parse(
    mut data: *const uint8_t,
    mut len: uint64_t,
    mut ar: *mut toks_arena,
    mut cfg: *mut toks_config,
    mut err: *mut toks_err,
) -> int64_t {
    memset(
        cfg as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<toks_config>() as size_t,
    );
    (*err).code = 0 as int64_t;
    (*err).what = ::core::ptr::null::<::core::ffi::c_char>();
    let mut root: *mut jv = ::core::ptr::null_mut::<jv>();
    let mut r: int64_t = toks_json_parse(data, len, ar, &raw mut root);
    if r != 0 as int64_t {
        return toks_fail(
            err,
            r,
            b"json syntax\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if (*root).type_0 as ::core::ffi::c_int != JV_OBJ as ::core::ffi::c_int {
        return toks_fail(
            err,
            TOKS_E_FORMAT as int64_t,
            b"json root is not an object\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    let mut model: *const jv = toks_jv_get(
        root,
        b"model\0" as *const u8 as *const ::core::ffi::c_char,
    );
    if model.is_null()
        || (*model).type_0 as ::core::ffi::c_int != JV_OBJ as ::core::ffi::c_int
    {
        return toks_fail(
            err,
            TOKS_E_FORMAT as int64_t,
            b"model missing\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    r = hf_refuses(root, err);
    if r != 0 as int64_t {
        return r;
    }
    if toks_jtype(model, b"WordPiece\0" as *const u8 as *const ::core::ffi::c_char) != 0
        || legacy_wordpiece(model) != 0
    {
        return parse_wordpiece(root, model, ar, cfg, err);
    }
    if uni_model(model) != 0 {
        return parse_unigram(root, model, ar, cfg, err);
    }
    r = read_trunc_pad(root, cfg, err);
    if r != 0 as int64_t {
        return r;
    }
    (*cfg).algo = TOKS_ALGO_BPE_BYTELEVEL as uint32_t;
    r = if is_spm(root, model) != 0 {
        parse_spm(root, model, ar, cfg, err)
    } else {
        parse_bytelevel(root, model, ar, cfg, err)
    };
    return if r != 0 as int64_t { r } else { trunc_check(cfg, err) };
}
unsafe extern "C" fn run_static_initializers() {
    TOKS_PATTERNS_N = (::core::mem::size_of::<[toks_pattern; 9]>() as usize)
        .wrapping_div(::core::mem::size_of::<toks_pattern>() as usize) as uint32_t;
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
