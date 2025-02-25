#![feature(negative_impls)]
use std::ffi::CStr;
use error::*;
use sdl3_sys::{self, init::{SDL_Init, SDL_InitFlags, SDL_Quit}, video::{SDL_CreateWindow, SDL_DestroyWindow, SDL_Window, SDL_WindowFlags}};

pub mod error;

pub struct SdlHandle(());

impl Drop for SdlHandle {
    fn drop(&mut self) {
        unsafe { SDL_Quit(); }
    }
}

pub fn init(err_buf: &mut ErrorBuffer, flags: SDL_InitFlags) -> Result<SdlHandle, SdlError<'_>> {
    if unsafe { SDL_Init(flags) } {
        Ok(SdlHandle(()))
    } else {
        Err(err_buf.get())
    }
}

impl SdlHandle {
    pub fn create_window<'a, 'b>(
        &'a self, err_buf:
        &'b mut ErrorBuffer,
        title: &CStr,
        w: u32,
        h: u32,
        flags: SDL_WindowFlags,
    ) -> Result<SdlWindow<'a>, SdlOrIntError<'b>> {
        let window = unsafe {
            SDL_CreateWindow(
                title.as_ptr(),
                w.try_into()?,
                h.try_into()?,
                flags,
            ).as_mut()
        };
        if let Some(window) = window {
            Ok(SdlWindow(window))
        } else {
            Err(err_buf.get().into())
        }
    }
}

pub struct SdlWindow<'a>(&'a mut SDL_Window);

impl !Send for SdlWindow<'_> {}
impl !Sync for SdlWindow<'_> {}

impl Drop for SdlWindow<'_> {
    fn drop(&mut self) {
        unsafe { SDL_DestroyWindow(self.0); }
    }
}

#[cfg(test)]
mod test {
    use std::time::Duration;
    use super::*;

    #[test]
    fn test0() {
        let mut err_buf = sdl_thread().unwrap();
        let err1 = err_buf.get();
        println!("error: {err1}");
        let err2 = err_buf.get();
        println!("error: {err2}");
        // err1;
    }

    #[test]
    fn test1() {
        let mut err_buf = sdl_thread().unwrap();
        let sdl = init(&mut err_buf, 0).unwrap();
        let window = sdl.create_window(&mut err_buf, c"test", 1280, 720, 0).unwrap();
        std::thread::sleep(Duration::from_secs(2));
    }
}
