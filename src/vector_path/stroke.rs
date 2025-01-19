use std::collections::BTreeMap;
use raylib::prelude::*;
use crate::appearance::Blending;
use super::gradient::Ramp as GradientRamp;

pub enum GradientStyle {
    /// Parallel to the path
    Along,
    /// Perpendicular to the path
    Across,
    /// Gradient ignores the path and acts like a linear fill
    Within,
}

pub struct Gradient {
    pub ramp: GradientRamp,
    pub style: GradientStyle,
}

pub enum Pattern {
    Solid(Color),
    Gradient {
        ramp: GradientRamp,
        style: GradientStyle,
    },
}

pub enum Align {
    Inside,
    Middle,
    Outside,
}

pub enum WidthProfile {
    Constant(f32),
    Variable(BTreeMap<f32, f32>),
}

pub struct Stroke {
    pub blend: Blending,
    pub pattern: Pattern,
    pub thick: WidthProfile,
    pub align: Align,
}

impl Default for Stroke {
    fn default() -> Self {
        Self {
            blend: Blending::default(),
            pattern: Pattern::Solid(Color::BLUEVIOLET),
            thick: WidthProfile::Constant(10.0),
            align: Align::Middle,
        }
    }
}
