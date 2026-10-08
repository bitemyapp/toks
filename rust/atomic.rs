//! Atomics used by the pool. Pointer storage has C layout; every concurrent
//! access uses these atomic operations, retaining the original memory order.
use std::sync::atomic::{AtomicU32, AtomicU64, Ordering};
pub(crate) trait Word: Copy {
    unsafe fn load(p: *const Self, order: Ordering) -> Self;
    unsafe fn store(p: *mut Self, value: Self, order: Ordering);
    unsafe fn add(p: *mut Self, value: Self, order: Ordering) -> Self;
    unsafe fn sub(p: *mut Self, value: Self, order: Ordering) -> Self;
    unsafe fn cas(p: *mut Self, old: Self, new: Self) -> (Self, bool);
}
macro_rules! word {
    ($t:ty, $a:ty) => {
        impl Word for $t {
            unsafe fn load(p: *const Self, o: Ordering) -> Self { <$a>::from_ptr(p.cast_mut()).load(o) }
            unsafe fn store(p: *mut Self, v: Self, o: Ordering) { <$a>::from_ptr(p).store(v, o) }
            unsafe fn add(p: *mut Self, v: Self, o: Ordering) -> Self { <$a>::from_ptr(p).fetch_add(v, o) }
            unsafe fn sub(p: *mut Self, v: Self, o: Ordering) -> Self { <$a>::from_ptr(p).fetch_sub(v, o) }
            unsafe fn cas(p: *mut Self, a: Self, b: Self) -> (Self, bool) {
                match <$a>::from_ptr(p).compare_exchange_weak(a, b, Ordering::SeqCst, Ordering::SeqCst) {
                    Ok(v) => (v, true), Err(v) => (v, false),
                }
            }
        }
    }
}
word!(u32, AtomicU32);
word!(u64, AtomicU64);
macro_rules! loads {
    ($($name:ident: $o:ident),*) => { $(
        #[inline] pub(crate) unsafe fn $name<T: Word>(p: *const T) -> T { T::load(p, Ordering::$o) }
    )* }
}
macro_rules! stores {
    ($($name:ident: $o:ident),*) => { $(
        #[inline] pub(crate) unsafe fn $name<T: Word>(p: *mut T, v: T) { T::store(p, v, Ordering::$o) }
    )* }
}
macro_rules! adds {
    ($($name:ident: $o:ident),*) => { $(
        #[inline] pub(crate) unsafe fn $name<T: Word>(p: *mut T, v: T) -> T { T::add(p, v, Ordering::$o) }
    )* }
}
loads!(atomic_load_acquire: Acquire, atomic_load_relaxed: Relaxed, atomic_load_seqcst: SeqCst);
stores!(atomic_store_release: Release, atomic_store_relaxed: Relaxed, atomic_store_seqcst: SeqCst);
adds!(atomic_xadd_release: Release, atomic_xadd_relaxed: Relaxed, atomic_xadd_seqcst: SeqCst);
#[inline] pub(crate) unsafe fn atomic_xsub_seqcst<T: Word>(p: *mut T, v: T) -> T { T::sub(p, v, Ordering::SeqCst) }
#[inline] pub(crate) unsafe fn atomic_cxchgweak_seqcst_seqcst<T: Word>(p: *mut T, a: T, b: T) -> (T, bool) { T::cas(p, a, b) }
