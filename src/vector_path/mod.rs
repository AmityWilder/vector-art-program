use amyvec::path_point::{Ctrl, Ctrl1, Ctrl2};
use raylib::prelude::*;
use crate::{
    appearance::{Appearance, StyleItem},
    document::layer::{LayerSettings, LayerType},
};

pub mod effect;
pub mod gradient;
pub mod stroke;
pub mod fill;
pub use amyvec::{
    curve,
    path_point,
};

use curve::Curve;
use path_point::PathPoint;

pub struct VectorPath {
    pub settings: LayerSettings,
    pub curve: Curve,
    pub appearance: Appearance,
}

impl VectorPath {
    pub fn new(settings: LayerSettings) -> Self {
        Self {
            settings,
            curve: Curve::new(),
            appearance: Appearance::default(),
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
                            for bez in self.curve.slices() {
                                d.draw_spline_segment_bezier_cubic(bez.p1, bez.c1_out, bez.c2_in, bez.p2, *thickness, color);
                            }
                            // cover up cuts between bezier curves
                            let radius = thickness * 0.5;
                            for pp in &self.curve.points {
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
        for bez in self.curve.slices() {
            d.draw_spline_segment_bezier_cubic(bez.p1, bez.c1_out, bez.c2_in, bez.p2, px_world_size, color);
        }
    }
}

pub trait DrawPathPoint: RaylibDraw {
    fn draw_path_point(
        &mut self,
        pp: &PathPoint,
        px_world_size: f32,
        color: Color,
        is_selected: bool,
        is_c_in_vis: bool,
        is_c_out_vis: bool,
    ) {
        if let Some(Ctrl1 { c1: (c1_side, c1), c2 }) = pp.c.as_ref() {
            let (is_c1_vis, is_c2_vis) = match c1_side {
                Ctrl::In => (is_c_in_vis, is_c_out_vis),
                Ctrl::Out => (is_c_out_vis, is_c_in_vis),
            };

            if is_c1_vis {
                self.draw_line_v(pp.p, c1, color);
                self.draw_circle_sector(c1, 3.5 * px_world_size, 0.0, 360.0, 10, color);
            }

            if is_c2_vis {
                if let Some(c2_ty) = c2.as_ref() {
                    let c2 = c2_ty.calculate(pp.p, *c1);
                    match c2_ty {
                        Ctrl2::Exact(_) => {
                            self.draw_line_v(pp.p, c2, color);
                            self.draw_circle_sector(c2, 3.5 * px_world_size, 0.0, 360.0, 10, color);
                        }

                        Ctrl2::Mirror(_) => {
                            self.draw_line_v(pp.p, c2, color.alpha(0.5));
                            let radius = 3.0 * px_world_size;
                            let size = 2.0 * radius;
                            self.draw_rectangle_pro(Rectangle::new(c2.x, c2.y, size, size), Vector2::new(radius, radius), 45.0, color);
                        }

                        Ctrl2::Transformed(_) => {
                            self.draw_line_v(pp.p, c2, color.alpha(0.5));
                            let radius = 3.0 * px_world_size;
                            const FRAC_SQRT_3_2: f32 = 0.86602540378;
                            self.draw_triangle(
                                Vector2::new( FRAC_SQRT_3_2,  0.5) * radius,
                                Vector2::new(-FRAC_SQRT_3_2,  0.5) * radius,
                                Vector2::new(           0.0, -1.0) * radius,
                                color
                            );
                        }

                        Ctrl2::Reflect => {
                            self.draw_line_v(pp.p, c2, color.alpha(0.5));
                            self.draw_ring(c2, 2.5 * px_world_size, 3.5 * px_world_size, 0.0, 360.0, 10, color);
                        }
                    }
                }
            }
        }

        const ANCHOR_EXTENT_INNER: f32 = 2.0;
        const ANCHOR_OUTLINE_THICK: f32 = 1.0;
        const ANCHOR_EXTENT_OUTER: f32 = ANCHOR_EXTENT_INNER + ANCHOR_OUTLINE_THICK;
        const ANCHOR_SIZE_INNER: f32 = ANCHOR_EXTENT_INNER * 2.0;
        const ANCHOR_SIZE_OUTER: f32 = ANCHOR_EXTENT_OUTER * 2.0;

        // outline
        {
            let rec_outer = Rectangle::new(
                pp.p.x - ANCHOR_EXTENT_OUTER * px_world_size,
                pp.p.y - ANCHOR_EXTENT_OUTER * px_world_size,
                ANCHOR_SIZE_OUTER * px_world_size,
                ANCHOR_SIZE_OUTER * px_world_size,
            );
            self.draw_rectangle_rec(rec_outer, color);
        }

        // fill
        if !is_selected {
            let rec_inner = Rectangle::new(
                pp.p.x - ANCHOR_EXTENT_INNER * px_world_size,
                pp.p.y - ANCHOR_EXTENT_INNER * px_world_size,
                ANCHOR_SIZE_INNER * px_world_size,
                ANCHOR_SIZE_INNER * px_world_size,
            );
            self.draw_rectangle_rec(rec_inner, Color::WHITE);
        }
    }
}

impl<D: RaylibDraw> DrawPathPoint for D {}
