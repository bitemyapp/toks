#[repr(C)]
pub struct toks_gen { _opaque: [u8; 0] }
#[repr(C)]
pub struct toks_uni { _opaque: [u8; 0] }
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
    fn toks_wp_scan_c(t: *const toks_wp_tables, a: *mut toks_wp_scan_args) -> uint64_t;
    fn toks_wp_encode_c(
        t: *const toks_wp_tables,
        a: *mut toks_wp_encode_args,
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
pub struct toks_dsink {
    pub out: *mut uint8_t,
    pub cap: uint64_t,
    pub n: uint64_t,
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
pub struct wpw {
    pub ctx: *const toks_ctx,
    pub t: *const toks_wp_tables,
    pub pieces: *mut toks_wp_piece,
    pub mat: *mut uint8_t,
    pub mat_cap: uint64_t,
    pub norm: *mut uint8_t,
    pub norm_cap: uint64_t,
    pub bounce: *mut uint32_t,
    pub bounce_n: uint64_t,
    pub cache: *mut uint8_t,
    pub cache_mask: uint64_t,
    pub tw: uint64_t,
    pub mode: uint32_t,
    pub ids: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct toks_wp_piece {
    pub off: uint32_t,
    pub len: uint32_t,
    pub end: uint32_t,
    pub flags: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct toks_wp_scan_args {
    pub text: *const uint8_t,
    pub len: uint64_t,
    pub pos: uint64_t,
    pub pieces: *mut toks_wp_piece,
    pub cap: uint64_t,
    pub n: uint64_t,
    pub mat: *mut uint8_t,
    pub mat_cap: uint64_t,
    pub mat_len: uint64_t,
    pub flags: uint64_t,
    pub rsv: [uint64_t; 6],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct toks_wp_encode_args {
    pub text: *const uint8_t,
    pub mat: *const uint8_t,
    pub pieces: *const toks_wp_piece,
    pub n: uint64_t,
    pub out: *mut uint32_t,
    pub room: uint64_t,
    pub n_out: uint64_t,
    pub hits: uint64_t,
    pub misses: uint64_t,
    pub probes: uint64_t,
    pub text_len: uint64_t,
    pub mat_len: uint64_t,
    pub cache: *mut uint8_t,
    pub cache_mask: uint64_t,
    pub tw: uint64_t,
    pub rsv: [uint64_t; 1],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct clst {
    pub b: [uint8_t; 8],
    pub n: uint32_t,
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
pub const UINT64_MAX: ::core::ffi::c_ulong = 18446744073709551615
    as ::core::ffi::c_ulong;
pub const TOKS_ADDED_NONSPECIAL: ::core::ffi::c_uint = 1 as ::core::ffi::c_uint;
pub const TOKS_ADDED_NONE: ::core::ffi::c_uint = 2 as ::core::ffi::c_uint;
pub const TOKS_ADDED_MASK: ::core::ffi::c_uint = 3 as ::core::ffi::c_uint;
pub const TOKS_NO_POSTPROCESS: ::core::ffi::c_uint = 4 as ::core::ffi::c_uint;
pub const TOKS_CONTINUATION: ::core::ffi::c_uint = 8 as ::core::ffi::c_uint;
pub const TOKS_NO_TRUNCATE: ::core::ffi::c_uint = 16 as ::core::ffi::c_uint;
pub const TOKS_NO_PAD: ::core::ffi::c_uint = 32 as ::core::ffi::c_uint;
pub const TOKS_SKIP_SPECIAL: ::core::ffi::c_uint = 1 as ::core::ffi::c_uint;
pub const TOKS_CHUNK_PIECES: ::core::ffi::c_int = 256 as ::core::ffi::c_int;
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
unsafe extern "C" fn toks_cpy(
    mut dst: *mut ::core::ffi::c_void,
    mut src: *const ::core::ffi::c_void,
    mut n: uint64_t,
) {
    memcpy(dst, src, n as size_t);
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
pub const TOKS_CACHE_BUCKETS: ::core::ffi::c_uint = 32768 as ::core::ffi::c_uint;
pub const TOKS_CACHE_BYTES: uint64_t = (TOKS_CACHE_BUCKETS as uint64_t)
    .wrapping_mul(TOKS_BUCKET as uint64_t);
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
unsafe extern "C" fn toks_scr_short(mut n: uint64_t) -> uint64_t {
    return if n == 0 as uint64_t {
        TOKS_CACHE_BYTES
    } else {
        n << 19 as ::core::ffi::c_int
    };
}
pub const TOKS_WP_WIN: ::core::ffi::c_uint = 8192 as ::core::ffi::c_uint;
pub const TOKS_SEG_TOKEN: ::core::ffi::c_uint = 1 as ::core::ffi::c_uint;
unsafe extern "C" fn scan(
    mut w: *const wpw,
    mut g: *const uint8_t,
    mut n: uint64_t,
    mut pos: uint64_t,
    mut fl: uint64_t,
    mut e: *const toks_emit,
    mut end: *mut uint64_t,
) -> uint64_t {
    let mut a: toks_wp_scan_args = toks_wp_scan_args {
        text: ::core::ptr::null::<uint8_t>(),
        len: 0,
        pos: 0,
        pieces: ::core::ptr::null_mut::<toks_wp_piece>(),
        cap: 0,
        n: 0,
        mat: ::core::ptr::null_mut::<uint8_t>(),
        mat_cap: 0,
        mat_len: 0,
        flags: 0,
        rsv: [0; 6],
    };
    memset(
        &raw mut a as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<toks_wp_scan_args>() as size_t,
    );
    let mut cap: uint64_t = TOKS_CHUNK_PIECES as uint64_t;
    if (*e).lim != UINT64_MAX as uint64_t
        && (*e).lim.wrapping_sub((*e).n).wrapping_add(2 as uint64_t) < cap
    {
        cap = (*e).lim.wrapping_sub((*e).n).wrapping_add(2 as uint64_t);
    }
    a.text = g;
    a.len = n;
    a.pos = pos;
    a.pieces = (*w).pieces;
    a.cap = cap;
    a.mat = (*w).mat;
    a.mat_cap = (*w).mat_cap;
    a.flags = fl;
    let mut np: uint64_t = toks_wp_scan_c((*w).t, &raw mut a);
    *end = a.pos;
    return np;
}
unsafe extern "C" fn model(
    mut w: *const wpw,
    mut g: *const uint8_t,
    mut gn: uint64_t,
    mut np: uint64_t,
    mut e: *mut toks_emit,
    mut bound: uint64_t,
) {
    let mut left: uint64_t = if (*e).n < (*e).cap {
        (*e).cap.wrapping_sub((*e).n)
    } else {
        0 as uint64_t
    };
    let mut direct: ::core::ffi::c_int = (left >= bound) as ::core::ffi::c_int;
    let mut k: toks_wp_encode_args = toks_wp_encode_args {
        text: ::core::ptr::null::<uint8_t>(),
        mat: ::core::ptr::null::<uint8_t>(),
        pieces: ::core::ptr::null::<toks_wp_piece>(),
        n: 0,
        out: ::core::ptr::null_mut::<uint32_t>(),
        room: 0,
        n_out: 0,
        hits: 0,
        misses: 0,
        probes: 0,
        text_len: 0,
        mat_len: 0,
        cache: ::core::ptr::null_mut::<uint8_t>(),
        cache_mask: 0,
        tw: 0,
        rsv: [0; 1],
    };
    memset(
        &raw mut k as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<toks_wp_encode_args>() as size_t,
    );
    k.text = g;
    k.mat = (*w).mat;
    k.pieces = (*w).pieces;
    k.n = np;
    k.out = if direct != 0 { (*e).out.offset((*e).n as isize) } else { (*w).bounce };
    k.room = if direct != 0 { left } else { (*w).bounce_n };
    k.text_len = gn;
    k.mat_len = (*w).mat_cap;
    k.cache = (*w).cache;
    k.cache_mask = (*w).cache_mask;
    k.tw = (*w).tw;
    let mut m: uint64_t = toks_wp_encode_c((*w).t, &raw mut k);
    if direct == 0 && left != 0 as uint64_t {
        toks_cpy(
            (*e).out.offset((*e).n as isize) as *mut ::core::ffi::c_void,
            (*w).bounce as *const ::core::ffi::c_void,
            (if left < m { left } else { m }).wrapping_mul(4 as uint64_t),
        );
    }
    (*e).n = (*e).n.wrapping_add(m);
    if (*e).n > (*e).lim {
        (*e).n = (*e).lim;
    }
}
unsafe extern "C" fn text_inplace(
    mut w: *const wpw,
    mut g: *const uint8_t,
    mut n: uint64_t,
    mut e: *mut toks_emit,
) {
    let mut pos: uint64_t = 0 as uint64_t;
    while pos < n && (*e).n < (*e).lim {
        let mut end: uint64_t = 0;
        let mut np: uint64_t = scan(
            w,
            g,
            n,
            pos,
            !(0 as ::core::ffi::c_ulonglong) as uint64_t,
            e,
            &raw mut end,
        );
        if np == 0 as uint64_t && end <= pos {
            return;
        }
        if np != 0 as uint64_t {
            model(w, g, n, np, e, end.wrapping_sub(pos));
        }
        pos = end;
    }
}
unsafe extern "C" fn text_norm(
    mut w: *const wpw,
    mut g: *const uint8_t,
    mut n: uint64_t,
    mut e: *mut toks_emit,
    mut nb: uint64_t,
) {
    let mut pos: uint64_t = 0 as uint64_t;
    while pos < n && (*e).n < (*e).lim {
        let mut end: uint64_t = 0;
        let mut np: uint64_t = scan(w, g, n, pos, 0 as uint64_t, e, &raw mut end);
        if np == 0 as uint64_t && end <= pos {
            return;
        }
        if (*w).ids == 0 as ::core::ffi::c_int {
            let mut k: uint64_t = 0 as uint64_t;
            while k < np {
                toks_put(
                    e,
                    nb
                        .wrapping_add((*(*w).pieces.offset(k as isize)).off as uint64_t)
                        .wrapping_add((*(*w).pieces.offset(k as isize)).len as uint64_t)
                        as uint32_t,
                );
                k = k.wrapping_add(1);
            }
        } else if np != 0 as uint64_t {
            model(w, g, n, np, e, end.wrapping_sub(pos));
        }
        pos = end;
    }
}
unsafe extern "C" fn gap_norm(
    mut w: *const wpw,
    mut g: *const uint8_t,
    mut n: uint64_t,
    mut e: *mut toks_emit,
    mut match_0: ::core::ffi::c_int,
    mut nbase: *mut uint64_t,
) {
    let mut r: int64_t = toks_norm((*(*w).t).flags, g, n, (*w).norm, (*w).norm_cap);
    let mut nl: uint64_t = r as uint64_t;
    if r < 0 as int64_t {
        return;
    }
    let mut ctx: *const toks_ctx = (*w).ctx;
    if match_0 == 0 || (*ctx).t.add_phases & 2 as uint64_t == 0 as uint64_t {
        text_norm(w, (*w).norm, nl, e, *nbase);
        *nbase = (*nbase).wrapping_add(nl);
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
        (*w).mode,
        1 as uint32_t,
        (*ctx).tier,
        (*w).norm,
        nl,
    );
    while (*e).n < (*e).lim && toks_seg_next(&raw mut it, &raw mut v) != 0 {
        if v.kind == TOKS_SEG_TOKEN as uint32_t {
            toks_put(
                e,
                if (*w).ids != 0 {
                    v.id
                } else {
                    (*nbase).wrapping_add(v.end) as uint32_t
                },
            );
        } else {
            text_norm(
                w,
                (*w).norm.offset(v.start as isize),
                v.end.wrapping_sub(v.start),
                e,
                (*nbase).wrapping_add(v.start),
            );
        }
    }
    *nbase = (*nbase).wrapping_add(nl);
}
unsafe extern "C" fn gap(
    mut w: *const wpw,
    mut g: *const uint8_t,
    mut n: uint64_t,
    mut e: *mut toks_emit,
    mut norm: ::core::ffi::c_int,
    mut match_0: ::core::ffi::c_int,
    mut nbase: *mut uint64_t,
) {
    if norm == 0 {
        text_inplace(w, g, n, e);
        return;
    }
    if (*w).ids == 0 as ::core::ffi::c_int || (*(*w).ctx).o.wp_win == 0 as uint32_t {
        gap_norm(w, g, n, e, match_0, nbase);
        return;
    }
    let mut r: uint64_t = 0 as uint64_t;
    while r < n && (*e).n < (*e).lim {
        let mut t: uint64_t = n;
        if n.wrapping_sub(r) > TOKS_WP_WIN as uint64_t {
            t = r.wrapping_add(TOKS_WP_WIN as uint64_t);
            while t < n
                && *g.offset(t as isize) as ::core::ffi::c_uint
                    != 0x20 as ::core::ffi::c_uint
                && *g.offset(t as isize) as ::core::ffi::c_uint
                    != 0x9 as ::core::ffi::c_uint
                && *g.offset(t as isize) as ::core::ffi::c_uint
                    != 0xa as ::core::ffi::c_uint
                && *g.offset(t as isize) as ::core::ffi::c_uint
                    != 0xd as ::core::ffi::c_uint
            {
                t = t.wrapping_add(1);
            }
        }
        gap_norm(w, g.offset(r as isize), t.wrapping_sub(r), e, match_0, nbase);
        r = t;
    }
}
#[no_mangle]
pub unsafe extern "C" fn toks_wp_run(
    mut ctx: *const toks_ctx,
    mut h: *mut toks_scratch,
    mut text: *const uint8_t,
    mut len: uint64_t,
    mut flags: uint32_t,
    mut out: *mut uint32_t,
    mut cap: uint64_t,
    mut ids: ::core::ffi::c_int,
) -> int64_t {
    let mut s: *mut uint8_t = (*h).base as uintptr_t as *mut uint8_t;
    let mut wk: *mut uint8_t = toks_scr_p(h, s.offset((*h).off_work as isize));
    let mut ml: uint64_t = (*h).max_len;
    let mut w: wpw = wpw {
        ctx: ::core::ptr::null::<toks_ctx>(),
        t: ::core::ptr::null::<toks_wp_tables>(),
        pieces: ::core::ptr::null_mut::<toks_wp_piece>(),
        mat: ::core::ptr::null_mut::<uint8_t>(),
        mat_cap: 0,
        norm: ::core::ptr::null_mut::<uint8_t>(),
        norm_cap: 0,
        bounce: ::core::ptr::null_mut::<uint32_t>(),
        bounce_n: 0,
        cache: ::core::ptr::null_mut::<uint8_t>(),
        cache_mask: 0,
        tw: 0,
        mode: 0,
        ids: 0,
    };
    w.ctx = ctx;
    w.t = (*ctx).wp as *const toks_wp_tables;
    w.pieces = wk as *mut ::core::ffi::c_void as *mut toks_wp_piece;
    w.mat = wk
        .offset((16 as uint64_t).wrapping_mul(TOKS_CHUNK_PIECES as uint64_t) as isize);
    w.mat_cap = (*ctx).wp_mat_cap;
    w.norm = w.mat.offset(w.mat_cap as isize);
    w.norm_cap = toks_align64(
        (3 as uint64_t).wrapping_mul(ml).wrapping_add(64 as uint64_t),
    );
    w.bounce = toks_scr_p(h, s.offset((*h).off_bounce as isize))
        as *mut ::core::ffi::c_void as *mut uint32_t;
    w.bounce_n = ml.wrapping_add(4 as uint64_t);
    w.cache = if ids != 0 as ::core::ffi::c_int {
        toks_scr_p(h, s.offset((*h).off_cache as isize))
    } else {
        ::core::ptr::null_mut::<uint8_t>()
    };
    w.cache_mask = toks_scr_short((*h).cache_mib)
        .wrapping_div(64 as uint64_t)
        .wrapping_sub(1 as uint64_t);
    w.tw = toks_tag_word((*h).epoch);
    w.mode = flags & TOKS_ADDED_MASK as uint32_t;
    w.ids = ids;
    let mut pp: ::core::ffi::c_int = (ids != 0 as ::core::ffi::c_int
        && flags & TOKS_NO_POSTPROCESS as uint32_t == 0 as uint32_t)
        as ::core::ffi::c_int;
    let mut doc: ::core::ffi::c_int = (ids != 0 as ::core::ffi::c_int
        && flags & TOKS_CONTINUATION as uint32_t == 0 as uint32_t) as ::core::ffi::c_int;
    let mut e: toks_emit = toks_emit {
        out: out,
        cap: cap,
        n: 0 as uint64_t,
        lim: UINT64_MAX as uint64_t,
    };
    if pp != 0 {
        let mut i: uint32_t = 0 as uint32_t;
        while i < (*ctx).n_pp_prefix {
            toks_put(&raw mut e, (*ctx).pp_ids[i as usize]);
            i = i.wrapping_add(1);
        }
    }
    if doc != 0 && (*ctx).o.trunc_on != 0
        && flags & TOKS_NO_TRUNCATE as uint32_t == 0 as uint32_t
    {
        let mut n_added: uint64_t = if pp != 0 {
            ((*ctx).n_pp_prefix as uint64_t).wrapping_add((*ctx).n_pp_suffix as uint64_t)
        } else {
            0 as uint64_t
        };
        e.lim = e.n.wrapping_add(((*ctx).o.trunc_max as uint64_t).wrapping_sub(n_added));
    }
    let mut mode: uint32_t = w.mode;
    let mut match_0: ::core::ffi::c_int = !(mode == TOKS_ADDED_NONE as uint32_t
        || (*ctx).t.add_n == 0 as uint64_t
        || mode == TOKS_ADDED_NONSPECIAL as uint32_t
            && (*ctx).n_nonspecial == 0 as uint32_t) as ::core::ffi::c_int;
    let mut norm: ::core::ffi::c_int = (ids == 0 as ::core::ffi::c_int
        || match_0 != 0 && (*ctx).t.add_phases & 2 as uint64_t != 0 as uint64_t)
        as ::core::ffi::c_int;
    let mut nbase: uint64_t = 0 as uint64_t;
    if match_0 == 0 {
        gap(
            &raw mut w,
            text,
            len,
            &raw mut e,
            norm,
            0 as ::core::ffi::c_int,
            &raw mut nbase,
        );
    } else {
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
            mode,
            0 as uint32_t,
            (*ctx).tier,
            text,
            len,
        );
        while e.n < e.lim && toks_seg_next(&raw mut it, &raw mut u) != 0 {
            if u.kind == TOKS_SEG_TOKEN as uint32_t {
                nbase = nbase.wrapping_add(u.end.wrapping_sub(u.start));
                toks_put(&raw mut e, if ids != 0 { u.id } else { nbase as uint32_t });
            } else {
                gap(
                    &raw mut w,
                    text.offset(u.start as isize),
                    u.end.wrapping_sub(u.start),
                    &raw mut e,
                    norm,
                    1 as ::core::ffi::c_int,
                    &raw mut nbase,
                );
            }
        }
    }
    e.lim = UINT64_MAX as uint64_t;
    if pp != 0 {
        let mut i_0: uint32_t = 0 as uint32_t;
        while i_0 < (*ctx).n_pp_suffix {
            toks_put(
                &raw mut e,
                (*ctx).pp_ids[(*ctx).n_pp_prefix.wrapping_add(i_0) as usize],
            );
            i_0 = i_0.wrapping_add(1);
        }
    }
    if doc != 0 && (*ctx).o.pad_on != 0
        && flags & TOKS_NO_PAD as uint32_t == 0 as uint32_t
    {
        toks_pad(&raw const (*ctx).o, &raw mut e);
    }
    return e.n as int64_t;
}
pub const CL_N: ::core::ffi::c_uint = 11 as ::core::ffi::c_uint;
static mut CL_PAT: [[uint8_t; 8]; 11] = unsafe {
    [
        ::core::mem::transmute::<[u8; 8], [uint8_t; 8]>(*b" .\0\0\0\0\0\0"),
        ::core::mem::transmute::<[u8; 8], [uint8_t; 8]>(*b" ?\0\0\0\0\0\0"),
        ::core::mem::transmute::<[u8; 8], [uint8_t; 8]>(*b" !\0\0\0\0\0\0"),
        ::core::mem::transmute::<[u8; 8], [uint8_t; 8]>(*b" ,\0\0\0\0\0\0"),
        ::core::mem::transmute::<[u8; 8], [uint8_t; 8]>(*b" ' \0\0\0\0\0"),
        ::core::mem::transmute::<[u8; 8], [uint8_t; 8]>(*b" n't\0\0\0\0"),
        ::core::mem::transmute::<[u8; 8], [uint8_t; 8]>(*b" 'm\0\0\0\0\0"),
        ::core::mem::transmute::<[u8; 8], [uint8_t; 8]>(*b" do not\0"),
        ::core::mem::transmute::<[u8; 8], [uint8_t; 8]>(*b" 's\0\0\0\0\0"),
        ::core::mem::transmute::<[u8; 8], [uint8_t; 8]>(*b" 've\0\0\0\0"),
        ::core::mem::transmute::<[u8; 8], [uint8_t; 8]>(*b" 're\0\0\0\0"),
    ]
};
static mut CL_PL: [uint8_t; 11] = [
    2 as ::core::ffi::c_int as uint8_t,
    2 as ::core::ffi::c_int as uint8_t,
    2 as ::core::ffi::c_int as uint8_t,
    2 as ::core::ffi::c_int as uint8_t,
    3 as ::core::ffi::c_int as uint8_t,
    4 as ::core::ffi::c_int as uint8_t,
    3 as ::core::ffi::c_int as uint8_t,
    7 as ::core::ffi::c_int as uint8_t,
    3 as ::core::ffi::c_int as uint8_t,
    4 as ::core::ffi::c_int as uint8_t,
    4 as ::core::ffi::c_int as uint8_t,
];
static mut CL_REP: [[uint8_t; 8]; 11] = unsafe {
    [
        ::core::mem::transmute::<[u8; 8], [uint8_t; 8]>(*b".\0\0\0\0\0\0\0"),
        ::core::mem::transmute::<[u8; 8], [uint8_t; 8]>(*b"?\0\0\0\0\0\0\0"),
        ::core::mem::transmute::<[u8; 8], [uint8_t; 8]>(*b"!\0\0\0\0\0\0\0"),
        ::core::mem::transmute::<[u8; 8], [uint8_t; 8]>(*b",\0\0\0\0\0\0\0"),
        ::core::mem::transmute::<[u8; 8], [uint8_t; 8]>(*b"'\0\0\0\0\0\0\0"),
        ::core::mem::transmute::<[u8; 8], [uint8_t; 8]>(*b"n't\0\0\0\0\0"),
        ::core::mem::transmute::<[u8; 8], [uint8_t; 8]>(*b"'m\0\0\0\0\0\0"),
        ::core::mem::transmute::<[u8; 8], [uint8_t; 8]>(*b" don't\0\0"),
        ::core::mem::transmute::<[u8; 8], [uint8_t; 8]>(*b"'s\0\0\0\0\0\0"),
        ::core::mem::transmute::<[u8; 8], [uint8_t; 8]>(*b"'ve\0\0\0\0\0"),
        ::core::mem::transmute::<[u8; 8], [uint8_t; 8]>(*b"'re\0\0\0\0\0"),
    ]
};
static mut CL_RL: [uint8_t; 11] = [
    1 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
    3 as ::core::ffi::c_int as uint8_t,
    2 as ::core::ffi::c_int as uint8_t,
    6 as ::core::ffi::c_int as uint8_t,
    2 as ::core::ffi::c_int as uint8_t,
    3 as ::core::ffi::c_int as uint8_t,
    3 as ::core::ffi::c_int as uint8_t,
];
unsafe extern "C" fn cl_push(
    mut s: *mut clst,
    mut q: uint32_t,
    mut c: uint8_t,
    mut o: *mut uint8_t,
    mut no: *mut uint32_t,
) {
    let fresh0 = (*s).n;
    (*s).n = (*s).n.wrapping_add(1);
    (*s).b[fresh0 as usize] = c;
    let mut it: uint32_t = 0 as uint32_t;
    while it < 9 as uint32_t && (*s).n != 0 as uint32_t {
        if (*s).n == CL_PL[q as usize] as uint32_t
            && memcmp(
                &raw mut (*s).b as *mut uint8_t as *const ::core::ffi::c_void,
                &raw const *(&raw const CL_PAT as *const [uint8_t; 8]).offset(q as isize)
                    as *const uint8_t as *const ::core::ffi::c_void,
                (*s).n as size_t,
            ) == 0 as ::core::ffi::c_int
        {
            memcpy(
                o.offset(*no as isize) as *mut ::core::ffi::c_void,
                &raw const *(&raw const CL_REP as *const [uint8_t; 8]).offset(q as isize)
                    as *const uint8_t as *const ::core::ffi::c_void,
                CL_RL[q as usize] as size_t,
            );
            *no = (*no).wrapping_add(CL_RL[q as usize] as uint32_t);
            (*s).n = 0 as uint32_t;
            return;
        }
        if memcmp(
            &raw mut (*s).b as *mut uint8_t as *const ::core::ffi::c_void,
            &raw const *(&raw const CL_PAT as *const [uint8_t; 8]).offset(q as isize)
                as *const uint8_t as *const ::core::ffi::c_void,
            (*s).n as size_t,
        ) == 0 as ::core::ffi::c_int
        {
            return;
        }
        let fresh1 = *no;
        *no = (*no).wrapping_add(1);
        *o.offset(fresh1 as isize) = (*s).b[0 as ::core::ffi::c_int as usize];
        (*s).n = (*s).n.wrapping_sub(1);
        let mut k: uint32_t = 0 as uint32_t;
        while k < (*s).n {
            (*s).b[k as usize] = (*s).b[k.wrapping_add(1 as uint32_t) as usize];
            k = k.wrapping_add(1);
        }
        it = it.wrapping_add(1);
    }
}
unsafe extern "C" fn cl_feed(
    mut st: *mut clst,
    mut q0: uint32_t,
    mut src: *const uint8_t,
    mut n: uint32_t,
    mut d: *mut toks_dsink,
) {
    let mut a: [uint8_t; 160] = [0; 160];
    let mut b: [uint8_t; 160] = [0; 160];
    let mut na: uint32_t = n;
    memcpy(
        &raw mut a as *mut uint8_t as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        n as size_t,
    );
    let mut q: uint32_t = q0;
    while q < CL_N as uint32_t {
        let mut nb: uint32_t = 0 as uint32_t;
        let mut i: uint32_t = 0 as uint32_t;
        while i < na {
            cl_push(
                st.offset(q as isize) as *mut clst,
                q,
                a[i as usize],
                &raw mut b as *mut uint8_t,
                &raw mut nb,
            );
            i = i.wrapping_add(1);
        }
        memcpy(
            &raw mut a as *mut uint8_t as *mut ::core::ffi::c_void,
            &raw mut b as *mut uint8_t as *const ::core::ffi::c_void,
            nb as size_t,
        );
        na = nb;
        q = q.wrapping_add(1);
    }
    toks_dput(
        d,
        &raw mut a as *mut uint8_t as *const ::core::ffi::c_void,
        na as uint64_t,
    );
}
unsafe extern "C" fn cleanup(
    mut d: *mut toks_dsink,
    mut sp: ::core::ffi::c_int,
    mut p: *const uint8_t,
    mut l: uint64_t,
) {
    let mut spaces: uint64_t = 0 as uint64_t;
    let mut i: uint64_t = 0 as uint64_t;
    while i < l {
        spaces = spaces
            .wrapping_add(
                (*p.offset(i as isize) as ::core::ffi::c_int == ' ' as i32)
                    as ::core::ffi::c_int as uint64_t,
            );
        i = i.wrapping_add(1);
    }
    if spaces == 0 as uint64_t {
        if sp == 0 {
            toks_dput(d, p as *const ::core::ffi::c_void, l);
            return;
        }
        let mut q: uint32_t = 0 as uint32_t;
        while q < CL_N as uint32_t {
            let mut k: uint32_t = (CL_PL[q as usize] as uint32_t)
                .wrapping_sub(1 as uint32_t);
            if q == 4 as uint32_t || q == 7 as uint32_t || l < k as uint64_t
                || memcmp(
                    p as *const ::core::ffi::c_void,
                    (&raw const *(&raw const CL_PAT as *const [uint8_t; 8])
                        .offset(q as isize) as *const uint8_t)
                        .offset(1 as ::core::ffi::c_int as isize)
                        as *const ::core::ffi::c_void,
                    k as size_t,
                ) != 0 as ::core::ffi::c_int
            {
                q = q.wrapping_add(1);
            } else {
                toks_dput(
                    d,
                    &raw const *(&raw const CL_REP as *const [uint8_t; 8])
                        .offset(q as isize) as *const uint8_t
                        as *const ::core::ffi::c_void,
                    CL_RL[q as usize] as uint64_t,
                );
                toks_dput(
                    d,
                    p.offset(k as isize) as *const ::core::ffi::c_void,
                    l.wrapping_sub(k as uint64_t),
                );
                return;
            }
        }
        toks_dput(
            d,
            b" \0" as *const u8 as *const ::core::ffi::c_char as *const uint8_t
                as *const ::core::ffi::c_void,
            1 as uint64_t,
        );
        toks_dput(d, p as *const ::core::ffi::c_void, l);
        return;
    }
    let mut st: [clst; 11] = [clst { b: [0; 8], n: 0 }; 11];
    memset(
        &raw mut st as *mut clst as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<[clst; 11]>() as size_t,
    );
    if sp != 0 {
        cl_feed(
            &raw mut st as *mut clst,
            0 as uint32_t,
            b" \0" as *const u8 as *const ::core::ffi::c_char as *const uint8_t,
            1 as uint32_t,
            d,
        );
    }
    let mut i_0: uint64_t = 0 as uint64_t;
    while i_0 < l {
        cl_feed(
            &raw mut st as *mut clst,
            0 as uint32_t,
            p.offset(i_0 as isize),
            1 as uint32_t,
            d,
        );
        i_0 = i_0.wrapping_add(1);
    }
    let mut q_0: uint32_t = 0 as uint32_t;
    while q_0 < CL_N as uint32_t {
        let mut tmp: [uint8_t; 8] = [0; 8];
        let mut m: uint32_t = st[q_0 as usize].n;
        memcpy(
            &raw mut tmp as *mut uint8_t as *mut ::core::ffi::c_void,
            &raw mut (*(&raw mut st as *mut clst).offset(q_0 as isize)).b as *mut uint8_t
                as *const ::core::ffi::c_void,
            m as size_t,
        );
        st[q_0 as usize].n = 0 as uint32_t;
        if q_0.wrapping_add(1 as uint32_t) < CL_N as uint32_t {
            cl_feed(
                &raw mut st as *mut clst,
                q_0.wrapping_add(1 as uint32_t),
                &raw mut tmp as *mut uint8_t,
                m,
                d,
            );
        } else {
            toks_dput(
                d,
                &raw mut tmp as *mut uint8_t as *const ::core::ffi::c_void,
                m as uint64_t,
            );
        }
        q_0 = q_0.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn toks_wp_decode(
    mut ctx: *const toks_ctx,
    mut ids: *const uint32_t,
    mut n: uint64_t,
    mut flags: uint32_t,
    mut out: *mut uint8_t,
    mut cap: uint64_t,
) -> int64_t {
    let mut k: uint64_t = 0 as uint64_t;
    return toks_wp_decode_k(ctx, ids, n, flags, out, cap, &raw mut k);
}
#[no_mangle]
pub unsafe extern "C" fn toks_wp_decode_k(
    mut ctx: *const toks_ctx,
    mut ids: *const uint32_t,
    mut n: uint64_t,
    mut flags: uint32_t,
    mut out: *mut uint8_t,
    mut cap: uint64_t,
    mut kept: *mut uint64_t,
) -> int64_t {
    let mut d: toks_dsink = toks_dsink {
        out: out,
        cap: cap,
        n: 0 as uint64_t,
    };
    let mut off: *const uint32_t = (*ctx).t.tok_off;
    let mut k: uint64_t = *kept;
    let mut i: uint64_t = 0 as uint64_t;
    while i < n {
        let mut id: uint32_t = toks_ld32(
            ids.offset(i as isize) as *const ::core::ffi::c_void,
        );
        if !(flags & TOKS_SKIP_SPECIAL as uint32_t != 0 as uint32_t
            && !(*ctx).special_ids.is_null()
            && toks_bit((*ctx).special_ids, id) != 0 as uint32_t)
        {
            let mut p: *const uint8_t = (*ctx)
                .t
                .tok_bytes
                .offset(*off.offset(id as isize) as isize);
            let mut l: uint64_t = (*off.offset(id.wrapping_add(1 as uint32_t) as isize))
                .wrapping_sub(*off.offset(id as isize)) as uint64_t;
            if (*ctx).o.dec_wordpiece == 0 as uint32_t {
                if k != 0 as uint64_t {
                    toks_dput(
                        &raw mut d,
                        b" \0" as *const u8 as *const ::core::ffi::c_char
                            as *const uint8_t as *const ::core::ffi::c_void,
                        1 as uint64_t,
                    );
                }
                toks_dput(&raw mut d, p as *const ::core::ffi::c_void, l);
                k = k.wrapping_add(1);
            } else {
                let mut sp: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                if k != 0 as uint64_t {
                    let mut pl: uint64_t = (*ctx).o.dec_prefix_len as uint64_t;
                    if l >= pl
                        && memcmp(
                            p as *const ::core::ffi::c_void,
                            &raw const (*ctx).o.dec_prefix as *const uint8_t
                                as *const ::core::ffi::c_void,
                            pl as size_t,
                        ) == 0 as ::core::ffi::c_int
                    {
                        p = p.offset(pl as isize);
                        l = l.wrapping_sub(pl);
                    } else {
                        sp = 1 as ::core::ffi::c_int;
                    }
                }
                k = k.wrapping_add(1);
                if (*ctx).o.dec_cleanup == 0 as uint32_t {
                    if sp != 0 {
                        toks_dput(
                            &raw mut d,
                            b" \0" as *const u8 as *const ::core::ffi::c_char
                                as *const uint8_t as *const ::core::ffi::c_void,
                            1 as uint64_t,
                        );
                    }
                    toks_dput(&raw mut d, p as *const ::core::ffi::c_void, l);
                } else {
                    cleanup(&raw mut d, sp, p, l);
                }
            }
        }
        i = i.wrapping_add(1);
    }
    *kept = k;
    return d.n as int64_t;
}
