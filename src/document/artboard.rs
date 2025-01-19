use raylib::prelude::*;

#[derive(Debug, Clone, Copy)]
pub struct IntRect2 {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

impl From<IntRect2> for Rectangle {
    fn from(value: IntRect2) -> Self {
        Rectangle {
            x:      value.x      as f32,
            y:      value.y      as f32,
            width:  value.width  as f32,
            height: value.height as f32,
        }
    }
}

impl From<IntRect2> for ffi::Rectangle {
    fn from(value: IntRect2) -> Self {
        ffi::Rectangle {
            x:      value.x      as f32,
            y:      value.y      as f32,
            width:  value.width  as f32,
            height: value.height as f32,
        }
    }
}

pub struct ArtBoard {
    pub name: String,
    pub rect: IntRect2,
}

impl ArtBoard {
    pub fn new(name: String, rect: IntRect2) -> Self {
        Self { name, rect }
    }
}
