use amymath::prelude::*;
use amyvec::curve::WidthProfile;
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

impl Pattern {
    pub fn draw_preview_rec(&self, d: &mut impl RaylibDraw, rec: &IRect2) {
        let mut d = d.begin_scissor_mode_irect2(rec);
        match self {
            Pattern::Solid(color) => d.draw_rectangle_irect2(rec, *color),
            Pattern::Gradient { ramp, style } => todo!(),
        }
    }
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
            pattern: Pattern::Solid(Color::new(10, 10, 10, 255)),
            thick: WidthProfile::Constant(10.0, 10.0),
            // thick: WidthProfile::init()
            //     .with_point(0.0, 15.0, 5.0)
            //     .with_point(0.5, 1.0, 1.0)
            //     .with_point(1.0, 1.0, 8.0)
            //     .build(),
            align: Align::Middle,
        }
    }
}
