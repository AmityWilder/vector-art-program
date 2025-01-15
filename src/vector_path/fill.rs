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

pub struct Fill {
    pub blend: Blending,
    pub pattern: Pattern,
}

impl Default for Fill {
    fn default() -> Self {
        Self {
            blend: Blending::default(),
            pattern: Pattern::Solid(Color::WHITE),
        }
    }
}
