use raylib::prelude::*;
use crate::vector_path::{fill::Fill, stroke::Stroke};

#[derive(Clone, Copy)]
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

pub enum StyleItem {
    Stroke(Stroke),
    Fill(Fill),
}

pub struct Appearance {
    pub items: Vec<StyleItem>,
}

impl Default for Appearance {
    fn default() -> Self {
        Self {
            items: vec![
                StyleItem::Stroke(Stroke::default()),
            ],
        }
    }
}

impl Appearance {
    pub fn new(items: Vec<StyleItem>) -> Self {
        Self {
            items,
        }
    }
}
