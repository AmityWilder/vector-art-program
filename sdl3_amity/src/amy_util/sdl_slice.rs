use std::{ops::{Deref, DerefMut}, ptr::NonNull};
use sdl3_sys::stdinc::SDL_free;

/// A slice allocated by SDL
pub struct SdlSlice<T> {
    mem: NonNull<T>,
    count: usize,
}

impl<T> SdlSlice<T> {
    pub(crate) fn try_from_raw_parts(mem: *mut T, count: usize) -> Option<Self> {
        NonNull::new(mem)
            .map(|mem| Self { mem, count })
    }
}

impl<T> Drop for SdlSlice<T> {
    fn drop(&mut self) {
        unsafe { SDL_free(self.mem.as_ptr().cast());}
    }
}

impl<T> Deref for SdlSlice<T> {
    type Target = [T];
    fn deref(&self) -> &Self::Target {
        unsafe { std::slice::from_raw_parts(self.mem.as_ptr(), self.count) }
    }
}

impl<T> DerefMut for SdlSlice<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { std::slice::from_raw_parts_mut(self.mem.as_ptr(), self.count) }
    }
}
