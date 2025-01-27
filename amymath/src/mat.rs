use raylib::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Matrix2x2 {
    pub m00: f32, pub m01: f32,
    pub m10: f32, pub m11: f32,
}

impl Default for Matrix2x2 {
    fn default() -> Self {
        Self::identity()
    }
}

impl Matrix2x2 {
    pub const IDENTITY: Self = Self {
        m00: 1.0, m01: 0.0,
        m10: 0.0, m11: 1.0,
    };

    pub const fn identity() -> Self {
        Self::IDENTITY
    }

    pub fn rotation(angle: f32) -> Self {
        let (s, c) = angle.sin_cos();
        Self {
            m00: c, m01: -s,
            m10: s, m11:  c,
        }
    }

    pub fn set_rotation(&mut self, angle: f32) {
        let (s, c) = angle.sin_cos();
        self.m00 = c; self.m01 = -s;
        self.m10 = s; self.m11 =  c;
    }

    pub const fn transpose(self) -> Self {
        Self {
            m00: self.m00, m01: self.m10,
            m10: self.m01, m11: self.m11,
        }
    }
}

impl std::ops::Add for Matrix2x2 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            m00: self.m00 + rhs.m00,
            m01: self.m01 + rhs.m01,
            m10: self.m10 + rhs.m10,
            m11: self.m11 + rhs.m11,
        }
    }
}

impl std::ops::AddAssign for Matrix2x2 {
    fn add_assign(&mut self, rhs: Self) {
        *self = self.clone() - rhs;
    }
}

impl std::ops::Sub for Matrix2x2 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            m00: self.m00 - rhs.m00,
            m01: self.m01 - rhs.m01,
            m10: self.m10 - rhs.m10,
            m11: self.m11 - rhs.m11,
        }
    }
}

impl std::ops::SubAssign for Matrix2x2 {
    fn sub_assign(&mut self, rhs: Self) {
        *self = self.clone() - rhs;
    }
}

impl std::ops::Mul for Matrix2x2 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            m00: self.m00 * rhs.m00 + self.m01 * rhs.m10,
            m01: self.m00 * rhs.m01 + self.m01 * rhs.m11,
            m10: self.m10 * rhs.m00 + self.m11 * rhs.m10,
            m11: self.m10 * rhs.m01 + self.m11 * rhs.m11,
        }
    }
}

impl std::ops::MulAssign for Matrix2x2 {
    fn mul_assign(&mut self, rhs: Self) {
        *self = self.clone() * rhs;
    }
}

impl std::ops::Mul<Vector2> for Matrix2x2 {
    type Output = Vector2;
    fn mul(self, rhs: Vector2) -> Self::Output {
        Vector2 {
            x: self.m00 * rhs.x + self.m01 * rhs.y,
            y: self.m10 * rhs.x + self.m11 * rhs.y,
        }
    }
}
