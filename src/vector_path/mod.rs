use std::collections::VecDeque;
use raylib::prelude::*;
use crate::{appearance::{Appearance, StyleItem}, layer::{LayerSettings, LayerType}};

pub mod effect;
pub mod path_point;
pub mod bezier_slice;
pub mod gradient;
pub mod stroke;
pub mod fill;

use bezier_slice::BezierSlice;
use path_point::PathPoint;

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

    pub fn slice(&self, start_index: usize) -> BezierSlice<'_> {
        let end_index = start_index + 1;
        BezierSlice::new(
            &self.points[start_index],
            &self.points[  end_index],
        )
    }

    pub fn get_slice(&self, start_index: usize) -> Option<BezierSlice<'_>> {
        start_index.checked_add(1).and_then(|end_index|
            (end_index < self.points.len()).then(||
                BezierSlice::new(
                    &self.points[start_index],
                    &self.points[  end_index],
                )
            )
        )
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
