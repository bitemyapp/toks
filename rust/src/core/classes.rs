extern "C" {
    static toks_ucd_stage1: [uint16_t; 4352];
    static toks_ucd_stage2: [uint16_t; 39680];
    static toks_han_ranges: [[uint32_t; 2]; 22];
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
pub struct toks_class_tables {
    pub ascii: *const uint8_t,
    pub stage1: *const uint16_t,
    pub stage2: *const uint8_t,
    pub n_blocks: uint32_t,
    pub _pad: uint32_t,
}
pub const TOKS_CLS_P: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
pub const TOKS_CLS_L: ::core::ffi::c_uint = 0x1 as ::core::ffi::c_uint;
pub const TOKS_CLS_N: ::core::ffi::c_uint = 0x2 as ::core::ffi::c_uint;
pub const TOKS_CLS_WS: ::core::ffi::c_uint = 0x3 as ::core::ffi::c_uint;
pub const TOKS_CLS_NL: ::core::ffi::c_uint = 0x4 as ::core::ffi::c_uint;
pub const TOKS_CLS_UPPER: ::core::ffi::c_uint = 0x8 as ::core::ffi::c_uint;
pub const TOKS_CLS_LOWER: ::core::ffi::c_uint = 0x10 as ::core::ffi::c_uint;
pub const TOKS_CLS_MARK: ::core::ffi::c_uint = 0x20 as ::core::ffi::c_uint;
pub const TOKS_CLS_FOLD_S: ::core::ffi::c_uint = 0x40 as ::core::ffi::c_uint;
pub const TOKS_CLS_HAN: ::core::ffi::c_uint = 0x80 as ::core::ffi::c_uint;
pub const TOKS_CLS_X: ::core::ffi::c_uint = 0x5 as ::core::ffi::c_uint;
pub const TOKS_CLS_CJK: ::core::ffi::c_uint = 0x80 as ::core::ffi::c_uint;
pub const TOKS_CLASSES_MARKS_ARE_LETTERS: ::core::ffi::c_uint = 0x1
    as ::core::ffi::c_uint;
pub const TOKS_CLASSES_HAN: ::core::ffi::c_uint = 0x2 as ::core::ffi::c_uint;
pub const TOKS_CLASSES_DSV3: ::core::ffi::c_uint = 0x8 as ::core::ffi::c_uint;
pub const TOKS_CLASSES_DIGITS: ::core::ffi::c_uint = 0x20 as ::core::ffi::c_uint;
pub const TOKS_CLASSES_BLOOM: ::core::ffi::c_uint = 0x40 as ::core::ffi::c_uint;
pub const TOKS_CLASSES_KNOWN: ::core::ffi::c_uint = TOKS_CLASSES_MARKS_ARE_LETTERS
    | TOKS_CLASSES_HAN | TOKS_CLASSES_DSV3 | TOKS_CLASSES_DIGITS | TOKS_CLASSES_BLOOM;
pub const TOKS_CLASSES_E_CAP: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
pub const TOKS_CLASSES_E_FLAGS: ::core::ffi::c_int = -(2 as ::core::ffi::c_int);
pub const TOKS_CLASSES_E_ARG: ::core::ffi::c_int = -(3 as ::core::ffi::c_int);
pub const TOKS_UCD_LU: ::core::ffi::c_uint = 0x1 as ::core::ffi::c_uint;
pub const TOKS_UCD_LL: ::core::ffi::c_uint = 0x2 as ::core::ffi::c_uint;
pub const TOKS_UCD_LT: ::core::ffi::c_uint = 0x4 as ::core::ffi::c_uint;
pub const TOKS_UCD_LM: ::core::ffi::c_uint = 0x8 as ::core::ffi::c_uint;
pub const TOKS_UCD_LO: ::core::ffi::c_uint = 0x10 as ::core::ffi::c_uint;
pub const TOKS_UCD_M: ::core::ffi::c_uint = 0x20 as ::core::ffi::c_uint;
pub const TOKS_UCD_N: ::core::ffi::c_uint = 0x40 as ::core::ffi::c_uint;
pub const TOKS_UCD_WS: ::core::ffi::c_uint = 0x80 as ::core::ffi::c_uint;
pub const TOKS_UCD_P: ::core::ffi::c_uint = 0x200 as ::core::ffi::c_uint;
pub const TOKS_UCD_S: ::core::ffi::c_uint = 0x400 as ::core::ffi::c_uint;
pub const TOKS_UCD_RNUM: ::core::ffi::c_uint = 0x800 as ::core::ffi::c_uint;
pub const TOKS_UCD_FOLD_S: ::core::ffi::c_uint = 0x100 as ::core::ffi::c_uint;
pub const TOKS_UCD_LETTERS: ::core::ffi::c_uint = TOKS_UCD_LU | TOKS_UCD_LL | TOKS_UCD_LT
    | TOKS_UCD_LM | TOKS_UCD_LO;
#[inline]
unsafe extern "C" fn toks_ucd_flags(mut cp: uint32_t) -> uint16_t {
    if cp > 0x10ffff as uint32_t {
        return 0 as uint16_t;
    }
    return toks_ucd_stage2[((toks_ucd_stage1[(cp >> 8 as ::core::ffi::c_int) as usize]
        as uint32_t) << 8 as ::core::ffi::c_int | cp & 0xff as uint32_t) as usize];
}
pub const TOKS_HAN_N: ::core::ffi::c_uint = 22 as ::core::ffi::c_uint;
pub const TOKS_CLS_ASCII_LEN: ::core::ffi::c_uint = 128 as ::core::ffi::c_uint;
pub const TOKS_CLS_STAGE1_LEN: ::core::ffi::c_uint = 0x1100 as ::core::ffi::c_uint;
pub const TOKS_CLS_STAGE1_BYTES: ::core::ffi::c_uint = (2 as ::core::ffi::c_uint)
    .wrapping_mul(TOKS_CLS_STAGE1_LEN);
pub const TOKS_CLS_BLOCK: ::core::ffi::c_uint = 256 as ::core::ffi::c_uint;
pub const TOKS_DSV3_CJK_N: ::core::ffi::c_uint = 2 as ::core::ffi::c_uint;
static mut TOKS_DSV3_CJK: [[uint32_t; 2]; 2] = [
    [0x3040 as ::core::ffi::c_uint, 0x30ff as ::core::ffi::c_uint],
    [0x4e00 as ::core::ffi::c_uint, 0x9fa5 as ::core::ffi::c_uint],
];
static mut TOKS_BLOOM_E: [uint32_t; 14] = [
    '!' as i32 as uint32_t,
    '(' as i32 as uint32_t,
    ')' as i32 as uint32_t,
    ',' as i32 as uint32_t,
    '.' as i32 as uint32_t,
    '?' as i32 as uint32_t,
    '|' as i32 as uint32_t,
    0x60c as ::core::ffi::c_uint,
    0x6d4 as ::core::ffi::c_uint,
    0x964 as ::core::ffi::c_uint,
    0x2026 as ::core::ffi::c_uint,
    0x3001 as ::core::ffi::c_uint,
    0x3002 as ::core::ffi::c_uint,
    0xff0c as ::core::ffi::c_uint,
];
unsafe extern "C" fn toks_class_from_flags(
    mut f: uint16_t,
    mut cp: uint32_t,
    mut marks_are_letters: ::core::ffi::c_int,
) -> uint8_t {
    let mut b: uint8_t = 0;
    if f as ::core::ffi::c_uint & TOKS_UCD_LETTERS != 0 as ::core::ffi::c_uint
        || marks_are_letters != 0 as ::core::ffi::c_int
            && f as ::core::ffi::c_uint & TOKS_UCD_M != 0 as ::core::ffi::c_uint
    {
        b = TOKS_CLS_L as uint8_t;
    } else if f as ::core::ffi::c_uint & TOKS_UCD_N != 0 as ::core::ffi::c_uint {
        b = TOKS_CLS_N as uint8_t;
    } else if cp == 0xa as uint32_t || cp == 0xd as uint32_t {
        b = TOKS_CLS_NL as uint8_t;
    } else if f as ::core::ffi::c_uint & TOKS_UCD_WS != 0 as ::core::ffi::c_uint {
        b = TOKS_CLS_WS as uint8_t;
    } else {
        b = TOKS_CLS_P as uint8_t;
    }
    if f as ::core::ffi::c_uint
        & (TOKS_UCD_LU | TOKS_UCD_LT | TOKS_UCD_LM | TOKS_UCD_LO | TOKS_UCD_M)
        != 0 as ::core::ffi::c_uint
    {
        b = (b as ::core::ffi::c_uint | TOKS_CLS_UPPER) as uint8_t;
    }
    if f as ::core::ffi::c_uint & (TOKS_UCD_LL | TOKS_UCD_LM | TOKS_UCD_LO | TOKS_UCD_M)
        != 0 as ::core::ffi::c_uint
    {
        b = (b as ::core::ffi::c_uint | TOKS_CLS_LOWER) as uint8_t;
    }
    if f as ::core::ffi::c_uint & TOKS_UCD_M != 0 as ::core::ffi::c_uint {
        b = (b as ::core::ffi::c_uint | TOKS_CLS_MARK) as uint8_t;
    }
    if f as ::core::ffi::c_uint & TOKS_UCD_FOLD_S != 0 as ::core::ffi::c_uint {
        b = (b as ::core::ffi::c_uint | TOKS_CLS_FOLD_S) as uint8_t;
    }
    return b;
}
unsafe extern "C" fn toks_class_dsv3(mut f: uint16_t, mut cp: uint32_t) -> uint8_t {
    let mut b: uint8_t = 0;
    let mut r: uint32_t = 0;
    if f as ::core::ffi::c_uint & (TOKS_UCD_LETTERS | TOKS_UCD_M)
        != 0 as ::core::ffi::c_uint
    {
        b = TOKS_CLS_L as uint8_t;
    } else if f as ::core::ffi::c_uint & TOKS_UCD_N != 0 as ::core::ffi::c_uint {
        b = TOKS_CLS_N as uint8_t;
    } else if cp == 0xa as uint32_t || cp == 0xd as uint32_t {
        b = TOKS_CLS_NL as uint8_t;
    } else if f as ::core::ffi::c_uint & TOKS_UCD_WS != 0 as ::core::ffi::c_uint {
        b = TOKS_CLS_WS as uint8_t;
    } else if f as ::core::ffi::c_uint & (TOKS_UCD_P | TOKS_UCD_S)
        != 0 as ::core::ffi::c_uint
    {
        b = TOKS_CLS_P as uint8_t;
    } else {
        b = TOKS_CLS_X as uint8_t;
    }
    r = 0 as uint32_t;
    while r < TOKS_DSV3_CJK_N as uint32_t {
        if cp >= TOKS_DSV3_CJK[r as usize][0 as ::core::ffi::c_int as usize]
            && cp <= TOKS_DSV3_CJK[r as usize][1 as ::core::ffi::c_int as usize]
        {
            b = (b as ::core::ffi::c_uint | TOKS_CLS_CJK) as uint8_t;
        }
        r = r.wrapping_add(1);
    }
    return b;
}
unsafe extern "C" fn toks_block_differs(
    mut a: *const uint8_t,
    mut b: *const uint8_t,
) -> uint32_t {
    let mut i: uint32_t = 0;
    i = 0 as uint32_t;
    while i < TOKS_CLS_BLOCK as uint32_t {
        if *a.offset(i as isize) as ::core::ffi::c_int
            != *b.offset(i as isize) as ::core::ffi::c_int
        {
            return i.wrapping_add(1 as uint32_t);
        }
        i = i.wrapping_add(1);
    }
    return 0 as uint32_t;
}
#[no_mangle]
pub unsafe extern "C" fn toks_classes_bytes(mut class_flags: uint32_t) -> uint64_t {
    if class_flags & !(TOKS_CLASSES_KNOWN as uint32_t) != 0 as uint32_t {
        return 0 as uint64_t;
    }
    return (TOKS_CLS_ASCII_LEN as uint64_t)
        .wrapping_add(TOKS_CLS_STAGE1_BYTES as uint64_t)
        .wrapping_add(
            (TOKS_CLS_STAGE1_LEN as uint64_t).wrapping_mul(TOKS_CLS_BLOCK as uint64_t),
        );
}
#[no_mangle]
pub unsafe extern "C" fn toks_classes_build(
    mut class_flags: uint32_t,
    mut buf: *mut uint8_t,
    mut buf_len: uint64_t,
    mut out: *mut toks_class_tables,
) -> ::core::ffi::c_int {
    let mut s1: *mut uint16_t = ::core::ptr::null_mut::<uint16_t>();
    let mut s2: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    let mut s2_off: uint64_t = 0;
    let mut hi: uint32_t = 0;
    let mut hr: uint32_t = 0;
    let mut marks_are_letters: ::core::ffi::c_int = 0;
    let mut han: ::core::ffi::c_int = 0;
    let mut dsv3: ::core::ffi::c_int = 0;
    let mut bloom: ::core::ffi::c_int = 0;
    let mut rnum: uint16_t = 0;
    if out.is_null() {
        return TOKS_CLASSES_E_ARG;
    }
    if class_flags & !(TOKS_CLASSES_KNOWN as uint32_t) != 0 as uint32_t {
        return TOKS_CLASSES_E_FLAGS;
    }
    marks_are_letters = (class_flags & TOKS_CLASSES_MARKS_ARE_LETTERS as uint32_t
        != 0 as uint32_t) as ::core::ffi::c_int;
    han = (class_flags & TOKS_CLASSES_HAN as uint32_t != 0 as uint32_t)
        as ::core::ffi::c_int;
    hr = 0 as ::core::ffi::c_uint as uint32_t;
    dsv3 = (class_flags & TOKS_CLASSES_DSV3 as uint32_t != 0 as uint32_t)
        as ::core::ffi::c_int;
    bloom = (class_flags & TOKS_CLASSES_BLOOM as uint32_t != 0 as uint32_t)
        as ::core::ffi::c_int;
    rnum = (if class_flags & TOKS_CLASSES_DIGITS as uint32_t != 0 as uint32_t {
        TOKS_UCD_RNUM
    } else {
        0 as ::core::ffi::c_uint
    }) as uint16_t;
    if buf.is_null() || buf_len < toks_classes_bytes(class_flags) {
        return TOKS_CLASSES_E_CAP;
    }
    s1 = buf.offset(TOKS_CLS_ASCII_LEN as isize) as *mut ::core::ffi::c_void
        as *mut uint16_t;
    s2 = buf.offset(TOKS_CLS_ASCII_LEN as isize).offset(TOKS_CLS_STAGE1_BYTES as isize);
    s2_off = 0 as uint64_t;
    hi = 0 as uint32_t;
    while hi < TOKS_CLS_STAGE1_LEN as uint32_t {
        let mut row: [uint8_t; 256] = [0; 256];
        let mut j: uint32_t = 0;
        let mut k: uint64_t = 0;
        let mut found: ::core::ffi::c_int = 0;
        j = 0 as uint32_t;
        while j < TOKS_CLS_BLOCK as uint32_t {
            let mut cp: uint32_t = hi
                .wrapping_mul(TOKS_CLS_BLOCK as uint32_t)
                .wrapping_add(j);
            let mut f: uint16_t = toks_ucd_flags(cp);
            f = (if (f as ::core::ffi::c_int & rnum as ::core::ffi::c_int)
                as ::core::ffi::c_uint != 0 as ::core::ffi::c_uint
            {
                f as ::core::ffi::c_uint | TOKS_UCD_N
            } else {
                f as ::core::ffi::c_uint
            }) as uint16_t;
            row[j as usize] = (if dsv3 != 0 as ::core::ffi::c_int {
                toks_class_dsv3(f, cp) as ::core::ffi::c_int
            } else {
                toks_class_from_flags(f, cp, marks_are_letters) as ::core::ffi::c_int
            }) as uint8_t;
            if bloom != 0 as ::core::ffi::c_int {
                let mut b: uint8_t = (row[j as usize] as ::core::ffi::c_uint
                    & 7 as ::core::ffi::c_uint) as uint8_t;
                row[j as usize] = (if b as ::core::ffi::c_uint == TOKS_CLS_WS
                    || b as ::core::ffi::c_uint == TOKS_CLS_NL
                {
                    b as ::core::ffi::c_uint
                } else {
                    TOKS_CLS_P
                }) as uint8_t;
                let mut e: uint32_t = 0 as uint32_t;
                while cp < 0x10000 as uint32_t && e < 14 as uint32_t {
                    row[j as usize] = (if cp == TOKS_BLOOM_E[e as usize] {
                        TOKS_CLS_WS
                    } else {
                        row[j as usize] as ::core::ffi::c_uint
                    }) as uint8_t;
                    e = e.wrapping_add(1);
                }
            }
            if han != 0 as ::core::ffi::c_int {
                while hr < TOKS_HAN_N as uint32_t
                    && cp
                        > toks_han_ranges[hr as usize][1 as ::core::ffi::c_int as usize]
                {
                    hr = hr.wrapping_add(1);
                }
                if hr < TOKS_HAN_N as uint32_t
                    && cp
                        >= toks_han_ranges[hr as usize][0 as ::core::ffi::c_int as usize]
                {
                    row[j as usize] = (row[j as usize] as ::core::ffi::c_uint
                        | TOKS_CLS_HAN) as uint8_t;
                }
            }
            j = j.wrapping_add(1);
        }
        found = 0 as ::core::ffi::c_int;
        k = 0 as uint64_t;
        while k < s2_off {
            if toks_block_differs(s2.offset(k as isize), &raw mut row as *mut uint8_t)
                == 0 as uint32_t
            {
                *s1.offset(hi as isize) = k.wrapping_div(TOKS_CLS_BLOCK as uint64_t)
                    as uint16_t;
                found = 1 as ::core::ffi::c_int;
                break;
            } else {
                k = k.wrapping_add(TOKS_CLS_BLOCK as uint64_t);
            }
        }
        if found == 0 as ::core::ffi::c_int {
            j = 0 as uint32_t;
            while j < TOKS_CLS_BLOCK as uint32_t {
                *s2.offset(s2_off.wrapping_add(j as uint64_t) as isize) = row[j
                    as usize];
                j = j.wrapping_add(1);
            }
            *s1.offset(hi as isize) = s2_off.wrapping_div(TOKS_CLS_BLOCK as uint64_t)
                as uint16_t;
            s2_off = s2_off.wrapping_add(TOKS_CLS_BLOCK as uint64_t);
        }
        hi = hi.wrapping_add(1);
    }
    hi = 0 as uint32_t;
    while hi < TOKS_CLS_ASCII_LEN as uint32_t {
        *buf.offset(hi as isize) = *s2.offset(hi as isize);
        hi = hi.wrapping_add(1);
    }
    (*out).ascii = buf;
    (*out).stage1 = s1;
    (*out).stage2 = s2;
    (*out).n_blocks = s2_off.wrapping_div(TOKS_CLS_BLOCK as uint64_t) as uint32_t;
    (*out)._pad = 0 as ::core::ffi::c_uint as uint32_t;
    return (TOKS_CLS_ASCII_LEN.wrapping_add(TOKS_CLS_STAGE1_BYTES) as uint64_t)
        .wrapping_add(s2_off) as ::core::ffi::c_int;
}
