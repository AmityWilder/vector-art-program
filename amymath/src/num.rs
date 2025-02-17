#[const_trait]
pub trait Arithmetic:
    Sized + Copy + PartialEq + Default +
    std::ops::Add<Output = Self> + std::ops::AddAssign +
    std::ops::Sub<Output = Self> + std::ops::SubAssign +
    std::ops::Div<Output = Self> + std::ops::DivAssign +
    std::ops::Mul<Output = Self> + std::ops::MulAssign +
{
    const ZERO: Self;
    const ONE: Self;
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn half(self) -> Self;
}

macro_rules! impl_arith {
    ($($T:ty),+ $(,)?) => {
        $(impl Arithmetic for $T {
            const ZERO: Self = 0;
            const ONE: Self = 1;
            #[inline]
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
            #[inline]
            fn half(self) -> Self {
                self * 0.5
            }
        })+
    };
}

impl_arith_f!{ f32, f64 }

#[const_trait]
pub trait Abs {
    type Output;
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn abs(self) -> Self::Output;
}

impl Abs for f32 {
    type Output = Self;
    #[inline]
    fn abs(self) -> Self::Output {
        self.abs()
    }
}
impl Abs for f64 {
    type Output = Self;
    #[inline]
    fn abs(self) -> Self::Output {
        self.abs()
    }
}

#[const_trait]
pub trait UnsignedAbs {
    type Output;
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn unsigned_abs(self) -> Self::Output;
}

macro_rules! impl_unsigned_abs {
    ($($T:ty => $U:ty),+ $(,)?) => {$(
        impl const UnsignedAbs for $T {
            type Output = $U;
            #[inline]
            fn unsigned_abs(self) -> Self::Output {
                self.unsigned_abs()
            }
        }
    )+};
}

impl_unsigned_abs!{
    i8 => u8,
    i16 => u16,
    i32 => u32,
    i64 => u64,
    i128 => u128,
    isize => usize,
}

#[const_trait]
pub trait CheckedAbs<Rhs = Self> {
    type Output;
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn checked_abs(self) -> Option<Self::Output>;
}

#[const_trait]
pub trait WrappingAbs<Rhs = Self> {
    type Output;
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn wrapping_abs(self) -> Self::Output;
}

#[const_trait]
pub trait SaturatingAbs<Rhs = Self> {
    type Output;
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn saturating_abs(self) -> Self::Output;
}

#[const_trait]
pub trait OverflowingAbs<Rhs = Self> {
    type Output;
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn overflowing_abs(self) -> (Self::Output, bool);
}

macro_rules! impl_abs {
    ($($T:ty),+ $(,)?) => {$(
        impl const Abs for $T {
            type Output = Self;
            #[inline]
            fn abs(self) -> Self::Output {
                self.abs()
            }
        }
        impl const CheckedAbs for $T {
            type Output = Self;
            #[inline]
            fn checked_abs(self) -> Option<Self::Output> {
                self.checked_abs()
            }
        }
        impl const WrappingAbs for $T {
            type Output = Self;
            #[inline]
            fn wrapping_abs(self) -> Self::Output {
                self.wrapping_abs()
            }
        }
        impl const SaturatingAbs for $T {
            type Output = Self;
            #[inline]
            fn saturating_abs(self) -> Self::Output {
                self.saturating_abs()
            }
        }
        impl const OverflowingAbs for $T {
            type Output = Self;
            #[inline]
            fn overflowing_abs(self) -> (Self::Output, bool) {
                self.overflowing_abs()
            }
        }
    )+};
}

impl_abs!{
    i8, i16, i32, i64, i128, isize,
}

#[const_trait]
pub trait CheckedAdd<Rhs = Self> {
    type Output;
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn checked_add(self, rhs: Rhs) -> Option<Self::Output>;
}
#[const_trait]
pub trait CheckedSub<Rhs = Self> {
    type Output;
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn checked_sub(self, rhs: Rhs) -> Option<Self::Output>;
}
#[const_trait]
pub trait CheckedMul<Rhs = Self> {
    type Output;
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn checked_mul(self, rhs: Rhs) -> Option<Self::Output>;
}
#[const_trait]
pub trait CheckedDiv<Rhs = Self> {
    type Output;
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn checked_div(self, rhs: Rhs) -> Option<Self::Output>;
}

#[const_trait]
pub trait WrappingAdd<Rhs = Self> {
    type Output;
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn wrapping_add(self, rhs: Rhs) -> Self::Output;
}
#[const_trait]
pub trait WrappingSub<Rhs = Self> {
    type Output;
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn wrapping_sub(self, rhs: Rhs) -> Self::Output;
}
#[const_trait]
pub trait WrappingMul<Rhs = Self> {
    type Output;
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn wrapping_mul(self, rhs: Rhs) -> Self::Output;
}
#[const_trait]
pub trait WrappingDiv<Rhs = Self> {
    type Output;
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn wrapping_div(self, rhs: Rhs) -> Self::Output;
}

#[const_trait]
pub trait SaturatingAdd<Rhs = Self> {
    type Output;
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn saturating_add(self, rhs: Rhs) -> Self::Output;
}
#[const_trait]
pub trait SaturatingSub<Rhs = Self> {
    type Output;
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn saturating_sub(self, rhs: Rhs) -> Self::Output;
}
#[const_trait]
pub trait SaturatingMul<Rhs = Self> {
    type Output;
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn saturating_mul(self, rhs: Rhs) -> Self::Output;
}
#[const_trait]
pub trait SaturatingDiv<Rhs = Self> {
    type Output;
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn saturating_div(self, rhs: Rhs) -> Self::Output;
}

#[const_trait]
pub trait OverflowingAdd<Rhs = Self> {
    type Output;
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn overflowing_add(self, rhs: Rhs) -> (Self::Output, bool);
}
#[const_trait]
pub trait OverflowingSub<Rhs = Self> {
    type Output;
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn overflowing_sub(self, rhs: Rhs) -> (Self::Output, bool);
}
#[const_trait]
pub trait OverflowingMul<Rhs = Self> {
    type Output;
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn overflowing_mul(self, rhs: Rhs) -> (Self::Output, bool);
}
#[const_trait]
pub trait OverflowingDiv<Rhs = Self> {
    type Output;
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn overflowing_div(self, rhs: Rhs) -> (Self::Output, bool);
}

macro_rules! impl_checked_ops {
    ($($T:ty),+ $(,)?) => {$(
        impl const CheckedAdd for $T {
            type Output = Self;
            #[inline]
            fn checked_add(self, rhs: Self) -> Option<Self::Output> {
                self.checked_add(rhs)
            }
        }
        impl const CheckedSub for $T {
            type Output = Self;
            #[inline]
            fn checked_sub(self, rhs: Self) -> Option<Self::Output> {
                self.checked_sub(rhs)
            }
        }
        impl const CheckedMul for $T {
            type Output = Self;
            #[inline]
            fn checked_mul(self, rhs: Self) -> Option<Self::Output> {
                self.checked_mul(rhs)
            }
        }
        impl const CheckedDiv for $T {
            type Output = Self;
            #[inline]
            fn checked_div(self, rhs: Self) -> Option<Self::Output> {
                self.checked_div(rhs)
            }
        }

        impl const WrappingAdd for $T {
            type Output = Self;
            #[inline]
            fn wrapping_add(self, rhs: Self) -> Self::Output {
                self.wrapping_add(rhs)
            }
        }
        impl const WrappingSub for $T {
            type Output = Self;
            #[inline]
            fn wrapping_sub(self, rhs: Self) -> Self::Output {
                self.wrapping_sub(rhs)
            }
        }
        impl const WrappingMul for $T {
            type Output = Self;
            #[inline]
            fn wrapping_mul(self, rhs: Self) -> Self::Output {
                self.wrapping_mul(rhs)
            }
        }
        impl const WrappingDiv for $T {
            type Output = Self;
            #[inline]
            fn wrapping_div(self, rhs: Self) -> Self::Output {
                self.wrapping_div(rhs)
            }
        }

        impl const SaturatingAdd for $T {
            type Output = Self;
            #[inline]
            fn saturating_add(self, rhs: Self) -> Self::Output {
                self.saturating_add(rhs)
            }
        }
        impl const SaturatingSub for $T {
            type Output = Self;
            #[inline]
            fn saturating_sub(self, rhs: Self) -> Self::Output {
                self.saturating_sub(rhs)
            }
        }
        impl const SaturatingMul for $T {
            type Output = Self;
            #[inline]
            fn saturating_mul(self, rhs: Self) -> Self::Output {
                self.saturating_mul(rhs)
            }
        }
        impl const SaturatingDiv for $T {
            type Output = Self;
            #[inline]
            fn saturating_div(self, rhs: Self) -> Self::Output {
                self.saturating_div(rhs)
            }
        }

        impl const OverflowingAdd for $T {
            type Output = Self;
            #[inline]
            fn overflowing_add(self, rhs: Self) -> (Self::Output, bool) {
                self.overflowing_add(rhs)
            }
        }
        impl const OverflowingSub for $T {
            type Output = Self;
            #[inline]
            fn overflowing_sub(self, rhs: Self) -> (Self::Output, bool) {
                self.overflowing_sub(rhs)
            }
        }
        impl const OverflowingMul for $T {
            type Output = Self;
            #[inline]
            fn overflowing_mul(self, rhs: Self) -> (Self::Output, bool) {
                self.overflowing_mul(rhs)
            }
        }
        impl const OverflowingDiv for $T {
            type Output = Self;
            #[inline]
            fn overflowing_div(self, rhs: Self) -> (Self::Output, bool) {
                self.overflowing_div(rhs)
            }
        }
    )+};
}

impl_checked_ops!{
    i8, i16, i32, i64, i128, isize,
    u8, u16, u32, u64, u128, usize,
}

pub trait ElementwiseOrd<T = Self> {
    /// Clamps each element separately
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn clamp(self, min: T, max: T) -> Self;

    /// Returns a new `Self` calling `max(self, other)` on each element
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn max(self, other: T) -> Self;

    /// Returns a new `Self` calling `min(self, other)` on each element
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn min(self, other: T) -> Self;
}

pub trait Sqrt {
    type Output;
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn sqrt(self) -> Self::Output;
}

impl Sqrt for f32 {
    type Output = Self;
    #[inline]
    fn sqrt(self) -> Self::Output {
        self.sqrt()
    }
}

impl Sqrt for f64 {
    type Output = Self;
    #[inline]
    fn sqrt(self) -> Self::Output {
        self.sqrt()
    }
}

pub trait ISqrt {
    type Output;
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn isqrt(self) -> Self::Output;
}

macro_rules! impl_isqrt {
    ($($T:ty),+ $(,)?) => {$(
        impl ISqrt for $T {
            type Output = Self;
            #[inline]
            fn isqrt(self) -> Self::Output {
                self.isqrt()
            }
        }
    )+};
}

impl_isqrt!{
    i8, i16, i32, i64, i128, isize,
    u8, u16, u32, u64, u128, usize,
}

#[const_trait]
pub trait Min<Rhs = Self> {
    type Output;
    #[must_use]
    fn min(self, other: Rhs) -> Self::Output;
}

#[const_trait]
pub trait Max<Rhs = Self> {
    type Output;
    #[must_use]
    fn max(self, other: Rhs) -> Self::Output;
}

#[const_trait]
pub trait MinMax: Sized {
    type Output;
    #[must_use]
    fn minmax(self, other: Self) -> (Self::Output, Self::Output);
}

#[const_trait]
pub trait Clamp<Min = Self, Max = Self> {
    type Output;
    #[must_use]
    fn clamp(self, min: Min, max: Max) -> Self::Output;
}

impl const Min for f32 {
    type Output = Self;
    #[inline]
    fn min(self, other: Self) -> Self::Output {
        self.min(other)
    }
}
impl const Max for f32 {
    type Output = Self;
    #[inline]
    fn max(self, other: Self) -> Self::Output {
        self.max(other)
    }
}
impl const MinMax for f32 {
    type Output = Self;
    #[inline]
    fn minmax(self, other: Self) -> (Self::Output, Self::Output) {
        if self <= other { (self, other) } else { (other, self) }
    }
}
impl const Clamp for f32 {
    type Output = Self;
    #[inline]
    fn clamp(self, min: Self, max: Self) -> Self::Output {
        self.clamp(min, max)
    }
}

impl const Min for f64 {
    type Output = Self;
    #[inline]
    fn min(self, other: Self) -> Self::Output {
        self.min(other)
    }
}
impl const Max for f64 {
    type Output = Self;
    #[inline]
    fn max(self, other: Self) -> Self::Output {
        self.max(other)
    }
}
impl const MinMax for f64 {
    type Output = Self;
    #[inline]
    fn minmax(self, other: Self) -> (Self::Output, Self::Output) {
        if self <= other { (self, other) } else { (other, self) }
    }
}
impl const Clamp for f64 {
    type Output = Self;
    #[inline]
    fn clamp(self, min: Self, max: Self) -> Self::Output {
        self.clamp(min, max)
    }
}

macro_rules! impl_minmax_int {
    ($($T:ty),+ $(,)?) => {$(
        impl const Min for $T {
            type Output = Self;
            #[inline]
            fn min(self, other: Self) -> Self::Output {
                if self <= other { self } else { other }
            }
        }
        impl const Max for $T {
            type Output = Self;
            #[inline]
            fn max(self, other: Self) -> Self::Output {
                if self <= other { other } else { self }
            }
        }
        impl const MinMax for $T {
            type Output = Self;
            #[inline]
            fn minmax(self, other: Self) -> (Self::Output, Self::Output) {
                if self <= other { (self, other) } else { (other, self) }
            }
        }
        impl const Clamp for $T {
            type Output = Self;
            #[inline]
            fn clamp(self, min: Self, max: Self) -> Self::Output {
                if self <= min { min } else if max <= self { max } else { self }
            }
        }
    )+};
}

impl_minmax_int!{
    i8, i16, i32, i64, i128, isize,
    u8, u16, u32, u64, u128, usize,
}

pub trait Midpoint<Rhs = Self> {
    type Output;
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn midpoint(self, other: Rhs) -> Self::Output;
}

impl Midpoint for f32 {
    type Output = Self;
    fn midpoint(self, other: Self) -> Self::Output {
        self.midpoint(other)
    }
}

impl Midpoint for f64 {
    type Output = Self;
    fn midpoint(self, other: Self) -> Self::Output {
        self.midpoint(other)
    }
}
