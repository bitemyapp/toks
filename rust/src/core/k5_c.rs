extern "C" {
    fn memcpy(
        dst: *mut ::core::ffi::c_void,
        src: *const ::core::ffi::c_void,
        n: size_t,
    ) -> *mut ::core::ffi::c_void;
    static TOKS_CRC32C_TAB: [uint32_t; 256];
    fn toks_k5_long_c(
        t: *const toks_tables,
        a: *mut toks_k6_args,
        lc: *mut toks_lcache,
    ) -> uint64_t;
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
pub struct toks_k5_args {
    pub text: *const uint8_t,
    pub len: uint64_t,
    pub ends: *const uint32_t,
    pub n: uint64_t,
    pub start: uint64_t,
    pub out: *mut uint32_t,
    pub room: uint64_t,
    pub n_out: uint64_t,
    pub cache: *mut uint8_t,
    pub cache_mask: uint64_t,
    pub work: *mut uint8_t,
    pub work_bytes: uint64_t,
    pub hits_static: uint64_t,
    pub hits_cache: uint64_t,
    pub misses: uint64_t,
    pub cache_tag: uint64_t,
    pub lcache: *mut ::core::ffi::c_void,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bpe_key {
    pub lo: uint64_t,
    pub hi: uint64_t,
}
pub const UINT64_MAX: ::core::ffi::c_ulong = 18446744073709551615
    as ::core::ffi::c_ulong;
pub const TOKS_ID_MASK: ::core::ffi::c_uint = 0x1fffff as ::core::ffi::c_uint;
pub const TOKS_KEY_MAXLEN: ::core::ffi::c_int = 15 as ::core::ffi::c_int;
pub const TOKS_HSEED: ::core::ffi::c_uint = 0x9e3779b9 as ::core::ffi::c_uint;
pub const TOKS_VAL_COUNT_SHIFT: ::core::ffi::c_int = 29 as ::core::ffi::c_int;
pub const TOKS_BUCKET: ::core::ffi::c_int = 64 as ::core::ffi::c_int;
pub const TOKS_TAG_MASK64: ::core::ffi::c_ulonglong = 0xffe00000ffe00000
    as ::core::ffi::c_ulonglong;
pub const TOKS_K5_WARM: ::core::ffi::c_int = 4096 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn toks_tag_word(mut tag: uint64_t) -> uint64_t {
    return (tag & 0x7ff as uint64_t) << 21 as ::core::ffi::c_int
        | (tag >> 11 as ::core::ffi::c_int & 0x7ff as uint64_t)
            << 53 as ::core::ffi::c_int;
}
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
unsafe extern "C" fn bpe_key_at(
    mut text: *const uint8_t,
    mut tlen: uint64_t,
    mut s: uint64_t,
    mut n: uint64_t,
) -> bpe_key {
    let mut k: bpe_key = bpe_key { lo: 0, hi: 0 };
    let mut p: *const uint8_t = text.offset(s as isize);
    if s.wrapping_add(16 as uint64_t) <= tlen {
        memcpy(
            &raw mut k.lo as *mut ::core::ffi::c_void,
            p as *const ::core::ffi::c_void,
            8 as size_t,
        );
        memcpy(
            &raw mut k.hi as *mut ::core::ffi::c_void,
            p.offset(8 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_void,
            8 as size_t,
        );
        k.lo = (k.lo as ::core::ffi::c_ulong
            & if n >= 8 as uint64_t {
                UINT64_MAX
            } else {
                ((1 as ::core::ffi::c_ulong) << (8 as uint64_t).wrapping_mul(n))
                    .wrapping_sub(1 as ::core::ffi::c_ulong)
            }) as uint64_t;
        k.hi = (k.hi as ::core::ffi::c_ulong
            & if n > 8 as uint64_t {
                ((1 as ::core::ffi::c_ulong)
                    << (8 as uint64_t).wrapping_mul(n.wrapping_sub(8 as uint64_t)))
                    .wrapping_sub(1 as ::core::ffi::c_ulong)
            } else {
                0 as ::core::ffi::c_ulong
            }) as uint64_t;
    } else if n > 8 as uint64_t {
        memcpy(
            &raw mut k.lo as *mut ::core::ffi::c_void,
            p as *const ::core::ffi::c_void,
            8 as size_t,
        );
        k.hi = bpe_load_le(
            p.offset(8 as ::core::ffi::c_int as isize),
            n.wrapping_sub(8 as uint64_t),
        );
    } else {
        k.lo = bpe_load_le(p, n);
        k.hi = 0 as uint64_t;
    }
    k.hi |= n << 56 as ::core::ffi::c_int;
    return k;
}
#[inline]
unsafe extern "C" fn bpe_key_hash(mut k: bpe_key) -> uint32_t {
    return toks_crc32c_u64(toks_crc32c_u64(TOKS_HSEED as uint32_t, k.lo), k.hi);
}
#[inline]
unsafe extern "C" fn bpe_bucket_get(
    mut b: *const uint8_t,
    mut k: bpe_key,
) -> *const uint8_t {
    let mut w: [uint64_t; 4] = [0; 4];
    memcpy(
        &raw mut w as *mut uint64_t as *mut ::core::ffi::c_void,
        b as *const ::core::ffi::c_void,
        32 as size_t,
    );
    if w[0 as ::core::ffi::c_int as usize] == k.lo
        && w[1 as ::core::ffi::c_int as usize] == k.hi
    {
        return b.offset(32 as ::core::ffi::c_int as isize);
    }
    if w[2 as ::core::ffi::c_int as usize] == k.lo
        && w[3 as ::core::ffi::c_int as usize] == k.hi
    {
        return b.offset(48 as ::core::ffi::c_int as isize);
    }
    return ::core::ptr::null::<uint8_t>();
}
#[inline]
unsafe extern "C" fn bpe_cache_get(
    mut b: *const uint8_t,
    mut k: bpe_key,
    mut tw: uint64_t,
) -> *const uint8_t {
    let mut v: *const uint8_t = bpe_bucket_get(b, k);
    if v.is_null() {
        return ::core::ptr::null::<uint8_t>();
    }
    let mut g: uint64_t = 0;
    memcpy(
        &raw mut g as *mut ::core::ffi::c_void,
        v.offset(8 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_void,
        8 as size_t,
    );
    return if g as ::core::ffi::c_ulonglong & TOKS_TAG_MASK64
        == tw as ::core::ffi::c_ulonglong
    {
        v
    } else {
        ::core::ptr::null::<uint8_t>()
    };
}
#[inline]
unsafe extern "C" fn bpe_val_pack_tag(
    mut val: *mut uint32_t,
    mut idv: *const uint32_t,
    mut n: uint32_t,
    mut tw: uint64_t,
) {
    *val.offset(0 as ::core::ffi::c_int as isize) = n << TOKS_VAL_COUNT_SHIFT
        | toks_ld32(idv as *const ::core::ffi::c_void) & TOKS_ID_MASK as uint32_t;
    *val.offset(1 as ::core::ffi::c_int as isize) = if n > 1 as uint32_t {
        toks_ld32(
            idv.offset(1 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_void,
        ) & TOKS_ID_MASK as uint32_t
    } else {
        0 as uint32_t
    };
    *val.offset(2 as ::core::ffi::c_int as isize) = (if n > 2 as uint32_t {
        toks_ld32(
            idv.offset(2 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_void,
        ) & TOKS_ID_MASK as uint32_t
    } else {
        0 as uint32_t
    }) | tw as uint32_t;
    *val.offset(3 as ::core::ffi::c_int as isize) = (if n > 3 as uint32_t {
        toks_ld32(
            idv.offset(3 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_void,
        ) & TOKS_ID_MASK as uint32_t
    } else {
        0 as uint32_t
    }) | (tw >> 32 as ::core::ffi::c_int) as uint32_t;
}
#[inline]
unsafe extern "C" fn bpe_val_put(
    mut v: *const uint8_t,
    mut out: *mut ::core::ffi::c_void,
) -> uint64_t {
    let mut w: [uint32_t; 4] = [0; 4];
    memcpy(
        &raw mut w as *mut uint32_t as *mut ::core::ffi::c_void,
        v as *const ::core::ffi::c_void,
        16 as size_t,
    );
    let mut count: uint64_t = (w[0 as ::core::ffi::c_int as usize]
        >> TOKS_VAL_COUNT_SHIFT) as uint64_t;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < 4 as ::core::ffi::c_int {
        w[i as usize] = (w[i as usize] as ::core::ffi::c_uint & TOKS_ID_MASK)
            as uint32_t;
        i += 1;
    }
    memcpy(out, &raw mut w as *mut uint32_t as *const ::core::ffi::c_void, 16 as size_t);
    return count;
}
#[inline]
unsafe extern "C" fn bpe_cache_fill(
    mut bucket: *mut uint8_t,
    mut k: bpe_key,
    mut val: *const uint32_t,
) {
    memcpy(
        bucket.offset(16 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_void,
        bucket as *const ::core::ffi::c_void,
        16 as size_t,
    );
    memcpy(
        bucket.offset(48 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_void,
        bucket.offset(32 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_void,
        16 as size_t,
    );
    memcpy(
        bucket as *mut ::core::ffi::c_void,
        &raw mut k.lo as *const ::core::ffi::c_void,
        8 as size_t,
    );
    memcpy(
        bucket.offset(8 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_void,
        &raw mut k.hi as *const ::core::ffi::c_void,
        8 as size_t,
    );
    memcpy(
        bucket.offset(32 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_void,
        val as *const ::core::ffi::c_void,
        16 as size_t,
    );
}
#[inline]
unsafe extern "C" fn bpe_words_probe(
    mut words: *const uint8_t,
    mut mask: uint64_t,
    mut h: uint32_t,
    mut k: bpe_key,
) -> *const uint8_t {
    let mut v: *const uint8_t = bpe_bucket_get(
        words
            .offset(
                (h as uint64_t & mask).wrapping_mul(TOKS_BUCKET as uint64_t) as isize,
            ),
        k,
    );
    if v.is_null() {
        v = bpe_bucket_get(
            words
                .offset(
                    ((h >> 16 as ::core::ffi::c_int | h << 16 as ::core::ffi::c_int)
                        as uint64_t & mask)
                        .wrapping_mul(TOKS_BUCKET as uint64_t) as isize,
                ),
            k,
        );
    }
    return v;
}
#[no_mangle]
pub unsafe extern "C" fn toks_k5_encode_c(
    mut t: *const toks_tables,
    mut a: *mut toks_k5_args,
) -> uint64_t {
    let mut text: *const uint8_t = (*a).text;
    let mut ends: *const uint32_t = (*a).ends;
    let tlen: uint64_t = (*a).len;
    let np: uint64_t = (*a).n;
    let cmask: uint64_t = (*a).cache_mask;
    let wbytes: uint64_t = (*a).work_bytes;
    let tw: uint64_t = toks_tag_word((*a).cache_tag) as uint64_t;
    let mut out: *mut uint32_t = (*a).out;
    let mut cache: *mut uint8_t = (*a).cache;
    let mut work: *mut uint8_t = (*a).work;
    let mut byte2id: *const uint32_t = (*t).byte2id;
    let mut words: *const uint8_t = (*t).words;
    let wmask: uint64_t = (*t).words_mask;
    let mut n_out: uint64_t = 0 as uint64_t;
    let mut hits_static: uint64_t = 0 as uint64_t;
    let mut hits_cache: uint64_t = 0 as uint64_t;
    let mut misses: uint64_t = 0 as uint64_t;
    let mut s: uint64_t = (*a).start;
    let warm: ::core::ffi::c_int = ((*a)
        .hits_static
        .wrapping_add((*a).hits_cache)
        .wrapping_add((*a).misses) >= TOKS_K5_WARM as uint64_t) as ::core::ffi::c_int;
    let mut current_block_27: u64;
    let mut i: uint64_t = 0 as uint64_t;
    while i < np {
        let mut ps: uint64_t = s;
        let mut len: uint64_t = (*ends.offset(i as isize) as uint64_t).wrapping_sub(s);
        let mut o: *mut uint32_t = out.offset(n_out as isize);
        s = *ends.offset(i as isize) as uint64_t;
        if len == 1 as uint64_t {
            toks_st32(
                o as *mut ::core::ffi::c_void,
                *byte2id.offset(*text.offset(ps as isize) as isize),
            );
            n_out = n_out.wrapping_add(1 as uint64_t);
            hits_static = hits_static.wrapping_add(1 as uint64_t);
        } else {
            let mut bucket: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
            let mut k: bpe_key = bpe_key {
                lo: 0 as uint64_t,
                hi: 0 as uint64_t,
            };
            if len <= TOKS_KEY_MAXLEN as uint64_t {
                k = bpe_key_at(text, tlen, ps, len);
                let mut h: uint32_t = bpe_key_hash(k);
                if !cache.is_null() {
                    bucket = cache
                        .offset(
                            (h as uint64_t & cmask).wrapping_mul(TOKS_BUCKET as uint64_t)
                                as isize,
                        );
                }
                let mut v: *const uint8_t = if warm != 0 && !bucket.is_null() {
                    bpe_cache_get(bucket, k, tw)
                } else {
                    ::core::ptr::null::<uint8_t>()
                };
                if !v.is_null() {
                    n_out = n_out
                        .wrapping_add(bpe_val_put(v, o as *mut ::core::ffi::c_void));
                    hits_cache = hits_cache.wrapping_add(1 as uint64_t);
                    current_block_27 = 15240798224410183470;
                } else {
                    v = if !words.is_null() {
                        bpe_words_probe(words, wmask, h, k)
                    } else {
                        ::core::ptr::null::<uint8_t>()
                    };
                    if !v.is_null() {
                        n_out = n_out
                            .wrapping_add(bpe_val_put(v, o as *mut ::core::ffi::c_void));
                        hits_static = hits_static.wrapping_add(1 as uint64_t);
                        if warm != 0 && !bucket.is_null() {
                            let mut val: [uint32_t; 4] = [0; 4];
                            memcpy(
                                &raw mut val as *mut uint32_t as *mut ::core::ffi::c_void,
                                v as *const ::core::ffi::c_void,
                                16 as size_t,
                            );
                            val[2 as ::core::ffi::c_int as usize] |= tw as uint32_t;
                            val[3 as ::core::ffi::c_int as usize]
                                |= (tw >> 32 as ::core::ffi::c_int) as uint32_t;
                            bpe_cache_fill(
                                bucket,
                                k,
                                &raw mut val as *mut uint32_t as *const uint32_t,
                            );
                        }
                        current_block_27 = 15240798224410183470;
                    } else {
                        v = if warm == 0 && !bucket.is_null() {
                            bpe_cache_get(bucket, k, tw)
                        } else {
                            ::core::ptr::null::<uint8_t>()
                        };
                        if !v.is_null() {
                            n_out = n_out
                                .wrapping_add(
                                    bpe_val_put(v, o as *mut ::core::ffi::c_void),
                                );
                            hits_cache = hits_cache.wrapping_add(1 as uint64_t);
                            current_block_27 = 15240798224410183470;
                        } else {
                            current_block_27 = 17788412896529399552;
                        }
                    }
                }
            } else {
                current_block_27 = 17788412896529399552;
            }
            match current_block_27 {
                15240798224410183470 => {}
                _ => {
                    let mut k6: toks_k6_args = toks_k6_args {
                        piece: text.offset(ps as isize),
                        len: len,
                        out: o,
                        work: work,
                        work_bytes: wbytes,
                        n_out: 0 as uint64_t,
                        merges: 0 as uint64_t,
                        rsv: 0 as uint64_t,
                    };
                    let mut n: uint64_t = toks_k5_long_c(
                        t,
                        &raw mut k6,
                        (*a).lcache as *mut toks_lcache,
                    );
                    misses = misses.wrapping_add(1 as uint64_t);
                    if !bucket.is_null() && n <= 4 as uint64_t {
                        let mut val_0: [uint32_t; 4] = [0; 4];
                        bpe_val_pack_tag(
                            &raw mut val_0 as *mut uint32_t,
                            o,
                            n as uint32_t,
                            tw,
                        );
                        bpe_cache_fill(
                            bucket,
                            k,
                            &raw mut val_0 as *mut uint32_t as *const uint32_t,
                        );
                    }
                    n_out = n_out.wrapping_add(n);
                }
            }
        }
        i = i.wrapping_add(1);
    }
    (*a).n_out = n_out;
    (*a).hits_static = (*a).hits_static.wrapping_add(hits_static);
    (*a).hits_cache = (*a).hits_cache.wrapping_add(hits_cache);
    (*a).misses = (*a).misses.wrapping_add(misses);
    return n_out;
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
