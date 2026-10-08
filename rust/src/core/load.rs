#[repr(C)]
pub struct toks_gen { _opaque: [u8; 0] }
#[repr(C)]
pub struct toks_uni { _opaque: [u8; 0] }
#[repr(C)]
pub struct toks_uni_src { _opaque: [u8; 0] }
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
    fn toks_plat_arena_free(p: *mut uint8_t, n: uint64_t);
    fn toks_plat_getenv(
        name: *const ::core::ffi::c_char,
        buf: *mut ::core::ffi::c_char,
        cap: uint64_t,
    ) -> int64_t;
    fn toks_plat_read_file(
        path: *const ::core::ffi::c_char,
        out: *mut *mut uint8_t,
        len: *mut uint64_t,
        is_dir: *mut ::core::ffi::c_int,
    ) -> int64_t;
    fn toks_plat_dir_lookup(
        dir: *const ::core::ffi::c_char,
        buf: *mut ::core::ffi::c_char,
        cap: uint64_t,
    ) -> int64_t;
    fn toks_dec_build(c: *mut toks_ctx) -> int64_t;
    fn toks_dec_free(c: *mut toks_ctx);
    fn toks_config_parse(
        data: *const uint8_t,
        len: uint64_t,
        ar: *mut toks_arena,
        cfg: *mut toks_config,
        err: *mut toks_err,
    ) -> int64_t;
    fn toks_gen_max_bytes() -> uint64_t;
    fn toks_compile(
        cfg: *const toks_config,
        ctx: *mut toks_ctx,
        parse_ar: *mut toks_arena,
    ) -> int64_t;
    fn toks_vocab_build(c: *mut toks_ctx, cfg: *const toks_config) -> int64_t;
    fn toks_gen_scr(g: *const toks_gen) -> uint64_t;
    fn toks_sha256(data: *const uint8_t, len: uint64_t, out: *mut uint8_t);
    fn toks_bpe_tables_bytes(cfg: *const toks_config) -> uint64_t;
    fn toks_bpe_build(
        t: *mut toks_tables,
        ar: *mut toks_arena,
        cfg: *const toks_config,
    ) -> int64_t;
    fn toks_uni_load(
        c: *mut toks_ctx,
        cfg: *const toks_config,
        why: *mut *const ::core::ffi::c_char,
    ) -> int64_t;
    fn toks_tmpl_invalid(t: *const toks_tables) -> *const ::core::ffi::c_char;
    fn toks_bound_terms_of(c: *const toks_ctx, b: *mut toks_bound_terms);
    fn toks_cpu_features() -> uint64_t;
    fn toks_spm_build(
        t: *mut toks_tables,
        cfg: *const toks_spm_config,
        mem: *mut *mut uint8_t,
        mem_len: *mut uint64_t,
        out: *mut *const toks_spm,
        err: *mut toks_err,
    ) -> int64_t;
    fn toks_tiktoken_arena_bound(
        ranks_len: uint64_t,
        config_len: uint64_t,
        wrapper_len: uint64_t,
    ) -> uint64_t;
    fn toks_tiktoken_parse(
        ranks: *const uint8_t,
        ranks_len: uint64_t,
        config: *const uint8_t,
        config_len: uint64_t,
        wrapper: *const uint8_t,
        wrapper_len: uint64_t,
        ar: *mut toks_arena,
        cfg: *mut toks_config,
        info: *mut toks_tiktoken_info,
        err: *mut toks_err,
    ) -> int64_t;
    fn toks_tiktoken_sniff(data: *const uint8_t, len: uint64_t) -> ::core::ffi::c_int;
    fn toks_wp_ctx_build(
        cfg: *const toks_config,
        par: *mut toks_arena,
        mem: *mut *mut uint8_t,
        mem_len: *mut uint64_t,
        out: *mut *const toks_wp_tables,
        err: *mut toks_err,
    ) -> int64_t;
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
pub struct toks_diag {
    pub code: int64_t,
    pub what: [::core::ffi::c_char; 248],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct toks_load_opts {
    pub size: uint32_t,
    pub tier: uint32_t,
    pub flags: uint32_t,
    pub rsv: uint32_t,
    pub diag: *mut toks_diag,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct toks_err {
    pub code: int64_t,
    pub what: *const ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct toks_tiktoken_info {
    pub n_ranks: uint32_t,
    pub first_special: uint32_t,
    pub chunk_chars: uint32_t,
    pub run_chars: uint32_t,
    pub n_trie: uint32_t,
    pub n_named: uint32_t,
    pub trie: [uint8_t; 32],
}
pub const TOKS_E_OPEN: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
pub const TOKS_E_FORMAT: ::core::ffi::c_int = -(2 as ::core::ffi::c_int);
pub const TOKS_E_UNSUPPORTED: ::core::ffi::c_int = -(3 as ::core::ffi::c_int);
pub const TOKS_E_TIER: ::core::ffi::c_int = -(5 as ::core::ffi::c_int);
pub const TOKS_E_LIMIT: ::core::ffi::c_int = -(9 as ::core::ffi::c_int);
pub const TOKS_E_ARG: ::core::ffi::c_int = -(10 as ::core::ffi::c_int);
pub const TOKS_E_NOMEM: ::core::ffi::c_int = -(11 as ::core::ffi::c_int);
pub const TOKS_MAX_SOURCE_BYTES: ::core::ffi::c_ulonglong = (256
    as ::core::ffi::c_ulonglong) << 20 as ::core::ffi::c_int;
pub const TOKS_TIER_AUTO: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
pub const TOKS_TIER_SCALAR: ::core::ffi::c_uint = 1 as ::core::ffi::c_uint;
pub const TOKS_TIER_NEON: ::core::ffi::c_uint = 2 as ::core::ffi::c_uint;
pub const TOKS_TIER_AVX2: ::core::ffi::c_uint = 3 as ::core::ffi::c_uint;
pub const TOKS_TIER_AVX512: ::core::ffi::c_uint = 4 as ::core::ffi::c_uint;
pub const TOKS_ALGO_BPE_SPM: ::core::ffi::c_uint = 2 as ::core::ffi::c_uint;
pub const TOKS_ALGO_UNIGRAM: ::core::ffi::c_uint = 3 as ::core::ffi::c_uint;
pub const TOKS_ALGO_WORDPIECE: ::core::ffi::c_uint = 4 as ::core::ffi::c_uint;
pub const TOKS_TABLES_VERSION: ::core::ffi::c_uint = 2 as ::core::ffi::c_uint;
pub const TOKS_CHUNK_PIECES: ::core::ffi::c_int = 256 as ::core::ffi::c_int;
pub const TOKS_TF_ASM_MERGE: ::core::ffi::c_uint = 0x40 as ::core::ffi::c_uint;
pub const TOKS_AF_SPECIAL: ::core::ffi::c_uint = 0x1 as ::core::ffi::c_uint;
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
unsafe extern "C" fn toks_ctx_identity(mut sha256: *const uint8_t) -> uint64_t {
    let mut h: uint64_t = (TOKS_TABLES_VERSION as uint64_t) << 56 as ::core::ffi::c_int;
    let mut i: uint32_t = 0 as uint32_t;
    while i < 8 as uint32_t {
        h ^= (*sha256.offset(i as isize) as uint64_t) << (8 as uint32_t).wrapping_mul(i);
        i = i.wrapping_add(1);
    }
    return h | 1 as uint64_t;
}
pub const TOKS_WP_MAT_CAP: ::core::ffi::c_uint = 24704 as ::core::ffi::c_uint;
pub const TOKS_WP_SCR_EXTRA: uint64_t = (16 as uint64_t)
    .wrapping_mul(TOKS_CHUNK_PIECES as uint64_t)
    .wrapping_add(TOKS_WP_MAT_CAP as uint64_t)
    .wrapping_add(512 as uint64_t);
#[inline]
unsafe extern "C" fn toks_config_arena_bound(mut len: uint64_t) -> uint64_t {
    return (32 as uint64_t)
        .wrapping_mul(len)
        .wrapping_add(65536 as uint64_t)
        .wrapping_add(toks_gen_max_bytes());
}
pub const TOKS_HAVE_K1_AVX512: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TOKS_HAVE_K3_CL100K_AVX512: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TOKS_HAVE_K3_O200K_AVX512: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TOKS_HAVE_K3_DSV3_AVX512: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TOKS_HAVE_K5_AVX512: ::core::ffi::c_int = cfg!(all(feature = "avx512", target_arch = "x86_64")) as ::core::ffi::c_int;
pub const TOKS_HAVE_NEON: ::core::ffi::c_int = TOKS_HAVE_K1_NEON
    | TOKS_HAVE_K3_CL100K_NEON | TOKS_HAVE_K3_O200K_NEON | TOKS_HAVE_K3_DSV3_NEON
    | TOKS_HAVE_K5_NEON | TOKS_HAVE_K7_SPM_NEON;
pub const TOKS_HAVE_AVX2: ::core::ffi::c_int = TOKS_HAVE_K1_AVX2
    | TOKS_HAVE_K3_CL100K_AVX2 | TOKS_HAVE_K3_O200K_AVX2 | TOKS_HAVE_K3_DSV3_AVX2
    | TOKS_HAVE_K5_AVX2 | TOKS_HAVE_K7_SPM_AVX2;
pub const TOKS_HAVE_AVX512: ::core::ffi::c_int = TOKS_HAVE_K1_AVX512
    | TOKS_HAVE_K3_CL100K_AVX512 | TOKS_HAVE_K3_O200K_AVX512 | TOKS_HAVE_K3_DSV3_AVX512
    | TOKS_HAVE_K5_AVX512;
unsafe extern "C" fn ctx_free(mut c: *mut toks_ctx) {
    if c.is_null() {
        return;
    }
    toks_dec_free(c as *mut toks_ctx);
    toks_tab_free((*c).mem_tables, (*c).mem_tables_len);
    toks_tab_free((*c).mem_bpe, (*c).mem_bpe_len);
    toks_tab_free((*c).mem_spm, (*c).mem_spm_len);
    toks_tab_free((*c).mem_wp, (*c).mem_wp_len);
    toks_tab_free((*c).mem_uni, (*c).mem_uni_len);
    toks_tab_free((*c).mem_voc, (*c).mem_voc_len);
    if !(*c).mem_gen.is_null() {
        toks_plat_free((*c).mem_gen as *mut ::core::ffi::c_void, (*c).mem_gen_len);
    }
    toks_plat_free(
        c as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<toks_ctx>() as uint64_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn toks_unload(mut ctx: *mut toks_ctx) {
    ctx_free(ctx);
}
unsafe extern "C" fn fail(
    mut o: *const toks_load_opts,
    mut code: int64_t,
    mut what: *const ::core::ffi::c_char,
) -> int64_t {
    if !o.is_null() && !(*o).diag.is_null() {
        (*(*o).diag).code = code;
        let mut i: uint64_t = 0 as uint64_t;
        if !what.is_null() {
            while *what.offset(i as isize) as ::core::ffi::c_int
                != 0 as ::core::ffi::c_int
                && i
                    < (::core::mem::size_of::<[::core::ffi::c_char; 248]>() as uint64_t)
                        .wrapping_sub(1 as uint64_t)
            {
                (*(*o).diag).what[i as usize] = *what.offset(i as isize);
                i = i.wrapping_add(1);
            }
        }
        (*(*o).diag).what[i as usize] = 0 as ::core::ffi::c_char;
    }
    return code;
}
unsafe extern "C" fn str_eq(
    mut a: *const ::core::ffi::c_char,
    mut b: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut i: uint64_t = 0 as uint64_t;
    while *a.offset(i as isize) as ::core::ffi::c_int != 0 as ::core::ffi::c_int
        && *a.offset(i as isize) as ::core::ffi::c_int
            == *b.offset(i as isize) as ::core::ffi::c_int && i < 16 as uint64_t
    {
        i = i.wrapping_add(1);
    }
    return (*a.offset(i as isize) as ::core::ffi::c_int
        == *b.offset(i as isize) as ::core::ffi::c_int) as ::core::ffi::c_int;
}
unsafe extern "C" fn pick_tier(
    mut want: uint32_t,
    mut f: uint64_t,
    mut why: *mut *const ::core::ffi::c_char,
) -> int64_t {
    if want == TOKS_TIER_AUTO as uint32_t {
        let mut e: [::core::ffi::c_char; 16] = [0; 16];
        let mut el: int64_t = toks_plat_getenv(
            b"TOKS_TIER\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut e as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 16]>() as uint64_t,
        );
        if el == -(2 as ::core::ffi::c_int) as int64_t
            || el > 0 as int64_t
                && str_eq(
                    &raw mut e as *mut ::core::ffi::c_char,
                    b"auto\0" as *const u8 as *const ::core::ffi::c_char,
                ) == 0
        {
            if el > 0 as int64_t
                && str_eq(
                    &raw mut e as *mut ::core::ffi::c_char,
                    b"scalar\0" as *const u8 as *const ::core::ffi::c_char,
                ) != 0
            {
                want = TOKS_TIER_SCALAR as uint32_t;
            } else if el > 0 as int64_t
                && str_eq(
                    &raw mut e as *mut ::core::ffi::c_char,
                    b"neon\0" as *const u8 as *const ::core::ffi::c_char,
                ) != 0
            {
                want = TOKS_TIER_NEON as uint32_t;
            } else if el > 0 as int64_t
                && str_eq(
                    &raw mut e as *mut ::core::ffi::c_char,
                    b"avx2\0" as *const u8 as *const ::core::ffi::c_char,
                ) != 0
            {
                want = TOKS_TIER_AVX2 as uint32_t;
            } else if el > 0 as int64_t
                && str_eq(
                    &raw mut e as *mut ::core::ffi::c_char,
                    b"avx512\0" as *const u8 as *const ::core::ffi::c_char,
                ) != 0
            {
                want = TOKS_TIER_AVX512 as uint32_t;
            } else {
                *why = b"TOKS_TIER: not one of auto, scalar, neon, avx2, avx512\0"
                    as *const u8 as *const ::core::ffi::c_char;
                return TOKS_E_TIER as int64_t;
            }
        }
    }
    let mut neon: ::core::ffi::c_int = (TOKS_HAVE_NEON != 0
        && f as ::core::ffi::c_ulonglong
            & ((1 as ::core::ffi::c_ulonglong) << 16 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_ulonglong) << 17 as ::core::ffi::c_int)
            == (1 as ::core::ffi::c_ulonglong) << 16 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_ulonglong) << 17 as ::core::ffi::c_int)
        as ::core::ffi::c_int;
    let mut avx2: ::core::ffi::c_int = (TOKS_HAVE_AVX2 != 0
        && f as ::core::ffi::c_ulonglong
            & ((1 as ::core::ffi::c_ulonglong) << 2 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_ulonglong) << 3 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_ulonglong) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_ulonglong) << 5 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_ulonglong) << 1 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_ulonglong) << 0 as ::core::ffi::c_int)
            == (1 as ::core::ffi::c_ulonglong) << 2 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_ulonglong) << 3 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_ulonglong) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_ulonglong) << 5 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_ulonglong) << 1 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_ulonglong) << 0 as ::core::ffi::c_int)
        as ::core::ffi::c_int;
    let mut avx512: ::core::ffi::c_int = (TOKS_HAVE_AVX512 != 0
        && f as ::core::ffi::c_ulonglong
            & ((1 as ::core::ffi::c_ulonglong) << 2 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_ulonglong) << 3 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_ulonglong) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_ulonglong) << 5 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_ulonglong) << 1 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_ulonglong) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_ulonglong) << 6 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_ulonglong) << 7 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_ulonglong) << 8 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_ulonglong) << 9 as ::core::ffi::c_int)
            == (1 as ::core::ffi::c_ulonglong) << 2 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_ulonglong) << 3 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_ulonglong) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_ulonglong) << 5 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_ulonglong) << 1 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_ulonglong) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_ulonglong) << 6 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_ulonglong) << 7 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_ulonglong) << 8 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_ulonglong) << 9 as ::core::ffi::c_int)
        as ::core::ffi::c_int;
    match want {
        TOKS_TIER_AUTO => {
            return (if avx512 != 0 {
                TOKS_TIER_AVX512
            } else if avx2 != 0 {
                TOKS_TIER_AVX2
            } else if neon != 0 {
                TOKS_TIER_NEON
            } else {
                TOKS_TIER_SCALAR
            }) as int64_t;
        }
        TOKS_TIER_SCALAR => return TOKS_TIER_SCALAR as int64_t,
        TOKS_TIER_NEON => {
            if neon != 0 {
                return TOKS_TIER_NEON as int64_t;
            }
            *why = b"tier neon: not on this cpu or not in this build\0" as *const u8
                as *const ::core::ffi::c_char;
            return TOKS_E_TIER as int64_t;
        }
        TOKS_TIER_AVX2 => {
            if avx2 != 0 {
                return TOKS_TIER_AVX2 as int64_t;
            }
            *why = b"tier avx2: not on this cpu or not in this build\0" as *const u8
                as *const ::core::ffi::c_char;
            return TOKS_E_TIER as int64_t;
        }
        TOKS_TIER_AVX512 => {
            if avx512 != 0 {
                return TOKS_TIER_AVX512 as int64_t;
            }
            *why = b"tier avx512: not on this cpu or not in this build\0" as *const u8
                as *const ::core::ffi::c_char;
            return TOKS_E_TIER as int64_t;
        }
        _ => {
            *why = b"tier: unknown value\0" as *const u8 as *const ::core::ffi::c_char;
            return TOKS_E_TIER as int64_t;
        }
    };
}
unsafe extern "C" fn ctx_new(
    mut name: *const ::core::ffi::c_char,
    mut plen: uint64_t,
    mut parse: *mut *mut uint8_t,
) -> *mut toks_ctx {
    *parse = toks_plat_alloc(plen) as *mut uint8_t;
    let mut c: *mut toks_ctx = toks_plat_alloc(
        ::core::mem::size_of::<toks_ctx>() as uint64_t,
    ) as *mut toks_ctx;
    if !c.is_null() {
        memset(
            c as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<toks_ctx>() as size_t,
        );
    }
    let mut i: uint32_t = 0 as uint32_t;
    while !c.is_null()
        && *name.offset(i as isize) as ::core::ffi::c_int != 0 as ::core::ffi::c_int
        && i < 63 as uint32_t
    {
        (*c).name[i as usize] = *name.offset(i as isize);
        i = i.wrapping_add(1);
    }
    return c;
}
unsafe extern "C" fn build_cfg(
    mut c: *mut toks_ctx,
    mut cfg: *const toks_config,
    mut par: *mut toks_arena,
    mut o: *const toks_load_opts,
    mut why: *mut *const ::core::ffi::c_char,
) -> int64_t {
    *why = b"compile\0" as *const u8 as *const ::core::ffi::c_char;
    let mut r: int64_t = toks_compile(
        cfg as *const toks_config,
        c as *mut toks_ctx,
        par,
    );
    if r != 0 as int64_t {
        return r;
    }
    let mut bad: *const ::core::ffi::c_char = toks_tmpl_invalid(&raw mut (*c).t);
    if !bad.is_null() {
        *why = bad;
        return TOKS_E_UNSUPPORTED as int64_t;
    }
    if (*cfg).algo == TOKS_ALGO_WORDPIECE as uint32_t {
        *why = b"wordpiece tables\0" as *const u8 as *const ::core::ffi::c_char;
        let mut werr: toks_err = toks_err {
            code: 0 as int64_t,
            what: ::core::ptr::null::<::core::ffi::c_char>(),
        };
        r = toks_wp_ctx_build(
            cfg as *const toks_config,
            par,
            &raw mut (*c).mem_wp,
            &raw mut (*c).mem_wp_len,
            &raw mut (*c).wp,
            &raw mut werr,
        );
        if r != 0 as int64_t {
            if !werr.what.is_null() {
                *why = werr.what;
            }
            return r;
        }
        (*c).wp_mat_cap = TOKS_WP_MAT_CAP as uint64_t;
        (*c).scr_extra = TOKS_WP_SCR_EXTRA;
    } else if (*cfg).algo == TOKS_ALGO_UNIGRAM as uint32_t {
        r = toks_uni_load(c as *mut toks_ctx, cfg as *const toks_config, why);
        if r != 0 as int64_t {
            return r;
        }
    } else if (*cfg).algo == TOKS_ALGO_BPE_SPM as uint32_t {
        let mut e2: toks_err = toks_err {
            code: 0 as int64_t,
            what: ::core::ptr::null::<::core::ffi::c_char>(),
        };
        r = toks_spm_build(
            &raw mut (*c).t,
            (*cfg).spm as *const toks_spm_config,
            &raw mut (*c).mem_spm,
            &raw mut (*c).mem_spm_len,
            &raw mut (*c).spm,
            &raw mut e2,
        );
        if r != 0 as int64_t {
            *why = if !e2.what.is_null() {
                e2.what
            } else {
                b"spm tables\0" as *const u8 as *const ::core::ffi::c_char
            };
            return r;
        }
        (*c).dc = toks_dchain {
            dec: &raw const (*(*c).spm).dec as *const toks_spm_op,
            n_dec: (*(*c).spm).n_dec,
            has_decoder: (*(*c).spm).has_decoder,
            bf_first: 0 as uint32_t,
            on: 1 as uint32_t,
            holes: (*(*c).spm).holes,
        };
    } else {
        *why = b"bpe tables\0" as *const u8 as *const ::core::ffi::c_char;
        (*c).mem_bpe_len = toks_bpe_tables_bytes(cfg as *const toks_config);
        if (*c).mem_bpe_len != 0 as uint64_t {
            (*c).mem_bpe = toks_plat_arena((*c).mem_bpe_len);
            if (*c).mem_bpe.is_null() {
                (*c).mem_bpe_len = 0 as uint64_t;
                return TOKS_E_NOMEM as int64_t;
            }
        }
        let mut bar: toks_arena = toks_arena {
            base: (*c).mem_bpe,
            len: (*c).mem_bpe_len,
            pos: 0 as uint64_t,
        };
        r = toks_bpe_build(&raw mut (*c).t, &raw mut bar, cfg as *const toks_config);
        if r != 0 as int64_t {
            return r;
        }
        toks_tab_seal((*c).mem_bpe as *mut ::core::ffi::c_void, (*c).mem_bpe_len);
        if !(*cfg).gen.is_null() {
            (*c).mem_gen = toks_plat_alloc((*cfg).gen_bytes) as *mut uint8_t;
            if (*c).mem_gen.is_null() {
                return TOKS_E_NOMEM as int64_t;
            }
            (*c).mem_gen_len = (*cfg).gen_bytes;
            memcpy(
                (*c).mem_gen as *mut ::core::ffi::c_void,
                (*cfg).gen as *const ::core::ffi::c_void,
                (*cfg).gen_bytes as size_t,
            );
            (*c).gen = (*c).mem_gen as *const ::core::ffi::c_void as *const toks_gen
                as *const toks_gen;
            (*c).scr_extra = toks_gen_scr((*c).gen as *const toks_gen);
        }
    }
    *why = b"decode tables\0" as *const u8 as *const ::core::ffi::c_char;
    r = toks_dec_build(c as *mut toks_ctx);
    if r != 0 as int64_t {
        return r;
    }
    *why = b"vocabulary index\0" as *const u8 as *const ::core::ffi::c_char;
    r = toks_vocab_build(c as *mut toks_ctx, cfg as *const toks_config);
    if r != 0 as int64_t {
        return r;
    }
    let mut bt: toks_bound_terms = toks_bound_terms {
        num: 0,
        den: 0,
        g: 0,
        x: 0,
        xd: 0,
        f: 0,
        p0: 0,
        p1: 0,
        l0: 0,
        l1: 0,
        e0: 0,
        e1: 0,
        pfirst: 0,
        why: 0,
    };
    toks_bound_terms_of(c, &raw mut bt);
    (*c).bound_num = bt.num;
    (*c).bound_den = bt.den;
    (*c).bound_g = bt.g;
    let mut i: uint64_t = 0 as uint64_t;
    while i < (*c).t.add_n {
        if (*(*c).t.add_entries.offset(i as isize)).flags as ::core::ffi::c_uint
            & TOKS_AF_SPECIAL == 0 as ::core::ffi::c_uint
        {
            (*c).n_nonspecial = (*c).n_nonspecial.wrapping_add(1);
        }
        i = i.wrapping_add(1);
    }
    (*c).identity = toks_ctx_identity(
        &raw mut (*c).source_sha256 as *mut uint8_t as *const uint8_t,
    );
    (*c).cpu_features = toks_cpu_features();
    r = pick_tier(
        if !o.is_null() { (*o).tier } else { TOKS_TIER_AUTO as uint32_t },
        (*c).cpu_features,
        why,
    );
    if r < 0 as int64_t {
        return r;
    }
    (*c).tier = r as uint32_t;
    if !(*c).spm.is_null() && (*c).tier != TOKS_TIER_SCALAR as uint32_t {
        (*c).t.flags = ((*c).t.flags as ::core::ffi::c_uint | TOKS_TF_ASM_MERGE)
            as uint32_t;
    }
    return 0 as int64_t;
}
unsafe extern "C" fn build(
    mut c: *mut toks_ctx,
    mut src: *const uint8_t,
    mut len: uint64_t,
    mut par: *mut toks_arena,
    mut o: *const toks_load_opts,
    mut why: *mut *const ::core::ffi::c_char,
) -> int64_t {
    let mut cfg: toks_config = toks_config {
        vocab: ::core::ptr::null::<*const uint8_t>(),
        vocab_len: ::core::ptr::null::<uint32_t>(),
        n_vocab: 0,
        n_vocab_raw: 0,
        drop: [0; 32],
        has_drop: 0,
        drop_unk: 0,
        drop_fuse: 0,
        m_left_id: ::core::ptr::null::<uint32_t>(),
        m_right_id: ::core::ptr::null::<uint32_t>(),
        m_out_id: ::core::ptr::null::<uint32_t>(),
        n_merges: 0,
        ignore_merges: 0,
        dec_byte_level: 0,
        nfc: 0,
        ids_as_rank: 0,
        rsv: 0,
        pattern: ::core::ptr::null::<toks_pattern>(),
        gen: ::core::ptr::null::<toks_gen>(),
        gen_bytes: 0,
        cut_chunk: 0,
        cut_run: 0,
        added: ::core::ptr::null_mut::<toks_cfg_added>(),
        n_added: 0,
        n_ids: 0,
        pp_single: ::core::ptr::null_mut::<toks_pp_piece>(),
        n_pp_single: 0,
        name: ::core::ptr::null::<::core::ffi::c_char>(),
        algo: 0,
        n_strings: 0,
        spm: ::core::ptr::null::<toks_spm_config>(),
        wp_flags: 0,
        wp_max_chars: 0,
        wp_unk_id: 0,
        wp_unk: ::core::ptr::null::<uint8_t>(),
        wp_unk_len: 0,
        wp_prefix_len: 0,
        wp_prefix: ::core::ptr::null::<uint8_t>(),
        o: toks_opts {
            trunc_on: 0,
            trunc_max: 0,
            trunc_stride: 0,
            pad_on: 0,
            pad_fixed: 0,
            pad_len: 0,
            pad_multiple: 0,
            pad_id: 0,
            pad_left: 0,
            wp_win: 0,
            dec_wordpiece: 0,
            dec_cleanup: 0,
            dec_prefix_len: 0,
            dec_prefix: [0; 16],
            pad_file: 0,
            pad_type_id: 0,
        },
        uni: ::core::ptr::null::<toks_uni_src>(),
    };
    let mut err: toks_err = toks_err {
        code: 0 as int64_t,
        what: ::core::ptr::null::<::core::ffi::c_char>(),
    };
    let mut r: int64_t = toks_config_parse(src, len, par, &raw mut cfg, &raw mut err);
    if r != 0 as int64_t {
        *why = if !err.what.is_null() {
            err.what
        } else {
            b"tokenizer config\0" as *const u8 as *const ::core::ffi::c_char
        };
        return r;
    }
    return build_cfg(c, &raw mut cfg, par, o, why);
}
unsafe extern "C" fn load_src(
    mut out: *mut *mut toks_ctx,
    mut src: *mut uint8_t,
    mut len: uint64_t,
    mut name: *const ::core::ffi::c_char,
    mut o: *const toks_load_opts,
) -> int64_t {
    let mut why: *const ::core::ffi::c_char = b"context\0" as *const u8
        as *const ::core::ffi::c_char;
    let mut r: int64_t = TOKS_E_NOMEM as int64_t;
    let mut plen: uint64_t = toks_config_arena_bound(len);
    let mut parse: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    let mut c: *mut toks_ctx = ctx_new(name, plen, &raw mut parse);
    if !c.is_null() && !parse.is_null() {
        toks_sha256(src, len, &raw mut (*c).source_sha256 as *mut uint8_t);
        let mut par: toks_arena = toks_arena {
            base: parse,
            len: plen,
            pos: 0 as uint64_t,
        };
        r = build(c, src, len, &raw mut par, o, &raw mut why);
    } else if c.is_null() {
        why = b"context\0" as *const u8 as *const ::core::ffi::c_char;
    } else {
        why = b"parse arena\0" as *const u8 as *const ::core::ffi::c_char;
    }
    toks_plat_free(parse as *mut ::core::ffi::c_void, plen);
    toks_plat_free(src as *mut ::core::ffi::c_void, len);
    if r != 0 as int64_t {
        ctx_free(c);
        return fail(o, r, why);
    }
    *out = c;
    return fail(o, 0 as int64_t, ::core::ptr::null::<::core::ffi::c_char>());
}
unsafe extern "C" fn join(
    mut dir: *const ::core::ffi::c_char,
    mut dl: uint64_t,
    mut leaf: *const ::core::ffi::c_char,
    mut buf: *mut ::core::ffi::c_char,
    mut cap: uint64_t,
) -> ::core::ffi::c_int {
    let mut ll: uint64_t = 0 as uint64_t;
    while *leaf.offset(ll as isize) as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
        ll = ll.wrapping_add(1);
    }
    let mut sep: uint64_t = (if dl > 0 as uint64_t
        && *dir.offset(dl.wrapping_sub(1 as uint64_t) as isize) as ::core::ffi::c_int
            != '/' as i32
        && *dir.offset(dl.wrapping_sub(1 as uint64_t) as isize) as ::core::ffi::c_int
            != '\\' as i32
    {
        1 as ::core::ffi::c_uint
    } else {
        0 as ::core::ffi::c_uint
    }) as uint64_t;
    if dl.wrapping_add(sep).wrapping_add(ll).wrapping_add(1 as uint64_t) > cap {
        return -(1 as ::core::ffi::c_int);
    }
    memcpy(
        buf as *mut ::core::ffi::c_void,
        dir as *const ::core::ffi::c_void,
        dl as size_t,
    );
    if sep != 0 as uint64_t {
        *buf.offset(dl as isize) = '/' as i32 as ::core::ffi::c_char;
    }
    memcpy(
        buf.offset(dl as isize).offset(sep as isize) as *mut ::core::ffi::c_void,
        leaf as *const ::core::ffi::c_void,
        (ll as size_t).wrapping_add(1 as size_t),
    );
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn load_tiktoken(
    mut out: *mut *mut toks_ctx,
    mut ranks: *mut uint8_t,
    mut rlen: uint64_t,
    mut dir: *const ::core::ffi::c_char,
    mut dl: uint64_t,
    mut name: *const ::core::ffi::c_char,
    mut q: ::core::ffi::c_int,
    mut o: *const toks_load_opts,
) -> int64_t {
    let mut p: [::core::ffi::c_char; 1024] = [0; 1024];
    let mut conf: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    let mut wrap: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    let mut cl: uint64_t = 0 as uint64_t;
    let mut wl: uint64_t = 0 as uint64_t;
    let mut is_dir: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut r: int64_t = TOKS_E_OPEN as int64_t;
    let mut why: *const ::core::ffi::c_char = if q != 0 {
        b"qwen.tiktoken needs tokenizer_config.json and tokenization_qwen.py beside it\0"
            as *const u8 as *const ::core::ffi::c_char
    } else {
        b"tiktoken.model needs tokenizer_config.json and tokenization_kimi.py beside it\0"
            as *const u8 as *const ::core::ffi::c_char
    };
    if join(
        dir,
        dl,
        b"tokenizer_config.json\0" as *const u8 as *const ::core::ffi::c_char,
        &raw mut p as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 1024]>() as uint64_t,
    ) == 0 as ::core::ffi::c_int
        && toks_plat_read_file(
            &raw mut p as *mut ::core::ffi::c_char,
            &raw mut conf,
            &raw mut cl,
            &raw mut is_dir,
        ) == 0 as int64_t
        && join(
            dir,
            dl,
            (if q != 0 {
                b"tokenization_qwen.py\0" as *const u8 as *const ::core::ffi::c_char
            } else {
                b"tokenization_kimi.py\0" as *const u8 as *const ::core::ffi::c_char
            }),
            &raw mut p as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 1024]>() as uint64_t,
        ) == 0 as ::core::ffi::c_int
        && toks_plat_read_file(
            &raw mut p as *mut ::core::ffi::c_char,
            &raw mut wrap,
            &raw mut wl,
            &raw mut is_dir,
        ) == 0 as int64_t
    {
        let mut plen: uint64_t = toks_tiktoken_arena_bound(rlen, cl, wl);
        let mut parse: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut c: *mut toks_ctx = ctx_new(name, plen, &raw mut parse);
        r = TOKS_E_NOMEM as int64_t;
        why = b"context\0" as *const u8 as *const ::core::ffi::c_char;
        if !c.is_null() && !parse.is_null() {
            let mut h: [uint8_t; 96] = [0; 96];
            toks_sha256(ranks, rlen, &raw mut h as *mut uint8_t);
            toks_sha256(
                conf,
                cl,
                (&raw mut h as *mut uint8_t).offset(32 as ::core::ffi::c_int as isize),
            );
            toks_sha256(
                wrap,
                wl,
                (&raw mut h as *mut uint8_t).offset(64 as ::core::ffi::c_int as isize),
            );
            toks_sha256(
                &raw mut h as *mut uint8_t,
                ::core::mem::size_of::<[uint8_t; 96]>() as uint64_t,
                &raw mut (*c).source_sha256 as *mut uint8_t,
            );
            let mut par: toks_arena = toks_arena {
                base: parse,
                len: plen,
                pos: 0 as uint64_t,
            };
            let mut cfg: toks_config = toks_config {
                vocab: ::core::ptr::null::<*const uint8_t>(),
                vocab_len: ::core::ptr::null::<uint32_t>(),
                n_vocab: 0,
                n_vocab_raw: 0,
                drop: [0; 32],
                has_drop: 0,
                drop_unk: 0,
                drop_fuse: 0,
                m_left_id: ::core::ptr::null::<uint32_t>(),
                m_right_id: ::core::ptr::null::<uint32_t>(),
                m_out_id: ::core::ptr::null::<uint32_t>(),
                n_merges: 0,
                ignore_merges: 0,
                dec_byte_level: 0,
                nfc: 0,
                ids_as_rank: 0,
                rsv: 0,
                pattern: ::core::ptr::null::<toks_pattern>(),
                gen: ::core::ptr::null::<toks_gen>(),
                gen_bytes: 0,
                cut_chunk: 0,
                cut_run: 0,
                added: ::core::ptr::null_mut::<toks_cfg_added>(),
                n_added: 0,
                n_ids: 0,
                pp_single: ::core::ptr::null_mut::<toks_pp_piece>(),
                n_pp_single: 0,
                name: ::core::ptr::null::<::core::ffi::c_char>(),
                algo: 0,
                n_strings: 0,
                spm: ::core::ptr::null::<toks_spm_config>(),
                wp_flags: 0,
                wp_max_chars: 0,
                wp_unk_id: 0,
                wp_unk: ::core::ptr::null::<uint8_t>(),
                wp_unk_len: 0,
                wp_prefix_len: 0,
                wp_prefix: ::core::ptr::null::<uint8_t>(),
                o: toks_opts {
                    trunc_on: 0,
                    trunc_max: 0,
                    trunc_stride: 0,
                    pad_on: 0,
                    pad_fixed: 0,
                    pad_len: 0,
                    pad_multiple: 0,
                    pad_id: 0,
                    pad_left: 0,
                    wp_win: 0,
                    dec_wordpiece: 0,
                    dec_cleanup: 0,
                    dec_prefix_len: 0,
                    dec_prefix: [0; 16],
                    pad_file: 0,
                    pad_type_id: 0,
                },
                uni: ::core::ptr::null::<toks_uni_src>(),
            };
            let mut info: toks_tiktoken_info = toks_tiktoken_info {
                n_ranks: 0,
                first_special: 0,
                chunk_chars: 0,
                run_chars: 0,
                n_trie: 0,
                n_named: 0,
                trie: [0; 32],
            };
            let mut err: toks_err = toks_err {
                code: 0 as int64_t,
                what: ::core::ptr::null::<::core::ffi::c_char>(),
            };
            r = toks_tiktoken_parse(
                ranks,
                rlen,
                conf,
                cl,
                wrap,
                wl,
                &raw mut par,
                &raw mut cfg,
                &raw mut info,
                &raw mut err,
            );
            why = if !err.what.is_null() {
                err.what
            } else {
                b"tiktoken model\0" as *const u8 as *const ::core::ffi::c_char
            };
            if r == 0 as int64_t {
                r = build_cfg(c, &raw mut cfg, &raw mut par, o, &raw mut why);
            }
        }
        toks_plat_free(parse as *mut ::core::ffi::c_void, plen);
        if r != 0 as int64_t {
            ctx_free(c);
            c = ::core::ptr::null_mut::<toks_ctx>();
        }
        if !c.is_null() {
            *out = c;
        }
    }
    toks_plat_free(ranks as *mut ::core::ffi::c_void, rlen);
    if !conf.is_null() {
        toks_plat_free(conf as *mut ::core::ffi::c_void, cl);
    }
    if !wrap.is_null() {
        toks_plat_free(wrap as *mut ::core::ffi::c_void, wl);
    }
    return fail(
        o,
        r,
        if r != 0 as int64_t { why } else { ::core::ptr::null::<::core::ffi::c_char>() },
    );
}
unsafe extern "C" fn check_opts(
    mut out: *mut *mut toks_ctx,
    mut o: *const toks_load_opts,
) -> int64_t {
    if out.is_null() {
        return TOKS_E_ARG as int64_t;
    }
    *out = ::core::ptr::null_mut::<toks_ctx>();
    if o.is_null() {
        return 0 as int64_t;
    }
    if (*o).size != ::core::mem::size_of::<toks_load_opts>() as uint32_t {
        return TOKS_E_ARG as int64_t;
    }
    if (*o).rsv != 0 as uint32_t || (*o).flags != 0 as uint32_t {
        return fail(
            o,
            TOKS_E_ARG as int64_t,
            b"toks_load_opts: rsv or flags (no load flags are defined)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    return 0 as int64_t;
}
#[no_mangle]
pub unsafe extern "C" fn toks_load(
    mut out: *mut *mut toks_ctx,
    mut path: *const ::core::ffi::c_char,
    mut o: *const toks_load_opts,
) -> int64_t {
    let mut r: int64_t = check_opts(out, o);
    if r != 0 as int64_t {
        return r;
    }
    if path.is_null() {
        return fail(
            o,
            TOKS_E_ARG as int64_t,
            b"path\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    let mut buf: [::core::ffi::c_char; 1024] = [0; 1024];
    let mut file: *const ::core::ffi::c_char = path;
    let mut src: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    let mut len: uint64_t = 0 as uint64_t;
    let mut is_dir: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    r = toks_plat_read_file(file, &raw mut src, &raw mut len, &raw mut is_dir);
    if r != 0 as int64_t && is_dir != 0 {
        r = toks_plat_dir_lookup(
            path,
            &raw mut buf as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 1024]>() as uint64_t,
        );
        if r != 0 as int64_t {
            let mut pl: uint64_t = 0 as uint64_t;
            while *path.offset(pl as isize) as ::core::ffi::c_int
                != 0 as ::core::ffi::c_int
                && pl < ::core::mem::size_of::<[::core::ffi::c_char; 1024]>() as uint64_t
            {
                pl = pl.wrapping_add(1);
            }
            if (join(
                path,
                pl,
                b"tiktoken.model\0" as *const u8 as *const ::core::ffi::c_char,
                &raw mut buf as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 1024]>() as uint64_t,
            ) != 0 as ::core::ffi::c_int
                || toks_plat_read_file(
                    &raw mut buf as *mut ::core::ffi::c_char,
                    &raw mut src,
                    &raw mut len,
                    &raw mut is_dir,
                ) != 0 as int64_t)
                && (join(
                    path,
                    pl,
                    b"qwen.tiktoken\0" as *const u8 as *const ::core::ffi::c_char,
                    &raw mut buf as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<[::core::ffi::c_char; 1024]>() as uint64_t,
                ) != 0 as ::core::ffi::c_int
                    || toks_plat_read_file(
                        &raw mut buf as *mut ::core::ffi::c_char,
                        &raw mut src,
                        &raw mut len,
                        &raw mut is_dir,
                    ) != 0 as int64_t)
            {
                return fail(
                    o,
                    TOKS_E_OPEN as int64_t,
                    b"model directory without tokenizer.json, tiktoken.model or qwen.tiktoken\0"
                        as *const u8 as *const ::core::ffi::c_char,
                );
            }
            r = 0 as int64_t;
        }
        file = &raw mut buf as *mut ::core::ffi::c_char;
        if src.is_null() {
            r = toks_plat_read_file(file, &raw mut src, &raw mut len, &raw mut is_dir);
        }
    }
    if r != 0 as int64_t {
        return fail(o, r, file);
    }
    let mut n: uint64_t = 0 as uint64_t;
    let mut b0: uint64_t = 0 as uint64_t;
    let mut b1: uint64_t = 0 as uint64_t;
    while *file.offset(n as isize) as ::core::ffi::c_int != 0 as ::core::ffi::c_int
        && n < ::core::mem::size_of::<[::core::ffi::c_char; 1024]>() as uint64_t
    {
        if *file.offset(n as isize) as ::core::ffi::c_int == '/' as i32
            || *file.offset(n as isize) as ::core::ffi::c_int == '\\' as i32
        {
            b1 = b0;
            b0 = n.wrapping_add(1 as uint64_t);
        }
        n = n.wrapping_add(1);
    }
    let mut name: [::core::ffi::c_char; 64] = [0; 64];
    let mut s: uint64_t = b0;
    let mut e: uint64_t = n;
    let mut q: ::core::ffi::c_int = str_eq(
        file.offset(b0 as isize),
        b"qwen.tiktoken\0" as *const u8 as *const ::core::ffi::c_char,
    );
    if (q != 0
        || str_eq(
            file.offset(b0 as isize),
            b"tokenizer.json\0" as *const u8 as *const ::core::ffi::c_char,
        ) != 0
        || str_eq(
            file.offset(b0 as isize),
            b"tiktoken.model\0" as *const u8 as *const ::core::ffi::c_char,
        ) != 0) && b0 > b1
    {
        s = b1;
        e = b0.wrapping_sub(1 as uint64_t);
    }
    let mut k: uint64_t = 0 as uint64_t;
    while s.wrapping_add(k) < e && k < 63 as uint64_t {
        name[k as usize] = *file.offset(s.wrapping_add(k) as isize);
        k = k.wrapping_add(1);
    }
    name[k as usize] = 0 as ::core::ffi::c_char;
    if toks_tiktoken_sniff(src, len) != 0 {
        return load_tiktoken(
            out,
            src,
            len,
            file,
            b0,
            &raw mut name as *mut ::core::ffi::c_char,
            q,
            o,
        );
    }
    return load_src(out, src, len, &raw mut name as *mut ::core::ffi::c_char, o);
}
#[no_mangle]
pub unsafe extern "C" fn toks_load_mem_copy(
    mut out: *mut *mut toks_ctx,
    mut data: *const ::core::ffi::c_void,
    mut len: uint64_t,
    mut o: *const toks_load_opts,
) -> int64_t {
    let mut r: int64_t = check_opts(out, o);
    if r != 0 as int64_t {
        return r;
    }
    if data.is_null() && len != 0 as uint64_t {
        return fail(
            o,
            TOKS_E_ARG as int64_t,
            b"data\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if len as ::core::ffi::c_ulonglong > TOKS_MAX_SOURCE_BYTES {
        return fail(
            o,
            TOKS_E_LIMIT as int64_t,
            b"source above 256 MiB\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if len == 0 as uint64_t {
        return fail(
            o,
            TOKS_E_FORMAT as int64_t,
            b"empty source\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if toks_tiktoken_sniff(data as *const uint8_t, len) != 0 {
        return fail(
            o,
            TOKS_E_UNSUPPORTED as int64_t,
            b"a tiktoken model is three files: toks_load its directory\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    let mut src: *mut uint8_t = toks_plat_alloc(len) as *mut uint8_t;
    if src.is_null() {
        return fail(
            o,
            TOKS_E_NOMEM as int64_t,
            b"source copy\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    memcpy(src as *mut ::core::ffi::c_void, data, len as size_t);
    return load_src(out, src, len, b"\0" as *const u8 as *const ::core::ffi::c_char, o);
}
pub const TOKS_HAVE_K1_NEON: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const TOKS_HAVE_K3_CL100K_NEON: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const TOKS_HAVE_K3_O200K_NEON: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const TOKS_HAVE_K3_DSV3_NEON: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const TOKS_HAVE_K5_NEON: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const TOKS_HAVE_K7_SPM_NEON: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const TOKS_HAVE_K1_AVX2: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const TOKS_HAVE_K3_CL100K_AVX2: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const TOKS_HAVE_K3_O200K_AVX2: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const TOKS_HAVE_K3_DSV3_AVX2: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const TOKS_HAVE_K5_AVX2: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const TOKS_HAVE_K7_SPM_AVX2: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
