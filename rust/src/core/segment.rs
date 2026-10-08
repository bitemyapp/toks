extern "C" {
    fn toks_k1_added_find_c(t: *const toks_tables, a: *mut toks_k1_args) -> uint64_t;
    #[cfg_attr(target_arch = "x86_64", link_name = "toks_k1_added_find_avx2")]
    fn toks_k1_added_find_neon(t: *const toks_tables, a: *mut toks_k1_args) -> uint64_t;
    #[cfg_attr(target_arch = "aarch64", link_name = "toks_k1_added_find_neon")]
    fn toks_k1_added_find_avx2(t: *const toks_tables, a: *mut toks_k1_args) -> uint64_t;
    static toks_rx_word: [[uint32_t; 2]; 796];
}
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
pub struct toks_k1_match {
    pub start: uint32_t,
    pub end: uint32_t,
    pub entry: uint32_t,
    pub rsv: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct toks_k1_args {
    pub text: *const uint8_t,
    pub len: uint64_t,
    pub pos: uint64_t,
    pub phase: uint64_t,
    pub m: *mut toks_k1_match,
    pub cap: uint64_t,
    pub n: uint64_t,
    pub next: uint64_t,
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
pub const UINT64_MAX: ::core::ffi::c_ulong = 18446744073709551615
    as ::core::ffi::c_ulong;
pub const TOKS_TIER_SCALAR: ::core::ffi::c_uint = 1 as ::core::ffi::c_uint;
pub const TOKS_TIER_NEON: ::core::ffi::c_uint = 2;
pub const TOKS_TIER_AVX2: ::core::ffi::c_uint = 3;
pub const TOKS_TIER_AVX512: ::core::ffi::c_uint = 4 as ::core::ffi::c_uint;
pub const TOKS_ADDED_NONSPECIAL: ::core::ffi::c_uint = 1 as ::core::ffi::c_uint;
pub const TOKS_AF_SPECIAL: ::core::ffi::c_uint = 0x1 as ::core::ffi::c_uint;
pub const TOKS_AF_LSTRIP: ::core::ffi::c_uint = 0x2 as ::core::ffi::c_uint;
pub const TOKS_AF_RSTRIP: ::core::ffi::c_uint = 0x4 as ::core::ffi::c_uint;
pub const TOKS_AF_SINGLE_WORD: ::core::ffi::c_uint = 0x8 as ::core::ffi::c_uint;
pub const TOKS_AF_PFX: ::core::ffi::c_uint = 0x20 as ::core::ffi::c_uint;
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
pub const TOKS_SEG_TOKEN: ::core::ffi::c_uint = 1 as ::core::ffi::c_uint;
pub const TOKS_SEG_GAP: ::core::ffi::c_uint = 2 as ::core::ffi::c_uint;
pub const TOKS_SEG_BATCH: ::core::ffi::c_uint = 16 as ::core::ffi::c_uint;
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
pub const TOKS_HAVE_K1_AVX512: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn toks_k1(
    mut t: *const toks_tables,
    mut a: *mut toks_k1_args,
    mut tier: uint32_t,
) -> uint64_t {
    match if tier == TOKS_TIER_NEON as uint32_t && TOKS_HAVE_K1_NEON != 0 {
        TOKS_TIER_NEON
    } else if tier == TOKS_TIER_AVX512 as uint32_t && TOKS_HAVE_K1_AVX512 != 0 {
        TOKS_TIER_AVX512
    } else if (tier == TOKS_TIER_AVX512 as uint32_t
        || tier == TOKS_TIER_AVX2 as uint32_t) && TOKS_HAVE_K1_AVX2 != 0
    {
        TOKS_TIER_AVX2
    } else {
        TOKS_TIER_SCALAR
    } {
        TOKS_TIER_NEON => return toks_k1_added_find_neon(t, a),
        TOKS_TIER_AVX2 => return toks_k1_added_find_avx2(t, a),
        _ => return toks_k1_added_find_c(t, a),
    };
}
pub const TOKS_RX_WORD_N: ::core::ffi::c_uint = 796 as ::core::ffi::c_uint;
static mut WS_SET: [uint32_t; 25] = [
    0x9 as ::core::ffi::c_uint,
    0xa as ::core::ffi::c_uint,
    0xb as ::core::ffi::c_uint,
    0xc as ::core::ffi::c_uint,
    0xd as ::core::ffi::c_uint,
    0x20 as ::core::ffi::c_uint,
    0x85 as ::core::ffi::c_uint,
    0xa0 as ::core::ffi::c_uint,
    0x1680 as ::core::ffi::c_uint,
    0x2000 as ::core::ffi::c_uint,
    0x2001 as ::core::ffi::c_uint,
    0x2002 as ::core::ffi::c_uint,
    0x2003 as ::core::ffi::c_uint,
    0x2004 as ::core::ffi::c_uint,
    0x2005 as ::core::ffi::c_uint,
    0x2006 as ::core::ffi::c_uint,
    0x2007 as ::core::ffi::c_uint,
    0x2008 as ::core::ffi::c_uint,
    0x2009 as ::core::ffi::c_uint,
    0x200a as ::core::ffi::c_uint,
    0x2028 as ::core::ffi::c_uint,
    0x2029 as ::core::ffi::c_uint,
    0x202f as ::core::ffi::c_uint,
    0x205f as ::core::ffi::c_uint,
    0x3000 as ::core::ffi::c_uint,
];
#[no_mangle]
pub unsafe extern "C" fn toks_is_regex_ws(mut cp: uint32_t) -> ::core::ffi::c_int {
    let mut lo: uint32_t = 0 as uint32_t;
    let mut hi: uint32_t = 25 as uint32_t;
    while lo < hi {
        let mut mid: uint32_t = lo
            .wrapping_add(hi.wrapping_sub(lo) >> 1 as ::core::ffi::c_int);
        if WS_SET[mid as usize] < cp {
            lo = mid.wrapping_add(1 as uint32_t);
        } else {
            hi = mid;
        }
    }
    return (lo < 25 as uint32_t && WS_SET[lo as usize] == cp) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn toks_is_regex_word(mut cp: uint32_t) -> ::core::ffi::c_int {
    let mut lo: uint32_t = 0 as uint32_t;
    let mut hi: uint32_t = TOKS_RX_WORD_N as uint32_t;
    while lo < hi {
        let mut mid: uint32_t = lo
            .wrapping_add(hi.wrapping_sub(lo) >> 1 as ::core::ffi::c_int);
        if toks_rx_word[mid as usize][1 as ::core::ffi::c_int as usize] < cp {
            lo = mid.wrapping_add(1 as uint32_t);
        } else {
            hi = mid;
        }
    }
    return (lo < TOKS_RX_WORD_N as uint32_t
        && toks_rx_word[lo as usize][0 as ::core::ffi::c_int as usize] <= cp)
        as ::core::ffi::c_int;
}
pub const SEG_NOT_CP: ::core::ffi::c_uint = 0xffffffff as ::core::ffi::c_uint;
unsafe extern "C" fn atom_at(
    mut p: *const uint8_t,
    mut avail: uint64_t,
    mut len: *mut uint32_t,
) -> uint32_t {
    let mut k: uint32_t = toks_utf8_len(p, avail);
    if k == 0 as uint32_t {
        *len = 1 as ::core::ffi::c_uint as uint32_t;
        return SEG_NOT_CP as uint32_t;
    }
    *len = k;
    return if k == 1 as uint32_t {
        *p.offset(0 as ::core::ffi::c_int as isize) as uint32_t
    } else {
        toks_cp_decode(p, k)
    };
}
unsafe extern "C" fn atom_before(
    mut text: *const uint8_t,
    mut end: uint64_t,
    mut len: *mut uint32_t,
) -> uint32_t {
    let mut k: uint64_t = end.wrapping_sub(1 as uint64_t);
    let mut back: uint32_t = 0 as uint32_t;
    while k > 0 as uint64_t
        && *text.offset(k as isize) as ::core::ffi::c_uint & 0xc0 as ::core::ffi::c_uint
            == 0x80 as ::core::ffi::c_uint && back < 3 as uint32_t
    {
        k = k.wrapping_sub(1);
        back = back.wrapping_add(1);
    }
    let mut l: uint32_t = 0 as uint32_t;
    let mut cp: uint32_t = atom_at(
        text.offset(k as isize),
        end.wrapping_sub(k),
        &raw mut l,
    );
    if cp != SEG_NOT_CP as uint32_t && l as uint64_t == end.wrapping_sub(k) {
        *len = l;
        return cp;
    }
    *len = 1 as ::core::ffi::c_uint as uint32_t;
    return SEG_NOT_CP as uint32_t;
}
unsafe extern "C" fn ws_run_before(
    mut text: *const uint8_t,
    mut end: uint64_t,
    mut floor: uint64_t,
) -> uint64_t {
    let mut j: uint64_t = end;
    while j > floor {
        let mut l: uint32_t = 0 as uint32_t;
        let mut cp: uint32_t = atom_before(text, j, &raw mut l);
        if cp == SEG_NOT_CP as uint32_t || toks_is_regex_ws(cp) == 0 {
            break;
        }
        j = j.wrapping_sub(l as uint64_t);
    }
    return j;
}
unsafe extern "C" fn ws_run_after(
    mut text: *const uint8_t,
    mut start: uint64_t,
    mut len: uint64_t,
) -> uint64_t {
    let mut i: uint64_t = start;
    while i < len {
        let mut l: uint32_t = 0 as uint32_t;
        let mut cp: uint32_t = atom_at(
            text.offset(i as isize),
            len.wrapping_sub(i),
            &raw mut l,
        );
        if cp == SEG_NOT_CP as uint32_t || toks_is_regex_ws(cp) == 0 {
            break;
        }
        i = i.wrapping_add(l as uint64_t);
    }
    return i;
}
unsafe extern "C" fn single_word_ok(
    mut text: *const uint8_t,
    mut s: uint64_t,
    mut e: uint64_t,
    mut len: uint64_t,
) -> ::core::ffi::c_int {
    let mut l: uint32_t = 0 as uint32_t;
    if s > 0 as uint64_t {
        let mut cp: uint32_t = atom_before(text, s, &raw mut l);
        if cp != SEG_NOT_CP as uint32_t && toks_is_regex_word(cp) != 0 {
            return 0 as ::core::ffi::c_int;
        }
    }
    if e < len {
        let mut cp_0: uint32_t = atom_at(
            text.offset(e as isize),
            len.wrapping_sub(e),
            &raw mut l,
        );
        if cp_0 != SEG_NOT_CP as uint32_t && toks_is_regex_word(cp_0) != 0 {
            return 0 as ::core::ffi::c_int;
        }
    }
    return 1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn toks_seg_begin(
    mut it: *mut toks_seg_iter,
    mut t: *const toks_tables,
    mut mode: uint32_t,
    mut phase: uint32_t,
    mut tier: uint32_t,
    mut text: *const uint8_t,
    mut len: uint64_t,
) {
    (*it).t = t;
    (*it).mode = mode;
    (*it).phase = phase;
    (*it).tier = tier;
    (*it).text = text;
    (*it).len = len;
    (*it).prev_end = 0 as uint64_t;
    (*it).pos = 0 as uint64_t;
    (*it).fin = 0 as uint32_t;
    (*it).pend = 0 as uint32_t;
    (*it).pend_id = 0 as ::core::ffi::c_uint as uint32_t;
    (*it).pend_start = 0 as uint64_t;
    (*it).pend_end = 0 as uint64_t;
    (*it).mn = 0 as uint32_t;
    (*it).mi = (*it).mn;
    (*it).rs_to = UINT64_MAX as uint64_t;
    (*it).rs_from = (*it).rs_to;
}
#[no_mangle]
pub unsafe extern "C" fn toks_seg_next(
    mut it: *mut toks_seg_iter,
    mut u: *mut toks_seg_out,
) -> ::core::ffi::c_int {
    if (*it).fin != 0 && (*it).pend == 0 {
        return 0 as ::core::ffi::c_int;
    }
    if (*it).pend != 0 {
        (*it).pend = 0 as uint32_t;
        (*u).kind = TOKS_SEG_TOKEN as uint32_t;
        (*u).id = (*it).pend_id;
        (*u).start = (*it).pend_start;
        (*u).end = (*it).pend_end;
        (*it).prev_end = (*it).pend_end;
        return 1 as ::core::ffi::c_int;
    }
    let mut t: *const toks_tables = (*it).t;
    let mut text: *const uint8_t = (*it).text;
    let mut len: uint64_t = (*it).len;
    while (*it).fin == 0
        && (*t).add_phases as ::core::ffi::c_ulonglong
            & (1 as ::core::ffi::c_ulonglong) << (*it).phase
            != 0 as ::core::ffi::c_ulonglong
    {
        if (*it).mi == (*it).mn {
            if (*it).pos >= len {
                break;
            }
            let mut a: toks_k1_args = toks_k1_args {
                text: text,
                len: len,
                pos: (*it).pos,
                phase: (*it).phase as uint64_t,
                m: &raw mut (*it).m as *mut toks_k1_match,
                cap: TOKS_SEG_BATCH as uint64_t,
                n: 0 as uint64_t,
                next: 0 as uint64_t,
            };
            (*it).mn = toks_k1(t, &raw mut a, (*it).tier) as uint32_t;
            (*it).mi = 0 as uint32_t;
            (*it).pos = a.next;
            if (*it).mn == 0 as uint32_t {
                break;
            }
        }
        let fresh0 = (*it).mi;
        (*it).mi = (*it).mi.wrapping_add(1);
        let mut m: *const toks_k1_match = (&raw mut (*it).m as *mut toks_k1_match)
            .offset(fresh0 as isize) as *mut toks_k1_match;
        let mut m_start: uint64_t = (*m).start as uint64_t;
        let mut m_end: uint64_t = (*m).end as uint64_t;
        let mut e: *const toks_added_entry = (*t).add_entries.offset((*m).entry as isize)
            as *const toks_added_entry;
        if !((*it).mode == TOKS_ADDED_NONSPECIAL as uint32_t
            && (*e).flags as ::core::ffi::c_uint & TOKS_AF_SPECIAL
                != 0 as ::core::ffi::c_uint)
        {
            if !((*e).flags as ::core::ffi::c_uint & TOKS_AF_SINGLE_WORD
                != 0 as ::core::ffi::c_uint
                && single_word_ok(text, m_start, m_end, len) == 0)
            {
                if (*e).flags as ::core::ffi::c_uint & TOKS_AF_PFX
                    != 0 as ::core::ffi::c_uint
                {
                    let mut k: uint64_t = if m_start == 0 as uint64_t {
                        0 as uint64_t
                    } else if *text.offset(m_start.wrapping_sub(1 as uint64_t) as isize)
                        as ::core::ffi::c_uint == 0x20 as ::core::ffi::c_uint
                    {
                        1 as uint64_t
                    } else if m_start >= 3 as uint64_t
                        && toks_meta_at(text, len, m_start.wrapping_sub(3 as uint64_t))
                            != 0
                    {
                        3 as uint64_t
                    } else {
                        UINT64_MAX as uint64_t
                    };
                    if k == UINT64_MAX as uint64_t {
                        continue;
                    }
                    m_start = m_start.wrapping_sub(k);
                }
                let mut start: uint64_t = m_start;
                let mut end: uint64_t = m_end;
                if (*e).flags as ::core::ffi::c_uint & TOKS_AF_LSTRIP
                    != 0 as ::core::ffi::c_uint
                {
                    let mut ns: uint64_t = ws_run_before(text, m_start, (*it).prev_end);
                    if ns < (*it).prev_end {
                        ns = (*it).prev_end;
                    }
                    start = ns;
                }
                if (*e).flags as ::core::ffi::c_uint & TOKS_AF_RSTRIP
                    != 0 as ::core::ffi::c_uint
                {
                    if m_end >= (*it).rs_from && m_end <= (*it).rs_to
                        && (m_end == (*it).rs_to
                            || *text.offset(m_end as isize) as ::core::ffi::c_uint
                                & 0xc0 as ::core::ffi::c_uint
                                != 0x80 as ::core::ffi::c_uint)
                    {
                        end = (*it).rs_to;
                    } else {
                        end = ws_run_after(text, m_end, len);
                        (*it).rs_from = m_end;
                        (*it).rs_to = end;
                    }
                }
                if start >= end {
                    continue;
                }
                if (*it).prev_end < start {
                    (*u).kind = TOKS_SEG_GAP as uint32_t;
                    (*u).id = 0 as ::core::ffi::c_uint as uint32_t;
                    (*u).start = (*it).prev_end;
                    (*u).end = start;
                    (*it).pend = 1 as uint32_t;
                    (*it).pend_id = (*e).id;
                    (*it).pend_start = start;
                    (*it).pend_end = end;
                    return 1 as ::core::ffi::c_int;
                }
                (*u).kind = TOKS_SEG_TOKEN as uint32_t;
                (*u).id = (*e).id;
                (*u).start = start;
                (*u).end = end;
                (*it).prev_end = end;
                return 1 as ::core::ffi::c_int;
            }
        }
    }
    (*it).fin = 1 as uint32_t;
    if (*it).prev_end < len {
        (*u).kind = TOKS_SEG_GAP as uint32_t;
        (*u).id = 0 as ::core::ffi::c_uint as uint32_t;
        (*u).start = (*it).prev_end;
        (*u).end = len;
        (*it).prev_end = len;
        return 1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
pub const TOKS_HAVE_K1_NEON: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const TOKS_HAVE_K1_AVX2: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
