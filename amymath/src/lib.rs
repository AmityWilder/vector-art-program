pub mod color;
pub mod vec;
pub mod mat;
pub mod rec;

pub mod prelude {
    pub use crate::{
        color::*,
        vec::*,
        mat::*,
        rec::*,
    };
}

pub trait Arithmetic:
    Sized + Copy +
    std::ops::Add<Output = Self> +
    std::ops::Sub<Output = Self> +
    std::ops::Div<Output = Self> +
    std::ops::Mul<Output = Self>
{
    const ZERO: Self;
    const ONE: Self;
    fn half(self) -> Self;
}

macro_rules! impl_arith {
    ($($T:ty),+ $(,)?) => {
        $(impl Arithmetic for $T {
            const ZERO: Self = 0;
            const ONE: Self = 1;
            fn half(self) -> Self {
                self / 2
            }
        })+
    };
}

impl_arith!{
    i8, i16, i32, i64, i128, isize,
    u8, u16, u32, u64, u128, usize,
}

macro_rules! impl_arith_f {
    ($($T:ty),+ $(,)?) => {
        $(impl Arithmetic for $T {
            const ZERO: Self = 0.0;
            const ONE: Self = 1.0;
            fn half(self) -> Self {
                self * 0.5
            }
        })+
    };
}

impl_arith_f!{ f32, f64 }
