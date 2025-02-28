use std::ffi::{c_char, CStr};

pub mod null_term_sdl_parray;
pub mod sdl_slice;

/// Wraps a raw C string with a safe C string wrapper, returning [`None`] if it is null.
///
/// This function will wrap the provided `ptr` with a `CStr` wrapper, which
/// allows inspection and interoperation of non-owned C strings. The total
/// size of the terminated buffer must be smaller than [`isize::MAX`] **bytes**
/// in memory (a restriction from [`std::slice::from_raw_parts`]).
///
/// # Safety
///
/// * The memory pointed to by `ptr` must contain a valid nul terminator at the
///   end of the string.
///
/// * `ptr` must be [valid] for reads of bytes up to and including the nul terminator.
///   This means in particular:
///
///     * The entire memory range of this `CStr` must be contained within a single allocated object!
///     * `ptr` must be non-null even for a zero-length cstr.
///
/// * The memory referenced by the returned `CStr` must not be mutated for
///   the duration of lifetime `'a`.
///
/// * The nul terminator must be within `isize::MAX` from `ptr`
///
/// > **Note**: This operation is intended to be a 0-cost cast but it is
/// > currently implemented with an up-front calculation of the length of
/// > the string. This is not guaranteed to always be the case.
///
/// # Caveat
///
/// The lifetime for the returned slice is inferred from its usage. To prevent accidental misuse,
/// it's suggested to tie the lifetime to whichever source lifetime is safe in the context,
/// such as by providing a helper function taking the lifetime of a host value for the slice,
/// or by explicit annotation.
///
/// # Examples
///
/// ```
/// use std::ffi::{c_char, CStr};
///
/// fn my_string() -> *const c_char {
///     c"hello".as_ptr()
/// }
///
/// unsafe {
///     let slice = cstr_from(my_string()).unwrap();
///     assert_eq!(slice.to_str().unwrap(), "hello");
/// }
/// ```
///
/// ```
/// use std::ffi::{c_char, CStr};
///
/// const HELLO_PTR: *const c_char = {
///     const BYTES: &[u8] = b"Hello, world!\0";
///     BYTES.as_ptr().cast()
/// };
/// const HELLO: Option<&CStr> = unsafe { cstr_from(HELLO_PTR) };
///
/// assert_eq!(Some(c"Hello, world!"), HELLO);
/// ```
///
/// [valid]: core::ptr#safety
pub const fn cstr_from<'a>(cstr: *const c_char) -> Option<&'a CStr> {
    if cstr.is_null() {
        None
    } else {
        Some(unsafe { CStr::from_ptr::<'a>(cstr) })
    }
}
