use std::{cell::OnceCell, error::Error, ffi::CStr, fmt, marker::PhantomData, num::TryFromIntError};
use sdl3_sys::error::*;

/// A handle for ensuring the SDL error buffer doesn't get overwritten before this error is consumed.
///
/// Due to Rust's "no two mutable references" rule, the existence of an [`SdlError`] referencing this
/// will block the use of any SDL functions capable of erroring until that error provably goes into
/// disuse or gets upgraded to an [`OwnedSdlError`].
///
/// An [`ErrorBuffer`] is not safe to be passed between threads, because SDL errors are buffered
/// statically per-thread.
pub struct ErrorBuffer(());

impl !Send for ErrorBuffer {}
impl !Sync for ErrorBuffer {}

/// Get an [`ErrorBuffer`] local to the current thread.
///
/// Returns [`None`] if called multiple times from the same thread.
pub fn err_buf() -> Option<ErrorBuffer> {
    use std::cell::RefCell;
    thread_local! { static IS_NEW_THREAD: RefCell<bool> = const{RefCell::new(true)}; }
    IS_NEW_THREAD.with(|is_new_thread| {
        let mut borrow = is_new_thread.borrow_mut();
        let old_value = *borrow;
        *borrow = false;
        old_value.then_some(ErrorBuffer(()))
    })
}

impl ErrorBuffer {
    /// Retrieve a message about the last error that occurred on the current
    /// thread.
    ///
    /// It is possible for multiple errors to occur before calling [`ErrorBuffer::get()`].
    /// Only the last error is returned.
    ///
    /// The message is only applicable when an SDL function has signaled an error.
    /// You must check the return values of SDL function calls to determine when to
    /// appropriately call [`ErrorBuffer::get()`]. You should *not* use the results of
    /// [`ErrorBuffer::get()`] to decide if an error has occurred! Sometimes SDL will set
    /// an error string even when reporting success.
    ///
    /// SDL will *not* clear the error string for successful API calls. You *must*
    /// check return values for failure cases before you can assume the error
    /// string applies.
    ///
    /// Error strings are set per-thread, so an error set in a different thread
    /// will not interfere with the current thread's operation.
    ///
    /// The returned value is a thread-local string which will remain valid until
    /// the current thread's error string is changed. The caller should make a copy
    /// if the value is needed after the next SDL API call.
    ///
    /// ### Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ### See also
    /// - [`ErrorBuffer::clear`]
    /// - [`ErrorBuffer::set`]
    pub fn get(&mut self) -> SdlError<'_> {
        SdlError {
            msg: OnceCell::new(),
            _unique: PhantomData,
        }
    }

    /// Clear any previous error message for this thread.
    ///
    /// Always returns true.
    ///
    /// ### Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ### See also
    /// - [`ErrorBuffer::get`]
    /// - [`ErrorBuffer::set`]
    pub fn clear(&mut self) -> bool {
        unsafe { SDL_ClearError() }
    }

    /// Set the SDL error message for the current thread.
    ///
    /// Calling this function will replace any previous error message that was set.
    ///
    /// This function always returns false, since SDL frequently uses false to
    /// signify a failing result, leading to this idiom:
    ///
    /// ```no_run
    /// if error_code {
    ///     return err_buf.set(c"This operation has failed: {}", error_code);
    /// }
    /// ```
    ///
    /// ### Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ### See also
    /// - [`ErrorBuffer::clear`]
    /// - [`ErrorBuffer::get`]
    /// - [`ErrorBuffer::set_v`]
    pub fn set(&mut self, msg: &CStr) -> bool {
        unsafe { SDL_SetError(msg.as_ptr()) }
    }

    /// Set the SDL error message for the current thread.
    ///
    /// Calling this function will replace any previous error message that was set.
    ///
    /// Always returns false.
    ///
    /// ### Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ### See also
    /// - [`ErrorBuffer::clear`]
    /// - [`ErrorBuffer::get`]
    /// - [`ErrorBuffer::set`]
    #[cfg(feature = "c_variadic")]
    pub fn set_v(&mut self, msg: &CStr, ap: sdl3_sys::ffi::VaList) -> bool {
        unsafe { SDL_SetErrorV(msg.as_ptr(), ap) }
    }
}

pub trait ToSdlError {
    type Success;
    /// Transform [`Some`] into [`Ok`] and [`None`] into [`SdlError`]
    fn sdl_err(self, err_buf: &mut ErrorBuffer) -> Result<Self::Success, SdlError<'_>>;
}

impl ToSdlError for bool {
    type Success = ();

    #[inline]
    fn sdl_err(self, err_buf: &mut ErrorBuffer) -> Result<(), SdlError<'_>> {
        if self { Ok(()) } else { Err(err_buf.get()) }
    }
}

impl<T> ToSdlError for Option<T> {
    type Success = T;

    #[inline]
    fn sdl_err(self, err_buf: &mut ErrorBuffer) -> Result<Self::Success, SdlError<'_>> {
        if let Some(succ) = self { Ok(succ) } else { Err(err_buf.get()) }
    }
}

pub trait ToSdlOrIntError {
    type Success;
    /// Transform [`Some`] into [`Ok`] and [`None`] into [`SdlOrIntError`]
    fn sdl_erri(self, err_buf: &mut ErrorBuffer) -> Result<Self::Success, SdlOrIntError<'_>>;
}

impl<T: ToSdlError> ToSdlOrIntError for T {
    type Success = <T as ToSdlError>::Success;

    #[inline]
    fn sdl_erri(self, err_buf: &mut ErrorBuffer) -> Result<Self::Success, SdlOrIntError<'_>> {
        self.sdl_err(err_buf).map_err(|e| e.into())
    }
}

pub struct SdlError<'a> {
    msg: OnceCell<&'a CStr>,
    /// Forbid storing lazy error that lives long enough for the error buffer to be overwritten
    _unique: PhantomData<&'a mut ErrorBuffer>,
}

impl<'a> SdlError<'a> {
    #[inline]
    pub fn msg(&self) -> &'a CStr {
        // Safety: SDL_GetError will never return a nullptr. At worst it will return an empty string.
        self.msg.get_or_init(|| unsafe { CStr::from_ptr(SDL_GetError()) })
    }
}

impl fmt::Debug for SdlError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SdlError")
            .field("msg", &self.msg())
            .finish()
    }
}

impl fmt::Display for SdlError<'_> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.msg().to_string_lossy().fmt(f)
    }
}

impl Error for SdlError<'_> {}

impl SdlError<'_> {
    #[inline]
    pub fn to_owned(self) -> OwnedSdlError {
        OwnedSdlError {
            msg: self.msg().to_string_lossy().to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct OwnedSdlError {
    pub msg: String,
}

impl fmt::Display for OwnedSdlError {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.msg.fmt(f)
    }
}

impl Error for OwnedSdlError {}

#[derive(Debug)]
pub enum SdlOrIntError<'a> {
    TryFromIntError(TryFromIntError),
    OtherIntError,
    SdlError(SdlError<'a>),
}

impl fmt::Display for SdlOrIntError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TryFromIntError(e) => write!(f, "integer conversion error: {e}"),
            Self::OtherIntError => write!(f, "other integer conversion error"),
            Self::SdlError(e) => write!(f, "SDL error: {e}"),
        }
    }
}

impl<'a> Error for SdlOrIntError<'a> {}

impl From<TryFromIntError> for SdlOrIntError<'_> {
    #[inline]
    fn from(value: TryFromIntError) -> Self {
        Self::TryFromIntError(value)
    }
}

impl<'a> From<SdlError<'a>> for SdlOrIntError<'a> {
    #[inline]
    fn from(value: SdlError<'a>) -> Self {
        Self::SdlError(value)
    }
}

/// An error which occurs when an operation times out without completing
#[derive(Debug)]
pub struct TimeoutError<'a>(pub SdlError<'a>);

impl<'a> fmt::Display for TimeoutError<'a> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "the operation timed out: {}", self.0)
    }
}

impl<'a> Error for TimeoutError<'a> {}

impl<'a> From<TimeoutError<'a>> for SdlError<'a> {
    #[inline]
    fn from(value: TimeoutError<'a>) -> Self {
        value.0
    }
}

#[derive(Debug)]
pub enum PlatformUnimplemented {
    FilesystemBasePath,
}

impl fmt::Display for PlatformUnimplemented {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::FilesystemBasePath => write!(f, "filesystem base path"),
        }
    }
}

/// An error which occurs when the current platform lacks support for the functionality
#[derive(Debug)]
pub struct PlatformUnimplFeatureError<'a>{
    pub err: SdlError<'a>,
    pub feature: PlatformUnimplemented,
}

impl<'a> fmt::Display for PlatformUnimplFeatureError<'a> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "the platform does not implement {} functionality: {}", self.feature, self.err)
    }
}

impl<'a> Error for PlatformUnimplFeatureError<'a> {}

impl<'a> PlatformUnimplFeatureError<'a> {
    pub fn new(err: SdlError<'a>, feature: PlatformUnimplemented) -> Self {
        Self { err, feature }
    }

    pub fn filesystem_base_path(err: SdlError<'a>) -> Self {
        Self { err, feature: PlatformUnimplemented::FilesystemBasePath }
    }
}
