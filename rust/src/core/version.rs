pub const TOKS_VERSION: [::core::ffi::c_char; 6] = unsafe {
    ::core::mem::transmute::<[u8; 6], [::core::ffi::c_char; 6]>(*b"0.3.2\0")
};
#[no_mangle]
pub unsafe extern "C" fn toks_version() -> *const ::core::ffi::c_char {
    return TOKS_VERSION.as_ptr();
}
