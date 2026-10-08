extern "C" {
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
pub struct toks_arena {
    pub base: *mut uint8_t,
    pub len: uint64_t,
    pub pos: uint64_t,
}
pub type C2RustUnnamed = ::core::ffi::c_uint;
pub const JV_MEM: C2RustUnnamed = 6;
pub const JV_OBJ: C2RustUnnamed = 5;
pub const JV_ARR: C2RustUnnamed = 4;
pub const JV_STR: C2RustUnnamed = 3;
pub const JV_NUM: C2RustUnnamed = 2;
pub const JV_BOOL: C2RustUnnamed = 1;
pub const JV_NULL: C2RustUnnamed = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct jv {
    pub type_0: uint8_t,
    pub num_float: uint8_t,
    pub rsv: uint16_t,
    pub s_len: uint32_t,
    pub s: *const uint8_t,
    pub child: *mut jv,
    pub next: *mut jv,
    pub num: int64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct jframe {
    pub obj: *mut jv,
    pub mem: *mut jv,
    pub last: *mut jv,
    pub phase: uint8_t,
}
pub const JF_MEMBER_END: C2RustUnnamed_0 = 2;
pub const JF_ELEM_END: C2RustUnnamed_0 = 3;
pub const JF_DONE: C2RustUnnamed_0 = 4;
pub const JF_VALUE: C2RustUnnamed_0 = 0;
pub const JF_KEY: C2RustUnnamed_0 = 1;
pub const JS_NOMEM: C2RustUnnamed_1 = 2;
pub const JS_OK: C2RustUnnamed_1 = 0;
pub const JS_FMT: C2RustUnnamed_1 = 1;
pub type C2RustUnnamed_0 = ::core::ffi::c_uint;
pub type C2RustUnnamed_1 = ::core::ffi::c_uint;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
pub const TOKS_E_FORMAT: ::core::ffi::c_int = -(2 as ::core::ffi::c_int);
pub const TOKS_E_NOMEM: ::core::ffi::c_int = -(11 as ::core::ffi::c_int);
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
pub const TOKS_JSON_DEPTH: ::core::ffi::c_uint = 64 as ::core::ffi::c_uint;
unsafe extern "C" fn jv_new(mut ar: *mut toks_arena, mut type_0: uint8_t) -> *mut jv {
    let mut v: *mut jv = toks_ar_alloc(
        ar,
        ::core::mem::size_of::<jv>() as uint64_t,
        8 as uint64_t,
    ) as *mut jv;
    if v.is_null() {
        return ::core::ptr::null_mut::<jv>();
    }
    memset(
        v as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<jv>() as size_t,
    );
    (*v).type_0 = type_0;
    return v;
}
unsafe extern "C" fn skip_ws(
    mut p: *const uint8_t,
    mut end: *const uint8_t,
) -> *const uint8_t {
    while p < end
        && (*p as ::core::ffi::c_uint == 0x20 as ::core::ffi::c_uint
            || *p as ::core::ffi::c_uint == 0x9 as ::core::ffi::c_uint
            || *p as ::core::ffi::c_uint == 0xa as ::core::ffi::c_uint
            || *p as ::core::ffi::c_uint == 0xd as ::core::ffi::c_uint)
    {
        p = p.offset(1);
    }
    return p;
}
unsafe extern "C" fn hex4(
    mut p: *const uint8_t,
    mut out: *mut uint32_t,
) -> ::core::ffi::c_int {
    let mut v: uint32_t = 0 as uint32_t;
    let mut i: uint32_t = 0 as uint32_t;
    while i < 4 as uint32_t {
        let mut c: uint8_t = *p.offset(i as isize);
        let mut d: uint32_t = 0;
        if c as ::core::ffi::c_int >= '0' as i32 && c as ::core::ffi::c_int <= '9' as i32
        {
            d = (c as ::core::ffi::c_int - '0' as i32) as uint32_t;
        } else if c as ::core::ffi::c_int >= 'a' as i32
            && c as ::core::ffi::c_int <= 'f' as i32
        {
            d = ((c as ::core::ffi::c_int - 'a' as i32) as uint32_t)
                .wrapping_add(10 as uint32_t);
        } else if c as ::core::ffi::c_int >= 'A' as i32
            && c as ::core::ffi::c_int <= 'F' as i32
        {
            d = ((c as ::core::ffi::c_int - 'A' as i32) as uint32_t)
                .wrapping_add(10 as uint32_t);
        } else {
            return 0 as ::core::ffi::c_int
        }
        v = v << 4 as ::core::ffi::c_int | d;
        i = i.wrapping_add(1);
    }
    *out = v;
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn parse_string(
    mut p: *const uint8_t,
    mut end: *const uint8_t,
    mut ar: *mut toks_arena,
    mut s: *mut *const uint8_t,
    mut s_len: *mut uint32_t,
    mut next: *mut *const uint8_t,
) -> ::core::ffi::c_int {
    if p >= end || *p as ::core::ffi::c_int != '"' as i32 {
        return JS_FMT as ::core::ffi::c_int;
    }
    p = p.offset(1);
    let mut q: *const uint8_t = p;
    while q < end && *q as ::core::ffi::c_int != '"' as i32 {
        if (*q as ::core::ffi::c_uint) < 0x20 as ::core::ffi::c_uint {
            return JS_FMT as ::core::ffi::c_int;
        }
        if *q as ::core::ffi::c_int == '\\' as i32 {
            q = q.offset(1);
            if q >= end {
                return JS_FMT as ::core::ffi::c_int;
            }
            if *q as ::core::ffi::c_int == 'u' as i32 {
                if end.offset_from(q) as ::core::ffi::c_long <= 4 as ::core::ffi::c_long
                {
                    return JS_FMT as ::core::ffi::c_int;
                }
                q = q.offset(4 as ::core::ffi::c_uint as isize);
            } else if *q as ::core::ffi::c_int == '"' as i32
                || *q as ::core::ffi::c_int == '\\' as i32
                || *q as ::core::ffi::c_int == '/' as i32
                || *q as ::core::ffi::c_int == 'b' as i32
                || *q as ::core::ffi::c_int == 'f' as i32
                || *q as ::core::ffi::c_int == 'n' as i32
                || *q as ::core::ffi::c_int == 'r' as i32
                || *q as ::core::ffi::c_int == 't' as i32
            {} else {
                return JS_FMT as ::core::ffi::c_int
            }
        }
        q = q.offset(1);
    }
    if q >= end {
        return JS_FMT as ::core::ffi::c_int;
    }
    let mut raw: uint64_t = q.offset_from(p) as ::core::ffi::c_long as uint64_t;
    let mut dst: *mut uint8_t = toks_ar_alloc(ar, raw, 8 as uint64_t) as *mut uint8_t;
    if dst.is_null() {
        return JS_NOMEM as ::core::ffi::c_int;
    }
    let mut n: uint64_t = 0 as uint64_t;
    let mut r: *const uint8_t = p;
    while r < q {
        let mut c: uint8_t = *r;
        if c as ::core::ffi::c_int == '\\' as i32 {
            r = r.offset(1);
            let mut e: uint8_t = *r;
            if e as ::core::ffi::c_int == 'u' as i32 {
                let mut cp: uint32_t = 0;
                if (q.offset_from(r) as ::core::ffi::c_long) < 5 as ::core::ffi::c_long
                    || hex4(r.offset(1 as ::core::ffi::c_uint as isize), &raw mut cp)
                        == 0
                {
                    return JS_FMT as ::core::ffi::c_int;
                }
                r = r.offset(5 as ::core::ffi::c_uint as isize);
                if cp >= 0xd800 as uint32_t && cp <= 0xdbff as uint32_t {
                    if (q.offset_from(r) as ::core::ffi::c_long)
                        < 6 as ::core::ffi::c_long
                        || *r.offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int != '\\' as i32
                        || *r.offset(1 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int != 'u' as i32
                    {
                        return JS_FMT as ::core::ffi::c_int;
                    }
                    let mut lo: uint32_t = 0;
                    if hex4(r.offset(2 as ::core::ffi::c_uint as isize), &raw mut lo)
                        == 0
                    {
                        return JS_FMT as ::core::ffi::c_int;
                    }
                    if lo < 0xdc00 as uint32_t || lo > 0xdfff as uint32_t {
                        return JS_FMT as ::core::ffi::c_int;
                    }
                    cp = (0x10000 as uint32_t)
                        .wrapping_add(
                            cp.wrapping_sub(0xd800 as uint32_t)
                                << 10 as ::core::ffi::c_int,
                        )
                        .wrapping_add(lo.wrapping_sub(0xdc00 as uint32_t));
                    r = r.offset(6 as ::core::ffi::c_uint as isize);
                } else if cp >= 0xdc00 as uint32_t && cp <= 0xdfff as uint32_t {
                    return JS_FMT as ::core::ffi::c_int
                }
                n = n
                    .wrapping_add(toks_utf8_put(dst.offset(n as isize), cp) as uint64_t);
            } else if e as ::core::ffi::c_int == 'n' as i32 {
                let fresh0 = n;
                n = n.wrapping_add(1);
                *dst.offset(fresh0 as isize) = '\n' as i32 as uint8_t;
                r = r.offset(1);
            } else if e as ::core::ffi::c_int == 't' as i32 {
                let fresh1 = n;
                n = n.wrapping_add(1);
                *dst.offset(fresh1 as isize) = '\t' as i32 as uint8_t;
                r = r.offset(1);
            } else if e as ::core::ffi::c_int == 'r' as i32 {
                let fresh2 = n;
                n = n.wrapping_add(1);
                *dst.offset(fresh2 as isize) = '\r' as i32 as uint8_t;
                r = r.offset(1);
            } else if e as ::core::ffi::c_int == 'b' as i32 {
                let fresh3 = n;
                n = n.wrapping_add(1);
                *dst.offset(fresh3 as isize) = 0x8 as uint8_t;
                r = r.offset(1);
            } else if e as ::core::ffi::c_int == 'f' as i32 {
                let fresh4 = n;
                n = n.wrapping_add(1);
                *dst.offset(fresh4 as isize) = 0xc as uint8_t;
                r = r.offset(1);
            } else {
                let fresh5 = n;
                n = n.wrapping_add(1);
                *dst.offset(fresh5 as isize) = e;
                r = r.offset(1);
            }
        } else {
            let fresh6 = n;
            n = n.wrapping_add(1);
            *dst.offset(fresh6 as isize) = c;
            r = r.offset(1);
        }
    }
    if toks_utf8_valid(dst, n) == 0 {
        return JS_FMT as ::core::ffi::c_int;
    }
    *s = dst;
    *s_len = n as uint32_t;
    *next = q.offset(1 as ::core::ffi::c_uint as isize);
    return JS_OK as ::core::ffi::c_int;
}
unsafe extern "C" fn parse_num(
    mut p: *const uint8_t,
    mut end: *const uint8_t,
    mut v: *mut jv,
) -> *const uint8_t {
    let mut q: *const uint8_t = p;
    let mut neg: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if q < end && *q as ::core::ffi::c_int == '-' as i32 {
        neg = 1 as ::core::ffi::c_int;
        q = q.offset(1);
    }
    if q >= end {
        return ::core::ptr::null::<uint8_t>();
    }
    if *q as ::core::ffi::c_int == '0' as i32 {
        q = q.offset(1);
    } else if *q as ::core::ffi::c_int >= '1' as i32
        && *q as ::core::ffi::c_int <= '9' as i32
    {
        while q < end && *q as ::core::ffi::c_int >= '0' as i32
            && *q as ::core::ffi::c_int <= '9' as i32
        {
            q = q.offset(1);
        }
    } else {
        return ::core::ptr::null::<uint8_t>()
    }
    let mut mag: uint64_t = 0 as uint64_t;
    let mut overflow: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut d: *const uint8_t = if neg != 0 {
        p.offset(1 as ::core::ffi::c_int as isize)
    } else {
        p
    };
    while d < q {
        let mut dg: uint64_t = (*d as ::core::ffi::c_int - '0' as i32) as uint64_t;
        if mag as ::core::ffi::c_ulonglong
            > (0x7fffffffffffffff as ::core::ffi::c_ulonglong)
                .wrapping_sub(dg as ::core::ffi::c_ulonglong)
                .wrapping_div(10 as ::core::ffi::c_ulonglong)
        {
            overflow = 1 as ::core::ffi::c_int;
        } else {
            mag = mag.wrapping_mul(10 as uint64_t).wrapping_add(dg);
        }
        d = d.offset(1);
    }
    let mut is_float: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if q < end && *q as ::core::ffi::c_int == '.' as i32 {
        is_float = 1 as ::core::ffi::c_int;
        q = q.offset(1);
        if q >= end || (*q as ::core::ffi::c_int) < '0' as i32
            || *q as ::core::ffi::c_int > '9' as i32
        {
            return ::core::ptr::null::<uint8_t>();
        }
        while q < end && *q as ::core::ffi::c_int >= '0' as i32
            && *q as ::core::ffi::c_int <= '9' as i32
        {
            q = q.offset(1);
        }
    }
    if q < end
        && (*q as ::core::ffi::c_int == 'e' as i32
            || *q as ::core::ffi::c_int == 'E' as i32)
    {
        is_float = 1 as ::core::ffi::c_int;
        q = q.offset(1);
        if q < end
            && (*q as ::core::ffi::c_int == '+' as i32
                || *q as ::core::ffi::c_int == '-' as i32)
        {
            q = q.offset(1);
        }
        if q >= end || (*q as ::core::ffi::c_int) < '0' as i32
            || *q as ::core::ffi::c_int > '9' as i32
        {
            return ::core::ptr::null::<uint8_t>();
        }
        while q < end && *q as ::core::ffi::c_int >= '0' as i32
            && *q as ::core::ffi::c_int <= '9' as i32
        {
            q = q.offset(1);
        }
    }
    if overflow != 0 {
        is_float = 1 as ::core::ffi::c_int;
        mag = 0 as uint64_t;
    }
    (*v).num_float = (is_float != 0 as ::core::ffi::c_int) as ::core::ffi::c_int
        as uint8_t;
    (*v).num = if neg != 0 { -(mag as int64_t) } else { mag as int64_t };
    (*v).s = p;
    (*v).s_len = q.offset_from(p) as ::core::ffi::c_long as uint32_t;
    return q;
}
unsafe extern "C" fn jf_link(
    mut f: *mut jframe,
    mut root: *mut *mut jv,
    mut v: *mut jv,
) {
    if !(*f).mem.is_null() {
        (*(*f).mem).child = v as *mut jv;
        (*f).mem = ::core::ptr::null_mut::<jv>();
        return;
    }
    if (*f).obj.is_null() {
        *root = v;
        return;
    }
    if (*f).last.is_null() {
        (*(*f).obj).child = v as *mut jv;
    } else {
        (*(*f).last).next = v as *mut jv;
    }
    (*f).last = v;
}
unsafe extern "C" fn jf_after_value(mut f: *mut jframe) {
    if (*f).obj.is_null() {
        (*f).phase = JF_DONE as ::core::ffi::c_int as uint8_t;
    } else if (*(*f).obj).type_0 as ::core::ffi::c_int == JV_ARR as ::core::ffi::c_int {
        (*f).phase = JF_ELEM_END as ::core::ffi::c_int as uint8_t;
    } else {
        (*f).phase = JF_MEMBER_END as ::core::ffi::c_int as uint8_t;
    };
}
#[no_mangle]
pub unsafe extern "C" fn toks_json_parse(
    mut data: *const uint8_t,
    mut len: uint64_t,
    mut ar: *mut toks_arena,
    mut out: *mut *mut jv,
) -> int64_t {
    *out = ::core::ptr::null_mut::<jv>();
    let mut p: *const uint8_t = data;
    let mut end: *const uint8_t = data.offset(len as isize);
    let mut st: [jframe; 64] = [jframe {
        obj: ::core::ptr::null_mut::<jv>(),
        mem: ::core::ptr::null_mut::<jv>(),
        last: ::core::ptr::null_mut::<jv>(),
        phase: 0,
    }; 64];
    let mut root: *mut jv = ::core::ptr::null_mut::<jv>();
    st[0 as ::core::ffi::c_int as usize].obj = ::core::ptr::null_mut::<jv>();
    st[0 as ::core::ffi::c_int as usize].mem = ::core::ptr::null_mut::<jv>();
    st[0 as ::core::ffi::c_int as usize].last = ::core::ptr::null_mut::<jv>();
    st[0 as ::core::ffi::c_int as usize].phase = JF_VALUE as ::core::ffi::c_int
        as uint8_t;
    let mut depth: uint32_t = 1 as uint32_t;
    p = skip_ws(p, end);
    if p >= end {
        return TOKS_E_FORMAT as int64_t;
    }
    loop {
        let mut f: *mut jframe = (&raw mut st as *mut jframe)
            .offset(depth.wrapping_sub(1 as uint32_t) as isize) as *mut jframe;
        if (*f).phase as ::core::ffi::c_int == JF_VALUE as ::core::ffi::c_int {
            let mut v: *mut jv = ::core::ptr::null_mut::<jv>();
            if *p as ::core::ffi::c_int == '{' as i32
                || *p as ::core::ffi::c_int == '[' as i32
            {
                let mut type_0: uint8_t = (if *p as ::core::ffi::c_int == '{' as i32 {
                    JV_OBJ as ::core::ffi::c_int
                } else {
                    JV_ARR as ::core::ffi::c_int
                }) as uint8_t;
                v = jv_new(ar, type_0);
                if v.is_null() {
                    return TOKS_E_NOMEM as int64_t;
                }
                if depth >= TOKS_JSON_DEPTH as uint32_t {
                    return TOKS_E_FORMAT as int64_t;
                }
                jf_link(f, &raw mut root, v);
                let mut q: *const uint8_t = skip_ws(
                    p.offset(1 as ::core::ffi::c_uint as isize),
                    end,
                );
                if q < end
                    && *q as ::core::ffi::c_int
                        == (if *p as ::core::ffi::c_int == '{' as i32 {
                            '}' as i32
                        } else {
                            ']' as i32
                        })
                {
                    p = q.offset(1 as ::core::ffi::c_uint as isize);
                    jf_after_value(f);
                } else {
                    st[depth as usize].obj = v;
                    st[depth as usize].mem = ::core::ptr::null_mut::<jv>();
                    st[depth as usize].last = ::core::ptr::null_mut::<jv>();
                    st[depth as usize].phase = (if type_0 as ::core::ffi::c_int
                        == JV_OBJ as ::core::ffi::c_int
                    {
                        JF_KEY as ::core::ffi::c_int
                    } else {
                        JF_VALUE as ::core::ffi::c_int
                    }) as uint8_t;
                    depth = depth.wrapping_add(1);
                    p = q;
                    if p >= end {
                        return TOKS_E_FORMAT as int64_t;
                    }
                }
            } else {
                if *p as ::core::ffi::c_int == '"' as i32 {
                    v = jv_new(ar, JV_STR as ::core::ffi::c_int as uint8_t);
                    if v.is_null() {
                        return TOKS_E_NOMEM as int64_t;
                    }
                    let mut next: *const uint8_t = ::core::ptr::null::<uint8_t>();
                    let mut r: ::core::ffi::c_int = parse_string(
                        p,
                        end,
                        ar,
                        &raw mut (*v).s,
                        &raw mut (*v).s_len,
                        &raw mut next,
                    );
                    if r == JS_FMT as ::core::ffi::c_int {
                        return TOKS_E_FORMAT as int64_t;
                    }
                    if r == JS_NOMEM as ::core::ffi::c_int {
                        return TOKS_E_NOMEM as int64_t;
                    }
                    p = next;
                } else if end.offset_from(p) as ::core::ffi::c_long
                    >= 4 as ::core::ffi::c_long
                    && *p.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        == 't' as i32
                    && *p.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        == 'r' as i32
                    && *p.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        == 'u' as i32
                    && *p.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        == 'e' as i32
                {
                    v = jv_new(ar, JV_BOOL as ::core::ffi::c_int as uint8_t);
                    if v.is_null() {
                        return TOKS_E_NOMEM as int64_t;
                    }
                    (*v).num = 1 as int64_t;
                    p = p.offset(4 as ::core::ffi::c_uint as isize);
                } else if end.offset_from(p) as ::core::ffi::c_long
                    >= 5 as ::core::ffi::c_long
                    && *p.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        == 'f' as i32
                    && *p.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        == 'a' as i32
                    && *p.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        == 'l' as i32
                    && *p.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        == 's' as i32
                    && *p.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        == 'e' as i32
                {
                    v = jv_new(ar, JV_BOOL as ::core::ffi::c_int as uint8_t);
                    if v.is_null() {
                        return TOKS_E_NOMEM as int64_t;
                    }
                    (*v).num = 0 as int64_t;
                    p = p.offset(5 as ::core::ffi::c_uint as isize);
                } else if end.offset_from(p) as ::core::ffi::c_long
                    >= 4 as ::core::ffi::c_long
                    && *p.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        == 'n' as i32
                    && *p.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        == 'u' as i32
                    && *p.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        == 'l' as i32
                    && *p.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        == 'l' as i32
                {
                    v = jv_new(ar, JV_NULL as ::core::ffi::c_int as uint8_t);
                    if v.is_null() {
                        return TOKS_E_NOMEM as int64_t;
                    }
                    p = p.offset(4 as ::core::ffi::c_uint as isize);
                } else {
                    v = jv_new(ar, JV_NUM as ::core::ffi::c_int as uint8_t);
                    if v.is_null() {
                        return TOKS_E_NOMEM as int64_t;
                    }
                    p = parse_num(p, end, v);
                    if p.is_null() {
                        return TOKS_E_FORMAT as int64_t;
                    }
                }
                jf_link(f, &raw mut root, v);
                jf_after_value(f);
            }
        } else if (*f).phase as ::core::ffi::c_int == JF_KEY as ::core::ffi::c_int {
            if p >= end || *p as ::core::ffi::c_int != '"' as i32 {
                return TOKS_E_FORMAT as int64_t;
            }
            let mut mem: *mut jv = jv_new(ar, JV_MEM as ::core::ffi::c_int as uint8_t);
            if mem.is_null() {
                return TOKS_E_NOMEM as int64_t;
            }
            let mut next_0: *const uint8_t = ::core::ptr::null::<uint8_t>();
            let mut r_0: ::core::ffi::c_int = parse_string(
                p,
                end,
                ar,
                &raw mut (*mem).s,
                &raw mut (*mem).s_len,
                &raw mut next_0,
            );
            if r_0 == JS_FMT as ::core::ffi::c_int {
                return TOKS_E_FORMAT as int64_t;
            }
            if r_0 == JS_NOMEM as ::core::ffi::c_int {
                return TOKS_E_NOMEM as int64_t;
            }
            if (*f).last.is_null() {
                (*(*f).obj).child = mem as *mut jv;
            } else {
                (*(*f).last).next = mem as *mut jv;
            }
            (*f).last = mem;
            (*f).mem = mem;
            p = skip_ws(next_0, end);
            if p >= end || *p as ::core::ffi::c_int != ':' as i32 {
                return TOKS_E_FORMAT as int64_t;
            }
            p = skip_ws(p.offset(1 as ::core::ffi::c_uint as isize), end);
            if p >= end {
                return TOKS_E_FORMAT as int64_t;
            }
            (*f).phase = JF_VALUE as ::core::ffi::c_int as uint8_t;
        } else if (*f).phase as ::core::ffi::c_int == JF_MEMBER_END as ::core::ffi::c_int
            || (*f).phase as ::core::ffi::c_int == JF_ELEM_END as ::core::ffi::c_int
        {
            let mut close: uint8_t = (if (*(*f).obj).type_0 as ::core::ffi::c_int
                == JV_OBJ as ::core::ffi::c_int
            {
                '}' as i32 as uint8_t as ::core::ffi::c_int
            } else {
                ']' as i32 as uint8_t as ::core::ffi::c_int
            }) as uint8_t;
            p = skip_ws(p, end);
            if p >= end {
                return TOKS_E_FORMAT as int64_t;
            }
            if *p as ::core::ffi::c_int == ',' as i32 {
                (*f).phase = (if (*f).phase as ::core::ffi::c_int
                    == JF_MEMBER_END as ::core::ffi::c_int
                {
                    JF_KEY as ::core::ffi::c_int
                } else {
                    JF_VALUE as ::core::ffi::c_int
                }) as uint8_t;
                p = skip_ws(p.offset(1 as ::core::ffi::c_uint as isize), end);
                if p >= end {
                    return TOKS_E_FORMAT as int64_t;
                }
            } else if *p as ::core::ffi::c_int == close as ::core::ffi::c_int {
                depth = depth.wrapping_sub(1);
                p = skip_ws(p.offset(1 as ::core::ffi::c_uint as isize), end);
                jf_after_value(
                    (&raw mut st as *mut jframe)
                        .offset(depth.wrapping_sub(1 as uint32_t) as isize)
                        as *mut jframe,
                );
            } else {
                return TOKS_E_FORMAT as int64_t
            }
        } else {
            p = skip_ws(p, end);
            if p != end {
                return TOKS_E_FORMAT as int64_t;
            }
            break;
        }
    }
    if root.is_null() {
        return TOKS_E_FORMAT as int64_t;
    }
    *out = root;
    return 0 as int64_t;
}
#[no_mangle]
pub unsafe extern "C" fn toks_jv_get(
    mut obj: *const jv,
    mut key: *const ::core::ffi::c_char,
) -> *mut jv {
    if obj.is_null()
        || (*obj).type_0 as ::core::ffi::c_int != JV_OBJ as ::core::ffi::c_int
    {
        return ::core::ptr::null_mut::<jv>();
    }
    let mut kl: uint64_t = 0 as uint64_t;
    while *key.offset(kl as isize) as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
        kl = kl.wrapping_add(1);
    }
    let mut found: *mut jv = ::core::ptr::null_mut::<jv>();
    let mut m: *mut jv = (*obj).child as *mut jv;
    while !m.is_null() {
        if (*m).type_0 as ::core::ffi::c_int == JV_MEM as ::core::ffi::c_int
            && (*m).s_len as uint64_t == kl
            && memcmp(
                (*m).s as *const ::core::ffi::c_void,
                key as *const ::core::ffi::c_void,
                kl as size_t,
            ) == 0 as ::core::ffi::c_int
        {
            found = (*m).child as *mut jv;
        }
        m = (*m).next as *mut jv;
    }
    return found;
}
