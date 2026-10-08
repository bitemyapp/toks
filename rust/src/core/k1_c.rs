extern "C" {
    fn memcpy(
        dst: *mut ::core::ffi::c_void,
        src: *const ::core::ffi::c_void,
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
pub const TOKS_K1_H4_BITS: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const RX_CH: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const RX_LAB: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
pub const RX_LLEN: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
pub const RX_NCH: ::core::ffi::c_int = 0xa as ::core::ffi::c_int;
pub const RX_ENT: ::core::ffi::c_int = 0xc as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn toks_ld32(mut p: *const ::core::ffi::c_void) -> uint32_t {
    let mut v: uint32_t = 0;
    memcpy(&raw mut v as *mut ::core::ffi::c_void, p, 4 as size_t);
    return v;
}
#[inline]
unsafe extern "C" fn toks_k1_h4(mut s: *const uint8_t) -> uint32_t {
    let mut k: uint32_t = 0;
    memcpy(
        &raw mut k as *mut ::core::ffi::c_void,
        s as *const ::core::ffi::c_void,
        4 as size_t,
    );
    return k.wrapping_mul(0x9e3779b1 as uint32_t)
        >> (32 as ::core::ffi::c_uint)
            .wrapping_sub(TOKS_K1_H4_BITS as ::core::ffi::c_uint);
}
unsafe extern "C" fn k1_radix(
    mut base: *const uint8_t,
    mut off: uint32_t,
    mut s: *const uint8_t,
    mut avail: uint64_t,
    mut entry: *mut uint64_t,
) -> uint64_t {
    let mut d: uint64_t = 0 as uint64_t;
    let mut best: uint64_t = 0 as uint64_t;
    loop {
        let mut nd: *const uint8_t = base.offset(off as isize);
        let mut ln: uint16_t = 0;
        let mut nc: uint16_t = 0;
        memcpy(
            &raw mut ln as *mut ::core::ffi::c_void,
            nd.offset(RX_LLEN as isize) as *const ::core::ffi::c_void,
            2 as size_t,
        );
        memcpy(
            &raw mut nc as *mut ::core::ffi::c_void,
            nd.offset(RX_NCH as isize) as *const ::core::ffi::c_void,
            2 as size_t,
        );
        let mut lab: uint32_t = toks_ld32(
            nd.offset(RX_LAB as isize) as *const ::core::ffi::c_void,
        );
        let mut ent: uint32_t = toks_ld32(
            nd.offset(RX_ENT as isize) as *const ::core::ffi::c_void,
        );
        let mut k: uint32_t = 0 as uint32_t;
        if ln as uint64_t > avail.wrapping_sub(d)
            || memcmp(
                base.offset(lab as isize) as *const ::core::ffi::c_void,
                s.offset(d as isize) as *const ::core::ffi::c_void,
                ln as size_t,
            ) != 0 as ::core::ffi::c_int
        {
            break;
        }
        d = d.wrapping_add(ln as uint64_t);
        if ent != 0 as uint32_t {
            best = d;
            *entry = ent.wrapping_sub(1 as uint32_t) as uint64_t;
        }
        if d == avail {
            break;
        }
        while k < nc as uint32_t
            && *base.offset(lab.wrapping_add(ln as uint32_t).wrapping_add(k) as isize)
                as ::core::ffi::c_int != *s.offset(d as isize) as ::core::ffi::c_int
        {
            k = k.wrapping_add(1);
        }
        if k == nc as uint32_t {
            break;
        }
        off = toks_ld32(nd.offset(RX_CH as isize) as *const ::core::ffi::c_void)
            .wrapping_add((16 as uint32_t).wrapping_mul(k));
    }
    return best;
}
#[no_mangle]
pub unsafe extern "C" fn toks_k1_added_find_c(
    mut t: *const toks_tables,
    mut a: *mut toks_k1_args,
) -> uint64_t {
    let mut text: *const uint8_t = (*a).text;
    let mut len: uint64_t = (*a).len;
    let mut pos: uint64_t = (*a).pos;
    let mut phase: uint64_t = (*a).phase;
    let mut n: uint64_t = 0 as uint64_t;
    (*a).n = 0 as uint64_t;
    (*a).next = len;
    if phase > 1 as uint64_t
        || (*t).add_phases as ::core::ffi::c_ulonglong
            & (1 as ::core::ffi::c_ulonglong) << phase == 0 as ::core::ffi::c_ulonglong
    {
        return 0 as uint64_t;
    }
    let mut ents: *const toks_added_entry = (*t).add_entries;
    let mut ab: *const uint8_t = (*t).add_bytes;
    let mut shufti_lo: *const uint8_t = (*t)
        .add_shufti
        .offset(phase.wrapping_mul(32 as uint64_t) as isize);
    let mut shufti_hi: *const uint8_t = (*t)
        .add_shufti
        .offset(phase.wrapping_mul(32 as uint64_t) as isize)
        .offset(16 as ::core::ffi::c_uint as isize);
    let mut index2: *const uint64_t = (*t)
        .add_index
        .offset(phase.wrapping_mul(65536 as uint64_t) as isize);
    let mut single: *const uint32_t = (*t)
        .add_single
        .offset(phase.wrapping_mul(256 as uint64_t) as isize);
    let mut i: uint64_t = pos;
    while i < len {
        let mut b0: uint8_t = *text.offset(i as isize);
        if !((*shufti_lo
            .offset((b0 as ::core::ffi::c_uint & 0xf as ::core::ffi::c_uint) as isize)
            as ::core::ffi::c_int
            & *shufti_hi
                .offset((b0 as ::core::ffi::c_int >> 4 as ::core::ffi::c_int) as isize)
                as ::core::ffi::c_int) as ::core::ffi::c_uint
            == 0 as ::core::ffi::c_uint)
        {
            let mut best_len: uint64_t = 0 as uint64_t;
            let mut best_entry: uint64_t = 0 as uint64_t;
            if len.wrapping_sub(i) >= 4 as uint64_t {
                let mut ent: uint64_t = *(*t)
                    .add_index
                    .offset(
                        ((2 as ::core::ffi::c_uint)
                            .wrapping_mul(65536 as ::core::ffi::c_uint) as uint64_t)
                            .wrapping_add(
                                (toks_k1_h4(text.offset(i as isize)) as uint64_t)
                                    << 1 as ::core::ffi::c_int | phase,
                            ) as isize,
                    );
                if ent >> 63 as ::core::ffi::c_int != 0 as uint64_t {
                    best_len = k1_radix(
                        (*t).add_cand as *const uint8_t,
                        ent as uint32_t,
                        text.offset(i as isize),
                        len.wrapping_sub(i),
                        &raw mut best_entry,
                    );
                    ent = 0 as uint64_t;
                }
                let mut cand: *const uint32_t = (*t)
                    .add_cand
                    .offset(
                        (ent as ::core::ffi::c_ulonglong
                            & 0xffffffff as ::core::ffi::c_ulonglong) as isize,
                    );
                let mut c: uint64_t = 0 as uint64_t;
                while c < ent >> 32 as ::core::ffi::c_int {
                    let mut x: *const toks_added_entry = ents
                        .offset(*cand.offset(c as isize) as isize)
                        as *const toks_added_entry;
                    if (*x).len as uint64_t <= len.wrapping_sub(i)
                        && memcmp(
                            ab.offset((*x).off as isize) as *const ::core::ffi::c_void,
                            text.offset(i as isize) as *const ::core::ffi::c_void,
                            (*x).len as size_t,
                        ) == 0 as ::core::ffi::c_int
                    {
                        best_len = (*x).len as uint64_t;
                        best_entry = *cand.offset(c as isize) as uint64_t;
                        break;
                    } else {
                        c = c.wrapping_add(1);
                    }
                }
            }
            if best_len == 0 as uint64_t && len.wrapping_sub(i) >= 2 as uint64_t {
                let mut ent_0: uint64_t = *index2
                    .offset(
                        (b0 as uint64_t
                            | (*text.offset(i.wrapping_add(1 as uint64_t) as isize)
                                as uint64_t) << 8 as ::core::ffi::c_int) as isize,
                    );
                let mut cand_0: *const uint32_t = (*t)
                    .add_cand
                    .offset(
                        (ent_0 as ::core::ffi::c_ulonglong
                            & 0xffffffff as ::core::ffi::c_ulonglong) as isize,
                    );
                let mut c_0: uint64_t = ent_0 >> 32 as ::core::ffi::c_int;
                while c_0 > 0 as uint64_t
                    && ((*ents
                        .offset(
                            *cand_0.offset(c_0.wrapping_sub(1 as uint64_t) as isize)
                                as isize,
                        ))
                        .len as ::core::ffi::c_uint) < 4 as ::core::ffi::c_uint
                {
                    let mut x_0: *const toks_added_entry = ents
                        .offset(
                            *cand_0.offset(c_0.wrapping_sub(1 as uint64_t) as isize)
                                as isize,
                        ) as *const toks_added_entry;
                    if (*x_0).len as uint64_t <= len.wrapping_sub(i)
                        && memcmp(
                            ab.offset((*x_0).off as isize) as *const ::core::ffi::c_void,
                            text.offset(i as isize) as *const ::core::ffi::c_void,
                            (*x_0).len as size_t,
                        ) == 0 as ::core::ffi::c_int
                    {
                        best_len = (*x_0).len as uint64_t;
                        best_entry = *cand_0
                            .offset(c_0.wrapping_sub(1 as uint64_t) as isize)
                            as uint64_t;
                    }
                    c_0 = c_0.wrapping_sub(1);
                }
            }
            if best_len == 0 as uint64_t && *single.offset(b0 as isize) != 0 as uint32_t
            {
                best_len = 1 as uint64_t;
                best_entry = (*single.offset(b0 as isize)).wrapping_sub(1 as uint32_t)
                    as uint64_t;
            }
            if best_len != 0 as uint64_t {
                let fresh0 = n;
                n = n.wrapping_add(1);
                *(*a).m.offset(fresh0 as isize) = toks_k1_match {
                    start: i as uint32_t,
                    end: i.wrapping_add(best_len) as uint32_t,
                    entry: best_entry as uint32_t,
                    rsv: 0 as uint32_t,
                };
                i = i.wrapping_add(best_len.wrapping_sub(1 as uint64_t));
                if n >= (*a).cap {
                    (*a).next = i.wrapping_add(1 as uint64_t);
                    break;
                }
            }
        }
        i = i.wrapping_add(1);
    }
    (*a).n = n;
    return n;
}
