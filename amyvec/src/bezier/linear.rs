use amymath::prelude::Rect2;
use raylib::prelude::*;
use super::Bezier;

#[derive(Debug)]
pub struct Linear {
    pub p1: Vector2,
    pub p2: Vector2,
}

impl Linear {
    pub fn new(p1: Vector2, p2: Vector2) -> Self {
        Self { p1, p2 }
    }

    #[inline]
    pub fn bounds(&self) -> Rect2 {
        Rect2::minmax_rec(self.p1, self.p2)
    }

    #[inline]
    pub fn position_at(&self, t: f32) -> Vector2 {
        self.p1.lerp(self.p2, t)
    }

    #[inline]
    pub fn velocity(&self) -> Vector2 {
        self.p2 - self.p1
    }

    /// Super easy, barely an inconvenience.
    #[inline]
    fn time_at(&self, pt: Vector2) -> f32 {
        let delta = self.p2 - self.p1;
        delta.dot(pt - self.p1) / delta.length_sqr()
    }
}

impl Bezier<Vector2> for Linear {
    #[inline]
    fn bounds(&self) -> Rect2 {
        self.bounds()
    }

    #[inline]
    fn position_at(&self, t: f32) -> Vector2 {
        self.position_at(t)
    }

    #[inline]
    fn velocity_at(&self, _t: f32) -> Vector2 {
        self.velocity()
    }

    #[inline]
    fn curvature_at(&self, _t: f32) -> f32 {
        0.0
    }

    #[inline]
    fn estimate_time_at(&self, pt: Vector2) -> Option<f32> {
        Some(self.time_at(pt))
    }
}
