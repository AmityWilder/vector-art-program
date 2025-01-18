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

    fn draw_rendered(&self, _d: &mut impl RaylibDraw) {
        // todo
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
            match pp.c_in {
                CtrlPoint::Exact(c_in) => {
                    d.draw_line_v(p, c_in, color);
                    d.draw_circle_v(c_in, 3.0, color);
                }
                CtrlPoint::Smooth => {
                    d.draw_line_v(p, c_in, color.alpha(0.5));
                    d.draw_circle_lines(c_in.x as i32, c_in.y as i32, 3.0, color);
                }
                CtrlPoint::Corner => (),
            }
            match pp.c_out {
                CtrlPoint::Exact(c_out) => {
                    d.draw_line_v(p, c_out, color);
                    d.draw_circle_v(c_out, 3.0, color);
                }
                CtrlPoint::Smooth => {
                    d.draw_line_v(p, c_out, color.alpha(0.5));
                    d.draw_circle_lines(c_out.x as i32, c_out.y as i32, 3.0, color);
                }
                CtrlPoint::Corner => (),
            }
            d.draw_circle_v(p, 4.0, color);
        }
    }
}
