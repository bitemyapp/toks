#[repr(C)]
pub struct toks_gen { _opaque: [u8; 0] }
#[repr(C)]
pub struct toks_uni_src { _opaque: [u8; 0] }
#[repr(C)]
pub struct toks_spm_config { _opaque: [u8; 0] }
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
    static TOKS_CRC32C_TAB: [uint32_t; 256];
    fn toks_alpha_bytes(s: *const uint8_t, n: uint32_t, out: *mut uint8_t) -> int64_t;
    fn toks_k6_bpe_c(t: *const toks_tables, a: *mut toks_k6_args) -> uint64_t;
    static toks_dict_n: uint32_t;
    static toks_dict: [uint8_t; 0];
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
pub struct toks_k6_args {
    pub piece: *const uint8_t,
    pub len: uint64_t,
    pub out: *mut uint32_t,
    pub work: *mut uint8_t,
    pub work_bytes: uint64_t,
    pub n_out: uint64_t,
    pub merges: uint64_t,
    pub rsv: uint64_t,
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
pub struct bpe_key {
    pub lo: uint64_t,
    pub hi: uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pm_cand {
    pub id: uint32_t,
    pub rb: uint32_t,
    pub s1: uint32_t,
    pub s2: uint32_t,
    pub le: [uint32_t; 2],
    pub leb: [uint32_t; 2],
    pub re: [uint32_t; 2],
    pub reb: [uint32_t; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bpe_mt {
    pub slots: *const uint64_t,
    pub shift: uint64_t,
    pub mask: uint64_t,
    pub maxprobe: uint64_t,
    pub r2i: *const uint32_t,
    pub pf: *const uint64_t,
}
pub const UINT32_MAX: ::core::ffi::c_uint = 4294967295 as ::core::ffi::c_uint;
pub const UINT64_MAX: ::core::ffi::c_ulong = 18446744073709551615
    as ::core::ffi::c_ulong;
pub const TOKS_E_FORMAT: ::core::ffi::c_int = -(2 as ::core::ffi::c_int);
pub const TOKS_E_UNSUPPORTED: ::core::ffi::c_int = -(3 as ::core::ffi::c_int);
pub const TOKS_E_NOMEM: ::core::ffi::c_int = -(11 as ::core::ffi::c_int);
pub const TOKS_MAX_IDS: ::core::ffi::c_uint = ((1 as ::core::ffi::c_uint)
    << 21 as ::core::ffi::c_int)
    .wrapping_sub(1 as ::core::ffi::c_uint);
pub const TOKS_ID_MASK: ::core::ffi::c_uint = 0x1fffff as ::core::ffi::c_uint;
pub const TOKS_TF_IGNORE_MERGES: ::core::ffi::c_uint = 0x1 as ::core::ffi::c_uint;
pub const TOKS_TF_IDS_AS_RANK: ::core::ffi::c_uint = 0x2 as ::core::ffi::c_uint;
pub const TOKS_TF_PROBE_LONG: ::core::ffi::c_uint = 0x80 as ::core::ffi::c_uint;
pub const TOKS_TF_PROBE_ASCII: ::core::ffi::c_uint = 0x100 as ::core::ffi::c_uint;
pub const TOKS_KEY_MAXLEN: ::core::ffi::c_int = 15 as ::core::ffi::c_int;
pub const TOKS_HSEED: ::core::ffi::c_uint = 0x9e3779b9 as ::core::ffi::c_uint;
pub const TOKS_VAL_COUNT_SHIFT: ::core::ffi::c_int = 29 as ::core::ffi::c_int;
pub const TOKS_BUCKET: ::core::ffi::c_int = 64 as ::core::ffi::c_int;
pub const TOKS_FIB64: ::core::ffi::c_ulonglong = 0x9e3779b97f4a7c15
    as ::core::ffi::c_ulonglong;
pub const TOKS_PRIO_BITS: ::core::ffi::c_int = 22 as ::core::ffi::c_int;
pub const TOKS_PRIO_NONE: ::core::ffi::c_uint = 0xffffffff as ::core::ffi::c_uint;
pub const TOKS_PM_STAGE1: ::core::ffi::c_int = 1056 as ::core::ffi::c_int;
pub const TOKS_PM_HDR: ::core::ffi::c_int = 4352 as ::core::ffi::c_int;
pub const TOKS_PM_BLOCK: ::core::ffi::c_int = 512 as ::core::ffi::c_int;
pub const TOKS_PM_ALWAYS: ::core::ffi::c_int = 63 as ::core::ffi::c_int;
pub const TOKS_APM_PAIRS: ::core::ffi::c_int = 256 as ::core::ffi::c_int;
pub const TOKS_APM_BYTES: ::core::ffi::c_int = 256 as ::core::ffi::c_int
    + 16384 as ::core::ffi::c_int * 16 as ::core::ffi::c_int;
pub const TOKS_VSEED: ::core::ffi::c_uint = 0x85ebca6b as ::core::ffi::c_uint;
pub const TOKS_X_BYTE2ID: toks_ext = toks_ext {
    pad: 0 as uint32_t,
    align: 4 as uint32_t,
};
pub const TOKS_X_MERGE_SLOTS: toks_ext = toks_ext {
    pad: 0 as uint32_t,
    align: 64 as uint32_t,
};
pub const TOKS_X_PAIRF: toks_ext = toks_ext {
    pad: 0 as uint32_t,
    align: 8 as uint32_t,
};
pub const TOKS_X_RANK2ID: toks_ext = toks_ext {
    pad: 0 as uint32_t,
    align: 4 as uint32_t,
};
pub const TOKS_X_BYTEPAIR: toks_ext = toks_ext {
    pad: 0 as uint32_t,
    align: 4 as uint32_t,
};
pub const TOKS_X_VHASH: toks_ext = toks_ext {
    pad: 0 as uint32_t,
    align: 8 as uint32_t,
};
pub const TOKS_X_WORDS: toks_ext = toks_ext {
    pad: 0 as uint32_t,
    align: 64 as uint32_t,
};
pub const TOKS_X_PREMERGE: toks_ext = toks_ext {
    pad: 0 as uint32_t,
    align: 8 as uint32_t,
};
pub const TOKS_X_APM: toks_ext = toks_ext {
    pad: 0 as uint32_t,
    align: 4 as uint32_t,
};
#[inline]
unsafe extern "C" fn toks_ld32(mut p: *const ::core::ffi::c_void) -> uint32_t {
    let mut v: uint32_t = 0;
    memcpy(&raw mut v as *mut ::core::ffi::c_void, p, 4 as size_t);
    return v;
}
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
unsafe extern "C" fn toks_tab_ar(
    mut a: *mut toks_arena,
    mut n: uint64_t,
    mut align: uint64_t,
    mut x: toks_ext,
) -> *mut ::core::ffi::c_void {
    let mut p: *mut ::core::ffi::c_void = toks_ar_alloc(
        a,
        n.wrapping_add(x.pad as uint64_t),
        align,
    );
    return p;
}
#[inline]
unsafe extern "C" fn toks_crc32c_u32(mut crc: uint32_t, mut v: uint32_t) -> uint32_t {
    crc = TOKS_CRC32C_TAB[((crc ^ v) & 0xff as uint32_t) as usize]
        ^ crc >> 8 as ::core::ffi::c_int;
    crc = TOKS_CRC32C_TAB[((crc ^ v >> 8 as ::core::ffi::c_int) & 0xff as uint32_t)
        as usize] ^ crc >> 8 as ::core::ffi::c_int;
    crc = TOKS_CRC32C_TAB[((crc ^ v >> 16 as ::core::ffi::c_int) & 0xff as uint32_t)
        as usize] ^ crc >> 8 as ::core::ffi::c_int;
    crc = TOKS_CRC32C_TAB[((crc ^ v >> 24 as ::core::ffi::c_int) & 0xff as uint32_t)
        as usize] ^ crc >> 8 as ::core::ffi::c_int;
    return crc;
}
#[inline]
unsafe extern "C" fn toks_crc32c_u64(mut crc: uint32_t, mut v: uint64_t) -> uint32_t {
    crc = toks_crc32c_u32(crc, v as uint32_t);
    crc = toks_crc32c_u32(crc, (v >> 32 as ::core::ffi::c_int) as uint32_t);
    return crc;
}
#[inline]
unsafe extern "C" fn bpe_pow2(mut n: uint64_t) -> uint64_t {
    let mut p: uint64_t = 1 as uint64_t;
    while p < n {
        p <<= 1 as ::core::ffi::c_int;
    }
    return p;
}
pub const BPE_PRIO_MASK: ::core::ffi::c_ulong = ((1 as ::core::ffi::c_ulong)
    << TOKS_PRIO_BITS)
    .wrapping_sub(1 as ::core::ffi::c_ulong);
pub const BPE_EMPTY_SLOT: ::core::ffi::c_ulong = UINT64_MAX;
#[inline]
unsafe extern "C" fn bpe_pair_key(mut left: uint32_t, mut right: uint32_t) -> uint64_t {
    return (left as uint64_t) << 21 as ::core::ffi::c_int | right as uint64_t;
}
#[inline]
unsafe extern "C" fn bpe_mt_of(mut t: *const toks_tables) -> bpe_mt {
    let mut mt: bpe_mt = bpe_mt {
        slots: (*t).merge_slots,
        shift: (*t).merge_shift,
        mask: (*t).merge_mask,
        maxprobe: (*t).merge_maxprobe,
        r2i: if (*t).flags & TOKS_TF_IDS_AS_RANK as uint32_t != 0 as uint32_t {
            ::core::ptr::null::<uint32_t>()
        } else {
            (*t).rank2id
        },
        pf: (*t).pairf,
    };
    return mt;
}
#[inline]
unsafe extern "C" fn bpe_pf_bits(
    mut key: uint64_t,
    mut shift: uint64_t,
    mut w: *mut uint64_t,
) -> uint64_t {
    let mut h: uint64_t = (key as ::core::ffi::c_ulonglong).wrapping_mul(TOKS_FIB64)
        as uint64_t;
    *w = h >> shift >> 1 as ::core::ffi::c_int;
    return (1 as uint64_t) << (h >> 20 as ::core::ffi::c_int & 63 as uint64_t)
        | (1 as uint64_t) << (h >> 26 as ::core::ffi::c_int & 63 as uint64_t);
}
#[inline]
unsafe extern "C" fn bpe_mt_find(mut mt: *const bpe_mt, mut key: uint64_t) -> uint32_t {
    let mut b: uint64_t = ((key as ::core::ffi::c_ulonglong).wrapping_mul(TOKS_FIB64)
        >> (*mt).shift & (*mt).mask as ::core::ffi::c_ulonglong) as uint64_t;
    let mut w: uint64_t = 0 as uint64_t;
    let mut m: uint64_t = if !(*mt).pf.is_null() {
        bpe_pf_bits(key, (*mt).shift, &raw mut w)
    } else {
        0 as uint64_t
    };
    if (if !(*mt).pf.is_null() { *(*mt).pf.offset(w as isize) & m } else { m }) != m {
        return TOKS_PRIO_NONE as uint32_t;
    }
    let mut i: uint64_t = 0 as uint64_t;
    while i < (*mt).maxprobe {
        let mut s: *const uint64_t = (*mt)
            .slots
            .offset(b.wrapping_mul(8 as uint64_t) as isize);
        let mut j: uint32_t = 0 as uint32_t;
        while j < 8 as uint32_t {
            if *s.offset(j as isize) == BPE_EMPTY_SLOT as uint64_t {
                return TOKS_PRIO_NONE as uint32_t;
            }
            if *s.offset(j as isize) >> TOKS_PRIO_BITS == key {
                return (*s.offset(j as isize) & BPE_PRIO_MASK as uint64_t) as uint32_t;
            }
            j = j.wrapping_add(1);
        }
        b = b.wrapping_add(1 as uint64_t) & (*mt).mask;
        i = i.wrapping_add(1);
    }
    return TOKS_PRIO_NONE as uint32_t;
}
#[inline]
unsafe extern "C" fn bpe_mt_id(mut mt: *const bpe_mt, mut prio: uint32_t) -> uint32_t {
    return if (*mt).r2i.is_null() { prio } else { *(*mt).r2i.offset(prio as isize) };
}
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
#[inline]
unsafe extern "C" fn bpe_vhash_h(mut p: *const uint8_t, mut len: uint64_t) -> uint32_t {
    let mut h: uint32_t = TOKS_VSEED as uint32_t ^ len as uint32_t;
    let mut i: uint64_t = 0 as uint64_t;
    while len.wrapping_sub(i) >= 8 as uint64_t {
        h = toks_crc32c_u64(h, bpe_load_le(p.offset(i as isize), 8 as uint64_t));
        i = i.wrapping_add(8 as uint64_t);
    }
    if i < len {
        h = toks_crc32c_u64(h, bpe_load_le(p.offset(i as isize), len.wrapping_sub(i)));
    }
    return h;
}
#[inline]
unsafe extern "C" fn bpe_apm_class(mut b: uint32_t) -> uint32_t {
    let mut l: uint32_t = (b | 0x20 as uint32_t).wrapping_sub(0x61 as uint32_t);
    return if l < 26 as uint32_t {
        l
    } else if b == 0x20 as uint32_t {
        26 as uint32_t
    } else if b.wrapping_sub(0x30 as uint32_t) < 10 as uint32_t {
        27 as uint32_t
    } else if b < 0x80 as uint32_t {
        28 as uint32_t
    } else if b < 0xc0 as uint32_t {
        29 as uint32_t
    } else if b < 0xe0 as uint32_t {
        30 as uint32_t
    } else {
        31 as uint32_t
    };
}
#[inline]
unsafe extern "C" fn bpe_pm_lbit(mut b: uint32_t) -> uint32_t {
    return if b < 0x80 as uint32_t {
        if b == 0x20 as uint32_t { 56 as uint32_t } else { 57 as uint32_t }
    } else if b < 0xc0 as uint32_t {
        (40 as uint32_t).wrapping_add(b & 15 as uint32_t)
    } else {
        63 as uint32_t
    };
}
#[inline]
unsafe extern "C" fn bpe_pm_rbit(mut b: uint32_t) -> uint32_t {
    return if b < 0x80 as uint32_t {
        58 as uint32_t
    } else if b < 0xc0 as uint32_t {
        63 as uint32_t
    } else if b < 0xe0 as uint32_t {
        59 as uint32_t
    } else if b < 0xf0 as uint32_t {
        (24 as uint32_t).wrapping_add(b & 15 as uint32_t)
    } else {
        60 as uint32_t
    };
}
#[inline]
unsafe extern "C" fn bpe_pm_index(
    mut b: *const uint8_t,
    mut avail: uint64_t,
    mut s1: *mut uint32_t,
    mut s2: *mut uint32_t,
) -> uint32_t {
    let mut b0: uint32_t = *b.offset(0 as ::core::ffi::c_int as isize) as uint32_t;
    if b0 >= 0xc0 as uint32_t && b0 < 0xe0 as uint32_t && avail >= 2 as uint64_t
        && *b.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint
            & 0xc0 as ::core::ffi::c_uint == 0x80 as ::core::ffi::c_uint
    {
        *s1 = b0.wrapping_sub(0xc0 as uint32_t);
        *s2 = (*b.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint
            & 63 as ::core::ffi::c_uint) as uint32_t;
        return 2 as uint32_t;
    }
    if b0 >= 0xe0 as uint32_t && b0 < 0xf0 as uint32_t && avail >= 3 as uint64_t
        && *b.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint
            & 0xc0 as ::core::ffi::c_uint == 0x80 as ::core::ffi::c_uint
        && *b.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint
            & 0xc0 as ::core::ffi::c_uint == 0x80 as ::core::ffi::c_uint
    {
        *s1 = (32 as uint32_t)
            .wrapping_add(
                (b0 & 15 as uint32_t) << 6 as ::core::ffi::c_int
                    | *b.offset(1 as ::core::ffi::c_int as isize) as uint32_t
                        & 63 as uint32_t,
            );
        *s2 = (*b.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint
            & 63 as ::core::ffi::c_uint) as uint32_t;
        return 3 as uint32_t;
    }
    return 0 as uint32_t;
}
#[inline]
unsafe extern "C" fn bpe_key_at(
    mut text: *const uint8_t,
    mut tlen: uint64_t,
    mut s: uint64_t,
    mut n: uint64_t,
) -> bpe_key {
    let mut k: bpe_key = bpe_key { lo: 0, hi: 0 };
    let mut p: *const uint8_t = text.offset(s as isize);
    if s.wrapping_add(16 as uint64_t) <= tlen {
        memcpy(
            &raw mut k.lo as *mut ::core::ffi::c_void,
            p as *const ::core::ffi::c_void,
            8 as size_t,
        );
        memcpy(
            &raw mut k.hi as *mut ::core::ffi::c_void,
            p.offset(8 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_void,
            8 as size_t,
        );
        k.lo = (k.lo as ::core::ffi::c_ulong
            & if n >= 8 as uint64_t {
                UINT64_MAX
            } else {
                ((1 as ::core::ffi::c_ulong) << (8 as uint64_t).wrapping_mul(n))
                    .wrapping_sub(1 as ::core::ffi::c_ulong)
            }) as uint64_t;
        k.hi = (k.hi as ::core::ffi::c_ulong
            & if n > 8 as uint64_t {
                ((1 as ::core::ffi::c_ulong)
                    << (8 as uint64_t).wrapping_mul(n.wrapping_sub(8 as uint64_t)))
                    .wrapping_sub(1 as ::core::ffi::c_ulong)
            } else {
                0 as ::core::ffi::c_ulong
            }) as uint64_t;
    } else if n > 8 as uint64_t {
        memcpy(
            &raw mut k.lo as *mut ::core::ffi::c_void,
            p as *const ::core::ffi::c_void,
            8 as size_t,
        );
        k.hi = bpe_load_le(
            p.offset(8 as ::core::ffi::c_int as isize),
            n.wrapping_sub(8 as uint64_t),
        );
    } else {
        k.lo = bpe_load_le(p, n);
        k.hi = 0 as uint64_t;
    }
    k.hi |= n << 56 as ::core::ffi::c_int;
    return k;
}
#[inline]
unsafe extern "C" fn bpe_key_hash(mut k: bpe_key) -> uint32_t {
    return toks_crc32c_u64(toks_crc32c_u64(TOKS_HSEED as uint32_t, k.lo), k.hi);
}
#[inline]
unsafe extern "C" fn bpe_bucket_get(
    mut b: *const uint8_t,
    mut k: bpe_key,
) -> *const uint8_t {
    let mut w: [uint64_t; 4] = [0; 4];
    memcpy(
        &raw mut w as *mut uint64_t as *mut ::core::ffi::c_void,
        b as *const ::core::ffi::c_void,
        32 as size_t,
    );
    if w[0 as ::core::ffi::c_int as usize] == k.lo
        && w[1 as ::core::ffi::c_int as usize] == k.hi
    {
        return b.offset(32 as ::core::ffi::c_int as isize);
    }
    if w[2 as ::core::ffi::c_int as usize] == k.lo
        && w[3 as ::core::ffi::c_int as usize] == k.hi
    {
        return b.offset(48 as ::core::ffi::c_int as isize);
    }
    return ::core::ptr::null::<uint8_t>();
}
#[inline]
unsafe extern "C" fn bpe_val_count(mut v: *const uint32_t) -> uint32_t {
    return *v.offset(0 as ::core::ffi::c_int as isize) >> TOKS_VAL_COUNT_SHIFT;
}
#[inline]
unsafe extern "C" fn bpe_val_pack_tag(
    mut val: *mut uint32_t,
    mut idv: *const uint32_t,
    mut n: uint32_t,
    mut tw: uint64_t,
) {
    *val.offset(0 as ::core::ffi::c_int as isize) = n << TOKS_VAL_COUNT_SHIFT
        | toks_ld32(idv as *const ::core::ffi::c_void) & TOKS_ID_MASK as uint32_t;
    *val.offset(1 as ::core::ffi::c_int as isize) = if n > 1 as uint32_t {
        toks_ld32(
            idv.offset(1 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_void,
        ) & TOKS_ID_MASK as uint32_t
    } else {
        0 as uint32_t
    };
    *val.offset(2 as ::core::ffi::c_int as isize) = (if n > 2 as uint32_t {
        toks_ld32(
            idv.offset(2 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_void,
        ) & TOKS_ID_MASK as uint32_t
    } else {
        0 as uint32_t
    }) | tw as uint32_t;
    *val.offset(3 as ::core::ffi::c_int as isize) = (if n > 3 as uint32_t {
        toks_ld32(
            idv.offset(3 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_void,
        ) & TOKS_ID_MASK as uint32_t
    } else {
        0 as uint32_t
    }) | (tw >> 32 as ::core::ffi::c_int) as uint32_t;
}
#[inline]
unsafe extern "C" fn bpe_words_put(
    mut words: *mut uint8_t,
    mut mask: uint64_t,
    mut h: uint32_t,
    mut k: bpe_key,
    mut val: *const uint32_t,
) -> ::core::ffi::c_int {
    let mut which: uint32_t = 0 as uint32_t;
    while which < 2 as uint32_t {
        let mut b: *mut uint8_t = words
            .offset(
                ((if which == 0 as uint32_t {
                    h
                } else {
                    h >> 16 as ::core::ffi::c_int | h << 16 as ::core::ffi::c_int
                }) as uint64_t & mask)
                    .wrapping_mul(TOKS_BUCKET as uint64_t) as isize,
            );
        let mut way: uint32_t = 0 as uint32_t;
        while way < 2 as uint32_t {
            if *b.offset(15 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint
                != 0 as ::core::ffi::c_uint
            {
                way = way.wrapping_add(1);
                b = b.offset(16 as ::core::ffi::c_int as isize);
            } else {
                memcpy(
                    b as *mut ::core::ffi::c_void,
                    &raw mut k.lo as *const ::core::ffi::c_void,
                    8 as size_t,
                );
                memcpy(
                    b.offset(8 as ::core::ffi::c_int as isize)
                        as *mut ::core::ffi::c_void,
                    &raw mut k.hi as *const ::core::ffi::c_void,
                    8 as size_t,
                );
                memcpy(
                    b.offset(32 as ::core::ffi::c_int as isize)
                        as *mut ::core::ffi::c_void,
                    val as *const ::core::ffi::c_void,
                    16 as size_t,
                );
                return 1 as ::core::ffi::c_int;
            }
        }
        which = which.wrapping_add(1);
    }
    return 0 as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn bpe_words_probe(
    mut words: *const uint8_t,
    mut mask: uint64_t,
    mut h: uint32_t,
    mut k: bpe_key,
) -> *const uint8_t {
    let mut v: *const uint8_t = bpe_bucket_get(
        words
            .offset(
                (h as uint64_t & mask).wrapping_mul(TOKS_BUCKET as uint64_t) as isize,
            ),
        k,
    );
    if v.is_null() {
        v = bpe_bucket_get(
            words
                .offset(
                    ((h >> 16 as ::core::ffi::c_int | h << 16 as ::core::ffi::c_int)
                        as uint64_t & mask)
                        .wrapping_mul(TOKS_BUCKET as uint64_t) as isize,
                ),
            k,
        );
    }
    return v;
}
unsafe extern "C" fn merge_buckets(mut n_merges: uint64_t) -> uint64_t {
    let mut b: uint64_t = bpe_pow2(
        n_merges.wrapping_div(5 as uint64_t).wrapping_add(1 as uint64_t),
    );
    return if b < 2 as uint64_t { 2 as uint64_t } else { b };
}
unsafe extern "C" fn vhash_slots(mut n_tokens: uint64_t) -> uint64_t {
    return bpe_pow2((2 as uint64_t).wrapping_mul(n_tokens).wrapping_add(1 as uint64_t));
}
unsafe extern "C" fn words_buckets(mut n_keys: uint64_t) -> uint64_t {
    return bpe_pow2(n_keys.wrapping_div(2 as uint64_t).wrapping_add(1 as uint64_t));
}
unsafe extern "C" fn premerge_size(mut nblk: uint64_t) -> uint64_t {
    return if nblk == 0 as uint64_t {
        0 as uint64_t
    } else {
        (TOKS_PM_HDR as uint64_t)
            .wrapping_add(
                (1 as uint64_t)
                    .wrapping_add(nblk)
                    .wrapping_mul(TOKS_PM_BLOCK as uint64_t),
            )
    };
}
unsafe extern "C" fn premerge_blocks(mut cfg: *const toks_config) -> uint64_t {
    if (*cfg).vocab.is_null() || (*cfg).vocab_len.is_null() {
        return TOKS_PM_STAGE1 as uint64_t;
    }
    let mut seen: [uint8_t; 132] = [0; 132];
    memset(
        &raw mut seen as *mut uint8_t as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<[uint8_t; 132]>() as size_t,
    );
    let mut nblk: uint64_t = 0 as uint64_t;
    let mut id: uint32_t = 0 as uint32_t;
    while id < (*cfg).n_vocab {
        let mut b: [uint8_t; 6] = [0; 6];
        let mut s1: uint32_t = 0 as uint32_t;
        let mut s2: uint32_t = 0 as uint32_t;
        let mut n: uint32_t = *(*cfg).vocab_len.offset(id as isize);
        if !(n < 2 as uint32_t || n > 6 as uint32_t) {
            let mut l: int64_t = toks_alpha_bytes(
                *(*cfg).vocab.offset(id as isize),
                n,
                &raw mut b as *mut uint8_t,
            );
            if (l == 2 as int64_t || l == 3 as int64_t)
                && bpe_pm_index(
                    &raw mut b as *mut uint8_t,
                    l as uint64_t,
                    &raw mut s1,
                    &raw mut s2,
                ) == l as uint32_t
            {
                if seen[(s1 >> 3 as ::core::ffi::c_int) as usize] as ::core::ffi::c_uint
                    & (1 as ::core::ffi::c_uint) << (s1 & 7 as uint32_t)
                    == 0 as ::core::ffi::c_uint
                {
                    seen[(s1 >> 3 as ::core::ffi::c_int) as usize] = (seen[(s1
                        >> 3 as ::core::ffi::c_int) as usize] as ::core::ffi::c_int
                        | ((1 as ::core::ffi::c_uint) << (s1 & 7 as uint32_t)) as uint8_t
                            as ::core::ffi::c_int) as uint8_t;
                    nblk = nblk.wrapping_add(1);
                }
            }
        }
        id = id.wrapping_add(1);
    }
    return nblk;
}
#[no_mangle]
pub unsafe extern "C" fn toks_bpe_tables_bytes(mut cfg: *const toks_config) -> uint64_t {
    let mut nm: uint64_t = (*cfg).n_merges as uint64_t;
    let mut nv: uint64_t = (*cfg).n_vocab as uint64_t;
    return ((256 as ::core::ffi::c_uint)
        .wrapping_mul(4 as ::core::ffi::c_uint)
        .wrapping_add(64 as ::core::ffi::c_uint) as uint64_t)
        .wrapping_add(
            merge_buckets(nm).wrapping_mul(64 as uint64_t).wrapping_add(64 as uint64_t),
        )
        .wrapping_add(
            merge_buckets(nm).wrapping_mul(4 as uint64_t).wrapping_add(64 as uint64_t),
        )
        .wrapping_add((4 as uint64_t).wrapping_mul(nm).wrapping_add(64 as uint64_t))
        .wrapping_add(
            (65536 as ::core::ffi::c_uint)
                .wrapping_mul(4 as ::core::ffi::c_uint)
                .wrapping_add(64 as ::core::ffi::c_uint) as uint64_t,
        )
        .wrapping_add(
            vhash_slots(nv)
                .wrapping_mul(8 as uint64_t)
                .wrapping_add(1024 as uint64_t)
                .wrapping_add(64 as uint64_t),
        )
        .wrapping_add(premerge_size(premerge_blocks(cfg)).wrapping_add(64 as uint64_t))
        .wrapping_add((TOKS_APM_BYTES as uint64_t).wrapping_add(64 as uint64_t))
        .wrapping_add(
            words_buckets(nv)
                .wrapping_mul(TOKS_BUCKET as uint64_t)
                .wrapping_add(64 as uint64_t),
        );
}
pub const PM_NONE: ::core::ffi::c_uint = UINT32_MAX;
pub const PM_ASCII: ::core::ffi::c_uint = 0x10000 as ::core::ffi::c_uint;
unsafe extern "C" fn pm_sim(
    mut t: *const toks_tables,
    mut b: *const uint8_t,
    mut n: uint32_t,
    mut id: uint32_t,
    mut c: *mut pm_cand,
) -> ::core::ffi::c_int {
    let mt: bpe_mt = bpe_mt_of(t) as bpe_mt;
    let mut sym: [uint32_t; 3] = [0; 3];
    let mut k: uint32_t = n;
    let mut nl: uint32_t = 0 as uint32_t;
    let mut nr: uint32_t = 0 as uint32_t;
    let mut lbd: uint32_t = 0 as uint32_t;
    let mut rbd: uint32_t = 0 as uint32_t;
    let mut i: uint32_t = 0 as uint32_t;
    while i < n {
        sym[i as usize] = *(*t).byte2id.offset(*b.offset(i as isize) as isize);
        i = i.wrapping_add(1);
    }
    let mut le: uint32_t = sym[0 as ::core::ffi::c_int as usize];
    let mut re: uint32_t = sym[n.wrapping_sub(1 as uint32_t) as usize];
    (*c).rb = 0 as ::core::ffi::c_uint as uint32_t;
    let mut j: uint32_t = 0 as uint32_t;
    while j < 2 as uint32_t {
        (*c).re[j as usize] = PM_NONE as uint32_t;
        (*c).le[j as usize] = (*c).re[j as usize];
        (*c).reb[j as usize] = 0 as ::core::ffi::c_uint as uint32_t;
        (*c).leb[j as usize] = (*c).reb[j as usize];
        j = j.wrapping_add(1);
    }
    while k > 1 as uint32_t {
        let mut best: uint32_t = PM_NONE as uint32_t;
        let mut at: uint32_t = 0 as uint32_t;
        let mut pr: uint32_t = 0 as uint32_t;
        let mut j_0: uint32_t = 0 as uint32_t;
        while j_0.wrapping_add(1 as uint32_t) < k {
            pr = bpe_mt_find(
                &raw const mt,
                bpe_pair_key(
                    sym[j_0 as usize],
                    sym[j_0.wrapping_add(1 as uint32_t) as usize],
                ),
            );
            if pr < best {
                best = pr;
                at = j_0;
            }
            j_0 = j_0.wrapping_add(1);
        }
        if best == PM_NONE as uint32_t {
            return 0 as ::core::ffi::c_int;
        }
        lbd = if lbd > best { lbd } else { best };
        rbd = if rbd > best { rbd } else { best };
        (*c).rb = if (*c).rb > best { (*c).rb } else { best };
        let mut left: ::core::ffi::c_int = (at == 0 as uint32_t) as ::core::ffi::c_int;
        let mut right: ::core::ffi::c_int = (at.wrapping_add(2 as uint32_t) == k)
            as ::core::ffi::c_int;
        sym[at as usize] = bpe_mt_id(&raw const mt, best);
        let mut j_1: uint32_t = at.wrapping_add(1 as uint32_t);
        while j_1.wrapping_add(1 as uint32_t) < k {
            sym[j_1 as usize] = sym[j_1.wrapping_add(1 as uint32_t) as usize];
            j_1 = j_1.wrapping_add(1);
        }
        k = (k as ::core::ffi::c_uint).wrapping_sub(1 as ::core::ffi::c_uint) as uint32_t
            as uint32_t;
        if left != 0 {
            (*c).le[nl as usize] = le;
            (*c).leb[nl as usize] = lbd;
            nl = nl.wrapping_add(1);
            le = sym[0 as ::core::ffi::c_int as usize];
            lbd = 0 as ::core::ffi::c_uint as uint32_t;
        }
        if right != 0 {
            (*c).re[nr as usize] = re;
            (*c).reb[nr as usize] = rbd;
            nr = nr.wrapping_add(1);
            re = sym[k.wrapping_sub(1 as uint32_t) as usize];
            rbd = 0 as ::core::ffi::c_uint as uint32_t;
        }
    }
    return (sym[0 as ::core::ffi::c_int as usize] == id) as ::core::ffi::c_int;
}
unsafe extern "C" fn pm_risk(
    mut off: *const uint32_t,
    mut ent: *const uint32_t,
    mut sym: uint32_t,
    mut bound: uint32_t,
    mut side: ::core::ffi::c_int,
) -> uint64_t {
    let mut m: uint64_t = 0 as uint64_t;
    if sym == PM_NONE as uint32_t {
        return 0 as uint64_t;
    }
    let mut i: uint32_t = *off.offset(sym as isize);
    while i < *off.offset(sym.wrapping_add(1 as uint32_t) as isize) {
        let mut b: uint32_t = *ent.offset(i as isize) & 511 as uint32_t;
        if !(*ent.offset(i as isize) >> 9 as ::core::ffi::c_int > bound) {
            if side == 2 as ::core::ffi::c_int {
                m = (m as ::core::ffi::c_ulong
                    | if b > 255 as uint32_t {
                        0xffffffff as ::core::ffi::c_ulong
                    } else {
                        (1 as ::core::ffi::c_ulong) << bpe_apm_class(b)
                    }) as uint64_t;
            } else {
                m = (m as ::core::ffi::c_ulong
                    | (1 as ::core::ffi::c_ulong)
                        << (if b > 255 as uint32_t {
                            TOKS_PM_ALWAYS as uint32_t
                        } else {
                            (if side == 0 as ::core::ffi::c_int {
                                bpe_pm_lbit(b)
                            } else {
                                bpe_pm_rbit(b)
                            })
                        })) as uint64_t;
            }
        }
        i = i.wrapping_add(1);
    }
    return m;
}
unsafe extern "C" fn pm_need(
    mut rel: *mut uint32_t,
    mut i: uint32_t,
    mut bound: uint32_t,
) {
    *rel.offset(i as isize) = if *rel.offset(i as isize) > bound {
        *rel.offset(i as isize)
    } else {
        bound.wrapping_add(1 as uint32_t)
    };
}
unsafe extern "C" fn premerge_build(
    mut t: *mut toks_tables,
    mut ar: *mut toks_arena,
    mut cfg: *const toks_config,
) -> int64_t {
    let mut offr: *mut uint32_t = ::core::ptr::null_mut::<uint32_t>();
    let mut offl: *mut uint32_t = ::core::ptr::null_mut::<uint32_t>();
    let mut k: uint32_t = 0;
    let mut ns: uint64_t = 0;
    let mut el: *const uint32_t = ::core::ptr::null::<uint32_t>();
    let mut nblk: uint32_t = 0;
    let mut na: uint32_t = 0;
    let mut s1blk: [uint32_t; 1056] = [0; 1056];
    let mut pm: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    let mut apm: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    let mut current_block: u64;
    let mut nv: uint32_t = (*cfg).n_vocab;
    (*t).premerge = ::core::ptr::null::<uint8_t>();
    (*t).apm = ::core::ptr::null::<uint8_t>();
    let mut nc: uint32_t = 0 as uint32_t;
    let mut id: uint32_t = 0 as uint32_t;
    while id < nv {
        let mut l: uint32_t = (*(*t)
            .tok_off
            .offset(id.wrapping_add(1 as uint32_t) as isize))
            .wrapping_sub(*(*t).tok_off.offset(id as isize));
        nc = nc
            .wrapping_add(
                (l == 2 as uint32_t || l == 3 as uint32_t) as ::core::ffi::c_int
                    as uint32_t,
            );
        id = id.wrapping_add(1);
    }
    if nc == 0 as uint32_t {
        return 0 as int64_t;
    }
    let mut cb: uint64_t = (nc as uint64_t)
        .wrapping_mul(::core::mem::size_of::<pm_cand>() as uint64_t);
    let mut rb: uint64_t = (8 as uint64_t)
        .wrapping_mul((nv as uint64_t).wrapping_add(1 as uint64_t));
    let mut ob: uint64_t = ((2 as ::core::ffi::c_uint)
        .wrapping_mul(4 as ::core::ffi::c_uint) as uint64_t)
        .wrapping_mul((nv as uint64_t).wrapping_add(2 as uint64_t));
    let mut cand: *mut pm_cand = toks_plat_alloc(cb) as *mut pm_cand;
    let mut rel: *mut uint32_t = toks_plat_alloc(rb) as *mut uint32_t;
    let mut off: *mut uint32_t = toks_plat_alloc(ob) as *mut uint32_t;
    let mut ent: *mut uint32_t = ::core::ptr::null_mut::<uint32_t>();
    let mut eb: uint64_t = 0 as uint64_t;
    let mut rc: int64_t = TOKS_E_NOMEM as int64_t;
    if !(cand.is_null() || rel.is_null() || off.is_null()) {
        memset(rel as *mut ::core::ffi::c_void, 0 as ::core::ffi::c_int, rb as size_t);
        memset(off as *mut ::core::ffi::c_void, 0 as ::core::ffi::c_int, ob as size_t);
        offr = off;
        offl = off.offset(nv as isize).offset(2 as ::core::ffi::c_uint as isize);
        k = 0 as uint32_t;
        let mut id_0: uint32_t = 0 as uint32_t;
        while id_0 < nv {
            let mut o: uint32_t = *(*t).tok_off.offset(id_0 as isize);
            let mut l_0: uint32_t = (*(*t)
                .tok_off
                .offset(id_0.wrapping_add(1 as uint32_t) as isize))
                .wrapping_sub(o);
            let mut s1: uint32_t = 0 as uint32_t;
            let mut s2: uint32_t = 0 as uint32_t;
            let mut b: *const uint8_t = (*t).tok_bytes.offset(o as isize);
            let mut ok: ::core::ffi::c_int = ((l_0 == 2 as uint32_t
                || l_0 == 3 as uint32_t)
                && bpe_pm_index(b, l_0 as uint64_t, &raw mut s1, &raw mut s2) == l_0)
                as ::core::ffi::c_int;
            if ok == 0 && l_0 == 2 as uint32_t
                && (*b.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    | *b.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
                    as ::core::ffi::c_uint & 0x80 as ::core::ffi::c_uint
                    == 0 as ::core::ffi::c_uint
            {
                s1 = PM_ASCII as uint32_t;
                s2 = (*b.offset(0 as ::core::ffi::c_int as isize) as uint32_t)
                    << 7 as ::core::ffi::c_int
                    | *b.offset(1 as ::core::ffi::c_int as isize) as uint32_t;
                ok = 1 as ::core::ffi::c_int;
            }
            if !(ok == 0 || model_token(cfg, id_0) == 0) {
                let mut c: *mut pm_cand = cand.offset(k as isize);
                if !(pm_sim(t, b, l_0, id_0, c) == 0) {
                    (*c).id = id_0;
                    (*c).s1 = s1;
                    (*c).s2 = s2;
                    pm_need(rel, (2 as uint32_t).wrapping_mul(id_0), (*c).rb);
                    pm_need(
                        rel,
                        (2 as uint32_t).wrapping_mul(id_0).wrapping_add(1 as uint32_t),
                        (*c).rb,
                    );
                    let mut j: uint32_t = 0 as uint32_t;
                    while j < 2 as uint32_t {
                        if (*c).le[j as usize] != PM_NONE as uint32_t {
                            pm_need(
                                rel,
                                (2 as uint32_t).wrapping_mul((*c).le[j as usize]),
                                (*c).leb[j as usize],
                            );
                        }
                        if (*c).re[j as usize] != PM_NONE as uint32_t {
                            pm_need(
                                rel,
                                (2 as uint32_t)
                                    .wrapping_mul((*c).re[j as usize])
                                    .wrapping_add(1 as uint32_t),
                                (*c).reb[j as usize],
                            );
                        }
                        j = j.wrapping_add(1);
                    }
                    k = (k as ::core::ffi::c_uint).wrapping_add(1 as ::core::ffi::c_uint)
                        as uint32_t as uint32_t;
                }
            }
            id_0 = id_0.wrapping_add(1);
        }
        nc = k;
        ns = (*t).merge_mask.wrapping_add(1 as uint64_t).wrapping_mul(8 as uint64_t);
        let mut pass: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        loop {
            if !(pass < 2 as ::core::ffi::c_int) {
                current_block = 15970011996474399071;
                break;
            }
            if pass == 1 as ::core::ffi::c_int {
                let mut i: uint32_t = 0 as uint32_t;
                while i < nv.wrapping_add(1 as uint32_t) {
                    let ref mut fresh0 = *offr
                        .offset(i.wrapping_add(1 as uint32_t) as isize);
                    *fresh0 = (*fresh0).wrapping_add(*offr.offset(i as isize));
                    let ref mut fresh1 = *offl
                        .offset(i.wrapping_add(1 as uint32_t) as isize);
                    *fresh1 = (*fresh1).wrapping_add(*offl.offset(i as isize));
                    i = i.wrapping_add(1);
                }
                eb = (4 as uint64_t)
                    .wrapping_mul(
                        (*offr.offset(nv.wrapping_add(1 as uint32_t) as isize)
                            as uint64_t)
                            .wrapping_add(
                                *offl.offset(nv.wrapping_add(1 as uint32_t) as isize)
                                    as uint64_t,
                            )
                            .wrapping_add(1 as uint64_t),
                    );
                ent = toks_plat_alloc(eb) as *mut uint32_t;
                if ent.is_null() {
                    current_block = 12138468505512240455;
                    break;
                }
            }
            let mut si: uint64_t = 0 as uint64_t;
            while si < ns {
                let mut sl: uint64_t = *(*t).merge_slots.offset(si as isize);
                if !(sl == BPE_EMPTY_SLOT as uint64_t) {
                    let mut key: uint64_t = sl >> TOKS_PRIO_BITS;
                    let mut pr: uint32_t = (sl & BPE_PRIO_MASK as uint64_t) as uint32_t;
                    let mut left: uint32_t = (key >> 21 as ::core::ffi::c_int)
                        as uint32_t;
                    let mut right: uint32_t = (key & 0x1fffff as uint64_t) as uint32_t;
                    if !(left >= nv || right >= nv) {
                        let mut ol: uint32_t = *(*t).tok_off.offset(left as isize);
                        let mut ll: uint32_t = (*(*t)
                            .tok_off
                            .offset(left.wrapping_add(1 as uint32_t) as isize))
                            .wrapping_sub(ol);
                        let mut orr: uint32_t = *(*t).tok_off.offset(right as isize);
                        let mut lr: uint32_t = (*(*t)
                            .tok_off
                            .offset(right.wrapping_add(1 as uint32_t) as isize))
                            .wrapping_sub(orr);
                        if pr < *rel.offset((2 as uint32_t).wrapping_mul(right) as isize)
                        {
                            let mut b_0: uint32_t = if ll > 0 as uint32_t {
                                *(*t)
                                    .tok_bytes
                                    .offset(
                                        ol.wrapping_add(ll).wrapping_sub(1 as uint32_t) as isize,
                                    ) as uint32_t
                            } else {
                                256 as uint32_t
                            };
                            if pass == 0 as ::core::ffi::c_int {
                                let ref mut fresh2 = *offr
                                    .offset(right.wrapping_add(2 as uint32_t) as isize);
                                *fresh2 = (*fresh2 as ::core::ffi::c_uint)
                                    .wrapping_add(1 as ::core::ffi::c_uint) as uint32_t
                                    as uint32_t;
                            } else {
                                let ref mut fresh3 = *offr
                                    .offset(right.wrapping_add(1 as uint32_t) as isize);
                                let fresh4 = *fresh3;
                                *fresh3 = (*fresh3).wrapping_add(1);
                                *ent.offset(fresh4 as isize) = pr << 9 as ::core::ffi::c_int
                                    | b_0;
                            }
                        }
                        if pr
                            < *rel
                                .offset(
                                    (2 as uint32_t)
                                        .wrapping_mul(left)
                                        .wrapping_add(1 as uint32_t) as isize,
                                )
                        {
                            let mut b_1: uint32_t = if lr > 0 as uint32_t {
                                *(*t).tok_bytes.offset(orr as isize) as uint32_t
                            } else {
                                256 as uint32_t
                            };
                            if pass == 0 as ::core::ffi::c_int {
                                let ref mut fresh5 = *offl
                                    .offset(left.wrapping_add(2 as uint32_t) as isize);
                                *fresh5 = (*fresh5 as ::core::ffi::c_uint)
                                    .wrapping_add(1 as ::core::ffi::c_uint) as uint32_t
                                    as uint32_t;
                            } else {
                                let ref mut fresh6 = *offl
                                    .offset(left.wrapping_add(1 as uint32_t) as isize);
                                let fresh7 = *fresh6;
                                *fresh6 = (*fresh6).wrapping_add(1);
                                *ent
                                    .offset(
                                        (*offr.offset(nv.wrapping_add(1 as uint32_t) as isize))
                                            .wrapping_add(fresh7) as isize,
                                    ) = pr << 9 as ::core::ffi::c_int | b_1;
                            }
                        }
                    }
                }
                si = si.wrapping_add(1);
            }
            pass += 1;
        }
        match current_block {
            12138468505512240455 => {}
            _ => {
                el = ent
                    .offset(
                        *offr.offset(nv.wrapping_add(1 as uint32_t) as isize) as isize,
                    );
                nblk = 0 as uint32_t;
                na = 0 as uint32_t;
                s1blk = [0; 1056];
                let mut i_0: uint32_t = 0 as uint32_t;
                while i_0 < TOKS_PM_STAGE1 as uint32_t {
                    s1blk[i_0 as usize] = 0 as ::core::ffi::c_uint as uint32_t;
                    i_0 = i_0.wrapping_add(1);
                }
                let mut i_1: uint32_t = 0 as uint32_t;
                while i_1 < nc {
                    if (*cand.offset(i_1 as isize)).s1 == PM_ASCII as uint32_t {
                        na = na.wrapping_add(1);
                    } else if s1blk[(*cand.offset(i_1 as isize)).s1 as usize]
                        == 0 as uint32_t
                    {
                        nblk = nblk.wrapping_add(1);
                        s1blk[(*cand.offset(i_1 as isize)).s1 as usize] = nblk;
                    }
                    i_1 = i_1.wrapping_add(1);
                }
                pm = if nblk > 0 as uint32_t {
                    toks_tab_ar(
                        ar,
                        premerge_size(nblk as uint64_t),
                        64 as uint64_t,
                        TOKS_X_PREMERGE,
                    ) as *mut uint8_t
                } else {
                    ::core::ptr::null_mut::<uint8_t>()
                };
                apm = if na > 0 as uint32_t {
                    toks_tab_ar(
                        ar,
                        TOKS_APM_BYTES as uint64_t,
                        64 as uint64_t,
                        TOKS_X_APM,
                    ) as *mut uint8_t
                } else {
                    ::core::ptr::null_mut::<uint8_t>()
                };
                if !(nblk > 0 as uint32_t && pm.is_null()
                    || na > 0 as uint32_t && apm.is_null())
                {
                    if !pm.is_null() {
                        memset(
                            pm as *mut ::core::ffi::c_void,
                            0 as ::core::ffi::c_int,
                            premerge_size(nblk as uint64_t) as size_t,
                        );
                        let mut i_2: uint32_t = 0 as uint32_t;
                        while i_2 < TOKS_PM_STAGE1 as uint32_t {
                            *(pm as *mut ::core::ffi::c_void as *mut uint32_t)
                                .offset(i_2 as isize) = (TOKS_PM_HDR as uint32_t)
                                .wrapping_add(
                                    s1blk[i_2 as usize].wrapping_mul(TOKS_PM_BLOCK as uint32_t),
                                );
                            i_2 = i_2.wrapping_add(1);
                        }
                    }
                    if !apm.is_null() {
                        memset(
                            apm.offset(TOKS_APM_PAIRS as isize)
                                as *mut ::core::ffi::c_void,
                            0xff as ::core::ffi::c_int,
                            (TOKS_APM_BYTES - TOKS_APM_PAIRS) as size_t,
                        );
                        let mut b_2: uint32_t = 0 as uint32_t;
                        while b_2 < 256 as uint32_t {
                            *apm.offset(b_2 as isize) = bpe_apm_class(b_2) as uint8_t;
                            b_2 = b_2.wrapping_add(1);
                        }
                    }
                    let mut i_3: uint32_t = 0 as uint32_t;
                    while i_3 < nc {
                        let mut c_0: *const pm_cand = cand.offset(i_3 as isize);
                        let mut a: ::core::ffi::c_int = ((*c_0).s1
                            == PM_ASCII as uint32_t) as ::core::ffi::c_int;
                        let mut ls: ::core::ffi::c_int = if a != 0 {
                            2 as ::core::ffi::c_int
                        } else {
                            0 as ::core::ffi::c_int
                        };
                        let mut rs: ::core::ffi::c_int = if a != 0 {
                            2 as ::core::ffi::c_int
                        } else {
                            1 as ::core::ffi::c_int
                        };
                        let mut before: uint64_t = pm_risk(
                            offr,
                            ent,
                            (*c_0).id,
                            (*c_0).rb,
                            ls,
                        );
                        let mut after: uint64_t = pm_risk(
                            offl,
                            el,
                            (*c_0).id,
                            (*c_0).rb,
                            rs,
                        );
                        let mut j_0: uint32_t = 0 as uint32_t;
                        while j_0 < 2 as uint32_t {
                            before
                                |= pm_risk(
                                    offr,
                                    ent,
                                    (*c_0).le[j_0 as usize],
                                    (*c_0).leb[j_0 as usize],
                                    ls,
                                );
                            after
                                |= pm_risk(
                                    offl,
                                    el,
                                    (*c_0).re[j_0 as usize],
                                    (*c_0).reb[j_0 as usize],
                                    rs,
                                );
                            j_0 = j_0.wrapping_add(1);
                        }
                        let mut v: [uint32_t; 4] = [
                            (*c_0).id,
                            after as uint32_t,
                            before as uint32_t,
                            0 as ::core::ffi::c_uint,
                        ];
                        if a != 0 {
                            memcpy(
                                apm
                                    .offset(TOKS_APM_PAIRS as isize)
                                    .offset((16 as uint32_t).wrapping_mul((*c_0).s2) as isize)
                                    as *mut ::core::ffi::c_void,
                                &raw mut v as *mut uint32_t as *const ::core::ffi::c_void,
                                16 as size_t,
                            );
                        } else {
                            let mut e: uint64_t = (*c_0).id as uint64_t
                                | (1 as uint64_t) << TOKS_PM_ALWAYS | before | after;
                            *(pm
                                .offset(
                                    *(pm as *const ::core::ffi::c_void as *const uint32_t)
                                        .offset((*c_0).s1 as isize) as isize,
                                )
                                .offset((8 as uint32_t).wrapping_mul((*c_0).s2) as isize)
                                as *mut ::core::ffi::c_void as *mut uint64_t) = e;
                        }
                        i_3 = i_3.wrapping_add(1);
                    }
                    (*t).premerge = pm;
                    (*t).apm = apm;
                    rc = 0 as int64_t;
                }
            }
        }
    }
    if !cand.is_null() {
        toks_plat_free(cand as *mut ::core::ffi::c_void, cb);
    }
    if !rel.is_null() {
        toks_plat_free(rel as *mut ::core::ffi::c_void, rb);
    }
    if !off.is_null() {
        toks_plat_free(off as *mut ::core::ffi::c_void, ob);
    }
    if !ent.is_null() {
        toks_plat_free(ent as *mut ::core::ffi::c_void, eb);
    }
    return rc;
}
#[no_mangle]
pub unsafe extern "C" fn toks_merge_slots(
    mut t: *mut toks_tables,
    mut slots: *mut uint64_t,
    mut pf: *mut uint64_t,
    mut nb: uint64_t,
    mut ml: *const uint32_t,
    mut mr: *const uint32_t,
    mut n: uint32_t,
    mut reach: *const uint32_t,
) -> uint64_t {
    let mut lg: uint64_t = 0 as uint64_t;
    let mut out: uint64_t = 0 as uint64_t;
    let mut w: uint64_t = 0 as uint64_t;
    while (1 as uint64_t) << lg < nb {
        lg = lg.wrapping_add(1);
    }
    memset(
        slots as *mut ::core::ffi::c_void,
        0xff as ::core::ffi::c_int,
        (nb as size_t).wrapping_mul(64 as size_t),
    );
    memset(
        pf as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        (nb as size_t).wrapping_mul(4 as size_t),
    );
    (*t).merge_slots = slots;
    (*t).pairf = pf;
    (*t).merge_mask = nb.wrapping_sub(1 as uint64_t);
    (*t).merge_shift = (64 as uint64_t).wrapping_sub(lg);
    (*t).merge_maxprobe = 1 as uint64_t;
    (*t).n_merges = n;
    let mut i: uint32_t = 0 as uint32_t;
    while i < n {
        if !reach.is_null()
            && *reach
                .offset((*ml.offset(i as isize) >> 5 as ::core::ffi::c_int) as isize)
                >> (*ml.offset(i as isize) & 31 as uint32_t)
                & *reach
                    .offset((*mr.offset(i as isize) >> 5 as ::core::ffi::c_int) as isize)
                    >> (*mr.offset(i as isize) & 31 as uint32_t) & 1 as uint32_t == 0
        {
            out = out.wrapping_add(1);
        } else {
            let mut key: uint64_t = bpe_pair_key(
                *ml.offset(i as isize),
                *mr.offset(i as isize),
            );
            let mut home: uint64_t = ((key as ::core::ffi::c_ulonglong)
                .wrapping_mul(TOKS_FIB64) >> (*t).merge_shift) as uint64_t;
            let mut d: uint64_t = 0 as uint64_t;
            let mut slot: *mut uint64_t = ::core::ptr::null_mut::<uint64_t>();
            while d < nb && slot.is_null() {
                let mut bucket: *mut uint64_t = slots
                    .offset(
                        (home.wrapping_add(d) & (*t).merge_mask)
                            .wrapping_mul(8 as uint64_t) as isize,
                    );
                let mut j: uint32_t = 0 as uint32_t;
                while j < 8 as uint32_t && slot.is_null() {
                    if *bucket.offset(j as isize) == BPE_EMPTY_SLOT as uint64_t
                        || *bucket.offset(j as isize) >> TOKS_PRIO_BITS == key
                    {
                        slot = bucket.offset(j as isize);
                    }
                    j = j.wrapping_add(1);
                }
                d = d.wrapping_add(1);
            }
            *slot = key << TOKS_PRIO_BITS | i as uint64_t;
            let mut m: uint64_t = bpe_pf_bits(key, (*t).merge_shift, &raw mut w);
            *pf.offset(w as isize) |= m;
            (*t).merge_maxprobe = if d > (*t).merge_maxprobe {
                d
            } else {
                (*t).merge_maxprobe
            };
        }
        i = i.wrapping_add(1);
    }
    return out;
}
unsafe extern "C" fn words_seat(
    mut t: *const toks_tables,
    mut words: *mut uint8_t,
    mut mask: uint64_t,
    mut k: bpe_key,
    mut id: uint32_t,
    mut work: *mut uint8_t,
    mut wbytes: uint64_t,
) -> ::core::ffi::c_int {
    let mut val: [uint32_t; 4] = [0; 4];
    let mut h: uint32_t = bpe_key_hash(k);
    bpe_val_pack_tag(
        &raw mut val as *mut uint32_t,
        &raw mut id,
        1 as uint32_t,
        0 as uint64_t,
    );
    if bpe_words_put(words, mask, h, k, &raw mut val as *mut uint32_t as *const uint32_t)
        != 0
    {
        return 1 as ::core::ffi::c_int;
    }
    let mut pass: uint32_t = 0 as uint32_t;
    while pass < 2 as uint32_t {
        let mut current_block_15: u64;
        let mut i: uint32_t = 0 as uint32_t;
        while i < 4 as uint32_t {
            let mut e: *mut uint8_t = words
                .offset(
                    ((if i >> 1 as ::core::ffi::c_int == 0 as uint32_t {
                        h
                    } else {
                        h >> 16 as ::core::ffi::c_int | h << 16 as ::core::ffi::c_int
                    }) as uint64_t & mask)
                        .wrapping_mul(TOKS_BUCKET as uint64_t) as isize,
                )
                .offset((16 as uint32_t).wrapping_mul(i & 1 as uint32_t) as isize);
            let mut ok: bpe_key = bpe_key { lo: 0, hi: 0 };
            let mut ov: [uint32_t; 4] = [0; 4];
            let mut got: [uint32_t; 15] = [0; 15];
            memcpy(
                &raw mut ok.lo as *mut ::core::ffi::c_void,
                e as *const ::core::ffi::c_void,
                8 as size_t,
            );
            memcpy(
                &raw mut ok.hi as *mut ::core::ffi::c_void,
                e.offset(8 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_void,
                8 as size_t,
            );
            memcpy(
                &raw mut ov as *mut uint32_t as *mut ::core::ffi::c_void,
                e.offset(32 as ::core::ffi::c_int as isize)
                    as *const ::core::ffi::c_void,
                16 as size_t,
            );
            if pass == 1 as uint32_t {
                let mut k6: toks_k6_args = toks_k6_args {
                    piece: e,
                    len: *e.offset(15 as ::core::ffi::c_int as isize) as uint64_t,
                    out: &raw mut got as *mut uint32_t,
                    work: work,
                    work_bytes: wbytes,
                    n_out: 0 as uint64_t,
                    merges: 0 as uint64_t,
                    rsv: 0 as uint64_t,
                };
                if bpe_val_count(&raw mut ov as *mut uint32_t) != 1 as uint32_t
                    || toks_k6_bpe_c(t, &raw mut k6) != 1 as uint64_t
                    || got[0 as ::core::ffi::c_int as usize]
                        != ov[0 as ::core::ffi::c_int as usize]
                            & TOKS_ID_MASK as uint32_t
                {
                    current_block_15 = 7095457783677275021;
                } else {
                    current_block_15 = 5399440093318478209;
                }
            } else {
                current_block_15 = 5399440093318478209;
            }
            match current_block_15 {
                5399440093318478209 => {
                    memcpy(
                        e as *mut ::core::ffi::c_void,
                        &raw mut k.lo as *const ::core::ffi::c_void,
                        8 as size_t,
                    );
                    memcpy(
                        e.offset(8 as ::core::ffi::c_int as isize)
                            as *mut ::core::ffi::c_void,
                        &raw mut k.hi as *const ::core::ffi::c_void,
                        8 as size_t,
                    );
                    memcpy(
                        e.offset(32 as ::core::ffi::c_int as isize)
                            as *mut ::core::ffi::c_void,
                        &raw mut val as *mut uint32_t as *const ::core::ffi::c_void,
                        16 as size_t,
                    );
                    if bpe_words_put(
                        words,
                        mask,
                        bpe_key_hash(ok),
                        ok,
                        &raw mut ov as *mut uint32_t as *const uint32_t,
                    ) != 0 || pass == 1 as uint32_t
                    {
                        return 1 as ::core::ffi::c_int;
                    }
                    memcpy(
                        e as *mut ::core::ffi::c_void,
                        &raw mut ok.lo as *const ::core::ffi::c_void,
                        8 as size_t,
                    );
                    memcpy(
                        e.offset(8 as ::core::ffi::c_int as isize)
                            as *mut ::core::ffi::c_void,
                        &raw mut ok.hi as *const ::core::ffi::c_void,
                        8 as size_t,
                    );
                    memcpy(
                        e.offset(32 as ::core::ffi::c_int as isize)
                            as *mut ::core::ffi::c_void,
                        &raw mut ov as *mut uint32_t as *const ::core::ffi::c_void,
                        16 as size_t,
                    );
                }
                _ => {}
            }
            i = i.wrapping_add(1);
        }
        pass = pass.wrapping_add(1);
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn words_room(
    mut words: *const uint8_t,
    mut mask: uint64_t,
    mut h: uint32_t,
) -> ::core::ffi::c_int {
    let mut b0: *const uint8_t = words
        .offset((h as uint64_t & mask).wrapping_mul(TOKS_BUCKET as uint64_t) as isize);
    let mut b1: *const uint8_t = words
        .offset(
            ((h >> 16 as ::core::ffi::c_int | h << 16 as ::core::ffi::c_int) as uint64_t
                & mask)
                .wrapping_mul(TOKS_BUCKET as uint64_t) as isize,
        );
    return (*b0.offset(15 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint
        == 0 as ::core::ffi::c_uint
        || *b0.offset(31 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint
            == 0 as ::core::ffi::c_uint
        || *b1.offset(15 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint
            == 0 as ::core::ffi::c_uint
        || *b1.offset(31 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint
            == 0 as ::core::ffi::c_uint) as ::core::ffi::c_int;
}
unsafe extern "C" fn model_token(
    mut cfg: *const toks_config,
    mut id: uint32_t,
) -> ::core::ffi::c_int {
    return ((*cfg).n_vocab_raw == 0 as uint32_t
        || toks_alpha_bytes(
            *(*cfg).vocab.offset(id as isize),
            *(*cfg).vocab_len.offset(id as isize),
            ::core::ptr::null_mut::<uint8_t>(),
        ) >= 0 as int64_t) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn toks_bpe_build(
    mut t: *mut toks_tables,
    mut ar: *mut toks_arena,
    mut cfg: *const toks_config,
) -> int64_t {
    let mut nv: uint32_t = (*cfg).n_vocab;
    let mut nm: uint32_t = (*cfg).n_merges;
    let mut ml: *const uint32_t = (*cfg).m_left_id;
    let mut mr: *const uint32_t = (*cfg).m_right_id;
    let mut mo: *const uint32_t = (*cfg).m_out_id;
    if nv == 0 as uint32_t || nv > (*t).n_ids {
        return TOKS_E_FORMAT as int64_t;
    }
    if nv > TOKS_MAX_IDS as uint32_t
        || nm as uint64_t > (BPE_PRIO_MASK as uint64_t).wrapping_add(1 as uint64_t)
    {
        return TOKS_E_UNSUPPORTED as int64_t;
    }
    (*t).flags
        &= !((TOKS_TF_IGNORE_MERGES | TOKS_TF_IDS_AS_RANK | TOKS_TF_PROBE_LONG
            | TOKS_TF_PROBE_ASCII) as uint32_t);
    if (*cfg).ignore_merges as ::core::ffi::c_uint != 0 as ::core::ffi::c_uint {
        (*t).flags = ((*t).flags as ::core::ffi::c_uint | TOKS_TF_IGNORE_MERGES)
            as uint32_t;
    }
    let mut byte2id: *mut uint32_t = toks_tab_ar(
        ar,
        (256 as ::core::ffi::c_uint).wrapping_mul(4 as ::core::ffi::c_uint) as uint64_t,
        64 as uint64_t,
        TOKS_X_BYTE2ID,
    ) as *mut uint32_t;
    if byte2id.is_null() {
        return TOKS_E_NOMEM as int64_t;
    }
    memset(
        byte2id as *mut ::core::ffi::c_void,
        0xff as ::core::ffi::c_int,
        (256 as ::core::ffi::c_uint).wrapping_mul(4 as ::core::ffi::c_uint) as size_t,
    );
    let mut id: uint32_t = 0 as uint32_t;
    while id < nv {
        if (*(*t).tok_off.offset(id.wrapping_add(1 as uint32_t) as isize))
            .wrapping_sub(*(*t).tok_off.offset(id as isize)) == 1 as uint32_t
            && model_token(cfg, id) != 0
        {
            *byte2id
                .offset(
                    *(*t).tok_bytes.offset(*(*t).tok_off.offset(id as isize) as isize)
                        as isize,
                ) = id;
        }
        id = id.wrapping_add(1);
    }
    let mut b: uint32_t = 0 as uint32_t;
    while b < 256 as uint32_t {
        if *byte2id.offset(b as isize) == UINT32_MAX as uint32_t
            && ((*cfg).drop[(b >> 3 as ::core::ffi::c_int) as usize]
                as ::core::ffi::c_int >> (b & 7 as uint32_t)) as ::core::ffi::c_uint
                & 1 as ::core::ffi::c_uint == 0 as ::core::ffi::c_uint
        {
            return TOKS_E_FORMAT as int64_t;
        }
        b = b.wrapping_add(1);
    }
    (*t).byte2id = byte2id;
    let mut i: uint32_t = 0 as uint32_t;
    while i < nm {
        if *ml.offset(i as isize) >= nv || *mr.offset(i as isize) >= nv
            || *mo.offset(i as isize) >= nv
        {
            return TOKS_E_FORMAT as int64_t;
        }
        i = i.wrapping_add(1);
    }
    let mut nb: uint64_t = merge_buckets(nm as uint64_t);
    let mut slots: *mut uint64_t = toks_tab_ar(
        ar,
        nb.wrapping_mul(64 as uint64_t),
        64 as uint64_t,
        TOKS_X_MERGE_SLOTS,
    ) as *mut uint64_t;
    let mut pf: *mut uint64_t = toks_tab_ar(
        ar,
        nb.wrapping_mul(4 as uint64_t),
        64 as uint64_t,
        TOKS_X_PAIRF,
    ) as *mut uint64_t;
    if slots.is_null() || pf.is_null() {
        return TOKS_E_NOMEM as int64_t;
    }
    toks_merge_slots(t, slots, pf, nb, ml, mr, nm, ::core::ptr::null::<uint32_t>());
    let mut mt: bpe_mt = bpe_mt_of(t);
    let mut as_rank: uint32_t = 1 as uint32_t;
    let mut prev: uint32_t = 0 as uint32_t;
    let mut have: uint32_t = 0 as uint32_t;
    let mut i_0: uint32_t = 0 as uint32_t;
    while i_0 < nm && as_rank != 0 as uint32_t
        && (*cfg).ids_as_rank as ::core::ffi::c_uint == 0 as ::core::ffi::c_uint
    {
        if !(bpe_mt_find(
            &raw mut mt,
            bpe_pair_key(*ml.offset(i_0 as isize), *mr.offset(i_0 as isize)),
        ) != i_0)
        {
            if have != 0 as uint32_t && *mo.offset(i_0 as isize) <= prev {
                as_rank = 0 as ::core::ffi::c_uint as uint32_t;
            }
            prev = *mo.offset(i_0 as isize);
            have = 1 as ::core::ffi::c_uint as uint32_t;
        }
        i_0 = i_0.wrapping_add(1);
    }
    if as_rank != 0 as uint32_t {
        let mut s: uint64_t = 0 as uint64_t;
        while s < nb.wrapping_mul(8 as uint64_t) {
            if *slots.offset(s as isize) != BPE_EMPTY_SLOT as uint64_t {
                *slots.offset(s as isize) = *slots.offset(s as isize)
                    & !(BPE_PRIO_MASK as uint64_t)
                    | *mo
                        .offset(
                            (*slots.offset(s as isize) & BPE_PRIO_MASK as uint64_t)
                                as isize,
                        ) as uint64_t;
            }
            s = s.wrapping_add(1);
        }
        (*t).flags = ((*t).flags as ::core::ffi::c_uint | TOKS_TF_IDS_AS_RANK)
            as uint32_t;
        (*t).rank2id = ::core::ptr::null::<uint32_t>();
    } else {
        let mut r2i: *mut uint32_t = toks_tab_ar(
            ar,
            (4 as uint64_t).wrapping_mul(nm as uint64_t),
            64 as uint64_t,
            TOKS_X_RANK2ID,
        ) as *mut uint32_t;
        if r2i.is_null() {
            return TOKS_E_NOMEM as int64_t;
        }
        memcpy(
            r2i as *mut ::core::ffi::c_void,
            mo as *const ::core::ffi::c_void,
            (4 as size_t).wrapping_mul(nm as size_t),
        );
        (*t).rank2id = r2i;
    }
    let mut bytepair: *mut uint32_t = toks_tab_ar(
        ar,
        (65536 as ::core::ffi::c_uint).wrapping_mul(4 as ::core::ffi::c_uint)
            as uint64_t,
        64 as uint64_t,
        TOKS_X_BYTEPAIR,
    ) as *mut uint32_t;
    if bytepair.is_null() {
        return TOKS_E_NOMEM as int64_t;
    }
    let mut bp: uint32_t = 0 as uint32_t;
    while bp < 65536 as uint32_t {
        *bytepair.offset(bp as isize) = bpe_mt_find(
            &raw mut mt,
            bpe_pair_key(
                *byte2id.offset((bp >> 8 as ::core::ffi::c_int) as isize),
                *byte2id.offset((bp & 0xff as uint32_t) as isize),
            ),
        );
        bp = bp.wrapping_add(1);
    }
    (*t).bytepair = bytepair;
    let mut n_raw: uint32_t = 0 as uint32_t;
    let mut n_keys: uint32_t = 0 as uint32_t;
    let mut n_in: uint32_t = 0 as uint32_t;
    let mut lmin: uint32_t = if (*cfg).ignore_merges as ::core::ffi::c_uint
        != 0 as ::core::ffi::c_uint
    {
        1 as uint32_t
    } else {
        (TOKS_KEY_MAXLEN as uint32_t).wrapping_add(1 as uint32_t)
    };
    let mut ascii: uint32_t = 1 as uint32_t;
    let mut id_0: uint32_t = 0 as uint32_t;
    while id_0 < nv {
        let mut l: uint32_t = if model_token(cfg, id_0) != 0 {
            (*(*t).tok_off.offset(id_0.wrapping_add(1 as uint32_t) as isize))
                .wrapping_sub(*(*t).tok_off.offset(id_0 as isize))
        } else {
            0 as uint32_t
        };
        n_raw = (n_raw as ::core::ffi::c_uint)
            .wrapping_add(
                if l > 0 as uint32_t {
                    1 as ::core::ffi::c_uint
                } else {
                    0 as ::core::ffi::c_uint
                },
            ) as uint32_t as uint32_t;
        n_keys = (n_keys as ::core::ffi::c_uint)
            .wrapping_add(
                if l >= 2 as uint32_t && l <= TOKS_KEY_MAXLEN as uint32_t {
                    1 as ::core::ffi::c_uint
                } else {
                    0 as ::core::ffi::c_uint
                },
            ) as uint32_t as uint32_t;
        id_0 = id_0.wrapping_add(1);
    }
    (*t).vhash = ::core::ptr::null::<uint64_t>();
    (*t).vhash_mask = 0 as uint64_t;
    if n_raw > 0 as uint32_t {
        let mut vs: uint64_t = vhash_slots(n_raw as uint64_t);
        let mut vh: *mut uint64_t = toks_tab_ar(
            ar,
            vs.wrapping_mul(8 as uint64_t).wrapping_add(1024 as uint64_t),
            64 as uint64_t,
            TOKS_X_VHASH,
        ) as *mut uint64_t;
        if vh.is_null() {
            return TOKS_E_NOMEM as int64_t;
        }
        memset(
            vh as *mut ::core::ffi::c_void,
            0xff as ::core::ffi::c_int,
            (vs as size_t).wrapping_mul(8 as size_t),
        );
        let mut ml_0: *mut uint32_t = vh.offset(vs as isize) as *mut ::core::ffi::c_void
            as *mut uint32_t;
        memset(
            ml_0 as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            1024 as size_t,
        );
        let mut cw: crate::Aligned<[uint8_t; 8448]> = crate::Aligned([0; 8448]);
        let mut cids: [uint32_t; 260] = [0; 260];
        (*t).premerge = ::core::ptr::null::<uint8_t>();
        (*t).apm = ::core::ptr::null::<uint8_t>();
        let mut current_block_86: u64;
        let mut id_1: uint32_t = 0 as uint32_t;
        while id_1 < nv {
            let mut o: uint32_t = *(*t).tok_off.offset(id_1 as isize);
            let mut l_0: uint32_t = (*(*t)
                .tok_off
                .offset(id_1.wrapping_add(1 as uint32_t) as isize))
                .wrapping_sub(o);
            if !(l_0 < lmin || model_token(cfg, id_1) == 0) {
                if (*cfg).ignore_merges as ::core::ffi::c_uint
                    == 0 as ::core::ffi::c_uint
                {
                    let mut hi: uint32_t = 0 as uint32_t;
                    let mut i_1: uint32_t = 0 as uint32_t;
                    while i_1 < l_0 {
                        hi
                            |= *(*t).tok_bytes.offset(o.wrapping_add(i_1) as isize)
                                as uint32_t;
                        i_1 = i_1.wrapping_add(1);
                    }
                    if hi >= 0x80 as uint32_t {
                        current_block_86 = 7178192492338286402;
                    } else {
                        let mut k6: toks_k6_args = toks_k6_args {
                            piece: (*t).tok_bytes.offset(o as isize),
                            len: l_0 as uint64_t,
                            out: &raw mut cids as *mut uint32_t,
                            work: &raw mut cw as *mut uint8_t,
                            work_bytes: ::core::mem::size_of::<[uint8_t; 8448]>()
                                as uint64_t,
                            n_out: 0 as uint64_t,
                            merges: 0 as uint64_t,
                            rsv: 0 as uint64_t,
                        };
                        if l_0 > 256 as uint32_t
                            || toks_k6_bpe_c(t, &raw mut k6) != 1 as uint64_t
                            || cids[0 as ::core::ffi::c_int as usize] != id_1
                        {
                            current_block_86 = 7178192492338286402;
                        } else {
                            current_block_86 = 15587532755333643506;
                        }
                    }
                } else {
                    if l_0 > TOKS_KEY_MAXLEN as uint32_t
                        && *(*t)
                            .tok_bytes
                            .offset(
                                o.wrapping_add(l_0).wrapping_sub(1 as uint32_t) as isize,
                            ) as ::core::ffi::c_uint >= 0x80 as ::core::ffi::c_uint
                        && ascii != 0 as uint32_t
                    {
                        let mut k6_0: toks_k6_args = toks_k6_args {
                            piece: (*t).tok_bytes.offset(o as isize),
                            len: l_0 as uint64_t,
                            out: &raw mut cids as *mut uint32_t,
                            work: &raw mut cw as *mut uint8_t,
                            work_bytes: ::core::mem::size_of::<[uint8_t; 8448]>()
                                as uint64_t,
                            n_out: 0 as uint64_t,
                            merges: 0 as uint64_t,
                            rsv: 0 as uint64_t,
                        };
                        ascii = (l_0 <= 256 as uint32_t
                            && toks_k6_bpe_c(t, &raw mut k6_0) == 1 as uint64_t
                            && cids[0 as ::core::ffi::c_int as usize] == id_1)
                            as ::core::ffi::c_int as uint32_t;
                    }
                    current_block_86 = 15587532755333643506;
                }
                match current_block_86 {
                    7178192492338286402 => {}
                    _ => {
                        let mut b0: uint8_t = *(*t).tok_bytes.offset(o as isize);
                        *ml_0.offset(b0 as isize) = if *ml_0.offset(b0 as isize) > l_0 {
                            *ml_0.offset(b0 as isize)
                        } else {
                            l_0
                        };
                        let mut h: uint32_t = bpe_vhash_h(
                            (*t).tok_bytes.offset(o as isize),
                            l_0 as uint64_t,
                        );
                        let mut s_0: uint64_t = h as uint64_t
                            & vs.wrapping_sub(1 as uint64_t);
                        while *vh.offset(s_0 as isize) != BPE_EMPTY_SLOT as uint64_t {
                            s_0 = s_0.wrapping_add(1 as uint64_t)
                                & vs.wrapping_sub(1 as uint64_t);
                        }
                        *vh.offset(s_0 as isize) = (id_1 as uint64_t)
                            << 32 as ::core::ffi::c_int | h as uint64_t;
                        n_in = (n_in as ::core::ffi::c_uint)
                            .wrapping_add(1 as ::core::ffi::c_uint) as uint32_t
                            as uint32_t;
                    }
                }
            }
            id_1 = id_1.wrapping_add(1);
        }
        (*t).vhash = vh;
        (*t).vhash_mask = vs.wrapping_sub(1 as uint64_t);
        if (*cfg).ignore_merges as ::core::ffi::c_uint == 0 as ::core::ffi::c_uint
            && n_in > 0 as uint32_t
        {
            (*t).flags = ((*t).flags as ::core::ffi::c_uint
                | (TOKS_TF_IGNORE_MERGES | TOKS_TF_PROBE_LONG | TOKS_TF_PROBE_ASCII))
                as uint32_t;
        }
    }
    let mut prc: int64_t = premerge_build(t, ar, cfg);
    if prc != 0 as int64_t {
        return prc;
    }
    (*t).words = ::core::ptr::null::<uint8_t>();
    (*t).words_mask = 0 as uint64_t;
    if n_keys > 0 as uint32_t {
        let mut wb: uint64_t = words_buckets(n_keys as uint64_t);
        let mut words: *mut uint8_t = toks_tab_ar(
            ar,
            wb.wrapping_mul(TOKS_BUCKET as uint64_t),
            64 as uint64_t,
            TOKS_X_WORDS,
        ) as *mut uint8_t;
        if words.is_null() {
            return TOKS_E_NOMEM as int64_t;
        }
        memset(
            words as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            (wb as size_t).wrapping_mul(TOKS_BUCKET as size_t),
        );
        (*t).words = words;
        (*t).words_mask = wb.wrapping_sub(1 as uint64_t);
        let mut work: crate::Aligned<[uint8_t; 736]> = crate::Aligned([0; 736]);
        let mut ids: [uint32_t; 15] = [0; 15];
        let mut placed: uint64_t = 0 as uint64_t;
        let mut unseated: uint32_t = 0 as uint32_t;
        let mut id_2: uint32_t = 0 as uint32_t;
        while id_2 < nv {
            let mut o_0: uint32_t = *(*t).tok_off.offset(id_2 as isize);
            let mut l_1: uint32_t = (*(*t)
                .tok_off
                .offset(id_2.wrapping_add(1 as uint32_t) as isize))
                .wrapping_sub(o_0);
            if !(l_1 < 2 as uint32_t || l_1 > TOKS_KEY_MAXLEN as uint32_t
                || model_token(cfg, id_2) == 0)
            {
                let mut k: bpe_key = bpe_key_at(
                    (*t).tok_bytes.offset(o_0 as isize),
                    l_1 as uint64_t,
                    0 as uint64_t,
                    l_1 as uint64_t,
                );
                let mut h_0: uint32_t = bpe_key_hash(k);
                let mut k6_1: toks_k6_args = toks_k6_args {
                    piece: (*t).tok_bytes.offset(o_0 as isize),
                    len: l_1 as uint64_t,
                    out: &raw mut ids as *mut uint32_t,
                    work: &raw mut work as *mut uint8_t,
                    work_bytes: ::core::mem::size_of::<[uint8_t; 736]>() as uint64_t,
                    n_out: 0 as uint64_t,
                    merges: 0 as uint64_t,
                    rsv: 0 as uint64_t,
                };
                if words_room(words, wb.wrapping_sub(1 as uint64_t), h_0) != 0 {
                    let mut n: uint64_t = toks_k6_bpe_c(t, &raw mut k6_1);
                    if !(n > 4 as uint64_t) {
                        let mut val: [uint32_t; 4] = [0; 4];
                        bpe_val_pack_tag(
                            &raw mut val as *mut uint32_t,
                            &raw mut ids as *mut uint32_t,
                            n as uint32_t,
                            0 as uint64_t,
                        );
                        placed = placed
                            .wrapping_add(
                                bpe_words_put(
                                    words,
                                    wb.wrapping_sub(1 as uint64_t),
                                    h_0,
                                    k,
                                    &raw mut val as *mut uint32_t as *const uint32_t,
                                ) as uint64_t,
                            );
                    }
                } else if !((*cfg).ignore_merges as ::core::ffi::c_uint
                    == 0 as ::core::ffi::c_uint)
                {
                    (*t).flags &= !(TOKS_TF_IGNORE_MERGES as uint32_t);
                    if toks_k6_bpe_c(t, &raw mut k6_1) != 1 as uint64_t
                        || ids[0 as ::core::ffi::c_int as usize] != id_2
                    {
                        unseated = (unseated as ::core::ffi::c_uint)
                            .wrapping_add(
                                if words_seat(
                                    t,
                                    words,
                                    wb.wrapping_sub(1 as uint64_t),
                                    k,
                                    id_2,
                                    &raw mut work as *mut uint8_t,
                                    ::core::mem::size_of::<[uint8_t; 736]>() as uint64_t,
                                ) != 0 as ::core::ffi::c_int
                                {
                                    0 as ::core::ffi::c_uint
                                } else {
                                    1 as ::core::ffi::c_uint
                                },
                            ) as uint32_t as uint32_t;
                    }
                    (*t).flags = ((*t).flags as ::core::ffi::c_uint
                        | TOKS_TF_IGNORE_MERGES) as uint32_t;
                }
            }
            id_2 = id_2.wrapping_add(1);
        }
        let mut d: *const uint8_t = &raw const toks_dict as *const uint8_t;
        let mut i_2: uint32_t = 0 as uint32_t;
        while i_2 < toks_dict_n && placed < (2 as uint64_t).wrapping_mul(wb) {
            let mut l_2: uint32_t = *d.offset(0 as ::core::ffi::c_int as isize)
                as uint32_t;
            let mut h_1: uint32_t = 0;
            let mut val_0: [uint32_t; 4] = [0; 4];
            let mut ok: uint32_t = 1 as uint32_t;
            let mut j: uint32_t = 0 as uint32_t;
            while j < l_2 {
                ok = (ok as ::core::ffi::c_uint
                    & if *byte2id
                        .offset(
                            *d.offset((1 as uint32_t).wrapping_add(j) as isize) as isize,
                        ) != UINT32_MAX as uint32_t
                    {
                        1 as ::core::ffi::c_uint
                    } else {
                        0 as ::core::ffi::c_uint
                    }) as uint32_t;
                j = j.wrapping_add(1);
            }
            let mut k_0: bpe_key = bpe_key_at(
                d.offset(1 as ::core::ffi::c_int as isize),
                l_2 as uint64_t,
                0 as uint64_t,
                l_2 as uint64_t,
            );
            h_1 = bpe_key_hash(k_0);
            if !(ok == 0 as uint32_t
                || !bpe_words_probe(words, wb.wrapping_sub(1 as uint64_t), h_1, k_0)
                    .is_null()
                || words_room(words, wb.wrapping_sub(1 as uint64_t), h_1) == 0)
            {
                let mut k6_2: toks_k6_args = toks_k6_args {
                    piece: d.offset(1 as ::core::ffi::c_int as isize),
                    len: l_2 as uint64_t,
                    out: &raw mut ids as *mut uint32_t,
                    work: &raw mut work as *mut uint8_t,
                    work_bytes: ::core::mem::size_of::<[uint8_t; 736]>() as uint64_t,
                    n_out: 0 as uint64_t,
                    merges: 0 as uint64_t,
                    rsv: 0 as uint64_t,
                };
                let mut n_0: uint64_t = toks_k6_bpe_c(t, &raw mut k6_2);
                if !(n_0 > 4 as uint64_t) {
                    bpe_val_pack_tag(
                        &raw mut val_0 as *mut uint32_t,
                        &raw mut ids as *mut uint32_t,
                        n_0 as uint32_t,
                        0 as uint64_t,
                    );
                    placed = placed
                        .wrapping_add(
                            bpe_words_put(
                                words,
                                wb.wrapping_sub(1 as uint64_t),
                                h_1,
                                k_0,
                                &raw mut val_0 as *mut uint32_t as *const uint32_t,
                            ) as uint64_t,
                        );
                }
            }
            i_2 = i_2.wrapping_add(1);
            d = d
                .offset(
                    (1 as ::core::ffi::c_uint)
                        .wrapping_add(
                            *d.offset(0 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint,
                        ) as isize,
                );
        }
        if (*cfg).ignore_merges as ::core::ffi::c_uint != 0 as ::core::ffi::c_uint
            && unseated == 0 as uint32_t
        {
            (*t).flags = ((*t).flags as ::core::ffi::c_uint
                | (TOKS_TF_PROBE_LONG
                    | (if ascii != 0 as uint32_t {
                        TOKS_TF_PROBE_ASCII
                    } else {
                        0 as ::core::ffi::c_uint
                    }))) as uint32_t;
        }
    }
    return 0 as int64_t;
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
