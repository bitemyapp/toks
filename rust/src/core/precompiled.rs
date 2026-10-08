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
    static TOKS_CRC32C_TAB: [uint32_t; 256];
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
pub struct toks_ext {
    pub pad: uint32_t,
    pub align: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct toks_arena {
    pub base: *mut uint8_t,
    pub len: uint64_t,
    pub pos: uint64_t,
}
pub type C2RustUnnamed = ::core::ffi::c_uint;
pub const TOKS_GC_ZWJ: C2RustUnnamed = 15;
pub const TOKS_GC_V: C2RustUnnamed = 14;
pub const TOKS_GC_T: C2RustUnnamed = 13;
pub const TOKS_GC_SPACINGMARK: C2RustUnnamed = 12;
pub const TOKS_GC_RI: C2RustUnnamed = 11;
pub const TOKS_GC_PREPEND: C2RustUnnamed = 10;
pub const TOKS_GC_LVT: C2RustUnnamed = 9;
pub const TOKS_GC_LV: C2RustUnnamed = 8;
pub const TOKS_GC_LF: C2RustUnnamed = 7;
pub const TOKS_GC_L: C2RustUnnamed = 6;
pub const TOKS_GC_INCB_CONSONANT: C2RustUnnamed = 5;
pub const TOKS_GC_EXTPICT: C2RustUnnamed = 4;
pub const TOKS_GC_EXTEND: C2RustUnnamed = 3;
pub const TOKS_GC_CONTROL: C2RustUnnamed = 2;
pub const TOKS_GC_CR: C2RustUnnamed = 1;
pub const TOKS_GC_ANY: C2RustUnnamed = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct toks_gc_state {
    pub prev: uint8_t,
    pub ri_odd: uint8_t,
    pub ep_run: uint8_t,
    pub ep_zwj: uint8_t,
    pub incb_cons: uint8_t,
    pub incb_link: uint8_t,
    pub rsv: [uint8_t; 2],
}
pub type C2RustUnnamed_0 = ::core::ffi::c_uint;
pub const TOKS_PC_VALUE: C2RustUnnamed_0 = 1;
pub const TOKS_PC_IDENT: C2RustUnnamed_0 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pc_blob {
    pub units: *const uint8_t,
    pub n_units: uint32_t,
    pub str_0: *const uint8_t,
    pub str_len: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pc_count {
    pub n_keys: uint32_t,
    pub n_single: uint32_t,
    pub n_multi: uint32_t,
    pub n_blocks: uint32_t,
    pub max_expand: uint32_t,
    pub block_used: [uint8_t; 4352],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pc_frame {
    pub node: uint32_t,
    pub next_label: uint32_t,
    pub depth: uint32_t,
    pub dead: uint32_t,
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
pub const TOKS_E_UNSUPPORTED: ::core::ffi::c_int = -(3 as ::core::ffi::c_int);
pub const TOKS_E_NOMEM: ::core::ffi::c_int = -(11 as ::core::ffi::c_int);
pub const TOKS_X_PC_STAGE1: toks_ext = toks_ext {
    pad: 0 as uint32_t,
    align: 2 as uint32_t,
};
pub const TOKS_X_PC_STAGE2: toks_ext = toks_ext {
    pad: 0 as uint32_t,
    align: 4 as uint32_t,
};
pub const TOKS_X_PC_KEYS: toks_ext = toks_ext {
    pad: 0 as uint32_t,
    align: 8 as uint32_t,
};
pub const TOKS_X_PC_VALS: toks_ext = toks_ext {
    pad: 0 as uint32_t,
    align: 4 as uint32_t,
};
pub const TOKS_X_PC_POOL: toks_ext = toks_ext {
    pad: 0 as uint32_t,
    align: 1 as uint32_t,
};
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
unsafe extern "C" fn toks_utf8_valid(
    mut p: *const uint8_t,
    mut n: uint64_t,
) -> ::core::ffi::c_int {
    let mut i: uint64_t = 0 as uint64_t;
    while i < n {
        let mut k: uint32_t = toks_utf8_len(p.offset(i as isize), n.wrapping_sub(i));
        if k == 0 as uint32_t {
            return 0 as ::core::ffi::c_int;
        }
        i = i.wrapping_add(k as uint64_t);
    }
    return 1 as ::core::ffi::c_int;
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
pub const TOKS_GC_CAT_MASK: ::core::ffi::c_uint = 0xf as ::core::ffi::c_uint;
pub const TOKS_GC_INCB_EXTEND: ::core::ffi::c_uint = 0x10 as ::core::ffi::c_uint;
pub const TOKS_GC_INCB_LINKER: ::core::ffi::c_uint = 0x20 as ::core::ffi::c_uint;
#[no_mangle]
pub unsafe extern "C" fn toks_gc_step(
    mut s: *mut toks_gc_state,
    mut cls: uint8_t,
) -> ::core::ffi::c_int {
    let mut a: uint32_t = cls as uint32_t & TOKS_GC_CAT_MASK as uint32_t;
    let mut brk: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    if (*s).prev as ::core::ffi::c_uint != 0xff as ::core::ffi::c_uint {
        let mut b: uint32_t = (*s).prev as uint32_t & TOKS_GC_CAT_MASK as uint32_t;
        if b == TOKS_GC_CR as ::core::ffi::c_int as uint32_t
            && a == TOKS_GC_LF as ::core::ffi::c_int as uint32_t
        {
            brk = 0 as ::core::ffi::c_int;
        } else if b == TOKS_GC_CONTROL as ::core::ffi::c_int as uint32_t
            || b == TOKS_GC_CR as ::core::ffi::c_int as uint32_t
            || b == TOKS_GC_LF as ::core::ffi::c_int as uint32_t
        {
            brk = 1 as ::core::ffi::c_int;
        } else if a == TOKS_GC_CONTROL as ::core::ffi::c_int as uint32_t
            || a == TOKS_GC_CR as ::core::ffi::c_int as uint32_t
            || a == TOKS_GC_LF as ::core::ffi::c_int as uint32_t
        {
            brk = 1 as ::core::ffi::c_int;
        } else if b == TOKS_GC_L as ::core::ffi::c_int as uint32_t
            && (a == TOKS_GC_L as ::core::ffi::c_int as uint32_t
                || a == TOKS_GC_V as ::core::ffi::c_int as uint32_t
                || a == TOKS_GC_LV as ::core::ffi::c_int as uint32_t
                || a == TOKS_GC_LVT as ::core::ffi::c_int as uint32_t)
        {
            brk = 0 as ::core::ffi::c_int;
        } else if (b == TOKS_GC_LV as ::core::ffi::c_int as uint32_t
            || b == TOKS_GC_V as ::core::ffi::c_int as uint32_t)
            && (a == TOKS_GC_V as ::core::ffi::c_int as uint32_t
                || a == TOKS_GC_T as ::core::ffi::c_int as uint32_t)
        {
            brk = 0 as ::core::ffi::c_int;
        } else if (b == TOKS_GC_LVT as ::core::ffi::c_int as uint32_t
            || b == TOKS_GC_T as ::core::ffi::c_int as uint32_t)
            && a == TOKS_GC_T as ::core::ffi::c_int as uint32_t
        {
            brk = 0 as ::core::ffi::c_int;
        } else if a == TOKS_GC_EXTEND as ::core::ffi::c_int as uint32_t
            || a == TOKS_GC_ZWJ as ::core::ffi::c_int as uint32_t
        {
            brk = 0 as ::core::ffi::c_int;
        } else if a == TOKS_GC_SPACINGMARK as ::core::ffi::c_int as uint32_t {
            brk = 0 as ::core::ffi::c_int;
        } else if b == TOKS_GC_PREPEND as ::core::ffi::c_int as uint32_t {
            brk = 0 as ::core::ffi::c_int;
        } else if a == TOKS_GC_INCB_CONSONANT as ::core::ffi::c_int as uint32_t {
            brk = !((*s).incb_cons as ::core::ffi::c_int != 0
                && (*s).incb_link as ::core::ffi::c_int != 0) as ::core::ffi::c_int;
        } else if b == TOKS_GC_ZWJ as ::core::ffi::c_int as uint32_t
            && a == TOKS_GC_EXTPICT as ::core::ffi::c_int as uint32_t
        {
            brk = ((*s).ep_zwj == 0) as ::core::ffi::c_int;
        } else if b == TOKS_GC_RI as ::core::ffi::c_int as uint32_t
            && a == TOKS_GC_RI as ::core::ffi::c_int as uint32_t
        {
            brk = ((*s).ri_odd == 0) as ::core::ffi::c_int;
        } else {
            brk = 1 as ::core::ffi::c_int;
        }
    }
    (*s).ri_odd = (if a == TOKS_GC_RI as ::core::ffi::c_int as uint32_t {
        ((*s).ri_odd == 0) as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    }) as uint8_t;
    let mut ep: uint8_t = (*s).ep_run;
    (*s).ep_run = (a == TOKS_GC_EXTPICT as ::core::ffi::c_int as uint32_t
        || a == TOKS_GC_EXTEND as ::core::ffi::c_int as uint32_t
            && ep as ::core::ffi::c_int != 0) as ::core::ffi::c_int as uint8_t;
    (*s).ep_zwj = (a == TOKS_GC_ZWJ as ::core::ffi::c_int as uint32_t
        && ep as ::core::ffi::c_int != 0) as ::core::ffi::c_int as uint8_t;
    if a == TOKS_GC_INCB_CONSONANT as ::core::ffi::c_int as uint32_t {
        (*s).incb_cons = 1 as uint8_t;
        (*s).incb_link = 0 as uint8_t;
    } else if cls as ::core::ffi::c_uint & TOKS_GC_INCB_LINKER
        != 0 as ::core::ffi::c_uint
    {
        if (*s).incb_cons != 0 {
            (*s).incb_link = 1 as uint8_t;
        }
    } else if cls as ::core::ffi::c_uint & TOKS_GC_INCB_EXTEND
        == 0 as ::core::ffi::c_uint
    {
        (*s).incb_cons = 0 as uint8_t;
        (*s).incb_link = 0 as uint8_t;
    }
    (*s).prev = cls;
    return brk;
}
#[no_mangle]
pub unsafe extern "C" fn toks_b64v(mut c: uint8_t) -> ::core::ffi::c_int {
    if c as ::core::ffi::c_int >= 'A' as i32 && c as ::core::ffi::c_int <= 'Z' as i32 {
        return c as ::core::ffi::c_int - 'A' as i32;
    }
    if c as ::core::ffi::c_int >= 'a' as i32 && c as ::core::ffi::c_int <= 'z' as i32 {
        return c as ::core::ffi::c_int - 'a' as i32 + 26 as ::core::ffi::c_int;
    }
    if c as ::core::ffi::c_int >= '0' as i32 && c as ::core::ffi::c_int <= '9' as i32 {
        return c as ::core::ffi::c_int - '0' as i32 + 52 as ::core::ffi::c_int;
    }
    if c as ::core::ffi::c_int == '+' as i32 {
        return 62 as ::core::ffi::c_int;
    }
    if c as ::core::ffi::c_int == '/' as i32 {
        return 63 as ::core::ffi::c_int;
    }
    return -(1 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn toks_b64_len(
    mut p: *const uint8_t,
    mut n: uint64_t,
) -> int64_t {
    if n == 0 as uint64_t || n & 3 as uint64_t != 0 as uint64_t {
        return -(1 as ::core::ffi::c_int) as int64_t;
    }
    let mut pad: uint64_t = (if *p.offset(n.wrapping_sub(1 as uint64_t) as isize)
        as ::core::ffi::c_int == '=' as i32
    {
        if *p.offset(n.wrapping_sub(2 as uint64_t) as isize) as ::core::ffi::c_int
            == '=' as i32
        {
            2 as ::core::ffi::c_uint
        } else {
            1 as ::core::ffi::c_uint
        }
    } else {
        0 as ::core::ffi::c_uint
    }) as uint64_t;
    let mut i: uint64_t = 0 as uint64_t;
    while i < n.wrapping_sub(pad) {
        if toks_b64v(*p.offset(i as isize)) < 0 as ::core::ffi::c_int {
            return -(1 as ::core::ffi::c_int) as int64_t;
        }
        i = i.wrapping_add(1);
    }
    if pad == 2 as uint64_t
        && toks_b64v(*p.offset(n.wrapping_sub(3 as uint64_t) as isize))
            & 0xf as ::core::ffi::c_int != 0 as ::core::ffi::c_int
    {
        return -(1 as ::core::ffi::c_int) as int64_t;
    }
    if pad == 1 as uint64_t
        && toks_b64v(*p.offset(n.wrapping_sub(2 as uint64_t) as isize))
            & 0x3 as ::core::ffi::c_int != 0 as ::core::ffi::c_int
    {
        return -(1 as ::core::ffi::c_int) as int64_t;
    }
    return n.wrapping_div(4 as uint64_t).wrapping_mul(3 as uint64_t).wrapping_sub(pad)
        as int64_t;
}
#[no_mangle]
pub unsafe extern "C" fn toks_b64_decode(
    mut p: *const uint8_t,
    mut n: uint64_t,
    mut out: *mut uint8_t,
    mut cap: uint64_t,
) -> int64_t {
    let mut m: int64_t = toks_b64_len(p, n);
    if m < 0 as int64_t || m as uint64_t > cap {
        return -(1 as ::core::ffi::c_int) as int64_t;
    }
    let mut o: uint64_t = 0 as uint64_t;
    let mut i: uint64_t = 0 as uint64_t;
    while i < n {
        let mut v: uint32_t = 0 as uint32_t;
        let mut k: uint32_t = 0 as uint32_t;
        let mut j: uint32_t = 0 as uint32_t;
        while j < 4 as uint32_t {
            if *p.offset(i.wrapping_add(j as uint64_t) as isize) as ::core::ffi::c_int
                == '=' as i32
            {
                break;
            }
            v
                |= (toks_b64v(*p.offset(i.wrapping_add(j as uint64_t) as isize))
                    as uint32_t)
                    << (18 as uint32_t).wrapping_sub((6 as uint32_t).wrapping_mul(j));
            k = k.wrapping_add(1);
            j = j.wrapping_add(1);
        }
        let fresh0 = o;
        o = o.wrapping_add(1);
        *out.offset(fresh0 as isize) = (v >> 16 as ::core::ffi::c_int) as uint8_t;
        if k > 2 as uint32_t {
            let fresh1 = o;
            o = o.wrapping_add(1);
            *out.offset(fresh1 as isize) = (v >> 8 as ::core::ffi::c_int) as uint8_t;
        }
        if k > 3 as uint32_t {
            let fresh2 = o;
            o = o.wrapping_add(1);
            *out.offset(fresh2 as isize) = v as uint8_t;
        }
        i = i.wrapping_add(4 as uint64_t);
    }
    return m;
}
unsafe extern "C" fn du_offset(mut u: uint32_t) -> uint32_t {
    return (u >> 10 as ::core::ffi::c_int)
        << ((u & (1 as uint32_t) << 9 as ::core::ffi::c_int) >> 6 as ::core::ffi::c_int);
}
unsafe extern "C" fn du_label(mut u: uint32_t) -> uint32_t {
    return u & 0x800000ff as uint32_t;
}
unsafe extern "C" fn du_value(mut u: uint32_t) -> uint32_t {
    return u & 0x7fffffff as uint32_t;
}
unsafe extern "C" fn du_leaf(mut u: uint32_t) -> ::core::ffi::c_int {
    return (u >> 8 as ::core::ffi::c_int & 1 as uint32_t != 0 as uint32_t)
        as ::core::ffi::c_int;
}
unsafe extern "C" fn unit_at(mut b: *const pc_blob, mut i: uint32_t) -> uint32_t {
    let mut p: *const uint8_t = (*b)
        .units
        .offset((4 as uint64_t).wrapping_mul(i as uint64_t) as isize);
    return *p.offset(0 as ::core::ffi::c_int as isize) as uint32_t
        | (*p.offset(1 as ::core::ffi::c_int as isize) as uint32_t)
            << 8 as ::core::ffi::c_int
        | (*p.offset(2 as ::core::ffi::c_int as isize) as uint32_t)
            << 16 as ::core::ffi::c_int
        | (*p.offset(3 as ::core::ffi::c_int as isize) as uint32_t)
            << 24 as ::core::ffi::c_int;
}
unsafe extern "C" fn pc_parse(
    mut b: *mut pc_blob,
    mut blob: *const uint8_t,
    mut len: uint64_t,
    mut why: *mut *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if len < 4 as uint64_t {
        *why = b"precompiled charsmap: shorter than its header\0" as *const u8
            as *const ::core::ffi::c_char;
        return -(1 as ::core::ffi::c_int);
    }
    let mut t: uint32_t = *blob.offset(0 as ::core::ffi::c_int as isize) as uint32_t
        | (*blob.offset(1 as ::core::ffi::c_int as isize) as uint32_t)
            << 8 as ::core::ffi::c_int
        | (*blob.offset(2 as ::core::ffi::c_int as isize) as uint32_t)
            << 16 as ::core::ffi::c_int
        | (*blob.offset(3 as ::core::ffi::c_int as isize) as uint32_t)
            << 24 as ::core::ffi::c_int;
    let mut nu: uint64_t = (t as uint64_t).wrapping_div(4 as uint64_t);
    if (4 as uint64_t).wrapping_add((4 as uint64_t).wrapping_mul(nu)) > len {
        *why = b"precompiled charsmap: trie past the blob\0" as *const u8
            as *const ::core::ffi::c_char;
        return -(1 as ::core::ffi::c_int);
    }
    if nu == 0 as uint64_t {
        *why = b"precompiled charsmap: empty trie\0" as *const u8
            as *const ::core::ffi::c_char;
        return -(1 as ::core::ffi::c_int);
    }
    if len.wrapping_sub((4 as uint64_t).wrapping_add((4 as uint64_t).wrapping_mul(nu)))
        > 0x3ffff as uint64_t
    {
        *why = b"precompiled charsmap: strings over 256 KiB\0" as *const u8
            as *const ::core::ffi::c_char;
        return -(1 as ::core::ffi::c_int);
    }
    (*b).units = blob.offset(4 as ::core::ffi::c_int as isize);
    (*b).n_units = nu as uint32_t;
    (*b).str_0 = blob
        .offset(4 as ::core::ffi::c_uint as isize)
        .offset((4 as uint64_t).wrapping_mul(nu) as isize);
    (*b).str_len = len
        .wrapping_sub((4 as uint64_t).wrapping_add((4 as uint64_t).wrapping_mul(nu)))
        as uint32_t;
    if toks_utf8_valid((*b).str_0, (*b).str_len as uint64_t) == 0 {
        *why = b"precompiled charsmap: strings not utf-8\0" as *const u8
            as *const ::core::ffi::c_char;
        return -(1 as ::core::ffi::c_int);
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn pc_value_len(mut b: *const pc_blob, mut v: uint32_t) -> int64_t {
    if v > (*b).str_len {
        return -(1 as ::core::ffi::c_int) as int64_t;
    }
    if v < (*b).str_len
        && *(*b).str_0.offset(v as isize) as ::core::ffi::c_uint
            & 0xc0 as ::core::ffi::c_uint == 0x80 as ::core::ffi::c_uint
    {
        return -(1 as ::core::ffi::c_int) as int64_t;
    }
    let mut e: uint32_t = v;
    while e < (*b).str_len
        && *(*b).str_0.offset(e as isize) as ::core::ffi::c_uint
            != 0 as ::core::ffi::c_uint
    {
        e = e.wrapping_add(1);
    }
    return e.wrapping_sub(v) as int64_t;
}
pub const PC_MAX_DEPTH: ::core::ffi::c_uint = 64 as ::core::ffi::c_uint;
pub const PC_MAX_PREFIXES: ::core::ffi::c_uint = (1 as ::core::ffi::c_uint)
    << 21 as ::core::ffi::c_int;
unsafe extern "C" fn mk_pack(mut k: *const uint8_t, mut n: uint32_t) -> uint64_t {
    let mut v: uint64_t = (n as uint64_t) << 56 as ::core::ffi::c_int;
    let mut i: uint32_t = 0 as uint32_t;
    while i < n {
        v |= (*k.offset(i as isize) as uint64_t) << (8 as uint32_t).wrapping_mul(i);
        i = i.wrapping_add(1);
    }
    return v;
}
unsafe extern "C" fn mk_hash(mut v: uint64_t) -> uint32_t {
    return toks_crc32c_u64(0x9e3779b9 as uint32_t, v);
}
unsafe extern "C" fn mk_insert(
    mut pc: *mut toks_pc,
    mut key: uint64_t,
    mut ent: uint32_t,
) {
    let mut h: uint32_t = mk_hash(key) & (*pc).mk_mask;
    while *(*pc).mk_key.offset(h as isize) != 0 as uint64_t {
        h = h.wrapping_add(1 as uint32_t) & (*pc).mk_mask;
    }
    *(*pc).mk_key.offset(h as isize) = key;
    *(*pc).mk_val.offset(h as isize) = ent;
}
#[no_mangle]
pub unsafe extern "C" fn toks_pc_multi(
    mut pc: *const toks_pc,
    mut key: *const uint8_t,
    mut n: uint32_t,
) -> uint32_t {
    if (*pc).mk_mask == 0 as uint32_t || n < 2 as uint32_t || n > 5 as uint32_t {
        return 0 as uint32_t;
    }
    let mut v: uint64_t = mk_pack(key, n);
    let mut h: uint32_t = mk_hash(v) & (*pc).mk_mask;
    let mut i: uint32_t = 0 as uint32_t;
    while i <= (*pc).mk_mask {
        let mut k: uint64_t = *(*pc).mk_key.offset(h as isize);
        if k == 0 as uint64_t {
            return 0 as uint32_t;
        }
        if k == v {
            return *(*pc).mk_val.offset(h as isize);
        }
        h = h.wrapping_add(1 as uint32_t) & (*pc).mk_mask;
        i = i.wrapping_add(1);
    }
    return 0 as uint32_t;
}
unsafe extern "C" fn pc_entry(mut off: uint32_t, mut len: uint32_t) -> uint32_t {
    return (TOKS_PC_VALUE as ::core::ffi::c_int as uint32_t) << 30 as ::core::ffi::c_int
        | off << 12 as ::core::ffi::c_int | len;
}
unsafe extern "C" fn one_char(mut k: *const uint8_t, mut n: uint32_t) -> int64_t {
    let mut l: uint32_t = toks_utf8_len(k, n as uint64_t);
    if l == 0 as uint32_t || l != n {
        return -(1 as ::core::ffi::c_int) as int64_t;
    }
    return if l == 1 as uint32_t {
        *k.offset(0 as ::core::ffi::c_int as isize) as int64_t
    } else {
        toks_cp_decode(k, l) as int64_t
    };
}
unsafe extern "C" fn pc_walk(
    mut b: *const pc_blob,
    mut cnt: *mut pc_count,
    mut pc: *mut toks_pc,
    mut why: *mut *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut st: [pc_frame; 65] = [pc_frame {
        node: 0,
        next_label: 0,
        depth: 0,
        dead: 0,
    }; 65];
    let mut key: [uint8_t; 65] = [0; 65];
    let mut root: uint32_t = du_offset(unit_at(b, 0 as uint32_t));
    let mut sp: uint32_t = 0 as uint32_t;
    st[0 as ::core::ffi::c_int as usize].node = root;
    st[0 as ::core::ffi::c_int as usize].next_label = 1 as ::core::ffi::c_uint
        as uint32_t;
    st[0 as ::core::ffi::c_int as usize].depth = 0 as ::core::ffi::c_uint as uint32_t;
    st[0 as ::core::ffi::c_int as usize].dead = 0 as ::core::ffi::c_uint as uint32_t;
    let mut pushes: uint64_t = 0 as uint64_t;
    loop {
        let mut f: *mut pc_frame = (&raw mut st as *mut pc_frame).offset(sp as isize)
            as *mut pc_frame;
        if (*f).next_label > 255 as uint32_t {
            if sp == 0 as uint32_t {
                break;
            }
            sp = sp.wrapping_sub(1);
        } else {
            let fresh3 = (*f).next_label;
            (*f).next_label = (*f).next_label.wrapping_add(1);
            let mut c: uint32_t = fresh3;
            let mut slot: uint32_t = (*f).node ^ c;
            if slot >= (*b).n_units {
                *why = b"precompiled charsmap: a transition outside the array (hf panics on such input)\0"
                    as *const u8 as *const ::core::ffi::c_char;
                return -(1 as ::core::ffi::c_int);
            }
            let mut u: uint32_t = unit_at(b, slot);
            if du_label(u) != c {
                continue;
            }
            let mut nxt: uint32_t = slot ^ du_offset(u);
            let mut d: uint32_t = (*f).depth;
            if d >= PC_MAX_DEPTH as uint32_t {
                *why = b"precompiled charsmap: key longer than 64 bytes\0" as *const u8
                    as *const ::core::ffi::c_char;
                return -(1 as ::core::ffi::c_int);
            }
            key[d as usize] = c as uint8_t;
            let mut dead: uint32_t = (*f).dead;
            if du_leaf(u) != 0 {
                if nxt >= (*b).n_units {
                    *why = b"precompiled charsmap: a leaf outside the array (hf panics)\0"
                        as *const u8 as *const ::core::ffi::c_char;
                    return -(1 as ::core::ffi::c_int);
                }
                let mut v: uint32_t = du_value(unit_at(b, nxt));
                let mut vl: int64_t = pc_value_len(b, v);
                if vl < 0 as int64_t {
                    *why = b"precompiled charsmap: a value off a char boundary (hf panics)\0"
                        as *const u8 as *const ::core::ffi::c_char;
                    return -(1 as ::core::ffi::c_int);
                }
                if toks_utf8_valid(
                    &raw mut key as *mut uint8_t,
                    d.wrapping_add(1 as uint32_t) as uint64_t,
                ) == 0
                {
                    *why = b"precompiled charsmap: a key that is not utf-8\0"
                        as *const u8 as *const ::core::ffi::c_char;
                    return -(1 as ::core::ffi::c_int);
                }
                (*cnt).n_keys = (*cnt).n_keys.wrapping_add(1);
                if dead == 0 {
                    let mut cp: int64_t = one_char(
                        &raw mut key as *mut uint8_t,
                        d.wrapping_add(1 as uint32_t),
                    );
                    let mut ent: uint32_t = pc_entry(v, vl as uint32_t);
                    if vl as uint32_t > 0xfff as uint32_t {
                        *why = b"precompiled charsmap: a value over 4 KiB\0" as *const u8
                            as *const ::core::ffi::c_char;
                        return -(1 as ::core::ffi::c_int);
                    }
                    let mut ex: uint32_t = (vl as uint32_t)
                        .wrapping_add(d)
                        .wrapping_div(d.wrapping_add(1 as uint32_t));
                    if cp >= 0 as int64_t {
                        (*cnt).n_single = (*cnt).n_single.wrapping_add(1);
                        if ex > (*cnt).max_expand {
                            (*cnt).max_expand = ex;
                        }
                        let mut blk: uint32_t = cp as uint32_t
                            >> 8 as ::core::ffi::c_int;
                        if pc.is_null() {
                            if (*cnt).block_used[blk as usize] == 0 {
                                (*cnt).block_used[blk as usize] = 1 as uint8_t;
                                (*cnt).n_blocks = (*cnt).n_blocks.wrapping_add(1);
                            }
                        } else {
                            *(*pc)
                                .stage2
                                .offset(
                                    (*(*pc).stage1.offset(blk as isize) as uint32_t)
                                        .wrapping_mul(256 as uint32_t)
                                        .wrapping_add(cp as uint32_t & 0xff as uint32_t) as isize,
                                ) = ent;
                        }
                    } else if d.wrapping_add(1 as uint32_t) <= 5 as uint32_t {
                        (*cnt).n_multi = (*cnt).n_multi.wrapping_add(1);
                        if ex > (*cnt).max_expand {
                            (*cnt).max_expand = ex;
                        }
                        if !pc.is_null() {
                            mk_insert(
                                pc,
                                mk_pack(
                                    &raw mut key as *mut uint8_t,
                                    d.wrapping_add(1 as uint32_t),
                                ),
                                ent,
                            );
                        }
                    }
                }
                dead = 1 as ::core::ffi::c_uint as uint32_t;
            }
            pushes = pushes.wrapping_add(1);
            if pushes > PC_MAX_PREFIXES as uint64_t {
                *why = b"precompiled charsmap: more than 2^21 key prefixes\0"
                    as *const u8 as *const ::core::ffi::c_char;
                return -(1 as ::core::ffi::c_int);
            }
            sp = sp.wrapping_add(1);
            st[sp as usize].node = nxt;
            st[sp as usize].next_label = 1 as ::core::ffi::c_uint as uint32_t;
            st[sp as usize].depth = d.wrapping_add(1 as uint32_t);
            st[sp as usize].dead = dead;
        }
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn pow2_at_least(mut v: uint32_t) -> uint32_t {
    let mut p: uint32_t = 16 as uint32_t;
    while p < v {
        p <<= 1 as ::core::ffi::c_int;
    }
    return p;
}
#[no_mangle]
pub unsafe extern "C" fn toks_pc_bytes(
    mut blob: *const uint8_t,
    mut len: uint64_t,
    mut why: *mut *const ::core::ffi::c_char,
) -> uint64_t {
    let mut b: pc_blob = pc_blob {
        units: ::core::ptr::null::<uint8_t>(),
        n_units: 0,
        str_0: ::core::ptr::null::<uint8_t>(),
        str_len: 0,
    };
    if pc_parse(&raw mut b, blob, len, why) != 0 as ::core::ffi::c_int {
        return 0 as uint64_t;
    }
    let mut cnt: pc_count = pc_count {
        n_keys: 0,
        n_single: 0,
        n_multi: 0,
        n_blocks: 0,
        max_expand: 0,
        block_used: [0; 4352],
    };
    memset(
        &raw mut cnt as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<pc_count>() as size_t,
    );
    if pc_walk(&raw mut b, &raw mut cnt, ::core::ptr::null_mut::<toks_pc>(), why)
        != 0 as ::core::ffi::c_int
    {
        return 0 as uint64_t;
    }
    let mut blocks: uint64_t = (cnt.n_blocks as uint64_t).wrapping_add(1 as uint64_t);
    let mut mk: uint32_t = pow2_at_least(
        (2 as uint32_t).wrapping_mul(cnt.n_multi).wrapping_add(2 as uint32_t),
    );
    return ((64 as ::core::ffi::c_uint)
        .wrapping_add(
            (2 as ::core::ffi::c_uint).wrapping_mul(0x1100 as ::core::ffi::c_uint),
        )
        .wrapping_add(64 as ::core::ffi::c_uint) as uint64_t)
        .wrapping_add(blocks.wrapping_mul(256 as uint64_t).wrapping_mul(4 as uint64_t))
        .wrapping_add(64 as uint64_t)
        .wrapping_add((mk as uint64_t).wrapping_mul(12 as uint64_t))
        .wrapping_add(64 as uint64_t)
        .wrapping_add(b.str_len as uint64_t)
        .wrapping_add(64 as uint64_t);
}
unsafe extern "C" fn ar_take(
    mut ar: *mut toks_arena,
    mut n: uint64_t,
    mut x: toks_ext,
) -> *mut ::core::ffi::c_void {
    return toks_tab_ar(ar, n, 64 as uint64_t, x);
}
#[no_mangle]
pub unsafe extern "C" fn toks_pc_build(
    mut pc: *mut toks_pc,
    mut blob: *const uint8_t,
    mut len: uint64_t,
    mut ar: *mut toks_arena,
    mut why: *mut *const ::core::ffi::c_char,
) -> int64_t {
    memset(
        pc as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<toks_pc>() as size_t,
    );
    let mut b: pc_blob = pc_blob {
        units: ::core::ptr::null::<uint8_t>(),
        n_units: 0,
        str_0: ::core::ptr::null::<uint8_t>(),
        str_len: 0,
    };
    if pc_parse(&raw mut b, blob, len, why) != 0 as ::core::ffi::c_int {
        return TOKS_E_UNSUPPORTED as int64_t;
    }
    let mut cnt: pc_count = pc_count {
        n_keys: 0,
        n_single: 0,
        n_multi: 0,
        n_blocks: 0,
        max_expand: 0,
        block_used: [0; 4352],
    };
    memset(
        &raw mut cnt as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<pc_count>() as size_t,
    );
    if pc_walk(&raw mut b, &raw mut cnt, ::core::ptr::null_mut::<toks_pc>(), why)
        != 0 as ::core::ffi::c_int
    {
        return TOKS_E_UNSUPPORTED as int64_t;
    }
    let mut nblk: uint32_t = cnt.n_blocks.wrapping_add(1 as uint32_t);
    (*pc).stage1 = ar_take(
        ar,
        (2 as ::core::ffi::c_uint).wrapping_mul(0x1100 as ::core::ffi::c_uint)
            as uint64_t,
        TOKS_X_PC_STAGE1,
    ) as *mut uint16_t;
    (*pc).stage2 = ar_take(
        ar,
        (nblk as uint64_t).wrapping_mul(256 as uint64_t).wrapping_mul(4 as uint64_t),
        TOKS_X_PC_STAGE2,
    ) as *mut uint32_t;
    let mut mk: uint32_t = pow2_at_least(
        (2 as uint32_t).wrapping_mul(cnt.n_multi).wrapping_add(2 as uint32_t),
    );
    (*pc).mk_key = ar_take(
        ar,
        (mk as uint64_t).wrapping_mul(8 as uint64_t),
        TOKS_X_PC_KEYS,
    ) as *mut uint64_t;
    (*pc).mk_val = ar_take(
        ar,
        (mk as uint64_t).wrapping_mul(4 as uint64_t),
        TOKS_X_PC_VALS,
    ) as *mut uint32_t;
    (*pc).pool = ar_take(ar, b.str_len as uint64_t, TOKS_X_PC_POOL) as *mut uint8_t;
    toks_ar_alloc(ar, 1 as uint64_t, 1 as uint64_t);
    if (*pc).stage1.is_null() || (*pc).stage2.is_null() || (*pc).mk_key.is_null()
        || (*pc).mk_val.is_null() || (*pc).pool.is_null()
    {
        *why = b"precompiled charsmap: arena\0" as *const u8
            as *const ::core::ffi::c_char;
        return TOKS_E_NOMEM as int64_t;
    }
    memset(
        (*pc).stage1 as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        (2 as ::core::ffi::c_uint).wrapping_mul(0x1100 as ::core::ffi::c_uint) as size_t,
    );
    memset(
        (*pc).stage2 as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        (nblk as size_t).wrapping_mul(256 as size_t).wrapping_mul(4 as size_t),
    );
    memset(
        (*pc).mk_key as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        (mk as size_t).wrapping_mul(8 as size_t),
    );
    memset(
        (*pc).mk_val as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        (mk as size_t).wrapping_mul(4 as size_t),
    );
    let mut next: uint32_t = 1 as uint32_t;
    let mut blk: uint32_t = 0 as uint32_t;
    while blk < 0x1100 as uint32_t {
        if cnt.block_used[blk as usize] != 0 {
            let fresh4 = next;
            next = next.wrapping_add(1);
            *(*pc).stage1.offset(blk as isize) = fresh4 as uint16_t;
        }
        blk = blk.wrapping_add(1);
    }
    (*pc).n_blocks = nblk;
    (*pc).mk_mask = mk.wrapping_sub(1 as uint32_t);
    memcpy(
        (*pc).pool as *mut ::core::ffi::c_void,
        b.str_0 as *const ::core::ffi::c_void,
        b.str_len as size_t,
    );
    (*pc).pool_len = b.str_len;
    let mut cnt2: pc_count = pc_count {
        n_keys: 0,
        n_single: 0,
        n_multi: 0,
        n_blocks: 0,
        max_expand: 0,
        block_used: [0; 4352],
    };
    memset(
        &raw mut cnt2 as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<pc_count>() as size_t,
    );
    if pc_walk(&raw mut b, &raw mut cnt2, pc, why) != 0 as ::core::ffi::c_int {
        return TOKS_E_UNSUPPORTED as int64_t;
    }
    (*pc).n_keys = cnt.n_keys;
    (*pc).n_live_single = cnt.n_single;
    (*pc).n_live_multi = cnt.n_multi;
    (*pc).max_expand = if cnt.max_expand > 1 as uint32_t {
        cnt.max_expand
    } else {
        1 as uint32_t
    };
    return 0 as int64_t;
}
