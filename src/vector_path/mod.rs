use std::collections::VecDeque;
use amylib::rc::*;
use path_point::PathPoint;
use raylib::prelude::*;
use crate::{appearance::{Appearance, StyleItem}, layer::{LayerSettings, LayerType}};

pub mod effect;
pub mod path_point;
pub mod gradient;
pub mod stroke;
pub mod fill;

pub struct VectorPath {
    pub settings: LayerSettings,
    pub points: VecDeque<PathPoint>,
    pub appearance: Appearance,
    pub is_closed: bool,
}

pub struct BezierSlice<'a> {
    a: &'a PathPoint,
    b: &'a PathPoint,
}

impl BezierSlice<'_> {
    pub fn position_at(&self, t: f32) -> Vector2 {
        let coefs = (
            (1.0 - t).powi(3),
            3.0*(1.0 - t).powi(2)*t,
            3.0*(1.0 - t)*t*t,
            t*t*t,
        );
        let ((_, p0, p1), (p2, p3, _)) = (self.a.calculate(), self.b.calculate());
        Vector2 {
            x: coefs.0*p0.x + coefs.1*p1.x + coefs.2*p2.x + coefs.3*p3.x,
            y: coefs.0*p0.y + coefs.1*p1.y + coefs.2*p2.y + coefs.3*p3.y,
        }
    }

    pub fn velocity_at(&self, t: f32) -> Vector2 {
        let coefs = (
            3.0*(1.0 - t).powi(2),
            6.0*(1.0 - t)*t,
            3.0*t*t,
        );
        let ((_, p0, p1), (p2, p3, _)) = (self.a.calculate(), self.b.calculate());
        Vector2 {
            x: coefs.0*(p1.x - p0.x) + coefs.1*(p2.x - p1.x) + coefs.2*(p3.x - p2.x),
            y: coefs.0*(p1.y - p0.y) + coefs.1*(p2.y - p1.y) + coefs.2*(p3.y - p2.y),
        }
    }

    pub fn acceleration_at(&self, t: f32) -> Vector2 {
        let coefs = (
            6.0*(1.0 - t),
            6.0*t,
        );
        let ((_, p0, p1), (p2, p3, _)) = (self.a.calculate(), self.b.calculate());
        Vector2 {
            x: coefs.0*(p2.x - 2.0*p1.x + p0.x) + coefs.1*(p3.x - 2.0*p2.x + p1.x),
            y: coefs.0*(p2.y - 2.0*p1.y + p0.y) + coefs.1*(p3.y - 2.0*p2.y + p1.y),
        }
    }
}

impl VectorPath {
    pub fn new(settings: LayerSettings) -> Self {
        Self {
            settings,
            points: VecDeque::new(),
            appearance: Appearance::default(),
            is_closed: false,
        }
    }

    pub fn slice(&self, start_index: usize) -> Option<BezierSlice<'_>> {
        self.points.get(start_index)
            .zip(self.points.get(start_index))
            .map(|(a, b)| BezierSlice { a, b })
    }

    pub fn edge_distance_to(&self, pt: Vector2) -> f32 {
        todo!()
    }

    fn calculate(&self) -> Vec<(Vector2, Vector2, Vector2)> {
        let mut iter = self.points.iter().map(|pp| pp.calculate());
        if self.is_closed {
            let first = iter.next();
            first.into_iter().chain(iter.into_iter().chain(first)).collect()
        } else {
            iter.collect()
        }
    }
}

impl LayerType for VectorPath {
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
                            for [a, b] in self.calculate().windows(2).map(|w| <[_; 2]>::try_from(w).unwrap()) {
                                d.draw_spline_segment_bezier_cubic(a.1, a.2, b.0, b.1, *thickness, color);
                            }
                            // cover up cuts between bezier curves
                            let radius = thickness * 0.5;
                            for pp in &self.points {
                                d.draw_circle_v(pp.p, radius, color);
                            }
                        }

                        _ => todo!(),
                    }
                }

                StyleItem::Fill(_fill) => todo!(),
            }
        }
    }

    fn draw_selected(&self, d: &mut impl RaylibDraw, px_world_size: f32) {
        let color = self.settings.color;
        for [a, b] in self.calculate().windows(2).map(|w| <[_; 2]>::try_from(w).unwrap()) {
            d.draw_spline_segment_bezier_cubic(a.1, a.2, b.0, b.1, px_world_size, color);
        }
    }
}
