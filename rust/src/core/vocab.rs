#[repr(C)]
pub struct toks_gen { _opaque: [u8; 0] }
#[repr(C)]
pub struct toks_uni { _opaque: [u8; 0] }
#[repr(C)]
pub struct toks_wp_tables { _opaque: [u8; 0] }
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
    fn memcmp(
        a: *const ::core::ffi::c_void,
        b: *const ::core::ffi::c_void,
        n: size_t,
    ) -> ::core::ffi::c_int;
    fn toks_plat_arena(n: uint64_t) -> *mut uint8_t;
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
pub struct toks_ext {
    pub pad: uint32_t,
    pub align: uint32_t,
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
pub const TOKS_SPM_D_BYTE_FALLBACK: C2RustUnnamed = 2;
pub type C2RustUnnamed = ::core::ffi::c_uint;
pub const TOKS_SPM_D_METASPACE: C2RustUnnamed = 5;
pub const TOKS_SPM_D_STRIP: C2RustUnnamed = 4;
pub const TOKS_SPM_D_FUSE: C2RustUnnamed = 3;
pub const TOKS_SPM_D_REPLACE: C2RustUnnamed = 1;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
pub const TOKS_E_ID: ::core::ffi::c_int = -(7 as ::core::ffi::c_int);
pub const TOKS_E_ARG: ::core::ffi::c_int = -(10 as ::core::ffi::c_int);
pub const TOKS_E_NOMEM: ::core::ffi::c_int = -(11 as ::core::ffi::c_int);
pub const TOKS_MAX_TOKEN_BYTES: ::core::ffi::c_uint = 65535 as ::core::ffi::c_uint;
pub const TOKS_ID_ADDED: ::core::ffi::c_uint = 1 as ::core::ffi::c_uint;
pub const TOKS_ID_SPECIAL: ::core::ffi::c_uint = 2 as ::core::ffi::c_uint;
pub const TOKS_ID_BYTE: ::core::ffi::c_uint = 4 as ::core::ffi::c_uint;
pub const TOKS_X_VOC_SLOTS: toks_ext = toks_ext {
    pad: 0 as uint32_t,
    align: 4 as uint32_t,
};
pub const TOKS_X_VOC_ADD: toks_ext = toks_ext {
    pad: 0 as uint32_t,
    align: 4 as uint32_t,
};
pub const TOKS_X_VOC_DEC: toks_ext = toks_ext {
    pad: 0 as uint32_t,
    align: 4 as uint32_t,
};
pub const TOKS_X_VOC_BITS: toks_ext = toks_ext {
    pad: 0 as uint32_t,
    align: 4 as uint32_t,
};
pub const TOKS_X_VOC_POOL: toks_ext = toks_ext {
    pad: 0 as uint32_t,
    align: 1 as uint32_t,
};
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
unsafe extern "C" fn toks_mix64(mut x: uint64_t, mut w: uint64_t) -> uint64_t {
    x = ((x ^ w) as ::core::ffi::c_ulonglong)
        .wrapping_mul(0xff51afd7ed558ccd as ::core::ffi::c_ulonglong) as uint64_t;
    return x ^ x >> 32 as ::core::ffi::c_int;
}
pub const TOKS_SPM_MAX_OPS: ::core::ffi::c_uint = 8 as ::core::ffi::c_uint;
unsafe extern "C" fn key_bytes(
    mut c: *const toks_ctx,
    mut key: uint32_t,
    mut n: *mut uint64_t,
) -> *const uint8_t {
    if key < (*c).voc_n_add {
        let mut r: *const uint32_t = (*c)
            .voc_add
            .offset((4 as uint64_t).wrapping_mul(key as uint64_t) as isize);
        *n = *r.offset(1 as ::core::ffi::c_int as isize) as uint64_t;
        return (*c)
            .voc_pool
            .offset(*r.offset(0 as ::core::ffi::c_int as isize) as isize);
    }
    let mut id: uint32_t = key.wrapping_sub((*c).voc_n_add);
    *n = (*(*c).t.tok_off.offset(id.wrapping_add(1 as uint32_t) as isize) as uint64_t)
        .wrapping_sub(*(*c).t.tok_off.offset(id as isize) as uint64_t);
    return (*c).t.tok_bytes.offset(*(*c).t.tok_off.offset(id as isize) as isize);
}
unsafe extern "C" fn key_id(mut c: *const toks_ctx, mut key: uint32_t) -> uint32_t {
    return if key < (*c).voc_n_add {
        *(*c)
            .voc_add
            .offset(
                (4 as uint64_t).wrapping_mul(key as uint64_t).wrapping_add(2 as uint64_t)
                    as isize,
            )
    } else {
        key.wrapping_sub((*c).voc_n_add)
    };
}
unsafe extern "C" fn vhash(mut s: *const uint8_t, mut n: uint64_t) -> uint64_t {
    let mut h: uint64_t = (0x9e3779b97f4a7c15 as ::core::ffi::c_ulonglong
        ^ n as ::core::ffi::c_ulonglong) as uint64_t;
    let mut w: uint64_t = 0;
    let mut i: uint64_t = 0 as uint64_t;
    while i.wrapping_add(8 as uint64_t) <= n {
        memcpy(
            &raw mut w as *mut ::core::ffi::c_void,
            s.offset(i as isize) as *const ::core::ffi::c_void,
            8 as size_t,
        );
        h = toks_mix64(h, w);
        i = i.wrapping_add(8 as uint64_t);
    }
    if i < n {
        w = 0 as uint64_t;
        memcpy(
            &raw mut w as *mut ::core::ffi::c_void,
            s.offset(i as isize) as *const ::core::ffi::c_void,
            n.wrapping_sub(i) as size_t,
        );
        h = toks_mix64(h, w);
    }
    return toks_mix64(h, 0xd6e8feb86659fd93 as uint64_t);
}
unsafe extern "C" fn voc_find(
    mut c: *const toks_ctx,
    mut slots: *mut uint32_t,
    mut s: *const uint8_t,
    mut n: uint64_t,
    mut h: uint64_t,
) -> *mut uint32_t {
    let mut i: uint64_t = h & (*c).voc_mask;
    let mut tag: uint32_t = (h >> 55 as ::core::ffi::c_int) as uint32_t;
    let mut k: uint64_t = 0 as uint64_t;
    while k <= (*c).voc_mask {
        let mut v: uint32_t = *slots.offset(i as isize);
        if v == 0 as uint32_t {
            return slots.offset(i as isize) as *mut uint32_t;
        }
        if v >> 23 as ::core::ffi::c_int == tag {
            let mut kn: uint64_t = 0;
            let mut kb: *const uint8_t = key_bytes(
                c,
                (v & 0x7fffff as uint32_t).wrapping_sub(1 as uint32_t),
                &raw mut kn,
            );
            if kn == n
                && memcmp(
                    kb as *const ::core::ffi::c_void,
                    s as *const ::core::ffi::c_void,
                    n as size_t,
                ) == 0 as ::core::ffi::c_int
            {
                return slots.offset(i as isize) as *mut uint32_t;
            }
        }
        i = i.wrapping_add(1 as uint64_t) & (*c).voc_mask;
        k = k.wrapping_add(1);
    }
    return ::core::ptr::null_mut::<uint32_t>();
}
unsafe extern "C" fn written(
    mut cfg: *const toks_config,
    mut id: uint32_t,
    mut s: *const uint8_t,
    mut n: uint64_t,
) -> ::core::ffi::c_int {
    return (id < (*cfg).n_vocab && *(*cfg).vocab_len.offset(id as isize) as uint64_t == n
        && memcmp(
            *(*cfg).vocab.offset(id as isize) as *const ::core::ffi::c_void,
            s as *const ::core::ffi::c_void,
            n as size_t,
        ) == 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
}
unsafe extern "C" fn dec_rec(
    mut c: *const toks_ctx,
    mut id: uint32_t,
) -> *const uint32_t {
    let mut lo: uint32_t = 0 as uint32_t;
    let mut hi: uint32_t = (*c).voc_n_dec;
    while lo < hi {
        let mut mid: uint32_t = lo
            .wrapping_add(hi.wrapping_sub(lo).wrapping_div(2 as uint32_t));
        if *(*c)
            .voc_dec
            .offset(
                (4 as uint64_t).wrapping_mul(mid as uint64_t).wrapping_add(2 as uint64_t)
                    as isize,
            ) < id
        {
            lo = mid.wrapping_add(1 as uint32_t);
        } else {
            hi = mid;
        }
    }
    return if lo < (*c).voc_n_dec
        && *(*c)
            .voc_dec
            .offset(
                (4 as uint64_t).wrapping_mul(lo as uint64_t).wrapping_add(2 as uint64_t)
                    as isize,
            ) == id
    {
        (*c).voc_dec.offset((4 as uint64_t).wrapping_mul(lo as uint64_t) as isize)
    } else {
        ::core::ptr::null::<uint32_t>()
    };
}
unsafe extern "C" fn byte_flag(mut c: *const toks_ctx, mut id: uint32_t) -> uint32_t {
    let mut o: uint32_t = *(*c).t.tok_off.offset(id as isize);
    let mut n: uint64_t = (*(*c)
        .t
        .tok_off
        .offset(id.wrapping_add(1 as uint32_t) as isize) as uint64_t)
        .wrapping_sub(o as uint64_t);
    return if (*c).voc_bf == 1 as uint32_t
        && toks_byte_token((*c).t.tok_bytes.offset(o as isize), n)
            >= 0 as ::core::ffi::c_int
        || (*c).voc_bf == 2 as uint32_t && n == 1 as uint64_t
    {
        TOKS_ID_BYTE as uint32_t
    } else {
        0 as uint32_t
    };
}
#[no_mangle]
pub unsafe extern "C" fn toks_vocab_build(
    mut c: *mut toks_ctx,
    mut cfg: *const toks_config,
) -> int64_t {
    let mut n_ids: uint32_t = (*c).t.n_ids;
    let mut n_add: uint32_t = (*cfg).n_added;
    let mut pool: uint64_t = 0 as uint64_t;
    let mut keys: uint64_t = (n_add as uint64_t).wrapping_add(n_ids as uint64_t);
    let mut i: uint32_t = 0 as uint32_t;
    while i < n_add {
        pool = pool.wrapping_add((*(*cfg).added.offset(i as isize)).len as uint64_t);
        i = i.wrapping_add(1);
    }
    let mut size: uint64_t = 16 as uint64_t;
    while size < keys.wrapping_add(keys.wrapping_div(4 as uint64_t)) {
        size <<= 1 as ::core::ffi::c_int;
    }
    let mut words: uint64_t = (n_ids as uint64_t)
        .wrapping_add(31 as uint64_t)
        .wrapping_div(32 as uint64_t);
    let mut o_add: uint64_t = (4 as uint64_t).wrapping_mul(size);
    let mut o_dec: uint64_t = o_add
        .wrapping_add((16 as uint64_t).wrapping_mul(n_add as uint64_t));
    let mut o_flags: uint64_t = o_dec
        .wrapping_add((16 as uint64_t).wrapping_mul(n_add as uint64_t));
    let mut o_pool: uint64_t = o_flags.wrapping_add((8 as uint64_t).wrapping_mul(words));
    let mut total: uint64_t = o_pool.wrapping_add(pool).wrapping_add(63 as uint64_t)
        & !(63 as ::core::ffi::c_uint as uint64_t);
    let mut blk: *mut uint8_t = toks_plat_arena(total);
    if blk.is_null() {
        return TOKS_E_NOMEM as int64_t;
    }
    memset(blk as *mut ::core::ffi::c_void, 0 as ::core::ffi::c_int, total as size_t);
    (*c).mem_voc = blk;
    (*c).mem_voc_len = total;
    let mut slots: *mut uint32_t = toks_tab(
        blk,
        0 as uint64_t,
        (4 as uint64_t).wrapping_mul(size),
        TOKS_X_VOC_SLOTS,
    ) as *mut uint32_t;
    (*c).voc_slots = slots;
    (*c).voc_mask = size.wrapping_sub(1 as uint64_t);
    (*c).voc_add = toks_tab(
        blk,
        o_add,
        (16 as uint64_t).wrapping_mul(n_add as uint64_t),
        TOKS_X_VOC_ADD,
    ) as *mut uint32_t;
    (*c).voc_dec = toks_tab(
        blk,
        o_dec,
        (16 as uint64_t).wrapping_mul(n_add as uint64_t),
        TOKS_X_VOC_DEC,
    ) as *mut uint32_t;
    (*c).voc_added = toks_tab(
        blk,
        o_flags,
        (4 as uint64_t).wrapping_mul(words),
        TOKS_X_VOC_BITS,
    ) as *mut uint32_t;
    (*c).voc_special = toks_tab(
        blk,
        o_flags.wrapping_add((4 as uint64_t).wrapping_mul(words)),
        (4 as uint64_t).wrapping_mul(words),
        TOKS_X_VOC_BITS,
    ) as *mut uint32_t;
    (*c).voc_pool = toks_tab(blk, o_pool, pool, TOKS_X_VOC_POOL) as *mut uint8_t;
    toks_tab_seal(blk as *mut ::core::ffi::c_void, total);
    let mut na: uint32_t = 0 as uint32_t;
    let mut at: uint32_t = 0 as uint32_t;
    let mut add: *mut uint32_t = (*c).voc_add as uintptr_t as *mut uint32_t;
    let mut pl: *mut uint8_t = (*c).voc_pool as uintptr_t as *mut uint8_t;
    (*c).voc_n_add = n_add;
    let mut i_0: uint32_t = 0 as uint32_t;
    while i_0 < n_add {
        let mut a: *const toks_cfg_added = (*cfg).added.offset(i_0 as isize)
            as *mut toks_cfg_added;
        if !((*a).id >= n_ids) {
            memcpy(
                pl.offset(at as isize) as *mut ::core::ffi::c_void,
                (*a).content as *const ::core::ffi::c_void,
                (*a).len as size_t,
            );
            *add.offset((4 as uint32_t).wrapping_mul(na) as isize) = at;
            *add
                .offset(
                    (4 as uint32_t).wrapping_mul(na).wrapping_add(1 as uint32_t) as isize,
                ) = (*a).len;
            *add
                .offset(
                    (4 as uint32_t).wrapping_mul(na).wrapping_add(2 as uint32_t) as isize,
                ) = (*a).id;
            *add
                .offset(
                    (4 as uint32_t).wrapping_mul(na).wrapping_add(3 as uint32_t) as isize,
                ) = 0 as ::core::ffi::c_uint as uint32_t;
            let mut h: uint64_t = vhash(pl.offset(at as isize), (*a).len as uint64_t);
            let mut sl: *mut uint32_t = voc_find(
                c,
                slots,
                pl.offset(at as isize),
                (*a).len as uint64_t,
                h,
            );
            if sl.is_null() {
                return TOKS_E_NOMEM as int64_t;
            }
            if *sl == 0 as uint32_t {
                *sl = na.wrapping_add(1 as uint32_t)
                    | ((h >> 55 as ::core::ffi::c_int) as uint32_t)
                        << 23 as ::core::ffi::c_int;
                at = at.wrapping_add((*a).len);
                na = na.wrapping_add(1);
            }
            let ref mut fresh0 = *add
                .offset(
                    (4 as uint32_t)
                        .wrapping_mul(
                            (*sl & 0x7fffff as uint32_t).wrapping_sub(1 as uint32_t),
                        )
                        .wrapping_add(3 as uint32_t) as isize,
                );
            *fresh0 = (*fresh0 as ::core::ffi::c_uint
                | if (*a).special as ::core::ffi::c_uint != 0 as ::core::ffi::c_uint {
                    1 as ::core::ffi::c_uint
                } else {
                    0 as ::core::ffi::c_uint
                }) as uint32_t;
        }
        i_0 = i_0.wrapping_add(1);
    }
    (*c).voc_pool = toks_tab_fit(
        (*c).voc_pool as *const ::core::ffi::c_void,
        at as uint64_t,
        TOKS_X_VOC_POOL,
    ) as *const uint8_t;
    let mut i_1: uint32_t = 0 as uint32_t;
    while i_1 < n_add {
        let mut a_0: *const toks_cfg_added = (*cfg).added.offset(i_1 as isize)
            as *mut toks_cfg_added;
        if !((*a_0).id >= n_ids) {
            let mut sl_0: *mut uint32_t = voc_find(
                c,
                slots,
                (*a_0).content,
                (*a_0).len as uint64_t,
                vhash((*a_0).content, (*a_0).len as uint64_t),
            );
            let mut ad: *mut uint32_t = (*c).voc_added as uintptr_t as *mut uint32_t;
            let mut sp: *mut uint32_t = (*c).voc_special as uintptr_t as *mut uint32_t;
            let mut m: uint32_t = (1 as uint32_t) << ((*a_0).id & 31 as uint32_t);
            *ad.offset(((*a_0).id >> 5 as ::core::ffi::c_int) as isize) |= m;
            if !sl_0.is_null() && *sl_0 != 0 as uint32_t
                && *add
                    .offset(
                        (4 as uint32_t)
                            .wrapping_mul(
                                (*sl_0 & 0x7fffff as uint32_t).wrapping_sub(1 as uint32_t),
                            )
                            .wrapping_add(3 as uint32_t) as isize,
                    ) != 0 as uint32_t
            {
                *sp.offset(((*a_0).id >> 5 as ::core::ffi::c_int) as isize) |= m;
            } else {
                *sp.offset(((*a_0).id >> 5 as ::core::ffi::c_int) as isize) &= !m;
            }
        }
        i_1 = i_1.wrapping_add(1);
    }
    let mut dec: *mut uint32_t = (*c).voc_dec as uintptr_t as *mut uint32_t;
    let mut nd: uint32_t = 0 as uint32_t;
    let mut id: uint32_t = 0 as uint32_t;
    while id < n_ids {
        if toks_bit((*c).voc_added, id) != 0 as uint32_t {
            *dec
                .offset(
                    (4 as uint32_t).wrapping_mul(nd).wrapping_add(2 as uint32_t) as isize,
                ) = id;
            nd = nd.wrapping_add(1);
        }
        id = id.wrapping_add(1);
    }
    (*c).voc_n_dec = nd;
    let mut i_2: uint32_t = 0 as uint32_t;
    while i_2 < n_add {
        let mut a_1: *const toks_cfg_added = (*cfg).added.offset(i_2 as isize)
            as *mut toks_cfg_added;
        let mut r: *mut uint32_t = if (*a_1).id < n_ids {
            dec_rec(c, (*a_1).id) as uintptr_t as *mut uint32_t
        } else {
            ::core::ptr::null_mut::<uint32_t>()
        };
        if !r.is_null()
            && (*r.offset(3 as ::core::ffi::c_int as isize) == 0 as uint32_t
                || (*a_1).last >= *r.offset(0 as ::core::ffi::c_int as isize))
        {
            *r.offset(0 as ::core::ffi::c_int as isize) = (*a_1).last;
            *r.offset(1 as ::core::ffi::c_int as isize) = i_2;
            *r.offset(3 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_uint
                as uint32_t;
        }
        i_2 = i_2.wrapping_add(1);
    }
    let mut k: uint32_t = 0 as uint32_t;
    while k < nd {
        let mut r_0: *mut uint32_t = dec
            .offset((4 as uint32_t).wrapping_mul(k) as isize);
        let mut a_2: *const toks_cfg_added = (*cfg)
            .added
            .offset(*r_0.offset(1 as ::core::ffi::c_int as isize) as isize)
            as *mut toks_cfg_added;
        let mut sl_1: *mut uint32_t = voc_find(
            c,
            slots,
            (*a_2).content,
            (*a_2).len as uint64_t,
            vhash((*a_2).content, (*a_2).len as uint64_t),
        );
        if *r_0.offset(3 as ::core::ffi::c_int as isize) == 0 as uint32_t
            || sl_1.is_null() || *sl_1 == 0 as uint32_t
        {
            return TOKS_E_NOMEM as int64_t;
        }
        *r_0.offset(0 as ::core::ffi::c_int as isize) = *add
            .offset(
                (4 as uint32_t)
                    .wrapping_mul(
                        (*sl_1 & 0x7fffff as uint32_t).wrapping_sub(1 as uint32_t),
                    ) as isize,
            );
        *r_0.offset(1 as ::core::ffi::c_int as isize) = (*a_2).len;
        *r_0.offset(3 as ::core::ffi::c_int as isize) = (TOKS_ID_ADDED
            | (*a_2).attr as ::core::ffi::c_uint) as uint32_t;
        k = k.wrapping_add(1);
    }
    (*c).voc_dec = toks_tab_fit(
        (*c).voc_dec as *const ::core::ffi::c_void,
        (16 as uint64_t).wrapping_mul(nd as uint64_t),
        TOKS_X_VOC_DEC,
    ) as *const uint32_t;
    (*c).voc_n_add = na;
    let mut id_0: uint32_t = 0 as uint32_t;
    while id_0 < n_ids {
        let mut o: uint64_t = *(*c).t.tok_off.offset(id_0 as isize) as uint64_t;
        let mut n: uint64_t = (*(*c)
            .t
            .tok_off
            .offset(id_0.wrapping_add(1 as uint32_t) as isize) as uint64_t)
            .wrapping_sub(o);
        if !(n == 0 as uint64_t) {
            let mut s: *const uint8_t = (*c).t.tok_bytes.offset(o as isize);
            let mut h_0: uint64_t = vhash(s, n);
            let mut sl_2: *mut uint32_t = voc_find(c, slots, s, n, h_0);
            if sl_2.is_null() {
                return TOKS_E_NOMEM as int64_t;
            }
            if !(*sl_2 != 0 as uint32_t
                && (*sl_2 & 0x7fffff as uint32_t).wrapping_sub(1 as uint32_t) < na)
            {
                if !(*sl_2 != 0 as uint32_t
                    && written(
                        cfg,
                        (*sl_2 & 0x7fffff as uint32_t)
                            .wrapping_sub(1 as uint32_t)
                            .wrapping_sub(na),
                        s,
                        n,
                    ) != 0 && written(cfg, id_0, s, n) == 0)
                {
                    *sl_2 = na.wrapping_add(id_0).wrapping_add(1 as uint32_t)
                        | ((h_0 >> 55 as ::core::ffi::c_int) as uint32_t)
                            << 23 as ::core::ffi::c_int;
                }
            }
        }
        id_0 = id_0.wrapping_add(1);
    }
    (*c).voc_bf = (if (*c).dc.on == 0 && (*c).wp.is_null()
        && (*c).dec_byte_level != 0 as uint32_t
    {
        2 as ::core::ffi::c_uint
    } else {
        0 as ::core::ffi::c_uint
    }) as uint32_t;
    let mut i_3: uint32_t = 0 as uint32_t;
    while (*c).dc.on != 0 && i_3 < (*c).dc.n_dec && i_3 < TOKS_SPM_MAX_OPS as uint32_t {
        if (*(*c).dc.dec.offset(i_3 as isize)).kind
            == TOKS_SPM_D_BYTE_FALLBACK as ::core::ffi::c_int as uint32_t
        {
            (*c).voc_bf = 1 as ::core::ffi::c_uint as uint32_t;
        }
        i_3 = i_3.wrapping_add(1);
    }
    return 0 as int64_t;
}
#[no_mangle]
pub unsafe extern "C" fn toks_token_to_id(
    mut ctx: *const toks_ctx,
    mut s: *const ::core::ffi::c_void,
    mut len: uint64_t,
) -> int64_t {
    if ctx.is_null() || s.is_null() && len != 0 as uint64_t {
        return TOKS_E_ARG as int64_t;
    }
    if len == 0 as uint64_t || len > TOKS_MAX_TOKEN_BYTES as uint64_t
        || (*ctx).voc_slots.is_null()
    {
        return TOKS_E_ID as int64_t;
    }
    let mut b: *const uint8_t = s as *const uint8_t;
    let mut sl: *mut uint32_t = voc_find(
        ctx,
        (*ctx).voc_slots as uintptr_t as *mut uint32_t,
        b,
        len,
        vhash(b, len),
    );
    if sl.is_null() || *sl == 0 as uint32_t {
        return TOKS_E_ID as int64_t;
    }
    return key_id(ctx, (*sl & 0x7fffff as uint32_t).wrapping_sub(1 as uint32_t))
        as int64_t;
}
#[no_mangle]
pub unsafe extern "C" fn toks_id_flags(
    mut ctx: *const toks_ctx,
    mut id: uint32_t,
) -> int64_t {
    if ctx.is_null() {
        return TOKS_E_ARG as int64_t;
    }
    if id >= (*ctx).t.n_ids {
        return TOKS_E_ID as int64_t;
    }
    let mut f: uint32_t = byte_flag(ctx, id);
    if !(*ctx).voc_added.is_null() && toks_bit((*ctx).voc_added, id) != 0 as uint32_t {
        f = (f as ::core::ffi::c_uint
            | (TOKS_ID_ADDED
                | (if toks_bit((*ctx).voc_special, id) != 0 as uint32_t {
                    TOKS_ID_SPECIAL
                } else {
                    0 as ::core::ffi::c_uint
                }))) as uint32_t;
    }
    return f as int64_t;
}
#[no_mangle]
pub unsafe extern "C" fn toks_added(
    mut ctx: *const toks_ctx,
    mut i: uint32_t,
    mut content: *mut *const ::core::ffi::c_void,
    mut len: *mut uint64_t,
    mut id: *mut uint32_t,
) -> int64_t {
    if ctx.is_null() || i >= (*ctx).voc_n_dec {
        return TOKS_E_ARG as int64_t;
    }
    let mut r: *const uint32_t = (*ctx)
        .voc_dec
        .offset((4 as uint64_t).wrapping_mul(i as uint64_t) as isize);
    if !content.is_null() {
        *content = (*ctx)
            .voc_pool
            .offset(*r.offset(0 as ::core::ffi::c_int as isize) as isize)
            as *const ::core::ffi::c_void;
    }
    if !len.is_null() {
        *len = *r.offset(1 as ::core::ffi::c_int as isize) as uint64_t;
    }
    if !id.is_null() {
        *id = *r.offset(2 as ::core::ffi::c_int as isize);
    }
    return (*r.offset(3 as ::core::ffi::c_int as isize)
        | byte_flag(ctx, *r.offset(2 as ::core::ffi::c_int as isize))) as int64_t;
}
