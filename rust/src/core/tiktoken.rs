#[repr(C)]
pub struct toks_gen { _opaque: [u8; 0] }
#[repr(C)]
pub struct toks_uni_src { _opaque: [u8; 0] }
#[repr(C)]
pub struct toks_spm_config { _opaque: [u8; 0] }
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
    fn toks_b64v(c: uint8_t) -> ::core::ffi::c_int;
    fn toks_b64_len(p: *const uint8_t, n: uint64_t) -> int64_t;
    fn toks_b64_decode(
        p: *const uint8_t,
        n: uint64_t,
        out: *mut uint8_t,
        cap: uint64_t,
    ) -> int64_t;
    fn toks_json_parse(
        data: *const uint8_t,
        len: uint64_t,
        ar: *mut toks_arena,
        out: *mut *mut jv,
    ) -> int64_t;
    fn toks_jv_get(obj: *const jv, key: *const ::core::ffi::c_char) -> *mut jv;
    static TOKS_PATTERNS: [toks_pattern; 0];
    static TOKS_PATTERN_KIMI: toks_pattern;
    fn toks_sidx_init(
        x: *mut toks_sidx,
        ar: *mut toks_arena,
        s: *const *const uint8_t,
        len: *const uint32_t,
        n: uint64_t,
    ) -> int64_t;
    fn toks_sidx_find(
        x: *const toks_sidx,
        a: *const uint8_t,
        al: uint32_t,
        b: *const uint8_t,
        bl: uint32_t,
    ) -> int64_t;
    fn toks_sidx_add(x: *mut toks_sidx, e: uint32_t) -> ::core::ffi::c_int;
    fn toks_gen_max_bytes() -> uint64_t;
}
pub type size_t = usize;
pub type __uint8_t = u8;
pub type __uint16_t = u16;
pub type __int32_t = i32;
pub type __uint32_t = u32;
pub type __int64_t = i64;
pub type __uint64_t = u64;
pub type int32_t = __int32_t;
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
pub struct toks_err {
    pub code: int64_t,
    pub what: *const ::core::ffi::c_char,
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
pub struct toks_sidx {
    pub s: *const *const uint8_t,
    pub len: *const uint32_t,
    pub slot: *mut uint32_t,
    pub mask: uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct toks_tiktoken_info {
    pub n_ranks: uint32_t,
    pub first_special: uint32_t,
    pub chunk_chars: uint32_t,
    pub run_chars: uint32_t,
    pub n_trie: uint32_t,
    pub n_named: uint32_t,
    pub trie: [uint8_t; 32],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct line {
    pub tok: uint64_t,
    pub tok_n: uint64_t,
    pub raw_n: uint32_t,
    pub rank: uint32_t,
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
pub const TOKS_E_FORMAT: ::core::ffi::c_int = -(2 as ::core::ffi::c_int);
pub const TOKS_E_UNSUPPORTED: ::core::ffi::c_int = -(3 as ::core::ffi::c_int);
pub const TOKS_E_LIMIT: ::core::ffi::c_int = -(9 as ::core::ffi::c_int);
pub const TOKS_E_NOMEM: ::core::ffi::c_int = -(11 as ::core::ffi::c_int);
pub const TOKS_MAX_TOKEN_BYTES: ::core::ffi::c_uint = 65535 as ::core::ffi::c_uint;
pub const TOKS_MAX_ADDED_BYTES: ::core::ffi::c_uint = 255 as ::core::ffi::c_uint;
pub const TOKS_MAX_IDS: ::core::ffi::c_uint = ((1 as ::core::ffi::c_uint)
    << 21 as ::core::ffi::c_int)
    .wrapping_sub(1 as ::core::ffi::c_uint);
pub const TOKS_MAX_SOURCE_BYTES: ::core::ffi::c_ulonglong = (256
    as ::core::ffi::c_ulonglong) << 20 as ::core::ffi::c_int;
pub const TOKS_ID_SPECIAL: ::core::ffi::c_uint = 2 as ::core::ffi::c_uint;
pub const TOKS_PRIO_BITS: ::core::ffi::c_int = 22 as ::core::ffi::c_int;
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
unsafe extern "C" fn toks_jstr(
    mut v: *const jv,
    mut s: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut n: uint64_t = 0 as uint64_t;
    while *s.offset(n as isize) as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
        n = n.wrapping_add(1);
    }
    return (!v.is_null()
        && (*v).type_0 as ::core::ffi::c_int == JV_STR as ::core::ffi::c_int
        && (*v).s_len as uint64_t == n
        && (n == 0 as uint64_t
            || memcmp(
                (*v).s as *const ::core::ffi::c_void,
                s as *const ::core::ffi::c_void,
                n as size_t,
            ) == 0 as ::core::ffi::c_int)) as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn toks_fail(
    mut err: *mut toks_err,
    mut code: int64_t,
    mut what: *const ::core::ffi::c_char,
) -> int64_t {
    (*err).code = code;
    (*err).what = what;
    return code;
}
#[inline]
unsafe extern "C" fn toks_config_arena_bound(mut len: uint64_t) -> uint64_t {
    return (32 as uint64_t)
        .wrapping_mul(len)
        .wrapping_add(65536 as uint64_t)
        .wrapping_add(toks_gen_max_bytes());
}
pub const TOKS_TIKTOKEN_RESERVED: ::core::ffi::c_uint = 256 as ::core::ffi::c_uint;
pub const TOKS_NS_CANON: ::core::ffi::c_uint = 0x10 as ::core::ffi::c_uint;
pub const TOKS_NS_COMPOSE: ::core::ffi::c_uint = 0x20 as ::core::ffi::c_uint;
pub const TOKS_NS_NFC: ::core::ffi::c_uint = TOKS_NS_CANON | TOKS_NS_COMPOSE;
unsafe extern "C" fn alphabet(mut b2u: *mut uint16_t) {
    let mut n: uint16_t = 0 as uint16_t;
    let mut b: uint32_t = 0 as uint32_t;
    while b < 256 as uint32_t {
        let mut self_0: ::core::ffi::c_int = (b >= 0x21 as uint32_t
            && b <= 0x7e as uint32_t || b >= 0xa1 as uint32_t && b <= 0xac as uint32_t
            || b >= 0xae as uint32_t) as ::core::ffi::c_int;
        *b2u.offset(b as isize) = (if self_0 != 0 {
            b as uint16_t as ::core::ffi::c_int
        } else {
            let fresh5 = n;
            n = n.wrapping_add(1);
            (0x100 as ::core::ffi::c_uint).wrapping_add(fresh5 as ::core::ffi::c_uint)
                as uint16_t as ::core::ffi::c_int
        }) as uint16_t;
        b = b.wrapping_add(1);
    }
}
unsafe extern "C" fn parse_line(
    mut d: *const uint8_t,
    mut s: uint64_t,
    mut e: uint64_t,
    mut l: *mut line,
) -> *const ::core::ffi::c_char {
    let mut sp: uint64_t = s;
    while sp < e && *d.offset(sp as isize) as ::core::ffi::c_int != ' ' as i32 {
        sp = sp.wrapping_add(1);
    }
    if sp == e {
        return b"tiktoken.model: a line without the space between token and rank\0"
            as *const u8 as *const ::core::ffi::c_char;
    }
    let mut i: uint64_t = s;
    while i < e {
        if *d.offset(i as isize) as ::core::ffi::c_int == '\r' as i32
            || *d.offset(i as isize) as ::core::ffi::c_int == '\t' as i32
            || *d.offset(i as isize) as ::core::ffi::c_int == ' ' as i32 && i != sp
        {
            return b"tiktoken.model: a line that is not '<base64> <rank>' (one space, LF line ends)\0"
                as *const u8 as *const ::core::ffi::c_char;
        }
        i = i.wrapping_add(1);
    }
    let mut rn: int64_t = toks_b64_len(d.offset(s as isize), sp.wrapping_sub(s));
    if rn < 0 as int64_t {
        return b"tiktoken.model: a token that is not canonical base64\0" as *const u8
            as *const ::core::ffi::c_char;
    }
    if rn > TOKS_MAX_TOKEN_BYTES as int64_t {
        return b"tiktoken.model: a token above TOKS_MAX_TOKEN_BYTES\0" as *const u8
            as *const ::core::ffi::c_char;
    }
    let mut ds: uint64_t = sp.wrapping_add(1 as uint64_t);
    let mut dn: uint64_t = e.wrapping_sub(ds);
    if dn == 0 as uint64_t || dn > 7 as uint64_t
        || *d.offset(ds as isize) as ::core::ffi::c_int == '0' as i32
            && dn > 1 as uint64_t
    {
        return b"tiktoken.model: a rank that is not a canonical decimal below TOKS_MAX_IDS\0"
            as *const u8 as *const ::core::ffi::c_char;
    }
    let mut r: uint64_t = 0 as uint64_t;
    let mut i_0: uint64_t = ds;
    while i_0 < e {
        if (*d.offset(i_0 as isize) as ::core::ffi::c_int) < '0' as i32
            || *d.offset(i_0 as isize) as ::core::ffi::c_int > '9' as i32
        {
            return b"tiktoken.model: a rank that is not a canonical decimal below TOKS_MAX_IDS\0"
                as *const u8 as *const ::core::ffi::c_char;
        }
        r = r
            .wrapping_mul(10 as uint64_t)
            .wrapping_add(
                (*d.offset(i_0 as isize) as ::core::ffi::c_int - '0' as i32) as uint64_t,
            );
        i_0 = i_0.wrapping_add(1);
    }
    if r >= TOKS_MAX_IDS as uint64_t {
        return b"tiktoken.model: a rank that is not a canonical decimal below TOKS_MAX_IDS\0"
            as *const u8 as *const ::core::ffi::c_char;
    }
    (*l).tok = s;
    (*l).tok_n = sp.wrapping_sub(s);
    (*l).raw_n = rn as uint32_t;
    (*l).rank = r as uint32_t;
    return ::core::ptr::null::<::core::ffi::c_char>();
}
unsafe extern "C" fn next_line(
    mut d: *const uint8_t,
    mut len: uint64_t,
    mut s: uint64_t,
    mut e: *mut uint64_t,
    mut next: *mut uint64_t,
) {
    let mut i: uint64_t = s;
    while i < len && *d.offset(i as isize) as ::core::ffi::c_int != '\n' as i32 {
        i = i.wrapping_add(1);
    }
    *next = if i < len { i.wrapping_add(1 as uint64_t) } else { len };
    *e = if i > s
        && *d.offset(i.wrapping_sub(1 as uint64_t) as isize) as ::core::ffi::c_int
            == '\r' as i32
    {
        i.wrapping_sub(1 as uint64_t)
    } else {
        i
    };
}
#[no_mangle]
pub unsafe extern "C" fn toks_tiktoken_arena_bound(
    mut ranks_len: uint64_t,
    mut config_len: uint64_t,
    mut wrapper_len: uint64_t,
) -> uint64_t {
    return (16 as uint64_t)
        .wrapping_mul(ranks_len)
        .wrapping_add((24 as uint64_t).wrapping_mul(TOKS_MAX_IDS as uint64_t))
        .wrapping_add(toks_config_arena_bound(config_len))
        .wrapping_add(
            (TOKS_TIKTOKEN_RESERVED as uint64_t)
                .wrapping_mul(
                    (3 as uint64_t)
                        .wrapping_mul(
                            ::core::mem::size_of::<toks_cfg_added>() as uint64_t,
                        )
                        .wrapping_add(64 as uint64_t),
                ),
        )
        .wrapping_add(65536 as uint64_t);
}
#[no_mangle]
pub unsafe extern "C" fn toks_tiktoken_ranks(
    mut d: *const uint8_t,
    mut len: uint64_t,
    mut ar: *mut toks_arena,
    mut cfg: *mut toks_config,
    mut err: *mut toks_err,
) -> int64_t {
    memset(
        cfg as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<toks_config>() as size_t,
    );
    (*err).code = 0 as int64_t;
    (*err).what = ::core::ptr::null::<::core::ffi::c_char>();
    if len == 0 as uint64_t {
        return toks_fail(
            err,
            TOKS_E_FORMAT as int64_t,
            b"tiktoken.model: empty\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if len as ::core::ffi::c_ulonglong > TOKS_MAX_SOURCE_BYTES {
        return toks_fail(
            err,
            TOKS_E_LIMIT as int64_t,
            b"tiktoken.model: above 256 MiB\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    let mut n_tok: uint64_t = 0 as uint64_t;
    let mut raw_total: uint64_t = 0 as uint64_t;
    let mut max_rank: uint64_t = 0 as uint64_t;
    let mut s: uint64_t = 0 as uint64_t;
    let mut e: uint64_t = 0 as uint64_t;
    let mut nx: uint64_t = 0 as uint64_t;
    while s < len {
        next_line(d, len, s, &raw mut e, &raw mut nx);
        if !(e == s) {
            let mut l: line = line {
                tok: 0,
                tok_n: 0,
                raw_n: 0,
                rank: 0,
            };
            let mut why: *const ::core::ffi::c_char = parse_line(d, s, e, &raw mut l);
            if !why.is_null() {
                return toks_fail(err, TOKS_E_FORMAT as int64_t, why);
            }
            n_tok = n_tok.wrapping_add(1);
            raw_total = raw_total.wrapping_add(l.raw_n as uint64_t);
            if l.rank as uint64_t > max_rank {
                max_rank = l.rank as uint64_t;
            }
        }
        s = nx;
    }
    if n_tok == 0 as uint64_t {
        return toks_fail(
            err,
            TOKS_E_FORMAT as int64_t,
            b"tiktoken.model: no tokens\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    let mut nv: uint32_t = (max_rank as uint32_t).wrapping_add(1 as uint32_t);
    let mut raw: *mut uint8_t = toks_ar_alloc(
        ar,
        raw_total.wrapping_add(1 as uint64_t),
        8 as uint64_t,
    ) as *mut uint8_t;
    let mut rp: *mut *const uint8_t = toks_ar_alloc(
        ar,
        (8 as uint64_t).wrapping_mul(nv as uint64_t),
        8 as uint64_t,
    ) as *mut *const uint8_t;
    let mut rl: *mut uint32_t = toks_ar_alloc(
        ar,
        (4 as uint64_t).wrapping_mul(nv as uint64_t),
        8 as uint64_t,
    ) as *mut uint32_t;
    let mut x: toks_sidx = toks_sidx {
        s: ::core::ptr::null::<*const uint8_t>(),
        len: ::core::ptr::null::<uint32_t>(),
        slot: ::core::ptr::null_mut::<uint32_t>(),
        mask: 0,
    };
    if raw.is_null() || rp.is_null() || rl.is_null()
        || toks_sidx_init(&raw mut x, ar, rp, rl, n_tok) != 0 as int64_t
    {
        return toks_fail(
            err,
            TOKS_E_NOMEM as int64_t,
            b"tiktoken.model arrays\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    memset(
        rl as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        (4 as size_t).wrapping_mul(nv as size_t),
    );
    memset(
        rp as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        (8 as size_t).wrapping_mul(nv as size_t),
    );
    let mut ro: uint64_t = 0 as uint64_t;
    let mut s_0: uint64_t = 0 as uint64_t;
    let mut e_0: uint64_t = 0 as uint64_t;
    let mut nx_0: uint64_t = 0 as uint64_t;
    while s_0 < len {
        next_line(d, len, s_0, &raw mut e_0, &raw mut nx_0);
        if !(e_0 == s_0) {
            let mut l_0: line = line {
                tok: 0,
                tok_n: 0,
                raw_n: 0,
                rank: 0,
            };
            parse_line(d, s_0, e_0, &raw mut l_0);
            if *rl.offset(l_0.rank as isize) != 0 as uint32_t {
                return toks_fail(
                    err,
                    TOKS_E_FORMAT as int64_t,
                    b"tiktoken.model: a rank appears twice\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
            }
            toks_b64_decode(
                d.offset(l_0.tok as isize),
                l_0.tok_n,
                raw.offset(ro as isize),
                l_0.raw_n as uint64_t,
            );
            let ref mut fresh0 = *rp.offset(l_0.rank as isize);
            *fresh0 = raw.offset(ro as isize);
            *rl.offset(l_0.rank as isize) = l_0.raw_n;
            if toks_sidx_add(&raw mut x, l_0.rank) != 0 as ::core::ffi::c_int {
                return toks_fail(
                    err,
                    TOKS_E_FORMAT as int64_t,
                    b"tiktoken.model: a token appears twice\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
            }
            ro = ro.wrapping_add(l_0.raw_n as uint64_t);
        }
        s_0 = nx_0;
    }
    let mut b: uint32_t = 0 as uint32_t;
    while b < 256 as uint32_t {
        let mut one: uint8_t = b as uint8_t;
        if toks_sidx_find(
            &raw mut x,
            &raw mut one,
            1 as uint32_t,
            ::core::ptr::null::<uint8_t>(),
            0 as uint32_t,
        ) < 0 as int64_t
        {
            return toks_fail(
                err,
                TOKS_E_UNSUPPORTED as int64_t,
                b"tiktoken.model lacks a one-byte token\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        b = b.wrapping_add(1);
    }
    let mut b2u: [uint16_t; 256] = [0; 256];
    alphabet(&raw mut b2u as *mut uint16_t);
    let mut alen: uint64_t = 0 as uint64_t;
    let mut i: uint64_t = 0 as uint64_t;
    while i < raw_total {
        alen = alen
            .wrapping_add(
                (if (b2u[*raw.offset(i as isize) as usize] as ::core::ffi::c_uint)
                    < 0x80 as ::core::ffi::c_uint
                {
                    1 as ::core::ffi::c_uint
                } else {
                    2 as ::core::ffi::c_uint
                }) as uint64_t,
            );
        i = i.wrapping_add(1);
    }
    let mut astr: *mut uint8_t = toks_ar_alloc(
        ar,
        alen.wrapping_add(1 as uint64_t),
        8 as uint64_t,
    ) as *mut uint8_t;
    let mut vs: *mut *const uint8_t = toks_ar_alloc(
        ar,
        (8 as uint64_t).wrapping_mul(nv as uint64_t),
        8 as uint64_t,
    ) as *mut *const uint8_t;
    let mut vl: *mut uint32_t = toks_ar_alloc(
        ar,
        (4 as uint64_t).wrapping_mul(nv as uint64_t),
        8 as uint64_t,
    ) as *mut uint32_t;
    if astr.is_null() || vs.is_null() || vl.is_null() {
        return toks_fail(
            err,
            TOKS_E_NOMEM as int64_t,
            b"tiktoken.model vocabulary\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    let mut ao: uint64_t = 0 as uint64_t;
    let mut id: uint32_t = 0 as uint32_t;
    while id < nv {
        let ref mut fresh1 = *vs.offset(id as isize);
        *fresh1 = astr.offset(ao as isize);
        let mut k: uint32_t = 0 as uint32_t;
        while k < *rl.offset(id as isize) {
            let mut cp: uint32_t = b2u[*(*rp.offset(id as isize)).offset(k as isize)
                as usize] as uint32_t;
            if cp < 0x80 as uint32_t {
                let fresh2 = ao;
                ao = ao.wrapping_add(1);
                *astr.offset(fresh2 as isize) = cp as uint8_t;
            } else {
                let fresh3 = ao;
                ao = ao.wrapping_add(1);
                *astr.offset(fresh3 as isize) = (0xc0 as uint32_t
                    | cp >> 6 as ::core::ffi::c_int) as uint8_t;
                let fresh4 = ao;
                ao = ao.wrapping_add(1);
                *astr.offset(fresh4 as isize) = (0x80 as uint32_t
                    | cp & 0x3f as uint32_t) as uint8_t;
            }
            k = k.wrapping_add(1);
        }
        *vl.offset(id as isize) = astr
            .offset(ao as isize)
            .offset_from(*vs.offset(id as isize)) as ::core::ffi::c_long as uint32_t;
        id = id.wrapping_add(1);
    }
    let mut nm: uint64_t = 0 as uint64_t;
    let mut pass: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while pass < 2 as ::core::ffi::c_int {
        let mut ml: *mut uint32_t = ::core::ptr::null_mut::<uint32_t>();
        let mut mr: *mut uint32_t = ::core::ptr::null_mut::<uint32_t>();
        let mut mo: *mut uint32_t = ::core::ptr::null_mut::<uint32_t>();
        if pass == 1 as ::core::ffi::c_int {
            if nm >= ((1 as ::core::ffi::c_uint) << TOKS_PRIO_BITS) as uint64_t {
                return toks_fail(
                    err,
                    TOKS_E_LIMIT as int64_t,
                    b"tiktoken.model: merges > 2^22\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
            }
            ml = toks_ar_alloc(
                ar,
                (4 as uint64_t).wrapping_mul(nm).wrapping_add(4 as uint64_t),
                8 as uint64_t,
            ) as *mut uint32_t;
            mr = toks_ar_alloc(
                ar,
                (4 as uint64_t).wrapping_mul(nm).wrapping_add(4 as uint64_t),
                8 as uint64_t,
            ) as *mut uint32_t;
            mo = toks_ar_alloc(
                ar,
                (4 as uint64_t).wrapping_mul(nm).wrapping_add(4 as uint64_t),
                8 as uint64_t,
            ) as *mut uint32_t;
            if ml.is_null() || mr.is_null() || mo.is_null() {
                return toks_fail(
                    err,
                    TOKS_E_NOMEM as int64_t,
                    b"tiktoken.model merges\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
        }
        let mut m: uint64_t = 0 as uint64_t;
        let mut id_0: uint32_t = 0 as uint32_t;
        while id_0 < nv {
            let mut t: *const uint8_t = *rp.offset(id_0 as isize);
            let mut k_0: uint32_t = 1 as uint32_t;
            while k_0 < *rl.offset(id_0 as isize) {
                let mut a: int64_t = toks_sidx_find(
                    &raw mut x,
                    t,
                    k_0,
                    ::core::ptr::null::<uint8_t>(),
                    0 as uint32_t,
                );
                if !(a < 0 as int64_t) {
                    let mut b_0: int64_t = toks_sidx_find(
                        &raw mut x,
                        t.offset(k_0 as isize),
                        (*rl.offset(id_0 as isize)).wrapping_sub(k_0),
                        ::core::ptr::null::<uint8_t>(),
                        0 as uint32_t,
                    );
                    if !(b_0 < 0 as int64_t) {
                        if pass == 1 as ::core::ffi::c_int {
                            *ml.offset(m as isize) = a as uint32_t;
                            *mr.offset(m as isize) = b_0 as uint32_t;
                            *mo.offset(m as isize) = id_0;
                        }
                        m = m.wrapping_add(1);
                    }
                }
                k_0 = k_0.wrapping_add(1);
            }
            id_0 = id_0.wrapping_add(1);
        }
        nm = m;
        if pass == 1 as ::core::ffi::c_int {
            (*cfg).m_left_id = ml;
            (*cfg).m_right_id = mr;
            (*cfg).m_out_id = mo;
        }
        pass += 1;
    }
    (*cfg).vocab = vs;
    (*cfg).vocab_len = vl;
    (*cfg).n_vocab = nv;
    (*cfg).n_merges = nm as uint32_t;
    (*cfg).n_ids = nv;
    (*cfg).ignore_merges = 1 as uint8_t;
    (*cfg).ids_as_rank = 1 as uint8_t;
    (*cfg).dec_byte_level = 1 as uint8_t;
    (*cfg).pattern = ::core::ptr::null::<toks_pattern>();
    return 0 as int64_t;
}
static mut WRAPPER_LINES: [*const ::core::ffi::c_char; 15] = [
    b"num_reserved_special_tokens = 256\0" as *const u8 as *const ::core::ffi::c_char,
    b"r\"\"\"[\\p{Han}]+\"\"\",\0" as *const u8 as *const ::core::ffi::c_char,
    b"r\"\"\"[^\\r\\n\\p{L}\\p{N}]?[\\p{Lu}\\p{Lt}\\p{Lm}\\p{Lo}\\p{M}&&[^\\p{Han}]]*[\\p{Ll}\\p{Lm}\\p{Lo}\\p{M}&&[^\\p{Han}]]+(?i:'s|'t|'re|'ve|'m|'ll|'d)?\"\"\",\0"
        as *const u8 as *const ::core::ffi::c_char,
    b"r\"\"\"[^\\r\\n\\p{L}\\p{N}]?[\\p{Lu}\\p{Lt}\\p{Lm}\\p{Lo}\\p{M}&&[^\\p{Han}]]+[\\p{Ll}\\p{Lm}\\p{Lo}\\p{M}&&[^\\p{Han}]]*(?i:'s|'t|'re|'ve|'m|'ll|'d)?\"\"\",\0"
        as *const u8 as *const ::core::ffi::c_char,
    b"r\"\"\"\\p{N}{1,3}\"\"\",\0" as *const u8 as *const ::core::ffi::c_char,
    b"r\"\"\" ?[^\\s\\p{L}\\p{N}]+[\\r\\n]*\"\"\",\0" as *const u8
        as *const ::core::ffi::c_char,
    b"r\"\"\"\\s*[\\r\\n]+\"\"\",\0" as *const u8 as *const ::core::ffi::c_char,
    b"r\"\"\"\\s+(?!\\S)\"\"\",\0" as *const u8 as *const ::core::ffi::c_char,
    b"r\"\"\"\\s+\"\"\",\0" as *const u8 as *const ::core::ffi::c_char,
    b"TIKTOKEN_MAX_ENCODE_CHARS = 400_000\0" as *const u8 as *const ::core::ffi::c_char,
    b"MAX_NO_WHITESPACES_CHARS = 25_000\0" as *const u8 as *const ::core::ffi::c_char,
    b"current_slice_is_space = s[0].isspace() if len(s) > 0 else False\0" as *const u8
        as *const ::core::ffi::c_char,
    b"if current_slice_len > max_consecutive_slice_len:\0" as *const u8
        as *const ::core::ffi::c_char,
    b"allowed_special=\"all\",\0" as *const u8 as *const ::core::ffi::c_char,
    b"disallowed_special=(),\0" as *const u8 as *const ::core::ffi::c_char,
];
static mut WRAPPER_WHY: [*const ::core::ffi::c_char; 15] = [
    b"tokenization_kimi.py lacks: num_reserved_special_tokens = 256\0" as *const u8
        as *const ::core::ffi::c_char,
    b"tokenization_kimi.py lacks the pattern line [\\p{Han}]+\0" as *const u8
        as *const ::core::ffi::c_char,
    b"tokenization_kimi.py lacks the pattern's first letter alternative\0" as *const u8
        as *const ::core::ffi::c_char,
    b"tokenization_kimi.py lacks the pattern's second letter alternative\0" as *const u8
        as *const ::core::ffi::c_char,
    b"tokenization_kimi.py lacks the pattern line \\p{N}{1,3}\0" as *const u8
        as *const ::core::ffi::c_char,
    b"tokenization_kimi.py lacks the pattern's punctuation alternative ( ?[^\\s\\p{L}\\p{N}]+[\\r\\n]*)\0"
        as *const u8 as *const ::core::ffi::c_char,
    b"tokenization_kimi.py lacks the pattern line \\s*[\\r\\n]+\0" as *const u8
        as *const ::core::ffi::c_char,
    b"tokenization_kimi.py lacks the pattern line \\s+(?!\\S)\0" as *const u8
        as *const ::core::ffi::c_char,
    b"tokenization_kimi.py lacks the pattern line \\s+\0" as *const u8
        as *const ::core::ffi::c_char,
    b"tokenization_kimi.py lacks: TIKTOKEN_MAX_ENCODE_CHARS = 400_000\0" as *const u8
        as *const ::core::ffi::c_char,
    b"tokenization_kimi.py lacks: MAX_NO_WHITESPACES_CHARS = 25_000\0" as *const u8
        as *const ::core::ffi::c_char,
    b"tokenization_kimi.py lacks the run splitter's first line (s[0].isspace())\0"
        as *const u8 as *const ::core::ffi::c_char,
    b"tokenization_kimi.py lacks the run splitter's cut (current_slice_len > max_consecutive_slice_len)\0"
        as *const u8 as *const ::core::ffi::c_char,
    b"tokenization_kimi.py lacks: allowed_special=\"all\",\0" as *const u8
        as *const ::core::ffi::c_char,
    b"tokenization_kimi.py lacks: disallowed_special=(),\0" as *const u8
        as *const ::core::ffi::c_char,
];
pub const N_WRAPPER_LINES: uint32_t = (::core::mem::size_of::<
    [*const ::core::ffi::c_char; 15],
>() as usize)
    .wrapping_div(::core::mem::size_of::<*const ::core::ffi::c_char>() as usize)
    as uint32_t;
unsafe extern "C" fn cstr_len(mut s: *const ::core::ffi::c_char) -> uint64_t {
    let mut n: uint64_t = 0 as uint64_t;
    while *s.offset(n as isize) as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
        n = n.wrapping_add(1);
    }
    return n;
}
unsafe extern "C" fn line_is(
    mut p: *const uint8_t,
    mut n: uint64_t,
    mut s: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    while n > 0 as uint64_t
        && (*p.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == ' ' as i32
            || *p.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == '\t' as i32)
    {
        p = p.offset(1);
        n = n.wrapping_sub(1);
    }
    while n > 0 as uint64_t
        && (*p.offset(n.wrapping_sub(1 as uint64_t) as isize) as ::core::ffi::c_int
            == ' ' as i32
            || *p.offset(n.wrapping_sub(1 as uint64_t) as isize) as ::core::ffi::c_int
                == '\t' as i32
            || *p.offset(n.wrapping_sub(1 as uint64_t) as isize) as ::core::ffi::c_int
                == '\r' as i32)
    {
        n = n.wrapping_sub(1);
    }
    let mut k: uint64_t = cstr_len(s);
    return (n == k
        && memcmp(
            p as *const ::core::ffi::c_void,
            s as *const ::core::ffi::c_void,
            k as size_t,
        ) == 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
}
unsafe extern "C" fn line_blank(
    mut p: *const uint8_t,
    mut n: uint64_t,
) -> ::core::ffi::c_int {
    let mut i: uint64_t = 0 as uint64_t;
    while i < n {
        if *p.offset(i as isize) as ::core::ffi::c_int != ' ' as i32
            && *p.offset(i as isize) as ::core::ffi::c_int != '\t' as i32
            && *p.offset(i as isize) as ::core::ffi::c_int != '\r' as i32
        {
            return 0 as ::core::ffi::c_int;
        }
        i = i.wrapping_add(1);
    }
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn scan_lines(
    mut w: *const uint8_t,
    mut n: uint64_t,
    mut L: *const *const ::core::ffi::c_char,
    mut nl: uint32_t,
    mut r0: uint32_t,
    mut r1: uint32_t,
    mut run_ok: *mut uint32_t,
) -> uint32_t {
    let mut seen: uint32_t = 0 as uint32_t;
    let mut run: uint32_t = 0 as uint32_t;
    *run_ok = 0 as ::core::ffi::c_uint as uint32_t;
    let mut s: uint64_t = 0 as uint64_t;
    let mut e: uint64_t = 0 as uint64_t;
    let mut nx: uint64_t = 0 as uint64_t;
    while s < n {
        next_line(w, n, s, &raw mut e, &raw mut nx);
        if !(line_blank(w.offset(s as isize), e.wrapping_sub(s)) != 0) {
            let mut hit: uint32_t = nl;
            let mut i: uint32_t = 0 as uint32_t;
            while i < nl {
                if line_is(
                    w.offset(s as isize),
                    e.wrapping_sub(s),
                    *L.offset(i as isize),
                ) != 0
                {
                    hit = i;
                    break;
                } else {
                    i = i.wrapping_add(1);
                }
            }
            if hit < nl {
                seen = (seen as ::core::ffi::c_uint | (1 as ::core::ffi::c_uint) << hit)
                    as uint32_t;
            }
            run = if hit >= r0 && hit <= r1 && hit == r0.wrapping_add(run) {
                run.wrapping_add(1 as uint32_t)
            } else if hit == r0 {
                1 as uint32_t
            } else {
                0 as uint32_t
            };
            if run == r1.wrapping_sub(r0).wrapping_add(1 as uint32_t) {
                *run_ok = 1 as ::core::ffi::c_uint as uint32_t;
            }
        }
        s = nx;
    }
    return seen;
}
unsafe extern "C" fn check_wrapper(
    mut w: *const uint8_t,
    mut n: uint64_t,
) -> *const ::core::ffi::c_char {
    if n == 0 as uint64_t
        || n > ((1 as ::core::ffi::c_uint) << 20 as ::core::ffi::c_int) as uint64_t
        || toks_utf8_valid(w, n) == 0
    {
        return b"tokenization_kimi.py: empty, above 1 MiB or not utf-8\0" as *const u8
            as *const ::core::ffi::c_char;
    }
    let mut pattern_ok: uint32_t = 0;
    let mut seen: uint32_t = scan_lines(
        w,
        n,
        &raw const WRAPPER_LINES as *const *const ::core::ffi::c_char,
        N_WRAPPER_LINES,
        1 as uint32_t,
        8 as uint32_t,
        &raw mut pattern_ok,
    );
    let mut i: uint32_t = 0 as uint32_t;
    while i < N_WRAPPER_LINES {
        if seen & (1 as uint32_t) << i == 0 as uint32_t {
            return WRAPPER_WHY[i as usize];
        }
        i = i.wrapping_add(1);
    }
    if pattern_ok == 0 as uint32_t {
        return b"tokenization_kimi.py: the pattern's eight alternatives are not consecutive, in order\0"
            as *const u8 as *const ::core::ffi::c_char;
    }
    return ::core::ptr::null::<::core::ffi::c_char>();
}
unsafe extern "C" fn attr_str(mut v: *const jv) -> *const jv {
    if v.is_null() || (*v).type_0 as ::core::ffi::c_int == JV_NULL as ::core::ffi::c_int
    {
        return ::core::ptr::null::<jv>();
    }
    if (*v).type_0 as ::core::ffi::c_int == JV_OBJ as ::core::ffi::c_int {
        static mut FLAGS: [*const ::core::ffi::c_char; 4] = [
            b"lstrip\0" as *const u8 as *const ::core::ffi::c_char,
            b"rstrip\0" as *const u8 as *const ::core::ffi::c_char,
            b"single_word\0" as *const u8 as *const ::core::ffi::c_char,
            b"normalized\0" as *const u8 as *const ::core::ffi::c_char,
        ];
        let mut k: uint32_t = 0 as uint32_t;
        while k < 4 as uint32_t {
            let mut f: *const jv = toks_jv_get(v, FLAGS[k as usize]);
            if !f.is_null()
                && !((*f).type_0 as ::core::ffi::c_int == JV_BOOL as ::core::ffi::c_int
                    && (*f).num == 0 as int64_t)
            {
                return ::core::ptr::null::<jv>();
            }
            k = k.wrapping_add(1);
        }
        v = toks_jv_get(v, b"content\0" as *const u8 as *const ::core::ffi::c_char);
    }
    return if !v.is_null()
        && (*v).type_0 as ::core::ffi::c_int == JV_STR as ::core::ffi::c_int
    {
        v
    } else {
        ::core::ptr::null::<jv>()
    };
}
unsafe extern "C" fn special_of(
    mut sp: *const toks_cfg_added,
    mut v: *const jv,
) -> int32_t {
    if v.is_null() || (*v).type_0 as ::core::ffi::c_int != JV_STR as ::core::ffi::c_int {
        return -(1 as int32_t);
    }
    let mut i: uint32_t = 0 as uint32_t;
    while i < TOKS_TIKTOKEN_RESERVED as uint32_t {
        if (*sp.offset(i as isize)).len == (*v).s_len
            && memcmp(
                (*sp.offset(i as isize)).content as *const ::core::ffi::c_void,
                (*v).s as *const ::core::ffi::c_void,
                (*v).s_len as size_t,
            ) == 0 as ::core::ffi::c_int
        {
            return i as int32_t;
        }
        i = i.wrapping_add(1);
    }
    return -(1 as int32_t);
}
unsafe extern "C" fn names_disjoint(
    mut sp: *const toks_cfg_added,
) -> ::core::ffi::c_int {
    let mut a: uint32_t = 0 as uint32_t;
    while a < TOKS_TIKTOKEN_RESERVED as uint32_t {
        let mut p: uint32_t = 0 as uint32_t;
        while p < (*sp.offset(a as isize)).len {
            let mut b: uint32_t = 0 as uint32_t;
            while b < TOKS_TIKTOKEN_RESERVED as uint32_t {
                if !(b == a
                    || *(*sp.offset(b as isize))
                        .content
                        .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        != *(*sp.offset(a as isize)).content.offset(p as isize)
                            as ::core::ffi::c_int)
                {
                    let mut k: uint32_t = if (*sp.offset(a as isize)).len.wrapping_sub(p)
                        < (*sp.offset(b as isize)).len
                    {
                        (*sp.offset(a as isize)).len.wrapping_sub(p)
                    } else {
                        (*sp.offset(b as isize)).len
                    };
                    if memcmp(
                        (*sp.offset(a as isize)).content.offset(p as isize)
                            as *const ::core::ffi::c_void,
                        (*sp.offset(b as isize)).content as *const ::core::ffi::c_void,
                        k as size_t,
                    ) == 0 as ::core::ffi::c_int
                    {
                        return 0 as ::core::ffi::c_int;
                    }
                }
                b = b.wrapping_add(1);
            }
            p = p.wrapping_add(1);
        }
        a = a.wrapping_add(1);
    }
    return 1 as ::core::ffi::c_int;
}
static mut CONFIG_KEYS: [*const ::core::ffi::c_char; 19] = [
    b"added_tokens_decoder\0" as *const u8 as *const ::core::ffi::c_char,
    b"additional_special_tokens\0" as *const u8 as *const ::core::ffi::c_char,
    b"bos_token\0" as *const u8 as *const ::core::ffi::c_char,
    b"eos_token\0" as *const u8 as *const ::core::ffi::c_char,
    b"unk_token\0" as *const u8 as *const ::core::ffi::c_char,
    b"pad_token\0" as *const u8 as *const ::core::ffi::c_char,
    b"clean_up_tokenization_spaces\0" as *const u8 as *const ::core::ffi::c_char,
    b"extra_special_tokens\0" as *const u8 as *const ::core::ffi::c_char,
    b"model_max_length\0" as *const u8 as *const ::core::ffi::c_char,
    b"tokenizer_class\0" as *const u8 as *const ::core::ffi::c_char,
    b"auto_map\0" as *const u8 as *const ::core::ffi::c_char,
    b"chat_template\0" as *const u8 as *const ::core::ffi::c_char,
    b"backend\0" as *const u8 as *const ::core::ffi::c_char,
    b"is_local\0" as *const u8 as *const ::core::ffi::c_char,
    b"local_files_only\0" as *const u8 as *const ::core::ffi::c_char,
    b"tool_parser_type\0" as *const u8 as *const ::core::ffi::c_char,
    b"processor_class\0" as *const u8 as *const ::core::ffi::c_char,
    b"padding_side\0" as *const u8 as *const ::core::ffi::c_char,
    b"model_specific_special_tokens\0" as *const u8 as *const ::core::ffi::c_char,
];
#[no_mangle]
pub unsafe extern "C" fn toks_tiktoken_kimi(
    mut config: *const uint8_t,
    mut config_len: uint64_t,
    mut wrapper: *const uint8_t,
    mut wrapper_len: uint64_t,
    mut ar: *mut toks_arena,
    mut cfg: *mut toks_config,
    mut info: *mut toks_tiktoken_info,
    mut err: *mut toks_err,
) -> int64_t {
    memset(
        info as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<toks_tiktoken_info>() as size_t,
    );
    let mut why: *const ::core::ffi::c_char = check_wrapper(wrapper, wrapper_len);
    if !why.is_null() {
        return toks_fail(err, TOKS_E_UNSUPPORTED as int64_t, why);
    }
    let mut n_ranks: uint32_t = 0 as uint32_t;
    let mut id: uint32_t = 0 as uint32_t;
    while id < (*cfg).n_vocab {
        n_ranks = (n_ranks as ::core::ffi::c_uint)
            .wrapping_add(
                if *(*cfg).vocab_len.offset(id as isize) != 0 as uint32_t {
                    1 as ::core::ffi::c_uint
                } else {
                    0 as ::core::ffi::c_uint
                },
            ) as uint32_t as uint32_t;
        id = id.wrapping_add(1);
    }
    if n_ranks != (*cfg).n_vocab {
        return toks_fail(
            err,
            TOKS_E_UNSUPPORTED as int64_t,
            b"tiktoken.model ranks are not 0..n-1 (the kimi wrapper numbers its specials from the token count)\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
    }
    let mut first: uint32_t = n_ranks;
    if (first as uint64_t).wrapping_add(TOKS_TIKTOKEN_RESERVED as uint64_t)
        > TOKS_MAX_IDS as uint64_t
    {
        return toks_fail(
            err,
            TOKS_E_LIMIT as int64_t,
            b"kimi specials: ids >= TOKS_MAX_IDS\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    let mut root: *mut jv = ::core::ptr::null_mut::<jv>();
    if toks_json_parse(config, config_len, ar, &raw mut root) != 0 as int64_t
        || (*root).type_0 as ::core::ffi::c_int != JV_OBJ as ::core::ffi::c_int
    {
        return toks_fail(
            err,
            TOKS_E_FORMAT as int64_t,
            b"tokenizer_config.json: not a json object\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    let mut m: *const jv = (*root).child;
    while !m.is_null() {
        let mut known: uint32_t = 0 as uint32_t;
        let mut k: uint32_t = 0 as uint32_t;
        while k
            < (::core::mem::size_of::<[*const ::core::ffi::c_char; 19]>() as usize)
                .wrapping_div(
                    ::core::mem::size_of::<*const ::core::ffi::c_char>() as usize,
                ) as uint32_t
        {
            let mut kl: uint64_t = cstr_len(CONFIG_KEYS[k as usize]);
            if (*m).s_len as uint64_t == kl
                && memcmp(
                    (*m).s as *const ::core::ffi::c_void,
                    CONFIG_KEYS[k as usize] as *const ::core::ffi::c_void,
                    kl as size_t,
                ) == 0 as ::core::ffi::c_int
            {
                known = 1 as ::core::ffi::c_uint as uint32_t;
            }
            k = k.wrapping_add(1);
        }
        if known == 0 as uint32_t {
            return toks_fail(
                err,
                TOKS_E_UNSUPPORTED as int64_t,
                b"tokenizer_config.json: a key the kimi reader does not know\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
        m = (*m).next;
    }
    if toks_jstr(
        toks_jv_get(
            root,
            b"tokenizer_class\0" as *const u8 as *const ::core::ffi::c_char,
        ),
        b"TikTokenTokenizer\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        return toks_fail(
            err,
            TOKS_E_UNSUPPORTED as int64_t,
            b"tokenizer_config.json: tokenizer_class is not TikTokenTokenizer\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
    }
    let mut am: *const jv = toks_jv_get(
        root,
        b"auto_map\0" as *const u8 as *const ::core::ffi::c_char,
    );
    let mut at: *const jv = if !am.is_null()
        && (*am).type_0 as ::core::ffi::c_int == JV_OBJ as ::core::ffi::c_int
    {
        toks_jv_get(am, b"AutoTokenizer\0" as *const u8 as *const ::core::ffi::c_char)
    } else {
        ::core::ptr::null_mut::<jv>()
    };
    if at.is_null() || (*at).type_0 as ::core::ffi::c_int != JV_ARR as ::core::ffi::c_int
        || toks_jstr(
            (*at).child,
            b"tokenization_kimi.TikTokenTokenizer\0" as *const u8
                as *const ::core::ffi::c_char,
        ) == 0
    {
        return toks_fail(
            err,
            TOKS_E_UNSUPPORTED as int64_t,
            b"tokenizer_config.json: auto_map AutoTokenizer is not tokenization_kimi.TikTokenTokenizer\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
    }
    let mut ex: *const jv = toks_jv_get(
        root,
        b"extra_special_tokens\0" as *const u8 as *const ::core::ffi::c_char,
    );
    let mut ms: *const jv = toks_jv_get(
        root,
        b"model_specific_special_tokens\0" as *const u8 as *const ::core::ffi::c_char,
    );
    let mut be: *const jv = toks_jv_get(
        root,
        b"backend\0" as *const u8 as *const ::core::ffi::c_char,
    );
    if !ex.is_null()
        && (*ex).type_0 as ::core::ffi::c_int == JV_OBJ as ::core::ffi::c_int
        && !(*ex).child.is_null()
        || !ms.is_null()
            && (*ms).type_0 as ::core::ffi::c_int != JV_NULL as ::core::ffi::c_int
            && !((*ms).type_0 as ::core::ffi::c_int == JV_OBJ as ::core::ffi::c_int
                && (*ms).child.is_null())
        || !be.is_null()
            && toks_jstr(be, b"custom\0" as *const u8 as *const ::core::ffi::c_char) == 0
    {
        return toks_fail(
            err,
            TOKS_E_UNSUPPORTED as int64_t,
            b"tokenizer_config.json: named extra special tokens or a backend other than custom\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
    }
    let mut sp: *mut toks_cfg_added = toks_ar_alloc(
        ar,
        (TOKS_TIKTOKEN_RESERVED as uint64_t)
            .wrapping_mul(::core::mem::size_of::<toks_cfg_added>() as uint64_t),
        8 as uint64_t,
    ) as *mut toks_cfg_added;
    let mut names: *mut uint8_t = toks_ar_alloc(
        ar,
        TOKS_TIKTOKEN_RESERVED.wrapping_mul(32 as ::core::ffi::c_uint) as uint64_t,
        8 as uint64_t,
    ) as *mut uint8_t;
    let mut in_dec: [uint8_t; 256] = [0; 256];
    if sp.is_null() || names.is_null() {
        return toks_fail(
            err,
            TOKS_E_NOMEM as int64_t,
            b"kimi specials\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    memset(
        sp as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        (TOKS_TIKTOKEN_RESERVED as size_t)
            .wrapping_mul(::core::mem::size_of::<toks_cfg_added>() as size_t),
    );
    memset(
        &raw mut in_dec as *mut uint8_t as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<[uint8_t; 256]>() as size_t,
    );
    let mut dec: *const jv = toks_jv_get(
        root,
        b"added_tokens_decoder\0" as *const u8 as *const ::core::ffi::c_char,
    );
    if !dec.is_null()
        && (*dec).type_0 as ::core::ffi::c_int != JV_OBJ as ::core::ffi::c_int
    {
        return toks_fail(
            err,
            TOKS_E_FORMAT as int64_t,
            b"tokenizer_config.json: added_tokens_decoder\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    let mut m_0: *const jv = if !dec.is_null() {
        (*dec).child
    } else {
        ::core::ptr::null_mut::<jv>()
    };
    while !m_0.is_null() {
        let mut id_0: uint64_t = 0 as uint64_t;
        if (*m_0).s_len == 0 as uint32_t || (*m_0).s_len > 7 as uint32_t
            || *(*m_0).s.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == '0' as i32 && (*m_0).s_len > 1 as uint32_t
        {
            return toks_fail(
                err,
                TOKS_E_FORMAT as int64_t,
                b"tokenizer_config.json: an added_tokens_decoder key is not an id\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
        let mut k_0: uint32_t = 0 as uint32_t;
        while k_0 < (*m_0).s_len {
            if (*(*m_0).s.offset(k_0 as isize) as ::core::ffi::c_int) < '0' as i32
                || *(*m_0).s.offset(k_0 as isize) as ::core::ffi::c_int > '9' as i32
            {
                return toks_fail(
                    err,
                    TOKS_E_FORMAT as int64_t,
                    b"tokenizer_config.json: an added_tokens_decoder key is not an id\0"
                        as *const u8 as *const ::core::ffi::c_char,
                );
            }
            id_0 = id_0
                .wrapping_mul(10 as uint64_t)
                .wrapping_add(
                    (*(*m_0).s.offset(k_0 as isize) as ::core::ffi::c_int - '0' as i32)
                        as uint64_t,
                );
            k_0 = k_0.wrapping_add(1);
        }
        if id_0 < first as uint64_t
            || id_0
                >= (first as uint64_t).wrapping_add(TOKS_TIKTOKEN_RESERVED as uint64_t)
        {
            return toks_fail(
                err,
                TOKS_E_UNSUPPORTED as int64_t,
                b"tokenizer_config.json: an added token outside the wrapper's 256 special ids\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
        let mut i: uint32_t = id_0.wrapping_sub(first as uint64_t) as uint32_t;
        if in_dec[i as usize] as ::core::ffi::c_uint != 0 as ::core::ffi::c_uint {
            return toks_fail(
                err,
                TOKS_E_UNSUPPORTED as int64_t,
                b"tokenizer_config.json: added_tokens_decoder repeats an id\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
        let mut v: *const jv = (*m_0).child;
        let mut c: *const jv = if !v.is_null()
            && (*v).type_0 as ::core::ffi::c_int == JV_OBJ as ::core::ffi::c_int
        {
            toks_jv_get(v, b"content\0" as *const u8 as *const ::core::ffi::c_char)
        } else {
            ::core::ptr::null_mut::<jv>()
        };
        let mut spc: *const jv = if !v.is_null()
            && (*v).type_0 as ::core::ffi::c_int == JV_OBJ as ::core::ffi::c_int
        {
            toks_jv_get(v, b"special\0" as *const u8 as *const ::core::ffi::c_char)
        } else {
            ::core::ptr::null_mut::<jv>()
        };
        if c.is_null()
            || (*c).type_0 as ::core::ffi::c_int != JV_STR as ::core::ffi::c_int
            || (*c).s_len == 0 as uint32_t || spc.is_null()
            || (*spc).type_0 as ::core::ffi::c_int != JV_BOOL as ::core::ffi::c_int
        {
            return toks_fail(
                err,
                TOKS_E_FORMAT as int64_t,
                b"tokenizer_config.json: an added token without content / special\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
        static mut FLAGS: [*const ::core::ffi::c_char; 4] = [
            b"lstrip\0" as *const u8 as *const ::core::ffi::c_char,
            b"rstrip\0" as *const u8 as *const ::core::ffi::c_char,
            b"single_word\0" as *const u8 as *const ::core::ffi::c_char,
            b"normalized\0" as *const u8 as *const ::core::ffi::c_char,
        ];
        let mut k_1: uint32_t = 0 as uint32_t;
        while k_1 < 4 as uint32_t {
            let mut f: *const jv = toks_jv_get(v, FLAGS[k_1 as usize]);
            if f.is_null()
                || (*f).type_0 as ::core::ffi::c_int != JV_BOOL as ::core::ffi::c_int
                || (*f).num != 0 as int64_t
            {
                return toks_fail(
                    err,
                    TOKS_E_UNSUPPORTED as int64_t,
                    b"tokenizer_config.json: an added token whose lstrip / rstrip / single_word / normalized is not false\0"
                        as *const u8 as *const ::core::ffi::c_char,
                );
            }
            k_1 = k_1.wrapping_add(1);
        }
        if (*c).s_len > TOKS_MAX_ADDED_BYTES as uint32_t {
            return toks_fail(
                err,
                TOKS_E_LIMIT as int64_t,
                b"tokenizer_config.json: an added token above TOKS_MAX_ADDED_BYTES\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
        in_dec[i as usize] = 1 as uint8_t;
        let ref mut fresh6 = (*sp.offset(i as isize)).content;
        *fresh6 = (*c).s;
        (*sp.offset(i as isize)).len = (*c).s_len;
        m_0 = (*m_0).next;
    }
    let mut i_0: uint32_t = 0 as uint32_t;
    while i_0 < TOKS_TIKTOKEN_RESERVED as uint32_t {
        (*sp.offset(i_0 as isize)).id = first.wrapping_add(i_0);
        if in_dec[i_0 as usize] as ::core::ffi::c_uint != 0 as ::core::ffi::c_uint {
            let mut k_2: uint32_t = 0 as uint32_t;
            while k_2 < (*sp.offset(i_0 as isize)).len {
                if (*(*sp.offset(i_0 as isize)).content.offset(k_2 as isize)
                    as ::core::ffi::c_uint) < 0x21 as ::core::ffi::c_uint
                    || *(*sp.offset(i_0 as isize)).content.offset(k_2 as isize)
                        as ::core::ffi::c_uint > 0x7e as ::core::ffi::c_uint
                {
                    return toks_fail(
                        err,
                        TOKS_E_UNSUPPORTED as int64_t,
                        b"tokenizer_config.json: a special name outside printable ascii\0"
                            as *const u8 as *const ::core::ffi::c_char,
                    );
                }
                k_2 = k_2.wrapping_add(1);
            }
        } else {
            static mut pre: [::core::ffi::c_char; 18] = unsafe {
                ::core::mem::transmute::<
                    [u8; 18],
                    [::core::ffi::c_char; 18],
                >(*b"<|reserved_token_\0")
            };
            let mut o: *mut uint8_t = names
                .offset((32 as uint32_t).wrapping_mul(i_0) as isize);
            let mut n: uint32_t = 0 as uint32_t;
            while pre[n as usize] as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                *o.offset(n as isize) = pre[n as usize] as uint8_t;
                n = n.wrapping_add(1);
            }
            let mut dg: [uint8_t; 8] = [0; 8];
            let mut nd: uint32_t = 0 as uint32_t;
            let mut v_0: uint32_t = first.wrapping_add(i_0);
            while nd == 0 as uint32_t || v_0 != 0 as uint32_t {
                let fresh7 = nd;
                nd = nd.wrapping_add(1);
                dg[fresh7 as usize] = ('0' as i32 as uint32_t)
                    .wrapping_add(v_0.wrapping_rem(10 as uint32_t)) as uint8_t;
                v_0 = (v_0 as ::core::ffi::c_uint)
                    .wrapping_div(10 as ::core::ffi::c_uint) as uint32_t as uint32_t;
            }
            while nd > 0 as uint32_t {
                nd = nd.wrapping_sub(1);
                let fresh8 = n;
                n = n.wrapping_add(1);
                *o.offset(fresh8 as isize) = dg[nd as usize];
            }
            let fresh9 = n;
            n = n.wrapping_add(1);
            *o.offset(fresh9 as isize) = '|' as i32 as uint8_t;
            let fresh10 = n;
            n = n.wrapping_add(1);
            *o.offset(fresh10 as isize) = '>' as i32 as uint8_t;
            let ref mut fresh11 = (*sp.offset(i_0 as isize)).content;
            *fresh11 = o;
            (*sp.offset(i_0 as isize)).len = n;
        }
        i_0 = i_0.wrapping_add(1);
    }
    if names_disjoint(sp) == 0 {
        return toks_fail(
            err,
            TOKS_E_UNSUPPORTED as int64_t,
            b"kimi specials: two names overlap or repeat\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    let mut named: [uint8_t; 256] = [0; 256];
    memset(
        &raw mut named as *mut uint8_t as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<[uint8_t; 256]>() as size_t,
    );
    static mut ATTRS: [*const ::core::ffi::c_char; 4] = [
        b"bos_token\0" as *const u8 as *const ::core::ffi::c_char,
        b"eos_token\0" as *const u8 as *const ::core::ffi::c_char,
        b"unk_token\0" as *const u8 as *const ::core::ffi::c_char,
        b"pad_token\0" as *const u8 as *const ::core::ffi::c_char,
    ];
    let mut k_3: uint32_t = 0 as uint32_t;
    while k_3 < 4 as uint32_t {
        let mut i_1: int32_t = special_of(
            sp,
            attr_str(toks_jv_get(root, ATTRS[k_3 as usize])),
        );
        if i_1 < 0 as int32_t {
            return toks_fail(
                err,
                TOKS_E_FORMAT as int64_t,
                b"tokenizer_config.json: bos / eos / unk / pad_token must each name a special token\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
        named[i_1 as usize] = 1 as uint8_t;
        k_3 = k_3.wrapping_add(1);
    }
    let mut k_4: uint32_t = 0 as uint32_t;
    while k_4 < 2 as uint32_t {
        let mut add: *const jv = if k_4 == 0 as uint32_t {
            toks_jv_get(
                root,
                b"additional_special_tokens\0" as *const u8 as *const ::core::ffi::c_char,
            ) as *const jv
        } else {
            ex
        };
        if !add.is_null()
            && (*add).type_0 as ::core::ffi::c_int != JV_NULL as ::core::ffi::c_int
            && (*add).type_0 as ::core::ffi::c_int != JV_ARR as ::core::ffi::c_int
            && k_4 == 0 as uint32_t
        {
            return toks_fail(
                err,
                TOKS_E_FORMAT as int64_t,
                b"tokenizer_config.json: additional_special_tokens\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        let mut v_1: *const jv = if !add.is_null()
            && (*add).type_0 as ::core::ffi::c_int == JV_ARR as ::core::ffi::c_int
        {
            (*add).child
        } else {
            ::core::ptr::null_mut::<jv>()
        };
        while !v_1.is_null() {
            let mut i_2: int32_t = special_of(sp, attr_str(v_1));
            if i_2 < 0 as int32_t {
                return toks_fail(
                    err,
                    TOKS_E_UNSUPPORTED as int64_t,
                    b"tokenizer_config.json: an additional special token that is not one of the 256 specials\0"
                        as *const u8 as *const ::core::ffi::c_char,
                );
            }
            named[i_2 as usize] = 1 as uint8_t;
            v_1 = (*v_1).next;
        }
        k_4 = k_4.wrapping_add(1);
    }
    let mut i_3: uint32_t = 0 as uint32_t;
    while i_3 < TOKS_TIKTOKEN_RESERVED as uint32_t {
        (*sp.offset(i_3 as isize)).special = named[i_3 as usize];
        (*sp.offset(i_3 as isize)).normalized = 0 as uint8_t;
        (*sp.offset(i_3 as isize)).attr = (if named[i_3 as usize] as ::core::ffi::c_uint
            != 0 as ::core::ffi::c_uint
        {
            TOKS_ID_SPECIAL
        } else {
            0 as ::core::ffi::c_uint
        }) as uint8_t;
        (*sp.offset(i_3 as isize)).last = i_3;
        if in_dec[i_3 as usize] as ::core::ffi::c_uint != 0 as ::core::ffi::c_uint
            || named[i_3 as usize] as ::core::ffi::c_uint != 0 as ::core::ffi::c_uint
        {
            (*info).trie[(i_3 >> 3 as ::core::ffi::c_int) as usize] = ((*info)
                .trie[(i_3 >> 3 as ::core::ffi::c_int) as usize] as ::core::ffi::c_uint
                | (1 as ::core::ffi::c_uint) << (i_3 & 7 as uint32_t)) as uint8_t;
            (*info).n_trie = (*info).n_trie.wrapping_add(1);
        }
        (*info).n_named = (*info).n_named.wrapping_add(named[i_3 as usize] as uint32_t);
        i_3 = i_3.wrapping_add(1);
    }
    (*cfg).added = sp;
    (*cfg).n_added = TOKS_TIKTOKEN_RESERVED as uint32_t;
    (*cfg).n_ids = first.wrapping_add(TOKS_TIKTOKEN_RESERVED as uint32_t);
    (*info).n_ranks = n_ranks;
    (*info).first_special = first;
    (*info).chunk_chars = 400000 as ::core::ffi::c_uint as uint32_t;
    (*info).run_chars = 25000 as ::core::ffi::c_uint as uint32_t;
    return 0 as int64_t;
}
static mut QWEN_LINES: [*const ::core::ffi::c_char; 17] = [
    b"class QWenTokenizer(PreTrainedTokenizer):\0" as *const u8
        as *const ::core::ffi::c_char,
    b"ENDOFTEXT,\0" as *const u8 as *const ::core::ffi::c_char,
    b"IMSTART,\0" as *const u8 as *const ::core::ffi::c_char,
    b"IMEND,\0" as *const u8 as *const ::core::ffi::c_char,
    b"PAT_STR = r\"\"\"(?i:'s|'t|'re|'ve|'m|'ll|'d)|[^\\r\\n\\p{L}\\p{N}]?\\p{L}+|\\p{N}| ?[^\\s\\p{L}\\p{N}]+[\\r\\n]*|\\s*[\\r\\n]+|\\s+(?!\\S)|\\s+\"\"\"\0"
        as *const u8 as *const ::core::ffi::c_char,
    b"ENDOFTEXT = \"<|endoftext|>\"\0" as *const u8 as *const ::core::ffi::c_char,
    b"IMSTART = \"<|im_start|>\"\0" as *const u8 as *const ::core::ffi::c_char,
    b"IMEND = \"<|im_end|>\"\0" as *const u8 as *const ::core::ffi::c_char,
    b"EXTRAS = tuple((f\"<|extra_{i}|>\" for i in range(205)))\0" as *const u8
        as *const ::core::ffi::c_char,
    b"SPECIAL_START_ID = 151643\0" as *const u8 as *const ::core::ffi::c_char,
    b"+ EXTRAS\0" as *const u8 as *const ::core::ffi::c_char,
    b"start=SPECIAL_START_ID,\0" as *const u8 as *const ::core::ffi::c_char,
    b"text = unicodedata.normalize(\"NFC\", text)\0" as *const u8
        as *const ::core::ffi::c_char,
    b"allowed_special: Union[Set, str] = \"all\",\0" as *const u8
        as *const ::core::ffi::c_char,
    b"disallowed_special: Union[Collection, str] = (),\0" as *const u8
        as *const ::core::ffi::c_char,
    b"text, allowed_special=allowed_special, disallowed_special=disallowed_special\0"
        as *const u8 as *const ::core::ffi::c_char,
    b"token_ids = [i for i in token_ids if i < self.eod_id]\0" as *const u8
        as *const ::core::ffi::c_char,
];
pub const N_QWEN_LINES: uint32_t = (::core::mem::size_of::<
    [*const ::core::ffi::c_char; 17],
>() as usize)
    .wrapping_div(::core::mem::size_of::<*const ::core::ffi::c_char>() as usize)
    as uint32_t;
pub const QWEN_FIRST: ::core::ffi::c_uint = 151643 as ::core::ffi::c_uint;
pub const QWEN_SPECIALS: ::core::ffi::c_uint = 208 as ::core::ffi::c_uint;
unsafe extern "C" fn qwen(
    mut config: *const uint8_t,
    mut config_len: uint64_t,
    mut wrapper: *const uint8_t,
    mut wrapper_len: uint64_t,
    mut ar: *mut toks_arena,
    mut cfg: *mut toks_config,
    mut err: *mut toks_err,
) -> int64_t {
    let mut run: uint32_t = 0;
    let mut seen: uint32_t = scan_lines(
        wrapper,
        wrapper_len,
        &raw const QWEN_LINES as *const *const ::core::ffi::c_char,
        N_QWEN_LINES,
        1 as uint32_t,
        3 as uint32_t,
        &raw mut run,
    );
    if seen.wrapping_add(1 as uint32_t) != (1 as uint32_t) << N_QWEN_LINES
        || run == 0 as uint32_t
    {
        return toks_fail(
            err,
            TOKS_E_UNSUPPORTED as int64_t,
            b"tokenization_qwen.py: not the Qwen-1 wrapper toks reads (a line of docs/models/qwen1.md differs)\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
    }
    let mut n_ranks: uint32_t = 0 as uint32_t;
    let mut id: uint32_t = 0 as uint32_t;
    while id < (*cfg).n_vocab {
        n_ranks = (n_ranks as ::core::ffi::c_uint)
            .wrapping_add(
                if *(*cfg).vocab_len.offset(id as isize) != 0 as uint32_t {
                    1 as ::core::ffi::c_uint
                } else {
                    0 as ::core::ffi::c_uint
                },
            ) as uint32_t as uint32_t;
        id = id.wrapping_add(1);
    }
    if n_ranks != QWEN_FIRST as uint32_t || (*cfg).n_vocab != QWEN_FIRST as uint32_t {
        return toks_fail(
            err,
            TOKS_E_UNSUPPORTED as int64_t,
            b"qwen.tiktoken: not ranks 0 .. 151642 (SPECIAL_START_ID = 151643)\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
    }
    let mut root: *mut jv = ::core::ptr::null_mut::<jv>();
    if toks_json_parse(config, config_len, ar, &raw mut root) != 0 as int64_t
        || (*root).type_0 as ::core::ffi::c_int != JV_OBJ as ::core::ffi::c_int
    {
        return toks_fail(
            err,
            TOKS_E_FORMAT as int64_t,
            b"tokenizer_config.json: not a json object\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    let mut keys: uint32_t = 0 as uint32_t;
    let mut m: *const jv = (*root).child;
    while !m.is_null() {
        keys = keys.wrapping_add(1);
        m = (*m).next;
    }
    let mut am: *const jv = toks_jv_get(
        root,
        b"auto_map\0" as *const u8 as *const ::core::ffi::c_char,
    );
    let mut at: *const jv = if !am.is_null()
        && (*am).type_0 as ::core::ffi::c_int == JV_OBJ as ::core::ffi::c_int
    {
        toks_jv_get(am, b"AutoTokenizer\0" as *const u8 as *const ::core::ffi::c_char)
    } else {
        ::core::ptr::null_mut::<jv>()
    };
    if keys
        != (2 as uint32_t)
            .wrapping_add(
                (toks_jv_get(
                    root,
                    b"model_max_length\0" as *const u8 as *const ::core::ffi::c_char,
                ) != NULL as *mut jv) as ::core::ffi::c_int as uint32_t,
            )
        || toks_jstr(
            toks_jv_get(
                root,
                b"tokenizer_class\0" as *const u8 as *const ::core::ffi::c_char,
            ),
            b"QWenTokenizer\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 || at.is_null()
        || (*at).type_0 as ::core::ffi::c_int != JV_ARR as ::core::ffi::c_int
        || toks_jstr(
            (*at).child,
            b"tokenization_qwen.QWenTokenizer\0" as *const u8
                as *const ::core::ffi::c_char,
        ) == 0
    {
        return toks_fail(
            err,
            TOKS_E_UNSUPPORTED as int64_t,
            b"tokenizer_config.json: not QWenTokenizer's (tokenizer_class, auto_map, model_max_length only)\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
    }
    let mut sp: *mut toks_cfg_added = toks_ar_alloc(
        ar,
        (QWEN_SPECIALS as uint64_t)
            .wrapping_mul(::core::mem::size_of::<toks_cfg_added>() as uint64_t),
        8 as uint64_t,
    ) as *mut toks_cfg_added;
    let mut names: *mut uint8_t = toks_ar_alloc(
        ar,
        QWEN_SPECIALS.wrapping_mul(16 as ::core::ffi::c_uint) as uint64_t,
        8 as uint64_t,
    ) as *mut uint8_t;
    if sp.is_null() || names.is_null() {
        return toks_fail(
            err,
            TOKS_E_NOMEM as int64_t,
            b"qwen specials\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    memset(
        sp as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        (QWEN_SPECIALS as size_t)
            .wrapping_mul(::core::mem::size_of::<toks_cfg_added>() as size_t),
    );
    static mut FIXED: [*const ::core::ffi::c_char; 3] = [
        b"<|endoftext|>\0" as *const u8 as *const ::core::ffi::c_char,
        b"<|im_start|>\0" as *const u8 as *const ::core::ffi::c_char,
        b"<|im_end|>\0" as *const u8 as *const ::core::ffi::c_char,
    ];
    let mut i: uint32_t = 0 as uint32_t;
    while i < QWEN_SPECIALS as uint32_t {
        let mut o: *mut uint8_t = names
            .offset((16 as uint32_t).wrapping_mul(i) as isize);
        let mut n: uint32_t = 0 as uint32_t;
        let mut v: uint32_t = i.wrapping_sub(3 as uint32_t);
        let mut s: *const ::core::ffi::c_char = if i < 3 as uint32_t {
            FIXED[i as usize]
        } else {
            b"<|extra_\0" as *const u8 as *const ::core::ffi::c_char
        };
        while *s.offset(n as isize) as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
            *o.offset(n as isize) = *s.offset(n as isize) as uint8_t;
            n = n.wrapping_add(1);
        }
        if i >= 3 as uint32_t {
            if v >= 100 as uint32_t {
                let fresh12 = n;
                n = n.wrapping_add(1);
                *o.offset(fresh12 as isize) = ('0' as i32 as uint32_t)
                    .wrapping_add(v.wrapping_div(100 as uint32_t)) as uint8_t;
            }
            if v >= 10 as uint32_t {
                let fresh13 = n;
                n = n.wrapping_add(1);
                *o.offset(fresh13 as isize) = ('0' as i32 as uint32_t)
                    .wrapping_add(
                        v.wrapping_div(10 as uint32_t).wrapping_rem(10 as uint32_t),
                    ) as uint8_t;
            }
            let fresh14 = n;
            n = n.wrapping_add(1);
            *o.offset(fresh14 as isize) = ('0' as i32 as uint32_t)
                .wrapping_add(v.wrapping_rem(10 as uint32_t)) as uint8_t;
            let fresh15 = n;
            n = n.wrapping_add(1);
            *o.offset(fresh15 as isize) = '|' as i32 as uint8_t;
            let fresh16 = n;
            n = n.wrapping_add(1);
            *o.offset(fresh16 as isize) = '>' as i32 as uint8_t;
        }
        *sp.offset(i as isize) = toks_cfg_added {
            content: o,
            len: n,
            id: (QWEN_FIRST as uint32_t).wrapping_add(i),
            special: 1 as uint8_t,
            normalized: 1 as uint8_t,
            lstrip: 0,
            rstrip: 0,
            single_word: 0,
            pfx: 0,
            attr: TOKS_ID_SPECIAL as uint8_t,
            rsv: 0,
            form: ::core::ptr::null::<uint8_t>(),
            form_len: 0,
            last: i,
        };
        i = i.wrapping_add(1);
    }
    (*cfg).added = sp;
    (*cfg).n_added = QWEN_SPECIALS as uint32_t;
    (*cfg).n_ids = QWEN_FIRST.wrapping_add(QWEN_SPECIALS) as uint32_t;
    (*cfg).nfc = TOKS_NS_NFC as uint8_t;
    (*cfg).pattern = (&raw const TOKS_PATTERNS as *const toks_pattern)
        .offset(2 as ::core::ffi::c_int as isize) as *const toks_pattern;
    return 0 as int64_t;
}
#[no_mangle]
pub unsafe extern "C" fn toks_tiktoken_parse(
    mut ranks: *const uint8_t,
    mut ranks_len: uint64_t,
    mut config: *const uint8_t,
    mut config_len: uint64_t,
    mut wrapper: *const uint8_t,
    mut wrapper_len: uint64_t,
    mut ar: *mut toks_arena,
    mut cfg: *mut toks_config,
    mut info: *mut toks_tiktoken_info,
    mut err: *mut toks_err,
) -> int64_t {
    let mut r: int64_t = toks_tiktoken_ranks(ranks, ranks_len, ar, cfg, err);
    if r != 0 as int64_t {
        return r;
    }
    let mut run: uint32_t = 0;
    memset(
        info as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<toks_tiktoken_info>() as size_t,
    );
    if scan_lines(
        wrapper,
        wrapper_len,
        &raw const QWEN_LINES as *const *const ::core::ffi::c_char,
        1 as uint32_t,
        1 as uint32_t,
        0 as uint32_t,
        &raw mut run,
    ) & 1 as uint32_t != 0 as uint32_t
    {
        return qwen(config, config_len, wrapper, wrapper_len, ar, cfg, err);
    }
    r = toks_tiktoken_kimi(config, config_len, wrapper, wrapper_len, ar, cfg, info, err);
    if r != 0 as int64_t {
        return r;
    }
    let mut ph: *mut toks_cfg_added = toks_ar_alloc(
        ar,
        ((*info).n_trie.wrapping_add(TOKS_TIKTOKEN_RESERVED as uint32_t) as uint64_t)
            .wrapping_mul(::core::mem::size_of::<toks_cfg_added>() as uint64_t),
        8 as uint64_t,
    ) as *mut toks_cfg_added;
    if ph.is_null() {
        return toks_fail(
            err,
            TOKS_E_NOMEM as int64_t,
            b"tiktoken: parse arena\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    let mut k: uint32_t = 0 as uint32_t;
    let mut i: uint32_t = 0 as uint32_t;
    while i < TOKS_TIKTOKEN_RESERVED as uint32_t {
        if ((*info).trie[(i >> 3 as ::core::ffi::c_int) as usize] as ::core::ffi::c_int
            >> (i & 7 as uint32_t)) as ::core::ffi::c_uint & 1 as ::core::ffi::c_uint
            != 0 as ::core::ffi::c_uint
        {
            *ph.offset(k as isize) = *(*cfg).added.offset(i as isize);
            (*ph.offset(k as isize)).normalized = 0 as uint8_t;
            k = k.wrapping_add(1);
        }
        i = i.wrapping_add(1);
    }
    let mut i_0: uint32_t = 0 as uint32_t;
    while i_0 < TOKS_TIKTOKEN_RESERVED as uint32_t {
        *ph.offset(k as isize) = *(*cfg).added.offset(i_0 as isize);
        (*ph.offset(k as isize)).normalized = 1 as uint8_t;
        k = k.wrapping_add(1);
        i_0 = i_0.wrapping_add(1);
    }
    (*cfg).added = ph;
    (*cfg).n_added = k;
    (*cfg).pattern = &raw const TOKS_PATTERN_KIMI;
    (*cfg).cut_chunk = (*info).chunk_chars;
    (*cfg).cut_run = (*info).run_chars;
    return 0 as int64_t;
}
#[no_mangle]
pub unsafe extern "C" fn toks_tiktoken_sniff(
    mut data: *const uint8_t,
    mut len: uint64_t,
) -> ::core::ffi::c_int {
    return (len > 0 as uint64_t
        && toks_b64v(*data.offset(0 as ::core::ffi::c_int as isize))
            >= 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
}
