use raylib::prelude::*;

use super::mat2::Matrix2x2;

#[derive(Debug)]
pub enum CtrlPoint {
    /// Described by a position in the world
    Exact(Vector2),

    /// Control mirrors opposite control
    ///
    /// If opposite control is not `Exact`, treat this control like `Corner`
    Smooth,

    /// Control is identical to anchor
    Corner,
}

pub use CtrlPoint::*;

pub trait ReflectVector {
    fn reflected_over(&self, across: Self) -> Self;
}
impl ReflectVector for Vector2 {
    fn reflected_over(&self, across: Self) -> Self {
        across * 2.0 - *self
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

impl CtrlPoint {
    pub fn calculate(&self, p: &Vector2, c_opp: &CtrlPoint) -> Vector2 {
        match (self, c_opp) {
            (&Exact(c), _) => c,
            (Corner, _) | (Smooth, Smooth | Corner) => *p,
            (Smooth, &Exact(c_opp)) => c_opp.reflected_over(*p),
        }
    }
}

pub struct PathPoint {
    pub c_in: CtrlPoint,
    pub p: Vector2,
    pub c_out: CtrlPoint,
}

impl PathPoint {
    pub fn new(c_in: CtrlPoint, p: Vector2, c_out: CtrlPoint) -> Self {
        Self { c_in, p, c_out }
    }

    /// Convert control points into world positions
    pub fn calculated(&self) -> (Vector2, Vector2, Vector2) {
        let p = self.p;
        let (c_in, c_out) = match (&self.c_in, &self.c_out) {
            (&Exact(c_in), &Exact(c_out)) => (c_in, c_out),
            (&Exact(c_in), Smooth) => (c_in, c_in.reflected_over(p)),
            (&Exact(c_in), Corner) => (c_in, p),
            (Smooth, &Exact(c_out)) => (c_out.reflected_over(p), c_out),
            (Corner, &Exact(c_out)) => (p, c_out),
            (Corner | Smooth, Corner | Smooth) => (p, p),
        };
        (c_in, p, c_out)
    }

    /// Replace `c_in` and `c_out` with `Corner` if they are closer to `p` than `snap_radius_sqr.sqrt()` pixels
    pub fn clean_corners(&mut self, snap_radius_sqr: f32) {
        if let Exact(c_in) = self.c_in {
            if c_in.distance_sqr_to(self.p) < snap_radius_sqr {
                self.c_in = Corner;
                if matches!(self.c_out, Smooth) {
                    self.c_out = Corner;
                    return; // both have been set
                }
            }
        }

        if let Exact(c_out) = self.c_out {
            if c_out.distance_sqr_to(self.p) < snap_radius_sqr {
                self.c_out = Corner;
                if matches!(self.c_out, Smooth) {
                    self.c_in = Corner;
                }
            }
        }
    }

    /// Translate the point and controls while keeping the controls' relative positions
    pub fn move_point(&mut self, delta: Vector2) {
        if let Exact(c_in) = &mut self.c_in {
            *c_in += delta;
        }

        self.p += delta;

        if let Exact(c_out) = &mut self.c_out {
            *c_out += delta;
        }
    }

    /// Move the point and controls while keeping the controls' relative positions
    pub fn set_point(&mut self, p: Vector2) {
        self.move_point(p - self.p);
    }

    pub fn transform(&mut self, mat: Matrix2x2) {
        if let Exact(c_in) = &mut self.c_in {
            *c_in = mat * *c_in;
        }

        self.p = mat * self.p;

        if let Exact(c_out) = &mut self.c_out {
            *c_out = mat * *c_out;
        }
    }
}
