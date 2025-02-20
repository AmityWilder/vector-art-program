use raylib::prelude::*;
use crate::vector_path::{fill::Fill, stroke::Stroke};

#[derive(Debug, Clone, Copy)]
pub struct Blending {
    pub opacity: f32,
    pub mode: BlendMode,
}

impl Default for Blending {
    fn default() -> Self {
        Self {
            opacity: 1.0,
            mode: BlendMode::BLEND_ALPHA,
        }
    }
}

impl Blending {
    pub fn is_non_default(&self) -> bool {
        (self.opacity - 1.0).abs() > f32::EPSILON || self.mode != BlendMode::BLEND_ALPHA
    }
}

#[derive(Debug, Clone)]
pub enum StyleItem {
    Stroke(Stroke),
    Fill(Fill),
}

#[derive(Default, Clone)]
pub struct Appearance {
    pub blend: Blending,
    pub items: Vec<StyleItem>,
}

impl Appearance {
    pub fn new(blend: Blending, items: Vec<StyleItem>) -> Self {
        Self {
            blend,
            items,
        }
    }
}
