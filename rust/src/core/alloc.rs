extern "C" {
    fn memset(
        dst: *mut ::core::ffi::c_void,
        c: ::core::ffi::c_int,
        n: size_t,
    ) -> *mut ::core::ffi::c_void;
}
pub type size_t = usize;
pub type __uint8_t = u8;
pub type __int32_t = i32;
pub type __uint32_t = u32;
pub type __uint64_t = u64;
pub type int32_t = __int32_t;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct s256 {
    pub h: [uint32_t; 8],
    pub len: uint64_t,
    pub buf: [uint32_t; 16],
    pub n: uint32_t,
}
#[no_mangle]
pub static mut TOKS_CRC32C_TAB: [uint32_t; 256] = [
    0 as ::core::ffi::c_uint,
    0xf26b8303 as ::core::ffi::c_uint,
    0xe13b70f7 as ::core::ffi::c_uint,
    0x1350f3f4 as ::core::ffi::c_uint,
    0xc79a971f as ::core::ffi::c_uint,
    0x35f1141c as ::core::ffi::c_uint,
    0x26a1e7e8 as ::core::ffi::c_uint,
    0xd4ca64eb as ::core::ffi::c_uint,
    0x8ad958cf as ::core::ffi::c_uint,
    0x78b2dbcc as ::core::ffi::c_uint,
    0x6be22838 as ::core::ffi::c_uint,
    0x9989ab3b as ::core::ffi::c_uint,
    0x4d43cfd0 as ::core::ffi::c_uint,
    0xbf284cd3 as ::core::ffi::c_uint,
    0xac78bf27 as ::core::ffi::c_uint,
    0x5e133c24 as ::core::ffi::c_uint,
    0x105ec76f as ::core::ffi::c_uint,
    0xe235446c as ::core::ffi::c_uint,
    0xf165b798 as ::core::ffi::c_uint,
    0x30e349b as ::core::ffi::c_uint,
    0xd7c45070 as ::core::ffi::c_uint,
    0x25afd373 as ::core::ffi::c_uint,
    0x36ff2087 as ::core::ffi::c_uint,
    0xc494a384 as ::core::ffi::c_uint,
    0x9a879fa0 as ::core::ffi::c_uint,
    0x68ec1ca3 as ::core::ffi::c_uint,
    0x7bbcef57 as ::core::ffi::c_uint,
    0x89d76c54 as ::core::ffi::c_uint,
    0x5d1d08bf as ::core::ffi::c_uint,
    0xaf768bbc as ::core::ffi::c_uint,
    0xbc267848 as ::core::ffi::c_uint,
    0x4e4dfb4b as ::core::ffi::c_uint,
    0x20bd8ede as ::core::ffi::c_uint,
    0xd2d60ddd as ::core::ffi::c_uint,
    0xc186fe29 as ::core::ffi::c_uint,
    0x33ed7d2a as ::core::ffi::c_uint,
    0xe72719c1 as ::core::ffi::c_uint,
    0x154c9ac2 as ::core::ffi::c_uint,
    0x61c6936 as ::core::ffi::c_uint,
    0xf477ea35 as ::core::ffi::c_uint,
    0xaa64d611 as ::core::ffi::c_uint,
    0x580f5512 as ::core::ffi::c_uint,
    0x4b5fa6e6 as ::core::ffi::c_uint,
    0xb93425e5 as ::core::ffi::c_uint,
    0x6dfe410e as ::core::ffi::c_uint,
    0x9f95c20d as ::core::ffi::c_uint,
    0x8cc531f9 as ::core::ffi::c_uint,
    0x7eaeb2fa as ::core::ffi::c_uint,
    0x30e349b1 as ::core::ffi::c_uint,
    0xc288cab2 as ::core::ffi::c_uint,
    0xd1d83946 as ::core::ffi::c_uint,
    0x23b3ba45 as ::core::ffi::c_uint,
    0xf779deae as ::core::ffi::c_uint,
    0x5125dad as ::core::ffi::c_uint,
    0x1642ae59 as ::core::ffi::c_uint,
    0xe4292d5a as ::core::ffi::c_uint,
    0xba3a117e as ::core::ffi::c_uint,
    0x4851927d as ::core::ffi::c_uint,
    0x5b016189 as ::core::ffi::c_uint,
    0xa96ae28a as ::core::ffi::c_uint,
    0x7da08661 as ::core::ffi::c_uint,
    0x8fcb0562 as ::core::ffi::c_uint,
    0x9c9bf696 as ::core::ffi::c_uint,
    0x6ef07595 as ::core::ffi::c_uint,
    0x417b1dbc as ::core::ffi::c_uint,
    0xb3109ebf as ::core::ffi::c_uint,
    0xa0406d4b as ::core::ffi::c_uint,
    0x522bee48 as ::core::ffi::c_uint,
    0x86e18aa3 as ::core::ffi::c_uint,
    0x748a09a0 as ::core::ffi::c_uint,
    0x67dafa54 as ::core::ffi::c_uint,
    0x95b17957 as ::core::ffi::c_uint,
    0xcba24573 as ::core::ffi::c_uint,
    0x39c9c670 as ::core::ffi::c_uint,
    0x2a993584 as ::core::ffi::c_uint,
    0xd8f2b687 as ::core::ffi::c_uint,
    0xc38d26c as ::core::ffi::c_uint,
    0xfe53516f as ::core::ffi::c_uint,
    0xed03a29b as ::core::ffi::c_uint,
    0x1f682198 as ::core::ffi::c_uint,
    0x5125dad3 as ::core::ffi::c_uint,
    0xa34e59d0 as ::core::ffi::c_uint,
    0xb01eaa24 as ::core::ffi::c_uint,
    0x42752927 as ::core::ffi::c_uint,
    0x96bf4dcc as ::core::ffi::c_uint,
    0x64d4cecf as ::core::ffi::c_uint,
    0x77843d3b as ::core::ffi::c_uint,
    0x85efbe38 as ::core::ffi::c_uint,
    0xdbfc821c as ::core::ffi::c_uint,
    0x2997011f as ::core::ffi::c_uint,
    0x3ac7f2eb as ::core::ffi::c_uint,
    0xc8ac71e8 as ::core::ffi::c_uint,
    0x1c661503 as ::core::ffi::c_uint,
    0xee0d9600 as ::core::ffi::c_uint,
    0xfd5d65f4 as ::core::ffi::c_uint,
    0xf36e6f7 as ::core::ffi::c_uint,
    0x61c69362 as ::core::ffi::c_uint,
    0x93ad1061 as ::core::ffi::c_uint,
    0x80fde395 as ::core::ffi::c_uint,
    0x72966096 as ::core::ffi::c_uint,
    0xa65c047d as ::core::ffi::c_uint,
    0x5437877e as ::core::ffi::c_uint,
    0x4767748a as ::core::ffi::c_uint,
    0xb50cf789 as ::core::ffi::c_uint,
    0xeb1fcbad as ::core::ffi::c_uint,
    0x197448ae as ::core::ffi::c_uint,
    0xa24bb5a as ::core::ffi::c_uint,
    0xf84f3859 as ::core::ffi::c_uint,
    0x2c855cb2 as ::core::ffi::c_uint,
    0xdeeedfb1 as ::core::ffi::c_uint,
    0xcdbe2c45 as ::core::ffi::c_uint,
    0x3fd5af46 as ::core::ffi::c_uint,
    0x7198540d as ::core::ffi::c_uint,
    0x83f3d70e as ::core::ffi::c_uint,
    0x90a324fa as ::core::ffi::c_uint,
    0x62c8a7f9 as ::core::ffi::c_uint,
    0xb602c312 as ::core::ffi::c_uint,
    0x44694011 as ::core::ffi::c_uint,
    0x5739b3e5 as ::core::ffi::c_uint,
    0xa55230e6 as ::core::ffi::c_uint,
    0xfb410cc2 as ::core::ffi::c_uint,
    0x92a8fc1 as ::core::ffi::c_uint,
    0x1a7a7c35 as ::core::ffi::c_uint,
    0xe811ff36 as ::core::ffi::c_uint,
    0x3cdb9bdd as ::core::ffi::c_uint,
    0xceb018de as ::core::ffi::c_uint,
    0xdde0eb2a as ::core::ffi::c_uint,
    0x2f8b6829 as ::core::ffi::c_uint,
    0x82f63b78 as ::core::ffi::c_uint,
    0x709db87b as ::core::ffi::c_uint,
    0x63cd4b8f as ::core::ffi::c_uint,
    0x91a6c88c as ::core::ffi::c_uint,
    0x456cac67 as ::core::ffi::c_uint,
    0xb7072f64 as ::core::ffi::c_uint,
    0xa457dc90 as ::core::ffi::c_uint,
    0x563c5f93 as ::core::ffi::c_uint,
    0x82f63b7 as ::core::ffi::c_uint,
    0xfa44e0b4 as ::core::ffi::c_uint,
    0xe9141340 as ::core::ffi::c_uint,
    0x1b7f9043 as ::core::ffi::c_uint,
    0xcfb5f4a8 as ::core::ffi::c_uint,
    0x3dde77ab as ::core::ffi::c_uint,
    0x2e8e845f as ::core::ffi::c_uint,
    0xdce5075c as ::core::ffi::c_uint,
    0x92a8fc17 as ::core::ffi::c_uint,
    0x60c37f14 as ::core::ffi::c_uint,
    0x73938ce0 as ::core::ffi::c_uint,
    0x81f80fe3 as ::core::ffi::c_uint,
    0x55326b08 as ::core::ffi::c_uint,
    0xa759e80b as ::core::ffi::c_uint,
    0xb4091bff as ::core::ffi::c_uint,
    0x466298fc as ::core::ffi::c_uint,
    0x1871a4d8 as ::core::ffi::c_uint,
    0xea1a27db as ::core::ffi::c_uint,
    0xf94ad42f as ::core::ffi::c_uint,
    0xb21572c as ::core::ffi::c_uint,
    0xdfeb33c7 as ::core::ffi::c_uint,
    0x2d80b0c4 as ::core::ffi::c_uint,
    0x3ed04330 as ::core::ffi::c_uint,
    0xccbbc033 as ::core::ffi::c_uint,
    0xa24bb5a6 as ::core::ffi::c_uint,
    0x502036a5 as ::core::ffi::c_uint,
    0x4370c551 as ::core::ffi::c_uint,
    0xb11b4652 as ::core::ffi::c_uint,
    0x65d122b9 as ::core::ffi::c_uint,
    0x97baa1ba as ::core::ffi::c_uint,
    0x84ea524e as ::core::ffi::c_uint,
    0x7681d14d as ::core::ffi::c_uint,
    0x2892ed69 as ::core::ffi::c_uint,
    0xdaf96e6a as ::core::ffi::c_uint,
    0xc9a99d9e as ::core::ffi::c_uint,
    0x3bc21e9d as ::core::ffi::c_uint,
    0xef087a76 as ::core::ffi::c_uint,
    0x1d63f975 as ::core::ffi::c_uint,
    0xe330a81 as ::core::ffi::c_uint,
    0xfc588982 as ::core::ffi::c_uint,
    0xb21572c9 as ::core::ffi::c_uint,
    0x407ef1ca as ::core::ffi::c_uint,
    0x532e023e as ::core::ffi::c_uint,
    0xa145813d as ::core::ffi::c_uint,
    0x758fe5d6 as ::core::ffi::c_uint,
    0x87e466d5 as ::core::ffi::c_uint,
    0x94b49521 as ::core::ffi::c_uint,
    0x66df1622 as ::core::ffi::c_uint,
    0x38cc2a06 as ::core::ffi::c_uint,
    0xcaa7a905 as ::core::ffi::c_uint,
    0xd9f75af1 as ::core::ffi::c_uint,
    0x2b9cd9f2 as ::core::ffi::c_uint,
    0xff56bd19 as ::core::ffi::c_uint,
    0xd3d3e1a as ::core::ffi::c_uint,
    0x1e6dcdee as ::core::ffi::c_uint,
    0xec064eed as ::core::ffi::c_uint,
    0xc38d26c4 as ::core::ffi::c_uint,
    0x31e6a5c7 as ::core::ffi::c_uint,
    0x22b65633 as ::core::ffi::c_uint,
    0xd0ddd530 as ::core::ffi::c_uint,
    0x417b1db as ::core::ffi::c_uint,
    0xf67c32d8 as ::core::ffi::c_uint,
    0xe52cc12c as ::core::ffi::c_uint,
    0x1747422f as ::core::ffi::c_uint,
    0x49547e0b as ::core::ffi::c_uint,
    0xbb3ffd08 as ::core::ffi::c_uint,
    0xa86f0efc as ::core::ffi::c_uint,
    0x5a048dff as ::core::ffi::c_uint,
    0x8ecee914 as ::core::ffi::c_uint,
    0x7ca56a17 as ::core::ffi::c_uint,
    0x6ff599e3 as ::core::ffi::c_uint,
    0x9d9e1ae0 as ::core::ffi::c_uint,
    0xd3d3e1ab as ::core::ffi::c_uint,
    0x21b862a8 as ::core::ffi::c_uint,
    0x32e8915c as ::core::ffi::c_uint,
    0xc083125f as ::core::ffi::c_uint,
    0x144976b4 as ::core::ffi::c_uint,
    0xe622f5b7 as ::core::ffi::c_uint,
    0xf5720643 as ::core::ffi::c_uint,
    0x7198540 as ::core::ffi::c_uint,
    0x590ab964 as ::core::ffi::c_uint,
    0xab613a67 as ::core::ffi::c_uint,
    0xb831c993 as ::core::ffi::c_uint,
    0x4a5a4a90 as ::core::ffi::c_uint,
    0x9e902e7b as ::core::ffi::c_uint,
    0x6cfbad78 as ::core::ffi::c_uint,
    0x7fab5e8c as ::core::ffi::c_uint,
    0x8dc0dd8f as ::core::ffi::c_uint,
    0xe330a81a as ::core::ffi::c_uint,
    0x115b2b19 as ::core::ffi::c_uint,
    0x20bd8ed as ::core::ffi::c_uint,
    0xf0605bee as ::core::ffi::c_uint,
    0x24aa3f05 as ::core::ffi::c_uint,
    0xd6c1bc06 as ::core::ffi::c_uint,
    0xc5914ff2 as ::core::ffi::c_uint,
    0x37faccf1 as ::core::ffi::c_uint,
    0x69e9f0d5 as ::core::ffi::c_uint,
    0x9b8273d6 as ::core::ffi::c_uint,
    0x88d28022 as ::core::ffi::c_uint,
    0x7ab90321 as ::core::ffi::c_uint,
    0xae7367ca as ::core::ffi::c_uint,
    0x5c18e4c9 as ::core::ffi::c_uint,
    0x4f48173d as ::core::ffi::c_uint,
    0xbd23943e as ::core::ffi::c_uint,
    0xf36e6f75 as ::core::ffi::c_uint,
    0x105ec76 as ::core::ffi::c_uint,
    0x12551f82 as ::core::ffi::c_uint,
    0xe03e9c81 as ::core::ffi::c_uint,
    0x34f4f86a as ::core::ffi::c_uint,
    0xc69f7b69 as ::core::ffi::c_uint,
    0xd5cf889d as ::core::ffi::c_uint,
    0x27a40b9e as ::core::ffi::c_uint,
    0x79b737ba as ::core::ffi::c_uint,
    0x8bdcb4b9 as ::core::ffi::c_uint,
    0x988c474d as ::core::ffi::c_uint,
    0x6ae7c44e as ::core::ffi::c_uint,
    0xbe2da0a5 as ::core::ffi::c_uint,
    0x4c4623a6 as ::core::ffi::c_uint,
    0x5f16d052 as ::core::ffi::c_uint,
    0xad7d5351 as ::core::ffi::c_uint,
];
#[no_mangle]
pub unsafe extern "C" fn toks_char_byte(mut cp: uint32_t) -> int32_t {
    if cp < 0x80 as uint32_t {
        if cp >= 0x21 as uint32_t && cp <= 0x7e as uint32_t {
            return cp as int32_t;
        }
        return -(1 as int32_t);
    }
    if cp >= 0xa1 as uint32_t && cp != 0xad as uint32_t && cp <= 0xff as uint32_t {
        return cp as int32_t;
    }
    if cp < 0x100 as uint32_t || cp > 0x143 as uint32_t {
        return -(1 as int32_t);
    }
    let mut idx: uint32_t = cp.wrapping_sub(0x100 as uint32_t);
    let mut seen: uint32_t = 0 as uint32_t;
    let mut i: uint32_t = 0 as uint32_t;
    while i < 256 as uint32_t {
        if !(i >= 0x21 as uint32_t && i <= 0x7e as uint32_t
            || i >= 0xa1 as uint32_t && i != 0xad as uint32_t)
        {
            if seen == idx {
                return i as int32_t;
            }
            seen = seen.wrapping_add(1);
        }
        i = i.wrapping_add(1);
    }
    return -(1 as int32_t);
}
static mut S256_K: [uint32_t; 64] = [
    0x428a2f98 as ::core::ffi::c_uint,
    0x71374491 as ::core::ffi::c_uint,
    0xb5c0fbcf as ::core::ffi::c_uint,
    0xe9b5dba5 as ::core::ffi::c_uint,
    0x3956c25b as ::core::ffi::c_uint,
    0x59f111f1 as ::core::ffi::c_uint,
    0x923f82a4 as ::core::ffi::c_uint,
    0xab1c5ed5 as ::core::ffi::c_uint,
    0xd807aa98 as ::core::ffi::c_uint,
    0x12835b01 as ::core::ffi::c_uint,
    0x243185be as ::core::ffi::c_uint,
    0x550c7dc3 as ::core::ffi::c_uint,
    0x72be5d74 as ::core::ffi::c_uint,
    0x80deb1fe as ::core::ffi::c_uint,
    0x9bdc06a7 as ::core::ffi::c_uint,
    0xc19bf174 as ::core::ffi::c_uint,
    0xe49b69c1 as ::core::ffi::c_uint,
    0xefbe4786 as ::core::ffi::c_uint,
    0xfc19dc6 as ::core::ffi::c_uint,
    0x240ca1cc as ::core::ffi::c_uint,
    0x2de92c6f as ::core::ffi::c_uint,
    0x4a7484aa as ::core::ffi::c_uint,
    0x5cb0a9dc as ::core::ffi::c_uint,
    0x76f988da as ::core::ffi::c_uint,
    0x983e5152 as ::core::ffi::c_uint,
    0xa831c66d as ::core::ffi::c_uint,
    0xb00327c8 as ::core::ffi::c_uint,
    0xbf597fc7 as ::core::ffi::c_uint,
    0xc6e00bf3 as ::core::ffi::c_uint,
    0xd5a79147 as ::core::ffi::c_uint,
    0x6ca6351 as ::core::ffi::c_uint,
    0x14292967 as ::core::ffi::c_uint,
    0x27b70a85 as ::core::ffi::c_uint,
    0x2e1b2138 as ::core::ffi::c_uint,
    0x4d2c6dfc as ::core::ffi::c_uint,
    0x53380d13 as ::core::ffi::c_uint,
    0x650a7354 as ::core::ffi::c_uint,
    0x766a0abb as ::core::ffi::c_uint,
    0x81c2c92e as ::core::ffi::c_uint,
    0x92722c85 as ::core::ffi::c_uint,
    0xa2bfe8a1 as ::core::ffi::c_uint,
    0xa81a664b as ::core::ffi::c_uint,
    0xc24b8b70 as ::core::ffi::c_uint,
    0xc76c51a3 as ::core::ffi::c_uint,
    0xd192e819 as ::core::ffi::c_uint,
    0xd6990624 as ::core::ffi::c_uint,
    0xf40e3585 as ::core::ffi::c_uint,
    0x106aa070 as ::core::ffi::c_uint,
    0x19a4c116 as ::core::ffi::c_uint,
    0x1e376c08 as ::core::ffi::c_uint,
    0x2748774c as ::core::ffi::c_uint,
    0x34b0bcb5 as ::core::ffi::c_uint,
    0x391c0cb3 as ::core::ffi::c_uint,
    0x4ed8aa4a as ::core::ffi::c_uint,
    0x5b9cca4f as ::core::ffi::c_uint,
    0x682e6ff3 as ::core::ffi::c_uint,
    0x748f82ee as ::core::ffi::c_uint,
    0x78a5636f as ::core::ffi::c_uint,
    0x84c87814 as ::core::ffi::c_uint,
    0x8cc70208 as ::core::ffi::c_uint,
    0x90befffa as ::core::ffi::c_uint,
    0xa4506ceb as ::core::ffi::c_uint,
    0xbef9a3f7 as ::core::ffi::c_uint,
    0xc67178f2 as ::core::ffi::c_uint,
];
unsafe extern "C" fn ror32(mut v: uint32_t, mut n: uint32_t) -> uint32_t {
    return v >> n | v << (32 as uint32_t).wrapping_sub(n);
}
unsafe extern "C" fn s256_block(mut s: *mut s256, mut w_in: *const uint32_t) {
    let mut w: [uint32_t; 64] = [0; 64];
    let mut i: uint32_t = 0 as uint32_t;
    while i < 16 as uint32_t {
        w[i as usize] = *w_in.offset(i as isize);
        i = i.wrapping_add(1);
    }
    let mut i_0: uint32_t = 16 as uint32_t;
    while i_0 < 64 as uint32_t {
        let mut a: uint32_t = ror32(
            w[i_0.wrapping_sub(15 as uint32_t) as usize],
            7 as uint32_t,
        ) ^ ror32(w[i_0.wrapping_sub(15 as uint32_t) as usize], 18 as uint32_t)
            ^ w[i_0.wrapping_sub(15 as uint32_t) as usize] >> 3 as ::core::ffi::c_int;
        let mut b: uint32_t = ror32(
            w[i_0.wrapping_sub(2 as uint32_t) as usize],
            17 as uint32_t,
        ) ^ ror32(w[i_0.wrapping_sub(2 as uint32_t) as usize], 19 as uint32_t)
            ^ w[i_0.wrapping_sub(2 as uint32_t) as usize] >> 10 as ::core::ffi::c_int;
        w[i_0 as usize] = w[i_0.wrapping_sub(16 as uint32_t) as usize]
            .wrapping_add(a)
            .wrapping_add(w[i_0.wrapping_sub(7 as uint32_t) as usize])
            .wrapping_add(b);
        i_0 = i_0.wrapping_add(1);
    }
    let mut h: [uint32_t; 8] = [0; 8];
    let mut i_1: uint32_t = 0 as uint32_t;
    while i_1 < 8 as uint32_t {
        h[i_1 as usize] = (*s).h[i_1 as usize];
        i_1 = i_1.wrapping_add(1);
    }
    let mut i_2: uint32_t = 0 as uint32_t;
    while i_2 < 64 as uint32_t {
        let mut t1: uint32_t = h[7 as ::core::ffi::c_int as usize]
            .wrapping_add(
                ror32(h[4 as ::core::ffi::c_int as usize], 6 as uint32_t)
                    ^ ror32(h[4 as ::core::ffi::c_int as usize], 11 as uint32_t)
                    ^ ror32(h[4 as ::core::ffi::c_int as usize], 25 as uint32_t),
            )
            .wrapping_add(
                h[4 as ::core::ffi::c_int as usize] & h[5 as ::core::ffi::c_int as usize]
                    ^ !h[4 as ::core::ffi::c_int as usize]
                        & h[6 as ::core::ffi::c_int as usize],
            )
            .wrapping_add(S256_K[i_2 as usize])
            .wrapping_add(w[i_2 as usize]);
        let mut t2: uint32_t = (ror32(h[0 as ::core::ffi::c_int as usize], 2 as uint32_t)
            ^ ror32(h[0 as ::core::ffi::c_int as usize], 13 as uint32_t)
            ^ ror32(h[0 as ::core::ffi::c_int as usize], 22 as uint32_t))
            .wrapping_add(
                h[0 as ::core::ffi::c_int as usize] & h[1 as ::core::ffi::c_int as usize]
                    ^ h[0 as ::core::ffi::c_int as usize]
                        & h[2 as ::core::ffi::c_int as usize]
                    ^ h[1 as ::core::ffi::c_int as usize]
                        & h[2 as ::core::ffi::c_int as usize],
            );
        h[7 as ::core::ffi::c_int as usize] = h[6 as ::core::ffi::c_int as usize];
        h[6 as ::core::ffi::c_int as usize] = h[5 as ::core::ffi::c_int as usize];
        h[5 as ::core::ffi::c_int as usize] = h[4 as ::core::ffi::c_int as usize];
        h[4 as ::core::ffi::c_int as usize] = h[3 as ::core::ffi::c_int as usize]
            .wrapping_add(t1);
        h[3 as ::core::ffi::c_int as usize] = h[2 as ::core::ffi::c_int as usize];
        h[2 as ::core::ffi::c_int as usize] = h[1 as ::core::ffi::c_int as usize];
        h[1 as ::core::ffi::c_int as usize] = h[0 as ::core::ffi::c_int as usize];
        h[0 as ::core::ffi::c_int as usize] = t1.wrapping_add(t2);
        i_2 = i_2.wrapping_add(1);
    }
    let mut i_3: uint32_t = 0 as uint32_t;
    while i_3 < 8 as uint32_t {
        (*s).h[i_3 as usize] = (*s).h[i_3 as usize].wrapping_add(h[i_3 as usize]);
        i_3 = i_3.wrapping_add(1);
    }
}
unsafe extern "C" fn s256_write(
    mut s: *mut s256,
    mut p: *const uint8_t,
    mut n: uint64_t,
) {
    (*s).len = (*s).len.wrapping_add(n);
    let mut i: uint64_t = 0 as uint64_t;
    while i < n {
        let mut nb: uint32_t = (*s).n & 63 as uint32_t;
        let mut w: uint32_t = nb >> 2 as ::core::ffi::c_int;
        let mut b: uint32_t = nb & 3 as uint32_t;
        if b == 0 as uint32_t && i.wrapping_add(4 as uint64_t) <= n {
            (*s).buf[w as usize] = (*p.offset(i as isize) as uint32_t)
                << 24 as ::core::ffi::c_int
                | (*p.offset(i.wrapping_add(1 as uint64_t) as isize) as uint32_t)
                    << 16 as ::core::ffi::c_int
                | (*p.offset(i.wrapping_add(2 as uint64_t) as isize) as uint32_t)
                    << 8 as ::core::ffi::c_int
                | *p.offset(i.wrapping_add(3 as uint64_t) as isize) as uint32_t;
            i = i.wrapping_add(4 as uint64_t);
            (*s).n = ((*s).n as ::core::ffi::c_uint)
                .wrapping_add(4 as ::core::ffi::c_uint) as uint32_t as uint32_t;
        } else {
            if b == 0 as uint32_t {
                (*s).buf[w as usize] = 0 as ::core::ffi::c_uint as uint32_t;
            }
            (*s).buf[w as usize]
                |= (*p.offset(i as isize) as uint32_t)
                    << (24 as uint32_t).wrapping_sub((8 as uint32_t).wrapping_mul(b));
            i = i.wrapping_add(1);
            (*s).n = (*s).n.wrapping_add(1);
        }
        if (*s).n & 63 as uint32_t == 0 as uint32_t && (*s).n != 0 as uint32_t {
            s256_block(s, &raw mut (*s).buf as *mut uint32_t as *const uint32_t);
            (*s).n = 0 as ::core::ffi::c_uint as uint32_t;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn toks_sha256(
    mut data: *const uint8_t,
    mut len: uint64_t,
    mut out: *mut uint8_t,
) {
    let mut s: s256 = s256 {
        h: [0; 8],
        len: 0,
        buf: [0; 16],
        n: 0,
    };
    s.h[0 as ::core::ffi::c_int as usize] = 0x6a09e667 as ::core::ffi::c_uint
        as uint32_t;
    s.h[1 as ::core::ffi::c_int as usize] = 0xbb67ae85 as ::core::ffi::c_uint
        as uint32_t;
    s.h[2 as ::core::ffi::c_int as usize] = 0x3c6ef372 as ::core::ffi::c_uint
        as uint32_t;
    s.h[3 as ::core::ffi::c_int as usize] = 0xa54ff53a as ::core::ffi::c_uint
        as uint32_t;
    s.h[4 as ::core::ffi::c_int as usize] = 0x510e527f as ::core::ffi::c_uint
        as uint32_t;
    s.h[5 as ::core::ffi::c_int as usize] = 0x9b05688c as ::core::ffi::c_uint
        as uint32_t;
    s.h[6 as ::core::ffi::c_int as usize] = 0x1f83d9ab as ::core::ffi::c_uint
        as uint32_t;
    s.h[7 as ::core::ffi::c_int as usize] = 0x5be0cd19 as ::core::ffi::c_uint
        as uint32_t;
    s.len = 0 as uint64_t;
    s.n = 0 as uint32_t;
    memset(
        &raw mut s.buf as *mut uint32_t as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<[uint32_t; 16]>() as size_t,
    );
    s256_write(&raw mut s, data, len);
    let mut bits: uint64_t = s.len.wrapping_mul(8 as uint64_t);
    let mut pad: uint8_t = 0x80 as uint8_t;
    s256_write(&raw mut s, &raw mut pad, 1 as uint64_t);
    while s.n & 63 as uint32_t != 56 as uint32_t {
        pad = 0 as uint8_t;
        s256_write(&raw mut s, &raw mut pad, 1 as uint64_t);
    }
    let mut lenb: [uint8_t; 8] = [0; 8];
    let mut i: uint32_t = 0 as uint32_t;
    while i < 8 as uint32_t {
        lenb[i as usize] = (bits
            >> (56 as uint32_t).wrapping_sub((8 as uint32_t).wrapping_mul(i)))
            as uint8_t;
        i = i.wrapping_add(1);
    }
    s256_write(&raw mut s, &raw mut lenb as *mut uint8_t, 8 as uint64_t);
    let mut i_0: uint32_t = 0 as uint32_t;
    while i_0 < 8 as uint32_t {
        *out.offset(i_0.wrapping_mul(4 as uint32_t) as isize) = (s.h[i_0 as usize]
            >> 24 as ::core::ffi::c_int) as uint8_t;
        *out
            .offset(
                i_0.wrapping_mul(4 as uint32_t).wrapping_add(1 as uint32_t) as isize,
            ) = (s.h[i_0 as usize] >> 16 as ::core::ffi::c_int) as uint8_t;
        *out
            .offset(
                i_0.wrapping_mul(4 as uint32_t).wrapping_add(2 as uint32_t) as isize,
            ) = (s.h[i_0 as usize] >> 8 as ::core::ffi::c_int) as uint8_t;
        *out
            .offset(
                i_0.wrapping_mul(4 as uint32_t).wrapping_add(3 as uint32_t) as isize,
            ) = s.h[i_0 as usize] as uint8_t;
        i_0 = i_0.wrapping_add(1);
    }
}
