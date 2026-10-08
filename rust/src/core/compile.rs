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
    fn toks_plat_alloc(n: uint64_t) -> *mut ::core::ffi::c_void;
    fn toks_plat_free(p: *mut ::core::ffi::c_void, n: uint64_t);
    fn toks_plat_arena(n: uint64_t) -> *mut uint8_t;
    fn toks_token_bytes(s: *const uint8_t, n: uint32_t, out: *mut uint8_t) -> uint32_t;
    fn toks_classes_bytes(class_flags: uint32_t) -> uint64_t;
    fn toks_classes_build(
        class_flags: uint32_t,
        buf: *mut uint8_t,
        buf_len: uint64_t,
        out: *mut toks_class_tables,
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
pub const TOKS_PPS_TOK: C2RustUnnamed = 1;
pub const TOKS_PPS_SEQ: C2RustUnnamed = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rx_cur {
    pub base: *mut uint8_t,
    pub node: uint32_t,
    pub lab: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct toks_class_tables {
    pub ascii: *const uint8_t,
    pub stage1: *const uint16_t,
    pub stage2: *const uint8_t,
    pub n_blocks: uint32_t,
    pub _pad: uint32_t,
}
pub type C2RustUnnamed_0 = ::core::ffi::c_uint;
pub const TOKS_BOUND_PHASE1: C2RustUnnamed_0 = 2;
pub const TOKS_BOUND_PHASE0: C2RustUnnamed_0 = 1;
pub const TOKS_BOUND_TEXT: C2RustUnnamed_0 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct toks_bound_terms {
    pub num: uint32_t,
    pub den: uint32_t,
    pub g: uint32_t,
    pub x: uint32_t,
    pub xd: uint32_t,
    pub f: uint32_t,
    pub p0: uint32_t,
    pub p1: uint32_t,
    pub l0: uint32_t,
    pub l1: uint32_t,
    pub e0: uint32_t,
    pub e1: uint32_t,
    pub pfirst: uint32_t,
    pub why: uint32_t,
}
pub const TOKS_SPM_PFX_NONE: C2RustUnnamed_1 = 0;
pub const TOKS_SPM_PFX_ALWAYS: C2RustUnnamed_1 = 2;
pub const TOKS_SPM_PFX_GAP: C2RustUnnamed_1 = 1;
pub type C2RustUnnamed_1 = ::core::ffi::c_uint;
pub const TOKS_SPM_PFX_FIRST: C2RustUnnamed_1 = 3;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
pub const TOKS_E_LIMIT: ::core::ffi::c_int = -(9 as ::core::ffi::c_int);
pub const TOKS_E_NOMEM: ::core::ffi::c_int = -(11 as ::core::ffi::c_int);
pub const TOKS_MAX_ADDED_BYTES: ::core::ffi::c_uint = 255 as ::core::ffi::c_uint;
pub const TOKS_ALGO_BPE_BYTELEVEL: ::core::ffi::c_uint = 1 as ::core::ffi::c_uint;
pub const TOKS_ALGO_BPE_SPM: ::core::ffi::c_uint = 2 as ::core::ffi::c_uint;
pub const TOKS_ALGO_UNIGRAM: ::core::ffi::c_uint = 3 as ::core::ffi::c_uint;
pub const TOKS_ALGO_WORDPIECE: ::core::ffi::c_uint = 4 as ::core::ffi::c_uint;
pub const TOKS_TABLES_MAGIC: ::core::ffi::c_uint = 0x544b4f54 as ::core::ffi::c_uint;
pub const TOKS_TABLES_VERSION: ::core::ffi::c_uint = 2 as ::core::ffi::c_uint;
pub const TOKS_C_BASE_MASK: ::core::ffi::c_uint = 0x7 as ::core::ffi::c_uint;
pub const TOKS_C_P: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
pub const TOKS_C_L: ::core::ffi::c_uint = 1 as ::core::ffi::c_uint;
pub const TOKS_C_N: ::core::ffi::c_uint = 2 as ::core::ffi::c_uint;
pub const TOKS_C_WS: ::core::ffi::c_uint = 3 as ::core::ffi::c_uint;
pub const TOKS_C_NL: ::core::ffi::c_uint = 4 as ::core::ffi::c_uint;
pub const TOKS_C_X: ::core::ffi::c_uint = 5 as ::core::ffi::c_uint;
pub const TOKS_C_UPPER: ::core::ffi::c_uint = 0x8 as ::core::ffi::c_uint;
pub const TOKS_C_LOWER: ::core::ffi::c_uint = 0x10 as ::core::ffi::c_uint;
pub const TOKS_C_FOLD_S: ::core::ffi::c_uint = 0x40 as ::core::ffi::c_uint;
pub const TOKS_C_HAN: ::core::ffi::c_uint = 0x80 as ::core::ffi::c_uint;
pub const TOKS_C_CJK: ::core::ffi::c_uint = 0x80 as ::core::ffi::c_uint;
pub const TOKS_TMPL_NONE: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
pub const TOKS_TMPL_CL100K: ::core::ffi::c_uint = 1 as ::core::ffi::c_uint;
pub const TOKS_TMPL_O200K: ::core::ffi::c_uint = 2 as ::core::ffi::c_uint;
pub const TOKS_TMPL_DSV3: ::core::ffi::c_uint = 3 as ::core::ffi::c_uint;
pub const TOKS_TP_CONTR_MASK: ::core::ffi::c_uint = 0x3 as ::core::ffi::c_uint;
pub const TOKS_TP_CONTR_CI: ::core::ffi::c_uint = 0x2 as ::core::ffi::c_uint;
pub const TOKS_TP_DIGITS_MASK: ::core::ffi::c_uint = 0x18 as ::core::ffi::c_uint;
pub const TOKS_TP_DIGITS_1_3: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
pub const TOKS_TP_DIGITS_1: ::core::ffi::c_uint = 0x8 as ::core::ffi::c_uint;
pub const TOKS_TP_DIGIT_CUT: ::core::ffi::c_uint = 0x200 as ::core::ffi::c_uint;
pub const TOKS_TP_NL_CUT: ::core::ffi::c_uint = 0x400 as ::core::ffi::c_uint;
pub const TOKS_TP_GB_SP: ::core::ffi::c_uint = 0x800 as ::core::ffi::c_uint;
pub const TOKS_TP_HAN: ::core::ffi::c_uint = 0x80 as ::core::ffi::c_uint;
pub const TOKS_TP_NO_SLASH: ::core::ffi::c_uint = 0x100 as ::core::ffi::c_uint;
pub const TOKS_TF_IGNORE_MERGES: ::core::ffi::c_uint = 0x1 as ::core::ffi::c_uint;
pub const TOKS_TF_CJK_L: ::core::ffi::c_uint = 0x4 as ::core::ffi::c_uint;
pub const TOKS_TF_CJK_B: ::core::ffi::c_uint = 0x8 as ::core::ffi::c_uint;
pub const TOKS_TF_CJK_D: ::core::ffi::c_uint = 0x10 as ::core::ffi::c_uint;
pub const TOKS_TF_TWIN: ::core::ffi::c_uint = 0x20 as ::core::ffi::c_uint;
pub const TOKS_AF_SPECIAL: ::core::ffi::c_uint = 0x1 as ::core::ffi::c_uint;
pub const TOKS_AF_LSTRIP: ::core::ffi::c_uint = 0x2 as ::core::ffi::c_uint;
pub const TOKS_AF_RSTRIP: ::core::ffi::c_uint = 0x4 as ::core::ffi::c_uint;
pub const TOKS_AF_SINGLE_WORD: ::core::ffi::c_uint = 0x8 as ::core::ffi::c_uint;
pub const TOKS_AF_NORMALIZED: ::core::ffi::c_uint = 0x10 as ::core::ffi::c_uint;
pub const TOKS_AF_PFX: ::core::ffi::c_uint = 0x20 as ::core::ffi::c_uint;
pub const TOKS_K1_H4_BITS: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const TOKS_K1_LIST: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const RX_CH: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const RX_LAB: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
pub const RX_LLEN: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
pub const RX_NCH: ::core::ffi::c_int = 0xa as ::core::ffi::c_int;
pub const RX_ENT: ::core::ffi::c_int = 0xc as ::core::ffi::c_int;
pub const TOKS_X_TOK_OFF: toks_ext = toks_ext {
    pad: 0 as uint32_t,
    align: 4 as uint32_t,
};
pub const TOKS_X_TOK_BYTES: toks_ext = toks_ext {
    pad: 0 as uint32_t,
    align: 1 as uint32_t,
};
pub const TOKS_X_CLS_ASCII: toks_ext = toks_ext {
    pad: 0 as uint32_t,
    align: 1 as uint32_t,
};
pub const TOKS_X_CLS_STAGE1: toks_ext = toks_ext {
    pad: 0 as uint32_t,
    align: 2 as uint32_t,
};
pub const TOKS_X_CLS_STAGE2: toks_ext = toks_ext {
    pad: 0 as uint32_t,
    align: 1 as uint32_t,
};
pub const TOKS_X_SPECIAL: toks_ext = toks_ext {
    pad: 0 as uint32_t,
    align: 4 as uint32_t,
};
pub const TOKS_X_ADD_ENTRIES: toks_ext = toks_ext {
    pad: 0 as uint32_t,
    align: 4 as uint32_t,
};
pub const TOKS_X_ADD_BYTES: toks_ext = toks_ext {
    pad: 0 as uint32_t,
    align: 1 as uint32_t,
};
pub const TOKS_X_ADD_SHUFTI: toks_ext = toks_ext {
    pad: 0 as uint32_t,
    align: 1 as uint32_t,
};
pub const TOKS_X_ADD_INDEX: toks_ext = toks_ext {
    pad: 0 as uint32_t,
    align: 8 as uint32_t,
};
pub const TOKS_X_ADD_SINGLE: toks_ext = toks_ext {
    pad: 0 as uint32_t,
    align: 4 as uint32_t,
};
pub const TOKS_X_ADD_CAND: toks_ext = toks_ext {
    pad: 0 as uint32_t,
    align: 4 as uint32_t,
};
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
unsafe extern "C" fn toks_tab_fit(
    mut p: *const ::core::ffi::c_void,
    mut n: uint64_t,
    mut x: toks_ext,
) -> *const ::core::ffi::c_void {
    return p;
}
#[inline]
unsafe extern "C" fn toks_tab_seal(mut b: *mut ::core::ffi::c_void, mut n: uint64_t) {}
pub const TOKS_NFC_X: ::core::ffi::c_uint = 3 as ::core::ffi::c_uint;
pub const TOKS_NFKC_X: ::core::ffi::c_uint = 11 as ::core::ffi::c_uint;
#[inline]
unsafe extern "C" fn toks_k1_h4(mut s: *const uint8_t) -> uint32_t {
    let mut k: uint32_t = 0;
    memcpy(
        &raw mut k as *mut ::core::ffi::c_void,
        s as *const ::core::ffi::c_void,
        4 as size_t,
    );
    return k.wrapping_mul(0x9e3779b1 as uint32_t)
        >> (32 as ::core::ffi::c_uint)
            .wrapping_sub(TOKS_K1_H4_BITS as ::core::ffi::c_uint);
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
pub const TOKS_NS_COMPAT: ::core::ffi::c_uint = 0x40 as ::core::ffi::c_uint;
pub const TOKS_UNI_TERM: ::core::ffi::c_uint = 0x80000000 as ::core::ffi::c_uint;
pub const TOKS_UNI_BASE: ::core::ffi::c_uint = 0x7fffffff as ::core::ffi::c_uint;
unsafe extern "C" fn up64(mut v: uint64_t) -> uint64_t {
    return v.wrapping_add(63 as uint64_t) & !(63 as ::core::ffi::c_uint as uint64_t);
}
unsafe extern "C" fn ascii_class(mut b: uint32_t) -> uint8_t {
    if b.wrapping_sub(0x41 as uint32_t) < 26 as uint32_t {
        return (TOKS_C_L | TOKS_C_UPPER) as uint8_t;
    }
    if b.wrapping_sub(0x61 as uint32_t) < 26 as uint32_t {
        return (TOKS_C_L | TOKS_C_LOWER) as uint8_t;
    }
    if b.wrapping_sub(0x30 as uint32_t) < 10 as uint32_t {
        return TOKS_C_N as uint8_t;
    }
    if b == 0xa as uint32_t || b == 0xd as uint32_t {
        return TOKS_C_NL as uint8_t;
    }
    if b == 0x9 as uint32_t || b == 0xb as uint32_t || b == 0xc as uint32_t
        || b == 0x20 as uint32_t
    {
        return TOKS_C_WS as uint8_t;
    }
    return TOKS_C_P as uint8_t;
}
#[no_mangle]
pub unsafe extern "C" fn toks_compile_cls_flags(mut t: *const toks_tables) -> int64_t {
    let mut d: ::core::ffi::c_int = ((*t).tmpl == TOKS_TMPL_DSV3 as uint32_t)
        as ::core::ffi::c_int;
    let mut b: uint32_t = 0 as uint32_t;
    while b < 128 as uint32_t {
        let mut want: uint8_t = ascii_class(b);
        if d != 0 && want as ::core::ffi::c_uint & TOKS_C_BASE_MASK == TOKS_C_L {
            want = TOKS_C_L as uint8_t;
        }
        if d != 0
            && (b < 9 as uint32_t || b.wrapping_sub(14 as uint32_t) < 18 as uint32_t
                || b == 127 as uint32_t)
        {
            want = TOKS_C_X as uint8_t;
        }
        if *(*t).cls_ascii.offset(b as isize) as ::core::ffi::c_int
            != want as ::core::ffi::c_int
        {
            return -(1 as ::core::ffi::c_int) as int64_t;
        }
        b = b.wrapping_add(1);
    }
    let mut cp: uint32_t = 0x4e00 as uint32_t;
    while d != 0 && cp < 0x9f00 as uint32_t {
        if toks_cls_cp(t, cp) as ::core::ffi::c_int
            != (TOKS_C_L | TOKS_C_CJK) as uint8_t as ::core::ffi::c_int
        {
            return 0 as int64_t;
        }
        cp = cp.wrapping_add(1);
    }
    if d != 0 {
        return TOKS_TF_CJK_D as int64_t;
    }
    static mut lo: [uint32_t; 2] = [
        0x4e00 as ::core::ffi::c_uint,
        0xac00 as ::core::ffi::c_uint,
    ];
    static mut hi: [uint32_t; 2] = [
        0xa400 as ::core::ffi::c_uint,
        0xd700 as ::core::ffi::c_uint,
    ];
    let keep: uint8_t = (TOKS_C_BASE_MASK | TOKS_C_FOLD_S) as uint8_t;
    let both: uint8_t = (TOKS_C_L | TOKS_C_UPPER | TOKS_C_LOWER) as uint8_t;
    let mut f: int64_t = (TOKS_TF_CJK_L | TOKS_TF_CJK_B) as int64_t;
    let mut r: uint32_t = 0 as uint32_t;
    while r < 2 as uint32_t && f != 0 as int64_t {
        let mut cp_0: uint32_t = lo[r as usize];
        while cp_0 < hi[r as usize] && f != 0 as int64_t {
            let mut c: uint8_t = toks_cls_cp(t, cp_0);
            if (c as ::core::ffi::c_int & keep as ::core::ffi::c_int) as uint8_t
                as ::core::ffi::c_uint != TOKS_C_L
            {
                f &= !(TOKS_TF_CJK_L as int64_t);
            }
            if c as ::core::ffi::c_int != both as ::core::ffi::c_int {
                f &= !(TOKS_TF_CJK_B as int64_t);
            }
            cp_0 = cp_0.wrapping_add(1);
        }
        r = r.wrapping_add(1);
    }
    return f;
}
unsafe extern "C" fn added_bytes(
    mut cfg: *const toks_config,
    mut a: *const toks_cfg_added,
    mut out: *mut uint8_t,
) -> uint32_t {
    if (*cfg).algo == TOKS_ALGO_WORDPIECE as uint32_t {
        if !out.is_null() {
            memcpy(
                out as *mut ::core::ffi::c_void,
                (*a).form as *const ::core::ffi::c_void,
                (*a).form_len as size_t,
            );
        }
        return (*a).form_len;
    }
    if (*cfg).algo != TOKS_ALGO_BPE_SPM as uint32_t
        && (*cfg).algo != TOKS_ALGO_UNIGRAM as uint32_t
    {
        return toks_token_bytes((*a).content, (*a).len, out);
    }
    let mut p: uint32_t = if (*a).pfx as ::core::ffi::c_uint != 0 as ::core::ffi::c_uint
    {
        3 as uint32_t
    } else {
        0 as uint32_t
    };
    if !out.is_null() {
        memcpy(
            out as *mut ::core::ffi::c_void,
            b"\xE2\x96\x81\0" as *const u8 as *const ::core::ffi::c_char
                as *const ::core::ffi::c_void,
            p as size_t,
        );
        memcpy(
            out.offset(p as isize) as *mut ::core::ffi::c_void,
            (*a).content as *const ::core::ffi::c_void,
            (*a).len as size_t,
        );
    }
    return p.wrapping_add((*a).len);
}
unsafe extern "C" fn vocab_bytes(
    mut cfg: *const toks_config,
    mut id: uint32_t,
    mut out: *mut uint8_t,
) -> uint32_t {
    if (*cfg).algo != TOKS_ALGO_BPE_SPM as uint32_t
        && (*cfg).algo != TOKS_ALGO_WORDPIECE as uint32_t
        && (*cfg).algo != TOKS_ALGO_UNIGRAM as uint32_t
    {
        return toks_token_bytes(
            *(*cfg).vocab.offset(id as isize),
            *(*cfg).vocab_len.offset(id as isize),
            out,
        );
    }
    if !out.is_null() && *(*cfg).vocab_len.offset(id as isize) != 0 as uint32_t {
        memcpy(
            out as *mut ::core::ffi::c_void,
            *(*cfg).vocab.offset(id as isize) as *const ::core::ffi::c_void,
            *(*cfg).vocab_len.offset(id as isize) as size_t,
        );
    }
    return *(*cfg).vocab_len.offset(id as isize);
}
pub const TOK_OWN: ::core::ffi::c_uint = 0x80000000 as ::core::ffi::c_uint;
pub const TOK_FORM: ::core::ffi::c_uint = 0x40000000 as ::core::ffi::c_uint;
unsafe extern "C" fn add_key(
    mut t: *const toks_tables,
    mut i: uint32_t,
    mut h4: ::core::ffi::c_int,
) -> int64_t {
    let mut x: *const toks_added_entry = (*t).add_entries.offset(i as isize)
        as *const toks_added_entry;
    let mut c: *const uint8_t = (*t).add_bytes.offset((*x).off as isize);
    if ((*x).len as ::core::ffi::c_uint)
        < (if h4 != 0 { 4 as ::core::ffi::c_uint } else { 2 as ::core::ffi::c_uint })
    {
        return -(1 as ::core::ffi::c_int) as int64_t;
    }
    return if h4 != 0 {
        ((toks_k1_h4(c) as uint64_t) << 1 as ::core::ffi::c_int | (*x).phase as uint64_t)
            as int64_t
    } else {
        (((*x).phase as uint32_t) << 16 as ::core::ffi::c_int
            | *c.offset(0 as ::core::ffi::c_int as isize) as uint32_t
            | (*c.offset(1 as ::core::ffi::c_int as isize) as uint32_t)
                << 8 as ::core::ffi::c_int) as int64_t
    };
}
unsafe extern "C" fn add_fill(
    mut t: *const toks_tables,
    mut idx: *mut uint64_t,
    mut nb: uint64_t,
    mut cand: *mut uint32_t,
    mut pos: uint64_t,
    mut h4: ::core::ffi::c_int,
) -> uint64_t {
    memset(
        idx as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        (nb as size_t).wrapping_mul(8 as size_t),
    );
    let mut i: uint32_t = 0 as uint32_t;
    while (i as uint64_t) < (*t).add_n {
        let mut k: int64_t = add_key(t, i, h4);
        if k >= 0 as int64_t {
            let ref mut fresh6 = *idx.offset(k as isize);
            *fresh6 = (*fresh6).wrapping_add(1 as uint64_t);
        }
        i = i.wrapping_add(1);
    }
    let mut s: uint64_t = 0 as uint64_t;
    while s < nb {
        let mut c: uint64_t = *idx.offset(s as isize);
        *idx.offset(s as isize) = pos;
        pos = pos.wrapping_add(c);
        s = s.wrapping_add(1);
    }
    let mut L: uint32_t = TOKS_MAX_ADDED_BYTES as uint32_t;
    while L >= 2 as uint32_t {
        let mut i_0: uint32_t = 0 as uint32_t;
        while (i_0 as uint64_t) < (*t).add_n {
            let mut k_0: int64_t = add_key(t, i_0, h4);
            if !(k_0 < 0 as int64_t
                || (*(*t).add_entries.offset(i_0 as isize)).len as uint32_t != L)
            {
                *cand
                    .offset(
                        (*idx.offset(k_0 as isize) as uint32_t as uint64_t)
                            .wrapping_add(
                                *idx.offset(k_0 as isize) >> 32 as ::core::ffi::c_int,
                            ) as isize,
                    ) = i_0;
                let ref mut fresh7 = *idx.offset(k_0 as isize);
                *fresh7 = (*fresh7 as ::core::ffi::c_ulonglong)
                    .wrapping_add(
                        (1 as ::core::ffi::c_ulonglong) << 32 as ::core::ffi::c_int,
                    ) as uint64_t as uint64_t;
            }
            i_0 = i_0.wrapping_add(1);
        }
        L = L.wrapping_sub(1);
    }
    return pos;
}
#[no_mangle]
pub unsafe extern "C" fn toks_added_cand_words(
    mut n2: uint64_t,
    mut n4: uint64_t,
    mut b4: uint64_t,
) -> uint64_t {
    let mut nodes: uint64_t = if n4 > TOKS_K1_LIST as uint64_t {
        (2 as uint64_t).wrapping_mul(n4)
    } else {
        0 as uint64_t
    };
    return (2 as uint64_t)
        .wrapping_mul(n2)
        .wrapping_add(
            (if nodes != 0 as uint64_t {
                (4 as uint64_t)
                    .wrapping_mul(nodes)
                    .wrapping_add(
                        b4
                            .wrapping_add(nodes)
                            .wrapping_add(35 as uint64_t)
                            .wrapping_div(4 as uint64_t),
                    )
                    .wrapping_add(4 as uint64_t)
            } else {
                0 as uint64_t
            }),
        );
}
unsafe extern "C" fn rx_s(mut t: *const toks_tables, mut e: uint32_t) -> *const uint8_t {
    return (*t).add_bytes.offset((*(*t).add_entries.offset(e as isize)).off as isize);
}
unsafe extern "C" fn rx_cmp(
    mut t: *const toks_tables,
    mut a: uint32_t,
    mut b: uint32_t,
) -> ::core::ffi::c_int {
    let mut la: uint32_t = (*(*t).add_entries.offset(a as isize)).len as uint32_t;
    let mut lb: uint32_t = (*(*t).add_entries.offset(b as isize)).len as uint32_t;
    let mut c: ::core::ffi::c_int = memcmp(
        rx_s(t, a) as *const ::core::ffi::c_void,
        rx_s(t, b) as *const ::core::ffi::c_void,
        (if la < lb { la } else { lb }) as size_t,
    );
    return if c != 0 as ::core::ffi::c_int {
        c
    } else {
        (la > lb) as ::core::ffi::c_int - (la < lb) as ::core::ffi::c_int
    };
}
unsafe extern "C" fn rx_sift(
    mut t: *const toks_tables,
    mut c: *mut uint32_t,
    mut k: uint32_t,
    mut n: uint32_t,
) {
    let mut j: uint32_t = (2 as uint32_t).wrapping_mul(k).wrapping_add(1 as uint32_t);
    while j < n {
        if j.wrapping_add(1 as uint32_t) < n
            && rx_cmp(
                t,
                *c.offset(j.wrapping_add(1 as uint32_t) as isize),
                *c.offset(j as isize),
            ) > 0 as ::core::ffi::c_int
        {
            j = j.wrapping_add(1);
        }
        if rx_cmp(t, *c.offset(k as isize), *c.offset(j as isize))
            >= 0 as ::core::ffi::c_int
        {
            return;
        }
        let mut x: uint32_t = *c.offset(k as isize);
        *c.offset(k as isize) = *c.offset(j as isize);
        *c.offset(j as isize) = x;
        k = j;
        j = (2 as uint32_t).wrapping_mul(k).wrapping_add(1 as uint32_t);
    }
}
unsafe extern "C" fn rx_node(
    mut t: *const toks_tables,
    mut c: *const uint32_t,
    mut lo: uint32_t,
    mut hi: uint32_t,
    mut d: uint32_t,
    mut r: *mut rx_cur,
    mut at: uint32_t,
) {
    let mut f: *const uint8_t = rx_s(t, *c.offset(lo as isize));
    let mut g: *const uint8_t = rx_s(
        t,
        *c.offset(hi.wrapping_sub(1 as uint32_t) as isize),
    );
    let mut lf: uint32_t = (*(*t).add_entries.offset(*c.offset(lo as isize) as isize))
        .len as uint32_t;
    let mut lg: uint32_t = (*(*t)
        .add_entries
        .offset(*c.offset(hi.wrapping_sub(1 as uint32_t) as isize) as isize))
        .len as uint32_t;
    let mut D: uint32_t = d;
    let mut nch: uint32_t = 0 as uint32_t;
    while D < lf && D < lg
        && *f.offset(D as isize) as ::core::ffi::c_int
            == *g.offset(D as isize) as ::core::ffi::c_int
    {
        D = D.wrapping_add(1);
    }
    let mut ent: uint32_t = if lf == D {
        (*c.offset(lo as isize)).wrapping_add(1 as uint32_t)
    } else {
        0 as uint32_t
    };
    let mut s: uint32_t = lo
        .wrapping_add((ent != 0 as uint32_t) as ::core::ffi::c_int as uint32_t);
    let mut ch: uint32_t = (*r).node;
    let mut lab: uint32_t = (*r).lab;
    let mut i: uint32_t = s;
    while i < hi {
        nch = nch
            .wrapping_add(
                (i == s
                    || *rx_s(t, *c.offset(i as isize)).offset(D as isize)
                        as ::core::ffi::c_int
                        != *rx_s(t, *c.offset(i.wrapping_sub(1 as uint32_t) as isize))
                            .offset(D as isize) as ::core::ffi::c_int)
                    as ::core::ffi::c_int as uint32_t,
            );
        i = i.wrapping_add(1);
    }
    (*r).node = ((*r).node as ::core::ffi::c_uint)
        .wrapping_add((16 as uint32_t).wrapping_mul(nch) as ::core::ffi::c_uint)
        as uint32_t as uint32_t;
    (*r).lab = (*r).lab.wrapping_add(D.wrapping_sub(d).wrapping_add(nch));
    memcpy(
        (*r).base.offset(lab as isize) as *mut ::core::ffi::c_void,
        f.offset(d as isize) as *const ::core::ffi::c_void,
        D.wrapping_sub(d) as size_t,
    );
    let mut nd: *mut uint8_t = (*r).base.offset(at as isize);
    let mut ln: uint16_t = D.wrapping_sub(d) as uint16_t;
    let mut nc: uint16_t = nch as uint16_t;
    toks_st32(nd.offset(RX_CH as isize) as *mut ::core::ffi::c_void, ch);
    toks_st32(nd.offset(RX_LAB as isize) as *mut ::core::ffi::c_void, lab);
    memcpy(
        nd.offset(RX_LLEN as isize) as *mut ::core::ffi::c_void,
        &raw mut ln as *const ::core::ffi::c_void,
        2 as size_t,
    );
    memcpy(
        nd.offset(RX_NCH as isize) as *mut ::core::ffi::c_void,
        &raw mut nc as *const ::core::ffi::c_void,
        2 as size_t,
    );
    toks_st32(nd.offset(RX_ENT as isize) as *mut ::core::ffi::c_void, ent);
    let mut i_0: uint32_t = s;
    let mut k: uint32_t = 0 as uint32_t;
    while i_0 < hi {
        let mut j: uint32_t = i_0.wrapping_add(1 as uint32_t);
        while j < hi
            && *rx_s(t, *c.offset(j as isize)).offset(D as isize) as ::core::ffi::c_int
                == *rx_s(t, *c.offset(i_0 as isize)).offset(D as isize)
                    as ::core::ffi::c_int
        {
            j = j.wrapping_add(1);
        }
        *(*r)
            .base
            .offset(lab.wrapping_add(D.wrapping_sub(d)).wrapping_add(k) as isize) = *rx_s(
                t,
                *c.offset(i_0 as isize),
            )
            .offset(D as isize);
        rx_node(t, c, i_0, j, D, r, ch.wrapping_add((16 as uint32_t).wrapping_mul(k)));
        i_0 = j;
        k = k.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn toks_added_index(
    mut t: *mut toks_tables,
    mut shuf: *mut uint8_t,
    mut idx: *mut uint64_t,
    mut single: *mut uint32_t,
    mut cand: *mut uint32_t,
) {
    memset(shuf as *mut ::core::ffi::c_void, 0 as ::core::ffi::c_int, 64 as size_t);
    memset(
        single as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        (2 as ::core::ffi::c_uint)
            .wrapping_mul(256 as ::core::ffi::c_uint)
            .wrapping_mul(4 as ::core::ffi::c_uint) as size_t,
    );
    (*t).add_phases = 0 as uint64_t;
    let mut i: uint32_t = 0 as uint32_t;
    while (i as uint64_t) < (*t).add_n {
        let mut x: *const toks_added_entry = (*t).add_entries.offset(i as isize)
            as *const toks_added_entry;
        let mut b0: uint8_t = *(*t).add_bytes.offset((*x).off as isize);
        let mut bit: uint8_t = ((1 as ::core::ffi::c_uint)
            << ((b0 as ::core::ffi::c_int >> 4 as ::core::ffi::c_int) as uint32_t
                & 7 as uint32_t)) as uint8_t;
        let ref mut fresh2 = *shuf
            .offset(
                ((*x).phase as ::core::ffi::c_uint)
                    .wrapping_mul(32 as ::core::ffi::c_uint)
                    .wrapping_add(b0 as ::core::ffi::c_uint & 15 as ::core::ffi::c_uint)
                    as isize,
            );
        *fresh2 = (*fresh2 as ::core::ffi::c_int | bit as ::core::ffi::c_int) as uint8_t;
        let ref mut fresh3 = *shuf
            .offset(
                ((*x).phase as ::core::ffi::c_uint)
                    .wrapping_mul(32 as ::core::ffi::c_uint)
                    .wrapping_add(16 as ::core::ffi::c_uint)
                    .wrapping_add(
                        (b0 as ::core::ffi::c_int >> 4 as ::core::ffi::c_int)
                            as ::core::ffi::c_uint,
                    ) as isize,
            );
        *fresh3 = (*fresh3 as ::core::ffi::c_int | bit as ::core::ffi::c_int) as uint8_t;
        if (*x).len as ::core::ffi::c_uint == 1 as ::core::ffi::c_uint {
            *single
                .offset(
                    ((*x).phase as ::core::ffi::c_uint)
                        .wrapping_mul(256 as ::core::ffi::c_uint)
                        .wrapping_add(b0 as ::core::ffi::c_uint) as isize,
                ) = i.wrapping_add(1 as uint32_t);
        }
        (*t).add_phases = ((*t).add_phases as ::core::ffi::c_ulonglong
            | (1 as ::core::ffi::c_ulonglong) << (*x).phase as ::core::ffi::c_int)
            as uint64_t;
        i = i.wrapping_add(1);
    }
    let mut used: uint64_t = add_fill(
        t,
        idx
            .offset(
                (2 as ::core::ffi::c_uint).wrapping_mul(65536 as ::core::ffi::c_uint)
                    as isize,
            ),
        ((2 as ::core::ffi::c_uint) << TOKS_K1_H4_BITS) as uint64_t,
        cand,
        add_fill(
            t,
            idx,
            (2 as ::core::ffi::c_uint).wrapping_mul(65536 as ::core::ffi::c_uint)
                as uint64_t,
            cand,
            0 as uint64_t,
            0 as ::core::ffi::c_int,
        ),
        1 as ::core::ffi::c_int,
    );
    let mut n4: uint64_t = 0 as uint64_t;
    let mut i_0: uint32_t = 0 as uint32_t;
    while (i_0 as uint64_t) < (*t).add_n {
        n4 = n4
            .wrapping_add(
                ((*(*t).add_entries.offset(i_0 as isize)).len as ::core::ffi::c_uint
                    >= 4 as ::core::ffi::c_uint) as ::core::ffi::c_int as uint64_t,
            );
        i_0 = i_0.wrapping_add(1);
    }
    let mut base: *mut uint8_t = cand as *mut uint8_t;
    let mut r: rx_cur = rx_cur {
        base: base,
        node: ((cand.offset(used as isize) as uintptr_t)
            .wrapping_add(15 as ::core::ffi::c_uint as uintptr_t)
            & !(15 as ::core::ffi::c_uint as uintptr_t))
            .wrapping_sub(base as uintptr_t) as uint32_t,
        lab: 0 as uint32_t,
    };
    r.lab = r
        .node
        .wrapping_add(
            (16 as uint32_t)
                .wrapping_mul(
                    (if n4 > TOKS_K1_LIST as uint64_t {
                        (2 as uint64_t).wrapping_mul(n4)
                    } else {
                        0 as uint64_t
                    }) as uint32_t,
                ),
        );
    let mut s: uint64_t = 0 as uint64_t;
    while s < ((2 as ::core::ffi::c_uint) << TOKS_K1_H4_BITS) as uint64_t {
        let mut h: *mut uint64_t = idx
            .offset(
                ((2 as ::core::ffi::c_uint).wrapping_mul(65536 as ::core::ffi::c_uint)
                    as uint64_t)
                    .wrapping_add(s) as isize,
            ) as *mut uint64_t;
        let mut n: uint32_t = (*h >> 32 as ::core::ffi::c_int) as uint32_t;
        let mut c: *mut uint32_t = cand.offset(*h as uint32_t as isize);
        if !(n <= TOKS_K1_LIST as uint32_t) {
            let mut k: uint32_t = n.wrapping_div(2 as uint32_t);
            loop {
                let fresh4 = k;
                k = k.wrapping_sub(1);
                if !(fresh4 > 0 as uint32_t) {
                    break;
                }
                rx_sift(t, c, k, n);
            }
            let mut m: uint32_t = n;
            loop {
                let fresh5 = m;
                m = m.wrapping_sub(1);
                if !(fresh5 > 1 as uint32_t) {
                    break;
                }
                let mut x_0: uint32_t = *c.offset(0 as ::core::ffi::c_int as isize);
                *c.offset(0 as ::core::ffi::c_int as isize) = *c.offset(m as isize);
                *c.offset(m as isize) = x_0;
                rx_sift(t, c, 0 as uint32_t, m);
            }
            let mut root: uint32_t = r.node;
            r.node = (r.node as ::core::ffi::c_uint)
                .wrapping_add(16 as ::core::ffi::c_uint) as uint32_t as uint32_t;
            rx_node(t, c, 0 as uint32_t, n, 0 as uint32_t, &raw mut r, root);
            *h = ((root as uint64_t | (n as uint64_t) << 32 as ::core::ffi::c_int)
                as ::core::ffi::c_ulonglong
                | (1 as ::core::ffi::c_ulonglong) << 63 as ::core::ffi::c_int)
                as uint64_t;
        }
        s = s.wrapping_add(1);
    }
    (*t).add_shufti = shuf;
    (*t).add_index = idx;
    (*t).add_single = single;
    (*t).add_cand = cand;
}
unsafe extern "C" fn build_added(
    mut t: *mut toks_tables,
    mut cfg: *const toks_config,
    mut ent: *mut toks_added_entry,
    mut ab: *mut uint8_t,
    mut shuf: *mut uint8_t,
    mut idx: *mut uint64_t,
    mut single: *mut uint32_t,
    mut cand: *mut uint32_t,
) {
    let mut shadowed: *const uint8_t = idx as *const ::core::ffi::c_void
        as *const uint8_t;
    let mut e: uint32_t = 0 as uint32_t;
    let mut off: uint64_t = 0 as uint64_t;
    let mut p: uint32_t = 0 as uint32_t;
    while p < 2 as uint32_t {
        let mut i: uint32_t = 0 as uint32_t;
        while i < (*cfg).n_added {
            let mut a: *const toks_cfg_added = (*cfg).added.offset(i as isize)
                as *mut toks_cfg_added;
            if !((*a).normalized as uint32_t != p
                || *shadowed.offset(i as isize) as ::core::ffi::c_uint
                    != 0 as ::core::ffi::c_uint)
            {
                let mut mb: *const uint8_t = if p != 0 as uint32_t
                    && !(*a).form.is_null()
                {
                    (*a).form
                } else {
                    (*a).content
                };
                let mut ml: uint32_t = if p != 0 as uint32_t && !(*a).form.is_null() {
                    (*a).form_len
                } else {
                    (*a).len
                };
                memset(
                    ent.offset(e as isize) as *mut toks_added_entry
                        as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    ::core::mem::size_of::<toks_added_entry>() as size_t,
                );
                (*ent.offset(e as isize)).off = off as uint32_t;
                (*ent.offset(e as isize)).len = ml as uint16_t;
                (*ent.offset(e as isize)).flags = ((if (*a).special
                    as ::core::ffi::c_uint != 0 as ::core::ffi::c_uint
                {
                    TOKS_AF_SPECIAL
                } else {
                    0 as ::core::ffi::c_uint
                })
                    | (if p != 0 as uint32_t {
                        TOKS_AF_NORMALIZED
                    } else {
                        0 as ::core::ffi::c_uint
                    })
                    | (if (*a).lstrip as ::core::ffi::c_uint != 0 as ::core::ffi::c_uint
                    {
                        TOKS_AF_LSTRIP
                    } else {
                        0 as ::core::ffi::c_uint
                    })
                    | (if (*a).rstrip as ::core::ffi::c_uint != 0 as ::core::ffi::c_uint
                    {
                        TOKS_AF_RSTRIP
                    } else {
                        0 as ::core::ffi::c_uint
                    })
                    | (if (*a).single_word as ::core::ffi::c_uint
                        != 0 as ::core::ffi::c_uint
                    {
                        TOKS_AF_SINGLE_WORD
                    } else {
                        0 as ::core::ffi::c_uint
                    })
                    | (if (*a).pfx as ::core::ffi::c_uint != 0 as ::core::ffi::c_uint {
                        TOKS_AF_PFX
                    } else {
                        0 as ::core::ffi::c_uint
                    })) as uint8_t;
                (*ent.offset(e as isize)).phase = p as uint8_t;
                (*ent.offset(e as isize)).id = (*a).id;
                memcpy(
                    ab.offset(off as isize) as *mut ::core::ffi::c_void,
                    mb as *const ::core::ffi::c_void,
                    ml as size_t,
                );
                off = off.wrapping_add(ml as uint64_t);
                e = e.wrapping_add(1);
            }
            i = i.wrapping_add(1);
        }
        p = p.wrapping_add(1);
    }
    (*t).add_entries = ent;
    (*t).add_bytes = ab;
    (*t).add_n = e as uint64_t;
    toks_added_index(t, shuf, idx, single, cand);
}
#[no_mangle]
pub unsafe extern "C" fn toks_compile(
    mut cfg: *const toks_config,
    mut ctx: *mut toks_ctx,
    mut parse_ar: *mut toks_arena,
) -> int64_t {
    let mut n_ids: uint32_t = (*cfg).n_ids;
    let mut n_vocab: uint32_t = (*cfg).n_vocab;
    let mut wp: ::core::ffi::c_int = ((*cfg).algo == TOKS_ALGO_WORDPIECE as uint32_t)
        as ::core::ffi::c_int;
    let mut tb: uint64_t = 0 as uint64_t;
    let mut id: uint32_t = 0 as uint32_t;
    while id < n_vocab {
        tb = tb
            .wrapping_add(
                vocab_bytes(
                    cfg as *const toks_config,
                    id,
                    ::core::ptr::null_mut::<uint8_t>(),
                ) as uint64_t,
            );
        id = id.wrapping_add(1);
    }
    let mut ab: uint64_t = 0 as uint64_t;
    let mut n_long: uint32_t = 0 as uint32_t;
    let mut n4: uint32_t = 0 as uint32_t;
    let mut b4: uint64_t = 0 as uint64_t;
    let mut i: uint32_t = 0 as uint32_t;
    while i < (*cfg).n_added {
        let mut a: *const toks_cfg_added = (*cfg).added.offset(i as isize)
            as *mut toks_cfg_added;
        let mut ml: uint32_t = if (*a).normalized as ::core::ffi::c_uint
            != 0 as ::core::ffi::c_uint && !(*a).form.is_null()
        {
            (*a).form_len
        } else {
            (*a).len
        };
        tb = tb
            .wrapping_add(
                added_bytes(
                    cfg as *const toks_config,
                    a,
                    ::core::ptr::null_mut::<uint8_t>(),
                ) as uint64_t,
            );
        ab = ab.wrapping_add(ml as uint64_t);
        n_long = (n_long as ::core::ffi::c_uint)
            .wrapping_add(
                if ml >= 2 as uint32_t {
                    1 as ::core::ffi::c_uint
                } else {
                    0 as ::core::ffi::c_uint
                },
            ) as uint32_t as uint32_t;
        n4 = (n4 as ::core::ffi::c_uint)
            .wrapping_add(
                if ml >= 4 as uint32_t {
                    1 as ::core::ffi::c_uint
                } else {
                    0 as ::core::ffi::c_uint
                },
            ) as uint32_t as uint32_t;
        b4 = b4
            .wrapping_add(
                (if ml >= 4 as uint32_t { ml } else { 0 as uint32_t }) as uint64_t,
            );
        i = i.wrapping_add(1);
    }
    if tb > 0xffffffff as uint64_t {
        return TOKS_E_LIMIT as int64_t;
    }
    let mut ctmp: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    let mut ctmp_len: uint64_t = 0 as uint64_t;
    let mut cls: uint64_t = 0 as uint64_t;
    let mut ct: toks_class_tables = toks_class_tables {
        ascii: ::core::ptr::null::<uint8_t>(),
        stage1: ::core::ptr::null::<uint16_t>(),
        stage2: ::core::ptr::null::<uint8_t>(),
        n_blocks: 0,
        _pad: 0,
    };
    memset(
        &raw mut ct as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<toks_class_tables>() as size_t,
    );
    if !(*cfg).pattern.is_null() {
        let mut cf: uint32_t = (*(*cfg).pattern).class_flags;
        ctmp_len = toks_classes_bytes(cf);
        ctmp = toks_plat_alloc(ctmp_len) as *mut uint8_t;
        if ctmp.is_null() {
            return TOKS_E_NOMEM as int64_t;
        }
        let mut used: ::core::ffi::c_int = toks_classes_build(
            cf,
            ctmp,
            ctmp_len,
            &raw mut ct,
        );
        if used <= 0 as ::core::ffi::c_int {
            toks_plat_free(ctmp as *mut ::core::ffi::c_void, ctmp_len);
            return TOKS_E_NOMEM as int64_t;
        }
        cls = used as uint64_t;
    }
    let mut o_bytes: uint64_t = up64(
        (n_ids as uint64_t).wrapping_add(1 as uint64_t).wrapping_mul(4 as uint64_t),
    );
    let mut o_cls: uint64_t = o_bytes.wrapping_add(up64(tb));
    let mut o_spec: uint64_t = o_cls.wrapping_add(up64(cls));
    let mut o_ent: uint64_t = o_spec
        .wrapping_add(
            up64(
                (n_ids as uint64_t)
                    .wrapping_add(63 as uint64_t)
                    .wrapping_div(64 as uint64_t)
                    .wrapping_mul(8 as uint64_t),
            ),
        );
    let mut o_ab: uint64_t = o_ent
        .wrapping_add(
            up64(
                ((*cfg).n_added as uint64_t)
                    .wrapping_mul(::core::mem::size_of::<toks_added_entry>() as uint64_t),
            ),
        );
    let mut o_shuf: uint64_t = o_ab.wrapping_add(up64(ab));
    let mut o_idx: uint64_t = o_shuf.wrapping_add(64 as uint64_t);
    let mut o_one: uint64_t = o_idx
        .wrapping_add(
            (2 as ::core::ffi::c_uint)
                .wrapping_mul(65536 as ::core::ffi::c_uint)
                .wrapping_mul(8 as ::core::ffi::c_uint) as uint64_t,
        )
        .wrapping_add(((16 as ::core::ffi::c_uint) << TOKS_K1_H4_BITS) as uint64_t);
    let mut o_cand: uint64_t = o_one
        .wrapping_add(
            (2 as ::core::ffi::c_uint)
                .wrapping_mul(256 as ::core::ffi::c_uint)
                .wrapping_mul(4 as ::core::ffi::c_uint) as uint64_t,
        );
    let mut total: uint64_t = if (*cfg).n_added != 0 as uint32_t {
        o_cand
            .wrapping_add(
                up64(
                    (4 as uint64_t)
                        .wrapping_mul(
                            toks_added_cand_words(n_long as uint64_t, n4 as uint64_t, b4),
                        ),
                ),
            )
    } else {
        o_ent
    };
    let mut mem: *mut uint8_t = toks_plat_arena(total);
    if mem.is_null() {
        if !ctmp.is_null() {
            toks_plat_free(ctmp as *mut ::core::ffi::c_void, ctmp_len);
        }
        return TOKS_E_NOMEM as int64_t;
    }
    let mut base: *mut uint8_t = mem;
    let mut t: *mut toks_tables = &raw mut (*ctx).t;
    memset(
        t as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<toks_tables>() as size_t,
    );
    (*t).magic = TOKS_TABLES_MAGIC as uint32_t;
    (*t).version = TOKS_TABLES_VERSION as uint32_t;
    (*t).algo = if (*cfg).algo == TOKS_ALGO_BPE_SPM as uint32_t
        || (*cfg).algo == TOKS_ALGO_WORDPIECE as uint32_t
        || (*cfg).algo == TOKS_ALGO_UNIGRAM as uint32_t
    {
        (*cfg).algo
    } else {
        TOKS_ALGO_BPE_BYTELEVEL as uint32_t
    };
    (*t).tmpl = if !(*cfg).pattern.is_null() {
        (*(*cfg).pattern).tmpl
    } else {
        TOKS_TMPL_NONE as uint32_t
    };
    (*t).tmpl_params = if !(*cfg).pattern.is_null() {
        (*(*cfg).pattern).params
    } else {
        0 as uint32_t
    };
    (*t).flags = (if (*cfg).ignore_merges as ::core::ffi::c_uint
        != 0 as ::core::ffi::c_uint
    {
        TOKS_TF_IGNORE_MERGES
    } else {
        0 as ::core::ffi::c_uint
    }) as uint32_t;
    (*t).n_ids = n_ids;
    let mut off: *mut uint32_t = toks_tab(
        base,
        0 as uint64_t,
        (n_ids as uint64_t).wrapping_add(1 as uint64_t).wrapping_mul(4 as uint64_t),
        TOKS_X_TOK_OFF,
    ) as *mut uint32_t;
    let mut bytes: *mut uint8_t = toks_tab(base, o_bytes, tb, TOKS_X_TOK_BYTES)
        as *mut uint8_t;
    let mut idx: *mut uint8_t = if (*cfg).n_added != 0 as uint32_t {
        toks_tab(base, o_idx, o_one.wrapping_sub(o_idx), TOKS_X_ADD_INDEX)
            as *mut uint8_t
    } else {
        ::core::ptr::null_mut::<uint8_t>()
    };
    memset(
        off as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        (n_ids as size_t).wrapping_add(1 as size_t).wrapping_mul(4 as size_t),
    );
    let mut id_0: uint32_t = 0 as uint32_t;
    while id_0 < n_vocab {
        *off.offset(id_0.wrapping_add(1 as uint32_t) as isize) = vocab_bytes(
            cfg as *const toks_config,
            id_0,
            ::core::ptr::null_mut::<uint8_t>(),
        );
        id_0 = id_0.wrapping_add(1);
    }
    let mut shadowed: *mut uint8_t = idx;
    if !shadowed.is_null() {
        memset(
            shadowed as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            (*cfg).n_added as size_t,
        );
    }
    let mut i_0: uint32_t = 0 as uint32_t;
    while i_0 < (*cfg).n_added {
        let mut a_0: *const toks_cfg_added = (*cfg).added.offset(i_0 as isize)
            as *mut toks_cfg_added;
        let mut o: *mut uint32_t = off
            .offset((*a_0).id.wrapping_add(1 as uint32_t) as isize) as *mut uint32_t;
        let mut form: uint32_t = if (*a_0).pfx as ::core::ffi::c_uint
            != 0 as ::core::ffi::c_uint
        {
            TOK_FORM as uint32_t
        } else {
            0 as uint32_t
        };
        let mut k: uint32_t = *o & 0xffff as uint32_t;
        if *o & TOK_OWN as uint32_t != 0 as uint32_t
            && ((*(*cfg).added.offset(k as isize)).len != (*a_0).len
                || memcmp(
                    (*(*cfg).added.offset(k as isize)).content
                        as *const ::core::ffi::c_void,
                    (*a_0).content as *const ::core::ffi::c_void,
                    (*a_0).len as size_t,
                ) != 0 as ::core::ffi::c_int)
        {
            *shadowed.offset(k as isize) = 1 as uint8_t;
        }
        if *o & TOK_OWN as uint32_t == 0 as uint32_t || form != 0 as uint32_t
            || *o & TOK_FORM as uint32_t == 0 as uint32_t
        {
            *o = TOK_OWN as uint32_t | form | i_0;
        }
        i_0 = i_0.wrapping_add(1);
    }
    let mut id_1: uint32_t = 0 as uint32_t;
    let mut at: uint32_t = 0 as uint32_t;
    while id_1 < n_ids {
        let mut v: uint32_t = *off.offset(id_1.wrapping_add(1 as uint32_t) as isize);
        if v & TOK_OWN as uint32_t != 0 as uint32_t {
            at = at
                .wrapping_add(
                    added_bytes(
                        cfg as *const toks_config,
                        (*cfg).added.offset((v & 0xffff as uint32_t) as isize)
                            as *mut toks_cfg_added,
                        bytes.offset(at as isize),
                    ),
                );
        } else if id_1 < n_vocab {
            at = at
                .wrapping_add(
                    vocab_bytes(
                        cfg as *const toks_config,
                        id_1,
                        bytes.offset(at as isize),
                    ),
                );
        }
        *off.offset(id_1.wrapping_add(1 as uint32_t) as isize) = at;
        id_1 = id_1.wrapping_add(1);
    }
    (*t).tok_off = off;
    (*t).tok_bytes = toks_tab_fit(
        bytes as *const ::core::ffi::c_void,
        *off.offset(n_ids as isize) as uint64_t,
        TOKS_X_TOK_BYTES,
    ) as *const uint8_t;
    if !ctmp.is_null() {
        let mut ca: *mut uint8_t = toks_tab(
            base,
            o_cls
                .wrapping_add(
                    ct.ascii.offset_from(ctmp) as ::core::ffi::c_long as uint64_t,
                ),
            128 as uint64_t,
            TOKS_X_CLS_ASCII,
        ) as *mut uint8_t;
        let mut c1: *mut uint8_t = toks_tab(
            base,
            o_cls
                .wrapping_add(
                    (ct.stage1 as *const uint8_t).offset_from(ctmp)
                        as ::core::ffi::c_long as uint64_t,
                ),
            (0x1100 as ::core::ffi::c_uint).wrapping_mul(2 as ::core::ffi::c_uint)
                as uint64_t,
            TOKS_X_CLS_STAGE1,
        ) as *mut uint8_t;
        let mut c2: *mut uint8_t = toks_tab(
            base,
            o_cls
                .wrapping_add(
                    ct.stage2.offset_from(ctmp) as ::core::ffi::c_long as uint64_t,
                ),
            (ct.n_blocks as uint64_t).wrapping_mul(256 as uint64_t),
            TOKS_X_CLS_STAGE2,
        ) as *mut uint8_t;
        memcpy(
            ca as *mut ::core::ffi::c_void,
            ct.ascii as *const ::core::ffi::c_void,
            128 as size_t,
        );
        memcpy(
            c1 as *mut ::core::ffi::c_void,
            ct.stage1 as *const ::core::ffi::c_void,
            (0x1100 as ::core::ffi::c_uint).wrapping_mul(2 as ::core::ffi::c_uint)
                as size_t,
        );
        memcpy(
            c2 as *mut ::core::ffi::c_void,
            ct.stage2 as *const ::core::ffi::c_void,
            (ct.n_blocks as size_t).wrapping_mul(256 as size_t),
        );
        (*t).cls_ascii = ca;
        (*t).cls_stage1 = c1 as *const ::core::ffi::c_void as *const uint16_t;
        (*t).cls_stage2 = c2;
        (*t).cls_nblocks = ct.n_blocks as uint64_t;
        toks_plat_free(ctmp as *mut ::core::ffi::c_void, ctmp_len);
        let mut cf_0: int64_t = toks_compile_cls_flags(t);
        (*t).flags = ((*t).flags as ::core::ffi::c_uint
            | (if cf_0 < 0 as int64_t {
                TOKS_TF_TWIN as uint32_t
            } else {
                cf_0 as uint32_t
            }) as ::core::ffi::c_uint) as uint32_t;
    }
    let mut spec_bytes: uint64_t = (n_ids as uint64_t)
        .wrapping_add(63 as uint64_t)
        .wrapping_div(64 as uint64_t)
        .wrapping_mul(8 as uint64_t);
    let mut spec: *mut uint8_t = toks_tab(base, o_spec, spec_bytes, TOKS_X_SPECIAL)
        as *mut uint8_t;
    memset(
        spec as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        spec_bytes as size_t,
    );
    let mut str: ::core::ffi::c_int = (wp != 0
        || (*cfg).algo == TOKS_ALGO_BPE_SPM as uint32_t
        || (*cfg).algo == TOKS_ALGO_UNIGRAM as uint32_t) as ::core::ffi::c_int;
    let mut i_1: uint32_t = 0 as uint32_t;
    while i_1 < (*cfg).n_added {
        let mut a_1: *const toks_cfg_added = (*cfg).added.offset(i_1 as isize)
            as *mut toks_cfg_added;
        let mut id_2: uint32_t = (*a_1).id;
        let mut o_0: uint32_t = *off.offset(id_2 as isize);
        let mut sl: uint32_t = (*off.offset(id_2.wrapping_add(1 as uint32_t) as isize))
            .wrapping_sub(o_0);
        let mut skip: ::core::ffi::c_int = ((*a_1).special as ::core::ffi::c_uint
            != 0 as ::core::ffi::c_uint) as ::core::ffi::c_int;
        if str != 0
            && (sl != (*a_1).len
                || memcmp(
                    bytes.offset(o_0 as isize) as *const ::core::ffi::c_void,
                    (*a_1).content as *const ::core::ffi::c_void,
                    sl as size_t,
                ) != 0 as ::core::ffi::c_int)
        {
            skip = 0 as ::core::ffi::c_int;
            let mut j: uint32_t = 0 as uint32_t;
            while j < (*cfg).n_added && skip == 0 {
                let mut b: *const toks_cfg_added = (*cfg).added.offset(j as isize)
                    as *mut toks_cfg_added;
                skip = ((*b).special as ::core::ffi::c_uint != 0 as ::core::ffi::c_uint
                    && (*b).len == sl
                    && memcmp(
                        (*b).content as *const ::core::ffi::c_void,
                        bytes.offset(o_0 as isize) as *const ::core::ffi::c_void,
                        sl as size_t,
                    ) == 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
                j = j.wrapping_add(1);
            }
        }
        if skip != 0 {
            let ref mut fresh0 = *spec
                .offset((id_2 >> 3 as ::core::ffi::c_int) as isize);
            *fresh0 = (*fresh0 as ::core::ffi::c_int
                | ((1 as ::core::ffi::c_uint) << (id_2 & 7 as uint32_t)) as uint8_t
                    as ::core::ffi::c_int) as uint8_t;
        }
        i_1 = i_1.wrapping_add(1);
    }
    let mut spec_words: *mut ::core::ffi::c_void = spec as *mut ::core::ffi::c_void;
    (*ctx).special_ids = spec_words as *mut uint32_t;
    if (*cfg).n_added != 0 as uint32_t {
        build_added(
            t,
            cfg as *const toks_config,
            toks_tab(
                base,
                o_ent,
                ((*cfg).n_added as uint64_t)
                    .wrapping_mul(
                        ::core::mem::size_of::<toks_added_entry>() as uint64_t,
                    ),
                TOKS_X_ADD_ENTRIES,
            ) as *mut toks_added_entry,
            toks_tab(base, o_ab, ab, TOKS_X_ADD_BYTES) as *mut uint8_t,
            toks_tab(base, o_shuf, 64 as uint64_t, TOKS_X_ADD_SHUFTI) as *mut uint8_t,
            idx as *mut ::core::ffi::c_void as *mut uint64_t,
            toks_tab(base, o_one, o_cand.wrapping_sub(o_one), TOKS_X_ADD_SINGLE)
                as *mut uint32_t,
            toks_tab(
                base,
                o_cand,
                (4 as uint64_t)
                    .wrapping_mul(
                        toks_added_cand_words(n_long as uint64_t, n4 as uint64_t, b4),
                    ),
                TOKS_X_ADD_CAND,
            ) as *mut uint32_t,
        );
    }
    toks_tab_seal(mem as *mut ::core::ffi::c_void, total);
    let mut k_0: uint32_t = 0 as uint32_t;
    memset(
        &raw mut (*ctx).pp_ids as *mut uint32_t as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<[uint32_t; 64]>() as size_t,
    );
    memset(
        &raw mut (*ctx).pp_type as *mut uint32_t as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<[uint32_t; 64]>() as size_t,
    );
    (*ctx).n_pp_prefix = 0 as ::core::ffi::c_uint as uint32_t;
    (*ctx).pp_seq_type = 0 as ::core::ffi::c_uint as uint32_t;
    let mut i_2: uint32_t = 0 as uint32_t;
    while i_2 < (*cfg).n_pp_single {
        if (*(*cfg).pp_single.offset(i_2 as isize)).kind
            == TOKS_PPS_TOK as ::core::ffi::c_int as uint32_t
        {
            (*ctx).pp_type[k_0 as usize] = (*(*cfg).pp_single.offset(i_2 as isize))
                .type_0;
            let fresh1 = k_0;
            k_0 = k_0.wrapping_add(1);
            (*ctx).pp_ids[fresh1 as usize] = (*(*cfg).pp_single.offset(i_2 as isize)).id;
        } else {
            (*ctx).n_pp_prefix = k_0;
            (*ctx).pp_seq_type = (*(*cfg).pp_single.offset(i_2 as isize)).type_0;
        }
        i_2 = i_2.wrapping_add(1);
    }
    (*ctx).n_pp_suffix = k_0.wrapping_sub((*ctx).n_pp_prefix);
    (*ctx).dec_byte_level = (*cfg).dec_byte_level as uint32_t;
    (*ctx).nfc = (*cfg).nfc as uint32_t;
    (*ctx).cut_chunk = (*cfg).cut_chunk;
    (*ctx).cut_run = (*cfg).cut_run;
    (*ctx).has_drop = (*cfg).has_drop;
    memcpy(
        &raw mut (*ctx).drop as *mut uint8_t as *mut ::core::ffi::c_void,
        &raw const (*cfg).drop as *const uint8_t as *const ::core::ffi::c_void,
        ::core::mem::size_of::<[uint8_t; 32]>() as size_t,
    );
    (*ctx).drop_unk = (*cfg).drop_unk;
    (*ctx).drop_fuse = (*cfg).drop_fuse;
    (*ctx).o = (*cfg).o;
    (*ctx).mem_tables = mem;
    (*ctx).mem_tables_len = total;
    return 0 as int64_t;
}
#[no_mangle]
pub unsafe extern "C" fn toks_tmpl_invalid(
    mut t: *const toks_tables,
) -> *const ::core::ffi::c_char {
    let mut p: uint32_t = (*t).tmpl_params;
    let mut han: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut dsv3: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if (*t).tmpl == TOKS_TMPL_NONE as uint32_t {
        return if p == 0 as uint32_t {
            ::core::ptr::null::<::core::ffi::c_char>()
        } else {
            b"template parameters: TOKS_TMPL_NONE takes none\0" as *const u8
                as *const ::core::ffi::c_char
        }
    } else if (*t).tmpl == TOKS_TMPL_CL100K as uint32_t {
        if p
            & !((0x7f as ::core::ffi::c_uint | TOKS_TP_DIGIT_CUT | TOKS_TP_NL_CUT
                | TOKS_TP_GB_SP) as uint32_t) != 0 as uint32_t
            || p & TOKS_TP_CONTR_MASK as uint32_t == TOKS_TP_CONTR_MASK as uint32_t
            || p & TOKS_TP_DIGIT_CUT as uint32_t != 0 as uint32_t
                && p & TOKS_TP_DIGITS_MASK as uint32_t != TOKS_TP_DIGITS_1 as uint32_t
                && p & TOKS_TP_DIGITS_MASK as uint32_t != TOKS_TP_DIGITS_1_3 as uint32_t
        {
            return b"template parameters: undefined for cl100k (kernels.md \xC2\xA73)\0"
                as *const u8 as *const ::core::ffi::c_char;
        }
    } else if (*t).tmpl == TOKS_TMPL_O200K as uint32_t {
        if p
            & !((TOKS_TP_CONTR_CI | TOKS_TP_DIGITS_1 | TOKS_TP_HAN | TOKS_TP_NO_SLASH)
                as uint32_t) != 0 as uint32_t
        {
            return b"template parameters: undefined for o200k (docs/templates/o200k.md \xC2\xA71, \xC2\xA76)\0"
                as *const u8 as *const ::core::ffi::c_char;
        }
        han = (p & TOKS_TP_HAN as uint32_t != 0 as uint32_t) as ::core::ffi::c_int;
    } else if (*t).tmpl == TOKS_TMPL_DSV3 as uint32_t {
        if p != 0 as uint32_t {
            return b"template parameters: dsv3 takes none (docs/templates/dsv3.md)\0"
                as *const u8 as *const ::core::ffi::c_char;
        }
        dsv3 = 1 as ::core::ffi::c_int;
    } else {
        return b"template: unknown\0" as *const u8 as *const ::core::ffi::c_char
    }
    if (*t).cls_ascii.is_null() || (*t).cls_stage1.is_null() || (*t).cls_stage2.is_null()
        || (*t).cls_nblocks == 0 as uint64_t || (*t).cls_nblocks > 0x1100 as uint64_t
    {
        return b"class tables: missing\0" as *const u8 as *const ::core::ffi::c_char;
    }
    let mut i: uint32_t = 0 as uint32_t;
    while i < 0x1100 as uint32_t {
        if *(*t).cls_stage1.offset(i as isize) as uint64_t >= (*t).cls_nblocks {
            return b"class tables: block index out of range\0" as *const u8
                as *const ::core::ffi::c_char;
        }
        i = i.wrapping_add(1);
    }
    let mut any: uint8_t = 0 as uint8_t;
    let mut i_0: uint32_t = 0 as uint32_t;
    while i_0 < 128 as uint32_t {
        any = (any as ::core::ffi::c_int
            | *(*t).cls_ascii.offset(i_0 as isize) as ::core::ffi::c_int) as uint8_t;
        i_0 = i_0.wrapping_add(1);
    }
    if any as ::core::ffi::c_uint & TOKS_C_HAN != 0 as ::core::ffi::c_uint {
        return b"class tables: Han bit on an ascii byte\0" as *const u8
            as *const ::core::ffi::c_char;
    }
    let mut i_1: uint64_t = 0 as uint64_t;
    while i_1 < (*t).cls_nblocks.wrapping_mul(256 as uint64_t) {
        any = (any as ::core::ffi::c_int
            | *(*t).cls_stage2.offset(i_1 as isize) as ::core::ffi::c_int) as uint8_t;
        i_1 = i_1.wrapping_add(1);
    }
    if dsv3 != 0 as ::core::ffi::c_int {
        if toks_cls_cp(t, 0x4e00 as uint32_t) as ::core::ffi::c_uint & TOKS_C_CJK
            == 0 as ::core::ffi::c_uint
            || *(*t).cls_ascii.offset(0 as ::core::ffi::c_int as isize)
                as ::core::ffi::c_uint & TOKS_C_BASE_MASK != TOKS_C_X
        {
            return b"class tables: dsv3 without its classes (TOKS_CLASSES_DSV3)\0"
                as *const u8 as *const ::core::ffi::c_char;
        }
        return ::core::ptr::null::<::core::ffi::c_char>();
    }
    if *(*t).cls_ascii.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint
        & TOKS_C_BASE_MASK == TOKS_C_X
    {
        return b"class tables: dsv3's outside dsv3\0" as *const u8
            as *const ::core::ffi::c_char;
    }
    if han != 0 as ::core::ffi::c_int
        && toks_cls_cp(t, 0x4e00 as uint32_t) as ::core::ffi::c_uint & TOKS_C_HAN
            == 0 as ::core::ffi::c_uint
    {
        return b"class tables: TOKS_TP_HAN without the Han bit (TOKS_CLASSES_HAN)\0"
            as *const u8 as *const ::core::ffi::c_char;
    }
    if han == 0 as ::core::ffi::c_int
        && any as ::core::ffi::c_uint & TOKS_C_HAN != 0 as ::core::ffi::c_uint
    {
        return b"class tables: the Han bit without TOKS_TP_HAN\0" as *const u8
            as *const ::core::ffi::c_char;
    }
    if (*t).tmpl == TOKS_TMPL_O200K as uint32_t
        && toks_cls_cp(t, 0x301 as uint32_t) as ::core::ffi::c_uint & TOKS_C_BASE_MASK
            != TOKS_C_P
    {
        return b"class tables: o200k with marks folded into letters\0" as *const u8
            as *const ::core::ffi::c_char;
    }
    return ::core::ptr::null::<::core::ffi::c_char>();
}
unsafe extern "C" fn bound_max(
    mut b: *mut toks_bound_terms,
    mut num: uint32_t,
    mut den: uint32_t,
    mut why: uint32_t,
) {
    if (num as uint64_t).wrapping_mul((*b).den as uint64_t)
        > ((*b).num as uint64_t).wrapping_mul(den as uint64_t)
    {
        (*b).num = num;
        (*b).den = den;
        (*b).why = why;
    }
}
unsafe extern "C" fn uni_has_meta(mut u: *const toks_uni) -> ::core::ffi::c_int {
    static mut M: [uint8_t; 3] = [
        0xe2 as ::core::ffi::c_uint as uint8_t,
        0x96 as ::core::ffi::c_uint as uint8_t,
        0x81 as ::core::ffi::c_uint as uint8_t,
    ];
    let mut node: uint32_t = 0 as uint32_t;
    let mut nb: uint32_t = (*(*u).cell.offset(0 as ::core::ffi::c_int as isize)).base;
    let mut n: uint32_t = if (*u).remap as ::core::ffi::c_int != 0 {
        1 as uint32_t
    } else {
        3 as uint32_t
    };
    let mut k: uint32_t = 0 as uint32_t;
    while k < n {
        let mut t: uint32_t = (nb & TOKS_UNI_BASE as uint32_t)
            .wrapping_add(
                (if (*u).remap as ::core::ffi::c_int != 0 {
                    0x20 as uint32_t
                } else {
                    M[k as usize] as uint32_t
                }),
            );
        if (*(*u).cell.offset(t as isize)).check != node as int32_t {
            return 0 as ::core::ffi::c_int;
        }
        node = t;
        nb = (*(*u).cell.offset(t as isize)).base;
        k = k.wrapping_add(1);
    }
    return (nb & TOKS_UNI_TERM as uint32_t != 0 as uint32_t) as ::core::ffi::c_int;
}
unsafe extern "C" fn utf8_chars(mut s: *const uint8_t, mut n: uint32_t) -> uint32_t {
    let mut k: uint32_t = 0 as uint32_t;
    let mut i: uint32_t = 0 as uint32_t;
    while i < n {
        k = k
            .wrapping_add(
                (*s.offset(i as isize) as ::core::ffi::c_uint
                    & 0xc0 as ::core::ffi::c_uint != 0x80 as ::core::ffi::c_uint)
                    as ::core::ffi::c_int as uint32_t,
            );
        i = i.wrapping_add(1);
    }
    return k;
}
unsafe extern "C" fn pc_chars_x(
    mut pc: *const toks_pc,
    mut num: *mut uint32_t,
    mut den: *mut uint32_t,
) {
    *num = 1 as ::core::ffi::c_uint as uint32_t;
    *den = 1 as ::core::ffi::c_uint as uint32_t;
    let mut hi: uint32_t = 0 as uint32_t;
    while hi < 0x1100 as uint32_t {
        let mut blk: uint32_t = *(*pc).stage1.offset(hi as isize) as uint32_t;
        if !(blk == 0 as uint32_t) {
            let mut kb: uint32_t = if hi < 0x8 as uint32_t {
                if hi == 0 as uint32_t { 1 as uint32_t } else { 2 as uint32_t }
            } else if hi < 0x100 as uint32_t {
                3 as uint32_t
            } else {
                4 as uint32_t
            };
            let mut lo: uint32_t = 0 as uint32_t;
            while lo < 256 as uint32_t {
                let mut e: uint32_t = *(*pc)
                    .stage2
                    .offset(blk.wrapping_mul(256 as uint32_t).wrapping_add(lo) as isize);
                if !(e == 0 as uint32_t) {
                    let mut c: uint32_t = utf8_chars(
                        (*pc)
                            .pool
                            .offset(
                                (e >> 12 as ::core::ffi::c_int & 0x3ffff as uint32_t)
                                    as isize,
                            ),
                        e & 0xfff as uint32_t,
                    );
                    let mut k: uint32_t = if hi == 0 as uint32_t
                        && lo >= 0x80 as uint32_t
                    {
                        2 as uint32_t
                    } else {
                        kb
                    };
                    if (c as uint64_t).wrapping_mul(*den as uint64_t)
                        > (*num as uint64_t).wrapping_mul(k as uint64_t)
                    {
                        *num = c;
                        *den = k;
                    }
                }
                lo = lo.wrapping_add(1);
            }
        }
        hi = hi.wrapping_add(1);
    }
    let mut i: uint64_t = 0 as uint64_t;
    while (*pc).mk_mask != 0 as uint32_t && i <= (*pc).mk_mask as uint64_t {
        let mut key: uint64_t = *(*pc).mk_key.offset(i as isize);
        if !(key == 0 as uint64_t) {
            let mut e_0: uint32_t = *(*pc).mk_val.offset(i as isize);
            let mut c_0: uint32_t = utf8_chars(
                (*pc)
                    .pool
                    .offset(
                        (e_0 >> 12 as ::core::ffi::c_int & 0x3ffff as uint32_t) as isize,
                    ),
                e_0 & 0xfff as uint32_t,
            );
            let mut k_0: uint32_t = (key >> 56 as ::core::ffi::c_int) as uint32_t;
            if k_0 != 0 as uint32_t
                && (c_0 as uint64_t).wrapping_mul(*den as uint64_t)
                    > (*num as uint64_t).wrapping_mul(k_0 as uint64_t)
            {
                *num = c_0;
                *den = k_0;
            }
        }
        i = i.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn toks_bound_terms_of(
    mut c: *const toks_ctx,
    mut b: *mut toks_bound_terms,
) {
    memset(
        b as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<toks_bound_terms>() as size_t,
    );
    (*b).x = 1 as ::core::ffi::c_uint as uint32_t;
    (*b).xd = 1 as ::core::ffi::c_uint as uint32_t;
    (*b).f = 1 as ::core::ffi::c_uint as uint32_t;
    if !(*c).spm.is_null() {
        let mut m: uint32_t = (*(*c).spm).pfx_mode;
        (*b).p0 = (if m == TOKS_SPM_PFX_GAP as ::core::ffi::c_int as uint32_t
            || m == TOKS_SPM_PFX_ALWAYS as ::core::ffi::c_int as uint32_t
        {
            1 as ::core::ffi::c_uint
        } else {
            0 as ::core::ffi::c_uint
        }) as uint32_t;
        (*b).p1 = (if m == TOKS_SPM_PFX_ALWAYS as ::core::ffi::c_int as uint32_t {
            1 as ::core::ffi::c_uint
        } else {
            0 as ::core::ffi::c_uint
        }) as uint32_t;
        (*b).pfirst = (if m != TOKS_SPM_PFX_NONE as ::core::ffi::c_int as uint32_t {
            1 as ::core::ffi::c_uint
        } else {
            0 as ::core::ffi::c_uint
        }) as uint32_t;
    } else if !(*c).uni.is_null() {
        let mut u: *const toks_uni = (*c).uni as *const toks_uni;
        let mut meta: ::core::ffi::c_int = uni_has_meta(u);
        let mut p: uint32_t = if meta != 0 {
            1 as uint32_t
        } else if (*u).byte_fallback as ::core::ffi::c_int != 0 {
            3 as uint32_t
        } else {
            1 as uint32_t
        };
        let mut ms: uint32_t = if (*u).cfg.metaspace as ::core::ffi::c_int != 0
            && (*u).cfg.meta_prepend as ::core::ffi::c_int != 0
        {
            p
        } else {
            0 as uint32_t
        };
        (*b).x = (*u).pre_x.wrapping_mul((*u).work_x);
        if (*u).byte_fallback == 0 && (*u).cfg.has_charsmap as ::core::ffi::c_int != 0 {
            let mut cn: uint32_t = 0;
            let mut cd: uint32_t = 0;
            pc_chars_x(&raw const (*u).pc, &raw mut cn, &raw mut cd);
            (*b).x = (*u).pre_x.wrapping_mul(cn);
            (*b).xd = cd;
        }
        (*b).f = (if (*u).byte_fallback as ::core::ffi::c_int != 0 && meta == 0 {
            3 as ::core::ffi::c_uint
        } else {
            1 as ::core::ffi::c_uint
        }) as uint32_t;
        (*b).p1 = ms;
        (*b).p0 = ms
            .wrapping_add(
                (if (*u).cfg.meta_prefix as ::core::ffi::c_int != 0 {
                    (if p > (1 as uint32_t).wrapping_add(ms) {
                        p
                    } else {
                        (1 as uint32_t).wrapping_add(ms)
                    })
                } else {
                    0 as uint32_t
                }),
            );
        (*b).pfirst = (*b).p0;
    } else if (*c).wp.is_null() {
        (*b).x = (if (*c).nfc != 0 as uint32_t {
            if (*c).nfc & TOKS_NS_COMPAT as uint32_t != 0 as uint32_t {
                TOKS_NFKC_X
            } else {
                TOKS_NFC_X
            }
        } else {
            1 as ::core::ffi::c_uint
        }) as uint32_t;
    }
    (*b).num = (*b).x.wrapping_mul((*b).f);
    (*b).den = (*b).xd;
    (*b).why = TOKS_BOUND_TEXT as ::core::ffi::c_int as uint32_t;
    let mut i: uint32_t = 0 as uint32_t;
    while (i as uint64_t) < (*c).t.add_n {
        let mut a: *const toks_added_entry = (*c).t.add_entries.offset(i as isize)
            as *const toks_added_entry;
        let mut l: uint32_t = (*a).len as uint32_t;
        if (*a).phase as ::core::ffi::c_uint != 0 as ::core::ffi::c_uint {
            l = utf8_chars(
                (*c).t.add_bytes.offset((*a).off as isize),
                (*a).len as uint32_t,
            );
            l = (l as ::core::ffi::c_uint)
                .wrapping_sub(
                    if !(*c).spm.is_null()
                        && (*a).flags as ::core::ffi::c_uint & TOKS_AF_PFX
                            != 0 as ::core::ffi::c_uint && l > 1 as uint32_t
                    {
                        1 as ::core::ffi::c_uint
                    } else {
                        0 as ::core::ffi::c_uint
                    },
                ) as uint32_t as uint32_t;
            if (*b).l1 == 0 as uint32_t || l < (*b).l1 {
                (*b).l1 = l;
                (*b).e1 = i;
            }
        } else if (*b).l0 == 0 as uint32_t || l < (*b).l0 {
            (*b).l0 = l;
            (*b).e0 = i;
        }
        i = i.wrapping_add(1);
    }
    if (*b).l0 != 0 as uint32_t {
        bound_max(
            b,
            (1 as uint32_t).wrapping_add((*b).p0),
            (*b).l0,
            TOKS_BOUND_PHASE0 as ::core::ffi::c_int as uint32_t,
        );
    }
    if (*b).l1 != 0 as uint32_t {
        bound_max(
            b,
            (*b).x.wrapping_mul((1 as uint32_t).wrapping_add((*b).p1)),
            (*b).xd.wrapping_mul((*b).l1),
            TOKS_BOUND_PHASE1 as ::core::ffi::c_int as uint32_t,
        );
    }
    (*b).g = (*c).n_pp_prefix.wrapping_add((*c).n_pp_suffix).wrapping_add((*b).pfirst);
}
