use std::{borrow::Cow, ffi::CStr, marker::PhantomData};

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
pub fn sdl_thread() -> Option<ErrorBuffer> {
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
            msg: unsafe { CStr::from_ptr(sdl3_sys::error::SDL_GetError()) }.to_string_lossy(),
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
        unsafe { sdl3_sys::error::SDL_ClearError() }
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
        unsafe { sdl3_sys::error::SDL_SetError(msg.as_ptr()) }
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
        unsafe { sdl3_sys::error::SDL_SetErrorV(msg.as_ptr(), ap) }
    }
}

pub struct SdlError<'a> {
    msg: Cow<'a, str>,
    /// Forbid storing lazy error that lives long enough for the error buffer to be overwritten
    _unique: PhantomData<&'a mut ErrorBuffer>,
}

impl std::fmt::Debug for SdlError<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SdlError")
            .field("msg", &self.msg)
            .finish()
    }
}

impl std::fmt::Display for SdlError<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.msg.fmt(f)
    }
}

impl std::error::Error for SdlError<'_> {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

impl SdlError<'_> {
    pub fn to_owned(self) -> OwnedSdlError {
        OwnedSdlError {
            msg: self.msg.to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct OwnedSdlError {
    pub msg: String,
}

impl std::fmt::Display for OwnedSdlError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.msg.fmt(f)
    }
}

impl std::error::Error for OwnedSdlError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

#[derive(Debug)]
pub enum SdlOrIntError<'a> {
    TryFromIntError(std::num::TryFromIntError),
    OtherIntError,
    SdlError(SdlError<'a>),
}

impl std::fmt::Display for SdlOrIntError<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TryFromIntError(e) => write!(f, "integer conversion error: {e}"),
            Self::OtherIntError => write!(f, "other integer conversion error"),
            Self::SdlError(e) => write!(f, "SDL error: {e}"),
        }
    }
}

impl std::error::Error for SdlOrIntError<'static> {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::TryFromIntError(e) => Some(e),
            Self::OtherIntError => None,
            Self::SdlError(e) => Some(e),
        }
    }
}

impl From<std::num::TryFromIntError> for SdlOrIntError<'_> {
    fn from(value: std::num::TryFromIntError) -> Self {
        Self::TryFromIntError(value)
    }
}

impl<'a> From<SdlError<'a>> for SdlOrIntError<'a> {
    fn from(value: SdlError<'a>) -> Self {
        Self::SdlError(value)
    }
}
