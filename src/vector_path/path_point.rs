use raylib::prelude::*;
use super::mat2::Matrix2x2;

pub trait ReflectVector {
    fn reflected_over(&self, across: Self) -> Self;
    fn reflected_to(&self, across: Self, length: f32) -> Self;
}
impl ReflectVector for Vector2 {
    fn reflected_over(&self, across: Self) -> Self {
        Self {
            x: across.x * 2.0 - self.x,
            y: across.y * 2.0 - self.y,
        }
    }

    fn reflected_to(&self, across: Self, mut length: f32) -> Self {
        let delta = *self - across;
        length /= delta.length();
        Self {
            x: across.x - length * delta.x,
            y: across.y - length * delta.y,
        }
    }
}
pub trait DistanceSqr {
    fn distance_sqr_to(&self, v: Self) -> f32;
}
impl DistanceSqr for Vector2 {
    fn distance_sqr_to(&self, v: Self) -> f32 {
        (*self - v).length_sqr()
    }
}

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
pub enum CtrlPt2 {
    Smooth,
    Mirror(f32),
    Exact(Vector2),
}

pub use CtrlPt2::*;

#[derive(Debug, Clone)]
pub struct CtrlPt1 {
    pub c1: (Ctrl, Vector2),
    pub c2: Option<CtrlPt2>,
}

#[derive(Debug, Clone)]
pub struct PathPoint {
    pub p: Vector2,
    pub ctrls: Option<CtrlPt1>,
}

impl PathPoint {
    pub fn calculate(&self) -> (Vector2, Vector2, Vector2) {
        let (c_in, c_out) = match &self.ctrls {
            Some(CtrlPt1 { c1: (c1_side, c1), c2 }) => {
                let c_opp = match &c2 {
                    None => self.p,
                    Some(Smooth) => c1.reflected_over(self.p),
                    Some(Mirror(s2)) => c1.reflected_to(self.p, *s2),
                    Some(Exact(c2)) => *c2,
                };
                match c1_side {
                    Ctrl::In  => (*c1, c_opp),
                    Ctrl::Out => (c_opp, *c1),
                }
            }
            None => (self.p, self.p),
        };

        (c_in, self.p, c_out)
    }

    pub fn ctrl(&self, side: Ctrl) -> Option<CtrlPt2> {
        self.ctrls.as_ref().and_then(|CtrlPt1 { c1: (c1_side, c1), c2 }|
            if c1_side == &side { Some(Exact(*c1)) } else { *c2 }
        )
    }

    pub fn ctrls_pov(&self, side: Ctrl) -> (Option<CtrlPt2>, Option<CtrlPt2>) {
        if let Some(CtrlPt1 { c1: (c1_side, c1), c2 }) = self.ctrls.as_ref() {
            let (c1, c2) = (Some(Exact(*c1)), *c2);
            if c1_side == &side { (c1, c2) } else { (c2, c1) }
        } else {
            (None, None)
        }
    }

    /// Translate the point and controls while keeping the controls' relative positions
    pub fn move_point(&mut self, delta: Vector2) {
        self.p += delta;
        if let Some(CtrlPt1 { c1: (_, c1), c2 }) = self.ctrls.as_mut() {
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

    pub fn transform(&mut self, _mat: Matrix2x2) {
        todo!("im not updating this anymore until it gets used")
    }

    pub fn draw_selected<D: RaylibDraw>(&self, d: &mut D, zoom_inv: f32, color: Color) {
        let draw_ctrl_exact = |d: &mut D, handle: Vector2| {
            d.draw_line_v(self.p, handle, color);
            d.draw_circle_v(handle, 3.5 * zoom_inv, color);
        };
        if let Some(CtrlPt1 { c1: (_, c1), c2 }) = self.ctrls.as_ref() {
            draw_ctrl_exact(d, *c1);

            if let Some(c2) = c2.as_ref() {
                match c2 {
                    CtrlPt2::Smooth => {
                        let c2 = c1.reflected_over(self.p);
                        d.draw_line_v(self.p, c2, color.alpha(0.5));
                        d.draw_ring(c2, 2.5 * zoom_inv, 3.5 * zoom_inv, 0.0, 360.0, 10, color);
                    }

                    CtrlPt2::Mirror(s2) => {
                        let c2 = c1.reflected_to(self.p, *s2);
                        d.draw_line_v(self.p, c2, color.alpha(0.5));
                        let radius = 3.0 * zoom_inv;
                        let size = 2.0 * radius;
                        d.draw_rectangle_pro(Rectangle::new(c2.x, c2.y, size, size), Vector2::new(radius, radius), 45.0, color);
                    }

                    CtrlPt2::Exact(c2) => {
                        draw_ctrl_exact(d, *c2);
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
                self.p.x - ANCHOR_EXTENT_OUTER * zoom_inv,
                self.p.y - ANCHOR_EXTENT_OUTER * zoom_inv,
                ANCHOR_SIZE_OUTER * zoom_inv,
                ANCHOR_SIZE_OUTER * zoom_inv,
            );
            d.draw_rectangle_rec(rec_outer, color);
        }
        // fill
        {
            let rec_inner = Rectangle::new(
                self.p.x - ANCHOR_EXTENT_INNER * zoom_inv,
                self.p.y - ANCHOR_EXTENT_INNER * zoom_inv,
                ANCHOR_SIZE_INNER * zoom_inv,
                ANCHOR_SIZE_INNER * zoom_inv,
            );
            d.draw_rectangle_rec(rec_inner, Color::WHITE);
        }
    }
}
