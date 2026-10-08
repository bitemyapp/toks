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
    #[cfg(all(feature = "avx512", target_arch = "x86_64"))]
    fn toks_k6_bpe_avx512(t: *const toks_tables, a: *mut toks_k6_args) -> uint64_t;
    fn toks_k6_bpe_c(t: *const toks_tables, a: *mut toks_k6_args) -> uint64_t;
    #[cfg_attr(target_arch = "x86_64", link_name = "toks_k6_bpe_avx2")]
    fn toks_k6_bpe_neon(t: *const toks_tables, a: *mut toks_k6_args) -> uint64_t;
    #[cfg_attr(target_arch = "aarch64", link_name = "toks_k6_bpe_neon")]
    fn toks_k6_bpe_avx2(t: *const toks_tables, a: *mut toks_k6_args) -> uint64_t;
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
pub struct toks_lcache {
    pub buckets: *mut uint8_t,
    pub arena: *mut uint8_t,
    pub mask: uint64_t,
    pub arena_bytes: uint64_t,
    pub pos: uint64_t,
    pub gen: uint64_t,
    pub hits: uint64_t,
    pub misses: uint64_t,
}
pub const TOKS_BUCKET: ::core::ffi::c_int = 64 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn toks_mix64(mut x: uint64_t, mut w: uint64_t) -> uint64_t {
    x = ((x ^ w) as ::core::ffi::c_ulonglong)
        .wrapping_mul(0xff51afd7ed558ccd as ::core::ffi::c_ulonglong) as uint64_t;
    return x ^ x >> 32 as ::core::ffi::c_int;
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
unsafe extern "C" fn lc_hash(mut p: *const uint8_t, mut n: uint64_t) -> uint32_t {
    let mut h: uint64_t = (0x9e3779b97f4a7c15 as ::core::ffi::c_ulonglong
        ^ n as ::core::ffi::c_ulonglong) as uint64_t;
    let mut i: uint64_t = 0 as uint64_t;
    while i.wrapping_add(8 as uint64_t) <= n {
        h = toks_mix64(h, bpe_load_le(p.offset(i as isize), 8 as uint64_t));
        i = i.wrapping_add(8 as uint64_t);
    }
    if i < n {
        h = toks_mix64(h, bpe_load_le(p.offset(i as isize), n.wrapping_sub(i)));
    }
    return crate::test_hash((h ^ h >> 29 as ::core::ffi::c_int) as uint32_t);
}
#[inline]
unsafe extern "C" fn lc_same(
    mut a: *const uint8_t,
    mut b: *const uint8_t,
    mut n: uint64_t,
) -> ::core::ffi::c_int {
    let mut i: uint64_t = 0 as uint64_t;
    while i.wrapping_add(8 as uint64_t) <= n {
        if bpe_load_le(a.offset(i as isize), 8 as uint64_t)
            != bpe_load_le(b.offset(i as isize), 8 as uint64_t)
        {
            return 0 as ::core::ffi::c_int;
        }
        i = i.wrapping_add(8 as uint64_t);
    }
    return (i == n
        || bpe_load_le(a.offset(i as isize), n.wrapping_sub(i))
            == bpe_load_le(b.offset(i as isize), n.wrapping_sub(i)))
        as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn lc_ids_at(mut len: uint64_t) -> uint64_t {
    return (4 as uint64_t).wrapping_add(len).wrapping_add(3 as uint64_t)
        & !(3 as ::core::ffi::c_uint as uint64_t);
}
#[inline]
unsafe extern "C" fn lc_bytes(mut len: uint64_t, mut n: uint64_t) -> uint64_t {
    return lc_ids_at(len)
        .wrapping_add((4 as uint64_t).wrapping_mul(n))
        .wrapping_add(27 as uint64_t) & !(15 as ::core::ffi::c_uint as uint64_t);
}
#[inline(always)]
unsafe extern "C" fn lc_get(
    mut lc: *mut toks_lcache,
    mut a: *mut toks_k6_args,
    mut s: *const uint32_t,
    mut h: uint32_t,
) -> uint64_t {
    let mut len: uint64_t = (*a).len;
    let mut hl: uint64_t = h as uint64_t | len << 32 as ::core::ffi::c_int;
    let mut g: uint64_t = 0;
    let mut w: uint32_t = 0 as uint32_t;
    while w < 4 as uint32_t {
        memcpy(
            &raw mut g as *mut ::core::ffi::c_void,
            s as *const ::core::ffi::c_void,
            8 as size_t,
        );
        if !(g != hl
            || *s.offset(3 as ::core::ffi::c_int as isize) != (*lc).gen as uint32_t
            || (*s.offset(2 as ::core::ffi::c_int as isize) as uint64_t)
                .wrapping_add(lc_bytes(len, 0 as uint64_t)) > (*lc).pos)
        {
            let mut e: *const uint8_t = (*lc)
                .arena
                .offset(*s.offset(2 as ::core::ffi::c_int as isize) as isize);
            let mut n: uint32_t = 0;
            memcpy(
                &raw mut n as *mut ::core::ffi::c_void,
                e as *const ::core::ffi::c_void,
                4 as size_t,
            );
            if !(n == 0 as uint32_t || n as uint64_t > len
                || (*s.offset(2 as ::core::ffi::c_int as isize) as uint64_t)
                    .wrapping_add(lc_bytes(len, n as uint64_t)) > (*lc).pos
                || lc_same(e.offset(4 as ::core::ffi::c_int as isize), (*a).piece, len)
                    == 0)
            {
                let mut j: uint64_t = 0 as uint64_t;
                while j < (4 as uint64_t).wrapping_mul(n as uint64_t) {
                    memcpy(
                        ((*a).out as *mut ::core::ffi::c_void as *mut uint8_t)
                            .offset(j as isize) as *mut ::core::ffi::c_void,
                        e.offset(lc_ids_at(len) as isize).offset(j as isize)
                            as *const ::core::ffi::c_void,
                        16 as size_t,
                    );
                    j = j.wrapping_add(16 as uint64_t);
                }
                (*a).n_out = n as uint64_t;
                (*a).merges = 0 as uint64_t;
                (*lc).hits = (*lc).hits.wrapping_add(1 as uint64_t);
                return n as uint64_t;
            }
        }
        w = w.wrapping_add(1);
        s = s.offset(4 as ::core::ffi::c_int as isize);
    }
    return 0 as uint64_t;
}
unsafe extern "C" fn lc_put(
    mut lc: *mut toks_lcache,
    mut a: *const toks_k6_args,
    mut s: *mut uint32_t,
    mut h: uint32_t,
    mut n: uint64_t,
) {
    let mut len: uint64_t = (*a).len;
    let mut need: uint64_t = lc_bytes(len, n);
    let mut pos: uint64_t = (*lc).pos;
    if need > (*lc).arena_bytes {
        return;
    }
    if need <= 64 as uint64_t
        && (pos & 63 as uint64_t).wrapping_add(need) > 64 as uint64_t
    {
        pos = pos.wrapping_add(63 as uint64_t)
            & !(63 as ::core::ffi::c_uint as uint64_t);
    }
    if pos > (*lc).arena_bytes.wrapping_sub(need) {
        (*lc).gen = if (*lc).gen >= 0xffffffff as uint64_t {
            1 as uint64_t
        } else {
            (*lc).gen.wrapping_add(1 as uint64_t)
        };
        if (*lc).gen == 1 as uint64_t {
            memset(
                (*lc).buckets as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                (*lc)
                    .mask
                    .wrapping_add(1 as uint64_t)
                    .wrapping_mul(TOKS_BUCKET as uint64_t) as size_t,
            );
        }
        pos = 0 as uint64_t;
    }
    let mut e: *mut uint8_t = (*lc).arena.offset(pos as isize);
    let mut n32: uint32_t = n as uint32_t;
    memcpy(
        e as *mut ::core::ffi::c_void,
        &raw mut n32 as *const ::core::ffi::c_void,
        4 as size_t,
    );
    let mut i: uint64_t = 0 as uint64_t;
    while i < len {
        *e.offset((4 as uint64_t).wrapping_add(i) as isize) = *(*a)
            .piece
            .offset(i as isize);
        i = i.wrapping_add(1);
    }
    let mut j: uint64_t = 0 as uint64_t;
    while j < (4 as uint64_t).wrapping_mul(n) {
        memcpy(
            e.offset(lc_ids_at(len) as isize).offset(j as isize)
                as *mut ::core::ffi::c_void,
            ((*a).out as *const ::core::ffi::c_void as *const uint8_t).offset(j as isize)
                as *const ::core::ffi::c_void,
            16 as size_t,
        );
        j = j.wrapping_add(16 as uint64_t);
    }
    memcpy(
        s.offset(12 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_void,
        s.offset(8 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_void,
        16 as size_t,
    );
    memcpy(
        s.offset(8 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_void,
        s.offset(4 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_void,
        16 as size_t,
    );
    memcpy(
        s.offset(4 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_void,
        s as *const ::core::ffi::c_void,
        16 as size_t,
    );
    *s.offset(0 as ::core::ffi::c_int as isize) = h;
    *s.offset(1 as ::core::ffi::c_int as isize) = len as uint32_t;
    *s.offset(2 as ::core::ffi::c_int as isize) = pos as uint32_t;
    *s.offset(3 as ::core::ffi::c_int as isize) = (*lc).gen as uint32_t;
    (*lc).pos = pos.wrapping_add(need);
}
#[no_mangle]
pub unsafe extern "C" fn toks_k5_long_c(
    mut t: *const toks_tables,
    mut a: *mut toks_k6_args,
    mut lc: *mut toks_lcache,
) -> uint64_t {
    if lc.is_null() || (*a).len < 5 as uint64_t {
        return toks_k6_bpe_c(t, a);
    }
    let mut h: uint32_t = lc_hash((*a).piece, (*a).len);
    let mut s: *mut uint32_t = (*lc)
        .buckets
        .offset(
            (h as uint64_t & (*lc).mask).wrapping_mul(TOKS_BUCKET as uint64_t) as isize,
        ) as *mut ::core::ffi::c_void as *mut uint32_t;
    let mut n: uint64_t = lc_get(lc, a, s, h);
    if n != 0 as uint64_t {
        return n;
    }
    n = toks_k6_bpe_c(t, a);
    (*lc).misses = (*lc).misses.wrapping_add(1 as uint64_t);
    if ((*a).len > 15 as uint64_t || n > 4 as uint64_t) && n != 0 as uint64_t {
        lc_put(lc, a, s, h, n);
    }
    return n;
}
#[no_mangle]
pub unsafe extern "C" fn toks_k5_long_neon(
    mut t: *const toks_tables,
    mut a: *mut toks_k6_args,
    mut lc: *mut toks_lcache,
) -> uint64_t {
    if lc.is_null() || (*a).len < 5 as uint64_t {
        return toks_k6_bpe_neon(t, a);
    }
    let mut h: uint32_t = lc_hash((*a).piece, (*a).len);
    let mut s: *mut uint32_t = (*lc)
        .buckets
        .offset(
            (h as uint64_t & (*lc).mask).wrapping_mul(TOKS_BUCKET as uint64_t) as isize,
        ) as *mut ::core::ffi::c_void as *mut uint32_t;
    let mut n: uint64_t = lc_get(lc, a, s, h);
    if n != 0 as uint64_t {
        return n;
    }
    n = toks_k6_bpe_neon(t, a);
    (*lc).misses = (*lc).misses.wrapping_add(1 as uint64_t);
    if ((*a).len > 15 as uint64_t || n > 4 as uint64_t) && n != 0 as uint64_t {
        lc_put(lc, a, s, h, n);
    }
    return n;
}
#[no_mangle]
pub unsafe extern "C" fn toks_k5_long_avx2(
    mut t: *const toks_tables,
    mut a: *mut toks_k6_args,
    mut lc: *mut toks_lcache,
) -> uint64_t {
    if lc.is_null() || (*a).len < 5 as uint64_t {
        return toks_k6_bpe_avx2(t, a);
    }
    let mut h: uint32_t = lc_hash((*a).piece, (*a).len);
    let mut s: *mut uint32_t = (*lc)
        .buckets
        .offset(
            (h as uint64_t & (*lc).mask).wrapping_mul(TOKS_BUCKET as uint64_t) as isize,
        ) as *mut ::core::ffi::c_void as *mut uint32_t;
    let mut n: uint64_t = lc_get(lc, a, s, h);
    if n != 0 as uint64_t {
        return n;
    }
    n = toks_k6_bpe_avx2(t, a);
    (*lc).misses = (*lc).misses.wrapping_add(1 as uint64_t);
    if ((*a).len > 15 as uint64_t || n > 4 as uint64_t) && n != 0 as uint64_t {
        lc_put(lc, a, s, h, n);
    }
    return n;
}
#[cfg(all(feature = "avx512", target_arch = "x86_64"))]
#[no_mangle]
pub unsafe extern "C" fn toks_k5_long_avx512(
    mut t: *const toks_tables,
    mut a: *mut toks_k6_args,
    mut lc: *mut toks_lcache,
) -> uint64_t {
    if lc.is_null() || (*a).len < 5 as uint64_t {
        return toks_k6_bpe_avx512(t, a);
    }
    let mut h: uint32_t = lc_hash((*a).piece, (*a).len);
    let mut s: *mut uint32_t = (*lc)
        .buckets
        .offset(
            (h as uint64_t & (*lc).mask).wrapping_mul(TOKS_BUCKET as uint64_t) as isize,
        ) as *mut ::core::ffi::c_void as *mut uint32_t;
    let mut n: uint64_t = lc_get(lc, a, s, h);
    if n != 0 as uint64_t {
        return n;
    }
    n = toks_k6_bpe_avx512(t, a);
    (*lc).misses = (*lc).misses.wrapping_add(1 as uint64_t);
    if ((*a).len > 15 as uint64_t || n > 4 as uint64_t) && n != 0 as uint64_t {
        lc_put(lc, a, s, h, n);
    }
    return n;
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
