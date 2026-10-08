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
    static TOKS_CRC32C_TAB: [uint32_t; 256];
}
pub type size_t = usize;
pub type __uint8_t = u8;
pub type __uint16_t = u16;
pub type __uint32_t = u32;
pub type __uint64_t = u64;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
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
pub struct bpe_mt {
    pub slots: *const uint64_t,
    pub shift: uint64_t,
    pub mask: uint64_t,
    pub maxprobe: uint64_t,
    pub r2i: *const uint32_t,
    pub pf: *const uint64_t,
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
pub const UINT32_MAX: ::core::ffi::c_uint = 4294967295 as ::core::ffi::c_uint;
pub const UINT64_MAX: ::core::ffi::c_ulong = 18446744073709551615
    as ::core::ffi::c_ulong;
pub const TOKS_ID_MASK: ::core::ffi::c_uint = 0x1fffff as ::core::ffi::c_uint;
pub const TOKS_TF_IGNORE_MERGES: ::core::ffi::c_uint = 0x1 as ::core::ffi::c_uint;
pub const TOKS_TF_IDS_AS_RANK: ::core::ffi::c_uint = 0x2 as ::core::ffi::c_uint;
pub const TOKS_TF_PROBE_LONG: ::core::ffi::c_uint = 0x80 as ::core::ffi::c_uint;
pub const TOKS_TF_PROBE_ASCII: ::core::ffi::c_uint = 0x100 as ::core::ffi::c_uint;
pub const TOKS_KEY_MAXLEN: ::core::ffi::c_int = 15 as ::core::ffi::c_int;
pub const TOKS_FIB64: ::core::ffi::c_ulonglong = 0x9e3779b97f4a7c15
    as ::core::ffi::c_ulonglong;
pub const TOKS_PRIO_BITS: ::core::ffi::c_int = 22 as ::core::ffi::c_int;
pub const TOKS_PRIO_NONE: ::core::ffi::c_uint = 0xffffffff as ::core::ffi::c_uint;
pub const TOKS_PM_NEVER: ::core::ffi::c_int = 21 as ::core::ffi::c_int;
pub const TOKS_APM_PAIRS: ::core::ffi::c_int = 256 as ::core::ffi::c_int;
pub const TOKS_VSEED: ::core::ffi::c_uint = 0x85ebca6b as ::core::ffi::c_uint;
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
unsafe extern "C" fn bpe_vhash_find(
    mut t: *const toks_tables,
    mut p: *const uint8_t,
    mut len: uint64_t,
    mut id_out: *mut uint32_t,
) -> ::core::ffi::c_int {
    if (*t).vhash.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    let mut h: uint32_t = bpe_vhash_h(p, len);
    let mut i: uint64_t = h as uint64_t & (*t).vhash_mask;
    let mut k: uint64_t = 0 as uint64_t;
    while k <= (*t).vhash_mask {
        let mut slot: uint64_t = *(*t).vhash.offset(i as isize);
        if slot == BPE_EMPTY_SLOT as uint64_t {
            return 0 as ::core::ffi::c_int;
        }
        if slot as uint32_t == h {
            let mut id: uint32_t = (slot >> 32 as ::core::ffi::c_int) as uint32_t;
            let mut o0: uint32_t = *(*t).tok_off.offset(id as isize);
            if (*(*t).tok_off.offset(id.wrapping_add(1 as uint32_t) as isize)
                as uint64_t)
                .wrapping_sub(o0 as uint64_t) == len
                && memcmp(
                    (*t).tok_bytes.offset(o0 as isize) as *const ::core::ffi::c_void,
                    p as *const ::core::ffi::c_void,
                    len as size_t,
                ) == 0 as ::core::ffi::c_int
            {
                *id_out = id;
                return 1 as ::core::ffi::c_int;
            }
        }
        i = i.wrapping_add(1 as uint64_t) & (*t).vhash_mask;
        k = k.wrapping_add(1);
    }
    return 0 as ::core::ffi::c_int;
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
unsafe extern "C" fn bpe_pm_at(
    mut pm: *const uint8_t,
    mut p: *const uint8_t,
    mut i: uint64_t,
    mut len: uint64_t,
    mut k: *mut uint32_t,
) -> uint32_t {
    let mut s1: uint32_t = 0 as uint32_t;
    let mut s2: uint32_t = 0 as uint32_t;
    let mut n: uint32_t = bpe_pm_index(
        p.offset(i as isize),
        len.wrapping_sub(i),
        &raw mut s1,
        &raw mut s2,
    );
    *k = 1 as ::core::ffi::c_uint as uint32_t;
    if n == 0 as uint32_t {
        return UINT32_MAX as uint32_t;
    }
    let mut off: uint32_t = *(pm as *const ::core::ffi::c_void as *const uint32_t)
        .offset(s1 as isize);
    let mut e: uint64_t = *(pm
        .offset(off as isize)
        .offset((8 as uint32_t).wrapping_mul(s2) as isize) as *const ::core::ffi::c_void
        as *const uint64_t);
    let mut lb: uint32_t = if i > 0 as uint64_t {
        bpe_pm_lbit(*p.offset(i.wrapping_sub(1 as uint64_t) as isize) as uint32_t)
    } else {
        TOKS_PM_NEVER as uint32_t
    };
    let mut rb: uint32_t = if i.wrapping_add(n as uint64_t) < len {
        bpe_pm_rbit(*p.offset(i.wrapping_add(n as uint64_t) as isize) as uint32_t)
    } else {
        TOKS_PM_NEVER as uint32_t
    };
    if e == 0 as uint64_t || (e >> lb | e >> rb) & 1 as uint64_t != 0 as uint64_t {
        return UINT32_MAX as uint32_t;
    }
    *k = n;
    return e as uint32_t & TOKS_ID_MASK as uint32_t;
}
pub const K6_END: ::core::ffi::c_uint = UINT32_MAX;
pub const K6_NONE: ::core::ffi::c_uint = UINT32_MAX;
pub const K6_SHORT: ::core::ffi::c_uint = 32 as ::core::ffi::c_uint;
pub const K6_SHIFT: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
unsafe extern "C" fn k6_init(
    mut t: *const toks_tables,
    mut mt: *const bpe_mt,
    mut p: *const uint8_t,
    mut len: uint64_t,
    mut id: *mut uint32_t,
    mut pr: *mut uint32_t,
) -> uint32_t {
    let mut pm: *const uint8_t = (*t).premerge;
    let mut apm: *const uint8_t = if len.wrapping_sub(4 as uint64_t)
        <= (TOKS_KEY_MAXLEN as ::core::ffi::c_uint)
            .wrapping_sub(4 as ::core::ffi::c_uint) as uint64_t
    {
        (*t).apm
    } else {
        ::core::ptr::null::<uint8_t>()
    };
    let mut b2i: *const uint32_t = (*t).byte2id;
    let mut bp: *const uint32_t = (*t).bytepair;
    let mut n: uint32_t = 0 as uint32_t;
    let mut pb: uint32_t = 0 as uint32_t;
    let mut pbyte: uint32_t = 0 as uint32_t;
    let mut i: uint64_t = 0 as uint64_t;
    while i < len {
        let mut b: uint32_t = *p.offset(i as isize) as uint32_t;
        let mut k: uint32_t = 1 as uint32_t;
        let mut s: uint32_t = UINT32_MAX as uint32_t;
        if !pm.is_null() && b >= 0xc0 as uint32_t {
            s = bpe_pm_at(pm, p, i, len, &raw mut k);
        }
        if !apm.is_null() && b < 0x80 as uint32_t && i.wrapping_add(2 as uint64_t) <= len
            && (*p.offset(i.wrapping_add(1 as uint64_t) as isize) as ::core::ffi::c_uint)
                < 0x80 as ::core::ffi::c_uint
        {
            let mut w: [uint32_t; 3] = [0; 3];
            let mut bf: uint32_t = if i > 0 as uint64_t {
                (1 as uint32_t)
                    << *apm
                        .offset(
                            *p.offset(i.wrapping_sub(1 as uint64_t) as isize) as isize,
                        ) as ::core::ffi::c_int
            } else {
                0 as uint32_t
            };
            let mut af: uint32_t = if i.wrapping_add(2 as uint64_t) < len {
                (1 as uint32_t)
                    << *apm
                        .offset(
                            *p.offset(i.wrapping_add(2 as uint64_t) as isize) as isize,
                        ) as ::core::ffi::c_int
            } else {
                0 as uint32_t
            };
            memcpy(
                &raw mut w as *mut uint32_t as *mut ::core::ffi::c_void,
                apm
                    .offset(TOKS_APM_PAIRS as isize)
                    .offset(
                        (16 as uint32_t)
                            .wrapping_mul(
                                b << 7 as ::core::ffi::c_int
                                    | *p.offset(i.wrapping_add(1 as uint64_t) as isize)
                                        as uint32_t,
                            ) as isize,
                    ) as *const ::core::ffi::c_void,
                12 as size_t,
            );
            if w[0 as ::core::ffi::c_int as usize] != UINT32_MAX as uint32_t
                && w[1 as ::core::ffi::c_int as usize] & af
                    | w[2 as ::core::ffi::c_int as usize] & bf == 0 as uint32_t
            {
                s = w[0 as ::core::ffi::c_int as usize];
                k = 2 as ::core::ffi::c_uint as uint32_t;
            }
        }
        let mut byte: uint32_t = (s == UINT32_MAX as uint32_t) as ::core::ffi::c_int
            as uint32_t;
        if byte != 0 {
            s = *b2i.offset(b as isize);
        }
        if n > 0 as uint32_t {
            *pr.offset(n.wrapping_sub(1 as uint32_t) as isize) = if byte != 0
                && pbyte != 0
            {
                *bp.offset((pb << 8 as ::core::ffi::c_int | b) as isize)
            } else {
                bpe_mt_find(
                    mt,
                    bpe_pair_key(*id.offset(n.wrapping_sub(1 as uint32_t) as isize), s),
                )
            };
        }
        *id.offset(n as isize) = s;
        pb = b;
        pbyte = byte;
        n = (n as ::core::ffi::c_uint).wrapping_add(1 as ::core::ffi::c_uint) as uint32_t
            as uint32_t;
        i = i.wrapping_add(k as uint64_t);
    }
    *pr.offset(n.wrapping_sub(1 as uint32_t) as isize) = K6_NONE as uint32_t;
    return n;
}
unsafe extern "C" fn k6_short(
    mut mt: *const bpe_mt,
    mut id: *mut uint32_t,
    mut key: *mut uint32_t,
    mut n: uint32_t,
    mut out: *mut uint32_t,
) -> uint64_t {
    let mut nx: [uint8_t; 32] = [0; 32];
    let mut pv: [uint8_t; 32] = [0; 32];
    let mut i: uint32_t = 0 as uint32_t;
    while i < n {
        nx[i as usize] = i.wrapping_add(1 as uint32_t) as uint8_t;
        pv[i as usize] = i.wrapping_sub(1 as uint32_t) as uint8_t;
        i = i.wrapping_add(1);
    }
    loop {
        let mut k: uint32_t = K6_NONE as uint32_t;
        let mut i_0: uint32_t = 0 as uint32_t;
        while i_0 < n {
            k = if *key.offset(i_0 as isize) < k {
                *key.offset(i_0 as isize)
            } else {
                k
            };
            i_0 = i_0.wrapping_add(1);
        }
        if k == K6_NONE as uint32_t {
            break;
        }
        let mut pos: uint32_t = k & (K6_SHORT as uint32_t).wrapping_sub(1 as uint32_t);
        let mut nid: uint32_t = bpe_mt_id(mt, k >> K6_SHIFT);
        let mut r: uint32_t = nx[pos as usize] as uint32_t;
        let mut rn: uint32_t = nx[r as usize] as uint32_t;
        let mut l: uint32_t = pv[pos as usize] as uint32_t;
        *id.offset(pos as isize) = nid;
        *key.offset(r as isize) = K6_NONE as uint32_t;
        nx[pos as usize] = rn as uint8_t;
        *key.offset(pos as isize) = K6_NONE as uint32_t;
        if rn < n {
            pv[rn as usize] = pos as uint8_t;
            let mut q: uint32_t = bpe_mt_find(
                mt,
                bpe_pair_key(nid, *id.offset(rn as isize)),
            );
            if q != K6_NONE as uint32_t {
                *key.offset(pos as isize) = q << K6_SHIFT | pos;
            }
        }
        if l != 0xff as uint32_t {
            let mut q_0: uint32_t = bpe_mt_find(
                mt,
                bpe_pair_key(*id.offset(l as isize), nid),
            );
            *key.offset(l as isize) = if q_0 == K6_NONE as uint32_t {
                K6_NONE as uint32_t
            } else {
                q_0 << K6_SHIFT | l
            };
        }
    }
    let mut m: uint64_t = 0 as uint64_t;
    let mut i_1: uint32_t = 0 as uint32_t;
    while i_1 < n {
        toks_st32(
            out.offset(m as isize) as *mut ::core::ffi::c_void,
            *id.offset(i_1 as isize),
        );
        m = m.wrapping_add(1 as uint64_t);
        i_1 = nx[i_1 as usize] as uint32_t;
    }
    return m;
}
unsafe extern "C" fn k6_keys(
    mut key: *mut uint32_t,
    mut pr: *const uint32_t,
    mut n: uint32_t,
) {
    let mut i: uint32_t = 0 as uint32_t;
    while i < n {
        *key.offset(i as isize) = if *pr.offset(i as isize) == K6_NONE as uint32_t {
            K6_NONE as uint32_t
        } else {
            *pr.offset(i as isize) << K6_SHIFT | i
        };
        i = i.wrapping_add(1);
    }
}
unsafe extern "C" fn heap_push(
    mut heap: *mut uint64_t,
    mut n: *mut uint64_t,
    mut e: uint64_t,
) {
    let mut i: uint64_t = *n;
    *n = (*n).wrapping_add(1 as uint64_t);
    while i > 0 as uint64_t {
        let mut p: uint64_t = i.wrapping_sub(1 as uint64_t) >> 1 as ::core::ffi::c_int;
        if *heap.offset(p as isize) <= e {
            break;
        }
        *heap.offset(i as isize) = *heap.offset(p as isize);
        i = p;
    }
    *heap.offset(i as isize) = e;
}
unsafe extern "C" fn heap_sift(
    mut heap: *mut uint64_t,
    mut n: uint64_t,
    mut i: uint64_t,
    mut e: uint64_t,
) {
    loop {
        let mut c: uint64_t = (2 as uint64_t)
            .wrapping_mul(i)
            .wrapping_add(1 as uint64_t);
        if c >= n {
            break;
        }
        if c.wrapping_add(1 as uint64_t) < n
            && *heap.offset(c.wrapping_add(1 as uint64_t) as isize)
                < *heap.offset(c as isize)
        {
            c = c.wrapping_add(1 as uint64_t);
        }
        if *heap.offset(c as isize) >= e {
            break;
        }
        *heap.offset(i as isize) = *heap.offset(c as isize);
        i = c;
    }
    *heap.offset(i as isize) = e;
}
unsafe extern "C" fn k6_long(
    mut mt: *const bpe_mt,
    mut work: *mut uint8_t,
    mut len: uint64_t,
    mut n: uint32_t,
    mut out: *mut uint32_t,
) -> uint64_t {
    let mut heap: *mut uint64_t = work as *mut ::core::ffi::c_void as *mut uint64_t;
    let mut ids: *mut uint32_t = heap.offset((2 as uint64_t).wrapping_mul(len) as isize)
        as *mut ::core::ffi::c_void as *mut uint32_t;
    let mut prev: *mut uint32_t = ids.offset(len as isize);
    let mut nxt: *mut uint32_t = prev.offset(len as isize);
    let mut cur: *mut uint32_t = nxt.offset(len as isize);
    let mut hn: uint64_t = 0 as uint64_t;
    let mut i: uint32_t = 0 as uint32_t;
    while i < n {
        *prev.offset(i as isize) = i.wrapping_sub(1 as uint32_t);
        *nxt.offset(i as isize) = if i.wrapping_add(1 as uint32_t) == n {
            K6_END as uint32_t
        } else {
            i.wrapping_add(1 as uint32_t)
        };
        if *cur.offset(i as isize) != K6_NONE as uint32_t {
            let fresh0 = hn;
            hn = hn.wrapping_add(1);
            *heap.offset(fresh0 as isize) = (*cur.offset(i as isize) as uint64_t)
                << 32 as ::core::ffi::c_int | i as uint64_t;
        }
        i = i.wrapping_add(1);
    }
    let mut i_0: uint64_t = hn.wrapping_div(2 as uint64_t);
    while i_0 > 0 as uint64_t {
        heap_sift(
            heap,
            hn,
            i_0.wrapping_sub(1 as uint64_t),
            *heap.offset(i_0.wrapping_sub(1 as uint64_t) as isize),
        );
        i_0 = i_0.wrapping_sub(1);
    }
    while hn > 0 as uint64_t {
        let mut e: uint64_t = *heap.offset(0 as ::core::ffi::c_int as isize);
        hn = hn.wrapping_sub(1 as uint64_t);
        heap_sift(heap, hn, 0 as uint64_t, *heap.offset(hn as isize));
        let mut pos: uint32_t = e as uint32_t;
        let mut prio: uint32_t = (e >> 32 as ::core::ffi::c_int) as uint32_t;
        if *cur.offset(pos as isize) != prio {
            continue;
        }
        let mut nid: uint32_t = bpe_mt_id(mt, prio);
        let mut r: uint32_t = *nxt.offset(pos as isize);
        let mut rn: uint32_t = *nxt.offset(r as isize);
        let mut l: uint32_t = *prev.offset(pos as isize);
        *ids.offset(pos as isize) = nid;
        *cur.offset(r as isize) = K6_NONE as uint32_t;
        *nxt.offset(pos as isize) = rn;
        *cur.offset(pos as isize) = K6_NONE as uint32_t;
        if rn != K6_END as uint32_t {
            *prev.offset(rn as isize) = pos;
            let mut q: uint32_t = bpe_mt_find(
                mt,
                bpe_pair_key(nid, *ids.offset(rn as isize)),
            );
            *cur.offset(pos as isize) = q;
            if q != K6_NONE as uint32_t {
                heap_push(
                    heap,
                    &raw mut hn,
                    (q as uint64_t) << 32 as ::core::ffi::c_int | pos as uint64_t,
                );
            }
        }
        if l != K6_END as uint32_t {
            let mut q_0: uint32_t = bpe_mt_find(
                mt,
                bpe_pair_key(*ids.offset(l as isize), nid),
            );
            *cur.offset(l as isize) = q_0;
            if q_0 != K6_NONE as uint32_t {
                heap_push(
                    heap,
                    &raw mut hn,
                    (q_0 as uint64_t) << 32 as ::core::ffi::c_int | l as uint64_t,
                );
            }
        }
    }
    let mut m: uint64_t = 0 as uint64_t;
    let mut i_1: uint32_t = 0 as uint32_t;
    while i_1 != K6_END as uint32_t {
        toks_st32(
            out.offset(m as isize) as *mut ::core::ffi::c_void,
            *ids.offset(i_1 as isize),
        );
        m = m.wrapping_add(1 as uint64_t);
        i_1 = *nxt.offset(i_1 as isize);
    }
    return m;
}
#[no_mangle]
pub unsafe extern "C" fn toks_k6_merge(
    mut t: *const toks_tables,
    mut work: *mut uint8_t,
    mut cap: uint64_t,
    mut n: uint64_t,
    mut out: *mut uint32_t,
) -> uint64_t {
    let mut mt: bpe_mt = bpe_mt_of(t);
    let mut ids: *mut uint32_t = work.offset((16 as uint64_t).wrapping_mul(cap) as isize)
        as *mut ::core::ffi::c_void as *mut uint32_t;
    let mut cur: *mut uint32_t = ids.offset((3 as uint64_t).wrapping_mul(cap) as isize);
    let mut i: uint64_t = 0 as uint64_t;
    while i.wrapping_add(1 as uint64_t) < n {
        *cur.offset(i as isize) = bpe_mt_find(
            &raw mut mt,
            bpe_pair_key(
                *ids.offset(i as isize),
                *ids.offset(i.wrapping_add(1 as uint64_t) as isize),
            ),
        );
        i = i.wrapping_add(1);
    }
    *cur.offset(n.wrapping_sub(1 as uint64_t) as isize) = K6_NONE as uint32_t;
    if n > K6_SHORT as uint64_t {
        return k6_long(&raw mut mt, work, cap, n as uint32_t, out);
    }
    let mut id: [uint32_t; 32] = [0; 32];
    let mut key: [uint32_t; 32] = [0; 32];
    memcpy(
        &raw mut id as *mut uint32_t as *mut ::core::ffi::c_void,
        ids as *const ::core::ffi::c_void,
        (4 as size_t).wrapping_mul(n as size_t),
    );
    k6_keys(&raw mut key as *mut uint32_t, cur, n as uint32_t);
    return k6_short(
        &raw mut mt,
        &raw mut id as *mut uint32_t,
        &raw mut key as *mut uint32_t,
        n as uint32_t,
        out,
    );
}
#[no_mangle]
pub unsafe extern "C" fn toks_k6_bpe_c(
    mut t: *const toks_tables,
    mut a: *mut toks_k6_args,
) -> uint64_t {
    let mut p: *const uint8_t = (*a).piece;
    let mut len: uint64_t = (*a).len;
    let mut out: *mut uint32_t = (*a).out;
    (*a).n_out = 0 as uint64_t;
    (*a).merges = 0 as uint64_t;
    if len == 0 as uint64_t {
        return 0 as uint64_t;
    }
    let f: uint32_t = (*t).flags;
    if f & TOKS_TF_IGNORE_MERGES as uint32_t != 0 as uint32_t && !(*t).vhash.is_null()
        && (f & TOKS_TF_PROBE_LONG as uint32_t == 0 as uint32_t
            || len > TOKS_KEY_MAXLEN as uint64_t
                && (f & TOKS_TF_PROBE_ASCII as uint32_t == 0 as uint32_t
                    || (*p.offset(len.wrapping_sub(1 as uint64_t) as isize)
                        as ::core::ffi::c_uint) < 0x80 as ::core::ffi::c_uint))
        && len
            <= toks_ld32(
                ((*t)
                    .vhash
                    .offset((*t).vhash_mask as isize)
                    .offset(1 as ::core::ffi::c_uint as isize)
                    as *const ::core::ffi::c_void as *const uint32_t)
                    .offset(
                        *p.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                            as isize,
                    ) as *const ::core::ffi::c_void,
            ) as uint64_t
    {
        let mut id: uint32_t = 0 as uint32_t;
        if bpe_vhash_find(t, p, len, &raw mut id) != 0 {
            toks_st32(out as *mut ::core::ffi::c_void, id);
            (*a).n_out = 1 as uint64_t;
            return 1 as uint64_t;
        }
    }
    let mut mt: bpe_mt = bpe_mt_of(t);
    let mut id_0: [uint32_t; 32] = [0; 32];
    let mut key: [uint32_t; 32] = [0; 32];
    let mut n: uint64_t = 0;
    if len <= K6_SHORT as uint64_t {
        let mut b2i: *const uint32_t = (*t).byte2id;
        let mut bp: *const uint32_t = (*t).bytepair;
        let mut any: uint32_t = *p.offset(len.wrapping_sub(1 as uint64_t) as isize)
            as uint32_t;
        let mut i: uint32_t = 0 as uint32_t;
        while (i.wrapping_add(1 as uint32_t) as uint64_t) < len {
            let mut q: uint32_t = *bp
                .offset(
                    ((*p.offset(i as isize) as uint32_t) << 8 as ::core::ffi::c_int
                        | *p.offset(i.wrapping_add(1 as uint32_t) as isize) as uint32_t)
                        as isize,
                );
            any |= *p.offset(i as isize) as uint32_t;
            id_0[i as usize] = *b2i.offset(*p.offset(i as isize) as isize);
            key[i as usize] = if q == K6_NONE as uint32_t {
                K6_NONE as uint32_t
            } else {
                q << K6_SHIFT | i
            };
            i = i.wrapping_add(1);
        }
        id_0[len.wrapping_sub(1 as uint64_t) as usize] = *b2i
            .offset(*p.offset(len.wrapping_sub(1 as uint64_t) as isize) as isize);
        key[len.wrapping_sub(1 as uint64_t) as usize] = K6_NONE as uint32_t;
        let mut ns: uint32_t = len as uint32_t;
        if any >= 0xc0 as uint32_t && !(*t).premerge.is_null()
            || len.wrapping_sub(4 as uint64_t)
                <= (TOKS_KEY_MAXLEN as ::core::ffi::c_uint)
                    .wrapping_sub(4 as ::core::ffi::c_uint) as uint64_t
                && !(*t).apm.is_null()
        {
            let mut pr: [uint32_t; 32] = [0; 32];
            ns = k6_init(
                t,
                &raw mut mt,
                p,
                len,
                &raw mut id_0 as *mut uint32_t,
                &raw mut pr as *mut uint32_t,
            );
            k6_keys(&raw mut key as *mut uint32_t, &raw mut pr as *mut uint32_t, ns);
        }
        n = k6_short(
            &raw mut mt,
            &raw mut id_0 as *mut uint32_t,
            &raw mut key as *mut uint32_t,
            ns,
            out,
        );
    } else {
        let mut ids: *mut uint32_t = (*a)
            .work
            .offset((16 as uint64_t).wrapping_mul(len) as isize)
            as *mut ::core::ffi::c_void as *mut uint32_t;
        let mut cur: *mut uint32_t = ids
            .offset((3 as uint64_t).wrapping_mul(len) as isize);
        let mut ns_0: uint32_t = k6_init(t, &raw mut mt, p, len, ids, cur);
        if ns_0 <= K6_SHORT as uint32_t {
            memcpy(
                &raw mut id_0 as *mut uint32_t as *mut ::core::ffi::c_void,
                ids as *const ::core::ffi::c_void,
                (4 as uint32_t).wrapping_mul(ns_0) as size_t,
            );
            k6_keys(&raw mut key as *mut uint32_t, cur, ns_0);
            n = k6_short(
                &raw mut mt,
                &raw mut id_0 as *mut uint32_t,
                &raw mut key as *mut uint32_t,
                ns_0,
                out,
            );
        } else {
            n = k6_long(&raw mut mt, (*a).work, len, ns_0, out);
        }
    }
    (*a).n_out = n;
    (*a).merges = len.wrapping_sub(n);
    return n;
}
