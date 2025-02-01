pub mod color;
pub mod vec;
pub mod mat;
pub mod rec;
pub mod rlgl;

pub mod prelude {
    pub use crate::{
        color::*,
        vec::*,
        mat::*,
        rec::*,
        rlgl::*,
    };
}
