use std::simd::{f32x2, f32x4, usizex4, StdFloat};
use crate::vector::Vec2;

#[derive(Debug, Clone, Copy)]
pub struct Matrix2x2(f32x4);

impl Default for Matrix2x2 {
    #[inline]
    fn default() -> Self {
        Self::IDENTITY
    }
}

impl Matrix2x2 {
    pub const IDENTITY: Self = Self::new(
        1.0, 0.0,
        0.0, 1.0,
    );

    #[inline]
    pub const fn new(
        m00: f32, m01: f32,
        m10: f32, m11: f32,
    ) -> Self {
        Self(f32x4::from_array([m00, m01, m10, m11]))
    }

    #[inline]
    pub const fn into_rows(self) -> [Vec2; 2] {
        let [m00, m01, m10, m11] = self.0.to_array();
        [Vec2(f32x2::from_array([m00, m01])),
         Vec2(f32x2::from_array([m10, m11]))]
    }

    #[inline]
    pub const fn into_cols(self) -> [Vec2; 2] {
        let [m00, m01, m10, m11] = self.0.to_array();
        [Vec2(f32x2::from_array([m00, m10])),
         Vec2(f32x2::from_array([m01, m11]))]
    }
}

impl std::ops::Neg for Matrix2x2 {
    type Output = Self;
    /// Wrapping
    #[inline]
    fn neg(self) -> Self::Output {
        Self(-self.0)
    }
}

impl std::ops::Add for Matrix2x2 {
    type Output = Self;
    /// Wrapping
    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl std::ops::Sub for Matrix2x2 {
    type Output = Self;
    /// Wrapping
    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

// impl std::ops::Mul for Matrix2x2 {
//     type Output = Self;
//     /// Wrapping
//     #[inline]
//     fn mul(self, rhs: Self) -> Self::Output {
//         let [m0c0, m0c1] = self.into_cols();
//         let [m1r0, m1r1] = self.into_rows();
//         let r0 = m0c0 * m1r0;
//         Self(
//             m0c0 * m1r0,
//             m0c1 * m1r1,
//         )
//     }
// }

impl std::ops::Mul<Vec2> for Matrix2x2 {
    type Output = Vec2;
    #[inline]
    fn mul(self, rhs: Vec2) -> Self::Output {
        let [r0, r1] = self.into_rows();
        Vec2::new(
            r0.dot(rhs),
            r1.dot(rhs),
        )
    }
}
