use amyvec::{curve::{Curve, WidthProfile}, path_point::{Ctrl, Ctrl1, PathPoint}};
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
            pattern: Pattern::Solid(Color::BLACK),
            // thick: WidthProfile::Constant(Vector2::new(10.0, 10.0)),
            thick: WidthProfile::Variable({
                let mut c = Curve::new();
                c.points.push_back(PathPoint { p: Vector2::new(10.0, 10.0), c: Some(Ctrl1 { c1: (Ctrl::Out, Vector2::new(10.0, 5.0)), c2: None }) });
                c.points.push_back(PathPoint { p: Vector2::new(1.0, 1.0), c: None });
                c
            }),
            align: Align::Middle,
        }
    }
}
