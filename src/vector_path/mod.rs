use std::collections::VecDeque;
use path_point::PathPoint;
use raylib::prelude::*;
use crate::{appearance::{Appearance, StyleItem}, layer::{LayerSettings, LayerType}};

pub mod mat2;
pub mod effect;
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
    pub points: VecDeque<PathPoint>,
    pub appearance: Appearance,
    pub is_closed: bool,
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

    fn draw_selected(&self, d: &mut impl RaylibDraw, _camera: &Camera2D, zoom_inv: f32) {
        let color = self.settings.color;
        for [a, b] in self.calculate().windows(2).map(|w| <[_; 2]>::try_from(w).unwrap()) {
            d.draw_spline_segment_bezier_cubic(a.1, a.2, b.0, b.1, zoom_inv, color);
        }
        for pp in &self.points {
            pp.draw_selected(d, zoom_inv, color);
        }
    }
}
