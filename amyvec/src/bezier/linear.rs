use crate::generics::*;

use super::Bezier;

#[derive(Debug)]
pub struct Linear<V: Vector> {
    pub p1: V,
    pub p2: V,
}

impl<V: Vector> Linear<V> {
    pub fn new(p1: V, p2: V) -> Self {
        Self { p1, p2 }
    }

    #[inline]
    pub fn bounds(&self) -> V::Rect {
        V::Rect::minmax_rec(self.p1, self.p2)
    }

    #[inline]
    pub fn position_at(&self, t: f32) -> V {
        self.p1 * (1.0 - t) + self.p2 * t
    }

    #[inline]
    pub fn velocity(&self) -> V {
        self.p2 - self.p1
    }

    /// Super easy, barely an inconvenience.
    #[inline]
    pub fn time_at(&self, pt: V) -> f32 {
        let delta = self.p2 - self.p1;
        delta.dot(pt - self.p1) / delta.length_sqr()
    }
}

impl<V: Vector> Bezier<V> for Linear<V> {
    #[inline]
    fn bounds(&self) -> V::Rect {
        self.bounds()
    }

    #[inline]
    fn position_at(&self, t: f32) -> V {
        self.position_at(t)
    }

    #[inline]
    fn velocity_at(&self, _t: f32) -> V {
        self.velocity()
    }
}
