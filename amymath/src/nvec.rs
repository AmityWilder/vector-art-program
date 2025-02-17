use std::ops::*;
use raylib::prelude::*;
use crate::prelude::*;

#[must_use]
pub trait Vector: Arithmetic {
    type Item;
    type Rect: Rect;
    fn max(self) -> Self::Item;
    fn min(self) -> Self::Item;
    fn sum(self) -> Self::Item;
    fn prod(self) -> Self::Item;
    fn dot(self, other: Self) -> Self::Item;
    fn mag_sqr(self) -> Self::Item;
    fn mag(self) -> Self::Item where Self::Item: Sqrt<Output = Self::Item>;
    fn dist_sqr(self, other: Self) -> Self::Item;
    fn dist(self, other: Self) -> Self::Item where Self::Item: Sqrt<Output = Self::Item>;
    fn unit(self) -> Self where Self::Item: Sqrt<Output = Self::Item>;
    fn dir(self, other: Self) -> Self where Self::Item: Sqrt<Output = Self::Item>;
}

// -----------------------------------------------------------------------------------------
// Vec2
// -----------------------------------------------------------------------------------------

#[must_use]
pub struct Vec2<T = f32> {
    pub x: T,
    pub y: T,
}

impl<T: Arithmetic + Min<Output = T> + Max<Output = T>> Vector for Vec2<T> {
    type Item = T;
    type Rect = Rect2<T>;
    #[inline]
    fn max(self) -> T {
        self.x.min(self.y)
    }
    #[inline]
    fn min(self) -> T {
        self.x.max(self.y)
    }
    #[inline]
    fn sum(self) -> T {
        self.sum()
    }
    #[inline]
    fn prod(self) -> T {
        self.prod()
    }
    #[inline]
    fn dot(self, other: Self) -> T {
        (self * other).sum()
    }
    #[inline]
    fn mag_sqr(self) -> T {
        self.dot(self)
    }
    #[inline]
    fn mag(self) -> T where T: Sqrt<Output = T> {
        self.mag_sqr().sqrt()
    }
    #[inline]
    fn dist_sqr(self, other: Self) -> T {
        self.dist_sqr(other)
    }
    #[inline]
    fn dist(self, other: Self) -> T where T: Sqrt<Output = T> {
        self.dist(other)
    }
    #[inline]
    fn unit(self) -> Self where T: Sqrt<Output = T> {
        self.unit()
    }
    #[inline]
    fn dir(self, other: Self) -> Self where T: Sqrt<Output = T> {
        self.dir(other)
    }
}

pub type IVec2 = Vec2<i32>;

impl<T> Vec2<T> {
    #[inline]
    pub const fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    /// Returns the greatest of the vector's elements
    #[inline]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub fn max(self) -> T where T: Ord {
        self.x.max(self.y)
    }

    /// Returns the smallest of the vector's elements
    #[inline]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub fn min(self) -> T where T: Ord {
        self.x.min(self.y)
    }

    /// Returns the sum of the vector's elements
    #[inline]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub fn sum(self) -> <T as Add>::Output
    where
        T: Add
    {
        self.x + self.y
    }

    /// Returns the product of the vector's elements
    #[inline]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub fn prod(self) -> <T as Mul>::Output
    where
        T: Mul
    {
        self.x * self.y
    }

    /// Returns the dot product of two vectors
    #[inline]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub fn dot(self, other: Self) -> T
    where
        T:
            Mul<Output = T> +
            Add<Output = T>
    {
        (self * other).sum()
    }

    /// Returns the square of the magnitude of the vector
    #[inline]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub fn mag_sqr(self) -> T
    where
        T:
            Copy +
            Add<Output = T> +
            Mul<Output = T>
    {
        self.dot(self)
    }

    /// Returns the square of the magnitude of the vector
    #[inline]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub fn mag(self) -> <T as Sqrt>::Output
    where
        T:
            Copy +
            Add<Output = T> +
            Mul<Output = T> +
            Sqrt
    {
        self.mag_sqr().sqrt()
    }

    /// Returns the square of the magnitude of the vector
    #[inline]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub fn dist_sqr(self, other: Self) -> T
    where
        T:
            Copy +
            Add<Output = T> +
            Sub<Output = T> +
            Mul<Output = T>
    {
        (other - self).mag_sqr()
    }

    /// Returns the square of the magnitude of the vector
    #[inline]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub fn dist(self, other: Self) -> <T as Sqrt>::Output
    where
        T:
            Copy +
            Add<Output = T> +
            Sub<Output = T> +
            Mul<Output = T> +
            Sqrt
    {
        self.dist_sqr(other).sqrt()
    }

    /// Returns the normalized unit vector
    #[inline]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub fn unit(self) -> Self
    where
        Self:
            Div<<T as Sqrt>::Output, Output = Self>,
        T:
            Copy +
            Add<Output = T> +
            Mul<Output = T> +
            Sqrt
    {
        self / self.mag()
    }

    /// Returns the normalized direction vector from `self` to `other`
    #[inline]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub fn dir(self, other: Self) -> Self
    where
        Self:
            Div<<T as Sqrt>::Output, Output = Self>,
        T:
            Copy +
            Add<Output = T> +
            Sub<Output = T> +
            Mul<Output = T> +
            Sqrt
    {
        (other - self).unit()
    }
}

impl<T> IntoIterator for Vec2<T> {
    type Item = T;
    type IntoIter = <[T; 2] as IntoIterator>::IntoIter;
    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        <[T; 2]>::from(self).into_iter()
    }
}

impl<T: Ord> ElementwiseOrd for Vec2<T> {
    #[inline]
    fn clamp(self, min: Self, max: Self) -> Self {
        Self {
            x: self.x.clamp(min.x, max.x),
            y: self.y.clamp(min.y, max.y),
        }
    }
    #[inline]
    fn max(self, other: Self) -> Self {
        Self {
            x: self.x.max(other.x),
            y: self.y.max(other.y),
        }
    }
    #[inline]
    fn min(self, other: Self) -> Self {
        Self {
            x: self.x.min(other.x),
            y: self.y.min(other.y),
        }
    }
}

impl<T: Ord + Copy> ElementwiseOrd<T> for Vec2<T> {
    #[inline]
    fn clamp(self, min: T, max: T) -> Self {
        Self {
            x: self.x.clamp(min, max),
            y: self.y.clamp(min, max),
        }
    }
    #[inline]
    fn max(self, other: T) -> Self {
        Self {
            x: self.x.max(other),
            y: self.y.max(other),
        }
    }
    #[inline]
    fn min(self, other: T) -> Self {
        Self {
            x: self.x.min(other),
            y: self.y.min(other),
        }
    }
}

impl<T: Midpoint> Midpoint for Vec2<T> {
    type Output = Vec2<T::Output>;
    #[inline]
    fn midpoint(self, other: Self) -> Self::Output {
        Vec2 {
            x: self.x.midpoint(other.x),
            y: self.y.midpoint(other.y),
        }
    }
}

impl<T: ~const Min<U> + Copy, U: Copy> const Min<Vec2<U>> for Vec2<T> {
    type Output = Vec2<T::Output>;
    #[inline]
    fn min(self, other: Vec2<U>) -> Self::Output {
        Vec2 {
            x: self.x.min(other.x),
            y: self.y.min(other.y),
        }
    }
}

impl<T: ~const Max<U> + Copy, U: Copy> const Max<Vec2<U>> for Vec2<T> {
    type Output = Vec2<T::Output>;
    #[inline]
    fn max(self, other: Vec2<U>) -> Self::Output {
        Vec2 {
            x: self.x.max(other.x),
            y: self.y.max(other.y),
        }
    }
}

impl<T: ~const MinMax<Output: Copy> + Copy> const MinMax for Vec2<T> {
    type Output = Vec2<T::Output>;
    #[inline]
    fn minmax(self, other: Self) -> (Self::Output, Self::Output) {
        let (xmin, xmax) = self.x.minmax(other.x);
        let (ymin, ymax) = self.y.minmax(other.y);
        (Vec2 { x: xmin, y: ymin }, Vec2 { x: xmax, y: ymax })
    }
}

impl<T: ~const Clamp<U, V, Output: Copy> + Copy, U: Copy, V: Copy> const Clamp<Vec2<U>, Vec2<V>> for Vec2<T> {
    type Output = Vec2<T::Output>;
    #[inline]
    fn clamp(self, min: Vec2<U>, max: Vec2<V>) -> Self::Output {
        Vec2 {
            x: self.x.clamp(min.x, max.x),
            y: self.y.clamp(min.y, max.y),
        }
    }
}

impl<T: PartialOrd> PartialOrd for Vec2<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        use std::cmp::Ordering;
        match self.x.partial_cmp(&other.x).zip(self.y.partial_cmp(&other.y)) {
            Some((Ordering::Less, Ordering::Less | Ordering::Equal) | (Ordering::Equal, Ordering::Less)) => Some(Ordering::Less),
            Some((Ordering::Equal, Ordering::Equal)) => Some(Ordering::Equal),
            Some((Ordering::Greater, Ordering::Greater | Ordering::Equal) | (Ordering::Equal, Ordering::Greater)) => Some(Ordering::Greater),
            Some((Ordering::Greater, Ordering::Less) | (Ordering::Less, Ordering::Greater)) | None => None,
        }
    }
}

impl<T> Index<usize> for Vec2<T> {
    type Output = T;
    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2.. => panic!("Out of bounds"),
        }
    }
}

impl<T> IndexMut<usize> for Vec2<T> {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2.. => panic!("Out of bounds"),
        }
    }
}

impl From<Vector2> for Vec2<f32> {
    #[inline]
    fn from(Vector2 { x, y }: Vector2) -> Self {
        Self { x, y }
    }
}

impl From<Vec2<f32>> for Vector2 {
    #[inline]
    fn from(Vec2 { x, y }: Vec2<f32>) -> Self {
        Self { x, y }
    }
}

impl<T: Copy> From<T> for Vec2<T> {
    #[inline]
    fn from(xy: T) -> Self {
        Self { x: xy, y: xy }
    }
}

impl<T> From<(T, T)> for Vec2<T> {
    #[inline]
    fn from((x, y): (T, T)) -> Self {
        Self { x, y }
    }
}
impl<T> From<Vec2<T>> for (T, T) {
    #[inline]
    fn from(v: Vec2<T>) -> Self {
        (v.x, v.y)
    }
}

impl<T> From<[T; 2]> for Vec2<T> {
    #[inline]
    fn from([x, y]: [T; 2]) -> Self {
        Self { x, y }
    }
}
impl<T> From<Vec2<T>> for [T; 2] {
    #[inline]
    fn from(v: Vec2<T>) -> Self {
        [v.x, v.y]
    }
}

impl<T: std::fmt::Debug> std::fmt::Debug for Vec2<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Vec2")
            .field("x", &self.x)
            .field("y", &self.y)
            .finish()
    }
}

impl<T: Default> Default for Vec2<T> {
    #[inline]
    fn default() -> Self {
        Self {
            x: Default::default(),
            y: Default::default(),
        }
    }
}

impl<T: Clone> Clone for Vec2<T> {
    #[inline]
    fn clone(&self) -> Self {
        Self {
            x: self.x.clone(),
            y: self.y.clone(),
        }
    }
}

impl<T: Copy> Copy for Vec2<T> {}

impl<T: std::hash::Hash> std::hash::Hash for Vec2<T> {
    #[inline]
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}

impl<T: PartialEq> PartialEq for Vec2<T> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl<T: Eq> Eq for Vec2<T> {}

macro_rules! impl_elementwise_unary {
    ($($Trait:ident::$fn:ident),+ $(,)?) => {$(
        impl<T: $Trait> $Trait for Vec2<T> {
            type Output = Vec2<T::Output>;
            #[inline]
            fn $fn(self) -> Self::Output {
                Vec2 {
                    x: self.x.$fn(),
                    y: self.y.$fn(),
                }
            }
        }
    )+};
}

impl_elementwise_unary!{
    Neg::neg,
    Not::not,
    Abs::abs,
    WrappingAbs::wrapping_abs,
    SaturatingAbs::saturating_abs,
}

impl<T: CheckedAbs> CheckedAbs for Vec2<T> {
    type Output = Vec2<T::Output>;
    fn checked_abs(self) -> Option<Self::Output> {
        self.x.checked_abs().zip(self.y.checked_abs()).map(|v| v.into())
    }
}

impl<T: OverflowingAbs> OverflowingAbs for Vec2<T> {
    type Output = Vec2<T::Output>;
    fn overflowing_abs(self) -> (Self::Output, bool) {
        let (x, overflow_x) = self.x.overflowing_abs();
        let (y, overflow_y) = self.y.overflowing_abs();
        (Vec2 { x, y }, overflow_x || overflow_y)
    }
}

macro_rules! impl_elementwise {
    ($($Trait:ident::$fn:ident$( = $TraitAssign:ident::$fn_assign:ident)?),+ $(,)?) => {$(
        impl<T: $Trait> $Trait for Vec2<T> {
            type Output = Vec2<<T as $Trait>::Output>;
            #[inline]
            fn $fn(self, rhs: Self) -> Self::Output {
                Vec2 {
                    x: self.x.$fn(rhs.x),
                    y: self.y.$fn(rhs.y),
                }
            }
        }
        impl<T: Copy + $Trait<T>> $Trait<T> for Vec2<T> {
            type Output = Vec2<<T as $Trait<T>>::Output>;
            #[inline]
            fn $fn(self, rhs: T) -> Self::Output {
                Vec2 {
                    x: self.x.$fn(rhs),
                    y: self.y.$fn(rhs),
                }
            }
        }
        $(impl<T: $Trait<Output = T> + Copy> $TraitAssign for Vec2<T> {
            #[inline]
            fn $fn_assign(&mut self, rhs: Self) {
                *self = (*self).$fn(rhs);
            }
        }
        impl<T: $Trait<Output = T> + Copy> $TraitAssign<T> for Vec2<T> {
            #[inline]
            fn $fn_assign(&mut self, rhs: T) {
                *self = (*self).$fn(rhs);
            }
        })?
    )+};
}

impl_elementwise!{
    Add::add = AddAssign::add_assign,
      WrappingAdd::  wrapping_add,
    SaturatingAdd::saturating_add,
    Sub::sub = SubAssign::sub_assign,
      WrappingSub::  wrapping_sub,
    SaturatingSub::saturating_sub,
    Mul::mul = MulAssign::mul_assign,
      WrappingMul::  wrapping_mul,
    SaturatingMul::saturating_mul,
    Div::div = DivAssign::div_assign,
      WrappingDiv::  wrapping_div,
    SaturatingDiv::saturating_div,
    Rem   ::rem    = RemAssign   ::rem_assign   ,
    BitAnd::bitand = BitAndAssign::bitand_assign,
    BitOr ::bitor  = BitOrAssign ::bitor_assign ,
    BitXor::bitxor = BitXorAssign::bitxor_assign,
    Shl   ::shl    = ShlAssign   ::shl_assign   ,
    Shr   ::shr    = ShrAssign   ::shr_assign   ,
}

macro_rules! impl_elementwise_checked {
    ($($Trait:ident::$fn:ident),+ $(,)?) => {$(
        impl<T: $Trait> $Trait for Vec2<T> {
            type Output = Vec2<T::Output>;
            #[inline]
            fn $fn(self, rhs: Self) -> Option<Self::Output> {
                self.x.$fn(rhs.x).zip(self.y.$fn(rhs.y))
                    .map(|v| v.into())
            }
        }
        impl<T: Copy + $Trait> $Trait<T> for Vec2<T> {
            type Output = Vec2<T::Output>;
            #[inline]
            fn $fn(self, rhs: T) -> Option<Self::Output> {
                self.x.$fn(rhs).zip(self.y.$fn(rhs))
                    .map(|v| v.into())
            }
        }
    )+};
}

impl_elementwise_checked!{
    CheckedAdd::checked_add,
    CheckedSub::checked_sub,
    CheckedMul::checked_mul,
    CheckedDiv::checked_div,
}

macro_rules! impl_elementwise_overflowing {
    ($($Trait:ident::$fn:ident),+ $(,)?) => {$(
        impl<T: $Trait> $Trait for Vec2<T> {
            type Output = Vec2<T::Output>;
            #[inline]
            fn $fn(self, rhs: Self) -> (Self::Output, bool) {
                let (x, overflow_x) = self.x.$fn(rhs.x);
                let (y, overflow_y) = self.y.$fn(rhs.y);
                (Vec2 { x, y }, overflow_x || overflow_y)
            }
        }
        impl<T: Copy + $Trait> $Trait<T> for Vec2<T> {
            type Output = Vec2<T::Output>;
            #[inline]
            fn $fn(self, rhs: T) -> (Self::Output, bool) {
                let (x, overflow_x) = self.x.$fn(rhs);
                let (y, overflow_y) = self.y.$fn(rhs);
                (Vec2 { x, y }, overflow_x || overflow_y)
            }
        }
    )+};
}

impl_elementwise_overflowing!{
    OverflowingAdd::overflowing_add,
    OverflowingSub::overflowing_sub,
    OverflowingMul::overflowing_mul,
    OverflowingDiv::overflowing_div,
}

impl<T: Arithmetic> Arithmetic for Vec2<T> {
    const ZERO: Self = Self { x: T::ZERO, y: T::ZERO };
    const ONE: Self = Self { x: T::ONE, y: T::ONE };
    #[inline]
    fn half(self) -> Self {
        Self { x: self.x.half(), y: self.y.half() }
    }
}

macro_rules! impl_lhs_vec_arith {
    ($($T:ty),+ $(,)?) => {$(
        impl Add<Vec2<$T>> for $T {
            type Output = Vec2<$T>;
            #[inline]
            fn add(self, rhs: Vec2<$T>) -> Self::Output {
                Vec2 {
                    x: self + rhs.x,
                    y: self + rhs.y,
                }
            }
        }
        impl Sub<Vec2<$T>> for $T {
            type Output = Vec2<$T>;
            #[inline]
            fn sub(self, rhs: Vec2<$T>) -> Self::Output {
                Vec2 {
                    x: self - rhs.x,
                    y: self - rhs.y,
                }
            }
        }
        impl Mul<Vec2<$T>> for $T {
            type Output = Vec2<$T>;
            #[inline]
            fn mul(self, rhs: Vec2<$T>) -> Self::Output {
                Vec2 {
                    x: self * rhs.x,
                    y: self * rhs.y,
                }
            }
        }
        impl Div<Vec2<$T>> for $T {
            type Output = Vec2<$T>;
            #[inline]
            fn div(self, rhs: Vec2<$T>) -> Self::Output {
                Vec2 {
                    x: self / rhs.x,
                    y: self / rhs.y,
                }
            }
        }
    )+};
}

impl_lhs_vec_arith!{
    i8, i16, i32, i64, i128, isize,
    u8, u16, u32, u64, u128, usize,
    f32, f64,
}

macro_rules! impl_lhs_vec_discrete_arith {
    ($($T:ty),+ $(,)?) => {$(
        impl BitAnd<Vec2<$T>> for $T {
            type Output = Vec2<$T>;
            #[inline]
            fn bitand(self, rhs: Vec2<$T>) -> Self::Output {
                Vec2 {
                    x: self & rhs.x,
                    y: self & rhs.y,
                }
            }
        }
        impl BitOr<Vec2<$T>> for $T {
            type Output = Vec2<$T>;
            #[inline]
            fn bitor(self, rhs: Vec2<$T>) -> Self::Output {
                Vec2 {
                    x: self | rhs.x,
                    y: self | rhs.y,
                }
            }
        }
        impl BitXor<Vec2<$T>> for $T {
            type Output = Vec2<$T>;
            #[inline]
            fn bitxor(self, rhs: Vec2<$T>) -> Self::Output {
                Vec2 {
                    x: self ^ rhs.x,
                    y: self ^ rhs.y,
                }
            }
        }
        impl Rem<Vec2<$T>> for $T {
            type Output = Vec2<$T>;
            #[inline]
            fn rem(self, rhs: Vec2<$T>) -> Self::Output {
                Vec2 {
                    x: self % rhs.x,
                    y: self % rhs.y,
                }
            }
        }
        impl Shl<Vec2<$T>> for $T {
            type Output = Vec2<$T>;
            #[inline]
            fn shl(self, rhs: Vec2<$T>) -> Self::Output {
                Vec2 {
                    x: self << rhs.x,
                    y: self << rhs.y,
                }
            }
        }
        impl Shr<Vec2<$T>> for $T {
            type Output = Vec2<$T>;
            #[inline]
            fn shr(self, rhs: Vec2<$T>) -> Self::Output {
                Vec2 {
                    x: self >> rhs.x,
                    y: self >> rhs.y,
                }
            }
        }
        impl CheckedAdd<Vec2<$T>> for $T {
            type Output = Vec2<$T>;
            #[inline]
            fn checked_add(self, rhs: Vec2<$T>) -> Option<Self::Output> {
                self.checked_add(rhs.x).zip(self.checked_add(rhs.y)).map(|v| v.into())
            }
        }
        impl CheckedSub<Vec2<$T>> for $T {
            type Output = Vec2<$T>;
            #[inline]
            fn checked_sub(self, rhs: Vec2<$T>) -> Option<Self::Output> {
                self.checked_sub(rhs.x).zip(self.checked_sub(rhs.y)).map(|v| v.into())
            }
        }
        impl CheckedMul<Vec2<$T>> for $T {
            type Output = Vec2<$T>;
            #[inline]
            fn checked_mul(self, rhs: Vec2<$T>) -> Option<Self::Output> {
                self.checked_mul(rhs.x).zip(self.checked_mul(rhs.y)).map(|v| v.into())
            }
        }
        impl CheckedDiv<Vec2<$T>> for $T {
            type Output = Vec2<$T>;
            #[inline]
            fn checked_div(self, rhs: Vec2<$T>) -> Option<Self::Output> {
                self.checked_div(rhs.x).zip(self.checked_div(rhs.y)).map(|v| v.into())
            }
        }
        impl WrappingAdd<Vec2<$T>> for $T {
            type Output = Vec2<$T>;
            #[inline]
            fn wrapping_add(self, rhs: Vec2<$T>) -> Self::Output {
                Vec2 {
                    x: self.wrapping_add(rhs.x),
                    y: self.wrapping_add(rhs.y),
                }
            }
        }
        impl WrappingSub<Vec2<$T>> for $T {
            type Output = Vec2<$T>;
            #[inline]
            fn wrapping_sub(self, rhs: Vec2<$T>) -> Self::Output {
                Vec2 {
                    x: self.wrapping_sub(rhs.x),
                    y: self.wrapping_sub(rhs.y),
                }
            }
        }
        impl WrappingMul<Vec2<$T>> for $T {
            type Output = Vec2<$T>;
            #[inline]
            fn wrapping_mul(self, rhs: Vec2<$T>) -> Self::Output {
                Vec2 {
                    x: self.wrapping_mul(rhs.x),
                    y: self.wrapping_mul(rhs.y),
                }
            }
        }
        impl WrappingDiv<Vec2<$T>> for $T {
            type Output = Vec2<$T>;
            #[inline]
            fn wrapping_div(self, rhs: Vec2<$T>) -> Self::Output {
                Vec2 {
                    x: self.wrapping_div(rhs.x),
                    y: self.wrapping_div(rhs.y),
                }
            }
        }
        impl SaturatingAdd<Vec2<$T>> for $T {
            type Output = Vec2<$T>;
            #[inline]
            fn saturating_add(self, rhs: Vec2<$T>) -> Self::Output {
                Vec2 {
                    x: self.saturating_add(rhs.x),
                    y: self.saturating_add(rhs.y),
                }
            }
        }
        impl SaturatingSub<Vec2<$T>> for $T {
            type Output = Vec2<$T>;
            #[inline]
            fn saturating_sub(self, rhs: Vec2<$T>) -> Self::Output {
                Vec2 {
                    x: self.saturating_sub(rhs.x),
                    y: self.saturating_sub(rhs.y),
                }
            }
        }
        impl SaturatingMul<Vec2<$T>> for $T {
            type Output = Vec2<$T>;
            #[inline]
            fn saturating_mul(self, rhs: Vec2<$T>) -> Self::Output {
                Vec2 {
                    x: self.saturating_mul(rhs.x),
                    y: self.saturating_mul(rhs.y),
                }
            }
        }
        impl SaturatingDiv<Vec2<$T>> for $T {
            type Output = Vec2<$T>;
            #[inline]
            fn saturating_div(self, rhs: Vec2<$T>) -> Self::Output {
                Vec2 {
                    x: self.saturating_div(rhs.x),
                    y: self.saturating_div(rhs.y),
                }
            }
        }
        impl OverflowingAdd<Vec2<$T>> for $T {
            type Output = Vec2<$T>;
            #[inline]
            fn overflowing_add(self, rhs: Vec2<$T>) -> (Self::Output, bool) {
                let (x, overflow_x) = self.overflowing_add(rhs.x);
                let (y, overflow_y) = self.overflowing_add(rhs.y);
                (Vec2 { x, y }, overflow_x || overflow_y)
            }
        }
        impl OverflowingSub<Vec2<$T>> for $T {
            type Output = Vec2<$T>;
            #[inline]
            fn overflowing_sub(self, rhs: Vec2<$T>) -> (Self::Output, bool) {
                let (x, overflow_x) = self.overflowing_sub(rhs.x);
                let (y, overflow_y) = self.overflowing_sub(rhs.y);
                (Vec2 { x, y }, overflow_x || overflow_y)
            }
        }
        impl OverflowingMul<Vec2<$T>> for $T {
            type Output = Vec2<$T>;
            #[inline]
            fn overflowing_mul(self, rhs: Vec2<$T>) -> (Self::Output, bool) {
                let (x, overflow_x) = self.overflowing_mul(rhs.x);
                let (y, overflow_y) = self.overflowing_mul(rhs.y);
                (Vec2 { x, y }, overflow_x || overflow_y)
            }
        }
        impl OverflowingDiv<Vec2<$T>> for $T {
            type Output = Vec2<$T>;
            #[inline]
            fn overflowing_div(self, rhs: Vec2<$T>) -> (Self::Output, bool) {
                let (x, overflow_x) = self.overflowing_div(rhs.x);
                let (y, overflow_y) = self.overflowing_div(rhs.y);
                (Vec2 { x, y }, overflow_x || overflow_y)
            }
        }
    )+};
}

impl_lhs_vec_discrete_arith!{
    i8, i16, i32, i64, i128, isize,
    u8, u16, u32, u64, u128, usize,
}

// -----------------------------------------------------------------------------------------
// Vec3
// -----------------------------------------------------------------------------------------

#[must_use]
pub struct Vec3<T = f32> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T: Arithmetic + Min<Output = T> + Max<Output = T>> Vector for Vec3<T> {
    type Item = T;
    type Rect = Rect3<T>;
    #[inline]
    fn max(self) -> T {
        self.x.min(self.y).min(self.z)
    }
    #[inline]
    fn min(self) -> T {
        self.x.max(self.y).max(self.z)
    }
    #[inline]
    fn sum(self) -> T {
        self.sum()
    }
    #[inline]
    fn prod(self) -> T {
        self.prod()
    }
    #[inline]
    fn dot(self, other: Self) -> T {
        (self * other).sum()
    }
    #[inline]
    fn mag_sqr(self) -> T {
        self.dot(self)
    }
    #[inline]
    fn mag(self) -> T where T: Sqrt<Output = T> {
        self.mag_sqr().sqrt()
    }
    #[inline]
    fn dist_sqr(self, other: Self) -> T {
        self.dist_sqr(other)
    }
    #[inline]
    fn dist(self, other: Self) -> T where T: Sqrt<Output = T> {
        self.dist(other)
    }
    #[inline]
    fn unit(self) -> Self where T: Sqrt<Output = T> {
        self.unit()
    }
    #[inline]
    fn dir(self, other: Self) -> Self where T: Sqrt<Output = T> {
        self.dir(other)
    }
}

pub type IVec3 = Vec3<i32>;

impl<T> Vec3<T> {
    #[inline]
    pub const fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }

    /// Returns the greatest of the vector's elements
    #[inline]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub fn max(self) -> T where T: Ord {
        self.x.max(self.y).max(self.z)
    }

    /// Returns the smallest of the vector's elements
    #[inline]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub fn min(self) -> T where T: Ord {
        self.x.min(self.y).min(self.z)
    }

    /// Returns the sum of the vector's elements
    #[inline]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub fn sum(self) -> T
    where
        T: Add<Output = T>
    {
        self.x + self.y + self.z
    }

    /// Returns the product of the vector's elements
    #[inline]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub fn prod(self) -> T
    where
        T: Mul<Output = T>
    {
        self.x * self.y * self.z
    }

    /// Returns the dot product of two vectors
    #[inline]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub fn dot(self, other: Self) -> T
    where
        T:
            Mul<Output = T> +
            Add<Output = T>
    {
        (self * other).sum()
    }

    /// Returns the square of the magnitude of the vector
    #[inline]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub fn mag_sqr(self) -> T
    where
        T:
            Copy +
            Add<Output = T> +
            Mul<Output = T>
    {
        self.dot(self)
    }

    /// Returns the square of the magnitude of the vector
    #[inline]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub fn mag(self) -> <T as Sqrt>::Output
    where
        T:
            Copy +
            Add<Output = T> +
            Mul<Output = T> +
            Sqrt
    {
        self.mag_sqr().sqrt()
    }

    /// Returns the square of the magnitude of the vector
    #[inline]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub fn dist_sqr(self, other: Self) -> T
    where
        T:
            Copy +
            Add<Output = T> +
            Sub<Output = T> +
            Mul<Output = T>
    {
        (other - self).mag_sqr()
    }

    /// Returns the square of the magnitude of the vector
    #[inline]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub fn dist(self, other: Self) -> <T as Sqrt>::Output
    where
        T:
            Copy +
            Add<Output = T> +
            Sub<Output = T> +
            Mul<Output = T> +
            Sqrt
    {
        self.dist_sqr(other).sqrt()
    }

    /// Returns the normalized unit vector
    #[inline]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub fn unit(self) -> Self
    where
        Self:
            Div<<T as Sqrt>::Output, Output = Self>,
        T:
            Copy +
            Add<Output = T> +
            Mul<Output = T> +
            Sqrt
    {
        self / self.mag()
    }

    /// Returns the normalized direction vector from `self` to `other`
    #[inline]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub fn dir(self, other: Self) -> Self
    where
        Self:
            Div<<T as Sqrt>::Output, Output = Self>,
        T:
            Copy +
            Add<Output = T> +
            Sub<Output = T> +
            Mul<Output = T> +
            Sqrt
    {
        (other - self).unit()
    }
}

impl<T> IntoIterator for Vec3<T> {
    type Item = T;
    type IntoIter = <[T; 3] as IntoIterator>::IntoIter;
    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        <[T; 3]>::from(self).into_iter()
    }
}

impl<T: Ord> ElementwiseOrd for Vec3<T> {
    #[inline]
    fn clamp(self, min: Self, max: Self) -> Self {
        Self {
            x: self.x.clamp(min.x, max.x),
            y: self.y.clamp(min.y, max.y),
            z: self.z.clamp(min.z, max.z),
        }
    }
    #[inline]
    fn max(self, other: Self) -> Self {
        Self {
            x: self.x.max(other.x),
            y: self.y.max(other.y),
            z: self.z.max(other.z),
        }
    }
    #[inline]
    fn min(self, other: Self) -> Self {
        Self {
            x: self.x.min(other.x),
            y: self.y.min(other.y),
            z: self.z.min(other.z),
        }
    }
}

impl<T: Ord + Copy> ElementwiseOrd<T> for Vec3<T> {
    #[inline]
    fn clamp(self, min: T, max: T) -> Self {
        Self {
            x: self.x.clamp(min, max),
            y: self.y.clamp(min, max),
            z: self.z.clamp(min, max),
        }
    }
    #[inline]
    fn max(self, other: T) -> Self {
        Self {
            x: self.x.max(other),
            y: self.y.max(other),
            z: self.z.max(other),
        }
    }
    #[inline]
    fn min(self, other: T) -> Self {
        Self {
            x: self.x.min(other),
            y: self.y.min(other),
            z: self.z.min(other),
        }
    }
}

impl<T: Midpoint> Midpoint for Vec3<T> {
    type Output = Vec3<T::Output>;
    #[inline]
    fn midpoint(self, other: Self) -> Self::Output {
        Vec3 {
            x: self.x.midpoint(other.x),
            y: self.y.midpoint(other.y),
            z: self.z.midpoint(other.z),
        }
    }
}

impl<T: ~const Min<U> + Copy, U: Copy> const Min<Vec3<U>> for Vec3<T> {
    type Output = Vec3<T::Output>;
    #[inline]
    fn min(self, other: Vec3<U>) -> Self::Output {
        Vec3 {
            x: self.x.min(other.x),
            y: self.y.min(other.y),
            z: self.z.min(other.z),
        }
    }
}

impl<T: ~const Max<U> + Copy, U: Copy> const Max<Vec3<U>> for Vec3<T> {
    type Output = Vec3<T::Output>;
    #[inline]
    fn max(self, other: Vec3<U>) -> Self::Output {
        Vec3 {
            x: self.x.max(other.x),
            y: self.y.max(other.y),
            z: self.z.max(other.z),
        }
    }
}

impl<T: ~const MinMax<Output: Copy> + Copy> const MinMax for Vec3<T> {
    type Output = Vec3<T::Output>;
    #[inline]
    fn minmax(self, other: Self) -> (Self::Output, Self::Output) {
        let (xmin, xmax) = self.x.minmax(other.x);
        let (ymin, ymax) = self.y.minmax(other.y);
        let (zmin, zmax) = self.z.minmax(other.z);
        (Vec3 { x: xmin, y: ymin, z: zmin }, Vec3 { x: xmax, y: ymax, z: zmax })
    }
}

impl<T: ~const Clamp<U, V, Output: Copy> + Copy, U: Copy, V: Copy> const Clamp<Vec3<U>, Vec3<V>> for Vec3<T> {
    type Output = Vec3<T::Output>;
    #[inline]
    fn clamp(self, min: Vec3<U>, max: Vec3<V>) -> Self::Output {
        Vec3 {
            x: self.x.clamp(min.x, max.x),
            y: self.y.clamp(min.y, max.y),
            z: self.z.clamp(min.z, max.z),
        }
    }
}

impl<T: PartialOrd> PartialOrd for Vec3<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        use std::cmp::Ordering;
        match self.x.partial_cmp(&other.x).zip(self.y.partial_cmp(&other.y)) {
            Some((Ordering::Less, Ordering::Less | Ordering::Equal) | (Ordering::Equal, Ordering::Less)) => Some(Ordering::Less),
            Some((Ordering::Equal, Ordering::Equal)) => Some(Ordering::Equal),
            Some((Ordering::Greater, Ordering::Greater | Ordering::Equal) | (Ordering::Equal, Ordering::Greater)) => Some(Ordering::Greater),
            Some((Ordering::Greater, Ordering::Less) | (Ordering::Less, Ordering::Greater)) | None => None,
        }
    }
}

impl<T> Index<usize> for Vec3<T> {
    type Output = T;
    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            3.. => panic!("Out of bounds"),
        }
    }
}

impl<T> IndexMut<usize> for Vec3<T> {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            3.. => panic!("Out of bounds"),
        }
    }
}

impl From<Vector3> for Vec3<f32> {
    #[inline]
    fn from(Vector3 { x, y, z }: Vector3) -> Self {
        Self { x, y, z }
    }
}

impl From<Vec3<f32>> for Vector3 {
    #[inline]
    fn from(Vec3 { x, y, z }: Vec3<f32>) -> Self {
        Self { x, y, z }
    }
}

impl<T: Copy> From<T> for Vec3<T> {
    #[inline]
    fn from(xyz: T) -> Self {
        Self { x: xyz, y: xyz, z: xyz }
    }
}

impl<T> From<(T, T, T)> for Vec3<T> {
    #[inline]
    fn from((x, y, z): (T, T, T)) -> Self {
        Self { x, y, z }
    }
}
impl<T> From<Vec3<T>> for (T, T, T) {
    #[inline]
    fn from(v: Vec3<T>) -> Self {
        (v.x, v.y, v.z)
    }
}

impl<T> From<[T; 3]> for Vec3<T> {
    #[inline]
    fn from([x, y, z]: [T; 3]) -> Self {
        Self { x, y, z }
    }
}
impl<T> From<Vec3<T>> for [T; 3] {
    #[inline]
    fn from(v: Vec3<T>) -> Self {
        [v.x, v.y, v.z]
    }
}

impl<T: std::fmt::Debug> std::fmt::Debug for Vec3<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Vec3")
            .field("x", &self.x)
            .field("y", &self.y)
            .field("z", &self.z)
            .finish()
    }
}

impl<T: Default> Default for Vec3<T> {
    #[inline]
    fn default() -> Self {
        Self {
            x: Default::default(),
            y: Default::default(),
            z: Default::default(),
        }
    }
}

impl<T: Clone> Clone for Vec3<T> {
    #[inline]
    fn clone(&self) -> Self {
        Self {
            x: self.x.clone(),
            y: self.y.clone(),
            z: self.z.clone(),
        }
    }
}

impl<T: Copy> Copy for Vec3<T> {}

impl<T: std::hash::Hash> std::hash::Hash for Vec3<T> {
    #[inline]
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
        self.z.hash(state);
    }
}

impl<T: PartialEq> PartialEq for Vec3<T> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

impl<T: Eq> Eq for Vec3<T> {}

macro_rules! impl_elementwise_unary {
    ($($Trait:ident::$fn:ident),+ $(,)?) => {$(
        impl<T: $Trait> $Trait for Vec3<T> {
            type Output = Vec3<T::Output>;
            #[inline]
            fn $fn(self) -> Self::Output {
                Vec3 {
                    x: self.x.$fn(),
                    y: self.y.$fn(),
                    z: self.z.$fn(),
                }
            }
        }
    )+};
}

impl_elementwise_unary!{
    Neg::neg,
    Not::not,
    Abs::abs,
    WrappingAbs::wrapping_abs,
    SaturatingAbs::saturating_abs,
}

impl<T: CheckedAbs> CheckedAbs for Vec3<T> {
    type Output = Vec3<T::Output>;
    fn checked_abs(self) -> Option<Self::Output> {
        if let (Some(x), Some(y), Some(z)) = (self.x.checked_abs(), self.y.checked_abs(), self.z.checked_abs()) {
            Some(Vec3 { x, y, z })
        } else { None }
    }
}

impl<T: OverflowingAbs> OverflowingAbs for Vec3<T> {
    type Output = Vec3<T::Output>;
    fn overflowing_abs(self) -> (Self::Output, bool) {
        let (x, overflow_x) = self.x.overflowing_abs();
        let (y, overflow_y) = self.y.overflowing_abs();
        let (z, overflow_z) = self.z.overflowing_abs();
        (Vec3 { x, y, z }, overflow_x || overflow_y || overflow_z)
    }
}

macro_rules! impl_elementwise {
    ($($Trait:ident::$fn:ident$( = $TraitAssign:ident::$fn_assign:ident)?),+ $(,)?) => {$(
        impl<T: $Trait> $Trait for Vec3<T> {
            type Output = Vec3<<T as $Trait>::Output>;
            #[inline]
            fn $fn(self, rhs: Self) -> Self::Output {
                Vec3 {
                    x: self.x.$fn(rhs.x),
                    y: self.y.$fn(rhs.y),
                    z: self.z.$fn(rhs.z),
                }
            }
        }
        impl<T: Copy + $Trait<T>> $Trait<T> for Vec3<T> {
            type Output = Vec3<<T as $Trait<T>>::Output>;
            #[inline]
            fn $fn(self, rhs: T) -> Self::Output {
                Vec3 {
                    x: self.x.$fn(rhs),
                    y: self.y.$fn(rhs),
                    z: self.z.$fn(rhs),
                }
            }
        }
        $(impl<T: $Trait<Output = T> + Copy> $TraitAssign for Vec3<T> {
            #[inline]
            fn $fn_assign(&mut self, rhs: Self) {
                *self = (*self).$fn(rhs);
            }
        }
        impl<T: $Trait<Output = T> + Copy> $TraitAssign<T> for Vec3<T> {
            #[inline]
            fn $fn_assign(&mut self, rhs: T) {
                *self = (*self).$fn(rhs);
            }
        })?
    )+};
}

impl_elementwise!{
    Add::add = AddAssign::add_assign,
      WrappingAdd::  wrapping_add,
    SaturatingAdd::saturating_add,
    Sub::sub = SubAssign::sub_assign,
      WrappingSub::  wrapping_sub,
    SaturatingSub::saturating_sub,
    Mul::mul = MulAssign::mul_assign,
      WrappingMul::  wrapping_mul,
    SaturatingMul::saturating_mul,
    Div::div = DivAssign::div_assign,
      WrappingDiv::  wrapping_div,
    SaturatingDiv::saturating_div,
    Rem   ::rem    = RemAssign   ::rem_assign   ,
    BitAnd::bitand = BitAndAssign::bitand_assign,
    BitOr ::bitor  = BitOrAssign ::bitor_assign ,
    BitXor::bitxor = BitXorAssign::bitxor_assign,
    Shl   ::shl    = ShlAssign   ::shl_assign   ,
    Shr   ::shr    = ShrAssign   ::shr_assign   ,
}

macro_rules! impl_elementwise_checked {
    ($($Trait:ident::$fn:ident),+ $(,)?) => {$(
        impl<T: $Trait> $Trait for Vec3<T> {
            type Output = Vec3<T::Output>;
            #[inline]
            fn $fn(self, rhs: Self) -> Option<Self::Output> {
                if let (Some(x), Some(y), Some(z)) = (self.x.$fn(rhs.x), self.y.$fn(rhs.y), self.z.$fn(rhs.z)) {
                    Some(Vec3 { x, y, z })
                } else { None }
            }
        }
        impl<T: Copy + $Trait> $Trait<T> for Vec3<T> {
            type Output = Vec3<T::Output>;
            #[inline]
            fn $fn(self, rhs: T) -> Option<Self::Output> {
                if let (Some(x), Some(y), Some(z)) = (self.x.$fn(rhs), self.y.$fn(rhs), self.z.$fn(rhs)) {
                    Some(Vec3 { x, y, z })
                } else { None }
            }
        }
    )+};
}

impl_elementwise_checked!{
    CheckedAdd::checked_add,
    CheckedSub::checked_sub,
    CheckedMul::checked_mul,
    CheckedDiv::checked_div,
}

macro_rules! impl_elementwise_overflowing {
    ($($Trait:ident::$fn:ident),+ $(,)?) => {$(
        impl<T: $Trait> $Trait for Vec3<T> {
            type Output = Vec3<T::Output>;
            #[inline]
            fn $fn(self, rhs: Self) -> (Self::Output, bool) {
                let (x, overflow_x) = self.x.$fn(rhs.x);
                let (y, overflow_y) = self.y.$fn(rhs.y);
                let (z, overflow_z) = self.z.$fn(rhs.z);
                (Vec3 { x, y, z }, overflow_x || overflow_y || overflow_z)
            }
        }
        impl<T: Copy + $Trait> $Trait<T> for Vec3<T> {
            type Output = Vec3<T::Output>;
            #[inline]
            fn $fn(self, rhs: T) -> (Self::Output, bool) {
                let (x, overflow_x) = self.x.$fn(rhs);
                let (y, overflow_y) = self.y.$fn(rhs);
                let (z, overflow_z) = self.z.$fn(rhs);
                (Vec3 { x, y, z }, overflow_x || overflow_y || overflow_z)
            }
        }
    )+};
}

impl_elementwise_overflowing!{
    OverflowingAdd::overflowing_add,
    OverflowingSub::overflowing_sub,
    OverflowingMul::overflowing_mul,
    OverflowingDiv::overflowing_div,
}

impl<T: Arithmetic> Arithmetic for Vec3<T> {
    const ZERO: Self = Self { x: T::ZERO, y: T::ZERO, z: T::ZERO };
    const ONE: Self = Self { x: T::ONE, y: T::ONE, z: T::ONE };
    #[inline]
    fn half(self) -> Self {
        Self { x: self.x.half(), y: self.y.half(), z: self.z.half() }
    }
}

macro_rules! impl_lhs_vec_arith {
    ($($T:ty),+ $(,)?) => {$(
        impl Add<Vec3<$T>> for $T {
            type Output = Vec3<$T>;
            #[inline]
            fn add(self, rhs: Vec3<$T>) -> Self::Output {
                Vec3 {
                    x: self + rhs.x,
                    y: self + rhs.y,
                    z: self + rhs.z,
                }
            }
        }
        impl Sub<Vec3<$T>> for $T {
            type Output = Vec3<$T>;
            #[inline]
            fn sub(self, rhs: Vec3<$T>) -> Self::Output {
                Vec3 {
                    x: self - rhs.x,
                    y: self - rhs.y,
                    z: self - rhs.z,
                }
            }
        }
        impl Mul<Vec3<$T>> for $T {
            type Output = Vec3<$T>;
            #[inline]
            fn mul(self, rhs: Vec3<$T>) -> Self::Output {
                Vec3 {
                    x: self * rhs.x,
                    y: self * rhs.y,
                    z: self * rhs.z,
                }
            }
        }
        impl Div<Vec3<$T>> for $T {
            type Output = Vec3<$T>;
            #[inline]
            fn div(self, rhs: Vec3<$T>) -> Self::Output {
                Vec3 {
                    x: self / rhs.x,
                    y: self / rhs.y,
                    z: self / rhs.z,
                }
            }
        }
    )+};
}

impl_lhs_vec_arith!{
    i8, i16, i32, i64, i128, isize,
    u8, u16, u32, u64, u128, usize,
    f32, f64,
}

macro_rules! impl_lhs_vec_discrete_arith {
    ($($T:ty),+ $(,)?) => {$(
        impl BitAnd<Vec3<$T>> for $T {
            type Output = Vec3<$T>;
            #[inline]
            fn bitand(self, rhs: Vec3<$T>) -> Self::Output {
                Vec3 {
                    x: self & rhs.x,
                    y: self & rhs.y,
                    z: self & rhs.z,
                }
            }
        }
        impl BitOr<Vec3<$T>> for $T {
            type Output = Vec3<$T>;
            #[inline]
            fn bitor(self, rhs: Vec3<$T>) -> Self::Output {
                Vec3 {
                    x: self | rhs.x,
                    y: self | rhs.y,
                    z: self | rhs.z,
                }
            }
        }
        impl BitXor<Vec3<$T>> for $T {
            type Output = Vec3<$T>;
            #[inline]
            fn bitxor(self, rhs: Vec3<$T>) -> Self::Output {
                Vec3 {
                    x: self ^ rhs.x,
                    y: self ^ rhs.y,
                    z: self ^ rhs.z,
                }
            }
        }
        impl Rem<Vec3<$T>> for $T {
            type Output = Vec3<$T>;
            #[inline]
            fn rem(self, rhs: Vec3<$T>) -> Self::Output {
                Vec3 {
                    x: self % rhs.x,
                    y: self % rhs.y,
                    z: self % rhs.z,
                }
            }
        }
        impl Shl<Vec3<$T>> for $T {
            type Output = Vec3<$T>;
            #[inline]
            fn shl(self, rhs: Vec3<$T>) -> Self::Output {
                Vec3 {
                    x: self << rhs.x,
                    y: self << rhs.y,
                    z: self << rhs.z,
                }
            }
        }
        impl Shr<Vec3<$T>> for $T {
            type Output = Vec3<$T>;
            #[inline]
            fn shr(self, rhs: Vec3<$T>) -> Self::Output {
                Vec3 {
                    x: self >> rhs.x,
                    y: self >> rhs.y,
                    z: self >> rhs.z,
                }
            }
        }
        impl CheckedAdd<Vec3<$T>> for $T {
            type Output = Vec3<$T>;
            #[inline]
            fn checked_add(self, rhs: Vec3<$T>) -> Option<Self::Output> {
                if let (Some(x), Some(y), Some(z)) = (self.checked_add(rhs.x), self.checked_add(rhs.y), self.checked_add(rhs.z)) {
                    Some(Vec3 { x, y, z })
                } else { None }
            }
        }
        impl CheckedSub<Vec3<$T>> for $T {
            type Output = Vec3<$T>;
            #[inline]
            fn checked_sub(self, rhs: Vec3<$T>) -> Option<Self::Output> {
                if let (Some(x), Some(y), Some(z)) = (self.checked_sub(rhs.x), self.checked_sub(rhs.y), self.checked_sub(rhs.z)) {
                    Some(Vec3 { x, y, z })
                } else { None }
            }
        }
        impl CheckedMul<Vec3<$T>> for $T {
            type Output = Vec3<$T>;
            #[inline]
            fn checked_mul(self, rhs: Vec3<$T>) -> Option<Self::Output> {
                if let (Some(x), Some(y), Some(z)) = (self.checked_mul(rhs.x), self.checked_mul(rhs.y), self.checked_mul(rhs.z)) {
                    Some(Vec3 { x, y, z })
                } else { None }
            }
        }
        impl CheckedDiv<Vec3<$T>> for $T {
            type Output = Vec3<$T>;
            #[inline]
            fn checked_div(self, rhs: Vec3<$T>) -> Option<Self::Output> {
                if let (Some(x), Some(y), Some(z)) = (self.checked_div(rhs.x), self.checked_div(rhs.y), self.checked_div(rhs.z)) {
                    Some(Vec3 { x, y, z })
                } else { None }
            }
        }
        impl WrappingAdd<Vec3<$T>> for $T {
            type Output = Vec3<$T>;
            #[inline]
            fn wrapping_add(self, rhs: Vec3<$T>) -> Self::Output {
                Vec3 {
                    x: self.wrapping_add(rhs.x),
                    y: self.wrapping_add(rhs.y),
                    z: self.wrapping_add(rhs.z),
                }
            }
        }
        impl WrappingSub<Vec3<$T>> for $T {
            type Output = Vec3<$T>;
            #[inline]
            fn wrapping_sub(self, rhs: Vec3<$T>) -> Self::Output {
                Vec3 {
                    x: self.wrapping_sub(rhs.x),
                    y: self.wrapping_sub(rhs.y),
                    z: self.wrapping_sub(rhs.z),
                }
            }
        }
        impl WrappingMul<Vec3<$T>> for $T {
            type Output = Vec3<$T>;
            #[inline]
            fn wrapping_mul(self, rhs: Vec3<$T>) -> Self::Output {
                Vec3 {
                    x: self.wrapping_mul(rhs.x),
                    y: self.wrapping_mul(rhs.y),
                    z: self.wrapping_mul(rhs.z),
                }
            }
        }
        impl WrappingDiv<Vec3<$T>> for $T {
            type Output = Vec3<$T>;
            #[inline]
            fn wrapping_div(self, rhs: Vec3<$T>) -> Self::Output {
                Vec3 {
                    x: self.wrapping_div(rhs.x),
                    y: self.wrapping_div(rhs.y),
                    z: self.wrapping_div(rhs.z),
                }
            }
        }
        impl SaturatingAdd<Vec3<$T>> for $T {
            type Output = Vec3<$T>;
            #[inline]
            fn saturating_add(self, rhs: Vec3<$T>) -> Self::Output {
                Vec3 {
                    x: self.saturating_add(rhs.x),
                    y: self.saturating_add(rhs.y),
                    z: self.saturating_add(rhs.z),
                }
            }
        }
        impl SaturatingSub<Vec3<$T>> for $T {
            type Output = Vec3<$T>;
            #[inline]
            fn saturating_sub(self, rhs: Vec3<$T>) -> Self::Output {
                Vec3 {
                    x: self.saturating_sub(rhs.x),
                    y: self.saturating_sub(rhs.y),
                    z: self.saturating_sub(rhs.z),
                }
            }
        }
        impl SaturatingMul<Vec3<$T>> for $T {
            type Output = Vec3<$T>;
            #[inline]
            fn saturating_mul(self, rhs: Vec3<$T>) -> Self::Output {
                Vec3 {
                    x: self.saturating_mul(rhs.x),
                    y: self.saturating_mul(rhs.y),
                    z: self.saturating_mul(rhs.z),
                }
            }
        }
        impl SaturatingDiv<Vec3<$T>> for $T {
            type Output = Vec3<$T>;
            #[inline]
            fn saturating_div(self, rhs: Vec3<$T>) -> Self::Output {
                Vec3 {
                    x: self.saturating_div(rhs.x),
                    y: self.saturating_div(rhs.y),
                    z: self.saturating_div(rhs.z),
                }
            }
        }
        impl OverflowingAdd<Vec3<$T>> for $T {
            type Output = Vec3<$T>;
            #[inline]
            fn overflowing_add(self, rhs: Vec3<$T>) -> (Self::Output, bool) {
                let (x, overflow_x) = self.overflowing_add(rhs.x);
                let (y, overflow_y) = self.overflowing_add(rhs.y);
                let (z, overflow_z) = self.overflowing_add(rhs.z);
                (Vec3 { x, y, z }, overflow_x || overflow_y || overflow_z)
            }
        }
        impl OverflowingSub<Vec3<$T>> for $T {
            type Output = Vec3<$T>;
            #[inline]
            fn overflowing_sub(self, rhs: Vec3<$T>) -> (Self::Output, bool) {
                let (x, overflow_x) = self.overflowing_sub(rhs.x);
                let (y, overflow_y) = self.overflowing_sub(rhs.y);
                let (z, overflow_z) = self.overflowing_sub(rhs.z);
                (Vec3 { x, y, z }, overflow_x || overflow_y || overflow_z)
            }
        }
        impl OverflowingMul<Vec3<$T>> for $T {
            type Output = Vec3<$T>;
            #[inline]
            fn overflowing_mul(self, rhs: Vec3<$T>) -> (Self::Output, bool) {
                let (x, overflow_x) = self.overflowing_mul(rhs.x);
                let (y, overflow_y) = self.overflowing_mul(rhs.y);
                let (z, overflow_z) = self.overflowing_mul(rhs.z);
                (Vec3 { x, y, z }, overflow_x || overflow_y || overflow_z)
            }
        }
        impl OverflowingDiv<Vec3<$T>> for $T {
            type Output = Vec3<$T>;
            #[inline]
            fn overflowing_div(self, rhs: Vec3<$T>) -> (Self::Output, bool) {
                let (x, overflow_x) = self.overflowing_div(rhs.x);
                let (y, overflow_y) = self.overflowing_div(rhs.y);
                let (z, overflow_z) = self.overflowing_div(rhs.z);
                (Vec3 { x, y, z }, overflow_x || overflow_y || overflow_z)
            }
        }
    )+};
}

impl_lhs_vec_discrete_arith!{
    i8, i16, i32, i64, i128, isize,
    u8, u16, u32, u64, u128, usize,
}
