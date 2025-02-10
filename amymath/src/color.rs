use raylib::prelude::*;

const FRAC_1_255: f32 = 1.0 / 255.0;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct NormalizedColor {
    r: f32,
    g: f32,
    b: f32,
    a: f32,
}

impl NormalizedColor {
    pub fn to_color(self) -> Color {
        Color {
            r: (self.r * 255.0).clamp(0.0, 255.0) as u8,
            g: (self.g * 255.0).clamp(0.0, 255.0) as u8,
            b: (self.b * 255.0).clamp(0.0, 255.0) as u8,
            a: (self.a * 255.0).clamp(0.0, 255.0) as u8,
        }
    }

    #[inline]
    pub fn saturate(self) -> Self {
        Self {
            r: self.r.clamp(0.0, 1.0),
            g: self.g.clamp(0.0, 1.0),
            b: self.b.clamp(0.0, 1.0),
            a: self.a.clamp(0.0, 1.0),
        }
    }

    #[inline]
    pub fn add_rgb(self, rhs: f32) -> Self {
        Self {
            r: self.r + rhs,
            g: self.g + rhs,
            b: self.b + rhs,
            a: self.a,
        }
    }

    #[inline]
    pub fn add_rgba(self, rhs: f32) -> Self {
        Self {
            r: self.r + rhs,
            g: self.g + rhs,
            b: self.b + rhs,
            a: self.a + rhs,
        }
    }

    #[inline]
    pub fn sub_rgb(self, rhs: f32) -> Self {
        Self {
            r: self.r - rhs,
            g: self.g - rhs,
            b: self.b - rhs,
            a: self.a,
        }
    }

    #[inline]
    pub fn sub_rgba(self, rhs: f32) -> Self {
        Self {
            r: self.r - rhs,
            g: self.g - rhs,
            b: self.b - rhs,
            a: self.a - rhs,
        }
    }

    #[inline]
    pub fn mul_rgb(self, rhs: f32) -> Self {
        Self {
            r: self.r * rhs,
            g: self.g * rhs,
            b: self.b * rhs,
            a: self.a,
        }
    }

    #[inline]
    pub fn mul_rgba(self, rhs: f32) -> Self {
        Self {
            r: self.r * rhs,
            g: self.g * rhs,
            b: self.b * rhs,
            a: self.a * rhs,
        }
    }

    #[inline]
    pub fn div_rgb(self, rhs: f32) -> Self {
        let rhs_inv = rhs.recip();
        Self {
            r: self.r * rhs_inv,
            g: self.g * rhs_inv,
            b: self.b * rhs_inv,
            a: self.a,
        }
    }

    #[inline]
    pub fn div_rgba(self, rhs: f32) -> Self {
        let rhs_inv = rhs.recip();
        Self {
            r: self.r * rhs_inv,
            g: self.g * rhs_inv,
            b: self.b * rhs_inv,
            a: self.a * rhs_inv,
        }
    }
}

impl std::ops::Add for NormalizedColor {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
            a: self.a + rhs.a,
        }
    }
}

impl std::ops::Sub for NormalizedColor {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            r: self.r - rhs.r,
            g: self.g - rhs.g,
            b: self.b - rhs.b,
            a: self.a - rhs.a,
        }
    }
}

impl std::ops::Mul for NormalizedColor {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            r: self.r * rhs.r,
            g: self.g * rhs.g,
            b: self.b * rhs.b,
            a: self.a * rhs.a,
        }
    }
}

impl std::ops::Div for NormalizedColor {
    type Output = Self;
    #[inline]
    fn div(self, rhs: Self) -> Self::Output {
        Self {
            r: self.r / rhs.r,
            g: self.g / rhs.g,
            b: self.b / rhs.b,
            a: self.a / rhs.a,
        }
    }
}

impl std::ops::AddAssign for NormalizedColor {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl std::ops::SubAssign for NormalizedColor {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl std::ops::MulAssign for NormalizedColor {
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl std::ops::DivAssign for NormalizedColor {
    #[inline]
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

pub trait ColorMath {
    fn mix(self, other: Self, amount: f32) -> Self;
    fn add(self, rhs: Self) -> Self;
    fn sub(self, rhs: Self) -> Self;
    fn mul(self, rhs: Self) -> Self;
    fn div(self, rhs: Self) -> Self;
    fn to_normalized(self) -> NormalizedColor;
}

impl ColorMath for Color {
    #[inline]
    fn mix(self, other: Self, amount: f32) -> Self {
        Self {
            r: lerp(self.r as f32, other.r as f32, amount) as u8,
            g: lerp(self.g as f32, other.g as f32, amount) as u8,
            b: lerp(self.b as f32, other.b as f32, amount) as u8,
            a: lerp(self.a as f32, other.a as f32, amount) as u8,
        }
    }

    #[inline]
    fn add(self, rhs: Self) -> Self {
        Self {
            r: self.r.saturating_add(rhs.r),
            g: self.g.saturating_add(rhs.g),
            b: self.b.saturating_add(rhs.b),
            a: self.a.saturating_add(rhs.a),
        }
    }

    #[inline]
    fn sub(self, rhs: Self) -> Self {
        Self {
            r: self.r.saturating_sub(rhs.r),
            g: self.g.saturating_sub(rhs.g),
            b: self.b.saturating_sub(rhs.b),
            a: self.a.saturating_sub(rhs.a),
        }
    }

    #[inline]
    fn mul(self, rhs: Self) -> Self {
        Self {
            r: ((self.r as f32 * rhs.r as f32) * FRAC_1_255) as u8,
            g: ((self.g as f32 * rhs.g as f32) * FRAC_1_255) as u8,
            b: ((self.b as f32 * rhs.b as f32) * FRAC_1_255) as u8,
            a: ((self.a as f32 * rhs.a as f32) * FRAC_1_255) as u8,
        }
    }

    #[inline]
    fn div(self, rhs: Self) -> Self {
        Self {
            r: (self.r as f32 / rhs.r as f32) as u8,
            g: (self.g as f32 / rhs.g as f32) as u8,
            b: (self.b as f32 / rhs.b as f32) as u8,
            a: (self.a as f32 / rhs.a as f32) as u8,
        }
    }

    #[inline]
    fn to_normalized(self) -> NormalizedColor {
        NormalizedColor {
            r: self.r as f32 * FRAC_1_255,
            g: self.g as f32 * FRAC_1_255,
            b: self.b as f32 * FRAC_1_255,
            a: self.a as f32 * FRAC_1_255,
        }
    }
}
