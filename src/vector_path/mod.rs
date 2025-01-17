use raylib::prelude::*;

pub mod gradient;
pub mod stroke;
pub mod fill;

use crate::{appearance::Appearance, layer::{LayerSettings, LayerType}};

fn mix(c0: &Color, c1: &Color, amount: f32) -> Color {
    Color {
        r: lerp(c0.r as f32, c1.r as f32, amount) as u8,
        g: lerp(c0.g as f32, c1.g as f32, amount) as u8,
        b: lerp(c0.b as f32, c1.b as f32, amount) as u8,
        a: lerp(c0.a as f32, c1.a as f32, amount) as u8,
    }
}

pub struct VectorPath {
    pub settings: LayerSettings,
    /// p1, c2, c3, p4, c5, c6...
    pub points: Vec<(Vector2, Vector2, Vector2)>,
    pub appearance: Appearance,
}

impl VectorPath {
    pub fn new(settings: LayerSettings) -> Self {
        Self {
            settings,
            points: Vec::new(),
            appearance: Appearance::default(),
        }
    }
}

impl LayerType for VectorPath {
    fn settings(&self) -> &LayerSettings {
        &self.settings
    }

    fn settings_mut(&mut self) -> &mut LayerSettings {
        &mut self.settings
    }

    fn draw_rendered(&self, _d: &mut impl RaylibDraw) {
        // todo
    }

    fn draw_selected(&self, d: &mut impl RaylibDraw) {
        let color = self.settings.color;
        for window in self.points.windows(2) {
            if let [(_c1_in, p1, c1_out), (c2_in, p2, _c2_out)] = window {
                d.draw_spline_segment_bezier_cubic(*p1, *c1_out, *c2_in, *p2, 1.0, color);
            }
        }
        for (c_in, p, c_out) in &self.points {
            d.draw_line_v(c_in, p, color);
            d.draw_line_v(p, c_out, color);
            d.draw_circle_v(c_in, 3.0, color);
            d.draw_circle_v(c_out, 3.0, color);
            d.draw_circle_v(p, 4.0, color);
        }
    }
}
