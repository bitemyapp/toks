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
pub const TOKS_C_X: ::core::ffi::c_uint = 5 as ::core::ffi::c_uint;
pub const TOKS_C_HAN: ::core::ffi::c_uint = 0x80 as ::core::ffi::c_uint;
pub const TOKS_C_CJK: ::core::ffi::c_uint = 0x80 as ::core::ffi::c_uint;
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
#[inline]
unsafe extern "C" fn toks_d3_atom(
    mut t: *const toks_tables,
    mut p: *const uint8_t,
    mut avail: uint64_t,
    mut k: *mut uint32_t,
) -> uint8_t {
    let mut cp: uint32_t = 0;
    return toks_k3_atom(t, p, avail, &raw mut cp, k, 0 as uint8_t);
}
unsafe extern "C" fn toks_d3_run(
    mut t: *const toks_tables,
    mut text: *const uint8_t,
    mut len: uint64_t,
    mut from: uint64_t,
    mut want: uint8_t,
    mut last: *mut uint64_t,
    mut stop: *mut uint8_t,
) -> uint64_t {
    let mut e: uint64_t = from;
    *last = from;
    *stop = 0xff as uint8_t;
    while e < len {
        let mut k: uint32_t = 0;
        let mut c: uint8_t = toks_d3_atom(
            t,
            text.offset(e as isize),
            len.wrapping_sub(e),
            &raw mut k,
        );
        if c as ::core::ffi::c_int != want as ::core::ffi::c_int {
            *stop = c;
            break;
        } else {
            *last = e;
            e = e.wrapping_add(k as uint64_t);
        }
    }
    return e;
}
#[no_mangle]
pub unsafe extern "C" fn toks_k3_scan_dsv3_c(
    mut t: *const toks_tables,
    mut a: *mut toks_k3_args,
) -> uint64_t {
    let mut text: *const uint8_t = (*a).text;
    let mut asc: *const uint8_t = (*t).cls_ascii;
    let mut len: uint64_t = (*a).len;
    let mut pos: uint64_t = (*a).pos;
    let mut cap: uint64_t = (*a).cap;
    let mut ends: *mut uint32_t = (*a).ends;
    let mut n: uint64_t = 0 as uint64_t;
    while pos < len && n < cap {
        let mut k1: uint32_t = 0;
        let mut l1: uint32_t = 0 as uint32_t;
        let mut c: uint8_t = toks_d3_atom(
            t,
            text.offset(pos as isize),
            len.wrapping_sub(pos),
            &raw mut k1,
        );
        let mut b: uint8_t = (c as ::core::ffi::c_uint & TOKS_C_BASE_MASK) as uint8_t;
        let mut r: uint8_t = (c as ::core::ffi::c_uint & TOKS_C_CJK) as uint8_t;
        let mut p1: uint64_t = pos.wrapping_add(k1 as uint64_t);
        let mut end: uint64_t = 0;
        let mut last: uint64_t = 0;
        let mut c1: uint8_t = 0xff as uint8_t;
        let mut stop: uint8_t = 0;
        if b as ::core::ffi::c_uint == TOKS_C_N {
            end = toks_k3_n13(t, text, len, p1, 0 as uint8_t);
        } else if b as ::core::ffi::c_uint == TOKS_C_L {
            end = toks_d3_run(
                t,
                text,
                len,
                p1,
                (TOKS_C_L | r as ::core::ffi::c_uint) as uint8_t,
                &raw mut last,
                &raw mut stop,
            );
        } else if b as ::core::ffi::c_uint == TOKS_C_P {
            if (*text.offset(pos as isize) as ::core::ffi::c_uint)
                < 0x80 as ::core::ffi::c_uint && p1 < len
                && (*text.offset(p1 as isize) as ::core::ffi::c_uint)
                    < 0x80 as ::core::ffi::c_uint
                && *asc.offset(*text.offset(p1 as isize) as isize) as ::core::ffi::c_uint
                    == TOKS_C_L
            {
                end = p1.wrapping_add(1 as uint64_t);
                while end < len
                    && (*text.offset(end as isize) as ::core::ffi::c_uint)
                        < 0x80 as ::core::ffi::c_uint
                    && *asc.offset(*text.offset(end as isize) as isize)
                        as ::core::ffi::c_uint == TOKS_C_L
                {
                    end = end.wrapping_add(1);
                }
            } else {
                end = toks_d3_run(
                    t,
                    text,
                    len,
                    p1,
                    (TOKS_C_P | r as ::core::ffi::c_uint) as uint8_t,
                    &raw mut last,
                    &raw mut stop,
                );
                end = if r as ::core::ffi::c_uint != 0 as ::core::ffi::c_uint {
                    end
                } else {
                    toks_k3_run(t, text, len, end, TOKS_C_NL as uint8_t, 0 as uint8_t)
                };
            }
        } else {
            if p1 < len {
                c1 = toks_d3_atom(
                    t,
                    text.offset(p1 as isize),
                    len.wrapping_sub(p1),
                    &raw mut l1,
                );
            }
            if b as ::core::ffi::c_uint == TOKS_C_WS
                || b as ::core::ffi::c_uint == TOKS_C_NL
            {
                if b as ::core::ffi::c_uint == TOKS_C_WS
                    && c1 as ::core::ffi::c_uint == TOKS_C_L
                {
                    end = toks_d3_run(
                        t,
                        text,
                        len,
                        p1.wrapping_add(l1 as uint64_t),
                        TOKS_C_L as uint8_t,
                        &raw mut last,
                        &raw mut stop,
                    );
                } else if *text.offset(pos as isize) as ::core::ffi::c_uint
                    == 0x20 as ::core::ffi::c_uint
                    && c1 as ::core::ffi::c_uint == TOKS_C_P
                {
                    end = toks_d3_run(
                        t,
                        text,
                        len,
                        p1.wrapping_add(l1 as uint64_t),
                        TOKS_C_P as uint8_t,
                        &raw mut last,
                        &raw mut stop,
                    );
                    end = toks_k3_run(
                        t,
                        text,
                        len,
                        end,
                        TOKS_C_NL as uint8_t,
                        0 as uint8_t,
                    );
                } else {
                    end = toks_k3_ws(
                        t,
                        text,
                        len,
                        pos,
                        1 as ::core::ffi::c_int,
                        0 as uint8_t,
                        TOKS_TP_DIGIT_CUT as uint32_t | TOKS_C_CJK as uint32_t,
                    );
                }
            } else if c1 as ::core::ffi::c_int
                == (TOKS_C_L | r as ::core::ffi::c_uint) as uint8_t as ::core::ffi::c_int
            {
                end = toks_d3_run(
                    t,
                    text,
                    len,
                    p1.wrapping_add(l1 as uint64_t),
                    (TOKS_C_L | r as ::core::ffi::c_uint) as uint8_t,
                    &raw mut last,
                    &raw mut stop,
                );
            } else {
                end = toks_d3_run(
                    t,
                    text,
                    len,
                    p1,
                    (TOKS_C_X | r as ::core::ffi::c_uint) as uint8_t,
                    &raw mut last,
                    &raw mut stop,
                );
                if stop as ::core::ffi::c_int
                    == (TOKS_C_L | r as ::core::ffi::c_uint) as uint8_t
                        as ::core::ffi::c_int && last > pos
                {
                    end = last;
                }
            }
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
