//! Operating-system boundary. The tokenizer core makes no allocations after load.
use libc::{c_char, c_int, c_void};
use std::ptr::{copy_nonoverlapping, null_mut};

#[cfg(not(feature = "test-allocator"))]
#[no_mangle]
pub unsafe extern "C" fn toks_plat_alloc(n: u64) -> *mut c_void {
    if n == 0 || n > usize::MAX as u64 { null_mut() } else { libc::malloc(n as usize) }
}
#[cfg(not(feature = "test-allocator"))]
#[no_mangle]
pub unsafe extern "C" fn toks_plat_free(p: *mut c_void, _: u64) { libc::free(p); }
unsafe fn page_bytes() -> usize {
    let n = libc::sysconf(libc::_SC_PAGESIZE);
    if n > 0 { n as usize } else { 4096 }
}
#[cfg(not(feature = "test-allocator"))]
#[no_mangle]
pub unsafe extern "C" fn toks_plat_arena(n: u64) -> *mut u8 {
    let pg = page_bytes();
    let hp = pg.max(2 << 20);
    if n == 0 || n > (usize::MAX - 2 * hp) as u64 { return null_mut(); }
    let z = (n as usize + pg - 1) & !(pg - 1);
    let total = z + hp;
    let m = libc::mmap(null_mut(), total, libc::PROT_READ | libc::PROT_WRITE,
        libc::MAP_PRIVATE | libc::MAP_ANONYMOUS, -1, 0);
    if m == libc::MAP_FAILED { return null_mut(); }
    let base = m.cast::<u8>();
    let head = (hp - (base as usize & (hp - 1))) & (hp - 1);
    let p = base.add(head);
    if head != 0 && libc::munmap(m, head) != 0 { libc::munmap(m, total); return null_mut(); }
    if libc::munmap(p.add(z).cast(), hp - head) != 0 {
        libc::munmap(p.cast(), z + hp - head); return null_mut();
    }
    #[cfg(target_os = "linux")]
    { libc::madvise(p.cast(), z, libc::MADV_HUGEPAGE); p.write_volatile(0); }
    #[cfg(feature = "test-guard")]
    crate::guard::toks_guard_block(p.cast(), n);
    p
}
#[cfg(not(feature = "test-allocator"))]
#[no_mangle]
pub unsafe extern "C" fn toks_plat_arena_free(p: *mut u8, n: u64) {
    if p.is_null() { return; }
    #[cfg(feature = "test-guard")]
    crate::guard::toks_guard_release(p.cast(), n);
    let pg = page_bytes(); libc::munmap(p.cast(), (n as usize + pg - 1) & !(pg - 1));
}
#[cfg(not(feature = "test-allocator"))]
#[no_mangle]
pub unsafe extern "C" fn toks_plat_hint_huge(p: *mut c_void, n: u64) {
    #[cfg(target_os = "linux")]
    {
        let a = (p as usize + 0x1fffff) & !0x1fffff;
        let b = (p as usize + n as usize) & !0x1fffff;
        if b > a { libc::madvise(a as *mut c_void, b - a, libc::MADV_HUGEPAGE); }
    }
    let _ = (p, n);
}
#[no_mangle]
pub unsafe extern "C" fn toks_plat_getenv(name: *const c_char, buf: *mut c_char, cap: u64) -> i64 {
    if cap == 0 { return -2; }
    buf.write(0);
    let v = libc::getenv(name);
    if v.is_null() || v.read() == 0 { return -1; }
    let mut n = 0;
    while v.add(n).read() != 0 {
        if n as u64 + 1 >= cap { buf.write(0); return -2; }
        buf.add(n).write(v.add(n).read());
        n += 1;
    }
    buf.add(n).write(0);
    n as i64
}
unsafe fn path_is_dir(path: *const c_char) -> bool {
    let fd = libc::open(path, libc::O_RDONLY | libc::O_DIRECTORY);
    if fd < 0 { false } else { libc::close(fd); true }
}
unsafe fn file_size(fd: c_int) -> u64 {
    let n = libc::lseek(fd, 0, libc::SEEK_END);
    if n <= 0 || libc::lseek(fd, 0, libc::SEEK_SET) != 0 { 0 } else { n as u64 }
}
#[no_mangle]
pub unsafe extern "C" fn toks_plat_read_file(path: *const c_char, out: *mut *mut u8, len: *mut u64, is_dir: *mut c_int) -> i64 {
    out.write(null_mut()); len.write(0); is_dir.write(0);
    if path_is_dir(path) { is_dir.write(1); return -1; }
    let fd = libc::open(path, libc::O_RDONLY | libc::O_NONBLOCK);
    if fd < 0 { return -1; }
    let n = file_size(fd);
    if n == 0 || n > 256 << 20 { libc::close(fd); return if n == 0 { -1 } else { -9 }; }
    let buf = toks_plat_alloc(n).cast::<u8>();
    if buf.is_null() { libc::close(fd); return -11; }
    let mut off = 0;
    while off < n as usize {
        let r = libc::read(fd, buf.add(off).cast(), n as usize - off);
        if r <= 0 { libc::close(fd); toks_plat_free(buf.cast(), n); return -1; }
        off += r as usize;
    }
    libc::close(fd); out.write(buf); len.write(n); 0
}
#[no_mangle]
pub unsafe extern "C" fn toks_plat_dir_lookup(dir: *const c_char, buf: *mut c_char, cap: u64) -> i64 {
    let mut n = 0;
    while n < cap as usize && dir.add(n).read() != 0 { n += 1; }
    if n == 0 || n == cap as usize { return -1; }
    let last = dir.add(n - 1).read() as u8;
    let sep = usize::from(last != b'/' && last != b'\\');
    let leaf = b"tokenizer.json\0";
    if n + sep + leaf.len() > cap as usize { return -1; }
    copy_nonoverlapping(dir, buf, n);
    if sep != 0 { buf.add(n).write(b'/' as c_char); }
    copy_nonoverlapping(leaf.as_ptr().cast(), buf.add(n + sep), leaf.len());
    if path_is_dir(buf) { return -1; }
    let fd = libc::open(buf, libc::O_RDONLY | libc::O_NONBLOCK);
    if fd < 0 { return -1; }
    let n = file_size(fd); libc::close(fd);
    if n != 0 { 0 } else { -1 }
}
#[cfg(not(feature = "test-api"))]
#[no_mangle]
pub extern "C" fn toks_cpu_features() -> u64 {
    let mut f = 0;
    #[cfg(target_arch = "x86_64")]
    {
        macro_rules! feat { ($name:tt, $bit:expr) => { if std::is_x86_feature_detected!($name) { f |= 1 << $bit; } }; }
        feat!("sse4.2", 0); feat!("popcnt", 1); feat!("avx2", 2);
        feat!("bmi1", 3); feat!("bmi2", 4); feat!("lzcnt", 5);
        feat!("avx512f", 6); feat!("avx512bw", 7); feat!("avx512vl", 8);
        feat!("avx512vbmi", 9); feat!("avx512vbmi2", 10);
    }
    #[cfg(target_arch = "aarch64")]
    {
        macro_rules! feat { ($name:tt, $bit:expr) => { if std::arch::is_aarch64_feature_detected!($name) { f |= 1 << $bit; } }; }
        feat!("neon", 16); feat!("crc", 17); feat!("dotprod", 18); feat!("lse", 19); feat!("sve", 20);
    }
    f
}

#[cfg(feature = "test-allocator")]
extern "C" {
    fn toks_plat_alloc(n: u64) -> *mut c_void;
    fn toks_plat_free(p: *mut c_void, n: u64);
    pub fn toks_plat_arena(n: u64) -> *mut u8;
    pub fn toks_plat_arena_free(p: *mut u8, n: u64);
}
