use path_point::{CtrlPoint, PathPoint};
use raylib::prelude::*;
use crate::{appearance::Appearance, layer::{LayerSettings, LayerType}};

pub mod mat2;
pub mod path_point;
pub mod gradient;
pub mod stroke;
pub mod fill;

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
    pub points: Vec<PathPoint>,
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

    fn draw_rendered(&self, d: &mut impl RaylibDraw) {
        let thickness = 10.0;
        let radius = thickness * 0.5;
        let color = Color::BLACK;
        for window in self.points.windows(2) {
            let [pp1, pp2] = window else { unreachable!("window of 2 should have 2 elements") };
            let (p1, p2) = (pp1.p, pp2.p);
            let c1_out = pp1.c_out.calculate(&p1, &pp1.c_in);
            let c2_in = pp2.c_in.calculate(&p2, &pp2.c_out);
            d.draw_spline_segment_bezier_cubic(p1, c1_out, c2_in, p2, thickness, color);
        }
        for point in &self.points {
            d.draw_circle_v(point.p, radius, color);
        }
    }

    fn draw_selected(&self, d: &mut impl RaylibDraw) {
        let color = self.settings.color;
        for window in self.points.windows(2) {
            let [pp1, pp2] = window else { unreachable!("window of 2 should have 2 elements") };
            let (p1, p2) = (pp1.p, pp2.p);
            let c1_out = pp1.c_out.calculate(&p1, &pp1.c_in);
            let c2_in = pp2.c_in.calculate(&p2, &pp2.c_out);
            d.draw_spline_segment_bezier_cubic(p1, c1_out, c2_in, p2, 1.0, color);
        }
        for pp in &self.points {
            let (c_in, p, c_out) = pp.calculated();
            for (c_self, c_self_ex) in [(&pp.c_in, c_in), (&pp.c_out, c_out)] {
                match c_self {
                    CtrlPoint::Exact(_) => {
                        d.draw_line_v(p, c_self_ex, color);
                        d.draw_circle_v(c_self_ex, 3.0, color);
                    }
                    CtrlPoint::Smooth => {
                        d.draw_line_v(p, c_self_ex, color.alpha(0.5));
                        d.draw_ring(c_self_ex, 2.0, 3.0, 0.0, 360.0, 10, color);
                    }
                    CtrlPoint::Corner => (),
                }
            }
            d.draw_circle_v(p, 4.0, color);
        }
    }
}
