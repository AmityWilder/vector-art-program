use amyvec::{curve::WidthProfile, path_point::{Ctrl, Ctrl1, Ctrl2}};
use raylib::prelude::*;
use raylib_rs::rlgl::*;
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

const FRAC_SQRT_3_2: f32 = std::f32::consts::SQRT_3 * 0.5;

pub struct VectorPath {
    pub settings: LayerSettings,
    pub curve: Curve,
    pub appearance: Appearance,
}

impl VectorPath {
    pub fn new(settings: LayerSettings, appearance: Appearance) -> Self {
        Self {
            settings,
            curve: Curve::new(),
            appearance,
        }
    }
}

impl LayerType for VectorPath {
    fn draw_rendered(&self, d: &mut impl RaylibDraw, camera: &Camera2D, scratch_rtex: &mut [RenderTexture2D]) {
        fn draw_inner(path: &VectorPath, d: &mut impl RaylibDraw) {
            for item in &path.appearance.items {
                match item {
                    StyleItem::Stroke(stroke) => {
                        match &stroke.pattern {
                            stroke::Pattern::Solid(color) => {
                                path.curve.draw_stroke(d, 80, &stroke.thick, *color);
                            }

                            _ => todo!(),
                        }
                    }

                    StyleItem::Fill(fill) => {
                        match &fill.pattern {
                            fill::Pattern::Solid(color) => {
                                path.curve.draw_fill(d, 40, *color);
                            }

                            _ => todo!(),
                        }
                    }
                }
            }
        }

        if self.appearance.blend.is_non_default() {
            let [scratch_rtex, ..] = scratch_rtex else { panic!("insufficient scratch textures") };
            // Safety: the texture mode ends before the method returns, at the same depth it was opened.
            unsafe { ffi::BeginTextureMode(**scratch_rtex); } d.clear_background(Color::BLANK);
            {
                let mut d = d.begin_mode2D(camera);
                draw_inner(self, &mut d);
            }
            unsafe { ffi::EndTextureMode(); }
            let mut d = d.begin_blend_mode(self.appearance.blend.mode);
            {
                let mut d = d.rl_set_texture(scratch_rtex.texture.id());
                {
                    let mut d = d.rl_begin_quads();

                    d.rl_normal3f(0.0, 0.0, 1.0);
                    d.rl_color4f(1.0, 1.0, 1.0, self.appearance.blend.opacity);

                    d.rl_tex_coord2f(0.0,  0.0); d.rl_vertex2i(0, 0);
                    d.rl_tex_coord2f(0.0, -1.0); d.rl_vertex2i(0, scratch_rtex.height());
                    d.rl_tex_coord2f(1.0, -1.0); d.rl_vertex2i(scratch_rtex.width(), scratch_rtex.height());
                    d.rl_tex_coord2f(1.0,  0.0); d.rl_vertex2i(scratch_rtex.width(), 0);
                }
            }
        } else {
            draw_inner(self, d);
        }
    }

    fn draw_selected(&self, d: &mut impl RaylibDraw, _px_world_size: f32) {
        let color = self.settings.color;
        self.curve.draw_lines(d, 32, color);
        for (width_profile, width_curve) in self.appearance.items.iter().filter_map(|item|
            if let StyleItem::Stroke(stroke::Stroke { thick: profile @ WidthProfile::Variable(profile_curve), .. }) = item {
                Some((profile, profile_curve))
            } else { None }
        ) {
            let num_width_ctrls = width_curve.points.len();
            for i in 0..num_width_ctrls {
                let t = i as f32 / (num_width_ctrls - 1) as f32;
                let v = self.curve.velocity_at(t).expect("t should be 0-1");
                if v.magnitude_sqr() < f32::EPSILON { continue; }
                let tangent = v.normalized();
                let (normal_cw, normal_cc) = (tangent.rotate_cw(), tangent.rotate_cc());
                let p = self.curve.position_at(t).expect("t should be 0-1");
                let extents = width_profile.extents_at(t);
                let p1 = p + normal_cc * extents.0;
                let p2 = p + normal_cw * extents.1;
                d.draw_line_v(p1, p2, color);
            }
        }
    }
}

pub const ANCHOR_EXTENT_INNER: f32 = 2.0;
pub const ANCHOR_OUTLINE_THICK: f32 = 1.0;
pub const ANCHOR_EXTENT_OUTER: f32 = ANCHOR_EXTENT_INNER + ANCHOR_OUTLINE_THICK;
pub const ANCHOR_SIZE_INNER: f32 = ANCHOR_EXTENT_INNER * 2.0;
pub const ANCHOR_SIZE_OUTER: f32 = ANCHOR_EXTENT_OUTER * 2.0;

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
