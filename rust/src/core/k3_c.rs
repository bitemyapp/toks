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
pub const TOKS_C_BASE_MASK: ::core::ffi::c_uint = 0x7 as ::core::ffi::c_uint;
pub const TOKS_C_P: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
pub const TOKS_C_L: ::core::ffi::c_uint = 1 as ::core::ffi::c_uint;
pub const TOKS_C_N: ::core::ffi::c_uint = 2 as ::core::ffi::c_uint;
pub const TOKS_C_WS: ::core::ffi::c_uint = 3 as ::core::ffi::c_uint;
pub const TOKS_C_NL: ::core::ffi::c_uint = 4 as ::core::ffi::c_uint;
pub const TOKS_C_FOLD_S: ::core::ffi::c_uint = 0x40 as ::core::ffi::c_uint;
pub const TOKS_C_HAN: ::core::ffi::c_uint = 0x80 as ::core::ffi::c_uint;
pub const TOKS_TP_CONTR_MASK: ::core::ffi::c_uint = 0x3 as ::core::ffi::c_uint;
pub const TOKS_TP_CONTR_NONE: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
pub const TOKS_TP_CONTR_CS: ::core::ffi::c_uint = 0x1 as ::core::ffi::c_uint;
pub const TOKS_TP_LPREFIX_ANY: ::core::ffi::c_uint = 0x4 as ::core::ffi::c_uint;
pub const TOKS_TP_DIGITS_MASK: ::core::ffi::c_uint = 0x18 as ::core::ffi::c_uint;
pub const TOKS_TP_DIGITS_1_3: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
pub const TOKS_TP_DIGITS_1: ::core::ffi::c_uint = 0x8 as ::core::ffi::c_uint;
pub const TOKS_TP_DIGITS_SP_RUN: ::core::ffi::c_uint = 0x10 as ::core::ffi::c_uint;
pub const TOKS_TP_PUNCT_NL: ::core::ffi::c_uint = 0x20 as ::core::ffi::c_uint;
pub const TOKS_TP_WS_NL: ::core::ffi::c_uint = 0x40 as ::core::ffi::c_uint;
pub const TOKS_TP_DIGIT_CUT: ::core::ffi::c_uint = 0x200 as ::core::ffi::c_uint;
pub const TOKS_TP_NL_CUT: ::core::ffi::c_uint = 0x400 as ::core::ffi::c_uint;
pub const TOKS_TP_GB_SP: ::core::ffi::c_uint = 0x800 as ::core::ffi::c_uint;
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
#[inline]
unsafe extern "C" fn toks_k3_atom(
    mut t: *const toks_tables,
    mut p: *const uint8_t,
    mut avail: uint64_t,
    mut cp: *mut uint32_t,
    mut k: *mut uint32_t,
    mut strip: uint8_t,
) -> uint8_t {
    let mut n: uint32_t = toks_utf8_len(p, avail);
    let mut c: uint8_t = 0;
    *cp = *p.offset(0 as ::core::ffi::c_int as isize) as uint32_t;
    *k = if n != 0 as uint32_t { n } else { 1 as uint32_t };
    if n <= 1 as uint32_t {
        return (if n == 0 as uint32_t {
            TOKS_C_P as uint8_t as ::core::ffi::c_int
        } else {
            *(*t).cls_ascii.offset(*p.offset(0 as ::core::ffi::c_int as isize) as isize)
                as ::core::ffi::c_int
        }) as uint8_t;
    }
    *cp = toks_cp_decode(p, n);
    c = toks_cls_cp(t, *cp);
    return (if c as ::core::ffi::c_uint & TOKS_C_HAN != 0 as ::core::ffi::c_uint {
        (c as ::core::ffi::c_int & !(strip as ::core::ffi::c_int)) as uint8_t
            as ::core::ffi::c_int
    } else {
        c as ::core::ffi::c_int
    }) as uint8_t;
}
#[inline]
unsafe extern "C" fn toks_k3_run(
    mut t: *const toks_tables,
    mut text: *const uint8_t,
    mut len: uint64_t,
    mut from: uint64_t,
    mut b: uint8_t,
    mut strip: uint8_t,
) -> uint64_t {
    let mut e: uint64_t = from;
    while e < len {
        let mut cp: uint32_t = 0;
        let mut k: uint32_t = 0;
        if toks_k3_atom(
            t,
            text.offset(e as isize),
            len.wrapping_sub(e),
            &raw mut cp,
            &raw mut k,
            strip,
        ) as ::core::ffi::c_uint & TOKS_C_BASE_MASK != b as ::core::ffi::c_uint
        {
            break;
        }
        e = e.wrapping_add(k as uint64_t);
    }
    return e;
}
#[inline]
unsafe extern "C" fn toks_k3_n13(
    mut t: *const toks_tables,
    mut text: *const uint8_t,
    mut len: uint64_t,
    mut p1: uint64_t,
    mut strip: uint8_t,
) -> uint64_t {
    let mut e: uint64_t = p1;
    let mut c: uint32_t = 1 as uint32_t;
    while c < 3 as uint32_t && e < len {
        let mut cp: uint32_t = 0;
        let mut k: uint32_t = 0;
        if toks_k3_atom(
            t,
            text.offset(e as isize),
            len.wrapping_sub(e),
            &raw mut cp,
            &raw mut k,
            strip,
        ) as ::core::ffi::c_uint & TOKS_C_BASE_MASK != TOKS_C_N
        {
            break;
        }
        e = e.wrapping_add(k as uint64_t);
        c = c.wrapping_add(1);
    }
    return e;
}
#[inline]
unsafe extern "C" fn toks_k3_nlcut(
    mut x: *const uint8_t,
    mut len: uint64_t,
    mut q: uint64_t,
) -> ::core::ffi::c_int {
    return (*x.offset(q.wrapping_sub(1 as uint64_t) as isize) as ::core::ffi::c_uint
        != 0xa as ::core::ffi::c_uint
        && (if *x.offset(q as isize) as ::core::ffi::c_uint == 0xa as ::core::ffi::c_uint
        {
            (*x.offset(q.wrapping_sub(1 as uint64_t) as isize) as ::core::ffi::c_uint
                != 0xd as ::core::ffi::c_uint) as ::core::ffi::c_int
        } else {
            (*x.offset(q as isize) as ::core::ffi::c_uint == 0xd as ::core::ffi::c_uint
                && q.wrapping_add(1 as uint64_t) < len
                && *x.offset(q.wrapping_add(1 as uint64_t) as isize)
                    as ::core::ffi::c_uint == 0xa as ::core::ffi::c_uint)
                as ::core::ffi::c_int
        }) != 0) as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn toks_k3_ws(
    mut t: *const toks_tables,
    mut text: *const uint8_t,
    mut len: uint64_t,
    mut pos: uint64_t,
    mut ws_nl: ::core::ffi::c_int,
    mut strip: uint8_t,
    mut p: uint32_t,
) -> uint64_t {
    let mut e: uint64_t = pos;
    let mut last_nl: uint64_t = 0 as uint64_t;
    let mut last_start: uint64_t = pos;
    let mut c: uint8_t = 0 as uint8_t;
    let mut b: uint8_t = 0 as uint8_t;
    while e < len {
        let mut cp: uint32_t = 0;
        let mut k: uint32_t = 0;
        if p & TOKS_TP_NL_CUT as uint32_t != 0 as uint32_t && e > pos
            && toks_k3_nlcut(text, len, e) != 0
        {
            len = e;
            break;
        } else {
            c = toks_k3_atom(
                t,
                text.offset(e as isize),
                len.wrapping_sub(e),
                &raw mut cp,
                &raw mut k,
                strip,
            );
            b = (c as ::core::ffi::c_uint & TOKS_C_BASE_MASK) as uint8_t;
            if b as ::core::ffi::c_uint != TOKS_C_WS
                && b as ::core::ffi::c_uint != TOKS_C_NL
            {
                break;
            }
            if b as ::core::ffi::c_uint == TOKS_C_NL {
                last_nl = e.wrapping_add(k as uint64_t);
            }
            last_start = e;
            e = e.wrapping_add(k as uint64_t);
        }
    }
    if ws_nl != 0 && last_nl != 0 as uint64_t {
        return last_nl;
    }
    return if e == len || last_start == pos
        || p & TOKS_TP_DIGIT_CUT as uint32_t != 0 as uint32_t
            && b as ::core::ffi::c_uint == TOKS_C_N
        || c as uint32_t & p & 0x80 as uint32_t != 0 as uint32_t
        || p & TOKS_TP_GB_SP as uint32_t != 0 as uint32_t
            && *text.offset(last_start as isize) as ::core::ffi::c_uint
                != 0x20 as ::core::ffi::c_uint
    {
        e
    } else {
        last_start
    };
}
#[no_mangle]
pub unsafe extern "C" fn toks_k3_scan_cl100k_c(
    mut t: *const toks_tables,
    mut a: *mut toks_k3_args,
) -> uint64_t {
    let mut current_block: u64;
    let mut text: *const uint8_t = (*a).text;
    let mut len: uint64_t = (*a).len;
    let mut pos: uint64_t = (*a).pos;
    let mut cap: uint64_t = (*a).cap;
    let mut ends: *mut uint32_t = (*a).ends;
    let mut params: uint32_t = (*t).tmpl_params;
    let mut contr: uint32_t = params & TOKS_TP_CONTR_MASK as uint32_t;
    let mut lprefix_any: ::core::ffi::c_int = (params & TOKS_TP_LPREFIX_ANY as uint32_t
        != 0 as uint32_t) as ::core::ffi::c_int;
    let mut digits: uint32_t = params & TOKS_TP_DIGITS_MASK as uint32_t;
    let mut punct_nl: ::core::ffi::c_int = (params & TOKS_TP_PUNCT_NL as uint32_t
        != 0 as uint32_t) as ::core::ffi::c_int;
    let mut ws_nl: ::core::ffi::c_int = (params & TOKS_TP_WS_NL as uint32_t
        != 0 as uint32_t) as ::core::ffi::c_int;
    let mut nl_cut: ::core::ffi::c_int = (params & TOKS_TP_NL_CUT as uint32_t
        != 0 as uint32_t) as ::core::ffi::c_int;
    let mut n: uint64_t = 0 as uint64_t;
    while pos < len && n < cap {
        let mut cp: uint32_t = 0;
        let mut k1: uint32_t = 0;
        let mut bc: uint8_t = toks_k3_atom(
            t,
            text.offset(pos as isize),
            len.wrapping_sub(pos),
            &raw mut cp,
            &raw mut k1,
            0 as uint8_t,
        );
        let mut bb: uint8_t = (bc as ::core::ffi::c_uint & TOKS_C_BASE_MASK) as uint8_t;
        let mut p1: uint64_t = pos.wrapping_add(k1 as uint64_t);
        let mut end: uint64_t = pos;
        let mut cp1: uint32_t = 0 as uint32_t;
        let mut l1: uint32_t = 0 as uint32_t;
        let mut bc1: uint8_t = 0 as uint8_t;
        let mut bb1: uint8_t = 0xff as uint8_t;
        if p1 < len
            && (contr != TOKS_TP_CONTR_NONE as uint32_t && cp == 0x27 as uint32_t
                || lprefix_any != 0
                    && (bb as ::core::ffi::c_uint == TOKS_C_P
                        || bb as ::core::ffi::c_uint == TOKS_C_WS)
                || cp == 0x20 as uint32_t)
        {
            bc1 = toks_k3_atom(
                t,
                text.offset(p1 as isize),
                len.wrapping_sub(p1),
                &raw mut cp1,
                &raw mut l1,
                0 as uint8_t,
            );
            bb1 = (bc1 as ::core::ffi::c_uint & TOKS_C_BASE_MASK) as uint8_t;
        }
        if contr != TOKS_TP_CONTR_NONE as uint32_t && cp == 0x27 as uint32_t && p1 < len
        {
            if contr == TOKS_TP_CONTR_CS as uint32_t {
                if cp1 == 0x73 as uint32_t || cp1 == 0x74 as uint32_t
                    || cp1 == 0x6d as uint32_t || cp1 == 0x64 as uint32_t
                {
                    end = p1.wrapping_add(l1 as uint64_t);
                    current_block = 3869124590432637393;
                } else if p1.wrapping_add(l1 as uint64_t) < len {
                    let mut cp2: uint32_t = 0;
                    let mut l2: uint32_t = 0;
                    toks_k3_atom(
                        t,
                        text.offset(p1 as isize).offset(l1 as isize),
                        len.wrapping_sub(p1).wrapping_sub(l1 as uint64_t),
                        &raw mut cp2,
                        &raw mut l2,
                        0 as uint8_t,
                    );
                    if cp1 == 0x72 as uint32_t && cp2 == 0x65 as uint32_t
                        || cp1 == 0x76 as uint32_t && cp2 == 0x65 as uint32_t
                        || cp1 == 0x6c as uint32_t && cp2 == 0x6c as uint32_t
                    {
                        end = p1
                            .wrapping_add(l1 as uint64_t)
                            .wrapping_add(l2 as uint64_t);
                        current_block = 3869124590432637393;
                    } else {
                        current_block = 14818589718467733107;
                    }
                } else {
                    current_block = 14818589718467733107;
                }
            } else if cp1 == 0x73 as uint32_t || cp1 == 0x53 as uint32_t
                || cp1 == 0x74 as uint32_t || cp1 == 0x54 as uint32_t
                || cp1 == 0x6d as uint32_t || cp1 == 0x4d as uint32_t
                || cp1 == 0x64 as uint32_t || cp1 == 0x44 as uint32_t
                || bc1 as ::core::ffi::c_uint & TOKS_C_FOLD_S != 0 as ::core::ffi::c_uint
            {
                end = p1.wrapping_add(l1 as uint64_t);
                current_block = 3869124590432637393;
            } else if p1.wrapping_add(l1 as uint64_t) < len {
                let mut cp2_0: uint32_t = 0;
                let mut l2_0: uint32_t = 0;
                toks_k3_atom(
                    t,
                    text.offset(p1 as isize).offset(l1 as isize),
                    len.wrapping_sub(p1).wrapping_sub(l1 as uint64_t),
                    &raw mut cp2_0,
                    &raw mut l2_0,
                    0 as uint8_t,
                );
                if (cp1 == 0x72 as uint32_t || cp1 == 0x52 as uint32_t)
                    && (cp2_0 == 0x65 as uint32_t || cp2_0 == 0x45 as uint32_t)
                    || (cp1 == 0x76 as uint32_t || cp1 == 0x56 as uint32_t)
                        && (cp2_0 == 0x65 as uint32_t || cp2_0 == 0x45 as uint32_t)
                    || (cp1 == 0x6c as uint32_t || cp1 == 0x4c as uint32_t)
                        && (cp2_0 == 0x6c as uint32_t || cp2_0 == 0x4c as uint32_t)
                {
                    end = p1.wrapping_add(l1 as uint64_t).wrapping_add(l2_0 as uint64_t);
                    current_block = 3869124590432637393;
                } else {
                    current_block = 14818589718467733107;
                }
            } else {
                current_block = 14818589718467733107;
            }
        } else {
            current_block = 14818589718467733107;
        }
        match current_block {
            14818589718467733107 => {
                if bb as ::core::ffi::c_uint == TOKS_C_L {
                    end = toks_k3_run(
                        t,
                        text,
                        len,
                        p1,
                        TOKS_C_L as uint8_t,
                        0 as uint8_t,
                    );
                } else if bb1 as ::core::ffi::c_uint == TOKS_C_L
                    && (if lprefix_any != 0 {
                        (bb as ::core::ffi::c_uint == TOKS_C_P
                            || bb as ::core::ffi::c_uint == TOKS_C_WS)
                            as ::core::ffi::c_int
                    } else {
                        (cp == 0x20 as uint32_t) as ::core::ffi::c_int
                    }) != 0
                {
                    end = toks_k3_run(
                        t,
                        text,
                        len,
                        p1.wrapping_add(l1 as uint64_t),
                        TOKS_C_L as uint8_t,
                        0 as uint8_t,
                    );
                } else if bb as ::core::ffi::c_uint == TOKS_C_N {
                    if digits == TOKS_TP_DIGITS_1_3 as uint32_t {
                        end = toks_k3_n13(t, text, len, p1, 0 as uint8_t);
                    } else if digits == TOKS_TP_DIGITS_1 as uint32_t {
                        end = p1;
                    } else {
                        end = toks_k3_run(
                            t,
                            text,
                            len,
                            p1,
                            TOKS_C_N as uint8_t,
                            0 as uint8_t,
                        );
                    }
                } else if digits == TOKS_TP_DIGITS_SP_RUN as uint32_t
                    && cp == 0x20 as uint32_t && bb1 as ::core::ffi::c_uint == TOKS_C_N
                {
                    end = toks_k3_run(
                        t,
                        text,
                        len,
                        p1.wrapping_add(l1 as uint64_t),
                        TOKS_C_N as uint8_t,
                        0 as uint8_t,
                    );
                } else {
                    if bb as ::core::ffi::c_uint == TOKS_C_P {
                        end = toks_k3_run(
                            t,
                            text,
                            len,
                            p1,
                            TOKS_C_P as uint8_t,
                            0 as uint8_t,
                        );
                        current_block = 1345366029464561491;
                    } else if cp == 0x20 as uint32_t
                        && bb1 as ::core::ffi::c_uint == TOKS_C_P
                    {
                        end = toks_k3_run(
                            t,
                            text,
                            len,
                            p1.wrapping_add(l1 as uint64_t),
                            TOKS_C_P as uint8_t,
                            0 as uint8_t,
                        );
                        current_block = 1345366029464561491;
                    } else {
                        end = toks_k3_ws(t, text, len, pos, ws_nl, 0 as uint8_t, params);
                        current_block = 3869124590432637393;
                    }
                    match current_block {
                        3869124590432637393 => {}
                        _ => {
                            if punct_nl != 0 {
                                let mut q: uint64_t = end;
                                end = toks_k3_run(
                                    t,
                                    text,
                                    len,
                                    end,
                                    TOKS_C_NL as uint8_t,
                                    0 as uint8_t,
                                );
                                while nl_cut != 0 && q < end
                                    && toks_k3_nlcut(text, len, q) == 0
                                {
                                    q = q.wrapping_add(1);
                                }
                                end = if nl_cut != 0 { q } else { end };
                            }
                        }
                    }
                }
            }
            _ => {}
        }
        if end <= pos {
            end = p1;
        }
        *ends.offset(n as isize) = end as uint32_t;
        n = n.wrapping_add(1);
        pos = end;
    }
    (*a).n = n;
    if n != 0 as uint64_t {
        (*a).pos = *ends.offset(n.wrapping_sub(1 as uint64_t) as isize) as uint64_t;
    }
    return n;
}
