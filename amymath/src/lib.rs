#![feature(
    const_trait_impl,
    const_ops,
    more_float_constants,
    maybe_uninit_array_assume_init,
    maybe_uninit_uninit_array,
    core_intrinsics, // unchecked_div and unchecked_rem used in `crate::uvec2::UVector2::grid_pos`
)]

pub mod color;
pub mod vec;
pub mod vec2;
pub mod ivec2;
pub mod uvec2;
pub mod mat;
pub mod nmat;
pub mod rec;
pub mod rect2;
pub mod irect2;
pub mod urect2;
pub mod rlgl;
// pub mod num;
pub mod set;

pub mod prelude {
    pub use crate::{
        color::*,
        vec::*,
        vec2::*,
        ivec2::*,
        uvec2::*,
        mat::*,
        nmat::*,
        rec::*,
        rect2::*,
        irect2::*,
        urect2::*,
        rlgl::*,
        set::*,
    };
}

#[const_trait]
pub trait MinMax: Sized {
    fn minmax(self, other: Self) -> (Self, Self);
}

macro_rules! impl_minmax {
    ($($T:ty),+ $(,)?) => {$(
        impl const MinMax for $T {
            fn minmax(self, other: $T) -> ($T, $T) {
                if self <= other { (self, other) } else { (other, self) }
            }
        }
    )+};
}

impl_minmax!{
    i8, i16, i32, i64, i128, isize,
    u8, u16, u32, u64, u128, usize,
    f32, f64,
}
