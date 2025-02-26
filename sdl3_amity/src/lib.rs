#![feature(negative_impls)]
#![cfg_attr(feature = "c_variadic", feature(c_variadic))]

pub mod ffi {
    pub use sdl3_sys::*;
}

pub mod error;
pub mod init;
pub mod video;
pub mod properties;

pub mod prelude {
    pub use crate::{
        error::*,
        init::*,
        video::*,
        properties::*,
    };
}

#[cfg(test)]
mod test {
    use std::time::Duration;
    use super::prelude::*;

    #[test]
    fn test0() {
        let mut eb = err_buf().unwrap();
        eb.set(c"all our food keeps blowing up");
        let err1 = eb.get();
        println!("error: {err1}");
        eb.set(c"the white zone is for immediate loading and unloading of passengers only, there is no stopping in the red zone");
        let err2 = eb.get();
        println!("error: {err2}");
        // err1;
    }

    #[test]
    fn test1() {
        let mut eb = err_buf().unwrap();
        let sdl = init(&mut eb, InitFlags::empty()).unwrap();
        let _window = Window::create(&sdl, &mut eb, c"test", 1280, 720, WindowFlags::empty()).unwrap();
        std::thread::sleep(Duration::from_secs(2));
    }
}
