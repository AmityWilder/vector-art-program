use amymath::prelude::CrossProduct;
use crate::{generics::*, polynomial::{quadratic_zero, QuadraticZeros}};

use super::{cubic::Cubic, if_bounded, Bezier};

#[derive(Debug)]
pub struct Quadratic<V: Vector> {
    pub p1: V,
    pub c:  V,
    pub p2: V,
}

impl<V: Vector> From<Quadratic<V>> for Cubic<V> {
    fn from(Quadratic { p1: p0, c: p1, p2 }: Quadratic<V>) -> Self {
        const FRAC_1_3: f32 = 1.0/3.0;
        Cubic {
            p1: p0,
            c1_out: (p0 + p1*2.0)*FRAC_1_3,
            c2_in:  (p1*2.0 + p2)*FRAC_1_3,
            p2: p0,
        }
    }
}

impl<V: Vector> Quadratic<V> {
    pub fn new(p1: V, c: V, p2: V) -> Self {
        Self { p1, c, p2 }
    }

    #[inline]
    pub fn min_bounds(&self) -> V::Rect {
        V::Rect::minmax_rec(self.p1, self.p2)
    }

    #[inline]
    pub fn max_bounds(&self) -> V::Rect {
        let (p0, p1, p2) = (self.p1, self.c, self.p2);
        V::Rect::minmax_rec(p0, p1).max_pt(p2)
    }

    pub fn bounds(&self) -> V::Rect {
        let (p0, p1, p2) = (self.p1, self.c, self.p2);

        let a = p0 - p1*2.0 + p2;
        let b = (p1 - p0)*2.0;
        let c = p0;
        a.into_iter().zip(b.into_iter()).zip(c.into_iter())
            .flat_map(|((a, b), c)| {
                match quadratic_zero(a, b, c) {
                    QuadraticZeros::NoSolution | QuadraticZeros::InfiniteSolutions => [None, None],
                    QuadraticZeros::OneSolution(x0) => [if_bounded(x0), None],
                    QuadraticZeros::TwoSolutions(x0, x1) => [if_bounded(x0), if_bounded(x1)],
                }.into_iter().flatten()
            })
            .map(|t| self.position_at(t))
            .fold(self.min_bounds(), |rec, p| rec.max_pt(p))
    }

    #[inline]
    pub fn position_at(&self, t: f32) -> V {
        let coefs = (
            (1.0 - t)*(1.0 - t),
            2.0*(1.0 - t)*t,
            t*t,
        );
        let (p0, p1, p2) = (self.p1, self.c, self.p2);
        p0*coefs.0 + p1*coefs.1 + p2*coefs.2
    }

    #[inline]
    fn velocity_at(&self, t: f32) -> V {
        let coefs = (
            2.0*(1.0 - t),
            2.0*t,
        );
        let (p0, p1, p2) = (self.p1, self.c, self.p2);
        (p1 - p0)*coefs.0 + (p2 - p1)*coefs.1
    }

    #[inline]
    fn acceleration(&self) -> V {
        let (p0, p1, p2) = (self.p1, self.c, self.p2);
        (p2 - p1*2.0 + p0)*2.0
    }
}

impl<V: Vector + CrossProduct<Output = f32>> Bezier<V> for Quadratic<V> {
    #[inline]
    fn max_bounds(&self) -> V::Rect {
        self.max_bounds()
    }

    #[inline]
    fn min_bounds(&self) -> V::Rect {
        self.min_bounds()
    }

    #[inline]
    fn bounds(&self) -> V::Rect {
        self.bounds()
    }

    #[inline]
    fn position_at(&self, t: f32) -> V {
        self.position_at(t)
    }

    #[inline]
    fn velocity_at(&self, t: f32) -> V {
        self.velocity_at(t)
    }

    #[inline]
    fn acceleration_at(&self, _t: f32) -> V {
        self.acceleration()
    }
}
