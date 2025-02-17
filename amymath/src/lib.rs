#![feature(const_trait_impl)]

pub mod color;
pub mod vec;
pub mod nvec;
pub mod mat;
pub mod nrect;
pub mod rlgl;
pub mod num;
pub mod set;

pub mod prelude {
    pub use crate::{
        color::*,
        vec::*,
        nvec::*,
        mat::*,
        nrect::*,
        rlgl::*,
        num::*,
        set::*,
    };
}
