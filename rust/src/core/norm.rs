extern "C" {
    fn memcpy(
        dst: *mut ::core::ffi::c_void,
        src: *const ::core::ffi::c_void,
        n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memcmp(
        a: *const ::core::ffi::c_void,
        b: *const ::core::ffi::c_void,
        n: size_t,
    ) -> ::core::ffi::c_int;
    static toks_bert_cls_s1: [uint16_t; 4352];
    static toks_bert_cls_s2: [uint8_t; 25856];
    static toks_bert_map_s1: [uint16_t; 4352];
    static toks_bert_map_s2: [uint16_t; 12800];
    static toks_bert_maps: [toks_bert_map; 3169];
    static toks_bert_pool: [uint32_t; 4893];
    static toks_nfc_stage1: [uint8_t; 4352];
    static toks_nfc_stage2: [uint32_t; 22784];
    static toks_nfc_pool: [uint32_t; 6619];
    static toks_nfc_comp: [uint64_t; 2048];
    static toks_nfc_kstage1: [uint8_t; 4352];
    static toks_nfc_kstage2: [uint32_t; 10496];
    static toks_nfc_fast: [uint64_t; 2048];
}
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
pub type size_t = usize;
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
pub struct toks_bert_map {
    pub dec_off: uint16_t,
    pub dec_len: uint8_t,
    pub rsv0: uint8_t,
    pub low_off: uint16_t,
    pub low_len: uint8_t,
    pub rsv1: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cur {
    pub i: uint64_t,
    pub nb: uint64_t,
    pub n: uint32_t,
    pub j: uint32_t,
    pub f: uint32_t,
    pub e: [uint32_t; 20],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct toks_nfc_plan {
    pub r: uint64_t,
    pub d0: uint64_t,
    pub d1: uint64_t,
    pub f: uint32_t,
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
pub const TOKS_E_CAP: ::core::ffi::c_int = -(8 as ::core::ffi::c_int);
pub const TOKS_C_BASE_MASK: ::core::ffi::c_uint = 0x7 as ::core::ffi::c_uint;
pub const TOKS_C_P: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
pub const TOKS_C_L: ::core::ffi::c_uint = 1 as ::core::ffi::c_uint;
pub const TOKS_C_N: ::core::ffi::c_uint = 2 as ::core::ffi::c_uint;
pub const TOKS_C_WS: ::core::ffi::c_uint = 3 as ::core::ffi::c_uint;
pub const TOKS_C_NL: ::core::ffi::c_uint = 4 as ::core::ffi::c_uint;
pub const TOKS_TMPL_CL100K: ::core::ffi::c_uint = 1 as ::core::ffi::c_uint;
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
pub const TOKS_NFC_X: ::core::ffi::c_uint = 3 as ::core::ffi::c_uint;
pub const TOKS_NFKC_X: ::core::ffi::c_uint = 11 as ::core::ffi::c_uint;
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
unsafe extern "C" fn toks_utf8_put(mut o: *mut uint8_t, mut cp: uint32_t) -> uint32_t {
    if cp < 0x80 as uint32_t {
        *o.offset(0 as ::core::ffi::c_int as isize) = cp as uint8_t;
        return 1 as uint32_t;
    }
    if cp < 0x800 as uint32_t {
        *o.offset(0 as ::core::ffi::c_int as isize) = (0xc0 as uint32_t
            | cp >> 6 as ::core::ffi::c_int) as uint8_t;
        *o.offset(1 as ::core::ffi::c_int as isize) = (0x80 as uint32_t
            | cp & 0x3f as uint32_t) as uint8_t;
        return 2 as uint32_t;
    }
    if cp < 0x10000 as uint32_t {
        *o.offset(0 as ::core::ffi::c_int as isize) = (0xe0 as uint32_t
            | cp >> 12 as ::core::ffi::c_int) as uint8_t;
        *o.offset(1 as ::core::ffi::c_int as isize) = (0x80 as uint32_t
            | cp >> 6 as ::core::ffi::c_int & 0x3f as uint32_t) as uint8_t;
        *o.offset(2 as ::core::ffi::c_int as isize) = (0x80 as uint32_t
            | cp & 0x3f as uint32_t) as uint8_t;
        return 3 as uint32_t;
    }
    *o.offset(0 as ::core::ffi::c_int as isize) = (0xf0 as uint32_t
        | cp >> 18 as ::core::ffi::c_int) as uint8_t;
    *o.offset(1 as ::core::ffi::c_int as isize) = (0x80 as uint32_t
        | cp >> 12 as ::core::ffi::c_int & 0x3f as uint32_t) as uint8_t;
    *o.offset(2 as ::core::ffi::c_int as isize) = (0x80 as uint32_t
        | cp >> 6 as ::core::ffi::c_int & 0x3f as uint32_t) as uint8_t;
    *o.offset(3 as ::core::ffi::c_int as isize) = (0x80 as uint32_t
        | cp & 0x3f as uint32_t) as uint8_t;
    return 4 as uint32_t;
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
pub const TOKS_NS_CLEAN: ::core::ffi::c_uint = 0x1 as ::core::ffi::c_uint;
pub const TOKS_NS_CJK: ::core::ffi::c_uint = 0x2 as ::core::ffi::c_uint;
pub const TOKS_NS_STRIP_MN: ::core::ffi::c_uint = 0x4 as ::core::ffi::c_uint;
pub const TOKS_NS_LOWER: ::core::ffi::c_uint = 0x8 as ::core::ffi::c_uint;
pub const TOKS_NS_CANON: ::core::ffi::c_uint = 0x10 as ::core::ffi::c_uint;
pub const TOKS_NS_COMPOSE: ::core::ffi::c_uint = 0x20 as ::core::ffi::c_uint;
pub const TOKS_NS_COMPAT: ::core::ffi::c_uint = 0x40 as ::core::ffi::c_uint;
pub const TOKS_NS_STRIP_M: ::core::ffi::c_uint = 0x80 as ::core::ffi::c_uint;
pub const TOKS_NS_NFC: ::core::ffi::c_uint = TOKS_NS_CANON | TOKS_NS_COMPOSE;
pub const TOKS_NORM_GHOST: ::core::ffi::c_uint = 0x200000 as ::core::ffi::c_uint;
pub const TOKS_NFC_SCAN_RUN: ::core::ffi::c_uint = 64 as ::core::ffi::c_uint;
pub const TOKS_BC_REMOVE: ::core::ffi::c_uint = 0x1 as ::core::ffi::c_uint;
pub const TOKS_BC_WS: ::core::ffi::c_uint = 0x2 as ::core::ffi::c_uint;
pub const TOKS_BC_CJK: ::core::ffi::c_uint = 0x4 as ::core::ffi::c_uint;
pub const TOKS_BC_LOWER: ::core::ffi::c_uint = 0x20 as ::core::ffi::c_uint;
pub const TOKS_BC_MN: ::core::ffi::c_uint = 0x80 as ::core::ffi::c_uint;
#[inline]
unsafe extern "C" fn toks_bert_cls(mut cp: uint32_t) -> uint8_t {
    if cp > 0x10ffff as uint32_t {
        return 0 as uint8_t;
    }
    return toks_bert_cls_s2[((toks_bert_cls_s1[(cp >> 8 as ::core::ffi::c_int) as usize]
        as uint32_t) << 8 as ::core::ffi::c_int | cp & 0xff as uint32_t) as usize];
}
#[inline]
unsafe extern "C" fn toks_bert_map_index(mut cp: uint32_t) -> uint16_t {
    if cp > 0x10ffff as uint32_t {
        return 0 as uint16_t;
    }
    return toks_bert_map_s2[((toks_bert_map_s1[(cp >> 8 as ::core::ffi::c_int) as usize]
        as uint32_t) << 8 as ::core::ffi::c_int | cp & 0xff as uint32_t) as usize];
}
pub const TOKS_NFC_DOFF_MASK: ::core::ffi::c_uint = 0x1fff as ::core::ffi::c_uint;
pub const TOKS_NFC_DLEN_SHIFT: ::core::ffi::c_int = 13 as ::core::ffi::c_int;
pub const TOKS_NFC_DLEN_MASK: ::core::ffi::c_uint = 0x7 as ::core::ffi::c_uint;
pub const TOKS_NFC_NB: ::core::ffi::c_uint = 0x10000 as ::core::ffi::c_uint;
pub const TOKS_NFC_NBK: ::core::ffi::c_uint = 0x20000 as ::core::ffi::c_uint;
pub const TOKS_NFC_KX: ::core::ffi::c_uint = 0x40000 as ::core::ffi::c_uint;
pub const TOKS_NFC_MARK: ::core::ffi::c_uint = 0x80000 as ::core::ffi::c_uint;
pub const TOKS_NFC_SECOND: ::core::ffi::c_uint = 0x200000 as ::core::ffi::c_uint;
pub const TOKS_NFC_CLS_SHIFT: ::core::ffi::c_int = 24 as ::core::ffi::c_int;
pub const TOKS_NFC_CLS_MASK: ::core::ffi::c_uint = 0x3f as ::core::ffi::c_uint;
pub const TOKS_NFC_CP_MASK: ::core::ffi::c_uint = 0x1fffff as ::core::ffi::c_uint;
pub const TOKS_NFC_HOT_BYTE: ::core::ffi::c_uint = 0xcc as ::core::ffi::c_uint;
pub const TOKS_NFKC_HOT_BYTE: ::core::ffi::c_uint = 0xc2 as ::core::ffi::c_uint;
pub const TOKS_NFC_MAX_DECOMP: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const TOKS_NFC_COMP_LOG2: ::core::ffi::c_int = 11 as ::core::ffi::c_int;
pub const TOKS_NFC_COMP_MAXPROBE: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const TOKS_NFC_FIB64: ::core::ffi::c_ulonglong = 0x9e3779b97f4a7c15
    as ::core::ffi::c_ulonglong;
#[inline]
unsafe extern "C" fn toks_nfc_info(mut cp: uint32_t) -> uint32_t {
    return toks_nfc_stage2[((toks_nfc_stage1[(cp >> 8 as ::core::ffi::c_int) as usize]
        as uint32_t) << 8 as ::core::ffi::c_int | cp & 0xff as uint32_t) as usize];
}
#[inline]
unsafe extern "C" fn toks_nfc_kinfo(mut cp: uint32_t) -> uint32_t {
    return toks_nfc_kstage2[((toks_nfc_kstage1[(cp >> 8 as ::core::ffi::c_int) as usize]
        as uint32_t) << 8 as ::core::ffi::c_int | cp & 0xff as uint32_t) as usize];
}
#[inline]
unsafe extern "C" fn hot_bytes(mut w: uint64_t, mut a: uint64_t) -> uint64_t {
    let mut lo6: uint64_t = (w as ::core::ffi::c_ulonglong
        & 0x3f3f3f3f3f3f3f3f as ::core::ffi::c_ulonglong)
        .wrapping_add(a as ::core::ffi::c_ulonglong) as uint64_t;
    return ((w & w << 1 as ::core::ffi::c_int & lo6 << 1 as ::core::ffi::c_int)
        as ::core::ffi::c_ulonglong & 0x8080808080808080 as ::core::ffi::c_ulonglong)
        as uint64_t;
}
#[inline]
unsafe extern "C" fn load64(mut p: *const uint8_t) -> uint64_t {
    let mut w: uint64_t = 0;
    memcpy(
        &raw mut w as *mut ::core::ffi::c_void,
        p as *const ::core::ffi::c_void,
        8 as size_t,
    );
    return w;
}
#[no_mangle]
pub unsafe extern "C" fn toks_k2(
    mut p: *const uint8_t,
    mut i: uint64_t,
    mut n: uint64_t,
    mut hot: uint8_t,
) -> uint64_t {
    let mut a: uint64_t = (0x101010101010101 as ::core::ffi::c_ulonglong)
        .wrapping_mul(
            (0x40 as ::core::ffi::c_uint)
                .wrapping_sub(hot as ::core::ffi::c_uint & 0x3f as ::core::ffi::c_uint)
                as ::core::ffi::c_ulonglong,
        ) as uint64_t;
    if n.wrapping_sub(i) >= 8 as uint64_t {
        let mut m: uint64_t = hot_bytes(load64(p.offset(i as isize)), a);
        if m != 0 as uint64_t {
            return i
                .wrapping_add(
                    (m as ::core::ffi::c_ulonglong).trailing_zeros() as i32 as uint64_t
                        >> 3 as ::core::ffi::c_int,
                );
        }
        i = i.wrapping_add(8 as uint64_t);
    }
    while n.wrapping_sub(i) >= 64 as uint64_t {
        let mut m_0: uint64_t = 0 as uint64_t;
        let mut k: uint32_t = 0 as uint32_t;
        while k < 8 as uint32_t {
            m_0
                |= hot_bytes(
                    load64(
                        p
                            .offset(i as isize)
                            .offset((8 as uint32_t).wrapping_mul(k) as isize),
                    ),
                    a,
                );
            k = k.wrapping_add(1);
        }
        if m_0 != 0 as uint64_t {
            break;
        }
        i = i.wrapping_add(64 as uint64_t);
    }
    while n.wrapping_sub(i) >= 8 as uint64_t {
        let mut m_1: uint64_t = hot_bytes(load64(p.offset(i as isize)), a);
        if m_1 != 0 as uint64_t {
            return i
                .wrapping_add(
                    (m_1 as ::core::ffi::c_ulonglong).trailing_zeros() as i32 as uint64_t
                        >> 3 as ::core::ffi::c_int,
                );
        }
        i = i.wrapping_add(8 as uint64_t);
    }
    while i < n
        && (*p.offset(i as isize) as ::core::ffi::c_int) < hot as ::core::ffi::c_int
    {
        i = i.wrapping_add(1);
    }
    return i;
}
pub const BAD: ::core::ffi::c_uint = 0x400000 as ::core::ffi::c_uint;
pub const DECOMP: ::core::ffi::c_uint = TOKS_NS_CANON | TOKS_NS_COMPAT
    | TOKS_NS_STRIP_MN;
pub const S_BASE: ::core::ffi::c_uint = 0xac00 as ::core::ffi::c_uint;
pub const L_BASE: ::core::ffi::c_uint = 0x1100 as ::core::ffi::c_uint;
pub const V_BASE: ::core::ffi::c_uint = 0x1161 as ::core::ffi::c_uint;
pub const T_BASE: ::core::ffi::c_uint = 0x11a7 as ::core::ffi::c_uint;
pub const L_COUNT: ::core::ffi::c_uint = 19 as ::core::ffi::c_uint;
pub const V_COUNT: ::core::ffi::c_uint = 21 as ::core::ffi::c_uint;
pub const T_COUNT: ::core::ffi::c_uint = 28 as ::core::ffi::c_uint;
pub const S_COUNT: ::core::ffi::c_uint = L_COUNT
    .wrapping_mul(V_COUNT)
    .wrapping_mul(T_COUNT);
unsafe extern "C" fn decomp(
    mut f: uint32_t,
    mut cp: uint32_t,
    mut e: *mut uint32_t,
) -> uint32_t {
    let mut s: uint32_t = cp.wrapping_sub(S_BASE as uint32_t);
    if f & DECOMP as uint32_t == 0 as uint32_t {
        *e.offset(0 as ::core::ffi::c_int as isize) = cp;
        return 1 as uint32_t;
    }
    let mut w: uint32_t = toks_nfc_info(cp);
    let mut dl: uint32_t = w >> TOKS_NFC_DLEN_SHIFT & TOKS_NFC_DLEN_MASK as uint32_t;
    let mut off: uint32_t = w & TOKS_NFC_DOFF_MASK as uint32_t;
    if f & TOKS_NS_COMPAT as uint32_t != 0 as uint32_t
        && w & TOKS_NFC_KX as uint32_t != 0 as uint32_t
    {
        dl = toks_nfc_kinfo(cp) >> 16 as ::core::ffi::c_int;
        off = toks_nfc_kinfo(cp) & 0xffff as uint32_t;
    }
    if f & TOKS_NS_COMPOSE as uint32_t == 0 as uint32_t && s < S_COUNT as uint32_t {
        *e.offset(0 as ::core::ffi::c_int as isize) = (L_BASE as uint32_t)
            .wrapping_add(
                s.wrapping_div((V_COUNT as uint32_t).wrapping_mul(T_COUNT as uint32_t)),
            );
        *e.offset(1 as ::core::ffi::c_int as isize) = (V_BASE as uint32_t)
            .wrapping_add(
                s
                    .wrapping_rem(
                        (V_COUNT as uint32_t).wrapping_mul(T_COUNT as uint32_t),
                    )
                    .wrapping_div(T_COUNT as uint32_t),
            );
        *e.offset(2 as ::core::ffi::c_int as isize) = (T_BASE as uint32_t)
            .wrapping_add(s.wrapping_rem(T_COUNT as uint32_t));
        return if s.wrapping_rem(T_COUNT as uint32_t) == 0 as uint32_t {
            2 as uint32_t
        } else {
            3 as uint32_t
        };
    }
    if dl == 0 as uint32_t {
        *e.offset(0 as ::core::ffi::c_int as isize) = cp
            | w
                & (TOKS_NFC_SECOND as uint32_t
                    | (TOKS_NFC_CLS_MASK as uint32_t) << TOKS_NFC_CLS_SHIFT);
        return 1 as uint32_t;
    }
    let mut j: uint32_t = 0 as uint32_t;
    while j < dl {
        *e.offset(j as isize) = toks_nfc_pool[off.wrapping_add(j) as usize];
        j = j.wrapping_add(1);
    }
    return dl;
}
unsafe extern "C" fn atom_chars(
    mut f: uint32_t,
    mut s: *const uint8_t,
    mut i: uint64_t,
    mut len: uint64_t,
    mut e: *mut uint32_t,
    mut nb: *mut uint64_t,
) -> uint32_t {
    let mut k: uint32_t = toks_utf8_len(s.offset(i as isize), len.wrapping_sub(i));
    *nb = (if k != 0 as uint32_t { k } else { 1 as uint32_t }) as uint64_t;
    if k == 0 as uint32_t {
        *e.offset(0 as ::core::ffi::c_int as isize) = (BAD
            | *s.offset(i as isize) as ::core::ffi::c_uint) as uint32_t;
        return 1 as uint32_t;
    }
    let mut cp: uint32_t = if k == 1 as uint32_t {
        *s.offset(i as isize) as uint32_t
    } else {
        toks_cp_decode(s.offset(i as isize), k)
    };
    let mut bc: uint8_t = (if f & (TOKS_NS_CLEAN as uint32_t | TOKS_NS_CJK as uint32_t)
        != 0 as uint32_t
    {
        toks_bert_cls(cp) as ::core::ffi::c_uint
    } else {
        0 as ::core::ffi::c_uint
    }) as uint8_t;
    if f & TOKS_NS_CLEAN as uint32_t != 0 as uint32_t
        && bc as ::core::ffi::c_uint & (TOKS_BC_REMOVE | TOKS_BC_WS)
            != 0 as ::core::ffi::c_uint
    {
        *e.offset(0 as ::core::ffi::c_int as isize) = 0x20 as ::core::ffi::c_uint
            as uint32_t;
        return if bc as ::core::ffi::c_uint & TOKS_BC_REMOVE != 0 as ::core::ffi::c_uint
        {
            0 as uint32_t
        } else {
            1 as uint32_t
        };
    }
    if f & TOKS_NS_CJK as uint32_t == 0 as uint32_t
        || bc as ::core::ffi::c_uint & TOKS_BC_CJK == 0 as ::core::ffi::c_uint
    {
        return decomp(f, cp, e);
    }
    *e.offset(0 as ::core::ffi::c_int as isize) = 0x20 as ::core::ffi::c_uint
        as uint32_t;
    let mut n: uint32_t = decomp(f, cp, e.offset(1 as ::core::ffi::c_int as isize));
    *e.offset(n.wrapping_add(1 as uint32_t) as isize) = 0x20 as ::core::ffi::c_uint
        as uint32_t;
    return n.wrapping_add(2 as uint32_t);
}
unsafe extern "C" fn compose(
    mut f: uint32_t,
    mut a: uint32_t,
    mut b: uint32_t,
) -> uint32_t {
    if f & TOKS_NS_COMPOSE as uint32_t == 0 as uint32_t
        || (a | b) & BAD as uint32_t != 0 as uint32_t
        || b & TOKS_NFC_SECOND as uint32_t == 0 as uint32_t
    {
        return 0 as uint32_t;
    }
    let mut ca: uint32_t = a & TOKS_NFC_CP_MASK as uint32_t;
    let mut cb: uint32_t = b & TOKS_NFC_CP_MASK as uint32_t;
    if ca.wrapping_sub(L_BASE as uint32_t) < L_COUNT as uint32_t
        && cb.wrapping_sub(V_BASE as uint32_t) < V_COUNT as uint32_t
    {
        return (S_BASE as uint32_t)
            .wrapping_add(
                ca
                    .wrapping_sub(L_BASE as uint32_t)
                    .wrapping_mul(V_COUNT as uint32_t)
                    .wrapping_add(cb.wrapping_sub(V_BASE as uint32_t))
                    .wrapping_mul(T_COUNT as uint32_t),
            );
    }
    if ca.wrapping_sub(S_BASE as uint32_t) < S_COUNT as uint32_t
        && cb.wrapping_sub((T_BASE as uint32_t).wrapping_add(1 as uint32_t))
            < (T_COUNT as uint32_t).wrapping_sub(1 as uint32_t)
        && ca.wrapping_sub(S_BASE as uint32_t).wrapping_rem(T_COUNT as uint32_t)
            == 0 as uint32_t
    {
        return ca.wrapping_add(cb.wrapping_sub(T_BASE as uint32_t));
    }
    let mut key: uint64_t = (ca as uint64_t) << 21 as ::core::ffi::c_int
        | cb as uint64_t;
    let mut h: uint64_t = ((key as ::core::ffi::c_ulonglong).wrapping_mul(TOKS_NFC_FIB64)
        >> 64 as ::core::ffi::c_int - TOKS_NFC_COMP_LOG2) as uint64_t;
    let mut p: uint32_t = 0 as uint32_t;
    while p < TOKS_NFC_COMP_MAXPROBE as uint32_t {
        let mut slot: uint64_t = toks_nfc_comp[(h.wrapping_add(p as uint64_t)
            & ((1 as ::core::ffi::c_uint) << TOKS_NFC_COMP_LOG2)
                .wrapping_sub(1 as ::core::ffi::c_uint) as uint64_t) as usize];
        if slot == 0 as uint64_t {
            return 0 as uint32_t;
        }
        if slot >> 21 as ::core::ffi::c_int == key {
            return (slot & 0x1fffff as uint64_t) as uint32_t;
        }
        p = p.wrapping_add(1);
    }
    return 0 as uint32_t;
}
unsafe extern "C" fn map_char(
    mut f: uint32_t,
    mut x: uint32_t,
    mut o: *mut uint32_t,
) -> uint32_t {
    let mut c: uint32_t = x & TOKS_NFC_CP_MASK as uint32_t;
    let mut bc: uint8_t = (if f
        & (TOKS_NS_STRIP_MN as uint32_t | TOKS_NS_LOWER as uint32_t) != 0 as uint32_t
    {
        toks_bert_cls(c) as ::core::ffi::c_uint
    } else {
        0 as ::core::ffi::c_uint
    }) as uint8_t;
    if f & TOKS_NS_STRIP_MN as uint32_t != 0 as uint32_t
        && bc as ::core::ffi::c_uint & TOKS_BC_MN != 0 as ::core::ffi::c_uint
    {
        return 0 as uint32_t;
    }
    if f & TOKS_NS_STRIP_M as uint32_t != 0 as uint32_t
        && toks_nfc_info(c) & TOKS_NFC_MARK as uint32_t != 0 as uint32_t
    {
        return 0 as uint32_t;
    }
    if f & TOKS_NS_LOWER as uint32_t == 0 as uint32_t
        || bc as ::core::ffi::c_uint & TOKS_BC_LOWER == 0 as ::core::ffi::c_uint
    {
        *o.offset(0 as ::core::ffi::c_int as isize) = c;
        return 1 as uint32_t;
    }
    let mut m: *const toks_bert_map = (&raw const toks_bert_maps as *const toks_bert_map)
        .offset(
            (toks_bert_map_index as unsafe extern "C" fn(uint32_t) -> uint16_t)(c)
                as isize,
        ) as *const toks_bert_map;
    let mut k: uint32_t = 0 as uint32_t;
    while k < (*m).low_len as uint32_t {
        *o.offset(k as isize) = toks_bert_pool[((*m).low_off as uint32_t).wrapping_add(k)
            as usize];
        k = k.wrapping_add(1);
    }
    return (*m).low_len as uint32_t;
}
unsafe extern "C" fn put(
    mut f: uint32_t,
    mut o: *mut uint8_t,
    mut x: uint32_t,
) -> uint64_t {
    let mut tmp: [uint8_t; 4] = [0; 4];
    let mut m: [uint32_t; 2] = [0; 2];
    if x & BAD as uint32_t != 0 as uint32_t {
        if !o.is_null() {
            *o.offset(0 as ::core::ffi::c_int as isize) = x as uint8_t;
        }
        return 1 as uint64_t;
    }
    let mut n: uint32_t = map_char(f, x, &raw mut m as *mut uint32_t);
    let mut w: uint64_t = 0 as uint64_t;
    let mut k: uint32_t = 0 as uint32_t;
    while k < n {
        w = w
            .wrapping_add(
                toks_utf8_put(
                    if !o.is_null() {
                        o.offset(w as isize)
                    } else {
                        &raw mut tmp as *mut uint8_t
                    },
                    m[k as usize],
                ) as uint64_t,
            );
        k = k.wrapping_add(1);
    }
    return w;
}
unsafe extern "C" fn cur_at(
    mut c: *mut cur,
    mut s: *const uint8_t,
    mut len: uint64_t,
    mut i: uint64_t,
    mut b: uint64_t,
) {
    (*c).j = 0 as uint32_t;
    (*c).n = 0 as uint32_t;
    (*c).nb = 0 as uint64_t;
    while i < b {
        (*c).n = atom_chars(
            (*c).f,
            s,
            i,
            len,
            &raw mut (*c).e as *mut uint32_t,
            &raw mut (*c).nb,
        );
        if (*c).n != 0 as uint32_t {
            break;
        }
        i = i.wrapping_add((*c).nb);
    }
    (*c).i = i;
}
#[inline]
unsafe extern "C" fn cur_next(
    mut c: *mut cur,
    mut s: *const uint8_t,
    mut len: uint64_t,
    mut b: uint64_t,
) {
    (*c).j = (*c).j.wrapping_add(1);
    if (*c).j < (*c).n {
        return;
    }
    cur_at(c, s, len, (*c).i.wrapping_add((*c).nb), b);
}
unsafe extern "C" fn nth_of_class(
    mut r0: *const cur,
    mut s: *const uint8_t,
    mut len: uint64_t,
    mut b: uint64_t,
    mut k: uint32_t,
    mut nth: uint32_t,
) -> uint32_t {
    let mut c: cur = *r0;
    while c.n != 0 as uint32_t
        && c.e[c.j as usize] >> TOKS_NFC_CLS_SHIFT & TOKS_NFC_CLS_MASK as uint32_t
            != 0 as uint32_t
    {
        if c.e[c.j as usize] >> TOKS_NFC_CLS_SHIFT & TOKS_NFC_CLS_MASK as uint32_t == k {
            if nth == 0 as uint32_t {
                return c.e[c.j as usize];
            }
            nth = nth.wrapping_sub(1);
        }
        cur_next(&raw mut c, s, len, b);
    }
    return 0 as uint32_t;
}
unsafe extern "C" fn nrun(
    mut f: uint32_t,
    mut s: *const uint8_t,
    mut len: uint64_t,
    mut a: uint64_t,
    mut b: uint64_t,
    mut out: *mut uint8_t,
) -> uint64_t {
    let mut acc: [uint64_t; 64] = [0; 64];
    let mut first: [uint32_t; 64] = [0; 64];
    let mut cons: [uint8_t; 64] = [0; 64];
    let mut o: uint64_t = 0 as uint64_t;
    let mut C: uint32_t = 0 as uint32_t;
    let mut have: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut c: cur = cur {
        i: 0,
        nb: 0,
        n: 0,
        j: 0,
        f: 0,
        e: [0; 20],
    };
    c.f = f;
    cur_at(&raw mut c, s, len, a, b);
    while c.n != 0 as uint32_t {
        let mut x: uint32_t = c.e[c.j as usize];
        if x >> TOKS_NFC_CLS_SHIFT & TOKS_NFC_CLS_MASK as uint32_t == 0 as uint32_t {
            let mut r: uint32_t = if have != 0 {
                compose(f, C, x)
            } else {
                0 as uint32_t
            };
            if r != 0 as uint32_t {
                C = r;
            } else {
                if have != 0 {
                    o = o.wrapping_add(put(f, out.offset(o as isize), C));
                }
                C = x;
                have = 1 as ::core::ffi::c_int;
            }
            cur_next(&raw mut c, s, len, b);
        } else {
            let mut r0: cur = c;
            let mut mask: uint64_t = 0 as uint64_t;
            let mut m: uint64_t = 0 as uint64_t;
            let mut ab: uint64_t = 0 as uint64_t;
            while c.n != 0 as uint32_t
                && c.e[c.j as usize] >> TOKS_NFC_CLS_SHIFT
                    & TOKS_NFC_CLS_MASK as uint32_t != 0 as uint32_t
            {
                let mut y: uint32_t = c.e[c.j as usize];
                let mut k: uint32_t = y >> TOKS_NFC_CLS_SHIFT
                    & TOKS_NFC_CLS_MASK as uint32_t;
                if mask >> k & 1 as uint64_t == 0 as uint64_t {
                    mask = (mask as ::core::ffi::c_ulonglong
                        | (1 as ::core::ffi::c_ulonglong) << k) as uint64_t;
                    acc[k as usize] = 0 as uint64_t;
                    first[k as usize] = y;
                    cons[k as usize] = 0 as uint8_t;
                }
                acc[k as usize] = acc[k as usize]
                    .wrapping_add(put(f, ::core::ptr::null_mut::<uint8_t>(), y));
                m = m.wrapping_add(1);
                cur_next(&raw mut c, s, len, b);
            }
            if m == 1 as uint64_t {
                let mut r_0: uint32_t = if have != 0 {
                    compose(f, C, x)
                } else {
                    0 as uint32_t
                };
                if r_0 != 0 as uint32_t {
                    C = r_0;
                } else {
                    if have != 0 {
                        o = o.wrapping_add(put(f, out.offset(o as isize), C));
                        have = 0 as ::core::ffi::c_int;
                    }
                    o = o.wrapping_add(put(f, out.offset(o as isize), x));
                }
            } else {
                if have != 0 {
                    let mut mm: uint64_t = mask;
                    while mm != 0 as uint64_t {
                        let mut k_0: uint32_t = (mm as ::core::ffi::c_ulonglong)
                            .trailing_zeros() as i32 as uint32_t;
                        let mut y_0: uint32_t = first[k_0 as usize];
                        let mut t: uint32_t = 0 as uint32_t;
                        while t < TOKS_NFC_MAX_DECOMP as uint32_t && y_0 != 0 as uint32_t
                        {
                            let mut r_1: uint32_t = compose(f, C, y_0);
                            if r_1 == 0 as uint32_t {
                                break;
                            }
                            C = r_1;
                            cons[k_0 as usize] = cons[k_0 as usize].wrapping_add(1);
                            ab = ab.wrapping_add(1);
                            acc[k_0 as usize] = acc[k_0 as usize]
                                .wrapping_sub(
                                    put(f, ::core::ptr::null_mut::<uint8_t>(), y_0),
                                );
                            y_0 = nth_of_class(
                                &raw mut r0,
                                s,
                                len,
                                b,
                                k_0,
                                cons[k_0 as usize] as uint32_t,
                            );
                            t = t.wrapping_add(1);
                        }
                        mm &= mm.wrapping_sub(1 as uint64_t);
                    }
                }
                if ab == m {
                    continue;
                }
                if have != 0 {
                    o = o.wrapping_add(put(f, out.offset(o as isize), C));
                    have = 0 as ::core::ffi::c_int;
                }
                let mut mm_0: uint64_t = mask;
                while mm_0 != 0 as uint64_t {
                    let mut k_1: uint32_t = (mm_0 as ::core::ffi::c_ulonglong)
                        .trailing_zeros() as i32 as uint32_t;
                    let mut nk: uint64_t = acc[k_1 as usize];
                    acc[k_1 as usize] = o;
                    o = o.wrapping_add(nk);
                    mm_0 &= mm_0.wrapping_sub(1 as uint64_t);
                }
                c = r0;
                let mut i: uint64_t = 0 as uint64_t;
                while i < m {
                    let mut y_1: uint32_t = c.e[c.j as usize];
                    let mut k_2: uint32_t = y_1 >> TOKS_NFC_CLS_SHIFT
                        & TOKS_NFC_CLS_MASK as uint32_t;
                    if cons[k_2 as usize] as ::core::ffi::c_int
                        != 0 as ::core::ffi::c_int
                    {
                        cons[k_2 as usize] = cons[k_2 as usize].wrapping_sub(1);
                    } else {
                        acc[k_2 as usize] = acc[k_2 as usize]
                            .wrapping_add(
                                put(f, out.offset(acc[k_2 as usize] as isize), y_1),
                            );
                    }
                    cur_next(&raw mut c, s, len, b);
                    i = i.wrapping_add(1);
                }
            }
        }
    }
    if have != 0 {
        o = o.wrapping_add(put(f, out.offset(o as isize), C));
    }
    return o;
}
#[no_mangle]
pub unsafe extern "C" fn toks_norm(
    mut f: uint32_t,
    mut text: *const uint8_t,
    mut len: uint64_t,
    mut out: *mut uint8_t,
    mut cap: uint64_t,
) -> int64_t {
    if cap
        < ((if f & TOKS_NS_COMPAT as uint32_t != 0 as uint32_t {
            TOKS_NFKC_X
        } else {
            TOKS_NFC_X
        }) as uint64_t)
            .wrapping_mul(len)
    {
        return TOKS_E_CAP as int64_t;
    }
    if f & !(TOKS_NS_COMPAT as uint32_t) != TOKS_NS_NFC as uint32_t {
        return nrun(f, text, len, 0 as uint64_t, len, out) as int64_t;
    }
    let mut pos: uint64_t = 0 as uint64_t;
    let mut o: uint64_t = 0 as uint64_t;
    let mut e: uint64_t = 0;
    while pos < len {
        let mut s: uint64_t = toks_nfc_scan(f, text, len, pos, &raw mut e);
        memcpy(
            out.offset(o as isize) as *mut ::core::ffi::c_void,
            text.offset(pos as isize) as *const ::core::ffi::c_void,
            (s as size_t).wrapping_sub(pos as size_t),
        );
        o = o.wrapping_add(s.wrapping_sub(pos));
        if s == len {
            break;
        }
        o = o.wrapping_add(nrun(f, text, len, s, e, out.offset(o as isize)));
        pos = e;
    }
    return o as int64_t;
}
#[no_mangle]
pub unsafe extern "C" fn toks_norm_char(
    mut f: uint32_t,
    mut cp: uint32_t,
    mut o: *mut uint32_t,
) -> uint32_t {
    let mut e: [uint32_t; 20] = [0; 20];
    let mut m: [uint32_t; 2] = [0; 2];
    let mut n: uint32_t = 0 as uint32_t;
    let mut nd: uint32_t = decomp(f, cp, &raw mut e as *mut uint32_t);
    let mut i: uint32_t = 0 as uint32_t;
    while i < nd {
        let mut k: uint32_t = map_char(f, e[i as usize], &raw mut m as *mut uint32_t);
        if k == 0 as uint32_t
            && e[i as usize] >> TOKS_NFC_CLS_SHIFT & TOKS_NFC_CLS_MASK as uint32_t
                == 0 as uint32_t
        {
            let fresh0 = n;
            n = n.wrapping_add(1);
            *o.offset(fresh0 as isize) = TOKS_NORM_GHOST as uint32_t;
        }
        let mut j: uint32_t = 0 as uint32_t;
        while j < k {
            let fresh1 = n;
            n = n.wrapping_add(1);
            *o.offset(fresh1 as isize) = e[i as usize]
                & (TOKS_NFC_CLS_MASK as uint32_t) << TOKS_NFC_CLS_SHIFT | m[j as usize];
            j = j.wrapping_add(1);
        }
        i = i.wrapping_add(1);
    }
    return n;
}
unsafe extern "C" fn run_is_nfc(
    mut f: uint32_t,
    mut s: *const uint8_t,
    mut len: uint64_t,
    mut a: uint64_t,
    mut b: uint64_t,
) -> ::core::ffi::c_int {
    let mut last: uint32_t = 0 as uint32_t;
    let mut dec: uint32_t = TOKS_NFC_SECOND as uint32_t
        | (TOKS_NFC_DLEN_MASK as uint32_t) << TOKS_NFC_DLEN_SHIFT
        | (if f & TOKS_NS_COMPAT as uint32_t != 0 as uint32_t {
            TOKS_NFC_KX as uint32_t
        } else {
            0 as uint32_t
        });
    let mut i: uint64_t = a;
    while i < b {
        let mut k: uint32_t = toks_utf8_len(s.offset(i as isize), len.wrapping_sub(i));
        if k == 0 as uint32_t {
            if i != a {
                return 0 as ::core::ffi::c_int;
            }
            i = i.wrapping_add(1);
        } else {
            let mut w: uint32_t = toks_nfc_info(
                if k == 1 as uint32_t {
                    *s.offset(i as isize) as uint32_t
                } else {
                    toks_cp_decode(s.offset(i as isize), k)
                },
            );
            let mut c: uint32_t = w >> TOKS_NFC_CLS_SHIFT
                & TOKS_NFC_CLS_MASK as uint32_t;
            if i == a
                && w
                    & (if f & TOKS_NS_COMPAT as uint32_t != 0 as uint32_t {
                        TOKS_NFC_NBK as uint32_t
                    } else {
                        TOKS_NFC_NB as uint32_t
                    }) == 0 as uint32_t
            {
                i = i.wrapping_add(k as uint64_t);
            } else {
                if w & dec != 0 as uint32_t || c < last {
                    return 0 as ::core::ffi::c_int;
                }
                last = c;
                i = i.wrapping_add(k as uint64_t);
            }
        }
    }
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn run_changes(
    mut f: uint32_t,
    mut s: *const uint8_t,
    mut len: uint64_t,
    mut a: uint64_t,
    mut b: uint64_t,
) -> ::core::ffi::c_int {
    if run_is_nfc(f, s, len, a, b) != 0 {
        return 0 as ::core::ffi::c_int;
    }
    if b.wrapping_sub(a) > TOKS_NFC_SCAN_RUN as uint64_t {
        return 1 as ::core::ffi::c_int;
    }
    let mut buf: [uint8_t; 704] = [0; 704];
    let mut n: uint64_t = nrun(f, s, len, a, b, &raw mut buf as *mut uint8_t);
    return (n != b.wrapping_sub(a)
        || memcmp(
            &raw mut buf as *mut uint8_t as *const ::core::ffi::c_void,
            s.offset(a as isize) as *const ::core::ffi::c_void,
            n as size_t,
        ) != 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn last_atom(
    mut s: *const uint8_t,
    mut p: uint64_t,
    mut h: uint64_t,
) -> uint64_t {
    if h.wrapping_sub(p) >= 2 as uint64_t
        && *s.offset(h.wrapping_sub(2 as uint64_t) as isize) as ::core::ffi::c_uint
            >= 0xc2 as ::core::ffi::c_uint
        && *s.offset(h.wrapping_sub(1 as uint64_t) as isize) as ::core::ffi::c_uint
            >= 0x80 as ::core::ffi::c_uint
        && (*s.offset(h.wrapping_sub(1 as uint64_t) as isize) as ::core::ffi::c_uint)
            < 0xc0 as ::core::ffi::c_uint
    {
        return h.wrapping_sub(2 as uint64_t);
    }
    return h.wrapping_sub(1 as uint64_t);
}
#[inline]
unsafe extern "C" fn clean_run(
    mut f: uint32_t,
    mut text: *const uint8_t,
    mut len: uint64_t,
    mut p: uint64_t,
    mut hot: uint8_t,
    mut rs: *mut uint64_t,
) -> uint64_t {
    let mut a: uint64_t = (0x101010101010101 as ::core::ffi::c_ulonglong)
        .wrapping_mul(
            (0x40 as ::core::ffi::c_uint)
                .wrapping_sub(hot as ::core::ffi::c_uint & 0x3f as ::core::ffi::c_uint)
                as ::core::ffi::c_ulonglong,
        ) as uint64_t;
    let mut last: uint64_t = *rs;
    let mut fast: *const uint64_t = (&raw const toks_nfc_fast as *const uint64_t)
        .offset(
            (if f & TOKS_NS_COMPAT as uint32_t != 0 as uint32_t {
                1024 as ::core::ffi::c_uint
            } else {
                0 as ::core::ffi::c_uint
            }) as isize,
        );
    while p < len {
        let mut b0: uint32_t = *text.offset(p as isize) as uint32_t;
        if b0 < hot as uint32_t {
            if len.wrapping_sub(p) < 8 as uint64_t {
                break;
            }
            let mut m: uint64_t = hot_bytes(load64(text.offset(p as isize)), a);
            if m == 0 as uint64_t {
                break;
            }
            let mut h: uint64_t = p
                .wrapping_add(
                    (m as ::core::ffi::c_ulonglong).trailing_zeros() as i32 as uint64_t
                        >> 3 as ::core::ffi::c_int,
                );
            last = last_atom(text, p, h);
            p = h;
        } else {
            if b0 >= 0xf0 as uint32_t || len.wrapping_sub(p) < 3 as uint64_t {
                break;
            }
            let mut b1: uint32_t = *text.offset(p.wrapping_add(1 as uint64_t) as isize)
                as uint32_t;
            let mut b2: uint32_t = *text.offset(p.wrapping_add(2 as uint64_t) as isize)
                as uint32_t;
            let mut cp: uint32_t = 0;
            let mut k: uint32_t = 0;
            if b0 < 0xe0 as uint32_t {
                if b1 & 0xc0 as uint32_t != 0x80 as uint32_t {
                    break;
                }
                cp = (b0 & 0x1f as uint32_t) << 6 as ::core::ffi::c_int
                    | b1 & 0x3f as uint32_t;
                k = 2 as ::core::ffi::c_uint as uint32_t;
            } else {
                if b1 & 0xc0 as uint32_t != 0x80 as uint32_t
                    || b2 & 0xc0 as uint32_t != 0x80 as uint32_t
                {
                    break;
                }
                cp = (b0 & 0xf as uint32_t) << 12 as ::core::ffi::c_int
                    | (b1 & 0x3f as uint32_t) << 6 as ::core::ffi::c_int
                    | b2 & 0x3f as uint32_t;
                if cp < 0x800 as uint32_t {
                    break;
                }
                k = 3 as ::core::ffi::c_uint as uint32_t;
            }
            if *fast.offset((cp >> 6 as ::core::ffi::c_int) as isize)
                >> (cp & 63 as uint32_t) & 1 as uint64_t == 0 as uint64_t
            {
                break;
            }
            last = p;
            p = p.wrapping_add(k as uint64_t);
        }
    }
    *rs = last;
    return p;
}
#[no_mangle]
pub unsafe extern "C" fn toks_nfc_scan(
    mut f: uint32_t,
    mut text: *const uint8_t,
    mut len: uint64_t,
    mut pos: uint64_t,
    mut run_end: *mut uint64_t,
) -> uint64_t {
    let mut rs: uint64_t = pos;
    let mut dirty: uint64_t = 0 as uint64_t;
    let mut hot: uint8_t = (if f & TOKS_NS_COMPAT as uint32_t != 0 as uint32_t {
        TOKS_NFKC_HOT_BYTE
    } else {
        TOKS_NFC_HOT_BYTE
    }) as uint8_t;
    while pos < len {
        let mut h: uint64_t = if *text.offset(pos as isize) as ::core::ffi::c_int
            >= hot as ::core::ffi::c_int
        {
            pos
        } else {
            toks_k2(text, pos, len, hot)
        };
        if h > pos {
            if dirty != 0 as uint64_t && run_changes(f, text, len, rs, pos) != 0 {
                *run_end = pos;
                return rs;
            }
            dirty = 0 as uint64_t;
            rs = last_atom(text, pos, h);
            pos = h;
            if pos == len {
                break;
            }
        }
        if dirty == 0 as uint64_t {
            let mut q: uint64_t = clean_run(f, text, len, pos, hot, &raw mut rs);
            if q != pos {
                pos = q;
                continue;
            }
        }
        let mut k: uint32_t = toks_utf8_len(
            text.offset(pos as isize),
            len.wrapping_sub(pos),
        );
        let mut w: uint32_t = if k != 0 as uint32_t {
            toks_nfc_info(toks_cp_decode(text.offset(pos as isize), k))
        } else {
            0 as uint32_t
        };
        if w
            & (if f & TOKS_NS_COMPAT as uint32_t != 0 as uint32_t {
                TOKS_NFC_NBK as uint32_t
            } else {
                TOKS_NFC_NB as uint32_t
            }) == 0 as uint32_t
        {
            if dirty != 0 as uint64_t && run_changes(f, text, len, rs, pos) != 0 {
                *run_end = pos;
                return rs;
            }
            dirty = 0 as uint64_t;
            rs = pos;
        } else {
            dirty = 1 as uint64_t;
        }
        pos = pos
            .wrapping_add(
                (if k != 0 as uint32_t { k } else { 1 as uint32_t }) as uint64_t,
            );
    }
    if dirty != 0 as uint64_t && run_changes(f, text, len, rs, len) != 0 {
        *run_end = len;
        return rs;
    }
    *run_end = len;
    return len;
}
#[no_mangle]
pub unsafe extern "C" fn toks_nfc_boundary(
    mut f: uint32_t,
    mut text: *const uint8_t,
    mut len: uint64_t,
    mut i: uint64_t,
) -> ::core::ffi::c_int {
    let mut k: uint32_t = toks_utf8_len(text.offset(i as isize), len.wrapping_sub(i));
    if k <= 1 as uint32_t {
        return 1 as ::core::ffi::c_int;
    }
    return (toks_nfc_info(toks_cp_decode(text.offset(i as isize), k))
        & (if f & TOKS_NS_COMPAT as uint32_t != 0 as uint32_t {
            TOKS_NFC_NBK as uint32_t
        } else {
            TOKS_NFC_NB as uint32_t
        }) == 0 as uint32_t) as ::core::ffi::c_int;
}
unsafe extern "C" fn atom_base(
    mut t: *const toks_tables,
    mut g: *const uint8_t,
    mut n: uint64_t,
    mut i: uint64_t,
    mut k: *mut uint32_t,
) -> uint32_t {
    if (*g.offset(i as isize) as ::core::ffi::c_uint) < 0x80 as ::core::ffi::c_uint {
        *k = 1 as ::core::ffi::c_uint as uint32_t;
        return *(*t).cls_ascii.offset(*g.offset(i as isize) as isize) as uint32_t
            & TOKS_C_BASE_MASK as uint32_t;
    }
    let mut m: uint32_t = toks_utf8_len(g.offset(i as isize), n.wrapping_sub(i));
    *k = if m != 0 as uint32_t { m } else { 1 as uint32_t };
    return if m != 0 as uint32_t {
        (toks_cls_cp(t, toks_cp_decode(g.offset(i as isize), m)) as ::core::ffi::c_uint
            & TOKS_C_BASE_MASK) as uint32_t
    } else {
        TOKS_C_P as uint32_t
    };
}
unsafe extern "C" fn y_base(
    mut t: *const toks_tables,
    mut f: uint32_t,
    mut g: *const uint8_t,
    mut n: uint64_t,
    mut p: uint64_t,
    mut at_change: ::core::ffi::c_int,
) -> uint32_t {
    let mut c: uint8_t = *g.offset(p as isize);
    if (c as ::core::ffi::c_uint) < 0x80 as ::core::ffi::c_uint {
        let mut b: uint32_t = *(*t).cls_ascii.offset(c as isize) as uint32_t
            & TOKS_C_BASE_MASK as uint32_t;
        return if at_change == 0 as ::core::ffi::c_int || b == TOKS_C_WS as uint32_t
            || b == TOKS_C_NL as uint32_t
        {
            b.wrapping_add(1 as uint32_t)
        } else {
            0 as uint32_t
        };
    }
    if at_change != 0 as ::core::ffi::c_int
        || (c as ::core::ffi::c_uint) < 0xc0 as ::core::ffi::c_uint
        || toks_nfc_boundary(f, g, n, p) == 0 as ::core::ffi::c_int
    {
        return 0 as uint32_t;
    }
    let mut k: uint32_t = 0;
    return atom_base(t, g, n, p, &raw mut k).wrapping_add(1 as uint32_t);
}
unsafe extern "C" fn restart_pair(
    mut bx: uint32_t,
    mut by: uint32_t,
) -> ::core::ffi::c_int {
    if bx == TOKS_C_L as uint32_t {
        return (by != TOKS_C_L as uint32_t) as ::core::ffi::c_int;
    }
    if bx == TOKS_C_N as uint32_t {
        return (by != TOKS_C_N as uint32_t) as ::core::ffi::c_int;
    }
    if bx == TOKS_C_P as uint32_t {
        return (by == TOKS_C_WS as uint32_t || by == TOKS_C_N as uint32_t)
            as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn atom_before(
    mut g: *const uint8_t,
    mut n: uint64_t,
    mut lo: uint64_t,
    mut p: uint64_t,
) -> uint64_t {
    if (*g.offset(p.wrapping_sub(1 as uint64_t) as isize) as ::core::ffi::c_uint)
        < 0x80 as ::core::ffi::c_uint
    {
        return p.wrapping_sub(1 as uint64_t);
    }
    let mut k: uint64_t = 2 as uint64_t;
    while k <= 4 as uint64_t && k <= p.wrapping_sub(lo) {
        if toks_utf8_len(
            g.offset(p as isize).offset(-(k as isize)),
            n.wrapping_sub(p.wrapping_sub(k)),
        ) as uint64_t == k
        {
            return p.wrapping_sub(k);
        }
        k = k.wrapping_add(1);
    }
    return p.wrapping_sub(1 as uint64_t);
}
unsafe extern "C" fn restart_before(
    mut t: *const toks_tables,
    mut f: uint32_t,
    mut g: *const uint8_t,
    mut n: uint64_t,
    mut lo: uint64_t,
    mut hi: uint64_t,
) -> uint64_t {
    if (*t).tmpl != TOKS_TMPL_CL100K as uint32_t {
        return lo;
    }
    let mut p: uint64_t = hi;
    while p > lo {
        let mut by: uint32_t = y_base(t, f, g, n, p, (p == hi) as ::core::ffi::c_int);
        let mut k: uint32_t = 0;
        if by != 0 as uint32_t
            && restart_pair(
                atom_base(t, g, n, atom_before(g, n, lo, p), &raw mut k),
                by.wrapping_sub(1 as uint32_t),
            ) != 0
        {
            return p;
        }
        p = p.wrapping_sub(1);
    }
    return lo;
}
unsafe extern "C" fn restart_after(
    mut t: *const toks_tables,
    mut f: uint32_t,
    mut g: *const uint8_t,
    mut n: uint64_t,
    mut lo: uint64_t,
    mut hi: uint64_t,
) -> uint64_t {
    if (*t).tmpl != TOKS_TMPL_CL100K as uint32_t {
        return if hi == n { n } else { 0 as uint64_t };
    }
    let mut q: uint64_t = lo;
    while q < hi {
        let mut k: uint32_t = 0;
        let mut bx: uint32_t = atom_base(t, g, n, q, &raw mut k);
        let mut p: uint64_t = q.wrapping_add(k as uint64_t);
        if p >= n {
            return n;
        }
        let mut by: uint32_t = y_base(t, f, g, n, p, (p == hi) as ::core::ffi::c_int);
        if by != 0 as uint32_t && restart_pair(bx, by.wrapping_sub(1 as uint32_t)) != 0 {
            return p;
        }
        q = p;
    }
    return if hi == n { n } else { 0 as uint64_t };
}
#[no_mangle]
pub unsafe extern "C" fn toks_nfc_plan_begin(
    mut p: *mut toks_nfc_plan,
    mut f: uint32_t,
    mut g: *const uint8_t,
    mut n: uint64_t,
) {
    (*p).r = 0 as uint64_t;
    (*p).d1 = n;
    (*p).f = f;
    (*p).d0 = toks_nfc_scan(f, g, n, 0 as uint64_t, &raw mut (*p).d1);
}
#[no_mangle]
pub unsafe extern "C" fn toks_nfc_plan_next(
    mut p: *mut toks_nfc_plan,
    mut t: *const toks_tables,
    mut g: *const uint8_t,
    mut n: uint64_t,
    mut s: *mut uint64_t,
    mut e: *mut uint64_t,
) -> ::core::ffi::c_int {
    if (*p).d0 >= n {
        return 0 as ::core::ffi::c_int;
    }
    let mut d1: uint64_t = (*p).d1;
    let mut e0: uint64_t = 0;
    let mut e1: uint64_t = 0;
    let mut end: uint64_t = 0;
    *s = restart_before(t, (*p).f, g, n, (*p).r, (*p).d0);
    loop {
        e0 = toks_nfc_scan((*p).f, g, n, d1, &raw mut e1);
        end = restart_after(t, (*p).f, g, n, d1, e0);
        if end != 0 as uint64_t {
            break;
        }
        d1 = e1;
    }
    *e = end;
    (*p).r = end;
    (*p).d0 = e0;
    (*p).d1 = e1;
    return 1 as ::core::ffi::c_int;
}
