use raylib::prelude::*;
use amymath::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Ctrl {
    In,
    Out,
}

impl Ctrl {
    pub const fn opposite(self) -> Self {
        match self {
            Ctrl::In  => Ctrl::Out,
            Ctrl::Out => Ctrl::In,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum PPPart {
    Ctrl(Ctrl),
    Anchor,
}

#[derive(Debug, Clone, Copy)]
pub enum Ctrl2 {
    // corner: G0 continuity

    // todo: add G2 continuity option

    /// Reflection of `c1` across `p` \
    /// G1 continuity
    Reflect,

    /// Reflection of `c1` across `p`, with a specific length \
    /// G1 continuity
    Mirror(f32),

    /// A transformation of the relative vector from `p` to `c1` \
    /// G0 continuity
    Transformed(Matrix2x2),

    /// A directly given position \
    /// G0 continuity
    Exact(Vector2),
}
use Ctrl2::*;

use crate::shaders::ShaderTable;

impl Ctrl2 {
    pub const fn is_reflect(&self) -> bool {
        matches!(self, Reflect)
    }

    pub const fn is_mirror(&self) -> bool {
        matches!(self, Mirror(_))
    }

    pub const fn is_exact(&self) -> bool {
        matches!(self, Exact(_))
    }

    pub fn calculate(&self, p: Vector2, c1: Vector2) -> Vector2 {
        match self {
            &Exact(c2) => c2,
            &Reflect => c1.reflected_over(p),
            &Mirror(s2) => c1.reflected_to(p, s2),
            Transformed(m2) => p + m2.mul(c1 - p),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Ctrl1 {
    pub c1: (Ctrl, Vector2),
    pub c2: Option<Ctrl2>,
}

#[derive(Debug, Clone, Copy)]
pub struct PathPoint {
    pub p: Vector2,
    pub c: Option<Ctrl1>,
}

impl PathPoint {
    #[inline]
    pub const fn is_c1_corner(&self) -> bool {
        self.c.is_none()
    }

    #[inline]
    pub const fn is_c2_corner(&self) -> bool {
        self.c2().is_none()
    }

    pub const fn c2(&self) -> Option<&Ctrl2> {
        match &self.c {
            Some(Ctrl1 { c2, .. }) => c2.as_ref(),
            _ => None,
        }
    }

    pub const fn c2_mut(&mut self) -> Option<&mut Ctrl2> {
        match &mut self.c {
            Some(Ctrl1 { c2, .. }) => c2.as_mut(),
            _ => None,
        }
    }

    pub fn calculate(&self) -> (Vector2, Vector2, Vector2) {
        let (c_in, c_out) = match &self.c {
            Some(Ctrl1 { c1: (c1_side, c1), c2 }) => {
                // debug_assert_ne!(&self.p, c1);
                let c_opp = c2.map_or(self.p, |c2| c2.calculate(self.p, *c1));
                match c1_side {
                    Ctrl::In  => (*c1, c_opp),
                    Ctrl::Out => (c_opp, *c1),
                }
            }
            None => (self.p, self.p),
        };

        (c_in, self.p, c_out)
    }

    pub fn ctrl(&self, side: Ctrl) -> Option<Ctrl2> {
        self.c.as_ref().and_then(|Ctrl1 { c1: (c1_side, c1), c2 }|
            if c1_side == &side { Some(Exact(*c1)) } else { *c2 }
        )
    }

    pub fn ctrl_pov(&self, side: Ctrl) -> (Option<Ctrl2>, Option<Ctrl2>) {
        if let Some(Ctrl1 { c1: (c1_side, c1), c2 }) = self.c.as_ref() {
            let (c1, c2) = (Some(Exact(*c1)), *c2);
            if c1_side == &side { (c1, c2) } else { (c2, c1) }
        } else {
            (None, None)
        }
    }

    /// Translate the point and controls while keeping the controls' relative positions
    pub fn move_point(&mut self, delta: Vector2) {
        self.p += delta;
        if let Some(Ctrl1 { c1: (_, c1), c2 }) = self.c.as_mut() {
            *c1 += delta;
            if let Some(Exact(c2)) = c2.as_mut() {
                *c2 += delta;
            }
        }
    }

    /// Move the point and controls while keeping the controls' relative positions
    pub fn set_point(&mut self, p: Vector2) {
        self.move_point(p - self.p);
    }

    pub fn _transform(&mut self, _mat: Matrix2x2) {
        todo!("im not updating this anymore until it gets used")
    }

    /// Draw normally
    pub fn draw(
        &self,
        d: &mut impl RaylibDraw,
        px_world_size: f32,
        color: Color,
        is_selected: bool,
        is_c_in_vis: bool,
        is_c_out_vis: bool,
        shader_table: &ShaderTable,
    ) {
        if let Some(Ctrl1 { c1: (c1_side, c1), c2 }) = self.c.as_ref() {
            let (is_c1_vis, is_c2_vis) = match c1_side {
                Ctrl::In => (is_c_in_vis, is_c_out_vis),
                Ctrl::Out => (is_c_out_vis, is_c_in_vis),
            };

            if is_c1_vis {
                d.draw_line_v(self.p, *c1, color);
                shader_table.draw_uv_tex(&mut d.begin_shader_mode(&shader_table.circle), *c1, 3.5 * px_world_size, color);
            }

            if is_c2_vis {
                if let Some(c2_ty) = c2.as_ref() {
                    let c2 = c2_ty.calculate(self.p, *c1);
                    match c2_ty {
                        Ctrl2::Exact(_) => {
                            d.draw_line_v(self.p, c2, color);
                            shader_table.draw_uv_tex(&mut d.begin_shader_mode(&shader_table.circle), c2, 3.5 * px_world_size, color);
                        }

                        Ctrl2::Mirror(_) => {
                            d.draw_line_v(self.p, c2, color.alpha(0.5));
                            let radius = 3.0 * px_world_size;
                            let size = 2.0 * radius;
                            d.draw_rectangle_pro(Rectangle::new(c2.x, c2.y, size, size), Vector2::new(radius, radius), 45.0, color);
                        }

                        Ctrl2::Transformed(_) => {
                            d.draw_line_v(self.p, c2, color.alpha(0.5));
                            let radius = 3.0 * px_world_size;
                            const FRAC_SQRT_3_2: f32 = 0.86602540378;
                            d.draw_triangle(
                                Vector2::new( FRAC_SQRT_3_2,  0.5) * radius,
                                Vector2::new(-FRAC_SQRT_3_2,  0.5) * radius,
                                Vector2::new(           0.0, -1.0) * radius,
                                color
                            );
                        }

                        Ctrl2::Reflect => {
                            d.draw_line_v(self.p, c2, color.alpha(0.5));
                            d.draw_ring(c2, 2.5 * px_world_size, 3.5 * px_world_size, 0.0, 360.0, 10, color);
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
                self.p.x - ANCHOR_EXTENT_OUTER * px_world_size,
                self.p.y - ANCHOR_EXTENT_OUTER * px_world_size,
                ANCHOR_SIZE_OUTER * px_world_size,
                ANCHOR_SIZE_OUTER * px_world_size,
            );
            d.draw_rectangle_rec(rec_outer, color);
        }

        // fill
        if !is_selected {
            let rec_inner = Rectangle::new(
                self.p.x - ANCHOR_EXTENT_INNER * px_world_size,
                self.p.y - ANCHOR_EXTENT_INNER * px_world_size,
                ANCHOR_SIZE_INNER * px_world_size,
                ANCHOR_SIZE_INNER * px_world_size,
            );
            d.draw_rectangle_rec(rec_inner, Color::WHITE);
        }
    }
}
