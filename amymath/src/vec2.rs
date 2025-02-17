use raylib::prelude::{*, Vector2 as RlVector2};
use crate::{prelude::*, MinMax};
use std::{mem, ops::*};

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

impl From<RlVector2> for Vector2 {
    #[inline]
    fn from(RlVector2 { x, y }: RlVector2) -> Self {
        Self { x, y }
    }
}

impl From<Vector2> for RlVector2 {
    #[inline]
    fn from(Vector2 { x, y }: Vector2) -> Self {
        Self { x, y }
    }
}

impl From<ffi::Vector2> for Vector2 {
    #[inline]
    fn from(ffi::Vector2 { x, y }: ffi::Vector2) -> Self {
        Self { x, y }
    }
}

impl From<Vector2> for ffi::Vector2 {
    #[inline]
    fn from(Vector2 { x, y }: Vector2) -> Self {
        Self { x, y }
    }
}

impl From<Vector2> for [f32; 2] {
    #[inline]
    fn from(Vector2 { x, y }: Vector2) -> Self {
        [x, y]
    }
}

impl From<[f32; 2]> for Vector2 {
    #[inline]
    fn from([x, y]: [f32; 2]) -> Self {
        Vector2 { x, y }
    }
}

impl From<Vector2> for (f32, f32) {
    #[inline]
    fn from(Vector2 { x, y }: Vector2) -> Self {
        (x, y)
    }
}

impl From<(f32, f32)> for Vector2 {
    #[inline]
    fn from((x, y): (f32, f32)) -> Self {
        Vector2 { x, y }
    }
}

impl Vector2 {
    pub const ZERO: Self = Self { x: 0.0, y: 0.0 };
    pub const ONE: Self = Self { x: 1.0, y: 1.0 };

    pub const ANGLE_0_RAD: Self = Self { x: 1.0, y: 0.0 };
    pub const ANGLE_FRAC_PI_6_RAD: Self = Self { x: 0.5 * std::f32::consts::SQRT_3, y: 0.5 };
    pub const ANGLE_FRAC_PI_4_RAD: Self = Self { x: std::f32::consts::FRAC_1_SQRT_2, y: std::f32::consts::FRAC_1_SQRT_2 };
    pub const ANGLE_FRAC_PI_3_RAD: Self = Self { x: 0.5, y: 0.5 * std::f32::consts::SQRT_3 };
    pub const ANGLE_FRAC_PI_2_RAD: Self = Self { x: 0.0, y: 1.0 };
    pub const ANGLE_FRAC_2PI_3_RAD: Self = Self { x: -0.5, y: 0.5 * std::f32::consts::SQRT_3 };
    pub const ANGLE_FRAC_3PI_4_RAD: Self = Self { x: -std::f32::consts::FRAC_1_SQRT_2, y: std::f32::consts::FRAC_1_SQRT_2 };
    pub const ANGLE_FRAC_5PI_6_RAD: Self = Self { x: -0.5 * std::f32::consts::SQRT_3, y: 0.5 };
    pub const ANGLE_PI_RAD: Self = Self { x: -1.0, y: 0.0 };
    pub const ANGLE_FRAC_7PI_6_RAD: Self = Self { x: -0.5 * std::f32::consts::SQRT_3, y: -0.5 };
    pub const ANGLE_FRAC_5PI_4_RAD: Self = Self { x: -std::f32::consts::FRAC_1_SQRT_2, y: -std::f32::consts::FRAC_1_SQRT_2 };
    pub const ANGLE_FRAC_4PI_3_RAD: Self = Self { x: -0.5, y: 0.5 * -std::f32::consts::SQRT_3 };
    pub const ANGLE_FRAC_3PI_2_RAD: Self = Self { x: 0.0, y: -1.0 };
    pub const ANGLE_FRAC_5PI_3_RAD: Self = Self { x: 0.5, y: -0.5 * std::f32::consts::SQRT_3 };
    pub const ANGLE_FRAC_7PI_4_RAD: Self = Self { x: std::f32::consts::FRAC_1_SQRT_2, y: -std::f32::consts::FRAC_1_SQRT_2 };
    pub const ANGLE_FRAC_11PI_6_RAD: Self = Self { x: 0.5 * std::f32::consts::SQRT_3, y: -0.5 };

    pub const EPSILON: Self = Self { x: f32::EPSILON, y: f32::EPSILON };
    pub const MIN: Self = Self { x: f32::MIN, y: f32::MIN };
    pub const MIN_POSITIVE: Self = Self { x: f32::MIN_POSITIVE, y: f32::MIN_POSITIVE };
    pub const MAX: Self = Self { x: f32::MAX, y: f32::MAX };
    pub const NAN: Self = Self { x: f32::NAN, y: f32::NAN };
    pub const INFINITY: Self = Self { x: f32::INFINITY, y: f32::INFINITY };
    pub const NEG_INFINITY: Self = Self { x: f32::NEG_INFINITY, y: f32::NEG_INFINITY };

    #[inline]
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    #[inline]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub const fn max_element(self) -> f32 {
        self.x.max(self.y)
    }

    #[inline]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub const fn min_element(self) -> f32 {
        self.x.min(self.y)
    }

    #[inline]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub const fn sum(self) -> f32 {
        self.x + self.y
    }

    #[inline]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub const fn prod(self) -> f32 {
        self.x * self.y
    }

    #[inline]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub const fn delta(self, other: Self) -> Self {
        Self {
            x: other.x - self.x,
            y: other.y - self.y,
        }
    }

    #[inline]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub const fn offset_iso(self, amount: f32) -> Self {
        Self {
            x: self.x + amount,
            y: self.y + amount,
        }
    }

    #[inline]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub const fn offset(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    #[inline]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub const fn scale_iso(self, amount: f32) -> Self {
        Self {
            x: self.x * amount,
            y: self.y * amount,
        }
    }

    #[inline]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub const fn scale(self, amount: Self) -> Self {
        Self {
            x: self.x * amount.x,
            y: self.y * amount.y,
        }
    }

    #[inline]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub const fn dot(self, other: Self) -> f32 {
        self.x * other.x + self.y * other.y
    }

    #[inline]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub const fn cross(self, other: Self) -> f32 {
        self.x * other.y - self.y * other.x
    }

    #[inline]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub const fn magnitude_sqr(self) -> f32 {
        self.dot(self)
    }

    #[inline]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub const fn is_unit(&self) -> bool {
        (self.magnitude_sqr() - 1.0).abs() <= f32::EPSILON
    }

    #[inline]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub fn magnitude(self) -> f32 {
        self.magnitude_sqr().sqrt()
    }

    #[inline]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub fn direction_and_magnitude(self) -> (Self, f32) {
        let mag = self.magnitude();
        (self / mag, mag)
    }

    #[inline]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub fn normalized(self) -> Self {
        self.direction_and_magnitude().0
    }

    #[inline]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub const fn distance_sqr(self, other: Self) -> f32 {
        self.delta(other).magnitude_sqr()
    }

    #[inline]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub fn distance(self, other: Self) -> f32 {
        self.distance_sqr(other).sqrt()
    }

    #[inline]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub fn delta_direction_and_distance(self, other: Self) -> (Self, (Self, f32)) {
        let delta = self.delta(other);
        (delta, delta.direction_and_magnitude())
    }

    #[inline]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub fn direction_and_distance(self, other: Self) -> (Self, f32) {
        self.delta_direction_and_distance(other).1
    }

    #[inline]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub fn direction(self, other: Self) -> Self {
        self.direction_and_distance(other).0
    }

    #[inline]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub const fn reflected_over(self, across: Self) -> Self {
        Self {
            x: 2.0*across.x - self.x,
            y: 2.0*across.y - self.y,
        }
    }

    #[inline]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub const fn reflected_at(self, across: Self, scale: f32) -> Self {
        let delta = across.delta(self);
        Self {
            x: across.x - scale * delta.x,
            y: across.y - scale * delta.y,
        }
    }

    #[inline]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub fn reflected_to(self, across: Self, mut length: f32) -> Self {
        let delta = across.delta(self);
        length /= delta.magnitude();
        Self {
            x: across.x - length * delta.x,
            y: across.y - length * delta.y,
        }
    }

    /// Linear interpolate
    #[inline]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub const fn lerp(p0: Self, p1: Self, t: f32) -> Self {
        Self {
            x: p0.x + t*(p1.x - p0.x),
            y: p0.y + t*(p1.y - p0.y),
        }
    }

    /// Linear velocity
    #[inline(always)]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub const fn lvel(p0: Self, p1: Self) -> Self {
        p0.delta(p1)
    }

    /// Quadratic interpolate
    #[inline]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub const fn qerp(p0: Self, p1: Self, p2: Self, t: f32) -> Self {
        Self {
            x: p1.x + (1.0 - t)*(1.0 - t)*(p0.x - p1.x) + t*t*(p2.x - p1.x),
            y: p1.y + (1.0 - t)*(1.0 - t)*(p0.y - p1.y) + t*t*(p2.y - p1.y),
        }
    }

    /// Quadratic velocity
    #[inline]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub const fn qvel(p0: Self, p1: Self, p2: Self, t: f32) -> Self {
        Self {
            x: 2.0*(1.0 - t)*(p1.x - p0.x) + 2.0*t*(p2.x - p1.x),
            y: 2.0*(1.0 - t)*(p1.y - p0.y) + 2.0*t*(p2.y - p1.y),
        }
    }

    /// Quadratic acceleration
    #[inline]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub const fn qacc(p0: Self, p1: Self, p2: Self) -> Self {
        Self {
            x: 2.0*(p2.x - 2.0*p1.x + p0.x),
            y: 2.0*(p2.y - 2.0*p1.y + p0.y),
        }
    }

    /// Cubic interpolate
    #[inline]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub const fn cerp(p0: Self, p1: Self, p2: Self, p3: Self, t: f32) -> Self {
        Self {
            x: (1.0 - t)*(1.0 - t)*(1.0 - t)*p0.x + 3.0*(1.0 - t)*(1.0 - t)*t*p1.x + 3.0*(1.0 - t)*t*t*p2.x + t*t*t*p3.x,
            y: (1.0 - t)*(1.0 - t)*(1.0 - t)*p0.y + 3.0*(1.0 - t)*(1.0 - t)*t*p1.y + 3.0*(1.0 - t)*t*t*p2.y + t*t*t*p3.y,
        }
    }

    /// Cubic velocity
    #[inline]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub const fn cvel(p0: Self, p1: Self, p2: Self, p3: Self, t: f32) -> Self {
        Self {
            x: 3.0*(1.0 - t)*(1.0 - t)*(p1.x - p0.x) + 6.0*(1.0 - t)*t*(p2.x - p1.x) + 3.0*t*t*(p3.x - p2.x),
            y: 3.0*(1.0 - t)*(1.0 - t)*(p1.y - p0.y) + 6.0*(1.0 - t)*t*(p2.y - p1.y) + 3.0*t*t*(p3.y - p2.y),
        }
    }

    /// Cubic acceleration
    #[inline]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub const fn cacc(p0: Self, p1: Self, p2: Self, p3: Self, t: f32) -> Self {
        Self {
            x: 6.0*(1.0 - t)*(p2.x - 2.0*p1.x + p0.x) + 6.0*t*(p3.x - 2.0*p2.x + p1.x),
            y: 6.0*(1.0 - t)*(p2.y - 2.0*p1.y + p0.y) + 6.0*t*(p3.y - 2.0*p2.y + p1.y),
        }
    }

    /// Cubic jerk
    #[inline]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub const fn cjerk(p0: Self, p1: Self, p2: Self, p3: Self) -> Self {
        Self {
            x: 6.0*(p3.x + 3.0*(p1.x - p2.x) - p0.x),
            y: 6.0*(p3.y + 3.0*(p1.y - p2.y) - p0.y),
        }
    }

    /// Negate the angle, turning `theta` into `-theta`
    ///
    /// `(cos(-theta), sin(-theta)) = (cos(theta), -sin(theta))`
    #[inline]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub const fn neg_angle(&self) -> Self {
        Self {
            x:  self.x,
            y: -self.y,
        }
    }

    /// Subtract the angle from `PI/2`
    ///
    /// `(cos(PI/2 - theta), sin(PI/2 - theta)) = (sin(theta), cos(theta))`
    #[inline]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub const fn frac_pi_2_minus(&self) -> Self {
        Self {
            x: self.y,
            y: self.x,
        }
    }

    /// Add the angles of the two vectors
    ///
    /// `cos(a + b) = cos(a)cos(b) - sin(a)sin(b)`
    /// `sin(a + b) = sin(a)cos(b) + cos(a)sin(b)`
    #[inline]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub const fn add_angle(self, other: Self) -> Self {
        Self {
            x: self.x*other.x - self.y*other.y,
            y: self.y*other.x + self.x*other.y,
        }
    }

    /// Subtract the angles of the two vectors
    ///
    /// `cos(a - b) = cos(a)cos(b) + sin(a)sin(b)`
    /// `sin(a - b) = sin(a)cos(b) - cos(a)sin(b)`
    #[inline]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub const fn sub_angle(self, other: Self) -> Self {
        Self {
            x: self.x*other.x + self.y*other.y,
            y: self.y*other.x - self.x*other.y,
        }
    }

    /// Doubles the angle of the vector
    #[inline]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub const fn double_angle(self) -> Self {
        self.add_angle(self)
    }

    /// Returns a vector whose angle is the square of half of the vector's angle
    ///
    /// `cos(theta/2) = +/-sqrt((1 + cos(theta))/2)`
    /// `sin(theta/2) = +/-sqrt((1 - cos(theta))/2)`
    #[inline]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub const fn half_angle_sqr(self) -> Self {
        Self {
            x: 0.5 + self.x * 0.5,
            y: 0.5 - self.x * 0.5,
        }
    }

    #[inline]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub fn angle_to(&self, other: &Self) -> f32 {
        let delta = self.delta(*other);
        let mut result = delta.y.atan2(delta.x);
        if result < 0.0 {
            result += 2.0 * std::f32::consts::PI;
        }
        result
    }

    /// Rotate by the specified angle
    #[inline]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub fn rotate(self, angle: f32) -> Self {
        let (sin, cos) = angle.sin_cos();
        self.add_angle(Self::new(cos, sin))
    }

    /// Rotate counter clockwise
    #[inline]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub const fn rotate_cc(self) -> Self {
        Self {
            x: -self.y,
            y:  self.x,
        }
    }

    /// Rotate clockwise
    #[inline]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub const fn rotate_cw(self) -> Self {
        Self {
            x:  self.y,
            y: -self.x,
        }
    }

    #[inline]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub const fn transform_with(self, mat: &Matrix2x2) -> Self {
        Self {
            x: self.x*mat.m00 + self.y*mat.m01,
            y: self.x*mat.m10 + self.y*mat.m11,
        }
    }

    #[inline]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub const fn is_nan(self) -> bool {
        self.x.is_nan() || self.y.is_nan()
    }

    #[inline]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub const fn is_infinite(self) -> bool {
        self.x.is_infinite() || self.y.is_infinite()
    }

    #[inline]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub const fn is_finite(self) -> bool {
        self.x.is_finite() && self.y.is_finite()
    }

    #[inline]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub const fn is_subnormal(self) -> bool {
        self.x.is_subnormal() || self.y.is_subnormal()
    }

    #[inline]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub const fn is_normal(self) -> bool {
        self.x.is_normal() && self.y.is_normal()
    }

    #[inline]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub const fn classify(self) -> [std::num::FpCategory; 2] {
        [self.x.classify(), self.y.classify()]
    }

    #[inline]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub const fn is_sign_positive(self) -> [bool; 2] {
        [self.x.is_sign_positive(), self.y.is_sign_positive()]
    }

    #[inline]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub const fn is_sign_negative(self) -> [bool; 2] {
        [self.x.is_sign_negative(), self.y.is_sign_negative()]
    }

    #[inline]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub const fn next_up(self) -> Self {
        Self {
            x: self.x.next_up(),
            y: self.y.next_up(),
        }
    }

    #[inline]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub const fn next_down(self) -> Self {
        Self {
            x: self.x.next_down(),
            y: self.y.next_down(),
        }
    }

    #[inline]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub const fn recip(self) -> Self {
        Self {
            x: self.x.recip(),
            y: self.y.recip(),
        }
    }

    #[inline]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub const fn max_v(self, other: Self) -> Self {
        Self {
            x: self.x.max(other.x),
            y: self.y.max(other.y),
        }
    }

    #[must_use = "this returns the result of the operation, without modifying the original"]
    #[inline]
    pub const fn max(self, other: f32) -> Self {
        Self {
            x: self.x.max(other),
            y: self.y.max(other),
        }
    }

    #[must_use = "this returns the result of the operation, without modifying the original"]
    #[inline]
    pub const fn min_v(self, other: Self) -> Self {
        Self {
            x: self.x.min(other.x),
            y: self.y.min(other.y),
        }
    }

    #[must_use = "this returns the result of the operation, without modifying the original"]
    #[inline]
    pub const fn min(self, other: f32) -> Self {
        Self {
            x: self.x.min(other),
            y: self.y.min(other),
        }
    }

    #[must_use = "this returns the result of the operation, without modifying the original"]
    #[inline]
    pub const fn minmax_v(self, other: Self) -> (Self, Self) {
        let (xmin, xmax) = self.x.minmax(other.x);
        let (ymin, ymax) = self.y.minmax(other.y);
        (Self { x: xmin, y: ymin }, Self { x: xmax, y: ymax })
    }

    #[must_use = "this returns the result of the operation, without modifying the original"]
    #[inline]
    pub const fn minmax(self, other: f32) -> (Self, Self) {
        let (xmin, xmax) = self.x.minmax(other);
        let (ymin, ymax) = self.y.minmax(other);
        (Self { x: xmin, y: ymin }, Self { x: xmax, y: ymax })
    }

    #[must_use = "this returns the result of the operation, without modifying the original"]
    #[inline]
    pub const fn midpoint_v(self, other: Self) -> Self {
        Self {
            x: self.x.midpoint(other.x),
            y: self.y.midpoint(other.y),
        }
    }

    #[must_use = "this returns the result of the operation, without modifying the original"]
    #[inline]
    pub const fn midpoint(self, other: f32) -> Self {
        Self {
            x: self.x.midpoint(other),
            y: self.y.midpoint(other),
        }
    }

    #[must_use = "this returns the result of the operation, without modifying the original"]
    #[inline]
    pub unsafe fn to_ivec_unchecked(self) -> IVector2 {
        IVector2 {
            x: self.x.to_int_unchecked(),
            y: self.y.to_int_unchecked(),
        }
    }

    #[must_use = "this returns the result of the operation, without modifying the original"]
    #[inline]
    pub unsafe fn to_uvec_unchecked(self) -> UVector2 {
        UVector2 {
            x: self.x.to_int_unchecked(),
            y: self.y.to_int_unchecked(),
        }
    }

    #[must_use = "this returns the result of the operation, without modifying the original"]
    #[inline]
    pub const fn to_bits(self) -> u64 {
        // Safety: Vector2 is POD
        unsafe { mem::transmute(self) }
    }

    #[must_use = "this returns the result of the operation, without modifying the original"]
    #[inline]
    pub const fn from_bits(v: u64) -> Self {
        // Safety: Vector2 is POD
        unsafe { mem::transmute(v) }
    }

    #[must_use = "this returns the result of the operation, without modifying the original"]
    #[inline]
    pub const fn to_be_bytes(self) -> [u8; mem::size_of::<Self>()] {
        self.to_bits().to_be_bytes()
    }

    #[must_use = "this returns the result of the operation, without modifying the original"]
    #[inline]
    pub const fn to_le_bytes(self) -> [u8; mem::size_of::<Self>()] {
        self.to_bits().to_le_bytes()
    }

    #[must_use = "this returns the result of the operation, without modifying the original"]
    #[inline]
    pub const fn to_ne_bytes(self) -> [u8; mem::size_of::<Self>()] {
        self.to_bits().to_ne_bytes()
    }

    #[must_use = "this returns the result of the operation, without modifying the original"]
    #[inline]
    pub const fn from_be_bytes(bytes: [u8; mem::size_of::<Self>()]) -> Self {
        Self::from_bits(u64::from_be_bytes(bytes))
    }

    #[must_use = "this returns the result of the operation, without modifying the original"]
    #[inline]
    pub const fn from_le_bytes(bytes: [u8; mem::size_of::<Self>()]) -> Self {
        Self::from_bits(u64::from_le_bytes(bytes))
    }

    #[must_use = "this returns the result of the operation, without modifying the original"]
    #[inline]
    pub const fn from_ne_bytes(bytes: [u8; mem::size_of::<Self>()]) -> Self {
        Self::from_bits(u64::from_ne_bytes(bytes))
    }

    #[must_use = "this returns the result of the operation, without modifying the original"]
    #[inline]
    pub fn total_cmp(&self, other: &Self) -> [std::cmp::Ordering; 2] {
        [self.x.total_cmp(&other.x), self.y.total_cmp(&other.y)]
    }

    #[must_use = "this returns the result of the operation, without modifying the original"]
    #[inline]
    pub const fn clamp_v(self, min: Self, max: Self) -> Self {
        Self {
            x: self.x.clamp(min.x, max.x),
            y: self.y.clamp(min.y, max.y),
        }
    }

    #[must_use = "this returns the result of the operation, without modifying the original"]
    #[inline]
    pub const fn clamp(self, min: f32, max: f32) -> Self {
        Self {
            x: self.x.clamp(min, max),
            y: self.y.clamp(min, max),
        }
    }

    #[must_use = "this returns the result of the operation, without modifying the original"]
    #[inline]
    pub const fn abs(self) -> Self {
        Self {
            x: self.x.abs(),
            y: self.y.abs(),
        }
    }

    #[must_use = "this returns the result of the operation, without modifying the original"]
    #[inline]
    pub const fn signum(self) -> Self {
        Self {
            x: self.x.signum(),
            y: self.y.signum(),
        }
    }

    #[must_use = "this returns the result of the operation, without modifying the original"]
    #[inline]
    pub const fn copysign_v(self, sign: Self) -> Self {
        Self {
            x: self.x.copysign(sign.x),
            y: self.y.copysign(sign.y),
        }
    }

    #[must_use = "this returns the result of the operation, without modifying the original"]
    #[inline]
    pub const fn copysign(self, sign: f32) -> Self {
        Self {
            x: self.x.copysign(sign),
            y: self.y.copysign(sign),
        }
    }

    #[must_use = "method returns a new number and does not mutate the original value"]
    #[inline]
    pub fn floor(self) -> Self {
        Self {
            x: self.x.floor(),
            y: self.y.floor(),
        }
    }

    #[must_use = "method returns a new number and does not mutate the original value"]
    #[inline]
    pub fn ceil(self) -> Self {
        Self {
            x: self.x.ceil(),
            y: self.y.ceil(),
        }
    }

    #[must_use = "method returns a new number and does not mutate the original value"]
    #[inline]
    pub fn round(self) -> Self {
        Self {
            x: self.x.round(),
            y: self.y.round(),
        }
    }

    #[must_use = "method returns a new number and does not mutate the original value"]
    #[inline]
    pub fn round_ties_even(self) -> Self {
        Self {
            x: self.x.round_ties_even(),
            y: self.y.round_ties_even(),
        }
    }

    #[must_use = "method returns a new number and does not mutate the original value"]
    #[inline]
    pub fn trunc(self) -> Self {
        Self {
            x: self.x.trunc(),
            y: self.y.trunc(),
        }
    }

    #[must_use = "method returns a new number and does not mutate the original value"]
    #[inline]
    pub fn fract(self) -> Self {
        self - self.trunc()
    }

    #[must_use = "method returns a new number and does not mutate the original value"]
    #[inline]
    pub fn mul_add(self, a: Self, b: Self) -> Self {
        Self {
            x: self.x.mul_add(a.x, b.x),
            y: self.y.mul_add(a.y, b.y),
        }
    }

    #[must_use = "method returns a new number and does not mutate the original value"]
    #[inline]
    pub fn div_euclid(self, rhs: Self) -> Self {
        Self {
            x: self.x.div_euclid(rhs.x),
            y: self.y.div_euclid(rhs.y),
        }
    }

    #[must_use = "method returns a new number and does not mutate the original value"]
    #[inline]
    pub fn rem_euclid(self, rhs: Self) -> Self {
        Self {
            x: self.x.rem_euclid(rhs.x),
            y: self.y.rem_euclid(rhs.y),
        }
    }


    #[must_use = "this returns the result of the operation, without modifying the original"]
    #[inline]
    pub fn powi_v(self, n: IVector2) -> Self {
        Self {
            x: self.x.powi(n.x),
            y: self.y.powi(n.y),
        }
    }


    #[must_use = "this returns the result of the operation, without modifying the original"]
    #[inline]
    pub fn powi(self, n: i32) -> Self {
        Self {
            x: self.x.powi(n),
            y: self.y.powi(n),
        }
    }


    #[must_use = "this returns the result of the operation, without modifying the original"]
    #[inline]
    pub fn powf_v(self, n: Self) -> Self {
        Self {
            x: self.x.powf(n.x),
            y: self.y.powf(n.y),
        }
    }


    #[must_use = "this returns the result of the operation, without modifying the original"]
    #[inline]
    pub fn powf(self, n: f32) -> Self {
        Self {
            x: self.x.powf(n),
            y: self.y.powf(n),
        }
    }


    #[must_use = "this returns the result of the operation, without modifying the original"]
    #[inline]
    pub fn sqrt(self) -> Self {
        Self {
            x: self.x.sqrt(),
            y: self.y.sqrt(),
        }
    }


    #[must_use = "this returns the result of the operation, without modifying the original"]
    #[inline]
    pub fn exp(self) -> Self {
        Self {
            x: self.x.exp(),
            y: self.y.exp(),
        }
    }


    #[must_use = "this returns the result of the operation, without modifying the original"]
    #[inline]
    pub fn exp2(self) -> Self {
        Self {
            x: self.x.exp2(),
            y: self.y.exp2(),
        }
    }


    #[must_use = "this returns the result of the operation, without modifying the original"]
    #[inline]
    pub fn ln(self) -> Self {
        Self {
            x: self.x.ln(),
            y: self.y.ln(),
        }
    }


    #[must_use = "this returns the result of the operation, without modifying the original"]
    #[inline]
    pub fn log_v(self, base: Self) -> Self {
        Self {
            x: self.x.log(base.x),
            y: self.y.log(base.y),
        }
    }


    #[must_use = "this returns the result of the operation, without modifying the original"]
    #[inline]
    pub fn log(self, base: f32) -> Self {
        Self {
            x: self.x.log(base),
            y: self.y.log(base),
        }
    }


    #[must_use = "this returns the result of the operation, without modifying the original"]
    #[inline]
    pub fn log2(self) -> Self {
        Self {
            x: self.x.log2(),
            y: self.y.log2(),
        }
    }


    #[must_use = "this returns the result of the operation, without modifying the original"]
    #[inline]
    pub fn log10(self) -> Self {
        Self {
            x: self.x.log10(),
            y: self.y.log10(),
        }
    }


    #[must_use = "this returns the result of the operation, without modifying the original"]
    #[inline]
    pub fn cbrt(self) -> Self {
        Self {
            x: self.x.cbrt(),
            y: self.y.cbrt(),
        }
    }


    #[must_use = "this returns the result of the operation, without modifying the original"]
    #[inline]
    pub fn hypot_v(self, other: Self) -> Self {
        Self {
            x: self.x.hypot(other.x),
            y: self.y.hypot(other.y),
        }
    }


    #[must_use = "this returns the result of the operation, without modifying the original"]
    #[inline]
    pub fn hypot(self, other: f32) -> Self {
        Self {
            x: self.x.hypot(other),
            y: self.y.hypot(other),
        }
    }

    /// Calculate the magnitude using [`f32::hypot`]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    #[inline]
    pub fn magnitude_hypot(self) -> f32 {
        self.x.hypot(self.y)
    }


    #[must_use = "this returns the result of the operation, without modifying the original"]
    #[inline]
    pub fn exp_m1(self) -> Self {
        Self {
            x: self.x.exp_m1(),
            y: self.y.exp_m1(),
        }
    }


    #[must_use = "this returns the result of the operation, without modifying the original"]
    #[inline]
    pub fn ln_1p(self) -> Self {
        Self {
            x: self.x.ln_1p(),
            y: self.y.ln_1p(),
        }
    }
}

impl Index<usize> for Vector2 {
    type Output = f32;
    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            _ => panic!("Out of bounds"),
        }
    }
}

impl IndexMut<usize> for Vector2 {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            _ => panic!("Out of bounds"),
        }
    }
}

impl Neg for Vector2 {
    type Output = Self;
    #[inline]
    fn neg(self) -> Self::Output {
        Self {
            x: self.x.neg(),
            y: self.y.neg(),
        }
    }
}

impl const Add for Vector2 {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x.add(rhs.x),
            y: self.y.add(rhs.y),
        }
    }
}

impl Sub for Vector2 {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x.sub(rhs.x),
            y: self.y.sub(rhs.y),
        }
    }
}

impl Mul for Vector2 {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x.mul(rhs.x),
            y: self.y.mul(rhs.y),
        }
    }
}

impl Div for Vector2 {
    type Output = Self;
    #[inline]
    fn div(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x.div(rhs.x),
            y: self.y.div(rhs.y),
        }
    }
}

impl const Add<f32> for Vector2 {
    type Output = Self;
    #[inline]
    fn add(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x.add(rhs),
            y: self.y.add(rhs),
        }
    }
}

impl Sub<f32> for Vector2 {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x.sub(rhs),
            y: self.y.sub(rhs),
        }
    }
}

impl Mul<f32> for Vector2 {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x.mul(rhs),
            y: self.y.mul(rhs),
        }
    }
}

impl Div<f32> for Vector2 {
    type Output = Self;
    #[inline]
    fn div(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x.div(rhs),
            y: self.y.div(rhs),
        }
    }
}

impl const Add<Vector2> for f32 {
    type Output = Vector2;
    #[inline]
    fn add(self, rhs: Vector2) -> Self::Output {
        Vector2 {
            x: self.add(rhs.x),
            y: self.add(rhs.y),
        }
    }
}

impl Sub<Vector2> for f32 {
    type Output = Vector2;
    #[inline]
    fn sub(self, rhs: Vector2) -> Self::Output {
        Vector2 {
            x: self.sub(rhs.x),
            y: self.sub(rhs.y),
        }
    }
}

impl Mul<Vector2> for f32 {
    type Output = Vector2;
    #[inline]
    fn mul(self, rhs: Vector2) -> Self::Output {
        Vector2 {
            x: self.mul(rhs.x),
            y: self.mul(rhs.y),
        }
    }
}

impl Div<Vector2> for f32 {
    type Output = Vector2;
    #[inline]
    fn div(self, rhs: Vector2) -> Self::Output {
        Vector2 {
            x: self.div(rhs.x),
            y: self.div(rhs.y),
        }
    }
}

impl AddAssign for Vector2 {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        *self = self.add(rhs);
    }
}

impl SubAssign for Vector2 {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        *self = self.sub(rhs);
    }
}

impl MulAssign for Vector2 {
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        *self = self.mul(rhs);
    }
}

impl DivAssign for Vector2 {
    #[inline]
    fn div_assign(&mut self, rhs: Self) {
        *self = self.div(rhs);
    }
}

impl AddAssign<f32> for Vector2 {
    #[inline]
    fn add_assign(&mut self, rhs: f32) {
        *self = self.add(rhs);
    }
}

impl SubAssign<f32> for Vector2 {
    #[inline]
    fn sub_assign(&mut self, rhs: f32) {
        *self = self.sub(rhs);
    }
}

impl MulAssign<f32> for Vector2 {
    #[inline]
    fn mul_assign(&mut self, rhs: f32) {
        *self = self.mul(rhs);
    }
}

impl DivAssign<f32> for Vector2 {
    #[inline]
    fn div_assign(&mut self, rhs: f32) {
        *self = self.div(rhs);
    }
}

impl Mul<Vector2> for &Matrix2x2 {
    type Output = Vector2;
    #[inline]
    fn mul(self, rhs: Vector2) -> Self::Output {
        rhs.transform_with(self)
    }
}

impl Mul<Vector2> for Matrix2x2 {
    type Output = Vector2;
    #[inline]
    fn mul(self, rhs: Vector2) -> Self::Output {
        rhs.transform_with(&self)
    }
}
