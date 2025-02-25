use std::{borrow::Cow, ffi::CStr, marker::PhantomData};

use sdl3_sys;

/// Cannot be passed between threads
pub struct ErrorBuffer(PhantomData<*mut ()>);

/// Returns [`None`] if called multiple times from the same thread
pub fn sdl_thread() -> Option<ErrorBuffer> {
    use std::cell::RefCell;
    thread_local! { static IS_NEW_THREAD: RefCell<bool> = const{RefCell::new(true)}; }
    IS_NEW_THREAD.with(|is_new_thread| {
        let mut borrow = is_new_thread.borrow_mut();
        let old_value = *borrow;
        *borrow = false;
        old_value.then_some(ErrorBuffer(PhantomData))
    })
}

pub struct SdlError<'a> {
    msg: Cow<'a, str>,
    /// Forbid storing lazy error that lives long enough for the error buffer to be overwritten
    _unique: PhantomData<&'a mut ErrorBuffer>,
}

impl<'a> SdlError<'a> {
    pub fn get(_: &'a mut ErrorBuffer) -> Self {
        Self {
            msg: unsafe { CStr::from_ptr(sdl3_sys::error::SDL_GetError()) }.to_string_lossy(),
            _unique: PhantomData,
        }
    }
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

pub struct SdlHandle(());

pub fn init(err_buf: &mut ErrorBuffer) -> Result<SdlHandle, SdlError<'_>> {
    if unsafe { sdl3_sys::init::SDL_Init(0) } {
        Ok(SdlHandle(()))
    } else {
        Err(SdlError::get(err_buf))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test0() {
        let mut err_buf = sdl_thread().unwrap();
        let err1 = SdlError::get(&mut err_buf);
        println!("error: {err1}");
        let err2 = SdlError::get(&mut err_buf);
        println!("error: {err2}");
        // err1;
    }
}
