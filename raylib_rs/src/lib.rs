mod core;
mod rlgl;
mod utils;

pub use core::{
    RaylibHandle,
    init_window,
};

#[derive(Debug, Clone, Copy, Default)]
// Matrix, 4x4 components, column major, OpenGL style, right handed
pub struct Matrix {
    m0: f32, m4: f32, m8: f32, m12: f32,      // Matrix first row (4 components)
    m1: f32, m5: f32, m9: f32, m13: f32,      // Matrix second row (4 components)
    m2: f32, m6: f32, m10: f32, m14: f32,     // Matrix third row (4 components)
    m3: f32, m7: f32, m11: f32, m15: f32,     // Matrix fourth row (4 components)
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Vector2 { x: f32, y: f32 } // todo

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyboardKey {} // todo
