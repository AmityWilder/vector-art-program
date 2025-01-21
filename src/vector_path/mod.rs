use path_point::{CtrlPt1, CtrlPt2, PathPoint, ReflectVector};
use raylib::prelude::*;
use crate::{appearance::{Appearance, StyleItem}, layer::{LayerSettings, LayerType}};

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
        for item in &self.appearance.items {
            match item {
                StyleItem::Stroke(stroke) => {
                    match (&stroke.thick, &stroke.align, &stroke.pattern) {
                        (
                            stroke::WidthProfile::Constant(thickness),
                            stroke::Align::Middle,
                            stroke::Pattern::Solid(color),
                        ) => {
                            let curve: Vec<_> = self.points.iter().map(|pp| pp.calculate()).collect();
                            for [a, b] in curve.windows(2).map(|w| <[_; 2]>::try_from(w).unwrap()) {
                                d.draw_spline_segment_bezier_cubic(a.1, a.2, b.0, b.1, *thickness, color);
                            }
                            let radius = thickness * 0.5;
                            for pp in &self.points {
                                d.draw_circle_v(pp.p, radius, color);
                            }
                        }

                        _ => todo!(),
                    }
                }

                StyleItem::Fill(_fill) => {
                    // todo
                }
            }
        }
    }

    fn draw_selected(&self, d: &mut impl RaylibDraw, _camera: &Camera2D, zoom_inv: f32) {
        let color = self.settings.color;
        let curve: Vec<_> = self.points.iter().map(|pp| pp.calculate()).collect();
        for [a, b] in curve.windows(2).map(|w| <[_; 2]>::try_from(w).unwrap()) {
            d.draw_spline_segment_bezier_cubic(a.1, a.2, b.0, b.1, zoom_inv, color);
        }
        for pp in &self.points {
            let p = pp.p;
            fn draw_ctrl_exact(d: &mut impl RaylibDraw, root: Vector2, handle: Vector2, head_radius: f32, color: Color) {
                d.draw_line_v(root, handle, color);
                d.draw_circle_v(handle, head_radius, color);
            }
            if let Some(CtrlPt1 { c1: (_, c1), c2 }) = pp.ctrls.as_ref() {
                draw_ctrl_exact(d, p, *c1, 3.0 * zoom_inv, color);
                if let Some(c2) = c2.as_ref() {
                    match c2 {
                        CtrlPt2::Smooth => {
                            let c2 = c1.reflected_over(p);
                            d.draw_line_v(p, c2, color.alpha(0.5));
                            d.draw_ring(c2, 2.0 * zoom_inv, 3.0 * zoom_inv, 0.0, 360.0, 10, color);
                        }
                        CtrlPt2::Mirror(s2) => {
                            let c2 = c1.reflected_to(p, *s2);
                            d.draw_line_v(p, c2, color.alpha(0.5));
                            d.draw_ring(c2, zoom_inv, 3.0 * zoom_inv, 0.0, 360.0, 10, color);
                        }
                        CtrlPt2::Exact(c2) => {
                            draw_ctrl_exact(d, p, *c2, 3.0 * zoom_inv, color);
                        }
                    }
                }
            }
            d.draw_circle_v(p, 4.0 * zoom_inv, color);
        }
    }
}
