use std::{mem, ops::*};
use crate::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct IVector2 {
    pub x: i32,
    pub y: i32,
}

impl From<IVector2> for [i32; 2] {
    #[inline]
    fn from(IVector2 { x, y }: IVector2) -> Self {
        [x, y]
    }
}

impl From<[i32; 2]> for IVector2 {
    #[inline]
    fn from([x, y]: [i32; 2]) -> Self {
        IVector2 { x, y }
    }
}

impl From<IVector2> for (i32, i32) {
    #[inline]
    fn from(IVector2 { x, y }: IVector2) -> Self {
        (x, y)
    }
}

impl From<(i32, i32)> for IVector2 {
    #[inline]
    fn from((x, y): (i32, i32)) -> Self {
        IVector2 { x, y }
    }
}

impl TryFrom<UVector2> for IVector2 {
    type Error = std::num::TryFromIntError;
    /// Returns an error if x or y is greater than [`i32::MAX`]
    fn try_from(UVector2 { x, y }: UVector2) -> Result<Self, Self::Error> {
        Ok(Self {
            x: x.try_into()?,
            y: y.try_into()?,
        })
    }
}

impl IVector2 {
    pub const MIN: Self = Self { x: i32::MIN, y: i32::MIN };
    pub const MAX: Self = Self { x: i32::MAX, y: i32::MAX };
    pub const ZERO: Self = Self { x: 0, y: 0 };
    pub const ONE: Self = Self { x: 1, y: 1 };

    #[inline]
    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    #[inline]
    pub const fn as_uvec2(self) -> UVector2 {
        UVector2 {
            x: self.x as u32,
            y: self.y as u32,
        }
    }

    #[inline]
    pub const fn as_vec2(self) -> Vector2 {
        Vector2 {
            x: self.x as f32,
            y: self.y as f32,
        }
    }

    #[inline]
    pub const fn max_element(self) -> i32 {
        if self.x <= self.y { self.y } else { self.x }
    }

    #[inline]
    pub const fn min_element(self) -> i32 {
        if self.x <= self.y { self.y } else { self.x }
    }

    #[inline]
    pub const fn sum(self) -> i32 {
        self.x + self.y
    }

    #[inline]
    pub const fn prod(self) -> i32 {
        self.x * self.y
    }

    #[inline]
    pub const fn delta(self, other: Self) -> Self {
        Self {
            x: other.x - self.x,
            y: other.y - self.y,
        }
    }

    #[inline]
    pub const fn count_ones(self) -> UVector2 {
        UVector2 {
            x: self.x.count_ones(),
            y: self.y.count_ones(),
        }
    }

    #[inline]
    pub const fn count_zeros(self) -> UVector2 {
        UVector2 {
            x: self.x.count_zeros(),
            y: self.y.count_zeros(),
        }
    }

    #[inline]
    pub const fn leading_zeros(self) -> UVector2 {
        UVector2 {
            x: self.x.leading_zeros(),
            y: self.y.leading_zeros(),
        }
    }

    #[inline]
    pub const fn trailing_zeros(self) -> UVector2 {
        UVector2 {
            x: self.x.trailing_zeros(),
            y: self.y.trailing_zeros(),
        }
    }

    #[inline]
    pub const fn leading_ones(self) -> UVector2 {
        UVector2 {
            x: self.x.leading_ones(),
            y: self.y.leading_ones(),
        }
    }

    #[inline]
    pub const fn trailing_ones(self) -> UVector2 {
        UVector2 {
            x: self.x.trailing_ones(),
            y: self.y.trailing_ones(),
        }
    }

    #[inline]
    pub const fn rotate_left_v(self, n: UVector2) -> Self {
        Self {
            x: self.x.rotate_left(n.x),
            y: self.y.rotate_left(n.y),
        }
    }

    #[inline]
    pub const fn rotate_left(self, n: u32) -> Self {
        Self {
            x: self.x.rotate_left(n),
            y: self.y.rotate_left(n),
        }
    }

    #[inline]
    pub const fn rotate_right_v(self, n: UVector2) -> Self {
        Self {
            x: self.x.rotate_right(n.x),
            y: self.y.rotate_right(n.y),
        }
    }

    #[inline]
    pub const fn rotate_right(self, n: u32) -> Self {
        Self {
            x: self.x.rotate_right(n),
            y: self.y.rotate_right(n),
        }
    }

    #[inline]
    pub const fn swap_bytes(self) -> Self {
        Self {
            x: self.y.swap_bytes(),
            y: self.x.swap_bytes(),
        }
    }

    #[inline]
    pub const fn reverse_bits(self) -> Self {
        Self {
            x: self.y.reverse_bits(),
            y: self.x.reverse_bits(),
        }
    }

    #[inline]
    pub const fn from_be(v: Self) -> Self {
        #[cfg(target_endian = "big")]
        {
            v
        }
        #[cfg(not(target_endian = "big"))]
        {
            v.swap_bytes()
        }
    }

    #[inline]
    pub const fn from_le(v: Self) -> Self {
        #[cfg(target_endian = "little")]
        {
            v
        }
        #[cfg(not(target_endian = "little"))]
        {
            v.swap_bytes()
        }
    }

    #[inline]
    pub const fn to_be(self) -> Self {
        #[cfg(target_endian = "big")]
        {
            self
        }
        #[cfg(not(target_endian = "big"))]
        {
            self.swap_bytes()
        }
    }

    #[inline]
    pub const fn to_le(self) -> Self {
        #[cfg(target_endian = "little")]
        {
            self
        }
        #[cfg(not(target_endian = "little"))]
        {
            self.swap_bytes()
        }
    }

    #[inline]
    pub const fn checked_add_v(self, rhs: Self) -> Option<Self> {
        let x = self.x.checked_add(rhs.x);
        let y = self.y.checked_add(rhs.y);
        if let (Some(x), Some(y)) = (x, y) {
            Some(Self { x, y })
        } else { None }
    }

    #[inline]
    pub const fn checked_add(self, rhs: i32) -> Option<Self> {
        let x = self.x.checked_add(rhs);
        let y = self.y.checked_add(rhs);
        if let (Some(x), Some(y)) = (x, y) {
            Some(Self { x, y })
        } else { None }
    }

    #[inline]
    pub const unsafe fn unchecked_add(self, rhs: Self) -> Self {
        Self {
            x: self.x.unchecked_add(rhs.x),
            y: self.y.unchecked_add(rhs.y),
        }
    }

    #[inline]
    pub const fn checked_add_unsigned_v(self, rhs: UVector2) -> Option<Self> {
        let x = self.x.checked_add_unsigned(rhs.x);
        let y = self.y.checked_add_unsigned(rhs.y);
        if let (Some(x), Some(y)) = (x, y) {
            Some(Self { x, y })
        } else { None }
    }

    #[inline]
    pub const fn checked_add_unsigned(self, rhs: u32) -> Option<Self> {
        let x = self.x.checked_add_unsigned(rhs);
        let y = self.y.checked_add_unsigned(rhs);
        if let (Some(x), Some(y)) = (x, y) {
            Some(Self { x, y })
        } else { None }
    }

    #[inline]
    pub const fn checked_sub_v(self, rhs: Self) -> Option<Self> {
        let x = self.x.checked_sub(rhs.x);
        let y = self.y.checked_sub(rhs.y);
        if let (Some(x), Some(y)) = (x, y) {
            Some(Self { x, y })
        } else { None }
    }

    #[inline]
    pub const fn checked_sub(self, rhs: i32) -> Option<Self> {
        let x = self.x.checked_sub(rhs);
        let y = self.y.checked_sub(rhs);
        if let (Some(x), Some(y)) = (x, y) {
            Some(Self { x, y })
        } else { None }
    }

    #[inline]
    pub const unsafe fn unchecked_sub(self, rhs: Self) -> Self {
        Self {
            x: self.x.unchecked_sub(rhs.x),
            y: self.y.unchecked_sub(rhs.y),
        }
    }

    #[inline]
    pub const fn checked_sub_unsigned_v(self, rhs: UVector2) -> Option<Self> {
        let x = self.x.checked_sub_unsigned(rhs.x);
        let y = self.y.checked_sub_unsigned(rhs.y);
        if let (Some(x), Some(y)) = (x, y) {
            Some(Self { x, y })
        } else { None }
    }

    #[inline]
    pub const fn checked_sub_unsigned(self, rhs: u32) -> Option<Self> {
        let x = self.x.checked_sub_unsigned(rhs);
        let y = self.y.checked_sub_unsigned(rhs);
        if let (Some(x), Some(y)) = (x, y) {
            Some(Self { x, y })
        } else { None }
    }

    #[inline]
    pub const fn checked_mul_v(self, rhs: Self) -> Option<Self> {
        let x = self.x.checked_mul(rhs.x);
        let y = self.y.checked_mul(rhs.y);
        if let (Some(x), Some(y)) = (x, y) {
            Some(Self { x, y })
        } else { None }
    }

    #[inline]
    pub const fn checked_mul(self, rhs: i32) -> Option<Self> {
        let x = self.x.checked_mul(rhs);
        let y = self.y.checked_mul(rhs);
        if let (Some(x), Some(y)) = (x, y) {
            Some(Self { x, y })
        } else { None }
    }

    #[inline]
    pub const unsafe fn unchecked_mul(self, rhs: Self) -> Self {
        Self {
            x: self.x.unchecked_mul(rhs.x),
            y: self.y.unchecked_mul(rhs.y),
        }
    }

    #[inline]
    pub const fn checked_div_v(self, rhs: Self) -> Option<Self> {
        let x = self.x.checked_div(rhs.x);
        let y = self.y.checked_div(rhs.y);
        if let (Some(x), Some(y)) = (x, y) {
            Some(Self { x, y })
        } else { None }
    }

    #[inline]
    pub const fn checked_div(self, rhs: i32) -> Option<Self> {
        let x = self.x.checked_div(rhs);
        let y = self.y.checked_div(rhs);
        if let (Some(x), Some(y)) = (x, y) {
            Some(Self { x, y })
        } else { None }
    }

    #[inline]
    pub const fn checked_div_euclid_v(self, rhs: Self) -> Option<Self> {
        let x = self.x.checked_div_euclid(rhs.x);
        let y = self.y.checked_div_euclid(rhs.y);
        if let (Some(x), Some(y)) = (x, y) {
            Some(Self { x, y })
        } else { None }
    }

    #[inline]
    pub const fn checked_div_euclid(self, rhs: i32) -> Option<Self> {
        let x = self.x.checked_div_euclid(rhs);
        let y = self.y.checked_div_euclid(rhs);
        if let (Some(x), Some(y)) = (x, y) {
            Some(Self { x, y })
        } else { None }
    }

    #[inline]
    pub const fn checked_rem_v(self, rhs: Self) -> Option<Self> {
        let x = self.x.checked_rem(rhs.x);
        let y = self.y.checked_rem(rhs.y);
        if let (Some(x), Some(y)) = (x, y) {
            Some(Self { x, y })
        } else { None }
    }

    #[inline]
    pub const fn checked_rem(self, rhs: i32) -> Option<Self> {
        let x = self.x.checked_rem(rhs);
        let y = self.y.checked_rem(rhs);
        if let (Some(x), Some(y)) = (x, y) {
            Some(Self { x, y })
        } else { None }
    }

    #[inline]
    pub const fn checked_rem_euclid_v(self, rhs: Self) -> Option<Self> {
        let x = self.x.checked_rem_euclid(rhs.x);
        let y = self.y.checked_rem_euclid(rhs.y);
        if let (Some(x), Some(y)) = (x, y) {
            Some(Self { x, y })
        } else { None }
    }

    #[inline]
    pub const fn checked_rem_euclid(self, rhs: i32) -> Option<Self> {
        let x = self.x.checked_rem_euclid(rhs);
        let y = self.y.checked_rem_euclid(rhs);
        if let (Some(x), Some(y)) = (x, y) {
            Some(Self { x, y })
        } else { None }
    }

    #[inline]
    pub const fn checked_neg(self) -> Option<Self> {
        let x = self.x.checked_neg();
        let y = self.y.checked_neg();
        if let (Some(x), Some(y)) = (x, y) {
            Some(Self { x, y })
        } else { None }
    }

    #[inline]
    pub const fn checked_shl_v(self, rhs: UVector2) -> Option<Self> {
        let x = self.x.checked_shl(rhs.x);
        let y = self.y.checked_shl(rhs.y);
        if let (Some(x), Some(y)) = (x, y) {
            Some(Self { x, y })
        } else { None }
    }

    #[inline]
    pub const fn checked_shl(self, rhs: u32) -> Option<Self> {
        let x = self.x.checked_shl(rhs);
        let y = self.y.checked_shl(rhs);
        if let (Some(x), Some(y)) = (x, y) {
            Some(Self { x, y })
        } else { None }
    }

    #[inline]
    pub const fn checked_shr_v(self, rhs: UVector2) -> Option<Self> {
        let x = self.x.checked_shr(rhs.x);
        let y = self.y.checked_shr(rhs.y);
        if let (Some(x), Some(y)) = (x, y) {
            Some(Self { x, y })
        } else { None }
    }

    #[inline]
    pub const fn checked_shr(self, rhs: u32) -> Option<Self> {
        let x = self.x.checked_shr(rhs);
        let y = self.y.checked_shr(rhs);
        if let (Some(x), Some(y)) = (x, y) {
            Some(Self { x, y })
        } else { None }
    }

    #[inline]
    pub const fn checked_abs(self) -> Option<Self> {
        let x = self.x.checked_abs();
        let y = self.y.checked_abs();
        if let (Some(x), Some(y)) = (x, y) {
            Some(Self { x, y })
        } else { None }
    }

    #[inline]
    pub const fn checked_pow_v(self, exp: UVector2) -> Option<Self> {
        let x = self.x.checked_pow(exp.x);
        let y = self.y.checked_pow(exp.y);
        if let (Some(x), Some(y)) = (x, y) {
            Some(Self { x, y })
        } else { None }
    }

    #[inline]
    pub const fn checked_pow(self, exp: u32) -> Option<Self> {
        let x = self.x.checked_pow(exp);
        let y = self.y.checked_pow(exp);
        if let (Some(x), Some(y)) = (x, y) {
            Some(Self { x, y })
        } else { None }
    }

    #[inline]
    pub const fn checked_isqrt(self) -> Option<Self> {
        let x = self.x.checked_isqrt();
        let y = self.y.checked_isqrt();
        if let (Some(x), Some(y)) = (x, y) {
            Some(Self { x, y })
        } else { None }
    }

    #[inline]
    pub const fn saturating_add_v(self, rhs: Self) -> Self {
        Self {
            x: self.x.saturating_add(rhs.x),
            y: self.y.saturating_add(rhs.y),
        }
    }

    #[inline]
    pub const fn saturating_add(self, rhs: i32) -> Self {
        Self {
            x: self.x.saturating_add(rhs),
            y: self.y.saturating_add(rhs),
        }
    }

    #[inline]
    pub const fn saturating_add_unsigned_v(self, rhs: UVector2) -> Self {
        Self {
            x: self.x.saturating_add_unsigned(rhs.x),
            y: self.y.saturating_add_unsigned(rhs.y),
        }
    }

    #[inline]
    pub const fn saturating_add_unsigned(self, rhs: u32) -> Self {
        Self {
            x: self.x.saturating_add_unsigned(rhs),
            y: self.y.saturating_add_unsigned(rhs),
        }
    }

    #[inline]
    pub const fn saturating_sub_v(self, rhs: Self) -> Self {
        Self {
            x: self.x.saturating_sub(rhs.x),
            y: self.y.saturating_sub(rhs.y),
        }
    }

    #[inline]
    pub const fn saturating_sub(self, rhs: i32) -> Self {
        Self {
            x: self.x.saturating_sub(rhs),
            y: self.y.saturating_sub(rhs),
        }
    }

    #[inline]
    pub const fn saturating_sub_unsigned_v(self, rhs: UVector2) -> Self {
        Self {
            x: self.x.saturating_sub_unsigned(rhs.x),
            y: self.y.saturating_sub_unsigned(rhs.y),
        }
    }

    #[inline]
    pub const fn saturating_sub_unsigned(self, rhs: u32) -> Self {
        Self {
            x: self.x.saturating_sub_unsigned(rhs),
            y: self.y.saturating_sub_unsigned(rhs),
        }
    }

    #[inline]
    pub const fn saturating_neg(self) -> Self {
        Self {
            x: self.x.saturating_neg(),
            y: self.y.saturating_neg(),
        }
    }

    #[inline]
    pub const fn saturating_abs(self) -> Self {
        Self {
            x: self.x.saturating_abs(),
            y: self.y.saturating_abs(),
        }
    }

    #[inline]
    pub const fn saturating_mul_v(self, rhs: Self) -> Self {
        Self {
            x: self.x.saturating_mul(rhs.x),
            y: self.y.saturating_mul(rhs.y),
        }
    }

    #[inline]
    pub const fn saturating_mul(self, rhs: i32) -> Self {
        Self {
            x: self.x.saturating_mul(rhs),
            y: self.y.saturating_mul(rhs),
        }
    }

    #[inline]
    pub const fn saturating_div_v(self, rhs: Self) -> Self {
        Self {
            x: self.x.saturating_div(rhs.x),
            y: self.y.saturating_div(rhs.y),
        }
    }

    #[inline]
    pub const fn saturating_div(self, rhs: i32) -> Self {
        Self {
            x: self.x.saturating_div(rhs),
            y: self.y.saturating_div(rhs),
        }
    }

    #[inline]
    pub const fn saturating_pow_v(self, exp: UVector2) -> Self {
        Self {
            x: self.x.saturating_pow(exp.x),
            y: self.y.saturating_pow(exp.y),
        }
    }

    #[inline]
    pub const fn saturating_pow(self, exp: u32) -> Self {
        Self {
            x: self.x.saturating_pow(exp),
            y: self.y.saturating_pow(exp),
        }
    }

    #[inline]
    pub const fn wrapping_add_v(self, rhs: Self) -> Self {
        Self {
            x: self.x.wrapping_add(rhs.x),
            y: self.y.wrapping_add(rhs.y),
        }
    }

    #[inline]
    pub const fn wrapping_add(self, rhs: i32) -> Self {
        Self {
            x: self.x.wrapping_add(rhs),
            y: self.y.wrapping_add(rhs),
        }
    }

    #[inline]
    pub const fn wrapping_add_unsigned_v(self, rhs: UVector2) -> Self {
        Self {
            x: self.x.wrapping_add_unsigned(rhs.x),
            y: self.y.wrapping_add_unsigned(rhs.y),
        }
    }

    #[inline]
    pub const fn wrapping_add_unsigned(self, rhs: u32) -> Self {
        Self {
            x: self.x.wrapping_add_unsigned(rhs),
            y: self.y.wrapping_add_unsigned(rhs),
        }
    }

    #[inline]
    pub const fn wrapping_sub_v(self, rhs: Self) -> Self {
        Self {
            x: self.x.wrapping_sub(rhs.x),
            y: self.y.wrapping_sub(rhs.y),
        }
    }

    #[inline]
    pub const fn wrapping_sub(self, rhs: i32) -> Self {
        Self {
            x: self.x.wrapping_sub(rhs),
            y: self.y.wrapping_sub(rhs),
        }
    }

    #[inline]
    pub const fn wrapping_sub_unsigned_v(self, rhs: UVector2) -> Self {
        Self {
            x: self.x.wrapping_sub_unsigned(rhs.x),
            y: self.y.wrapping_sub_unsigned(rhs.y),
        }
    }

    #[inline]
    pub const fn wrapping_sub_unsigned(self, rhs: u32) -> Self {
        Self {
            x: self.x.wrapping_sub_unsigned(rhs),
            y: self.y.wrapping_sub_unsigned(rhs),
        }
    }

    #[inline]
    pub const fn wrapping_mul_v(self, rhs: Self) -> Self {
        Self {
            x: self.x.wrapping_mul(rhs.x),
            y: self.y.wrapping_mul(rhs.y),
        }
    }

    #[inline]
    pub const fn wrapping_mul(self, rhs: i32) -> Self {
        Self {
            x: self.x.wrapping_mul(rhs),
            y: self.y.wrapping_mul(rhs),
        }
    }

    #[inline]
    pub const fn wrapping_div_v(self, rhs: Self) -> Self {
        Self {
            x: self.x.wrapping_div(rhs.x),
            y: self.y.wrapping_div(rhs.y),
        }
    }

    #[inline]
    pub const fn wrapping_div(self, rhs: i32) -> Self {
        Self {
            x: self.x.wrapping_div(rhs),
            y: self.y.wrapping_div(rhs),
        }
    }

    #[inline]
    pub const fn wrapping_div_euclid_v(self, rhs: Self) -> Self {
        Self {
            x: self.x.wrapping_div_euclid(rhs.x),
            y: self.y.wrapping_div_euclid(rhs.y),
        }
    }

    #[inline]
    pub const fn wrapping_div_euclid(self, rhs: i32) -> Self {
        Self {
            x: self.x.wrapping_div_euclid(rhs),
            y: self.y.wrapping_div_euclid(rhs),
        }
    }

    #[inline]
    pub const fn wrapping_rem_v(self, rhs: Self) -> Self {
        Self {
            x: self.x.wrapping_rem(rhs.x),
            y: self.y.wrapping_rem(rhs.y),
        }
    }

    #[inline]
    pub const fn wrapping_rem(self, rhs: i32) -> Self {
        Self {
            x: self.x.wrapping_rem(rhs),
            y: self.y.wrapping_rem(rhs),
        }
    }

    #[inline]
    pub const fn wrapping_rem_euclid_v(self, rhs: Self) -> Self {
        Self {
            x: self.x.wrapping_rem_euclid(rhs.x),
            y: self.y.wrapping_rem_euclid(rhs.y),
        }
    }

    #[inline]
    pub const fn wrapping_rem_euclid(self, rhs: i32) -> Self {
        Self {
            x: self.x.wrapping_rem_euclid(rhs),
            y: self.y.wrapping_rem_euclid(rhs),
        }
    }

    #[inline]
    pub const fn wrapping_neg(self) -> Self {
        Self {
            x: self.x.wrapping_neg(),
            y: self.y.wrapping_neg(),
        }
    }

    #[inline]
    pub const fn wrapping_shl_v(self, rhs: UVector2) -> Self {
        Self {
            x: self.x.wrapping_shl(rhs.x),
            y: self.y.wrapping_shl(rhs.y),
        }
    }

    #[inline]
    pub const fn wrapping_shl(self, rhs: u32) -> Self {
        Self {
            x: self.x.wrapping_shl(rhs),
            y: self.y.wrapping_shl(rhs),
        }
    }

    #[inline]
    pub const fn wrapping_shr_v(self, rhs: UVector2) -> Self {
        Self {
            x: self.x.wrapping_shr(rhs.x),
            y: self.y.wrapping_shr(rhs.y),
        }
    }

    #[inline]
    pub const fn wrapping_shr(self, rhs: u32) -> Self {
        Self {
            x: self.x.wrapping_shr(rhs),
            y: self.y.wrapping_shr(rhs),
        }
    }

    #[inline]
    pub const fn wrapping_abs(self) -> Self {
        Self {
            x: self.x.wrapping_abs(),
            y: self.y.wrapping_abs(),
        }
    }

    #[inline]
    pub const fn unsigned_abs(self) -> UVector2 {
        UVector2 {
            x: self.x.unsigned_abs(),
            y: self.y.unsigned_abs(),
        }
    }

    #[inline]
    pub const fn wrapping_pow_v(self, exp: UVector2) -> Self {
        Self {
            x: self.x.wrapping_pow(exp.x),
            y: self.y.wrapping_pow(exp.y),
        }
    }

    #[inline]
    pub const fn wrapping_pow(self, exp: u32) -> Self {
        Self {
            x: self.x.wrapping_pow(exp),
            y: self.y.wrapping_pow(exp),
        }
    }

    #[inline]
    pub const fn overflowing_add_v(self, rhs: Self) -> (Self, bool) {
        let (x, overflow_x) = self.x.overflowing_add(rhs.x);
        let (y, overflow_y) = self.y.overflowing_add(rhs.y);
        (Self { x, y }, overflow_x || overflow_y)
    }

    #[inline]
    pub const fn overflowing_add(self, rhs: i32) -> (Self, bool) {
        let (x, overflow_x) = self.x.overflowing_add(rhs);
        let (y, overflow_y) = self.y.overflowing_add(rhs);
        (Self { x, y }, overflow_x || overflow_y)
    }

    #[inline]
    pub const fn overflowing_add_unsigned_v(self, rhs: UVector2) -> (Self, bool) {
        let (x, overflow_x) = self.x.overflowing_add_unsigned(rhs.x);
        let (y, overflow_y) = self.y.overflowing_add_unsigned(rhs.y);
        (Self { x, y }, overflow_x || overflow_y)
    }

    #[inline]
    pub const fn overflowing_add_unsigned(self, rhs: u32) -> (Self, bool) {
        let (x, overflow_x) = self.x.overflowing_add_unsigned(rhs);
        let (y, overflow_y) = self.y.overflowing_add_unsigned(rhs);
        (Self { x, y }, overflow_x || overflow_y)
    }

    #[inline]
    pub const fn overflowing_sub_v(self, rhs: Self) -> (Self, bool) {
        let (x, overflow_x) = self.x.overflowing_sub(rhs.x);
        let (y, overflow_y) = self.y.overflowing_sub(rhs.y);
        (Self { x, y }, overflow_x || overflow_y)
    }

    #[inline]
    pub const fn overflowing_sub(self, rhs: i32) -> (Self, bool) {
        let (x, overflow_x) = self.x.overflowing_sub(rhs);
        let (y, overflow_y) = self.y.overflowing_sub(rhs);
        (Self { x, y }, overflow_x || overflow_y)
    }

    #[inline]
    pub const fn overflowing_sub_unsigned_v(self, rhs: UVector2) -> (Self, bool) {
        let (x, overflow_x) = self.x.overflowing_sub_unsigned(rhs.x);
        let (y, overflow_y) = self.y.overflowing_sub_unsigned(rhs.y);
        (Self { x, y }, overflow_x || overflow_y)
    }

    #[inline]
    pub const fn overflowing_sub_unsigned(self, rhs: u32) -> (Self, bool) {
        let (x, overflow_x) = self.x.overflowing_sub_unsigned(rhs);
        let (y, overflow_y) = self.y.overflowing_sub_unsigned(rhs);
        (Self { x, y }, overflow_x || overflow_y)
    }

    #[inline]
    pub const fn overflowing_mul_v(self, rhs: Self) -> (Self, bool) {
        let (x, overflow_x) = self.x.overflowing_mul(rhs.x);
        let (y, overflow_y) = self.y.overflowing_mul(rhs.y);
        (Self { x, y }, overflow_x || overflow_y)
    }

    #[inline]
    pub const fn overflowing_mul(self, rhs: i32) -> (Self, bool) {
        let (x, overflow_x) = self.x.overflowing_mul(rhs);
        let (y, overflow_y) = self.y.overflowing_mul(rhs);
        (Self { x, y }, overflow_x || overflow_y)
    }

    #[inline]
    pub const fn overflowing_div_v(self, rhs: Self) -> (Self, bool) {
        let (x, overflow_x) = self.x.overflowing_div(rhs.x);
        let (y, overflow_y) = self.y.overflowing_div(rhs.y);
        (Self { x, y }, overflow_x || overflow_y)
    }

    #[inline]
    pub const fn overflowing_div(self, rhs: i32) -> (Self, bool) {
        let (x, overflow_x) = self.x.overflowing_div(rhs);
        let (y, overflow_y) = self.y.overflowing_div(rhs);
        (Self { x, y }, overflow_x || overflow_y)
    }

    #[inline]
    pub const fn overflowing_div_euclid_v(self, rhs: Self) -> (Self, bool) {
        let (x, overflow_x) = self.x.overflowing_div_euclid(rhs.x);
        let (y, overflow_y) = self.y.overflowing_div_euclid(rhs.y);
        (Self { x, y }, overflow_x || overflow_y)
    }

    #[inline]
    pub const fn overflowing_div_euclid(self, rhs: i32) -> (Self, bool) {
        let (x, overflow_x) = self.x.overflowing_div_euclid(rhs);
        let (y, overflow_y) = self.y.overflowing_div_euclid(rhs);
        (Self { x, y }, overflow_x || overflow_y)
    }

    #[inline]
    pub const fn overflowing_rem_v(self, rhs: Self) -> (Self, bool) {
        let (x, overflow_x) = self.x.overflowing_rem(rhs.x);
        let (y, overflow_y) = self.y.overflowing_rem(rhs.y);
        (Self { x, y }, overflow_x || overflow_y)
    }

    #[inline]
    pub const fn overflowing_rem(self, rhs: i32) -> (Self, bool) {
        let (x, overflow_x) = self.x.overflowing_rem(rhs);
        let (y, overflow_y) = self.y.overflowing_rem(rhs);
        (Self { x, y }, overflow_x || overflow_y)
    }

    #[inline]
    pub const fn overflowing_rem_euclid_v(self, rhs: Self) -> (Self, bool) {
        let (x, overflow_x) = self.x.overflowing_rem_euclid(rhs.x);
        let (y, overflow_y) = self.y.overflowing_rem_euclid(rhs.y);
        (Self { x, y }, overflow_x || overflow_y)
    }

    #[inline]
    pub const fn overflowing_rem_euclid(self, rhs: i32) -> (Self, bool) {
        let (x, overflow_x) = self.x.overflowing_rem_euclid(rhs);
        let (y, overflow_y) = self.y.overflowing_rem_euclid(rhs);
        (Self { x, y }, overflow_x || overflow_y)
    }

    #[inline]
    pub const fn overflowing_neg(self) -> (Self, bool) {
        let (x, overflow_x) = self.x.overflowing_neg();
        let (y, overflow_y) = self.y.overflowing_neg();
        (Self { x, y }, overflow_x || overflow_y)
    }

    #[inline]
    pub const fn overflowing_shl_v(self, rhs: UVector2) -> (Self, bool) {
        let (x, overflow_x) = self.x.overflowing_shl(rhs.x);
        let (y, overflow_y) = self.y.overflowing_shl(rhs.y);
        (Self { x, y }, overflow_x || overflow_y)
    }

    #[inline]
    pub const fn overflowing_shl(self, rhs: u32) -> (Self, bool) {
        let (x, overflow_x) = self.x.overflowing_shl(rhs);
        let (y, overflow_y) = self.y.overflowing_shl(rhs);
        (Self { x, y }, overflow_x || overflow_y)
    }

    #[inline]
    pub const fn overflowing_shr_v(self, rhs: UVector2) -> (Self, bool) {
        let (x, overflow_x) = self.x.overflowing_shr(rhs.x);
        let (y, overflow_y) = self.y.overflowing_shr(rhs.y);
        (Self { x, y }, overflow_x || overflow_y)
    }

    #[inline]
    pub const fn overflowing_shr(self, rhs: u32) -> (Self, bool) {
        let (x, overflow_x) = self.x.overflowing_shr(rhs);
        let (y, overflow_y) = self.y.overflowing_shr(rhs);
        (Self { x, y }, overflow_x || overflow_y)
    }

    #[inline]
    pub const fn overflowing_abs(self) -> (Self, bool) {
        let (x, overflow_x) = self.x.overflowing_abs();
        let (y, overflow_y) = self.y.overflowing_abs();
        (Self { x, y }, overflow_x || overflow_y)
    }

    #[inline]
    pub const fn overflowing_pow_v(self, exp: UVector2) -> (Self, bool) {
        let (x, overflow_x) = self.x.overflowing_pow(exp.x);
        let (y, overflow_y) = self.y.overflowing_pow(exp.y);
        (Self { x, y }, overflow_x || overflow_y)
    }

    #[inline]
    pub const fn overflowing_pow(self, exp: u32) -> (Self, bool) {
        let (x, overflow_x) = self.x.overflowing_pow(exp);
        let (y, overflow_y) = self.y.overflowing_pow(exp);
        (Self { x, y }, overflow_x || overflow_y)
    }

    #[inline]
    pub const fn pow_v(self, exp: UVector2) -> Self {
        Self {
            x: self.x.pow(exp.x),
            y: self.y.pow(exp.y),
        }
    }

    #[inline]
    pub const fn pow(self, exp: u32) -> Self {
        Self {
            x: self.x.pow(exp),
            y: self.y.pow(exp),
        }
    }

    #[inline]
    pub const fn isqrt(self) -> Self {
        Self {
            x: self.x.isqrt(),
            y: self.y.isqrt(),
        }
    }

    #[inline]
    pub const fn div_euclid_v(self, rhs: Self) -> Self {
        Self {
            x: self.x.div_euclid(rhs.x),
            y: self.y.div_euclid(rhs.y),
        }
    }

    #[inline]
    pub const fn div_euclid(self, rhs: i32) -> Self {
        Self {
            x: self.x.div_euclid(rhs),
            y: self.y.div_euclid(rhs),
        }
    }

    #[inline]
    pub const fn rem_euclid_v(self, rhs: Self) -> Self {
        Self {
            x: self.x.rem_euclid(rhs.x),
            y: self.y.rem_euclid(rhs.y),
        }
    }

    #[inline]
    pub const fn rem_euclid(self, rhs: i32) -> Self {
        Self {
            x: self.x.rem_euclid(rhs),
            y: self.y.rem_euclid(rhs),
        }
    }

    #[inline]
    pub const fn ilog_v(self, base: Self) -> UVector2 {
        UVector2 {
            x: self.x.ilog(base.x),
            y: self.y.ilog(base.y),
        }
    }

    #[inline]
    pub const fn ilog(self, base: i32) -> UVector2 {
        UVector2 {
            x: self.x.ilog(base),
            y: self.y.ilog(base),
        }
    }

    #[inline]
    pub const fn ilog2(self) -> UVector2 {
        UVector2 {
            x: self.x.ilog2(),
            y: self.y.ilog2(),
        }
    }

    #[inline]
    pub const fn ilog10(self) -> UVector2 {
        UVector2 {
            x: self.x.ilog10(),
            y: self.y.ilog10(),
        }
    }

    #[inline]
    pub const fn checked_ilog(self, base: Self) -> Option<UVector2> {
        let x = self.x.checked_ilog(base.x);
        let y = self.y.checked_ilog(base.y);
        if let (Some(x), Some(y)) = (x, y) {
            Some(UVector2 { x, y })
        } else { None }
    }

    #[inline]
    pub const fn checked_ilog2(self) -> Option<UVector2> {
        let x = self.x.checked_ilog2();
        let y = self.y.checked_ilog2();
        if let (Some(x), Some(y)) = (x, y) {
            Some(UVector2 { x, y })
        } else { None }
    }

    #[inline]
    pub const fn checked_ilog10(self) -> Option<UVector2> {
        let x = self.x.checked_ilog10();
        let y = self.y.checked_ilog10();
        if let (Some(x), Some(y)) = (x, y) {
            Some(UVector2 { x, y })
        } else { None }
    }

    #[inline]
    pub const fn abs(self) -> Self {
        Self {
            x: self.x.abs(),
            y: self.y.abs(),
        }
    }

    #[inline]
    pub const fn abs_diff_v(self, other: Self) -> UVector2 {
        UVector2 {
            x: self.x.abs_diff(other.x),
            y: self.y.abs_diff(other.y),
        }
    }

    #[inline]
    pub const fn abs_diff(self, other: i32) -> UVector2 {
        UVector2 {
            x: self.x.abs_diff(other),
            y: self.y.abs_diff(other),
        }
    }

    #[inline]
    pub const fn signum(self) -> Self {
        Self {
            x: self.x.signum(),
            y: self.y.signum(),
        }
    }

    #[inline]
    pub const fn is_positive(self) -> [bool; 2] {
        [self.x.is_positive(), self.y.is_positive()]
    }

    #[inline]
    pub const fn is_negative(self) -> [bool; 2] {
        [self.x.is_negative(), self.y.is_negative()]
    }

    #[inline]
    pub const fn to_be_bytes(self) -> [u8; std::mem::size_of::<Self>()] {
        self.to_be().to_ne_bytes()
    }

    #[inline]
    pub const fn to_le_bytes(self) -> [u8; std::mem::size_of::<Self>()] {
        self.to_le().to_ne_bytes()
    }

    #[inline]
    pub const fn to_ne_bytes(self) -> [u8; std::mem::size_of::<Self>()] {
        // SAFETY: integer pairs are plain old datatypes so we can always transmute them to arrays of bytes
        // (i think?)
        unsafe { std::mem::transmute(self) }
    }

    #[inline]
    pub const fn from_be_bytes(bytes: [u8; std::mem::size_of::<Self>()]) -> Self {
        Self::from_be(Self::from_ne_bytes(bytes))
    }

    #[inline]
    pub const fn from_le_bytes(bytes: [u8; std::mem::size_of::<Self>()]) -> Self {
        Self::from_le(Self::from_ne_bytes(bytes))
    }

    #[inline]
    pub const fn from_ne_bytes(bytes: [u8; std::mem::size_of::<Self>()]) -> Self {
        unsafe { mem::transmute(bytes) }
    }
}

impl Index<usize> for IVector2 {
    type Output = i32;
    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            _ => panic!("Out of bounds"),
        }
    }
}

impl IndexMut<usize> for IVector2 {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            _ => panic!("Out of bounds"),
        }
    }
}

impl Neg for IVector2 {
    type Output = Self;
    #[inline]
    fn neg(self) -> Self::Output {
        Self {
            x: self.x.neg(),
            y: self.y.neg(),
        }
    }
}

impl Not for IVector2 {
    type Output = Self;
    #[inline]
    fn not(self) -> Self::Output {
        Self {
            x: self.x.not(),
            y: self.y.not(),
        }
    }
}

impl const Add for IVector2 {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x.add(rhs.x),
            y: self.y.add(rhs.y),
        }
    }
}

impl Sub for IVector2 {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x.sub(rhs.x),
            y: self.y.sub(rhs.y),
        }
    }
}

impl Mul for IVector2 {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x.mul(rhs.x),
            y: self.y.mul(rhs.y),
        }
    }
}

impl Div for IVector2 {
    type Output = Self;
    #[inline]
    fn div(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x.div(rhs.x),
            y: self.y.div(rhs.y),
        }
    }
}

impl Rem for IVector2 {
    type Output = Self;
    #[inline]
    fn rem(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x.rem(rhs.x),
            y: self.y.rem(rhs.y),
        }
    }
}

impl BitAnd for IVector2 {
    type Output = Self;
    #[inline]
    fn bitand(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x.bitand(rhs.x),
            y: self.y.bitand(rhs.y),
        }
    }
}

impl BitOr for IVector2 {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x.bitor(rhs.x),
            y: self.y.bitor(rhs.y),
        }
    }
}

impl BitXor for IVector2 {
    type Output = Self;
    #[inline]
    fn bitxor(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x.bitxor(rhs.x),
            y: self.y.bitxor(rhs.y),
        }
    }
}

impl Shl for IVector2 {
    type Output = Self;
    #[inline]
    fn shl(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x.shl(rhs.x),
            y: self.y.shl(rhs.y),
        }
    }
}

impl Shr for IVector2 {
    type Output = Self;
    #[inline]
    fn shr(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x.shr(rhs.x),
            y: self.y.shr(rhs.y),
        }
    }
}

impl const Add<i32> for IVector2 {
    type Output = Self;
    #[inline]
    fn add(self, rhs: i32) -> Self::Output {
        Self {
            x: self.x.add(rhs),
            y: self.y.add(rhs),
        }
    }
}

impl Sub<i32> for IVector2 {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: i32) -> Self::Output {
        Self {
            x: self.x.sub(rhs),
            y: self.y.sub(rhs),
        }
    }
}

impl Mul<i32> for IVector2 {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: i32) -> Self::Output {
        Self {
            x: self.x.mul(rhs),
            y: self.y.mul(rhs),
        }
    }
}

impl Div<i32> for IVector2 {
    type Output = Self;
    #[inline]
    fn div(self, rhs: i32) -> Self::Output {
        Self {
            x: self.x.div(rhs),
            y: self.y.div(rhs),
        }
    }
}

impl Rem<i32> for IVector2 {
    type Output = Self;
    #[inline]
    fn rem(self, rhs: i32) -> Self::Output {
        Self {
            x: self.x.rem(rhs),
            y: self.y.rem(rhs),
        }
    }
}

impl BitAnd<i32> for IVector2 {
    type Output = Self;
    #[inline]
    fn bitand(self, rhs: i32) -> Self::Output {
        Self {
            x: self.x.bitand(rhs),
            y: self.y.bitand(rhs),
        }
    }
}

impl BitOr<i32> for IVector2 {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: i32) -> Self::Output {
        Self {
            x: self.x.bitor(rhs),
            y: self.y.bitor(rhs),
        }
    }
}

impl BitXor<i32> for IVector2 {
    type Output = Self;
    #[inline]
    fn bitxor(self, rhs: i32) -> Self::Output {
        Self {
            x: self.x.bitxor(rhs),
            y: self.y.bitxor(rhs),
        }
    }
}

impl Shl<i32> for IVector2 {
    type Output = Self;
    #[inline]
    fn shl(self, rhs: i32) -> Self::Output {
        Self {
            x: self.x.shl(rhs),
            y: self.y.shl(rhs),
        }
    }
}

impl Shr<i32> for IVector2 {
    type Output = Self;
    #[inline]
    fn shr(self, rhs: i32) -> Self::Output {
        Self {
            x: self.x.shr(rhs),
            y: self.y.shr(rhs),
        }
    }
}

impl const Add<IVector2> for i32 {
    type Output = IVector2;
    #[inline]
    fn add(self, rhs: IVector2) -> Self::Output {
        IVector2 {
            x: self.add(rhs.x),
            y: self.add(rhs.y),
        }
    }
}

impl Sub<IVector2> for i32 {
    type Output = IVector2;
    #[inline]
    fn sub(self, rhs: IVector2) -> Self::Output {
        IVector2 {
            x: self.sub(rhs.x),
            y: self.sub(rhs.y),
        }
    }
}

impl Mul<IVector2> for i32 {
    type Output = IVector2;
    #[inline]
    fn mul(self, rhs: IVector2) -> Self::Output {
        IVector2 {
            x: self.mul(rhs.x),
            y: self.mul(rhs.y),
        }
    }
}

impl Div<IVector2> for i32 {
    type Output = IVector2;
    #[inline]
    fn div(self, rhs: IVector2) -> Self::Output {
        IVector2 {
            x: self.div(rhs.x),
            y: self.div(rhs.y),
        }
    }
}

impl Rem<IVector2> for i32 {
    type Output = IVector2;
    #[inline]
    fn rem(self, rhs: IVector2) -> Self::Output {
        IVector2 {
            x: self.rem(rhs.x),
            y: self.rem(rhs.y),
        }
    }
}

impl BitAnd<IVector2> for i32 {
    type Output = IVector2;
    #[inline]
    fn bitand(self, rhs: IVector2) -> Self::Output {
        IVector2 {
            x: self.bitand(rhs.x),
            y: self.bitand(rhs.y),
        }
    }
}

impl BitOr<IVector2> for i32 {
    type Output = IVector2;
    #[inline]
    fn bitor(self, rhs: IVector2) -> Self::Output {
        IVector2 {
            x: self.bitor(rhs.x),
            y: self.bitor(rhs.y),
        }
    }
}

impl BitXor<IVector2> for i32 {
    type Output = IVector2;
    #[inline]
    fn bitxor(self, rhs: IVector2) -> Self::Output {
        IVector2 {
            x: self.bitxor(rhs.x),
            y: self.bitxor(rhs.y),
        }
    }
}

impl Shl<IVector2> for i32 {
    type Output = IVector2;
    #[inline]
    fn shl(self, rhs: IVector2) -> Self::Output {
        IVector2 {
            x: self.shl(rhs.x),
            y: self.shl(rhs.y),
        }
    }
}

impl Shr<IVector2> for i32 {
    type Output = IVector2;
    #[inline]
    fn shr(self, rhs: IVector2) -> Self::Output {
        IVector2 {
            x: self.shr(rhs.x),
            y: self.shr(rhs.y),
        }
    }
}

impl AddAssign for IVector2 {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        *self = self.add(rhs);
    }
}

impl SubAssign for IVector2 {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        *self = self.sub(rhs);
    }
}

impl MulAssign for IVector2 {
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        *self = self.mul(rhs);
    }
}

impl DivAssign for IVector2 {
    #[inline]
    fn div_assign(&mut self, rhs: Self) {
        *self = self.div(rhs);
    }
}

impl RemAssign for IVector2 {
    #[inline]
    fn rem_assign(&mut self, rhs: Self) {
        *self = self.rem(rhs);
    }
}

impl BitAndAssign for IVector2 {
    #[inline]
    fn bitand_assign(&mut self, rhs: Self) {
        *self = self.bitand(rhs);
    }
}

impl BitOrAssign for IVector2 {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) {
        *self = self.bitor(rhs);
    }
}

impl BitXorAssign for IVector2 {
    #[inline]
    fn bitxor_assign(&mut self, rhs: Self) {
        *self = self.bitxor(rhs);
    }
}

impl ShlAssign for IVector2 {
    #[inline]
    fn shl_assign(&mut self, rhs: Self) {
        *self = self.shl(rhs);
    }
}

impl ShrAssign for IVector2 {
    #[inline]
    fn shr_assign(&mut self, rhs: Self) {
        *self = self.shr(rhs);
    }
}

impl AddAssign<i32> for IVector2 {
    #[inline]
    fn add_assign(&mut self, rhs: i32) {
        *self = self.add(rhs);
    }
}

impl SubAssign<i32> for IVector2 {
    #[inline]
    fn sub_assign(&mut self, rhs: i32) {
        *self = self.sub(rhs);
    }
}

impl MulAssign<i32> for IVector2 {
    #[inline]
    fn mul_assign(&mut self, rhs: i32) {
        *self = self.mul(rhs);
    }
}

impl DivAssign<i32> for IVector2 {
    #[inline]
    fn div_assign(&mut self, rhs: i32) {
        *self = self.div(rhs);
    }
}

impl RemAssign<i32> for IVector2 {
    #[inline]
    fn rem_assign(&mut self, rhs: i32) {
        *self = self.rem(rhs);
    }
}

impl BitAndAssign<i32> for IVector2 {
    #[inline]
    fn bitand_assign(&mut self, rhs: i32) {
        *self = self.bitand(rhs);
    }
}

impl BitOrAssign<i32> for IVector2 {
    #[inline]
    fn bitor_assign(&mut self, rhs: i32) {
        *self = self.bitor(rhs);
    }
}

impl BitXorAssign<i32> for IVector2 {
    #[inline]
    fn bitxor_assign(&mut self, rhs: i32) {
        *self = self.bitxor(rhs);
    }
}

impl ShlAssign<i32> for IVector2 {
    #[inline]
    fn shl_assign(&mut self, rhs: i32) {
        *self = self.shl(rhs);
    }
}

impl ShrAssign<i32> for IVector2 {
    #[inline]
    fn shr_assign(&mut self, rhs: i32) {
        *self = self.shr(rhs);
    }
}
