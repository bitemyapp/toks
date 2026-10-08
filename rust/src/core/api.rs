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
    fn toks_plat_hint_huge(p: *mut ::core::ffi::c_void, n: uint64_t);
    fn toks_seg_begin(
        it: *mut toks_seg_iter,
        t: *const toks_tables,
        mode: uint32_t,
        phase: uint32_t,
        tier: uint32_t,
        text: *const uint8_t,
        len: uint64_t,
    );
    fn toks_seg_next(it: *mut toks_seg_iter, u: *mut toks_seg_out) -> ::core::ffi::c_int;
    fn toks_lossy_ids(
        ctx: *const toks_ctx,
        d: *mut toks_lossy,
        ids: *const uint32_t,
        n: uint64_t,
        flags: uint32_t,
    ) -> int64_t;
    fn toks_lossy_end(d: *mut toks_lossy);
    fn toks_gen_run(
        ctx: *const toks_ctx,
        h: *mut toks_scratch,
        seg: *const uint8_t,
        len: uint64_t,
        base: uint64_t,
        e: *mut toks_emit,
        ids: ::core::ffi::c_int,
    );
    fn toks_uni_run(
        ctx: *const toks_ctx,
        h: *const toks_scratch,
        text: *const uint8_t,
        len: uint64_t,
        flags: uint32_t,
        out: *mut uint32_t,
        cap: uint64_t,
        ids: ::core::ffi::c_int,
    ) -> int64_t;
    fn toks_uni_dec(
        ctx: *const toks_ctx,
        ids: *const uint32_t,
        n: uint64_t,
        flags: uint32_t,
        out: *mut uint8_t,
        cap: uint64_t,
    ) -> int64_t;
    fn toks_wp_run(
        ctx: *const toks_ctx,
        h: *mut toks_scratch,
        text: *const uint8_t,
        len: uint64_t,
        flags: uint32_t,
        out: *mut uint32_t,
        cap: uint64_t,
        ids: ::core::ffi::c_int,
    ) -> int64_t;
    fn toks_wp_decode(
        ctx: *const toks_ctx,
        ids: *const uint32_t,
        n: uint64_t,
        flags: uint32_t,
        out: *mut uint8_t,
        cap: uint64_t,
    ) -> int64_t;
    fn toks_k3_scan_cl100k_c(t: *const toks_tables, a: *mut toks_k3_args) -> uint64_t;
    #[cfg_attr(target_arch = "x86_64", link_name = "toks_k3_scan_cl100k_avx2")]
    fn toks_k3_scan_cl100k_neon(t: *const toks_tables, a: *mut toks_k3_args) -> uint64_t;
    #[cfg_attr(target_arch = "aarch64", link_name = "toks_k3_scan_cl100k_neon")]
    fn toks_k3_scan_cl100k_avx2(t: *const toks_tables, a: *mut toks_k3_args) -> uint64_t;
    fn toks_k3_scan_o200k_c(t: *const toks_tables, a: *mut toks_k3_args) -> uint64_t;
    #[cfg_attr(target_arch = "x86_64", link_name = "toks_k3_scan_o200k_avx2")]
    fn toks_k3_scan_o200k_neon(t: *const toks_tables, a: *mut toks_k3_args) -> uint64_t;
    #[cfg_attr(target_arch = "aarch64", link_name = "toks_k3_scan_o200k_neon")]
    fn toks_k3_scan_o200k_avx2(t: *const toks_tables, a: *mut toks_k3_args) -> uint64_t;
    fn toks_k3_scan_dsv3_c(t: *const toks_tables, a: *mut toks_k3_args) -> uint64_t;
    #[cfg_attr(target_arch = "x86_64", link_name = "toks_k3_scan_dsv3_avx2")]
    fn toks_k3_scan_dsv3_neon(t: *const toks_tables, a: *mut toks_k3_args) -> uint64_t;
    #[cfg_attr(target_arch = "aarch64", link_name = "toks_k3_scan_dsv3_neon")]
    fn toks_k3_scan_dsv3_avx2(t: *const toks_tables, a: *mut toks_k3_args) -> uint64_t;
    #[cfg(all(feature = "avx512", target_arch = "x86_64"))]
    fn toks_k5_encode_avx512(t: *const toks_tables, a: *mut toks_k5_args) -> uint64_t;
    fn toks_k5_encode_c(t: *const toks_tables, a: *mut toks_k5_args) -> uint64_t;
    #[cfg_attr(target_arch = "x86_64", link_name = "toks_k5_encode_avx2")]
    fn toks_k5_encode_neon(t: *const toks_tables, a: *mut toks_k5_args) -> uint64_t;
    #[cfg_attr(target_arch = "aarch64", link_name = "toks_k5_encode_neon")]
    fn toks_k5_encode_avx2(t: *const toks_tables, a: *mut toks_k5_args) -> uint64_t;
    fn toks_k2(
        text: *const uint8_t,
        pos: uint64_t,
        len: uint64_t,
        hot: uint8_t,
    ) -> uint64_t;
    fn toks_norm(
        steps: uint32_t,
        text: *const uint8_t,
        len: uint64_t,
        out: *mut uint8_t,
        cap: uint64_t,
    ) -> int64_t;
    fn toks_nfc_plan_begin(
        p: *mut toks_nfc_plan,
        f: uint32_t,
        g: *const uint8_t,
        n: uint64_t,
    );
    fn toks_nfc_plan_next(
        p: *mut toks_nfc_plan,
        t: *const toks_tables,
        g: *const uint8_t,
        n: uint64_t,
        s: *mut uint64_t,
        e: *mut uint64_t,
    ) -> ::core::ffi::c_int;
    fn toks_spm_encode(
        t: *const toks_tables,
        s: *const toks_spm,
        text: *const uint8_t,
        len: uint64_t,
        at_start: ::core::ffi::c_int,
        out: *mut uint32_t,
        cap: uint64_t,
        n: uint64_t,
        cache: *mut uint8_t,
        cache_mask: uint64_t,
        tag: uint64_t,
        work: *mut uint8_t,
        flags: uint32_t,
    ) -> uint64_t;
    fn toks_spm_pieces(
        s: *const toks_spm,
        text: *const uint8_t,
        len: uint64_t,
        at_start: ::core::ffi::c_int,
        base: uint64_t,
        out: *mut uint32_t,
        cap: uint64_t,
        n: uint64_t,
    ) -> uint64_t;
    fn toks_spm_decode(
        t: *const toks_tables,
        s: *const toks_spm,
        special: *const uint32_t,
        flags: uint32_t,
        ids: *const uint32_t,
        n: uint64_t,
        out: *mut uint8_t,
        cap: uint64_t,
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
pub type emit = toks_emit;
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
pub struct toks_memo_head {
    pub pos: uint64_t,
    pub hits: uint64_t,
    pub drought: uint64_t,
    pub probes: uint64_t,
    pub differ: uint64_t,
    pub vpos: uint64_t,
    pub lap: uint64_t,
    pub run: uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct memo_rec {
    pub hash: uint64_t,
    pub pos: uint64_t,
    pub key: uint32_t,
    pub n_ids: uint32_t,
    pub len: uint32_t,
    pub epoch: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct toks_seg_out {
    pub kind: uint32_t,
    pub id: uint32_t,
    pub start: uint64_t,
    pub end: uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct toks_k5_args {
    pub text: *const uint8_t,
    pub len: uint64_t,
    pub ends: *const uint32_t,
    pub n: uint64_t,
    pub start: uint64_t,
    pub out: *mut uint32_t,
    pub room: uint64_t,
    pub n_out: uint64_t,
    pub cache: *mut uint8_t,
    pub cache_mask: uint64_t,
    pub work: *mut uint8_t,
    pub work_bytes: uint64_t,
    pub hits_static: uint64_t,
    pub hits_cache: uint64_t,
    pub misses: uint64_t,
    pub cache_tag: uint64_t,
    pub lcache: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct toks_lcache {
    pub buckets: *mut uint8_t,
    pub arena: *mut uint8_t,
    pub mask: uint64_t,
    pub arena_bytes: uint64_t,
    pub pos: uint64_t,
    pub gen: uint64_t,
    pub hits: uint64_t,
    pub misses: uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct toks_k3_args {
    pub text: *const uint8_t,
    pub len: uint64_t,
    pub pos: uint64_t,
    pub ends: *mut uint32_t,
    pub cap: uint64_t,
    pub n: uint64_t,
    pub flags: uint64_t,
    pub rsv: uint64_t,
}
pub type toks_k3_fn = Option<
    unsafe extern "C" fn(*const toks_tables, *mut toks_k3_args) -> uint64_t,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct toks_nfc_plan {
    pub r: uint64_t,
    pub d0: uint64_t,
    pub d1: uint64_t,
    pub f: uint32_t,
}
pub const TOKS_SPM_PFX_GAP: C2RustUnnamed = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct toks_seg_iter {
    pub t: *const toks_tables,
    pub text: *const uint8_t,
    pub len: uint64_t,
    pub pos: uint64_t,
    pub prev_end: uint64_t,
    pub pend_start: uint64_t,
    pub pend_end: uint64_t,
    pub rs_from: uint64_t,
    pub rs_to: uint64_t,
    pub pend_id: uint32_t,
    pub mode: uint32_t,
    pub phase: uint32_t,
    pub tier: uint32_t,
    pub fin: uint32_t,
    pub pend: uint32_t,
    pub mi: uint32_t,
    pub mn: uint32_t,
    pub m: [toks_k1_match; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct toks_k1_match {
    pub start: uint32_t,
    pub end: uint32_t,
    pub entry: uint32_t,
    pub rsv: uint32_t,
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
pub struct toks_dsink {
    pub out: *mut uint8_t,
    pub cap: uint64_t,
    pub n: uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct toks_info {
    pub size: uint32_t,
    pub abi_major: uint32_t,
    pub abi_minor: uint32_t,
    pub algorithm: uint32_t,
    pub tier: uint32_t,
    pub n_ids: uint32_t,
    pub n_added: uint32_t,
    pub paths: uint32_t,
    pub cpu_features: uint64_t,
    pub max_text: uint64_t,
    pub control_isolation: uint32_t,
    pub rsv: uint32_t,
    pub source_sha256: [uint8_t; 32],
    pub image_sha256: [uint8_t; 32],
    pub name: [::core::ffi::c_char; 64],
    pub trunc_on: uint32_t,
    pub trunc_max: uint32_t,
    pub trunc_stride: uint32_t,
    pub pad_on: uint32_t,
    pub pad_fixed: uint32_t,
    pub pad_id: uint32_t,
    pub pad_type_id: uint32_t,
    pub pad_len: uint32_t,
    pub pad_multiple: uint32_t,
    pub pad_left: uint32_t,
    pub n_template_prefix: uint32_t,
    pub n_template_suffix: uint32_t,
    pub seq_type_id: uint32_t,
    pub rsv2: uint32_t,
}
pub type C2RustUnnamed = ::core::ffi::c_uint;
pub const TOKS_SPM_PFX_FIRST: C2RustUnnamed = 3;
pub const TOKS_SPM_PFX_ALWAYS: C2RustUnnamed = 2;
pub const TOKS_SPM_PFX_NONE: C2RustUnnamed = 0;
pub const UINT32_MAX: ::core::ffi::c_uint = 4294967295 as ::core::ffi::c_uint;
pub const UINT64_MAX: ::core::ffi::c_ulong = 18446744073709551615
    as ::core::ffi::c_ulong;
pub const TOKS_ABI_MAJOR: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TOKS_ABI_MINOR: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const TOKS_E_SCRATCH: ::core::ffi::c_int = -(6 as ::core::ffi::c_int);
pub const TOKS_E_ID: ::core::ffi::c_int = -(7 as ::core::ffi::c_int);
pub const TOKS_E_CAP: ::core::ffi::c_int = -(8 as ::core::ffi::c_int);
pub const TOKS_E_LIMIT: ::core::ffi::c_int = -(9 as ::core::ffi::c_int);
pub const TOKS_E_ARG: ::core::ffi::c_int = -(10 as ::core::ffi::c_int);
pub const TOKS_MAX_TEXT: ::core::ffi::c_ulonglong = (1 as ::core::ffi::c_ulonglong)
    << 29 as ::core::ffi::c_int;
pub const TOKS_TIER_SCALAR: ::core::ffi::c_uint = 1 as ::core::ffi::c_uint;
pub const TOKS_TIER_NEON: ::core::ffi::c_uint = 2;
pub const TOKS_TIER_AVX2: ::core::ffi::c_uint = 3;
pub const TOKS_TIER_AVX512: ::core::ffi::c_uint = 4 as ::core::ffi::c_uint;
pub const TOKS_SCRATCH_MEMO_SET: ::core::ffi::c_uint = (1 as ::core::ffi::c_uint)
    << 20 as ::core::ffi::c_int;
pub const TOKS_ADDED_ALL: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
pub const TOKS_ADDED_NONSPECIAL: ::core::ffi::c_uint = 1 as ::core::ffi::c_uint;
pub const TOKS_ADDED_NONE: ::core::ffi::c_uint = 2 as ::core::ffi::c_uint;
pub const TOKS_ADDED_MASK: ::core::ffi::c_uint = 3 as ::core::ffi::c_uint;
pub const TOKS_NO_POSTPROCESS: ::core::ffi::c_uint = 4 as ::core::ffi::c_uint;
pub const TOKS_CONTINUATION: ::core::ffi::c_uint = 8 as ::core::ffi::c_uint;
pub const TOKS_NO_TRUNCATE: ::core::ffi::c_uint = 16 as ::core::ffi::c_uint;
pub const TOKS_NO_PAD: ::core::ffi::c_uint = 32 as ::core::ffi::c_uint;
pub const TOKS_SKIP_SPECIAL: ::core::ffi::c_uint = 1 as ::core::ffi::c_uint;
pub const TOKS_DECODE_RAW: ::core::ffi::c_uint = 2 as ::core::ffi::c_uint;
pub const TOKS_PATH_SCAN: ::core::ffi::c_uint = 1 as ::core::ffi::c_uint;
pub const TOKS_PATH_NORMALIZE: ::core::ffi::c_uint = 2 as ::core::ffi::c_uint;
pub const TOKS_CHUNK_PIECES: ::core::ffi::c_int = 256 as ::core::ffi::c_int;
pub const TOKS_TMPL_NONE: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
pub const TOKS_TMPL_O200K: ::core::ffi::c_uint = 2 as ::core::ffi::c_uint;
pub const TOKS_TMPL_DSV3: ::core::ffi::c_uint = 3 as ::core::ffi::c_uint;
pub const TOKS_TF_TWIN: ::core::ffi::c_uint = 0x20 as ::core::ffi::c_uint;
pub const TOKS_BUCKET: ::core::ffi::c_int = 64 as ::core::ffi::c_int;
pub const TOKS_TAG_MAX: ::core::ffi::c_uint = 0x3fffff as ::core::ffi::c_uint;
pub const TOKS_K5_WARM: ::core::ffi::c_int = 4096 as ::core::ffi::c_int;
pub const TOKS_FIB64: ::core::ffi::c_ulonglong = 0x9e3779b97f4a7c15
    as ::core::ffi::c_ulonglong;
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
unsafe extern "C" fn toks_bit(mut bits: *const uint32_t, mut id: uint32_t) -> uint32_t {
    return *bits.offset((id >> 5 as ::core::ffi::c_int) as isize)
        >> (id & 31 as uint32_t) & 1 as uint32_t;
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
unsafe extern "C" fn toks_pad(mut o: *const toks_opts, mut e: *mut toks_emit) {
    let mut target: uint64_t = if (*o).pad_fixed != 0 {
        (*o).pad_len as uint64_t
    } else {
        (*e).n
    };
    let mut m: uint64_t = (*o).pad_multiple as uint64_t;
    if m != 0 as uint64_t && target.wrapping_rem(m) != 0 as uint64_t {
        target = target.wrapping_add(m.wrapping_sub(target.wrapping_rem(m)));
    }
    if (*e).n >= target {
        return;
    }
    let mut p: uint64_t = target.wrapping_sub((*e).n);
    if (*o).pad_left == 0 as uint32_t {
        let mut i: uint64_t = 0 as uint64_t;
        while i < p {
            toks_put(e, (*o).pad_id);
            i = i.wrapping_add(1);
        }
        return;
    }
    let mut w0: uint64_t = if (*e).n < (*e).cap { (*e).n } else { (*e).cap };
    if (*e).cap > p {
        let mut keep: uint64_t = if w0 < (*e).cap.wrapping_sub(p) {
            w0
        } else {
            (*e).cap.wrapping_sub(p)
        };
        let mut i_0: uint64_t = keep;
        while i_0 > 0 as uint64_t {
            toks_st32(
                (*e)
                    .out
                    .offset(p as isize)
                    .offset(i_0 as isize)
                    .offset(-(1 as ::core::ffi::c_uint as isize))
                    as *mut ::core::ffi::c_void,
                toks_ld32(
                    (*e)
                        .out
                        .offset(i_0 as isize)
                        .offset(-(1 as ::core::ffi::c_uint as isize))
                        as *const ::core::ffi::c_void,
                ),
            );
            i_0 = i_0.wrapping_sub(1);
        }
    }
    let mut f: uint64_t = if p < (*e).cap { p } else { (*e).cap };
    let mut i_1: uint64_t = 0 as uint64_t;
    while i_1 < f {
        toks_st32(
            (*e).out.offset(i_1 as isize) as *mut ::core::ffi::c_void,
            (*o).pad_id,
        );
        i_1 = i_1.wrapping_add(1);
    }
    (*e).n = (*e).n.wrapping_add(p);
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
pub const TOKS_SCRATCH_MAGIC: ::core::ffi::c_ulonglong = 0x31524353534b4f54
    as ::core::ffi::c_ulonglong;
pub const TOKS_CACHE_BUCKETS: ::core::ffi::c_uint = 32768 as ::core::ffi::c_uint;
pub const TOKS_CACHE_BYTES: uint64_t = (TOKS_CACHE_BUCKETS as uint64_t)
    .wrapping_mul(TOKS_BUCKET as uint64_t);
pub const TOKS_SCR_HDR: ::core::ffi::c_uint = 128 as ::core::ffi::c_uint;
pub const TOKS_SCR_FIXED: uint64_t = (TOKS_SCR_HDR as uint64_t)
    .wrapping_add((4 as uint64_t).wrapping_mul(TOKS_CHUNK_PIECES as uint64_t));
pub const TOKS_NFC_X: ::core::ffi::c_uint = 3 as ::core::ffi::c_uint;
pub const TOKS_NFKC_X: ::core::ffi::c_uint = 11 as ::core::ffi::c_uint;
#[inline]
unsafe extern "C" fn toks_align64(mut v: uint64_t) -> uint64_t {
    return v.wrapping_add(63 as uint64_t) & !(63 as ::core::ffi::c_uint as uint64_t);
}
#[inline]
unsafe extern "C" fn toks_mix64(mut x: uint64_t, mut w: uint64_t) -> uint64_t {
    x = ((x ^ w) as ::core::ffi::c_ulonglong)
        .wrapping_mul(0xff51afd7ed558ccd as ::core::ffi::c_ulonglong) as uint64_t;
    return x ^ x >> 32 as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn toks_scr_header(
    mut scr: *mut ::core::ffi::c_void,
) -> *mut toks_scratch {
    let mut base: uint64_t = scr as uintptr_t as uint64_t;
    return (scr as *mut uint8_t).offset(toks_align64(base).wrapping_sub(base) as isize)
        as *mut ::core::ffi::c_void as *mut toks_scratch;
}
#[inline]
unsafe extern "C" fn toks_scr_p(
    mut h: *const toks_scratch,
    mut p: *mut uint8_t,
) -> *mut uint8_t {
    return p;
}
#[inline]
unsafe extern "C" fn toks_scr_zero(
    mut h: *const toks_scratch,
    mut p: *mut uint8_t,
    mut n: uint64_t,
) {
    memset(p as *mut ::core::ffi::c_void, 0 as ::core::ffi::c_int, n as size_t);
}
#[inline]
unsafe extern "C" fn toks_scr_carved(
    mut ctx: *const toks_ctx,
    mut h: *mut toks_scratch,
) {}
#[inline]
unsafe extern "C" fn toks_scr_work(mut max_len: uint64_t) -> uint64_t {
    return toks_align64(
        (256 as uint64_t).wrapping_add((32 as uint64_t).wrapping_mul(max_len)),
    );
}
#[inline]
unsafe extern "C" fn toks_scr_bounce(mut max_len: uint64_t) -> uint64_t {
    return toks_align64(
        (4 as uint64_t).wrapping_mul(max_len.wrapping_add(4 as uint64_t)),
    );
}
#[inline]
unsafe extern "C" fn toks_scr_tmax(mut max_len: uint64_t, mut x: uint32_t) -> uint64_t {
    return if x != 0 as uint32_t {
        (x as uint64_t).wrapping_mul(max_len)
    } else {
        max_len
    };
}
#[inline]
unsafe extern "C" fn toks_scr_norm(mut max_len: uint64_t, mut x: uint32_t) -> uint64_t {
    return if x != 0 as uint32_t {
        toks_align64(toks_scr_tmax(max_len, x))
    } else {
        0 as uint64_t
    };
}
#[inline]
unsafe extern "C" fn toks_scr_cache_mib(mut flags: uint32_t) -> uint64_t {
    return flags as uint64_t >> 12 as ::core::ffi::c_int & 0xff as uint64_t;
}
#[inline]
unsafe extern "C" fn toks_scr_cache_ok(mut n: uint64_t) -> ::core::ffi::c_int {
    return (n == 0 as uint64_t
        || n >= 4 as uint64_t && n <= 128 as uint64_t
            && n & n.wrapping_sub(1 as uint64_t) == 0 as uint64_t) as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn toks_scr_short(mut n: uint64_t) -> uint64_t {
    return if n == 0 as uint64_t {
        TOKS_CACHE_BYTES
    } else {
        n << 19 as ::core::ffi::c_int
    };
}
#[inline]
unsafe extern "C" fn toks_scr_long_buckets(mut n: uint64_t) -> uint64_t {
    return n << 17 as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn toks_scr_long_arena(mut n: uint64_t) -> uint64_t {
    return (3 as uint64_t).wrapping_mul(n << 17 as ::core::ffi::c_int);
}
#[inline]
unsafe extern "C" fn toks_scr_caches(mut n: uint64_t) -> uint64_t {
    return toks_scr_short(n).wrapping_add(n << 19 as ::core::ffi::c_int);
}
pub const TOKS_MEMO_MIB: ::core::ffi::c_uint = 4 as ::core::ffi::c_uint;
#[inline]
unsafe extern "C" fn toks_scr_memo_bytes(mut flags: uint32_t) -> uint64_t {
    return (if flags
        & (TOKS_SCRATCH_MEMO_SET as uint32_t
            | 0xfff as ::core::ffi::c_uint as uint32_t & 0xfff as uint32_t)
        != 0 as uint32_t
    {
        flags as uint64_t & 0xfff as uint64_t
    } else {
        TOKS_MEMO_MIB as uint64_t
    }) << 20 as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn toks_scr_memo(
    mut h: *mut toks_scratch,
    mut bytes: *mut uint64_t,
) -> *mut uint8_t {
    *bytes = (*h)
        .off_work
        .wrapping_sub((*h).off_cache)
        .wrapping_sub(toks_scr_caches((*h).cache_mib));
    return if *bytes != 0 as uint64_t {
        toks_scr_p(
            h,
            ((*h).base as uintptr_t as *mut uint8_t)
                .offset((*h).off_cache as isize)
                .offset(toks_scr_caches((*h).cache_mib) as isize),
        )
    } else {
        ::core::ptr::null_mut::<uint8_t>()
    };
}
pub const TOKS_MEMO_DRY: ::core::ffi::c_uint = 1024 as ::core::ffi::c_uint;
pub const TOKS_MEMO_GHOSTS: ::core::ffi::c_uint = 2 as ::core::ffi::c_uint;
#[inline]
unsafe extern "C" fn toks_scratch_size(
    mut max_len: uint64_t,
    mut x: uint32_t,
    mut n: uint64_t,
) -> uint64_t {
    let mut t: uint64_t = toks_scr_tmax(max_len, x);
    return (63 as uint64_t)
        .wrapping_add(TOKS_SCR_FIXED)
        .wrapping_add(toks_scr_caches(n))
        .wrapping_add(toks_scr_work(t))
        .wrapping_add(toks_scr_bounce(t))
        .wrapping_add(toks_scr_norm(max_len, x));
}
pub const TOKS_WP_MAT_CAP: ::core::ffi::c_uint = 24704 as ::core::ffi::c_uint;
pub const TOKS_WP_SCR_EXTRA: uint64_t = (16 as uint64_t)
    .wrapping_mul(TOKS_CHUNK_PIECES as uint64_t)
    .wrapping_add(TOKS_WP_MAT_CAP as uint64_t)
    .wrapping_add(512 as uint64_t);
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
pub const TOKS_SEG_TOKEN: ::core::ffi::c_uint = 1 as ::core::ffi::c_uint;
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
pub const TOKS_HAVE_K3_CL100K_AVX512: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TOKS_HAVE_K3_O200K_AVX512: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TOKS_HAVE_K3_DSV3_AVX512: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TOKS_HAVE_K5_AVX512: ::core::ffi::c_int = cfg!(all(feature = "avx512", target_arch = "x86_64")) as ::core::ffi::c_int;
#[inline(always)]
unsafe extern "C" fn toks_k3_short<const DSV3: bool>(
    mut twin: toks_k3_fn,
    mut t: *const toks_tables,
    mut a: *mut toks_k3_args,
    mut n: *mut uint64_t,
) -> ::core::ffi::c_int {
    let mut r: uint64_t = (*a).len.wrapping_sub((*a).pos);
    let mut p: *const uint8_t = (*a).text.offset((*a).pos as isize);
    // Match each architecture's native/scalar crossover without comparing
    // function addresses (which Rust may merge or duplicate during codegen).
    let (ascii_min, other_min) = if cfg!(target_arch = "aarch64") {
        if DSV3 { (16, 32) } else { (7, 24) }
    } else {
        if DSV3 { (7, 16) } else { (4, 16) }
    };
    if r >= other_min {
        return 0 as ::core::ffi::c_int;
    }
    if r == 1 as uint64_t
        || r.wrapping_sub(2 as uint64_t) < 3 as uint64_t
            && *p.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint
                >= 0xc0 as ::core::ffi::c_uint && toks_utf8_len(p, r) as uint64_t == r
    {
        toks_st32((*a).ends as *mut ::core::ffi::c_void, (*a).len as uint32_t);
        (*a).pos = (*a).len;
        (*a).n = 1 as uint64_t;
        *n = 1 as uint64_t;
        return 1 as ::core::ffi::c_int;
    }
    if r >= ascii_min {
        let mut w: uint64_t = toks_ld32(p as *const ::core::ffi::c_void) as uint64_t
            | (toks_ld32(
                p
                    .offset(
                        (if r < 8 as uint64_t {
                            r.wrapping_sub(4 as uint64_t)
                        } else {
                            4 as uint64_t
                        }) as isize,
                    ) as *const ::core::ffi::c_void,
            ) as uint64_t) << 32 as ::core::ffi::c_int;
        if ((w >> 7 as ::core::ffi::c_int & 0x101010101010101 as uint64_t)
            .wrapping_mul(0x101010101010101 as uint64_t) >> 56 as ::core::ffi::c_int)
            < 4 as uint64_t
        {
            return 0 as ::core::ffi::c_int;
        }
    }
    *n = twin.expect("non-null function pointer")(t, a);
    return 1 as ::core::ffi::c_int;
}
#[inline(always)]
unsafe extern "C" fn toks_k3_parts<const DSV3: bool>(
    mut part: toks_k3_fn,
    mut twin: toks_k3_fn,
    mut t: *const toks_tables,
    mut a: *mut toks_k3_args,
) -> uint64_t {
    let mut rsv: uint64_t = (*a).rsv;
    let mut n: uint64_t = part.expect("non-null function pointer")(t, a);
    let mut m: uint64_t = 0;
    let mut cap: uint64_t = 0;
    if (*a).rsv != 0 as uint64_t {
        let mut b: toks_k3_args = *a;
        b.ends = b.ends.offset(n as isize);
        b.cap = b.cap.wrapping_sub(n);
        while b.rsv != 0 as uint64_t && b.cap != 0 as uint64_t && b.pos < b.len {
            if b.pos < b.rsv {
                cap = b.cap;
                b.cap = 1 as uint64_t;
                n = n
                    .wrapping_add(
                        twin.expect("non-null function pointer")(t, &raw mut b),
                    );
                b.ends = b.ends.offset(1);
                b.cap = cap.wrapping_sub(1 as uint64_t);
            } else if toks_k3_short::<DSV3>(twin, t, &raw mut b, &raw mut m) != 0 {
                n = n.wrapping_add(m);
                break;
            } else {
                m = part.expect("non-null function pointer")(t, &raw mut b);
                n = n.wrapping_add(m);
                b.ends = b.ends.offset(m as isize);
                b.cap = b.cap.wrapping_sub(m);
            }
        }
        (*a).n = n;
        (*a).pos = b.pos;
    }
    (*a).rsv = rsv;
    return n;
}
#[inline]
unsafe extern "C" fn toks_k3(
    mut t: *const toks_tables,
    mut a: *mut toks_k3_args,
    mut tier: uint32_t,
) -> uint64_t {
    let mut n: uint64_t = 0;
    if toks_k3_short::<false>(
        Some(
            toks_k3_scan_cl100k_c
                as unsafe extern "C" fn(
                    *const toks_tables,
                    *mut toks_k3_args,
                ) -> uint64_t,
        ),
        t,
        a,
        &raw mut n,
    ) != 0
    {
        return n;
    }
    match if tier == TOKS_TIER_NEON as uint32_t && TOKS_HAVE_K3_CL100K_NEON != 0 {
        TOKS_TIER_NEON
    } else if tier == TOKS_TIER_AVX512 as uint32_t && TOKS_HAVE_K3_CL100K_AVX512 != 0 {
        TOKS_TIER_AVX512
    } else if (tier == TOKS_TIER_AVX512 as uint32_t
        || tier == TOKS_TIER_AVX2 as uint32_t) && TOKS_HAVE_K3_CL100K_AVX2 != 0
    {
        TOKS_TIER_AVX2
    } else {
        TOKS_TIER_SCALAR
    } {
        TOKS_TIER_NEON => {
            return toks_k3_parts::<false>(
                Some(
                    toks_k3_scan_cl100k_neon
                        as unsafe extern "C" fn(
                            *const toks_tables,
                            *mut toks_k3_args,
                        ) -> uint64_t,
                ),
                Some(
                    toks_k3_scan_cl100k_c
                        as unsafe extern "C" fn(
                            *const toks_tables,
                            *mut toks_k3_args,
                        ) -> uint64_t,
                ),
                t,
                a,
            );
        }
        TOKS_TIER_AVX2 => {
            return toks_k3_parts::<false>(
                Some(
                    toks_k3_scan_cl100k_avx2
                        as unsafe extern "C" fn(
                            *const toks_tables,
                            *mut toks_k3_args,
                        ) -> uint64_t,
                ),
                Some(
                    toks_k3_scan_cl100k_c
                        as unsafe extern "C" fn(
                            *const toks_tables,
                            *mut toks_k3_args,
                        ) -> uint64_t,
                ),
                t,
                a,
            );
        }
        _ => return toks_k3_scan_cl100k_c(t, a),
    };
}
#[inline]
unsafe extern "C" fn toks_k3_o200k(
    mut t: *const toks_tables,
    mut a: *mut toks_k3_args,
    mut tier: uint32_t,
) -> uint64_t {
    let mut n: uint64_t = 0;
    if toks_k3_short::<false>(
        Some(
            toks_k3_scan_o200k_c
                as unsafe extern "C" fn(
                    *const toks_tables,
                    *mut toks_k3_args,
                ) -> uint64_t,
        ),
        t,
        a,
        &raw mut n,
    ) != 0
    {
        return n;
    }
    match if tier == TOKS_TIER_NEON as uint32_t && TOKS_HAVE_K3_O200K_NEON != 0 {
        TOKS_TIER_NEON
    } else if tier == TOKS_TIER_AVX512 as uint32_t && TOKS_HAVE_K3_O200K_AVX512 != 0 {
        TOKS_TIER_AVX512
    } else if (tier == TOKS_TIER_AVX512 as uint32_t
        || tier == TOKS_TIER_AVX2 as uint32_t) && TOKS_HAVE_K3_O200K_AVX2 != 0
    {
        TOKS_TIER_AVX2
    } else {
        TOKS_TIER_SCALAR
    } {
        TOKS_TIER_NEON => {
            return toks_k3_parts::<false>(
                Some(
                    toks_k3_scan_o200k_neon
                        as unsafe extern "C" fn(
                            *const toks_tables,
                            *mut toks_k3_args,
                        ) -> uint64_t,
                ),
                Some(
                    toks_k3_scan_o200k_c
                        as unsafe extern "C" fn(
                            *const toks_tables,
                            *mut toks_k3_args,
                        ) -> uint64_t,
                ),
                t,
                a,
            );
        }
        TOKS_TIER_AVX2 => {
            return toks_k3_parts::<false>(
                Some(
                    toks_k3_scan_o200k_avx2
                        as unsafe extern "C" fn(
                            *const toks_tables,
                            *mut toks_k3_args,
                        ) -> uint64_t,
                ),
                Some(
                    toks_k3_scan_o200k_c
                        as unsafe extern "C" fn(
                            *const toks_tables,
                            *mut toks_k3_args,
                        ) -> uint64_t,
                ),
                t,
                a,
            );
        }
        _ => return toks_k3_scan_o200k_c(t, a),
    };
}
#[inline]
unsafe extern "C" fn toks_k3_dsv3(
    mut t: *const toks_tables,
    mut a: *mut toks_k3_args,
    mut tier: uint32_t,
) -> uint64_t {
    let mut n: uint64_t = 0;
    if toks_k3_short::<true>(
        Some(
            toks_k3_scan_dsv3_c
                as unsafe extern "C" fn(
                    *const toks_tables,
                    *mut toks_k3_args,
                ) -> uint64_t,
        ),
        t,
        a,
        &raw mut n,
    ) != 0
    {
        return n;
    }
    match if tier == TOKS_TIER_NEON as uint32_t && TOKS_HAVE_K3_DSV3_NEON != 0 {
        TOKS_TIER_NEON
    } else if tier == TOKS_TIER_AVX512 as uint32_t && TOKS_HAVE_K3_DSV3_AVX512 != 0 {
        TOKS_TIER_AVX512
    } else if (tier == TOKS_TIER_AVX512 as uint32_t
        || tier == TOKS_TIER_AVX2 as uint32_t) && TOKS_HAVE_K3_DSV3_AVX2 != 0
    {
        TOKS_TIER_AVX2
    } else {
        TOKS_TIER_SCALAR
    } {
        TOKS_TIER_NEON => {
            return toks_k3_parts::<true>(
                Some(
                    toks_k3_scan_dsv3_neon
                        as unsafe extern "C" fn(
                            *const toks_tables,
                            *mut toks_k3_args,
                        ) -> uint64_t,
                ),
                Some(
                    toks_k3_scan_dsv3_c
                        as unsafe extern "C" fn(
                            *const toks_tables,
                            *mut toks_k3_args,
                        ) -> uint64_t,
                ),
                t,
                a,
            );
        }
        TOKS_TIER_AVX2 => {
            return toks_k3_parts::<true>(
                Some(
                    toks_k3_scan_dsv3_avx2
                        as unsafe extern "C" fn(
                            *const toks_tables,
                            *mut toks_k3_args,
                        ) -> uint64_t,
                ),
                Some(
                    toks_k3_scan_dsv3_c
                        as unsafe extern "C" fn(
                            *const toks_tables,
                            *mut toks_k3_args,
                        ) -> uint64_t,
                ),
                t,
                a,
            );
        }
        _ => return toks_k3_scan_dsv3_c(t, a),
    };
}
#[inline]
unsafe extern "C" fn toks_k5(
    mut t: *const toks_tables,
    mut a: *mut toks_k5_args,
    mut tier: uint32_t,
) -> uint64_t {
    match if tier == TOKS_TIER_NEON as uint32_t && TOKS_HAVE_K5_NEON != 0 {
        TOKS_TIER_NEON
    } else if tier == TOKS_TIER_AVX512 as uint32_t && TOKS_HAVE_K5_AVX512 != 0 {
        TOKS_TIER_AVX512
    } else if (tier == TOKS_TIER_AVX512 as uint32_t
        || tier == TOKS_TIER_AVX2 as uint32_t) && TOKS_HAVE_K5_AVX2 != 0
    {
        TOKS_TIER_AVX2
    } else {
        TOKS_TIER_SCALAR
    } {
        #[cfg(all(feature = "avx512", target_arch = "x86_64"))]
        TOKS_TIER_AVX512 => return toks_k5_encode_avx512(t, a),
        TOKS_TIER_NEON => return toks_k5_encode_neon(t, a),
        TOKS_TIER_AVX2 => return toks_k5_encode_avx2(t, a),
        _ => return toks_k5_encode_c(t, a),
    };
}
pub const TOKS_NS_COMPAT: ::core::ffi::c_uint = 0x40 as ::core::ffi::c_uint;
pub const TOKS_NFC_HOT_BYTE: ::core::ffi::c_uint = 0xcc as ::core::ffi::c_uint;
pub const TOKS_NFKC_HOT_BYTE: ::core::ffi::c_uint = 0xc2 as ::core::ffi::c_uint;
pub const TOKS_SPM_NOPFX: ::core::ffi::c_uint = 4 as ::core::ffi::c_uint;
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
unsafe extern "C" fn scr_x(mut ctx: *const toks_ctx) -> uint32_t {
    return if (*ctx).nfc != 0 as uint32_t {
        if (*ctx).nfc & TOKS_NS_COMPAT as uint32_t != 0 as uint32_t {
            TOKS_NFKC_X as uint32_t
        } else {
            TOKS_NFC_X as uint32_t
        }
    } else if (*ctx).has_drop != 0 as uint32_t {
        TOKS_NFC_X as uint32_t
    } else {
        0 as uint32_t
    };
}
unsafe extern "C" fn scr_memo(
    mut ctx: *const toks_ctx,
    mut flags: uint32_t,
) -> uint64_t {
    return if !(*ctx).wp.is_null() || !(*ctx).uni.is_null() {
        0 as uint64_t
    } else {
        toks_scr_memo_bytes(flags)
    };
}
#[no_mangle]
pub unsafe extern "C" fn toks_scratch_bytes(
    mut ctx: *const toks_ctx,
    mut max_len: uint64_t,
    mut flags: uint32_t,
) -> uint64_t {
    let mut n: uint64_t = toks_scr_cache_mib(flags);
    n = if toks_scr_cache_ok(n) != 0 { n } else { 0 as uint64_t };
    if max_len as ::core::ffi::c_ulonglong > TOKS_MAX_TEXT {
        max_len = TOKS_MAX_TEXT as uint64_t;
    }
    return if !ctx.is_null() {
        toks_scratch_size(max_len, scr_x(ctx), n)
            .wrapping_add(scr_memo(ctx, flags))
            .wrapping_add((*ctx).scr_extra)
    } else {
        toks_scratch_size(max_len, TOKS_NFKC_X as uint32_t, n)
            .wrapping_add(toks_scr_memo_bytes(flags))
            .wrapping_add(TOKS_WP_SCR_EXTRA)
    };
}
#[no_mangle]
pub unsafe extern "C" fn toks_scratch_init(
    mut ctx: *const toks_ctx,
    mut scr: *mut ::core::ffi::c_void,
    mut bytes: uint64_t,
    mut flags: uint32_t,
) -> int64_t {
    if ctx.is_null() || scr.is_null() && bytes != 0 as uint64_t {
        return TOKS_E_ARG as int64_t;
    }
    let mut n: uint64_t = toks_scr_cache_mib(flags);
    if flags
        & !(TOKS_SCRATCH_MEMO_SET as uint32_t
            | 0xfff as ::core::ffi::c_uint as uint32_t & 0xfff as uint32_t
            | (0xff as ::core::ffi::c_uint as uint32_t & 0xff as uint32_t)
                << 12 as ::core::ffi::c_int) != 0 as uint32_t
        || toks_scr_cache_ok(n) == 0
    {
        return TOKS_E_ARG as int64_t;
    }
    let mut extra: uint64_t = (*ctx).scr_extra.wrapping_add(scr_memo(ctx, flags));
    if bytes < toks_scratch_size(0 as uint64_t, scr_x(ctx), n).wrapping_add(extra) {
        return TOKS_E_SCRATCH as int64_t;
    }
    let mut lo: uint64_t = 0 as uint64_t;
    let mut hi: uint64_t = TOKS_MAX_TEXT as uint64_t;
    while lo < hi {
        let mut mid: uint64_t = lo
            .wrapping_add(
                hi.wrapping_sub(lo).wrapping_add(1 as uint64_t)
                    >> 1 as ::core::ffi::c_int,
            );
        if toks_scratch_size(mid, scr_x(ctx), n).wrapping_add(extra) <= bytes {
            lo = mid;
        } else {
            hi = mid.wrapping_sub(1 as uint64_t);
        }
    }
    extra = (*ctx).scr_extra;
    let mut base: uint64_t = scr as uintptr_t as uint64_t;
    let mut a: uint64_t = toks_align64(base).wrapping_sub(base);
    let mut s0: *mut uint8_t = scr as *mut uint8_t;
    let mut h: *mut toks_scratch = s0.offset(a as isize) as *mut ::core::ffi::c_void
        as *mut toks_scratch;
    let mut epoch: uint64_t = 0 as uint64_t;
    let mut gen: uint64_t = 0 as uint64_t;
    let mut memo: uint64_t = scr_memo(ctx, flags);
    if crate::raw_header_word(&raw const (*h).magic) as ::core::ffi::c_ulonglong == TOKS_SCRATCH_MAGIC && crate::raw_header_word(&raw const (*h).base) == base
        && crate::raw_header_word(&raw const (*h).bytes) == bytes && crate::raw_header_word(&raw const (*h).off_cache) == a.wrapping_add(TOKS_SCR_FIXED)
        && crate::raw_header_word(&raw const (*h).cache_mib) == n
        && crate::raw_header_word(&raw const (*h).off_work)
            == crate::raw_header_word(&raw const (*h).off_cache).wrapping_add(toks_scr_caches(n)).wrapping_add(memo)
        && crate::raw_header_word(&raw const (*h).epoch) >= 1 as uint64_t && crate::raw_header_word(&raw const (*h).epoch) < TOKS_TAG_MAX as uint64_t
        && (n == 0 as uint64_t
            || crate::raw_header_word(&raw const (*h).long_gen) >= 1 as uint64_t && crate::raw_header_word(&raw const (*h).long_gen) < 0xffffffff as uint64_t)
        && (crate::raw_header_word(&raw const (*h).off_long) != 0 as uint64_t) as ::core::ffi::c_int
            == (n != 0 as uint64_t && (*ctx).spm.is_null()) as ::core::ffi::c_int
    {
        epoch = crate::raw_header_word(&raw const (*h).epoch).wrapping_add(1 as uint64_t);
        gen = crate::raw_header_word(&raw const (*h).long_gen).wrapping_add(1 as uint64_t);
    }
    memset(
        h as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<toks_scratch>() as size_t,
    );
    (*h).magic = TOKS_SCRATCH_MAGIC as uint64_t;
    (*h).identity = (*ctx).identity;
    (*h).base = base;
    (*h).bytes = bytes;
    (*h).max_len = lo;
    (*h).off_cache = a.wrapping_add(TOKS_SCR_FIXED);
    (*h).off_work = (*h).off_cache.wrapping_add(toks_scr_caches(n)).wrapping_add(memo);
    (*h).off_bounce = (*h)
        .off_work
        .wrapping_add(toks_scr_work(toks_scr_tmax(lo, scr_x(ctx))))
        .wrapping_add(extra);
    (*h).epoch = if epoch != 0 as uint64_t { epoch } else { 1 as uint64_t };
    (*h).cache_mib = n;
    (*h).off_long = if n != 0 as uint64_t && (*ctx).spm.is_null() {
        (*h).off_cache.wrapping_add(toks_scr_short(n))
    } else {
        0 as uint64_t
    };
    (*h).long_gen = if n != 0 as uint64_t {
        if epoch != 0 as uint64_t { gen } else { 1 as uint64_t }
    } else {
        0 as uint64_t
    };
    toks_scr_carved(ctx as *const toks_ctx, h);
    if epoch == 0 as uint64_t && n != 0 as uint64_t {
        toks_plat_hint_huge(
            s0.offset((*h).off_cache as isize) as *mut ::core::ffi::c_void,
            toks_scr_caches(n),
        );
    }
    if epoch == 0 as uint64_t {
        toks_scr_zero(
            h,
            s0.offset((*h).off_cache as isize),
            if (*h).off_long != 0 as uint64_t {
                toks_scr_short(n).wrapping_add(toks_scr_long_buckets(n))
            } else {
                toks_scr_caches(n)
            },
        );
    }
    if memo != 0 as uint64_t {
        toks_scr_zero(
            h,
            s0.offset((*h).off_cache as isize).offset(toks_scr_caches(n) as isize),
            if epoch == 0 as uint64_t {
                (64 as uint64_t).wrapping_add(memo.wrapping_div(32 as uint64_t))
            } else {
                64 as uint64_t
            },
        );
    }
    return 0 as int64_t;
}
unsafe extern "C" fn scratch_get(
    mut ctx: *const toks_ctx,
    mut scr: *mut ::core::ffi::c_void,
    mut len: uint64_t,
) -> *mut toks_scratch {
    if scr.is_null() {
        return ::core::ptr::null_mut::<toks_scratch>();
    }
    let mut base: uint64_t = scr as uintptr_t as uint64_t;
    let mut h: *mut toks_scratch = toks_scr_header(scr);
    if crate::raw_header_word(&raw const (*h).magic) as ::core::ffi::c_ulonglong != TOKS_SCRATCH_MAGIC
        || crate::raw_header_word(&raw const (*h).identity) != (*ctx).identity || crate::raw_header_word(&raw const (*h).base) != base || len > crate::raw_header_word(&raw const (*h).max_len)
    {
        return ::core::ptr::null_mut::<toks_scratch>();
    }
    return h;
}
unsafe extern "C" fn emit1(mut e: *mut emit, mut v: uint32_t) {
    if (*e).n < (*e).cap {
        toks_st32((*e).out.offset((*e).n as isize) as *mut ::core::ffi::c_void, v);
    }
    (*e).n = (*e).n.wrapping_add(1);
}
unsafe extern "C" fn k5_run(
    mut ctx: *const toks_ctx,
    mut h: *mut toks_scratch,
    mut text: *const uint8_t,
    mut len: uint64_t,
    mut ends: *const uint32_t,
    mut n: uint64_t,
    mut start: uint64_t,
    mut e: *mut emit,
) {
    let mut s: *mut uint8_t = (*h).base as uintptr_t as *mut uint8_t;
    let mut left: uint64_t = if (*e).n < (*e).cap {
        (*e).cap.wrapping_sub((*e).n)
    } else {
        0 as uint64_t
    };
    let mut direct: ::core::ffi::c_int = (left
        >= (*ends.offset(n.wrapping_sub(1 as uint64_t) as isize) as uint64_t)
            .wrapping_sub(start)
            .wrapping_add(4 as uint64_t)) as ::core::ffi::c_int;
    let mut bounce: *mut uint32_t = toks_scr_p(h, s.offset((*h).off_bounce as isize))
        as *mut ::core::ffi::c_void as *mut uint32_t;
    let mut k: toks_k5_args = toks_k5_args {
        text: ::core::ptr::null::<uint8_t>(),
        len: 0,
        ends: ::core::ptr::null::<uint32_t>(),
        n: 0,
        start: 0,
        out: ::core::ptr::null_mut::<uint32_t>(),
        room: 0,
        n_out: 0,
        cache: ::core::ptr::null_mut::<uint8_t>(),
        cache_mask: 0,
        work: ::core::ptr::null_mut::<uint8_t>(),
        work_bytes: 0,
        hits_static: 0,
        hits_cache: 0,
        misses: 0,
        cache_tag: 0,
        lcache: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    };
    k.text = text;
    k.len = len;
    k.ends = ends;
    k.n = n;
    k.start = start;
    k.out = if direct != 0 { (*e).out.offset((*e).n as isize) } else { bounce };
    k.room = if direct != 0 {
        left
    } else {
        toks_scr_tmax((*h).max_len, scr_x(ctx)).wrapping_add(4 as uint64_t)
    };
    k.n_out = 0 as uint64_t;
    let mut warm: ::core::ffi::c_int = ((*h)
        .hits_static
        .wrapping_add((*h).hits_cache)
        .wrapping_add((*h).misses) >= TOKS_K5_WARM as uint64_t) as ::core::ffi::c_int;
    k.cache = toks_scr_p(h, s.offset((*h).off_cache as isize));
    k.cache_mask = (if warm != 0 {
        toks_scr_short((*h).cache_mib)
    } else {
        (32768 as ::core::ffi::c_uint as uint64_t).wrapping_mul(64 as uint64_t)
    })
        .wrapping_div(64 as uint64_t)
        .wrapping_sub(1 as uint64_t);
    k.cache_mask = crate::test_hash(k.cache_mask);
    k.work = toks_scr_p(h, s.offset((*h).off_work as isize));
    k.work_bytes = (*h)
        .off_bounce
        .wrapping_sub((*h).off_work)
        .wrapping_sub((*ctx).scr_extra);
    k.hits_static = (*h).hits_static;
    k.hits_cache = (*h).hits_cache;
    k.misses = (*h).misses;
    k.cache_tag = (*h).epoch;
    let mut nb: uint64_t = toks_scr_long_buckets((*h).cache_mib);
    let mut lc: toks_lcache = toks_lcache {
        buckets: toks_scr_p(h, s.offset((*h).off_long as isize)),
        arena: toks_scr_p(h, s.offset((*h).off_long as isize).offset(nb as isize)),
        mask: crate::test_hash(nb.wrapping_div(64 as uint64_t).wrapping_sub(1 as uint64_t)),
        arena_bytes: toks_scr_long_arena((*h).cache_mib),
        pos: (*h).long_pos,
        gen: (*h).long_gen,
        hits: 0 as uint64_t,
        misses: 0 as uint64_t,
    };
    k.lcache = (if (*h).off_long != 0 as uint64_t && warm != 0 {
        &raw mut lc
    } else {
        ::core::ptr::null_mut::<toks_lcache>()
    }) as *mut ::core::ffi::c_void;
    let mut m: uint64_t = toks_k5(&raw const (*ctx).t, &raw mut k, (*ctx).tier);
    (*h).hits_static = k.hits_static;
    (*h).hits_cache = k.hits_cache;
    (*h).misses = k.misses;
    (*h).long_pos = lc.pos;
    (*h).long_gen = lc.gen;
    if direct == 0 && left != 0 as uint64_t {
        toks_cpy(
            (*e).out.offset((*e).n as isize) as *mut ::core::ffi::c_void,
            bounce as *const ::core::ffi::c_void,
            (if left < m { left } else { m }).wrapping_mul(4 as uint64_t),
        );
    }
    (*e).n = (*e).n.wrapping_add(m);
}
unsafe extern "C" fn dropped(
    mut ctx: *const toks_ctx,
    mut c: uint8_t,
) -> ::core::ffi::c_int {
    return (((*ctx).drop[(c as ::core::ffi::c_int >> 3 as ::core::ffi::c_int) as usize]
        as ::core::ffi::c_int >> (c as ::core::ffi::c_uint & 7 as ::core::ffi::c_uint))
        as ::core::ffi::c_uint & 1 as ::core::ffi::c_uint) as ::core::ffi::c_int;
}
unsafe extern "C" fn any_dropped(
    mut ctx: *const toks_ctx,
    mut p: *const uint8_t,
    mut n: uint64_t,
) -> ::core::ffi::c_int {
    let mut i: uint64_t = 0 as uint64_t;
    while i < n {
        if dropped(ctx, *p.offset(i as isize)) != 0 {
            return 1 as ::core::ffi::c_int;
        }
        i = i.wrapping_add(1);
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn run_drop(
    mut ctx: *const toks_ctx,
    mut h: *mut toks_scratch,
    mut seg: *const uint8_t,
    mut pos: uint64_t,
    mut ends: *const uint32_t,
    mut n: uint64_t,
    mut e: *mut emit,
) {
    let mut norm: *mut uint8_t = toks_scr_p(
        h,
        ((*h).base as uintptr_t as *mut uint8_t)
            .offset((*h).off_bounce as isize)
            .offset(toks_scr_bounce(toks_scr_tmax((*h).max_len, scr_x(ctx))) as isize),
    );
    let mut in_norm: ::core::ffi::c_int = (seg >= norm as *const uint8_t
        && seg
            < norm.offset(toks_scr_norm((*h).max_len, scr_x(ctx)) as isize)
                as *const uint8_t) as ::core::ffi::c_int;
    let mut i: uint64_t = 0 as uint64_t;
    let mut st: uint64_t = pos;
    while i < n {
        let mut j: uint64_t = i;
        let mut a: uint64_t = st;
        while j < n
            && any_dropped(
                ctx,
                seg.offset(a as isize),
                (*ends.offset(j as isize) as uint64_t).wrapping_sub(a),
            ) == 0
        {
            a = *ends.offset(j as isize) as uint64_t;
            j = j.wrapping_add(1);
        }
        if j > i {
            k5_run(
                ctx,
                h,
                seg,
                *ends.offset(j.wrapping_sub(1 as uint64_t) as isize) as uint64_t,
                ends.offset(i as isize),
                j.wrapping_sub(i),
                st,
                e,
            );
            st = *ends.offset(j.wrapping_sub(1 as uint64_t) as isize) as uint64_t;
        }
        if j == n {
            break;
        }
        let mut dst: *mut uint8_t = if in_norm != 0 {
            seg.offset(st as isize) as uintptr_t as *mut uint8_t
        } else {
            norm
        };
        let mut m: uint32_t = 0 as uint32_t;
        let mut unk: uint32_t = 0 as uint32_t;
        let mut k: uint64_t = st;
        while k < *ends.offset(j as isize) as uint64_t {
            if dropped(ctx, *seg.offset(k as isize)) == 0 {
                let fresh0 = m;
                m = m.wrapping_add(1);
                *dst.offset(fresh0 as isize) = *seg.offset(k as isize);
                unk = 0 as ::core::ffi::c_uint as uint32_t;
            } else if (*ctx).drop_unk != 0 as uint32_t {
                if m != 0 as uint32_t {
                    k5_run(
                        ctx,
                        h,
                        dst,
                        m as uint64_t,
                        &raw mut m,
                        1 as uint64_t,
                        0 as uint64_t,
                        e,
                    );
                }
                if unk == 0 as uint32_t || (*ctx).drop_fuse == 0 as uint32_t {
                    emit1(e, (*ctx).drop_unk.wrapping_sub(1 as uint32_t));
                }
                m = 0 as ::core::ffi::c_uint as uint32_t;
                unk = 1 as ::core::ffi::c_uint as uint32_t;
            }
            k = k.wrapping_add(1);
        }
        if m != 0 as uint32_t {
            k5_run(
                ctx,
                h,
                dst,
                m as uint64_t,
                &raw mut m,
                1 as uint64_t,
                0 as uint64_t,
                e,
            );
        }
        st = *ends.offset(j as isize) as uint64_t;
        i = j.wrapping_add(1 as uint64_t);
    }
}
#[no_mangle]
pub unsafe extern "C" fn toks_round(
    mut ctx: *const toks_ctx,
    mut h: *mut toks_scratch,
    mut seg: *const uint8_t,
    mut len: uint64_t,
    mut pos: uint64_t,
    mut ends: *const uint32_t,
    mut n: uint64_t,
    mut base: uint64_t,
    mut e: *mut emit,
    mut ids: ::core::ffi::c_int,
) {
    if ids == 0 as ::core::ffi::c_int {
        let mut i: uint64_t = 0 as uint64_t;
        while i < n {
            emit1(
                e,
                base.wrapping_add(*ends.offset(i as isize) as uint64_t) as uint32_t,
            );
            i = i.wrapping_add(1);
        }
    } else if (*ctx).has_drop != 0 as uint32_t
        && any_dropped(
            ctx,
            seg.offset(pos as isize),
            (*ends.offset(n.wrapping_sub(1 as uint64_t) as isize) as uint64_t)
                .wrapping_sub(pos),
        ) != 0
    {
        run_drop(ctx, h, seg, pos, ends, n, e);
    } else {
        k5_run(ctx, h, seg, len, ends, n, pos, e);
    };
}
unsafe extern "C" fn run_text(
    mut ctx: *const toks_ctx,
    mut h: *mut toks_scratch,
    mut seg: *const uint8_t,
    mut len: uint64_t,
    mut base: uint64_t,
    mut e: *mut emit,
    mut ids: ::core::ffi::c_int,
    mut at_start: ::core::ffi::c_int,
) {
    let mut s: *mut uint8_t = (*h).base as uintptr_t as *mut uint8_t;
    let mut ends: *mut uint32_t = toks_scr_p(
        h,
        (h as *mut uint8_t).offset(TOKS_SCR_HDR as isize),
    ) as *mut ::core::ffi::c_void as *mut uint32_t;
    if !(*ctx).spm.is_null() {
        if ids != 0 as ::core::ffi::c_int {
            (*e).n = toks_spm_encode(
                &raw const (*ctx).t,
                (*ctx).spm as *const toks_spm,
                seg,
                len,
                at_start & 1 as ::core::ffi::c_int,
                (*e).out,
                (*e).cap,
                (*e).n,
                toks_scr_p(h, s.offset((*h).off_cache as isize)),
                crate::test_hash(toks_scr_caches((*h).cache_mib)
                    .wrapping_div(64 as uint64_t)
                    .wrapping_sub(1 as uint64_t)),
                (*h).epoch,
                toks_scr_p(h, s.offset((*h).off_work as isize)),
                at_start as uint32_t & TOKS_SPM_NOPFX as uint32_t
                    | (*ctx).tier << 8 as ::core::ffi::c_int,
            );
        } else {
            (*e).n = toks_spm_pieces(
                (*ctx).spm as *const toks_spm,
                seg,
                len,
                at_start & 1 as ::core::ffi::c_int,
                base,
                (*e).out,
                (*e).cap,
                (*e).n,
            );
        }
        return;
    }
    if !(*ctx).gen.is_null() {
        toks_gen_run(
            ctx as *const toks_ctx,
            h,
            seg,
            len,
            base,
            e as *mut toks_emit,
            ids,
        );
        return;
    }
    let mut pos: uint64_t = 0 as uint64_t;
    let mut tier: uint32_t = if (*ctx).t.flags & TOKS_TF_TWIN as uint32_t
        != 0 as uint32_t
    {
        TOKS_TIER_SCALAR as uint32_t
    } else {
        (*ctx).tier
    };
    while pos < len {
        let mut n: uint64_t = 0;
        let mut end: uint64_t = 0;
        if (*ctx).t.tmpl != TOKS_TMPL_NONE as uint32_t {
            let mut k: toks_k3_args = toks_k3_args {
                text: ::core::ptr::null::<uint8_t>(),
                len: 0,
                pos: 0,
                ends: ::core::ptr::null_mut::<uint32_t>(),
                cap: 0,
                n: 0,
                flags: 0,
                rsv: 0,
            };
            k.text = seg;
            k.len = len;
            k.pos = pos;
            k.ends = ends;
            k.cap = TOKS_CHUNK_PIECES as uint64_t;
            k.n = 0 as uint64_t;
            k.flags = 0 as uint64_t;
            k.rsv = 0 as uint64_t;
            n = if (*ctx).t.tmpl == TOKS_TMPL_O200K as uint32_t {
                toks_k3_o200k(&raw const (*ctx).t, &raw mut k, tier)
            } else if (*ctx).t.tmpl == TOKS_TMPL_DSV3 as uint32_t {
                toks_k3_dsv3(&raw const (*ctx).t, &raw mut k, tier)
            } else {
                toks_k3(&raw const (*ctx).t, &raw mut k, tier)
            };
            end = k.pos;
            if n == 0 as uint64_t || end <= pos || end > len {
                return;
            }
        } else {
            *ends.offset(0 as ::core::ffi::c_int as isize) = len as uint32_t;
            n = 1 as uint64_t;
            end = len;
        }
        toks_round(ctx, h, seg, len, pos, ends, n, base, e, ids);
        pos = end;
    }
}
unsafe extern "C" fn run_units(
    mut ctx: *const toks_ctx,
    mut h: *mut toks_scratch,
    mut seg: *const uint8_t,
    mut n: uint64_t,
    mut nb: uint64_t,
    mut mode: uint32_t,
    mut e: *mut emit,
    mut ids: ::core::ffi::c_int,
    mut at_start: ::core::ffi::c_int,
) {
    if mode == TOKS_ADDED_NONE as uint32_t
        || (*ctx).t.add_phases & 2 as uint64_t == 0 as uint64_t
    {
        run_text(ctx, h, seg, n, nb, e, ids, at_start);
        return;
    }
    let mut it1: toks_seg_iter = toks_seg_iter {
        t: ::core::ptr::null::<toks_tables>(),
        text: ::core::ptr::null::<uint8_t>(),
        len: 0,
        pos: 0,
        prev_end: 0,
        pend_start: 0,
        pend_end: 0,
        rs_from: 0,
        rs_to: 0,
        pend_id: 0,
        mode: 0,
        phase: 0,
        tier: 0,
        fin: 0,
        pend: 0,
        mi: 0,
        mn: 0,
        m: [toks_k1_match {
            start: 0,
            end: 0,
            entry: 0,
            rsv: 0,
        }; 16],
    };
    let mut v: toks_seg_out = toks_seg_out {
        kind: 0,
        id: 0,
        start: 0,
        end: 0,
    };
    toks_seg_begin(
        &raw mut it1,
        &raw const (*ctx).t,
        mode,
        1 as uint32_t,
        (*ctx).tier,
        seg,
        n,
    );
    let mut pfx: ::core::ffi::c_int = (!(*ctx).spm.is_null()
        && (*(*ctx).spm).pfx_mode == TOKS_SPM_PFX_GAP as ::core::ffi::c_int as uint32_t)
        as ::core::ffi::c_int;
    while toks_seg_next(&raw mut it1, &raw mut v) != 0 {
        if v.kind == TOKS_SEG_TOKEN as uint32_t {
            if pfx != 0 && v.start == 0 as uint64_t
                && (*seg.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint
                    == 0x20 as ::core::ffi::c_uint
                    || toks_meta_at(seg, n, 0 as uint64_t) != 0)
            {
                if ids != 0 {
                    run_text(
                        ctx,
                        h,
                        b"\xE2\x96\x81\0" as *const u8 as *const ::core::ffi::c_char
                            as *const uint8_t,
                        3 as uint64_t,
                        nb,
                        e,
                        ids,
                        TOKS_SPM_NOPFX as ::core::ffi::c_int,
                    );
                } else {
                    emit1(
                        e,
                        nb
                            .wrapping_add(
                                (if *seg.offset(0 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint == 0x20 as ::core::ffi::c_uint
                                {
                                    1 as ::core::ffi::c_uint
                                } else {
                                    3 as ::core::ffi::c_uint
                                }) as uint64_t,
                            ) as uint32_t,
                    );
                }
            }
            emit1(e, if ids != 0 { v.id } else { nb.wrapping_add(v.end) as uint32_t });
        } else {
            run_text(
                ctx,
                h,
                seg.offset(v.start as isize),
                v.end.wrapping_sub(v.start),
                nb.wrapping_add(v.start),
                e,
                ids,
                (at_start != 0 && v.start == 0 as uint64_t) as ::core::ffi::c_int
                    | (if pfx != 0 && v.start != 0 as uint64_t {
                        TOKS_SPM_NOPFX as ::core::ffi::c_int
                    } else {
                        0 as ::core::ffi::c_int
                    }),
            );
        }
    }
}
unsafe extern "C" fn run_gap(
    mut ctx: *const toks_ctx,
    mut h: *mut toks_scratch,
    mut g: *const uint8_t,
    mut n: uint64_t,
    mut nb: uint64_t,
    mut mode: uint32_t,
    mut e: *mut emit,
    mut ids: ::core::ffi::c_int,
    mut at_start: ::core::ffi::c_int,
) -> int64_t {
    let mut pl: toks_nfc_plan = toks_nfc_plan {
        r: 0,
        d0: 0,
        d1: 0,
        f: 0,
    };
    pl.d0 = n;
    let mut hb: uint8_t = (if (*ctx).nfc & TOKS_NS_COMPAT as uint32_t != 0 as uint32_t {
        TOKS_NFKC_HOT_BYTE
    } else {
        TOKS_NFC_HOT_BYTE
    }) as uint8_t;
    if (*ctx).nfc != 0 as uint32_t && toks_k2(g, 0 as uint64_t, n, hb) < n {
        toks_nfc_plan_begin(&raw mut pl, (*ctx).nfc, g, n);
    }
    if pl.d0 == n {
        run_units(ctx, h, g, n, nb, mode, e, ids, at_start);
        return n as int64_t;
    }
    let mut buf: *mut uint8_t = toks_scr_p(
        h,
        ((*h).base as uintptr_t as *mut uint8_t)
            .offset((*h).off_bounce as isize)
            .offset(toks_scr_bounce(toks_scr_tmax((*h).max_len, scr_x(ctx))) as isize),
    );
    let mut m: int64_t = 0;
    if mode != TOKS_ADDED_NONE as uint32_t
        && (*ctx).t.add_phases & 2 as uint64_t != 0 as uint64_t || !(*ctx).gen.is_null()
    {
        m = toks_norm(
            (*ctx).nfc,
            g,
            n,
            buf,
            ((if (*ctx).nfc & TOKS_NS_COMPAT as uint32_t != 0 as uint32_t {
                TOKS_NFKC_X
            } else {
                TOKS_NFC_X
            }) as uint64_t)
                .wrapping_mul(n),
        );
        if m < 0 as int64_t {
            return m;
        }
        run_units(ctx, h, buf, m as uint64_t, nb, mode, e, ids, at_start);
        return m;
    }
    let mut r: uint64_t = 0 as uint64_t;
    let mut o: uint64_t = nb;
    let mut s: uint64_t = 0;
    let mut t: uint64_t = 0;
    while toks_nfc_plan_next(
        &raw mut pl,
        &raw const (*ctx).t,
        g,
        n,
        &raw mut s,
        &raw mut t,
    ) != 0
    {
        if s > r {
            run_text(
                ctx,
                h,
                g.offset(r as isize),
                s.wrapping_sub(r),
                o,
                e,
                ids,
                (at_start != 0 && r == 0 as uint64_t) as ::core::ffi::c_int,
            );
            o = o.wrapping_add(s.wrapping_sub(r));
        }
        m = toks_norm(
            (*ctx).nfc,
            g.offset(s as isize),
            t.wrapping_sub(s),
            buf,
            ((if (*ctx).nfc & TOKS_NS_COMPAT as uint32_t != 0 as uint32_t {
                TOKS_NFKC_X
            } else {
                TOKS_NFC_X
            }) as uint64_t)
                .wrapping_mul(t.wrapping_sub(s)),
        );
        if m < 0 as int64_t {
            return m;
        }
        run_text(
            ctx,
            h,
            buf,
            m as uint64_t,
            o,
            e,
            ids,
            (at_start != 0 && s == 0 as uint64_t) as ::core::ffi::c_int,
        );
        o = o.wrapping_add(m as uint64_t);
        r = t;
    }
    if r < n {
        run_text(
            ctx,
            h,
            g.offset(r as isize),
            n.wrapping_sub(r),
            o,
            e,
            ids,
            (at_start != 0 && r == 0 as uint64_t) as ::core::ffi::c_int,
        );
        o = o.wrapping_add(n.wrapping_sub(r));
    }
    return o.wrapping_sub(nb) as int64_t;
}
#[cfg_attr(feature = "test-internals", no_mangle)]
pub unsafe extern "C" fn py_isspace(mut c: uint32_t) -> ::core::ffi::c_int {
    if c < 0x80 as uint32_t {
        return (c >= 0x9 as uint32_t && c <= 0xd as uint32_t
            || c >= 0x1c as uint32_t && c <= 0x20 as uint32_t) as ::core::ffi::c_int;
    }
    return (c == 0x85 as uint32_t || c == 0xa0 as uint32_t || c == 0x1680 as uint32_t
        || c >= 0x2000 as uint32_t && c <= 0x200a as uint32_t || c == 0x2028 as uint32_t
        || c == 0x2029 as uint32_t || c == 0x202f as uint32_t || c == 0x205f as uint32_t
        || c == 0x3000 as uint32_t) as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn run8(
    mut w: uint64_t,
    mut run_0: *mut uint32_t,
    mut sp_run: *mut ::core::ffi::c_int,
    mut cut_run: uint32_t,
) -> ::core::ffi::c_int {
    let mut s: uint64_t = (((w as ::core::ffi::c_ulonglong)
        .wrapping_add(0x7777777777777777 as ::core::ffi::c_ulonglong)
        & !(w as ::core::ffi::c_ulonglong)
            .wrapping_add(0x7272727272727272 as ::core::ffi::c_ulonglong)
        | (w as ::core::ffi::c_ulonglong)
            .wrapping_add(0x6464646464646464 as ::core::ffi::c_ulonglong)
            & !(w as ::core::ffi::c_ulonglong)
                .wrapping_add(0x5f5f5f5f5f5f5f5f as ::core::ffi::c_ulonglong))
        & 0x8080808080808080 as ::core::ffi::c_ulonglong) as uint64_t;
    let mut o: uint64_t = ((if *sp_run != 0 as ::core::ffi::c_int { !s } else { s })
        as ::core::ffi::c_ulonglong & 0x8080808080808080 as ::core::ffi::c_ulonglong)
        as uint64_t;
    if cut_run < 8 as uint32_t
        || (*run_0)
            .wrapping_add(
                (if o != 0 as uint64_t {
                    (o as ::core::ffi::c_ulonglong).trailing_zeros() as i32 as uint32_t
                        >> 3 as ::core::ffi::c_int
                } else {
                    8 as uint32_t
                }),
            ) > cut_run
    {
        return 0 as ::core::ffi::c_int;
    }
    if o == 0 as uint64_t {
        *run_0 = (*run_0 as ::core::ffi::c_uint).wrapping_add(8 as ::core::ffi::c_uint)
            as uint32_t as uint32_t;
        return 1 as ::core::ffi::c_int;
    }
    *sp_run = (s >> 63 as ::core::ffi::c_int) as ::core::ffi::c_int;
    o = ((if *sp_run != 0 as ::core::ffi::c_int { !s } else { s })
        as ::core::ffi::c_ulonglong & 0x8080808080808080 as ::core::ffi::c_ulonglong)
        as uint64_t;
    *run_0 = if o != 0 as uint64_t {
        (o as ::core::ffi::c_ulonglong).leading_zeros() as i32 as uint32_t
            >> 3 as ::core::ffi::c_int
    } else {
        8 as uint32_t
    };
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn run_piece(
    mut ctx: *const toks_ctx,
    mut h: *mut toks_scratch,
    mut seg: *const uint8_t,
    mut len: uint64_t,
    mut base: uint64_t,
    mut specials: ::core::ffi::c_int,
    mut e: *mut emit,
    mut ids: ::core::ffi::c_int,
    mut at_start: ::core::ffi::c_int,
) {
    if specials == 0 as ::core::ffi::c_int
        || (*ctx).t.add_phases & 2 as uint64_t == 0 as uint64_t
    {
        run_text(ctx, h, seg, len, base, e, ids, at_start);
        return;
    }
    let mut it: toks_seg_iter = toks_seg_iter {
        t: ::core::ptr::null::<toks_tables>(),
        text: ::core::ptr::null::<uint8_t>(),
        len: 0,
        pos: 0,
        prev_end: 0,
        pend_start: 0,
        pend_end: 0,
        rs_from: 0,
        rs_to: 0,
        pend_id: 0,
        mode: 0,
        phase: 0,
        tier: 0,
        fin: 0,
        pend: 0,
        mi: 0,
        mn: 0,
        m: [toks_k1_match {
            start: 0,
            end: 0,
            entry: 0,
            rsv: 0,
        }; 16],
    };
    let mut v: toks_seg_out = toks_seg_out {
        kind: 0,
        id: 0,
        start: 0,
        end: 0,
    };
    toks_seg_begin(
        &raw mut it,
        &raw const (*ctx).t,
        TOKS_ADDED_ALL as uint32_t,
        1 as uint32_t,
        (*ctx).tier,
        seg,
        len,
    );
    while toks_seg_next(&raw mut it, &raw mut v) != 0 {
        if v.kind == TOKS_SEG_TOKEN as uint32_t {
            emit1(e, if ids != 0 { v.id } else { base.wrapping_add(v.end) as uint32_t });
        } else {
            run_text(
                ctx,
                h,
                seg.offset(v.start as isize),
                v.end.wrapping_sub(v.start),
                base.wrapping_add(v.start),
                e,
                ids,
                (at_start != 0 && v.start == 0 as uint64_t) as ::core::ffi::c_int,
            );
        }
    }
}
#[cfg_attr(feature = "test-internals", no_mangle)]
pub unsafe extern "C" fn run_cuts(
    mut ctx: *const toks_ctx,
    mut h: *mut toks_scratch,
    mut t: *const uint8_t,
    mut len: uint64_t,
    mut base: uint64_t,
    mut specials: ::core::ffi::c_int,
    mut e: *mut emit,
    mut ids: ::core::ffi::c_int,
    mut at_start: ::core::ffi::c_int,
) {
    let mut piece: uint64_t = 0 as uint64_t;
    let mut pos: uint64_t = if len <= (*ctx).cut_run as uint64_t
        && len <= (*ctx).cut_chunk as uint64_t
    {
        len
    } else {
        0 as uint64_t
    };
    let mut in_chunk: uint32_t = 0 as uint32_t;
    let mut run_0: uint32_t = 0 as uint32_t;
    let mut sp_run: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while pos < len {
        let mut k: uint32_t = toks_utf8_len(
            t.offset(pos as isize),
            len.wrapping_sub(pos),
        );
        let mut cp: uint32_t = *t.offset(pos as isize) as uint32_t;
        if k == 1 as uint32_t {
            if len.wrapping_sub(pos) >= 8 as uint64_t
                && in_chunk.wrapping_add(8 as uint32_t) <= (*ctx).cut_chunk
            {
                let mut w: uint64_t = bpe_load_le(t.offset(pos as isize), 8 as uint64_t);
                if w as ::core::ffi::c_ulonglong
                    & 0x8080808080808080 as ::core::ffi::c_ulonglong
                    == 0 as ::core::ffi::c_ulonglong
                    && run8(w, &raw mut run_0, &raw mut sp_run, (*ctx).cut_run) != 0
                {
                    in_chunk = (in_chunk as ::core::ffi::c_uint)
                        .wrapping_add(8 as ::core::ffi::c_uint) as uint32_t as uint32_t;
                    pos = pos.wrapping_add(8 as uint64_t);
                    continue;
                }
            }
        } else if k > 1 as uint32_t {
            cp = if cp == 0xc2 as uint32_t
                || cp.wrapping_sub(0xe1 as uint32_t) < 3 as uint32_t
            {
                toks_cp_decode(t.offset(pos as isize), k)
            } else {
                0xffffffff as uint32_t
            };
        } else {
            k = 1 as ::core::ffi::c_uint as uint32_t;
            cp = 0xffffffff as ::core::ffi::c_uint as uint32_t;
        }
        let mut sp: ::core::ffi::c_int = py_isspace(cp);
        if in_chunk == (*ctx).cut_chunk {
            run_piece(
                ctx,
                h,
                t.offset(piece as isize),
                pos.wrapping_sub(piece),
                base.wrapping_add(piece),
                specials,
                e,
                ids,
                (at_start != 0 && piece == 0 as uint64_t) as ::core::ffi::c_int,
            );
            piece = pos;
            in_chunk = 0 as ::core::ffi::c_uint as uint32_t;
        }
        if in_chunk == 0 as uint32_t || sp != sp_run {
            run_0 = 1 as ::core::ffi::c_uint as uint32_t;
            sp_run = sp;
        } else {
            run_0 = run_0.wrapping_add(1);
            if run_0 > (*ctx).cut_run {
                run_piece(
                    ctx,
                    h,
                    t.offset(piece as isize),
                    pos.wrapping_sub(piece),
                    base.wrapping_add(piece),
                    specials,
                    e,
                    ids,
                    (at_start != 0 && piece == 0 as uint64_t) as ::core::ffi::c_int,
                );
                piece = pos;
                run_0 = 1 as ::core::ffi::c_uint as uint32_t;
            }
        }
        in_chunk = in_chunk.wrapping_add(1);
        pos = pos.wrapping_add(k as uint64_t);
    }
    if len != 0 as uint64_t {
        run_piece(
            ctx,
            h,
            t.offset(piece as isize),
            len.wrapping_sub(piece),
            base.wrapping_add(piece),
            specials,
            e,
            ids,
            (at_start != 0 && piece == 0 as uint64_t) as ::core::ffi::c_int,
        );
    }
}
pub const MEMO_MIN: ::core::ffi::c_uint = 256 as ::core::ffi::c_uint;
#[inline]
unsafe extern "C" fn memo_ring(mut mb: uint64_t) -> uint64_t {
    return mb.wrapping_sub(64 as uint64_t).wrapping_sub(mb >> 5 as ::core::ffi::c_int);
}
#[inline]
unsafe extern "C" fn memo_need(mut n: uint64_t, mut k: uint64_t) -> uint64_t {
    return ((32 as ::core::ffi::c_ulonglong)
        .wrapping_add(
            n.wrapping_add(7 as uint64_t) as ::core::ffi::c_ulonglong
                & !(7 as ::core::ffi::c_ulonglong),
        )
        .wrapping_add((4 as uint64_t).wrapping_mul(k) as ::core::ffi::c_ulonglong)
        .wrapping_add(63 as ::core::ffi::c_ulonglong)
        & !(63 as ::core::ffi::c_ulonglong)) as uint64_t;
}
#[inline]
unsafe extern "C" fn memo_at(
    mut m: *mut uint8_t,
    mut mb: uint64_t,
    mut p: uint64_t,
) -> *mut memo_rec {
    return m
        .offset(64 as ::core::ffi::c_uint as isize)
        .offset((mb >> 5 as ::core::ffi::c_int) as isize)
        .offset(p as isize) as *mut ::core::ffi::c_void as *mut memo_rec;
}
#[inline]
unsafe extern "C" fn memo_set(
    mut m: *mut uint8_t,
    mut mb: uint64_t,
    mut hash: uint64_t,
) -> *mut memo_rec {
    return m
        .offset(64 as ::core::ffi::c_uint as isize)
        .offset(
            (64 as uint64_t)
                .wrapping_mul(
                    (hash >> 32 as ::core::ffi::c_int)
                        .wrapping_mul(mb >> 11 as ::core::ffi::c_int)
                        >> 32 as ::core::ffi::c_int,
                ) as isize,
        ) as *mut ::core::ffi::c_void as *mut memo_rec;
}
unsafe extern "C" fn memo_way(
    mut st: *mut memo_rec,
    mut hash: uint64_t,
    mut key: uint32_t,
    mut n: uint64_t,
    mut epoch: uint32_t,
    mut pos: uint64_t,
    mut ring: uint64_t,
    mut rec: ::core::ffi::c_int,
) -> *mut memo_rec {
    let mut mark: [::core::ffi::c_int; 2] = [0; 2];
    let mut w: uint32_t = 0 as uint32_t;
    while w < 2 as uint32_t {
        if (*st.offset(w as isize)).hash == hash && (*st.offset(w as isize)).key == key
            && (*st.offset(w as isize)).len as uint64_t == n
            && (rec != 0 as ::core::ffi::c_int
                || (*st.offset(w as isize)).n_ids == UINT32_MAX as uint32_t
                || (*st.offset(w as isize)).epoch != epoch)
        {
            return st.offset(w as isize) as *mut memo_rec;
        }
        w = w.wrapping_add(1);
    }
    let mut w_0: uint32_t = 0 as uint32_t;
    while w_0 < 2 as uint32_t {
        let mut x: *const memo_rec = st.offset(w_0 as isize) as *mut memo_rec;
        mark[w_0 as usize] = ((*x).n_ids == UINT32_MAX as uint32_t)
            as ::core::ffi::c_int;
        if (*x).len == 0 as uint32_t || (*x).epoch != epoch
            || mark[w_0 as usize] == 0
                && (pos.wrapping_sub((*x).pos) > ring
                    || (*x)
                        .pos
                        .wrapping_rem(ring)
                        .wrapping_add(
                            memo_need((*x).len as uint64_t, (*x).n_ids as uint64_t),
                        ) > ring)
        {
            return st.offset(w_0 as isize) as *mut memo_rec;
        }
        w_0 = w_0.wrapping_add(1);
    }
    if mark[0 as ::core::ffi::c_int as usize] != mark[1 as ::core::ffi::c_int as usize] {
        return if mark[0 as ::core::ffi::c_int as usize] != 0 {
            st.offset(0 as ::core::ffi::c_int as isize) as *mut memo_rec
        } else {
            st.offset(1 as ::core::ffi::c_int as isize) as *mut memo_rec
        };
    }
    if mark[0 as ::core::ffi::c_int as usize] == 0 as ::core::ffi::c_int
        && rec == 0 as ::core::ffi::c_int
    {
        return ::core::ptr::null_mut::<memo_rec>();
    }
    return if (*st.offset(0 as ::core::ffi::c_int as isize)).pos
        <= (*st.offset(1 as ::core::ffi::c_int as isize)).pos
    {
        st.offset(0 as ::core::ffi::c_int as isize) as *mut memo_rec
    } else {
        st.offset(1 as ::core::ffi::c_int as isize) as *mut memo_rec
    };
}
unsafe extern "C" fn memo_hash(
    mut g: *const uint8_t,
    mut n: uint64_t,
    mut seed: uint64_t,
) -> uint64_t {
    let mut x: [uint64_t; 8] = [0; 8];
    let mut w: [uint64_t; 8] = [0; 8];
    let mut at: [*const uint8_t; 3] = [
        g,
        g
            .offset(n.wrapping_div(2 as uint64_t) as isize)
            .offset(-(32 as ::core::ffi::c_uint as isize)),
        g.offset(n as isize).offset(-(64 as ::core::ffi::c_uint as isize)),
    ];
    let mut j: uint64_t = 0 as uint64_t;
    while j < 8 as uint64_t {
        x[j as usize] = (seed as ::core::ffi::c_ulonglong
            ^ (n as ::core::ffi::c_ulonglong)
                .wrapping_add((j as ::core::ffi::c_ulonglong).wrapping_mul(TOKS_FIB64)))
            as uint64_t;
        j = j.wrapping_add(1);
    }
    let mut k: uint64_t = 0 as uint64_t;
    while k < 3 as uint64_t {
        memcpy(
            &raw mut w as *mut uint64_t as *mut ::core::ffi::c_void,
            at[k as usize] as *const ::core::ffi::c_void,
            64 as size_t,
        );
        let mut j_0: uint64_t = 0 as uint64_t;
        while j_0 < 8 as uint64_t {
            x[j_0 as usize] = toks_mix64(x[j_0 as usize], w[j_0 as usize]);
            j_0 = j_0.wrapping_add(1);
        }
        k = k.wrapping_add(1);
    }
    let mut j_1: uint64_t = 1 as uint64_t;
    while j_1 < 8 as uint64_t {
        x[0 as ::core::ffi::c_int as usize] = toks_mix64(
            x[0 as ::core::ffi::c_int as usize],
            x[j_1 as usize],
        );
        j_1 = j_1.wrapping_add(1);
    }
    return crate::test_hash(x[0 as ::core::ffi::c_int as usize]);
}
unsafe extern "C" fn memo_get(
    mut m: *mut uint8_t,
    mut mb: uint64_t,
    mut epoch: uint32_t,
    mut g: *const uint8_t,
    mut n: uint64_t,
    mut key: uint32_t,
    mut hash: uint64_t,
    mut e: *mut emit,
    mut mk: *mut *mut memo_rec,
) -> ::core::ffi::c_int {
    let mut ring: uint64_t = memo_ring(mb);
    let mut hd: *mut toks_memo_head = m as *mut ::core::ffi::c_void
        as *mut toks_memo_head;
    let mut st: *mut memo_rec = memo_set(m, mb, hash);
    let mut s: *mut memo_rec = ::core::ptr::null_mut::<memo_rec>();
    (*hd).probes = (*hd).probes.wrapping_add(1);
    let mut w: uint32_t = 0 as uint32_t;
    while w < 2 as uint32_t && s.is_null() {
        if (*st.offset(w as isize)).hash == hash
            && (*st.offset(w as isize)).epoch == epoch
            && (*st.offset(w as isize)).key == key
            && (*st.offset(w as isize)).len as uint64_t == n
        {
            s = st.offset(w as isize) as *mut memo_rec;
        }
        w = w.wrapping_add(1);
    }
    if s.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    if (*s).n_ids == UINT32_MAX as uint32_t {
        *mk = s;
        return 2 as ::core::ffi::c_int;
    }
    if (*hd).pos.wrapping_sub((*s).pos) > ring
        || (*s).pos.wrapping_rem(ring).wrapping_add(memo_need(n, (*s).n_ids as uint64_t))
            > ring
    {
        return 0 as ::core::ffi::c_int;
    }
    if memcmp(
        memo_at(m, mb, (*s).pos.wrapping_rem(ring))
            .offset(1 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_void,
        g as *const ::core::ffi::c_void,
        n as size_t,
    ) != 0 as ::core::ffi::c_int
    {
        (*hd).differ = (*hd).differ.wrapping_add(1);
        return 0 as ::core::ffi::c_int;
    }
    let mut left: uint64_t = if (*e).n < (*e).cap {
        (*e).cap.wrapping_sub((*e).n)
    } else {
        0 as uint64_t
    };
    let mut k: uint64_t = (*s).n_ids as uint64_t;
    if left != 0 as uint64_t {
        toks_cpy(
            (*e).out.offset((*e).n as isize) as *mut ::core::ffi::c_void,
            (memo_at(m, mb, (*s).pos.wrapping_rem(ring))
                .offset(1 as ::core::ffi::c_int as isize) as *mut uint8_t)
                .offset(
                    (n.wrapping_add(7 as uint64_t) as ::core::ffi::c_ulonglong
                        & !(7 as ::core::ffi::c_ulonglong)) as isize,
                ) as *const ::core::ffi::c_void,
            (4 as uint64_t).wrapping_mul((if left < k { left } else { k })),
        );
    }
    (*e).n = (*e).n.wrapping_add(k);
    (*hd).hits = (*hd).hits.wrapping_add(1);
    (*hd).drought = 0 as uint64_t;
    (*hd).vpos = (*hd).vpos.wrapping_add(memo_need(n, k));
    (*hd).run = 0 as uint64_t;
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn memo_put(
    mut m: *mut uint8_t,
    mut mb: uint64_t,
    mut g: *const uint8_t,
    mut n: uint64_t,
    mut key: uint32_t,
    mut hash: uint64_t,
    mut ids: *const uint32_t,
    mut k: uint64_t,
    mut need: uint64_t,
) {
    let mut hd: *mut toks_memo_head = m as *mut ::core::ffi::c_void
        as *mut toks_memo_head;
    let mut r: *mut memo_rec = memo_at(m, mb, (*hd).pos.wrapping_rem(memo_ring(mb)));
    (*r).hash = hash;
    (*r).key = key;
    (*r).n_ids = k as uint32_t;
    (*r).len = n as uint32_t;
    memcpy(
        r.offset(1 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_void,
        g as *const ::core::ffi::c_void,
        n as size_t,
    );
    toks_cpy(
        (r.offset(1 as ::core::ffi::c_int as isize) as *mut uint8_t)
            .offset(
                (n.wrapping_add(7 as uint64_t) as ::core::ffi::c_ulonglong
                    & !(7 as ::core::ffi::c_ulonglong)) as isize,
            ) as *mut ::core::ffi::c_void,
        ids as *const ::core::ffi::c_void,
        (4 as uint64_t).wrapping_mul(k),
    );
    (*hd).pos = (*hd).pos.wrapping_add(need);
    (*hd).drought = (*hd).drought.wrapping_add(need);
    (*hd).vpos = (*hd).vpos.wrapping_add(need);
}
unsafe extern "C" fn memo_publish(
    mut m: *mut uint8_t,
    mut mb: uint64_t,
    mut epoch: uint32_t,
    mut p: uint64_t,
) {
    let mut ring: uint64_t = memo_ring(mb);
    let mut pos: uint64_t = (*(m as *const ::core::ffi::c_void as *const toks_memo_head))
        .pos;
    while pos.wrapping_sub(p) <= ring && p < pos {
        let mut r: *mut memo_rec = memo_at(m, mb, p.wrapping_rem(ring));
        if (*r).len == 0 as uint32_t {
            p = p.wrapping_add(ring.wrapping_sub(p.wrapping_rem(ring)));
        } else {
            let mut s: *mut memo_rec = memo_way(
                memo_set(m, mb, (*r).hash),
                (*r).hash,
                (*r).key,
                (*r).len as uint64_t,
                epoch,
                pos,
                ring,
                1 as ::core::ffi::c_int,
            );
            *s = *r;
            (*s).pos = p;
            (*s).epoch = epoch;
            (*s).len = 0 as ::core::ffi::c_uint as uint32_t;
            (*s).len = (*r).len;
            p = p.wrapping_add(memo_need((*r).len as uint64_t, (*r).n_ids as uint64_t));
        }
    }
}
unsafe extern "C" fn run_seg(
    mut ctx: *const toks_ctx,
    mut h: *mut toks_scratch,
    mut g: *const uint8_t,
    mut n: uint64_t,
    mut nb: uint64_t,
    mut flags: uint32_t,
    mut mode: uint32_t,
    mut e: *mut emit,
    mut ids: ::core::ffi::c_int,
    mut at_start: ::core::ffi::c_int,
    mut cut: ::core::ffi::c_int,
) -> int64_t {
    let mut mb: uint64_t = 0 as uint64_t;
    let mut hash: uint64_t = 0 as uint64_t;
    let mut n0: uint64_t = (*e).n;
    let mut m: *mut uint8_t = if ids != 0 as ::core::ffi::c_int
        && n >= MEMO_MIN as uint64_t
    {
        toks_scr_memo(h, &raw mut mb)
    } else {
        ::core::ptr::null_mut::<uint8_t>()
    };
    let mut key: uint32_t = flags | (at_start as uint32_t) << 8 as ::core::ffi::c_int
        | (cut as uint32_t) << 9 as ::core::ffi::c_int
        | mode << 10 as ::core::ffi::c_int;
    let mut got: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut mk: *mut memo_rec = ::core::ptr::null_mut::<memo_rec>();
    if !m.is_null() {
        hash = memo_hash(g, n, (*h).identity);
        got = memo_get(m, mb, (*h).epoch as uint32_t, g, n, key, hash, e, &raw mut mk);
        if got == 1 as ::core::ffi::c_int {
            return n as int64_t;
        }
    }
    let mut r: int64_t = n as int64_t;
    if cut != 0 as ::core::ffi::c_int {
        run_cuts(
            ctx,
            h,
            g,
            n,
            nb,
            (mode != TOKS_ADDED_NONE as uint32_t) as ::core::ffi::c_int,
            e,
            ids,
            at_start,
        );
    } else {
        r = run_gap(ctx, h, g, n, nb, mode, e, ids, at_start);
    }
    if !m.is_null() && r >= 0 as int64_t && (*e).n <= (*e).cap {
        let mut hd: *mut toks_memo_head = m as *mut ::core::ffi::c_void
            as *mut toks_memo_head;
        let mut ring: uint64_t = memo_ring(mb);
        let mut need: uint64_t = memo_need(n, (*e).n.wrapping_sub(n0));
        let mut vpos: uint64_t = (*hd).vpos;
        let mut put: ::core::ffi::c_int = (((*hd).drought < ring
            || got == 2 as ::core::ffi::c_int)
            && need <= ring.wrapping_div(2 as uint64_t)) as ::core::ffi::c_int;
        if put != 0 && (*hd).pos.wrapping_sub((*hd).lap).wrapping_add(need) > ring {
            (*hd).run = (*hd)
                .run
                .wrapping_add(
                    (if got == 2 as ::core::ffi::c_int
                        && vpos.wrapping_sub((*mk).pos) <= ring
                    {
                        TOKS_MEMO_DRY.wrapping_div(TOKS_MEMO_GHOSTS)
                    } else {
                        1 as ::core::ffi::c_uint
                    }) as uint64_t,
                );
            if (*hd).run < TOKS_MEMO_DRY as uint64_t {
                put = 0 as ::core::ffi::c_int;
                (*hd).vpos = (*hd).vpos.wrapping_add(need);
            } else {
                if (*hd).pos.wrapping_rem(ring) != 0 as uint64_t {
                    (*memo_at(m, mb, (*hd).pos.wrapping_rem(ring))).len = 0
                        as ::core::ffi::c_uint as uint32_t;
                    (*hd).pos = (*hd)
                        .pos
                        .wrapping_add(ring.wrapping_sub((*hd).pos.wrapping_rem(ring)));
                }
                (*hd).lap = (*hd).pos;
                (*hd).run = 0 as uint64_t;
            }
        }
        let mut s: *mut memo_rec = if put != 0 {
            ::core::ptr::null_mut::<memo_rec>()
        } else {
            memo_way(
                memo_set(m, mb, hash),
                hash,
                key,
                n,
                (*h).epoch as uint32_t,
                (*hd).pos,
                ring,
                0 as ::core::ffi::c_int,
            )
        };
        if put != 0 {
            memo_put(
                m,
                mb,
                g,
                n,
                key,
                hash,
                (*e).out.offset(n0 as isize),
                (*e).n.wrapping_sub(n0),
                need,
            );
        } else if !s.is_null() {
            (*s).hash = hash;
            (*s).key = key;
            (*s).len = n as uint32_t;
            (*s).n_ids = UINT32_MAX as uint32_t;
            (*s).epoch = (*h).epoch as uint32_t;
            (*s).pos = vpos;
        }
    }
    return r;
}
unsafe extern "C" fn run(
    mut ctx: *const toks_ctx,
    mut text_v: *const ::core::ffi::c_void,
    mut len: uint64_t,
    mut flags: uint32_t,
    mut out: *mut uint32_t,
    mut cap: uint64_t,
    mut scr: *mut ::core::ffi::c_void,
    mut ids: ::core::ffi::c_int,
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
    if text_v.is_null() && len != 0 as uint64_t || out.is_null() && cap != 0 as uint64_t
    {
        return TOKS_E_ARG as int64_t;
    }
    if len as ::core::ffi::c_ulonglong > TOKS_MAX_TEXT {
        return TOKS_E_LIMIT as int64_t;
    }
    let mut h: *mut toks_scratch = scratch_get(ctx, scr, len);
    if h.is_null() {
        return TOKS_E_SCRATCH as int64_t;
    }
    if !(*ctx).wp.is_null() {
        return toks_wp_run(
            ctx as *const toks_ctx,
            h,
            text_v as *const uint8_t,
            len,
            flags,
            out,
            cap,
            ids,
        );
    }
    if !(*ctx).uni.is_null() {
        return toks_uni_run(
            ctx as *const toks_ctx,
            h,
            text_v as *const uint8_t,
            len,
            flags,
            out,
            cap,
            ids,
        );
    }
    let mut text: *const uint8_t = text_v as *const uint8_t;
    let mut cont: ::core::ffi::c_int = (flags & TOKS_CONTINUATION as uint32_t
        != 0 as uint32_t) as ::core::ffi::c_int;
    let mut mode: uint32_t = flags & TOKS_ADDED_MASK as uint32_t;
    let mut pp: ::core::ffi::c_int = (ids != 0 as ::core::ffi::c_int
        && flags & TOKS_NO_POSTPROCESS as uint32_t == 0 as uint32_t)
        as ::core::ffi::c_int;
    let mut e: emit = toks_emit {
        out: out,
        cap: cap,
        n: 0 as uint64_t,
        lim: UINT64_MAX as uint64_t,
    };
    let mut mb: uint64_t = 0 as uint64_t;
    let mut memo: *mut uint8_t = if ids != 0 as ::core::ffi::c_int {
        toks_scr_memo(h, &raw mut mb)
    } else {
        ::core::ptr::null_mut::<uint8_t>()
    };
    let mut pos0: uint64_t = if !memo.is_null() {
        (*(memo as *const ::core::ffi::c_void as *const toks_memo_head)).pos
    } else {
        0 as uint64_t
    };
    if pp != 0 {
        let mut i: uint32_t = 0 as uint32_t;
        while i < (*ctx).n_pp_prefix {
            emit1(&raw mut e, (*ctx).pp_ids[i as usize]);
            i = i.wrapping_add(1);
        }
    }
    if (*ctx).cut_run != 0 as uint32_t {
        if mode == TOKS_ADDED_ALL as uint32_t
            && (*ctx).t.add_phases & 1 as uint64_t != 0 as uint64_t
        {
            let mut it: toks_seg_iter = toks_seg_iter {
                t: ::core::ptr::null::<toks_tables>(),
                text: ::core::ptr::null::<uint8_t>(),
                len: 0,
                pos: 0,
                prev_end: 0,
                pend_start: 0,
                pend_end: 0,
                rs_from: 0,
                rs_to: 0,
                pend_id: 0,
                mode: 0,
                phase: 0,
                tier: 0,
                fin: 0,
                pend: 0,
                mi: 0,
                mn: 0,
                m: [toks_k1_match {
                    start: 0,
                    end: 0,
                    entry: 0,
                    rsv: 0,
                }; 16],
            };
            let mut u: toks_seg_out = toks_seg_out {
                kind: 0,
                id: 0,
                start: 0,
                end: 0,
            };
            toks_seg_begin(
                &raw mut it,
                &raw const (*ctx).t,
                TOKS_ADDED_ALL as uint32_t,
                0 as uint32_t,
                (*ctx).tier,
                text,
                len,
            );
            while toks_seg_next(&raw mut it, &raw mut u) != 0 {
                if u.kind == TOKS_SEG_TOKEN as uint32_t {
                    emit1(&raw mut e, if ids != 0 { u.id } else { u.end as uint32_t });
                } else {
                    run_seg(
                        ctx,
                        h,
                        text.offset(u.start as isize),
                        u.end.wrapping_sub(u.start),
                        u.start,
                        flags,
                        mode,
                        &raw mut e,
                        ids,
                        (cont == 0 && u.start == 0 as uint64_t) as ::core::ffi::c_int,
                        1 as ::core::ffi::c_int,
                    );
                }
            }
        } else {
            run_seg(
                ctx,
                h,
                text,
                len,
                0 as uint64_t,
                flags,
                mode,
                &raw mut e,
                ids,
                (cont == 0) as ::core::ffi::c_int,
                1 as ::core::ffi::c_int,
            );
        }
    } else if mode == TOKS_ADDED_NONE as uint32_t || (*ctx).t.add_n == 0 as uint64_t
        || mode == TOKS_ADDED_NONSPECIAL as uint32_t
            && (*ctx).n_nonspecial == 0 as uint32_t
    {
        let mut r: int64_t = run_seg(
            ctx,
            h,
            text,
            len,
            0 as uint64_t,
            flags,
            TOKS_ADDED_NONE as uint32_t,
            &raw mut e,
            ids,
            (cont == 0) as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
        );
        if r < 0 as int64_t {
            return r;
        }
    } else {
        let mut nb: uint64_t = 0 as uint64_t;
        let mut re: uint64_t = 0 as uint64_t;
        let mut it_0: toks_seg_iter = toks_seg_iter {
            t: ::core::ptr::null::<toks_tables>(),
            text: ::core::ptr::null::<uint8_t>(),
            len: 0,
            pos: 0,
            prev_end: 0,
            pend_start: 0,
            pend_end: 0,
            rs_from: 0,
            rs_to: 0,
            pend_id: 0,
            mode: 0,
            phase: 0,
            tier: 0,
            fin: 0,
            pend: 0,
            mi: 0,
            mn: 0,
            m: [toks_k1_match {
                start: 0,
                end: 0,
                entry: 0,
                rsv: 0,
            }; 16],
        };
        let mut u_0: toks_seg_out = toks_seg_out {
            kind: 0,
            id: 0,
            start: 0,
            end: 0,
        };
        toks_seg_begin(
            &raw mut it_0,
            &raw const (*ctx).t,
            mode,
            0 as uint32_t,
            (*ctx).tier,
            text,
            len,
        );
        while toks_seg_next(&raw mut it_0, &raw mut u_0) != 0 {
            let mut s_nb: uint64_t = nb
                .wrapping_sub(
                    (if re > u_0.start {
                        re.wrapping_sub(u_0.start)
                    } else {
                        0 as uint64_t
                    }),
                );
            let mut e_nb: uint64_t = 0;
            if u_0.kind == TOKS_SEG_TOKEN as uint32_t {
                e_nb = s_nb.wrapping_add(u_0.end.wrapping_sub(u_0.start));
                emit1(&raw mut e, if ids != 0 { u_0.id } else { e_nb as uint32_t });
            } else {
                let mut r_0: int64_t = run_seg(
                    ctx,
                    h,
                    text.offset(u_0.start as isize),
                    u_0.end.wrapping_sub(u_0.start),
                    s_nb,
                    flags,
                    mode,
                    &raw mut e,
                    ids,
                    (cont == 0 && u_0.start == 0 as uint64_t) as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                );
                if r_0 < 0 as int64_t {
                    return r_0;
                }
                e_nb = s_nb.wrapping_add(r_0 as uint64_t);
            }
            if u_0.end >= re {
                re = u_0.end;
                nb = e_nb;
            }
        }
    }
    if ids != 0 as ::core::ffi::c_int && cont == 0 && (*ctx).o.trunc_on != 0
        && flags & TOKS_NO_TRUNCATE as uint32_t == 0 as uint32_t
    {
        let mut lim: uint64_t = ((*ctx).o.trunc_max as uint64_t)
            .wrapping_sub(
                (if pp != 0 { (*ctx).n_pp_suffix } else { 0 as uint32_t }) as uint64_t,
            );
        if e.n > lim {
            e.n = lim;
        }
    }
    if pp != 0 {
        let mut i_0: uint32_t = 0 as uint32_t;
        while i_0 < (*ctx).n_pp_suffix {
            emit1(
                &raw mut e,
                (*ctx).pp_ids[(*ctx).n_pp_prefix.wrapping_add(i_0) as usize],
            );
            i_0 = i_0.wrapping_add(1);
        }
    }
    if ids != 0 as ::core::ffi::c_int && cont == 0 && (*ctx).o.pad_on != 0
        && flags & TOKS_NO_PAD as uint32_t == 0 as uint32_t
    {
        toks_pad(&raw const (*ctx).o, &raw mut e);
    }
    if !memo.is_null() && e.n <= cap {
        memo_publish(memo, mb, (*h).epoch as uint32_t, pos0);
    }
    return e.n as int64_t;
}
#[no_mangle]
pub unsafe extern "C" fn toks_encode(
    mut ctx: *const toks_ctx,
    mut text: *const ::core::ffi::c_void,
    mut len: uint64_t,
    mut flags: uint32_t,
    mut out: *mut uint32_t,
    mut cap: uint64_t,
    mut scr: *mut ::core::ffi::c_void,
) -> int64_t {
    return run(ctx, text, len, flags, out, cap, scr, 1 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn toks_pieces(
    mut ctx: *const toks_ctx,
    mut text: *const ::core::ffi::c_void,
    mut len: uint64_t,
    mut flags: uint32_t,
    mut ends: *mut uint32_t,
    mut cap: uint64_t,
    mut scr: *mut ::core::ffi::c_void,
) -> int64_t {
    return run(ctx, text, len, flags, ends, cap, scr, 0 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn toks_template(
    mut ctx: *const toks_ctx,
    mut ids: *mut uint32_t,
    mut type_ids: *mut uint32_t,
    mut cap: uint64_t,
    mut n_prefix: *mut uint32_t,
) -> int64_t {
    if ctx.is_null() || ids.is_null() && cap != 0 as uint64_t {
        return TOKS_E_ARG as int64_t;
    }
    let mut n: uint32_t = (*ctx).n_pp_prefix.wrapping_add((*ctx).n_pp_suffix);
    if !ids.is_null() && cap < n as uint64_t {
        return TOKS_E_CAP as int64_t;
    }
    let mut i: uint32_t = 0 as uint32_t;
    while !ids.is_null() && i < n {
        toks_st32(
            ids.offset(i as isize) as *mut ::core::ffi::c_void,
            (*ctx).pp_ids[i as usize],
        );
        if !type_ids.is_null() {
            toks_st32(
                type_ids.offset(i as isize) as *mut ::core::ffi::c_void,
                (*ctx).pp_type[i as usize],
            );
        }
        i = i.wrapping_add(1);
    }
    if !n_prefix.is_null() {
        *n_prefix = (*ctx).n_pp_prefix;
    }
    return n as int64_t;
}
#[no_mangle]
pub unsafe extern "C" fn toks_encode_bound(
    mut ctx: *const toks_ctx,
    mut len: uint64_t,
) -> uint64_t {
    if ctx.is_null() {
        return 0 as uint64_t;
    }
    let mut num: uint64_t = (*ctx).bound_num as uint64_t;
    let mut den: uint64_t = (*ctx).bound_den as uint64_t;
    let mut q: uint64_t = len.wrapping_div(den);
    if q
        > (UINT64_MAX as uint64_t)
            .wrapping_sub((*ctx).bound_g as uint64_t)
            .wrapping_sub(num)
            .wrapping_div(num)
    {
        return UINT64_MAX as uint64_t;
    }
    let mut b: uint64_t = num
        .wrapping_mul(q)
        .wrapping_add(
            num
                .wrapping_mul(len.wrapping_rem(den))
                .wrapping_add(den)
                .wrapping_sub(1 as uint64_t)
                .wrapping_div(den),
        )
        .wrapping_add((*ctx).bound_g as uint64_t);
    if (*ctx).o.pad_on != 0 {
        let mut t: uint64_t = if (*ctx).o.pad_fixed != 0 {
            (*ctx).o.pad_len as uint64_t
        } else {
            b
        };
        let mut m: uint64_t = (*ctx).o.pad_multiple as uint64_t;
        if m != 0 as uint64_t && t.wrapping_rem(m) != 0 as uint64_t {
            t = if t > (UINT64_MAX as uint64_t).wrapping_sub(m) {
                UINT64_MAX as uint64_t
            } else {
                t.wrapping_add(m.wrapping_sub(t.wrapping_rem(m)))
            };
        }
        b = if t > b { t } else { b };
    }
    return b;
}
#[no_mangle]
pub unsafe extern "C" fn toks_decode(
    mut ctx: *const toks_ctx,
    mut ids: *const uint32_t,
    mut n: uint64_t,
    mut flags: uint32_t,
    mut out: *mut uint8_t,
    mut cap: uint64_t,
) -> int64_t {
    if ctx.is_null() || ids.is_null() && n != 0 as uint64_t
        || out.is_null() && cap != 0 as uint64_t
    {
        return TOKS_E_ARG as int64_t;
    }
    if flags & !(TOKS_SKIP_SPECIAL as uint32_t | TOKS_DECODE_RAW as uint32_t)
        != 0 as uint32_t
    {
        return TOKS_E_ARG as int64_t;
    }
    let mut i: uint64_t = 0 as uint64_t;
    while i < n {
        if toks_ld32(ids.offset(i as isize) as *const ::core::ffi::c_void)
            >= (*ctx).t.n_ids
        {
            return TOKS_E_ID as int64_t;
        }
        i = i.wrapping_add(1);
    }
    if !(*ctx).wp.is_null() {
        return toks_wp_decode(ctx as *const toks_ctx, ids, n, flags, out, cap);
    }
    if !(*ctx).uni.is_null() {
        return toks_uni_dec(ctx as *const toks_ctx, ids, n, flags, out, cap);
    }
    if !(*ctx).spm.is_null() {
        return toks_spm_decode(
            &raw const (*ctx).t,
            (*ctx).spm as *const toks_spm,
            (*ctx).special_ids,
            flags,
            ids,
            n,
            out,
            cap,
        );
    }
    if flags & TOKS_DECODE_RAW as uint32_t != 0 as uint32_t {
        let mut spec: *const uint32_t = if flags & TOKS_SKIP_SPECIAL as uint32_t
            != 0 as uint32_t
        {
            (*ctx).special_ids
        } else {
            ::core::ptr::null_mut::<uint32_t>()
        };
        let mut d: toks_dsink = toks_dsink {
            out: out,
            cap: cap,
            n: 0 as uint64_t,
        };
        let mut i_0: uint64_t = 0 as uint64_t;
        while i_0 < n {
            let mut id: uint32_t = toks_ld32(
                ids.offset(i_0 as isize) as *const ::core::ffi::c_void,
            );
            let mut o: uint32_t = *(*ctx).t.tok_off.offset(id as isize);
            let mut k: uint64_t = (*(*ctx)
                .t
                .tok_off
                .offset(id.wrapping_add(1 as uint32_t) as isize) as uint64_t)
                .wrapping_sub(o as uint64_t);
            if k != 0 as uint64_t
                && (spec.is_null() || toks_bit(spec, id) == 0 as uint32_t)
            {
                toks_dput(
                    &raw mut d,
                    (*ctx).t.tok_bytes.offset(o as isize) as *const ::core::ffi::c_void,
                    k,
                );
            }
            i_0 = i_0.wrapping_add(1);
        }
        return d.n as int64_t;
    }
    let mut d_0: toks_lossy = toks_lossy {
        out: ::core::ptr::null_mut::<uint8_t>(),
        cap: 0,
        n: 0,
        np: 0,
        pend: [0; 4],
    };
    memset(
        &raw mut d_0 as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<toks_lossy>() as size_t,
    );
    d_0.out = out;
    d_0.cap = cap;
    toks_lossy_ids(ctx as *const toks_ctx, &raw mut d_0, ids, n, flags);
    toks_lossy_end(&raw mut d_0);
    return d_0.n as int64_t;
}
#[no_mangle]
pub unsafe extern "C" fn toks_token(
    mut ctx: *const toks_ctx,
    mut id: uint32_t,
    mut len: *mut uint64_t,
) -> *const uint8_t {
    let mut p: *const uint8_t = ::core::ptr::null::<uint8_t>();
    let mut k: uint64_t = 0 as uint64_t;
    if !ctx.is_null() && id < (*ctx).t.n_ids {
        k = (*(*ctx).t.tok_off.offset(id.wrapping_add(1 as uint32_t) as isize))
            .wrapping_sub(*(*ctx).t.tok_off.offset(id as isize)) as uint64_t;
        if k != 0 as uint64_t {
            p = (*ctx)
                .t
                .tok_bytes
                .offset(*(*ctx).t.tok_off.offset(id as isize) as isize);
        }
    }
    if !len.is_null() {
        *len = k;
    }
    return p;
}
pub const INFO_SIZE_03: ::core::ffi::c_ulong = 184 as ::core::ffi::c_ulong;
#[no_mangle]
pub unsafe extern "C" fn toks_get_info(
    mut ctx: *const toks_ctx,
    mut o: *mut toks_info,
) -> int64_t {
    if ctx.is_null() || o.is_null()
        || (*o).size != ::core::mem::size_of::<toks_info>() as uint32_t
            && (*o).size != INFO_SIZE_03 as uint32_t
    {
        return TOKS_E_ARG as int64_t;
    }
    let mut i: toks_info = toks_info {
        size: 0,
        abi_major: 0,
        abi_minor: 0,
        algorithm: 0,
        tier: 0,
        n_ids: 0,
        n_added: 0,
        paths: 0,
        cpu_features: 0,
        max_text: 0,
        control_isolation: 0,
        rsv: 0,
        source_sha256: [0; 32],
        image_sha256: [0; 32],
        name: [0; 64],
        trunc_on: 0,
        trunc_max: 0,
        trunc_stride: 0,
        pad_on: 0,
        pad_fixed: 0,
        pad_id: 0,
        pad_type_id: 0,
        pad_len: 0,
        pad_multiple: 0,
        pad_left: 0,
        n_template_prefix: 0,
        n_template_suffix: 0,
        seq_type_id: 0,
        rsv2: 0,
    };
    memset(
        &raw mut i as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<toks_info>() as size_t,
    );
    i.size = (*o).size;
    i.abi_major = TOKS_ABI_MAJOR as uint32_t;
    i.abi_minor = TOKS_ABI_MINOR as uint32_t;
    i.algorithm = (*ctx).t.algo;
    i.tier = (*ctx).tier;
    i.n_ids = (*ctx).t.n_ids;
    i.n_added = (*ctx).t.add_n as uint32_t;
    i.paths = ((if !(*ctx).gen.is_null() {
        0 as ::core::ffi::c_uint
    } else {
        TOKS_PATH_SCAN
    }) | TOKS_PATH_NORMALIZE) as uint32_t;
    i.cpu_features = (*ctx).cpu_features;
    i.max_text = TOKS_MAX_TEXT as uint64_t;
    i.control_isolation = 0 as ::core::ffi::c_uint as uint32_t;
    memcpy(
        &raw mut i.source_sha256 as *mut uint8_t as *mut ::core::ffi::c_void,
        &raw const (*ctx).source_sha256 as *const uint8_t as *const ::core::ffi::c_void,
        32 as size_t,
    );
    memcpy(
        &raw mut i.name as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
        &raw const (*ctx).name as *const ::core::ffi::c_char
            as *const ::core::ffi::c_void,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>() as size_t,
    );
    i
        .name[(::core::mem::size_of::<[::core::ffi::c_char; 64]>() as usize)
        .wrapping_sub(1 as usize) as usize] = 0 as ::core::ffi::c_char;
    i.trunc_on = (*ctx).o.trunc_on;
    i.trunc_max = (*ctx).o.trunc_max;
    i.trunc_stride = (*ctx).o.trunc_stride;
    i.pad_on = (*ctx).o.pad_file;
    i.pad_fixed = (*ctx).o.pad_fixed;
    i.pad_id = (*ctx).o.pad_id;
    i.pad_type_id = (*ctx).o.pad_type_id;
    i.pad_len = (*ctx).o.pad_len;
    i.pad_multiple = (*ctx).o.pad_multiple;
    i.pad_left = (*ctx).o.pad_left;
    i.n_template_prefix = (*ctx).n_pp_prefix;
    i.n_template_suffix = (*ctx).n_pp_suffix;
    i.seq_type_id = (*ctx).pp_seq_type;
    memcpy(
        o as *mut ::core::ffi::c_void,
        &raw mut i as *const ::core::ffi::c_void,
        (*o).size as size_t,
    );
    return 0 as int64_t;
}
pub const TOKS_HAVE_K3_CL100K_NEON: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const TOKS_HAVE_K3_O200K_NEON: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const TOKS_HAVE_K3_DSV3_NEON: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const TOKS_HAVE_K5_NEON: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const TOKS_HAVE_K3_CL100K_AVX2: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const TOKS_HAVE_K3_O200K_AVX2: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const TOKS_HAVE_K3_DSV3_AVX2: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const TOKS_HAVE_K5_AVX2: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
