use amymath::prelude::*;
use raylib::prelude::*;
use crate::appearance::Blending;
use super::gradient::Ramp as GradientRamp;

#[derive(Debug, Clone, Copy)]
pub enum GradientStyle {
    Linear,
    Radial,
}

#[derive(Debug, Clone)]
pub enum Pattern {
    Solid(Color),
    Gradient {
        ramp: GradientRamp,
        style: GradientStyle,
    },
}

impl Default for Pattern {
    fn default() -> Self {
        Self::Solid(Color::BLANK)
    }
}

impl Pattern {
    pub fn draw_preview_rec(&self, d: &mut impl RaylibDraw, rec: &IRect2) {
        let mut d = d.begin_scissor_mode_irect2(rec);
        match self {
            Pattern::Solid(color) => d.draw_rectangle_irect2(rec, *color),
            Pattern::Gradient { .. } => todo!(),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct Fill {
    pub blend: Blending,
    pub pattern: Pattern,
}
