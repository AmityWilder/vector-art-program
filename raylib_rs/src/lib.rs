#![feature(extern_types, cfg_match)]

mod external;
mod core;
mod rlgl;
mod utils;

pub use core::{
    RaylibHandle,
    init_window,
};

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, Default)]
pub(crate) struct rl_float16 {
    v: [f32; 16],
}

#[derive(Debug, Clone, Copy, Default)]
// Matrix, 4x4 components, column major, OpenGL style, right handed
pub struct Matrix {
    m0: f32, m4: f32, m8: f32, m12: f32,      // Matrix first row (4 components)
    m1: f32, m5: f32, m9: f32, m13: f32,      // Matrix second row (4 components)
    m2: f32, m6: f32, m10: f32, m14: f32,     // Matrix third row (4 components)
    m3: f32, m7: f32, m11: f32, m15: f32,     // Matrix fourth row (4 components)
}

impl From<Matrix> for rl_float16 {
    /// Get float array of matrix data
    fn from(mat: Matrix) -> Self {
        rl_float16 {
            v: [
                mat.m0,
                mat.m1,
                mat.m2,
                mat.m3,
                mat.m4,
                mat.m5,
                mat.m6,
                mat.m7,
                mat.m8,
                mat.m9,
                mat.m10,
                mat.m11,
                mat.m12,
                mat.m13,
                mat.m14,
                mat.m15,
            ]
        }
    }
}

impl Matrix {
    pub const IDENTITY: Self = Self {
        m0: 1.0, m4: 0.0, m8: 0.0, m12: 0.0,
        m1: 0.0, m5: 1.0, m9: 0.0, m13: 0.0,
        m2: 0.0, m6: 0.0, m10: 1.0, m14: 0.0,
        m3: 0.0, m7: 0.0, m11: 0.0, m15: 1.0,
    };

    /// Transposes provided matrix
    pub fn transpose(&self) -> Self {
        Self {
            m0:  self.m0,
            m1:  self.m4,
            m2:  self.m8,
            m3:  self.m12,
            m4:  self.m1,
            m5:  self.m5,
            m6:  self.m9,
            m7:  self.m13,
            m8:  self.m2,
            m9:  self.m6,
            m10: self.m10,
            m11: self.m14,
            m12: self.m3,
            m13: self.m7,
            m14: self.m11,
            m15: self.m15,
        }
    }

    /// Invert provided matrix
    pub fn invert(&self) -> Self {
        // Cache the matrix values (speed optimization)
        let (a00, a01, a02, a03) = (self.m0,  self.m1,  self.m2,  self.m3 );
        let (a10, a11, a12, a13) = (self.m4,  self.m5,  self.m6,  self.m7 );
        let (a20, a21, a22, a23) = (self.m8,  self.m9,  self.m10, self.m11);
        let (a30, a31, a32, a33) = (self.m12, self.m13, self.m14, self.m15);

        let b00 = a00*a11 - a01*a10;
        let b01 = a00*a12 - a02*a10;
        let b02 = a00*a13 - a03*a10;
        let b03 = a01*a12 - a02*a11;
        let b04 = a01*a13 - a03*a11;
        let b05 = a02*a13 - a03*a12;
        let b06 = a20*a31 - a21*a30;
        let b07 = a20*a32 - a22*a30;
        let b08 = a20*a33 - a23*a30;
        let b09 = a21*a32 - a22*a31;
        let b10 = a21*a33 - a23*a31;
        let b11 = a22*a33 - a23*a32;

        // Calculate the invert determinant (inlined to avoid double-caching)
        let inv_det = (b00*b11 - b01*b10 + b02*b09 + b03*b08 - b04*b07 + b05*b06).recip();

        Self {
            m0:  ( a11*b11 - a12*b10 + a13*b09) * inv_det,
            m1:  (-a01*b11 + a02*b10 - a03*b09) * inv_det,
            m2:  ( a31*b05 - a32*b04 + a33*b03) * inv_det,
            m3:  (-a21*b05 + a22*b04 - a23*b03) * inv_det,
            m4:  (-a10*b11 + a12*b08 - a13*b07) * inv_det,
            m5:  ( a00*b11 - a02*b08 + a03*b07) * inv_det,
            m6:  (-a30*b05 + a32*b02 - a33*b01) * inv_det,
            m7:  ( a20*b05 - a22*b02 + a23*b01) * inv_det,
            m8:  ( a10*b10 - a11*b08 + a13*b06) * inv_det,
            m9:  (-a00*b10 + a01*b08 - a03*b06) * inv_det,
            m10: ( a30*b04 - a31*b02 + a33*b00) * inv_det,
            m11: (-a20*b04 + a21*b02 - a23*b00) * inv_det,
            m12: (-a10*b09 + a11*b07 - a12*b06) * inv_det,
            m13: ( a00*b09 - a01*b07 + a02*b06) * inv_det,
            m14: (-a30*b03 + a31*b01 - a32*b00) * inv_det,
            m15: ( a20*b03 - a21*b01 + a22*b00) * inv_det,
        }
    }
}

impl std::ops::Mul for Matrix {
    type Output = Self;

    /// Get two matrix multiplication
    /// NOTE: When multiplying matrices... the order matters!
    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            m0:  self.m0  * rhs.m0 + self.m1  * rhs.m4 + self.m2  * rhs.m8  + self.m3  * rhs.m12,
            m1:  self.m0  * rhs.m1 + self.m1  * rhs.m5 + self.m2  * rhs.m9  + self.m3  * rhs.m13,
            m2:  self.m0  * rhs.m2 + self.m1  * rhs.m6 + self.m2  * rhs.m10 + self.m3  * rhs.m14,
            m3:  self.m0  * rhs.m3 + self.m1  * rhs.m7 + self.m2  * rhs.m11 + self.m3  * rhs.m15,
            m4:  self.m4  * rhs.m0 + self.m5  * rhs.m4 + self.m6  * rhs.m8  + self.m7  * rhs.m12,
            m5:  self.m4  * rhs.m1 + self.m5  * rhs.m5 + self.m6  * rhs.m9  + self.m7  * rhs.m13,
            m6:  self.m4  * rhs.m2 + self.m5  * rhs.m6 + self.m6  * rhs.m10 + self.m7  * rhs.m14,
            m7:  self.m4  * rhs.m3 + self.m5  * rhs.m7 + self.m6  * rhs.m11 + self.m7  * rhs.m15,
            m8:  self.m8  * rhs.m0 + self.m9  * rhs.m4 + self.m10 * rhs.m8  + self.m11 * rhs.m12,
            m9:  self.m8  * rhs.m1 + self.m9  * rhs.m5 + self.m10 * rhs.m9  + self.m11 * rhs.m13,
            m10: self.m8  * rhs.m2 + self.m9  * rhs.m6 + self.m10 * rhs.m10 + self.m11 * rhs.m14,
            m11: self.m8  * rhs.m3 + self.m9  * rhs.m7 + self.m10 * rhs.m11 + self.m11 * rhs.m15,
            m12: self.m12 * rhs.m0 + self.m13 * rhs.m4 + self.m14 * rhs.m8  + self.m15 * rhs.m12,
            m13: self.m12 * rhs.m1 + self.m13 * rhs.m5 + self.m14 * rhs.m9  + self.m15 * rhs.m13,
            m14: self.m12 * rhs.m2 + self.m13 * rhs.m6 + self.m14 * rhs.m10 + self.m15 * rhs.m14,
            m15: self.m12 * rhs.m3 + self.m13 * rhs.m7 + self.m14 * rhs.m11 + self.m15 * rhs.m15,
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Vector2 {
    x: f32,
    y: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyboardKey {} // todo
