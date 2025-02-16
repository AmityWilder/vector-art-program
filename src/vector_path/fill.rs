use amymath::prelude::*;
use raylib::prelude::*;
use crate::appearance::Blending;
use super::gradient::Ramp as GradientRamp;

pub enum GradientStyle {
    Linear,
    Radial,
}

pub enum Pattern {
    Solid(Color),
    Gradient {
        ramp: GradientRamp,
        style: GradientStyle,
    },
}

impl Pattern {
    pub fn draw_preview_rec(&self, d: &mut impl RaylibDraw, rec: &IRect2) {
        let mut d = d.begin_scissor_mode_irect2(rec);
        match self {
            Pattern::Solid(color) => d.draw_rectangle_irect2(rec, *color),
            Pattern::Gradient { ramp, style } => todo!(),
        }
    }
}

pub struct Fill {
    pub blend: Blending,
    pub pattern: Pattern,
}

impl Default for Fill {
    fn default() -> Self {
        Self {
            blend: Blending::default(),
            pattern: Pattern::Solid(Color::SLATEBLUE),
        }
    }
}
