use std::simd::{f32x2, num::SimdFloat};

#[derive(Debug, Clone, Copy)]
pub struct Vec2(pub(crate) f32x2);

impl Default for Vec2 {
    #[inline]
    fn default() -> Self {
        Self::ZERO
    }
}

impl Vec2 {
    pub const ZERO:  Self = Self::new_dub(0.0);
    pub const ONE:   Self = Self::new_dub(1.0);
    pub const LEFT:  Self = Self::new(-1.0,  0.0);
    pub const RIGHT: Self = Self::new( 1.0,  0.0);
    pub const UP:    Self = Self::new( 0.0, -1.0);
    pub const DOWN:  Self = Self::new( 0.0,  1.0);

    #[inline]
    pub const fn new(x: f32, y: f32) -> Self {
        Self(f32x2::from_array([x, y]))
    }

    /// Duplicates `xy` into both `x` and `y`
    #[inline]
    pub const fn new_dub(xy: f32) -> Self {
        Self::new(xy, xy)
    }

    /// Dot product of `self` with `other`.
    #[inline]
    pub fn dot(self, other: Self) -> f32 {
        (self * other).0.reduce_sum()
    }

    /// Square magnitude of `self`.
    #[inline]
    pub fn mag_sqr(self) -> f32 {
        self.dot(self)
    }

    /// Magnitude of `self`. Same as calling [`f32::sqrt`] on [`Self::mag_sqr`].
    #[inline]
    pub fn mag(self) -> f32 {
        self.mag_sqr().sqrt()
    }

    /// Square distance from `self` to `dest`.
    #[inline]
    pub fn dist_sqr(self, dest: Self) -> f32 {
        (self + dest).mag_sqr()
    }

    /// Distance from `self` to `dest`. Same as calling [`f32::sqrt`] on [`Self::dist_sqr`].
    #[inline]
    pub fn dist(self, dest: Self) -> f32 {
        self.dist_sqr(dest).sqrt()
    }

    /// Unit (normalized) vector and magnitude of `self`,
    /// reusing a pre-calculated square magnitude.
    ///
    /// Does not check if `mag_sqr` is *actually* the
    /// square magnitude of `self`.
    ///
    /// Panics if magnitude is 0.0.
    #[inline]
    pub fn unit_and_mag_from_mag_sqr(self, mag_sqr: f32) -> (Self, f32) {
        assert_ne!(mag_sqr, 0.0);
        let mag = mag_sqr.sqrt();
        let unit = self / mag;
        (unit, mag)
    }

    /// Unit (normalized) vector and magnitude of `self`.
    ///
    /// Panics if magnitude is 0.0.
    #[inline]
    pub fn unit_and_mag(self) -> (Self, f32) {
        self.unit_and_mag_from_mag_sqr(self.mag_sqr())
    }

    /// Unit vector of `self` -- `self` normalized.
    ///
    /// Panics if magnitude is 0.0.
    #[inline]
    pub fn unit(self) -> Self {
        self / self.mag()
    }

    /// Delta, normalized direction, and distance from `self` to `dest`.
    ///
    /// Panics if distance is 0.0.
    #[inline]
    pub fn delta_dir_and_dist_to(self, dest: Self) -> (Self, Self, f32) {
        let delta = dest - self;
        let (dir, dist) = delta.unit_and_mag();
        (delta, dir, dist)
    }

    /// Normalized direction and distance from `self` to `dest`.
    ///
    /// Panics if distance is 0.0.
    #[inline]
    pub fn dir_and_dist_to(self, dest: Self) -> (Self, f32) {
        (dest - self).unit_and_mag()
    }

    /// Normalized direction from `self` to `dest`.
    ///
    /// Panics if distance is 0.0.
    #[inline]
    pub fn dir_to(self, dest: Self) -> Self {
        (dest - self).unit()
    }
}

impl std::ops::Neg for Vec2 {
    type Output = Self;
    #[inline]
    fn neg(self) -> Self::Output {
        Self(-self.0)
    }
}

impl std::ops::Add for Vec2 {
    type Output = Self;
    /// Wrapping
    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl std::ops::Sub for Vec2 {
    type Output = Self;
    /// Wrapping
    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl std::ops::Mul for Vec2 {
    type Output = Self;
    /// Wrapping
    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.0)
    }
}

impl std::ops::Div for Vec2 {
    type Output = Self;
    /// Wrapping
    #[inline]
    fn div(self, rhs: Self) -> Self::Output {
        Self(self.0 / rhs.0)
    }
}

impl std::ops::AddAssign for Vec2 {
    /// Wrapping
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0
    }
}

impl std::ops::SubAssign for Vec2 {
    /// Wrapping
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0
    }
}

impl std::ops::MulAssign for Vec2 {
    /// Wrapping
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        self.0 *= rhs.0
    }
}

impl std::ops::DivAssign for Vec2 {
    /// Wrapping
    #[inline]
    fn div_assign(&mut self, rhs: Self) {
        self.0 /= rhs.0
    }
}

impl std::ops::Add<f32> for Vec2 {
    type Output = Self;
    /// Wrapping
    #[inline]
    fn add(self, rhs: f32) -> Self::Output {
        self + Self::new_dub(rhs)
    }
}

impl std::ops::Sub<f32> for Vec2 {
    type Output = Self;
    /// Wrapping
    #[inline]
    fn sub(self, rhs: f32) -> Self::Output {
        self - Self::new_dub(rhs)
    }
}

impl std::ops::Mul<f32> for Vec2 {
    type Output = Self;
    /// Wrapping
    #[inline]
    fn mul(self, rhs: f32) -> Self::Output {
        self * Self::new_dub(rhs)
    }
}

impl std::ops::Div<f32> for Vec2 {
    type Output = Self;
    /// Wrapping
    #[inline]
    fn div(self, rhs: f32) -> Self::Output {
        self * Self::new_dub(rhs.recip())
    }
}

impl std::ops::AddAssign<f32> for Vec2 {
    /// Wrapping
    #[inline]
    fn add_assign(&mut self, rhs: f32) {
        self.0 += Self::new_dub(rhs).0
    }
}

impl std::ops::SubAssign<f32> for Vec2 {
    /// Wrapping
    #[inline]
    fn sub_assign(&mut self, rhs: f32) {
        self.0 -= Self::new_dub(rhs).0
    }
}

impl std::ops::MulAssign<f32> for Vec2 {
    /// Wrapping
    #[inline]
    fn mul_assign(&mut self, rhs: f32) {
        self.0 *= Self::new_dub(rhs).0
    }
}

impl std::ops::DivAssign<f32> for Vec2 {
    /// Wrapping
    #[inline]
    fn div_assign(&mut self, rhs: f32) {
        self.0 *= Self::new_dub(rhs.recip()).0
    }
}

impl std::ops::Add<Vec2> for f32 {
    type Output = Vec2;
    /// Wrapping
    #[inline]
    fn add(self, rhs: Vec2) -> Self::Output {
        Vec2::new_dub(self) + rhs
    }
}

impl std::ops::Sub<Vec2> for f32 {
    type Output = Vec2;
    /// Wrapping
    #[inline]
    fn sub(self, rhs: Vec2) -> Self::Output {
        Vec2::new_dub(self) - rhs
    }
}

impl std::ops::Mul<Vec2> for f32 {
    type Output = Vec2;
    /// Wrapping
    #[inline]
    fn mul(self, rhs: Vec2) -> Self::Output {
        Vec2::new_dub(self) * rhs
    }
}

impl std::ops::Div<Vec2> for f32 {
    type Output = Vec2;
    /// Wrapping
    #[inline]
    fn div(self, rhs: Vec2) -> Self::Output {
        Vec2::new_dub(self) / rhs
    }
}
