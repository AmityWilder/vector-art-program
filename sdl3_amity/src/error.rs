use std::{borrow::Cow, ffi::CStr, marker::PhantomData};

pub struct ErrorBuffer(());

impl !Send for ErrorBuffer {}
impl !Sync for ErrorBuffer {}

/// Returns [`None`] if called multiple times from the same thread
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
    pub fn get(&mut self) -> SdlError<'_> {
        SdlError {
            msg: unsafe { CStr::from_ptr(sdl3_sys::error::SDL_GetError()) }.to_string_lossy(),
            _unique: PhantomData,
        }
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
