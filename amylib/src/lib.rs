pub mod collections;
pub mod iter;
pub mod rc;

/// Returns uninitialized memory
pub const unsafe fn uninit<T>() -> T {
    std::mem::MaybeUninit::uninit().assume_init()
}
