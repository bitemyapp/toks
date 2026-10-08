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
    fn toks_norm_char(steps: uint32_t, cp: uint32_t, o: *mut uint32_t) -> uint32_t;
    static toks_bert_cls_s1: [uint16_t; 4352];
    static toks_bert_cls_s2: [uint8_t; 25856];
}
pub type size_t = usize;
pub type __uint8_t = u8;
pub type __uint16_t = u16;
pub type __int32_t = i32;
pub type __uint32_t = u32;
pub type __uint64_t = u64;
pub type int32_t = __int32_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct toks_wp_tables {
    pub flags: uint32_t,
    pub max_chars: uint32_t,
    pub unk_id: uint32_t,
    pub word_maxlen: uint32_t,
    pub cont_maxlen: uint32_t,
    pub prefix_len: uint32_t,
    pub ascii_cls: [uint8_t; 128],
    pub ascii_cls0: [uint8_t; 128],
    pub word: *const toks_wp_entry,
    pub word_mask: uint64_t,
    pub cont: *const toks_wp_entry,
    pub cont_mask: uint64_t,
    pub keys: *const uint8_t,
    pub wtab: *const uint8_t,
    pub wtab_mask: uint64_t,
    pub wcell: *const toks_wp_cell,
    pub ccell: *const toks_wp_cell,
    pub wterm: *const int32_t,
    pub cterm: *const int32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct toks_wp_cell {
    pub base: uint32_t,
    pub check: int32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct toks_wp_entry {
    pub h: uint32_t,
    pub id: uint32_t,
    pub off: uint32_t,
    pub len: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct toks_wp_piece {
    pub off: uint32_t,
    pub len: uint32_t,
    pub end: uint32_t,
    pub flags: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct toks_wp_scan_args {
    pub text: *const uint8_t,
    pub len: uint64_t,
    pub pos: uint64_t,
    pub pieces: *mut toks_wp_piece,
    pub cap: uint64_t,
    pub n: uint64_t,
    pub mat: *mut uint8_t,
    pub mat_cap: uint64_t,
    pub mat_len: uint64_t,
    pub flags: uint64_t,
    pub rsv: [uint64_t; 6],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wps {
    pub t: *const toks_wp_tables,
    pub a: *mut toks_wp_scan_args,
    pub piece_cap: uint64_t,
    pub run_cap: uint64_t,
    pub need: uint64_t,
    pub open: ::core::ffi::c_int,
    pub mat: ::core::ffi::c_int,
    pub hole: ::core::ffi::c_int,
    pub over: ::core::ffi::c_int,
    pub start: uint64_t,
    pub end: uint64_t,
    pub moff: uint64_t,
    pub nch: uint64_t,
    pub run_n: uint64_t,
}
pub const SC_CHANGE: C2RustUnnamed = 6;
pub const SC_CJK: C2RustUnnamed = 5;
pub const SC_INVALID: C2RustUnnamed = 7;
pub const SC_PUNCT: C2RustUnnamed = 2;
pub const SC_HOLE_MN: C2RustUnnamed = 4;
pub const SC_HOLE: C2RustUnnamed = 3;
pub const SC_SPLIT: C2RustUnnamed = 1;
pub const SC_WORD: C2RustUnnamed = 0;
pub type C2RustUnnamed = ::core::ffi::c_uint;
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
pub const TOKS_NS_CLEAN: ::core::ffi::c_uint = 0x1 as ::core::ffi::c_uint;
pub const TOKS_NS_CJK: ::core::ffi::c_uint = 0x2 as ::core::ffi::c_uint;
pub const TOKS_NS_STRIP_MN: ::core::ffi::c_uint = 0x4 as ::core::ffi::c_uint;
pub const TOKS_NS_LOWER: ::core::ffi::c_uint = 0x8 as ::core::ffi::c_uint;
pub const TOKS_NORM_MAX_OUT: ::core::ffi::c_uint = 8 as ::core::ffi::c_uint;
pub const TOKS_NORM_GHOST: ::core::ffi::c_uint = 0x200000 as ::core::ffi::c_uint;
pub const TOKS_BC_REMOVE: ::core::ffi::c_uint = 0x1 as ::core::ffi::c_uint;
pub const TOKS_BC_WS: ::core::ffi::c_uint = 0x2 as ::core::ffi::c_uint;
pub const TOKS_BC_CJK: ::core::ffi::c_uint = 0x4 as ::core::ffi::c_uint;
pub const TOKS_BC_PUNCT: ::core::ffi::c_uint = 0x8 as ::core::ffi::c_uint;
pub const TOKS_BC_DECOMP: ::core::ffi::c_uint = 0x10 as ::core::ffi::c_uint;
pub const TOKS_BC_LOWER: ::core::ffi::c_uint = 0x20 as ::core::ffi::c_uint;
pub const TOKS_BC_NS: ::core::ffi::c_uint = 0x40 as ::core::ffi::c_uint;
pub const TOKS_BC_MN: ::core::ffi::c_uint = 0x80 as ::core::ffi::c_uint;
#[inline]
unsafe extern "C" fn toks_bert_cls(mut cp: uint32_t) -> uint8_t {
    if cp > 0x10ffff as uint32_t {
        return 0 as uint8_t;
    }
    return toks_bert_cls_s2[((toks_bert_cls_s1[(cp >> 8 as ::core::ffi::c_int) as usize]
        as uint32_t) << 8 as ::core::ffi::c_int | cp & 0xff as uint32_t) as usize];
}
pub const TOKS_WPF_CLEAN: ::core::ffi::c_uint = TOKS_NS_CLEAN;
pub const TOKS_WPF_CHINESE: ::core::ffi::c_uint = TOKS_NS_CJK;
pub const TOKS_WPF_STRIP: ::core::ffi::c_uint = TOKS_NS_STRIP_MN;
pub const TOKS_WPF_LOWER: ::core::ffi::c_uint = TOKS_NS_LOWER;
pub const TOKS_WPA_WORD: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
pub const TOKS_WPA_FOLD: ::core::ffi::c_uint = 1 as ::core::ffi::c_uint;
pub const TOKS_WPA_SPLIT: ::core::ffi::c_uint = 2 as ::core::ffi::c_uint;
pub const TOKS_WPA_PUNCT: ::core::ffi::c_uint = 3 as ::core::ffi::c_uint;
pub const TOKS_WPA_REMOVE: ::core::ffi::c_uint = 4 as ::core::ffi::c_uint;
pub const TOKS_WPP_MAT: ::core::ffi::c_uint = 0x1 as ::core::ffi::c_uint;
pub const TOKS_WPP_OVER: ::core::ffi::c_uint = 0x2 as ::core::ffi::c_uint;
pub const TOKS_WPP_INVALID: ::core::ffi::c_uint = 0x4 as ::core::ffi::c_uint;
pub const TOKS_WP_INVALID_CP: ::core::ffi::c_uint = 0xffffffff as ::core::ffi::c_uint;
#[inline]
unsafe extern "C" fn wp_utf8(
    mut p: *const uint8_t,
    mut n: uint64_t,
    mut cp: *mut uint32_t,
) -> uint32_t {
    let mut b0: uint32_t = *p.offset(0 as ::core::ffi::c_int as isize) as uint32_t;
    if b0 < 0x80 as uint32_t {
        *cp = b0;
        return 1 as uint32_t;
    }
    *cp = TOKS_WP_INVALID_CP as uint32_t;
    if b0 < 0xc2 as uint32_t || b0 > 0xf4 as uint32_t || n < 2 as uint64_t {
        return 1 as uint32_t;
    }
    let mut b1: uint32_t = *p.offset(1 as ::core::ffi::c_int as isize) as uint32_t;
    if b0 < 0xe0 as uint32_t {
        if b1 & 0xc0 as uint32_t != 0x80 as uint32_t {
            return 1 as uint32_t;
        }
        *cp = (b0 & 0x1f as uint32_t) << 6 as ::core::ffi::c_int | b1 & 0x3f as uint32_t;
        return 2 as uint32_t;
    }
    let mut lo: uint32_t = 0x80 as uint32_t;
    let mut hi: uint32_t = 0xbf as uint32_t;
    if b0 == 0xe0 as uint32_t {
        lo = 0xa0 as ::core::ffi::c_uint as uint32_t;
    }
    if b0 == 0xed as uint32_t {
        hi = 0x9f as ::core::ffi::c_uint as uint32_t;
    }
    if b0 == 0xf0 as uint32_t {
        lo = 0x90 as ::core::ffi::c_uint as uint32_t;
    }
    if b0 == 0xf4 as uint32_t {
        hi = 0x8f as ::core::ffi::c_uint as uint32_t;
    }
    if b1 < lo || b1 > hi || n < 3 as uint64_t {
        return 1 as uint32_t;
    }
    let mut b2: uint32_t = *p.offset(2 as ::core::ffi::c_int as isize) as uint32_t;
    if b2 & 0xc0 as uint32_t != 0x80 as uint32_t {
        return 1 as uint32_t;
    }
    if b0 < 0xf0 as uint32_t {
        *cp = (b0 & 0xf as uint32_t) << 12 as ::core::ffi::c_int
            | (b1 & 0x3f as uint32_t) << 6 as ::core::ffi::c_int | b2 & 0x3f as uint32_t;
        return 3 as uint32_t;
    }
    if n < 4 as uint64_t {
        return 1 as uint32_t;
    }
    let mut b3: uint32_t = *p.offset(3 as ::core::ffi::c_int as isize) as uint32_t;
    if b3 & 0xc0 as uint32_t != 0x80 as uint32_t {
        return 1 as uint32_t;
    }
    *cp = (b0 & 0x7 as uint32_t) << 18 as ::core::ffi::c_int
        | (b1 & 0x3f as uint32_t) << 12 as ::core::ffi::c_int
        | (b2 & 0x3f as uint32_t) << 6 as ::core::ffi::c_int | b3 & 0x3f as uint32_t;
    return 4 as uint32_t;
}
#[no_mangle]
pub unsafe extern "C" fn toks_wp_ascii_class(
    mut f: uint32_t,
    mut b: uint32_t,
) -> uint8_t {
    if b == 0x9 as uint32_t || b == 0xa as uint32_t || b == 0xd as uint32_t
        || b == 0x20 as uint32_t
    {
        return TOKS_WPA_SPLIT as uint8_t;
    }
    if b == 0xb as uint32_t || b == 0xc as uint32_t {
        return (if f & TOKS_WPF_CLEAN as uint32_t != 0 {
            TOKS_WPA_REMOVE
        } else {
            TOKS_WPA_SPLIT
        }) as uint8_t;
    }
    if b < 0x20 as uint32_t || b == 0x7f as uint32_t {
        return (if f & TOKS_WPF_CLEAN as uint32_t != 0 {
            TOKS_WPA_REMOVE
        } else {
            TOKS_WPA_WORD
        }) as uint8_t;
    }
    if b >= 0x21 as uint32_t && b <= 0x2f as uint32_t
        || b >= 0x3a as uint32_t && b <= 0x40 as uint32_t
        || b >= 0x5b as uint32_t && b <= 0x60 as uint32_t
        || b >= 0x7b as uint32_t && b <= 0x7e as uint32_t
    {
        return TOKS_WPA_PUNCT as uint8_t;
    }
    if b >= 0x41 as uint32_t && b <= 0x5a as uint32_t
        && f & TOKS_WPF_LOWER as uint32_t != 0
    {
        return TOKS_WPA_FOLD as uint8_t;
    }
    return TOKS_WPA_WORD as uint8_t;
}
unsafe extern "C" fn wp_class(mut f: uint32_t, mut cls: uint8_t) -> uint32_t {
    if f & TOKS_WPF_CLEAN as uint32_t != 0
        && cls as ::core::ffi::c_uint & TOKS_BC_REMOVE != 0
    {
        return SC_HOLE as ::core::ffi::c_int as uint32_t;
    }
    if cls as ::core::ffi::c_uint & TOKS_BC_WS != 0 {
        return SC_SPLIT as ::core::ffi::c_int as uint32_t;
    }
    if f & TOKS_WPF_CHINESE as uint32_t != 0
        && cls as ::core::ffi::c_uint & TOKS_BC_CJK != 0
    {
        return SC_CJK as ::core::ffi::c_int as uint32_t;
    }
    if f & TOKS_WPF_STRIP as uint32_t != 0
        && cls as ::core::ffi::c_uint & TOKS_BC_MN != 0
        && cls as ::core::ffi::c_uint & TOKS_BC_DECOMP == 0
    {
        return SC_HOLE_MN as ::core::ffi::c_int as uint32_t;
    }
    if f & TOKS_WPF_STRIP as uint32_t != 0
        && cls as ::core::ffi::c_uint & (TOKS_BC_DECOMP | TOKS_BC_NS) != 0
        || f & TOKS_WPF_LOWER as uint32_t != 0
            && cls as ::core::ffi::c_uint & TOKS_BC_LOWER != 0
    {
        return SC_CHANGE as ::core::ffi::c_int as uint32_t;
    }
    if cls as ::core::ffi::c_uint & TOKS_BC_PUNCT != 0 {
        return SC_PUNCT as ::core::ffi::c_int as uint32_t;
    }
    return SC_WORD as ::core::ffi::c_int as uint32_t;
}
#[no_mangle]
pub unsafe extern "C" fn toks_wp_mat_min(mut t: *const toks_wp_tables) -> uint64_t {
    return (8 as uint64_t)
        .wrapping_mul(((*t).max_chars as uint64_t).wrapping_add(8 as uint64_t))
        .wrapping_add(64 as uint64_t);
}
unsafe extern "C" fn emit(
    mut s: *mut wps,
    mut off: uint32_t,
    mut len: uint64_t,
    mut end: uint64_t,
    mut flags: uint32_t,
) {
    let fresh0 = (*(*s).a).n;
    (*(*s).a).n = (*(*s).a).n.wrapping_add(1);
    let mut p: *mut toks_wp_piece = (*(*s).a).pieces.offset(fresh0 as isize)
        as *mut toks_wp_piece;
    (*p).off = off;
    (*p).len = len as uint32_t;
    (*p).end = end as uint32_t;
    (*p).flags = flags;
}
unsafe extern "C" fn emit1(
    mut s: *mut wps,
    mut off: uint32_t,
    mut len: uint64_t,
    mut end: uint64_t,
    mut flags: uint32_t,
) {
    emit(
        s,
        off,
        len,
        end,
        flags
            | (if (*(*s).t).max_chars == 0 as uint32_t {
                TOKS_WPP_OVER as uint32_t
            } else {
                0 as uint32_t
            }),
    );
}
unsafe extern "C" fn mat_cp(mut s: *mut wps, mut cp: uint32_t) {
    (*(*s).a).mat_len = (*(*s).a)
        .mat_len
        .wrapping_add(
            toks_utf8_put((*(*s).a).mat.offset((*(*s).a).mat_len as isize), cp)
                as uint64_t,
        );
}
unsafe extern "C" fn run_get(mut s: *const wps, mut k: uint64_t) -> uint32_t {
    let mut v: uint32_t = 0;
    memcpy(
        &raw mut v as *mut ::core::ffi::c_void,
        (*(*s).a)
            .mat
            .offset((*s).piece_cap as isize)
            .offset((4 as uint64_t).wrapping_mul(k) as isize)
            as *const ::core::ffi::c_void,
        4 as size_t,
    );
    return v;
}
unsafe extern "C" fn run_flush(mut s: *mut wps) {
    let mut prev: uint32_t = 0 as uint32_t;
    let mut pass: uint32_t = 0 as uint32_t;
    while pass < 256 as uint32_t && (*s).run_n > 0 as uint64_t {
        let mut next: uint32_t = 256 as uint32_t;
        let mut k: uint64_t = 0 as uint64_t;
        while k < (*s).run_n {
            let mut c: uint32_t = run_get(s, k) >> 24 as ::core::ffi::c_int;
            if c > prev && c < next {
                next = c;
            }
            k = k.wrapping_add(1);
        }
        if next == 256 as uint32_t {
            break;
        }
        let mut k_0: uint64_t = 0 as uint64_t;
        while k_0 < (*s).run_n {
            let mut v: uint32_t = run_get(s, k_0);
            if v >> 24 as ::core::ffi::c_int == next {
                mat_cp(s, v & 0xffffff as uint32_t);
            }
            k_0 = k_0.wrapping_add(1);
        }
        prev = next;
        pass = pass.wrapping_add(1);
    }
    (*s).run_n = 0 as uint64_t;
}
unsafe extern "C" fn close_piece(mut s: *mut wps, mut end: uint64_t) {
    if (*s).open == 0 {
        return;
    }
    let mut fl: uint32_t = if (*s).over != 0 {
        TOKS_WPP_OVER as uint32_t
    } else {
        0 as uint32_t
    };
    if (*s).mat != 0 {
        run_flush(s);
        emit(
            s,
            (*s).moff as uint32_t,
            (*(*s).a).mat_len.wrapping_sub((*s).moff),
            end,
            fl | TOKS_WPP_MAT as uint32_t,
        );
    } else {
        emit(s, (*s).start as uint32_t, (*s).end.wrapping_sub((*s).start), end, fl);
    }
    (*s).open = 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn to_mat(mut s: *mut wps) -> ::core::ffi::c_int {
    if (*s).mat != 0 {
        return 1 as ::core::ffi::c_int;
    }
    if (*s).piece_cap < (*(*s).a).mat_len
        || (*s).piece_cap.wrapping_sub((*(*s).a).mat_len) < (*s).need
    {
        return 0 as ::core::ffi::c_int;
    }
    (*s).moff = (*(*s).a).mat_len;
    memcpy(
        (*(*s).a).mat.offset((*(*s).a).mat_len as isize) as *mut ::core::ffi::c_void,
        (*(*s).a).text.offset((*s).start as isize) as *const ::core::ffi::c_void,
        ((*s).end as size_t).wrapping_sub((*s).start as size_t),
    );
    (*(*s).a).mat_len = (*(*s).a)
        .mat_len
        .wrapping_add((*s).end.wrapping_sub((*s).start));
    (*s).mat = 1 as ::core::ffi::c_int;
    (*s).hole = 0 as ::core::ffi::c_int;
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn count_char(mut s: *mut wps) {
    (*s).nch = (*s).nch.wrapping_add(1);
    if (*s).nch > (*(*s).t).max_chars as uint64_t {
        (*s).over = 1 as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn add_raw(
    mut s: *mut wps,
    mut i: uint64_t,
    mut l: uint64_t,
) -> ::core::ffi::c_int {
    if (*s).open == 0 {
        (*s).open = 1 as ::core::ffi::c_int;
        (*s).mat = 0 as ::core::ffi::c_int;
        (*s).hole = 0 as ::core::ffi::c_int;
        (*s).over = 0 as ::core::ffi::c_int;
        (*s).run_n = 0 as uint64_t;
        (*s).start = i;
        (*s).end = i.wrapping_add(l);
        (*s).nch = 0 as uint64_t;
        count_char(s);
        return 1 as ::core::ffi::c_int;
    }
    count_char(s);
    if (*s).over != 0 {
        (*s).end = i.wrapping_add(l);
        return 1 as ::core::ffi::c_int;
    }
    if (*s).mat == 0 && (*s).hole == 0 {
        (*s).end = i.wrapping_add(l);
        return 1 as ::core::ffi::c_int;
    }
    if to_mat(s) == 0 {
        return 0 as ::core::ffi::c_int;
    }
    if (*s).run_n != 0 {
        run_flush(s);
    }
    memcpy(
        (*(*s).a).mat.offset((*(*s).a).mat_len as isize) as *mut ::core::ffi::c_void,
        (*(*s).a).text.offset(i as isize) as *const ::core::ffi::c_void,
        l as size_t,
    );
    (*(*s).a).mat_len = (*(*s).a).mat_len.wrapping_add(l);
    return 1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn toks_wp_scan_c(
    mut t: *const toks_wp_tables,
    mut a: *mut toks_wp_scan_args,
) -> uint64_t {
    let mut f: uint32_t = (*t).flags & (*a).flags as uint32_t;
    let mut own: [uint8_t; 128] = [0; 128];
    let mut acls: *const uint8_t = if f == (*t).flags {
        &raw const (*t).ascii_cls as *const uint8_t
    } else if f == 0 as uint32_t {
        &raw const (*t).ascii_cls0 as *const uint8_t
    } else {
        &raw mut own as *mut uint8_t as *const uint8_t
    };
    if acls == &raw mut own as *mut uint8_t as *const uint8_t {
        let mut b: uint32_t = 0 as uint32_t;
        while b < 128 as uint32_t {
            own[b as usize] = toks_wp_ascii_class(f, b);
            b = b.wrapping_add(1);
        }
    }
    let mut s: wps = wps {
        t: ::core::ptr::null::<toks_wp_tables>(),
        a: ::core::ptr::null_mut::<toks_wp_scan_args>(),
        piece_cap: 0,
        run_cap: 0,
        need: 0,
        open: 0,
        mat: 0,
        hole: 0,
        over: 0,
        start: 0,
        end: 0,
        moff: 0,
        nch: 0,
        run_n: 0,
    };
    memset(
        &raw mut s as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<wps>() as size_t,
    );
    s.t = t;
    s.a = a;
    s.need = (4 as uint64_t)
        .wrapping_mul(((*t).max_chars as uint64_t).wrapping_add(8 as uint64_t));
    s.piece_cap = if (*a).mat_cap > s.need {
        (*a).mat_cap.wrapping_sub(s.need)
    } else {
        0 as uint64_t
    };
    s.run_cap = s.need.wrapping_div(4 as uint64_t);
    (*a).n = 0 as uint64_t;
    let mut text: *const uint8_t = (*a).text;
    let mut len: uint64_t = (*a).len;
    let mut i: uint64_t = (*a).pos;
    let mut o: [uint32_t; 8] = [0; 8];
    while i < len {
        if (*a).n.wrapping_add(2 as uint64_t) > (*a).cap {
            if s.open != 0 && s.mat != 0 {
                (*a).mat_len = s.moff;
            }
            (*a).pos = if s.open != 0 { s.start } else { i };
            return (*a).n;
        }
        let mut b_0: uint32_t = *text.offset(i as isize) as uint32_t;
        if b_0 < 0x80 as uint32_t
            && *acls.offset(b_0 as isize) as ::core::ffi::c_uint <= TOKS_WPA_FOLD
        {
            let mut j: uint64_t = i.wrapping_add(1 as uint64_t);
            while j < len
                && (*text.offset(j as isize) as ::core::ffi::c_uint)
                    < 0x80 as ::core::ffi::c_uint
                && *acls.offset(*text.offset(j as isize) as isize) as ::core::ffi::c_uint
                    <= TOKS_WPA_FOLD
            {
                j = j.wrapping_add(1);
            }
            if s.open == 0 {
                s.open = 1 as ::core::ffi::c_int;
                s.mat = 0 as ::core::ffi::c_int;
                s.hole = 0 as ::core::ffi::c_int;
                s.over = 0 as ::core::ffi::c_int;
                s.run_n = 0 as uint64_t;
                s.start = i;
                s.end = i;
                s.nch = 0 as uint64_t;
            }
            if s.over != 0 || s.mat == 0 && s.hole == 0 {
                s.nch = s.nch.wrapping_add(j.wrapping_sub(i));
                if s.nch > (*t).max_chars as uint64_t {
                    s.over = 1 as ::core::ffi::c_int;
                }
                s.end = j;
                i = j;
            } else {
                while i < j {
                    if add_raw(&raw mut s, i, 1 as uint64_t) == 0 {
                        (*a).pos = s.start;
                        return (*a).n;
                    }
                    i = i.wrapping_add(1);
                }
            }
        } else {
            let mut cp: uint32_t = b_0;
            let mut l: uint64_t = 1 as uint64_t;
            let mut c: uint32_t = 0;
            let mut cls: uint8_t = 0 as uint8_t;
            if b_0 < 0x80 as uint32_t {
                let mut ac: uint8_t = *acls.offset(b_0 as isize);
                c = (if ac as ::core::ffi::c_uint == TOKS_WPA_SPLIT {
                    SC_SPLIT as ::core::ffi::c_int
                } else if ac as ::core::ffi::c_uint == TOKS_WPA_PUNCT {
                    SC_PUNCT as ::core::ffi::c_int
                } else if ac as ::core::ffi::c_uint == TOKS_WPA_REMOVE {
                    SC_HOLE as ::core::ffi::c_int
                } else {
                    SC_WORD as ::core::ffi::c_int
                }) as uint32_t;
            } else {
                l = wp_utf8(text.offset(i as isize), len.wrapping_sub(i), &raw mut cp)
                    as uint64_t;
                if cp == TOKS_WP_INVALID_CP as uint32_t {
                    c = SC_INVALID as ::core::ffi::c_int as uint32_t;
                } else {
                    cls = toks_bert_cls(cp);
                    c = wp_class(f, cls);
                }
            }
            match c {
                0 => {
                    if add_raw(&raw mut s, i, l) == 0 {
                        (*a).pos = s.start;
                        return (*a).n;
                    }
                }
                1 => {
                    close_piece(&raw mut s, i);
                }
                3 => {
                    if s.open != 0 && s.mat == 0 {
                        s.hole = 1 as ::core::ffi::c_int;
                    }
                }
                4 => {
                    if s.open != 0 {
                        if s.mat != 0 {
                            if cls as ::core::ffi::c_uint & TOKS_BC_NS
                                == 0 as ::core::ffi::c_uint && s.run_n != 0
                            {
                                run_flush(&raw mut s);
                            }
                        } else {
                            s.hole = 1 as ::core::ffi::c_int;
                        }
                    }
                }
                2 | 7 => {
                    close_piece(&raw mut s, i);
                    emit1(
                        &raw mut s,
                        i as uint32_t,
                        l,
                        i.wrapping_add(l),
                        if c == SC_INVALID as ::core::ffi::c_int as uint32_t {
                            TOKS_WPP_INVALID as uint32_t
                        } else {
                            0 as uint32_t
                        },
                    );
                }
                5 => {
                    close_piece(&raw mut s, i);
                    let mut n: uint32_t = toks_norm_char(
                        f,
                        cp,
                        &raw mut o as *mut uint32_t,
                    );
                    if n == 1 as uint32_t && o[0 as ::core::ffi::c_int as usize] == cp {
                        emit1(
                            &raw mut s,
                            i as uint32_t,
                            l,
                            i.wrapping_add(l),
                            0 as uint32_t,
                        );
                    } else {
                        let mut m0: uint64_t = (*a).mat_len;
                        if s.piece_cap < m0
                            || s.piece_cap.wrapping_sub(m0)
                                < (4 as ::core::ffi::c_uint).wrapping_mul(TOKS_NORM_MAX_OUT)
                                    as uint64_t
                        {
                            (*a).pos = i;
                            return (*a).n;
                        }
                        let mut k: uint32_t = 0 as uint32_t;
                        while k < n {
                            if o[k as usize] != TOKS_NORM_GHOST as uint32_t {
                                mat_cp(&raw mut s, o[k as usize]);
                            }
                            k = k.wrapping_add(1);
                        }
                        if (*a).mat_len > m0 {
                            emit1(
                                &raw mut s,
                                m0 as uint32_t,
                                (*a).mat_len.wrapping_sub(m0),
                                i.wrapping_add(l),
                                TOKS_WPP_MAT as uint32_t,
                            );
                        }
                    }
                }
                6 => {
                    let mut n_0: uint32_t = toks_norm_char(
                        f,
                        cp,
                        &raw mut o as *mut uint32_t,
                    );
                    let mut cut: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                    let mut added: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                    let mut k_0: uint32_t = 0 as uint32_t;
                    while k_0 < n_0 {
                        if o[k_0 as usize] == TOKS_NORM_GHOST as uint32_t {
                            if s.open != 0 && s.mat != 0 && s.run_n != 0 {
                                run_flush(&raw mut s);
                            }
                        } else {
                            let mut xc: uint8_t = toks_bert_cls(
                                o[k_0 as usize] & 0xffffff as uint32_t,
                            );
                            if xc as ::core::ffi::c_uint & (TOKS_BC_PUNCT | TOKS_BC_WS)
                                != 0
                                || f & TOKS_WPF_CHINESE as uint32_t != 0
                                    && xc as ::core::ffi::c_uint & TOKS_BC_CJK != 0
                            {
                                close_piece(&raw mut s, i);
                                if xc as ::core::ffi::c_uint & TOKS_BC_WS == 0 {
                                    let mut m0_0: uint64_t = (*a).mat_len;
                                    if s.piece_cap < m0_0
                                        || s.piece_cap.wrapping_sub(m0_0) < 4 as uint64_t
                                    {
                                        (*a).pos = i;
                                        return (*a).n;
                                    }
                                    mat_cp(&raw mut s, o[k_0 as usize]);
                                    emit1(
                                        &raw mut s,
                                        m0_0 as uint32_t,
                                        (*a).mat_len.wrapping_sub(m0_0),
                                        i.wrapping_add(l),
                                        TOKS_WPP_MAT as uint32_t,
                                    );
                                }
                                cut = 1 as ::core::ffi::c_int;
                            } else {
                                if s.open == 0 {
                                    s.open = 1 as ::core::ffi::c_int;
                                    s.mat = 0 as ::core::ffi::c_int;
                                    s.hole = 0 as ::core::ffi::c_int;
                                    s.over = 0 as ::core::ffi::c_int;
                                    s.run_n = 0 as uint64_t;
                                    s.start = i;
                                    s.end = i;
                                    s.nch = 0 as uint64_t;
                                }
                                count_char(&raw mut s);
                                added = 1 as ::core::ffi::c_int;
                                if !(s.over != 0) {
                                    if to_mat(&raw mut s) == 0 {
                                        (*a).pos = s.start;
                                        return (*a).n;
                                    }
                                    if o[k_0 as usize] >> 24 as ::core::ffi::c_int
                                        == 0 as uint32_t
                                    {
                                        if s.run_n != 0 {
                                            run_flush(&raw mut s);
                                        }
                                        mat_cp(&raw mut s, o[k_0 as usize]);
                                    } else if s.run_n < s.run_cap {
                                        memcpy(
                                            (*a)
                                                .mat
                                                .offset(s.piece_cap as isize)
                                                .offset((4 as uint64_t).wrapping_mul(s.run_n) as isize)
                                                as *mut ::core::ffi::c_void,
                                            (&raw mut o as *mut uint32_t).offset(k_0 as isize)
                                                as *mut uint32_t as *const ::core::ffi::c_void,
                                            4 as size_t,
                                        );
                                        s.run_n = s.run_n.wrapping_add(1);
                                    }
                                }
                            }
                        }
                        k_0 = k_0.wrapping_add(1);
                    }
                    if s.open != 0 && cut == 0 {
                        if s.mat != 0 || s.over != 0 {
                            s.end = i.wrapping_add(l);
                        } else if added == 0 {
                            s.hole = 1 as ::core::ffi::c_int;
                        }
                    }
                }
                _ => {}
            }
            i = i.wrapping_add(l);
        }
    }
    close_piece(&raw mut s, len);
    (*a).pos = len;
    return (*a).n;
}
