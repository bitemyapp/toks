#[repr(C)]
pub struct toks_gen { _opaque: [u8; 0] }
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
    fn toks_norm(
        steps: uint32_t,
        text: *const uint8_t,
        len: uint64_t,
        out: *mut uint8_t,
        cap: uint64_t,
    ) -> int64_t;
    fn toks_uni_build(
        src: *const toks_uni_src,
        u: *mut *const toks_uni,
        mem: *mut *mut uint8_t,
        mem_len: *mut uint64_t,
        why: *mut *const ::core::ffi::c_char,
    ) -> int64_t;
    fn toks_uni_area(u: *const toks_uni, len: uint64_t) -> uint64_t;
    fn toks_uni_normalize(
        u: *const toks_uni,
        text: *const uint8_t,
        len: uint64_t,
        gap_start: ::core::ffi::c_int,
        dst: *mut uint8_t,
        cap: uint64_t,
    ) -> int64_t;
    fn toks_uni_encode_segment(
        u: *const toks_uni,
        text: *const uint8_t,
        len: uint64_t,
        pre_normalized: ::core::ffi::c_int,
        gap_start: ::core::ffi::c_int,
        c: *mut toks_uni_call,
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
pub struct toks_k1_match {
    pub start: uint32_t,
    pub end: uint32_t,
    pub entry: uint32_t,
    pub rsv: uint32_t,
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
pub struct toks_seg_out {
    pub kind: uint32_t,
    pub id: uint32_t,
    pub start: uint64_t,
    pub end: uint64_t,
}
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
pub const TOKS_UNI_DEC_BFRF: C2RustUnnamed = 3;
pub const TOKS_UNI_DEC_NONE: C2RustUnnamed = 0;
pub type C2RustUnnamed = ::core::ffi::c_uint;
pub const TOKS_UNI_DEC_RBF: C2RustUnnamed = 2;
pub const TOKS_UNI_DEC_META: C2RustUnnamed = 1;
pub const UINT64_MAX: ::core::ffi::c_ulong = 18446744073709551615
    as ::core::ffi::c_ulong;
pub const TOKS_E_UNSUPPORTED: ::core::ffi::c_int = -(3 as ::core::ffi::c_int);
pub const TOKS_E_SCRATCH: ::core::ffi::c_int = -(6 as ::core::ffi::c_int);
pub const TOKS_ADDED_NONSPECIAL: ::core::ffi::c_uint = 1 as ::core::ffi::c_uint;
pub const TOKS_ADDED_NONE: ::core::ffi::c_uint = 2 as ::core::ffi::c_uint;
pub const TOKS_ADDED_MASK: ::core::ffi::c_uint = 3 as ::core::ffi::c_uint;
pub const TOKS_NO_POSTPROCESS: ::core::ffi::c_uint = 4 as ::core::ffi::c_uint;
pub const TOKS_CONTINUATION: ::core::ffi::c_uint = 8 as ::core::ffi::c_uint;
pub const TOKS_NO_TRUNCATE: ::core::ffi::c_uint = 16 as ::core::ffi::c_uint;
pub const TOKS_NO_PAD: ::core::ffi::c_uint = 32 as ::core::ffi::c_uint;
pub const TOKS_BUCKET: ::core::ffi::c_int = 64 as ::core::ffi::c_int;
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
pub const TOKS_CACHE_BUCKETS: ::core::ffi::c_uint = 32768 as ::core::ffi::c_uint;
pub const TOKS_CACHE_BYTES: uint64_t = (TOKS_CACHE_BUCKETS as uint64_t)
    .wrapping_mul(TOKS_BUCKET as uint64_t);
pub const TOKS_NFC_X: ::core::ffi::c_uint = 3 as ::core::ffi::c_uint;
pub const TOKS_NFKC_X: ::core::ffi::c_uint = 11 as ::core::ffi::c_uint;
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
unsafe extern "C" fn toks_scr_short(mut n: uint64_t) -> uint64_t {
    return if n == 0 as uint64_t {
        TOKS_CACHE_BYTES
    } else {
        n << 19 as ::core::ffi::c_int
    };
}
pub const TOKS_SEG_TOKEN: ::core::ffi::c_uint = 1 as ::core::ffi::c_uint;
pub const TOKS_NS_COMPAT: ::core::ffi::c_uint = 0x40 as ::core::ffi::c_uint;
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
unsafe extern "C" fn uni_pre(
    mut c: *const toks_uni_cfg,
    mut t: *const uint8_t,
    mut n: *mut uint64_t,
    mut r: *mut uint8_t,
    mut p: *mut uint8_t,
) -> *const uint8_t {
    let mut s: *const uint8_t = t;
    let mut m: uint64_t = *n;
    let mut k: uint32_t = 0 as uint32_t;
    while k < (*c).rep_n as uint32_t {
        let mut o: uint64_t = 0 as uint64_t;
        let mut pl: uint64_t = (*c).rep_pl[k as usize] as uint64_t;
        let mut i: uint64_t = 0 as uint64_t;
        while i < m {
            if *s.offset(i as isize) as ::core::ffi::c_int
                == (*c).rep_p[k as usize][0 as ::core::ffi::c_int as usize]
                    as ::core::ffi::c_int && m.wrapping_sub(i) >= pl
                && memcmp(
                    s.offset(i as isize) as *const ::core::ffi::c_void,
                    &raw const *(&raw const (*c).rep_p as *const [uint8_t; 4])
                        .offset(k as isize) as *const uint8_t
                        as *const ::core::ffi::c_void,
                    pl as size_t,
                ) == 0 as ::core::ffi::c_int
            {
                memcpy(
                    r.offset(o as isize) as *mut ::core::ffi::c_void,
                    &raw const *(&raw const (*c).rep_c as *const [uint8_t; 4])
                        .offset(k as isize) as *const uint8_t
                        as *const ::core::ffi::c_void,
                    (*c).rep_cl[k as usize] as size_t,
                );
                o = o.wrapping_add((*c).rep_cl[k as usize] as uint64_t);
                i = i.wrapping_add(pl);
            } else {
                let fresh0 = i;
                i = i.wrapping_add(1);
                let fresh1 = o;
                o = o.wrapping_add(1);
                *r.offset(fresh1 as isize) = *s.offset(fresh0 as isize);
            }
        }
        s = r;
        m = o;
        k = k.wrapping_add(1);
    }
    if (*c).form as ::core::ffi::c_uint != 0 as ::core::ffi::c_uint {
        m = toks_norm(
            (*c).form as uint32_t,
            s,
            m,
            p,
            ((if (*c).form as ::core::ffi::c_uint & TOKS_NS_COMPAT
                != 0 as ::core::ffi::c_uint
            {
                TOKS_NFKC_X
            } else {
                TOKS_NFC_X
            }) as uint64_t)
                .wrapping_mul(m),
        ) as uint64_t;
        s = p;
    }
    *n = m;
    return s;
}
#[no_mangle]
pub unsafe extern "C" fn toks_uni_load(
    mut c: *mut toks_ctx,
    mut cfg: *const toks_config,
    mut why: *mut *const ::core::ffi::c_char,
) -> int64_t {
    let mut u: *const toks_uni = ::core::ptr::null::<toks_uni>();
    let mut mem: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    let mut ml: uint64_t = 0 as uint64_t;
    *why = b"Unigram tables\0" as *const u8 as *const ::core::ffi::c_char;
    let mut r: int64_t = toks_uni_build(
        (*cfg).uni as *const toks_uni_src,
        &raw mut u,
        &raw mut mem,
        &raw mut ml,
        why,
    );
    if r != 0 as int64_t {
        return r;
    }
    (*c).uni = u as *const toks_uni;
    (*c).dc = toks_dchain {
        dec: &raw const (*u).dec as *const toks_spm_op,
        n_dec: (*u).n_dec,
        has_decoder: ((*u).cfg.dec as ::core::ffi::c_int
            != TOKS_UNI_DEC_NONE as ::core::ffi::c_int) as ::core::ffi::c_int
            as uint32_t,
        bf_first: ((*u).cfg.dec as ::core::ffi::c_int
            == TOKS_UNI_DEC_BFRF as ::core::ffi::c_int) as ::core::ffi::c_int
            as uint32_t,
        on: 1 as uint32_t,
        holes: ::core::ptr::null::<uint32_t>(),
    };
    (*c).mem_uni = mem;
    (*c).mem_uni_len = ml;
    let mut i: uint32_t = 0 as uint32_t;
    while i < (*cfg).n_added {
        let mut a: *const toks_cfg_added = (*cfg).added.offset(i as isize)
            as *mut toks_cfg_added;
        if !((*a).normalized == 0) {
            let mut sp: uint32_t = if (*u).cfg.has_charsmap as ::core::ffi::c_int != 0 {
                toks_pc_char(&raw const (*u).pc, 0x2581 as uint32_t)
            } else {
                0 as uint32_t
            };
            if (*u).cfg.rep_n as ::core::ffi::c_uint != 0 as ::core::ffi::c_uint
                || (*u).cfg.form as ::core::ffi::c_uint != 0 as ::core::ffi::c_uint
            {
                *why = b"normalized added tokens under Replace / NF* / StripAccents / Lowercase steps (Unigram)\0"
                    as *const u8 as *const ::core::ffi::c_char;
                return TOKS_E_UNSUPPORTED as int64_t;
            }
            if (*u).cfg.has_charsmap == 0
                || (*u).cfg.meta_prefix as ::core::ffi::c_int != 0
                || (*u).cfg.meta_replace as ::core::ffi::c_int != 0
                || sp == 0 as uint32_t || sp & 0xfff as uint32_t != 1 as uint32_t
                || *(*u)
                    .pc
                    .pool
                    .offset(
                        (sp >> 12 as ::core::ffi::c_int & 0x3ffff as uint32_t) as isize,
                    ) as ::core::ffi::c_uint != 0x20 as ::core::ffi::c_uint
            {
                *why = b"normalized added tokens with a chain that keeps U+2581 (Unigram)\0"
                    as *const u8 as *const ::core::ffi::c_char;
                return TOKS_E_UNSUPPORTED as int64_t;
            }
            let mut buf: [uint8_t; 4096] = [0; 4096];
            let mut m: int64_t = toks_uni_normalize(
                u,
                (*a).content,
                (*a).len as uint64_t,
                1 as ::core::ffi::c_int,
                &raw mut buf as *mut uint8_t,
                ::core::mem::size_of::<[uint8_t; 4096]>() as uint64_t,
            );
            if m != (*a).len as int64_t
                || memcmp(
                    &raw mut buf as *mut uint8_t as *const ::core::ffi::c_void,
                    (*a).content as *const ::core::ffi::c_void,
                    (*a).len as size_t,
                ) != 0 as ::core::ffi::c_int
            {
                *why = b"a normalized added token that the normalizer changes (Unigram)\0"
                    as *const u8 as *const ::core::ffi::c_char;
                return TOKS_E_UNSUPPORTED as int64_t;
            }
        }
        i = i.wrapping_add(1);
    }
    return 0 as int64_t;
}
#[no_mangle]
pub unsafe extern "C" fn toks_uni_run(
    mut ctx: *const toks_ctx,
    mut h: *const toks_scratch,
    mut text: *const uint8_t,
    mut len: uint64_t,
    mut flags: uint32_t,
    mut out: *mut uint32_t,
    mut cap: uint64_t,
    mut ids: ::core::ffi::c_int,
) -> int64_t {
    let mut u: *const toks_uni = (*ctx).uni as *const toks_uni;
    let mut work: *mut uint8_t = toks_scr_p(
        h,
        ((*h).base as uintptr_t as *mut uint8_t).offset((*h).off_work as isize),
    );
    let mut wbytes: uint64_t = (*h)
        .off_bounce
        .wrapping_sub((*h).off_work)
        .wrapping_add(
            toks_scr_bounce(
                toks_scr_tmax(
                    (*h).max_len,
                    (if (*ctx).nfc != 0 as uint32_t {
                        (if (*ctx).nfc & TOKS_NS_COMPAT as uint32_t != 0 as uint32_t {
                            TOKS_NFKC_X as uint32_t
                        } else {
                            TOKS_NFC_X as uint32_t
                        })
                    } else {
                        0 as uint32_t
                    }),
                ),
            ),
        );
    let mut one: uint64_t = toks_uni_area(u, len);
    if (3 as uint64_t).wrapping_mul(one) > wbytes {
        return TOKS_E_SCRATCH as int64_t;
    }
    let mut mode: uint32_t = flags & TOKS_ADDED_MASK as uint32_t;
    let mut pp: ::core::ffi::c_int = (ids != 0 as ::core::ffi::c_int
        && flags & TOKS_NO_POSTPROCESS as uint32_t == 0 as uint32_t)
        as ::core::ffi::c_int;
    let mut cont: ::core::ffi::c_int = (flags & TOKS_CONTINUATION as uint32_t
        != 0 as uint32_t) as ::core::ffi::c_int;
    let mut c: toks_uni_call = toks_uni_call {
        e: toks_emit {
            out: ::core::ptr::null_mut::<uint32_t>(),
            cap: 0,
            n: 0,
            lim: 0,
        },
        nbase: 0,
        pieces: 0,
        rsv: 0,
        work: ::core::ptr::null_mut::<uint8_t>(),
        work_bytes: 0,
        cache: ::core::ptr::null_mut::<uint8_t>(),
        cache_mask: 0,
        tw: 0,
    };
    memset(
        &raw mut c as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<toks_uni_call>() as size_t,
    );
    c.e = toks_emit {
        out: out,
        cap: cap,
        n: 0 as uint64_t,
        lim: UINT64_MAX as uint64_t,
    };
    c.pieces = (ids == 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
    c.work = work;
    c.work_bytes = (2 as uint64_t).wrapping_mul(one);
    if ids != 0 as ::core::ffi::c_int {
        c.cache = toks_scr_p(
            h,
            ((*h).base as uintptr_t as *mut uint8_t).offset((*h).off_cache as isize),
        );
        c.cache_mask = toks_scr_short((*h).cache_mib)
            .wrapping_div(64 as uint64_t)
            .wrapping_sub(1 as uint64_t);
        c.tw = toks_tag_word((*h).epoch);
    }
    if pp != 0 {
        let mut i: uint32_t = 0 as uint32_t;
        while i < (*ctx).n_pp_prefix {
            toks_put(&raw mut c.e, (*ctx).pp_ids[i as usize]);
            i = i.wrapping_add(1);
        }
    }
    if ids != 0 as ::core::ffi::c_int && cont == 0 && (*ctx).o.trunc_on != 0 as uint32_t
        && flags & TOKS_NO_TRUNCATE as uint32_t == 0 as uint32_t
    {
        c.e.lim = c
            .e
            .n
            .wrapping_add((*ctx).o.trunc_max as uint64_t)
            .wrapping_sub(
                (if pp != 0 {
                    (*ctx).n_pp_prefix.wrapping_add((*ctx).n_pp_suffix) as uint64_t
                } else {
                    0 as uint64_t
                }),
            );
    }
    let mut r: int64_t = 0 as int64_t;
    let mut pb: *mut uint8_t = work.offset((2 as uint64_t).wrapping_mul(one) as isize);
    let mut rb: *mut uint8_t = if (*u).cfg.form as ::core::ffi::c_uint
        != 0 as ::core::ffi::c_uint
    {
        work.offset(one as isize)
    } else {
        pb
    };
    if mode == TOKS_ADDED_NONE as uint32_t || (*ctx).t.add_n == 0 as uint64_t
        || mode == TOKS_ADDED_NONSPECIAL as uint32_t
            && (*ctx).n_nonspecial == 0 as uint32_t
    {
        let mut n: uint64_t = len;
        let mut x: *const uint8_t = uni_pre(
            &raw const (*u).cfg,
            text,
            &raw mut n,
            rb,
            pb,
        );
        r = toks_uni_encode_segment(
            u,
            x,
            n,
            0 as ::core::ffi::c_int,
            (cont == 0) as ::core::ffi::c_int,
            &raw mut c,
        );
    } else {
        let mut phase1: ::core::ffi::c_int = ((*ctx).t.add_phases & 2 as uint64_t
            != 0 as uint64_t) as ::core::ffi::c_int;
        let mut nb: uint64_t = 0 as uint64_t;
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
        let mut g: toks_seg_out = toks_seg_out {
            kind: 0,
            id: 0,
            start: 0,
            end: 0,
        };
        toks_seg_begin(
            &raw mut it,
            &raw const (*ctx).t,
            mode,
            0 as uint32_t,
            (*ctx).tier,
            text,
            len,
        );
        while r >= 0 as int64_t && c.e.n < c.e.lim
            && toks_seg_next(&raw mut it, &raw mut g) != 0
        {
            if g.kind == TOKS_SEG_TOKEN as uint32_t {
                nb = nb.wrapping_add(g.end.wrapping_sub(g.start));
                toks_put(&raw mut c.e, if ids != 0 { g.id } else { nb as uint32_t });
            } else {
                let mut gl: uint64_t = g.end.wrapping_sub(g.start);
                let mut gt: *const uint8_t = uni_pre(
                    &raw const (*u).cfg,
                    text.offset(g.start as isize),
                    &raw mut gl,
                    rb,
                    pb,
                );
                let mut gap_start: ::core::ffi::c_int = !(cont != 0
                    && g.start == 0 as uint64_t) as ::core::ffi::c_int;
                if phase1 == 0 {
                    c.nbase = nb;
                    c.work = work;
                    c.work_bytes = (2 as uint64_t).wrapping_mul(one);
                    r = toks_uni_encode_segment(
                        u,
                        gt,
                        gl,
                        0 as ::core::ffi::c_int,
                        gap_start,
                        &raw mut c,
                    );
                    if r >= 0 as int64_t {
                        nb = nb.wrapping_add(r as uint64_t);
                    }
                } else {
                    let mut m: int64_t = toks_uni_normalize(
                        u,
                        gt,
                        gl,
                        gap_start,
                        work,
                        one,
                    );
                    if m < 0 as int64_t {
                        r = TOKS_E_SCRATCH as int64_t;
                        break;
                    } else {
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
                            work,
                            m as uint64_t,
                        );
                        while r >= 0 as int64_t && c.e.n < c.e.lim
                            && toks_seg_next(&raw mut it1, &raw mut v) != 0
                        {
                            if v.kind == TOKS_SEG_TOKEN as uint32_t {
                                toks_put(
                                    &raw mut c.e,
                                    if ids != 0 {
                                        v.id
                                    } else {
                                        nb.wrapping_add(v.end) as uint32_t
                                    },
                                );
                            } else {
                                c.nbase = nb.wrapping_add(v.start);
                                c.work = work.offset(one as isize);
                                c.work_bytes = (2 as uint64_t).wrapping_mul(one);
                                r = toks_uni_encode_segment(
                                    u,
                                    work.offset(v.start as isize),
                                    v.end.wrapping_sub(v.start),
                                    1 as ::core::ffi::c_int,
                                    1 as ::core::ffi::c_int,
                                    &raw mut c,
                                );
                            }
                        }
                        nb = nb.wrapping_add(m as uint64_t);
                    }
                }
            }
        }
    }
    if r < 0 as int64_t {
        return r;
    }
    c.e.lim = UINT64_MAX as uint64_t;
    if pp != 0 {
        let mut i_0: uint32_t = 0 as uint32_t;
        while i_0 < (*ctx).n_pp_suffix {
            toks_put(
                &raw mut c.e,
                (*ctx).pp_ids[(*ctx).n_pp_prefix.wrapping_add(i_0) as usize],
            );
            i_0 = i_0.wrapping_add(1);
        }
    }
    if ids != 0 as ::core::ffi::c_int && cont == 0 && (*ctx).o.pad_on != 0
        && flags & TOKS_NO_PAD as uint32_t == 0 as uint32_t
    {
        toks_pad(&raw const (*ctx).o, &raw mut c.e);
    }
    return c.e.n as int64_t;
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
