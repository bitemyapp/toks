//! Test-only placement hooks implemented by the upstream protected-page harness.
//! Keep these out of production builds; opaque pointers accept each translated
//! module's ABI-identical context and scratch types without forming references.
use libc::c_void;

extern "C" {
    pub fn toks_guard_tab(
        owner: *const c_void,
        at: *const c_void,
        n: u64,
        align: u64,
    ) -> *mut c_void;
    pub fn toks_guard_fit(p: *const c_void, n: u64, align: u64) -> *const c_void;
    pub fn toks_guard_owner(p: *const c_void) -> *mut u8;
    pub fn toks_guard_seal(block: *mut c_void, n: u64);
    pub fn toks_guard_block(block: *const c_void, n: u64);
    pub fn toks_guard_release(block: *const c_void, n: u64);
    pub fn toks_guard_scr(ctx: *const c_void, h: *mut c_void);
    pub fn toks_guard_scr_at(h: *const c_void, off: u64) -> *mut u8;
    pub fn toks_guard_scr_zero(h: *const c_void, off: u64, n: u64);
}
