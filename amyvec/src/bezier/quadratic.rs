use amymath::prelude::*;
use crate::polynomial::{quadratic_zero, QuadraticZeros};
use super::{cubic::Cubic, if_bounded, Bezier};

#[derive(Debug)]
pub struct Quadratic {
    pub p1: Vector2,
    pub c:  Vector2,
    pub p2: Vector2,
}

impl From<Quadratic> for Cubic {
    fn from(Quadratic { p1: p0, c: p1, p2 }: Quadratic) -> Self {
        const FRAC_1_3: f32 = 1.0/3.0;
        Cubic {
            p1: p0,
            c1_out: (p0 + p1*2.0)*FRAC_1_3,
            c2_in:  (p1*2.0 + p2)*FRAC_1_3,
            p2: p0,
        }
    }
}

impl Quadratic {
    pub fn new(p1: Vector2, c: Vector2, p2: Vector2) -> Self {
        Self { p1, c, p2 }
    }

    #[inline]
    pub fn min_bounds(&self) -> Rect2 {
        Rect2::from_minmax(self.p1, self.p2)
    }

    #[inline]
    pub fn max_bounds(&self) -> Rect2 {
        let (p0, p1, p2) = (self.p1, self.c, self.p2);
        Rect2::from_minmax(p0, p1).union_v(p2)
    }

    pub fn bounds(&self) -> Rect2 {
        let (p0, p1, p2) = (self.p1, self.c, self.p2);

        let a = p0 - p1*2.0 + p2;
        let b = (p1 - p0)*2.0;
        let c = p0;
        [(a.x, b.x, c.x), (a.y, b.y, c.y)].into_iter()
            .flat_map(|(a, b, c)| {
                match quadratic_zero(a, b, c) {
                    QuadraticZeros::NoSolution | QuadraticZeros::InfiniteSolutions => [None, None],
                    QuadraticZeros::OneSolution(x0) => [if_bounded(x0), None],
                    QuadraticZeros::TwoSolutions(x0, x1) => [if_bounded(x0), if_bounded(x1)],
                }.into_iter().flatten()
            })
            .map(|t| self.position_at(t))
            .fold(self.min_bounds(), |rec, p| rec.union_v(p))
    }

    #[inline]
    pub fn position_at(&self, t: f32) -> Vector2 {
        let coefs = (
            (1.0 - t)*(1.0 - t),
            2.0*(1.0 - t)*t,
            t*t,
        );
        let (p0, p1, p2) = (self.p1, self.c, self.p2);
        p0*coefs.0 + p1*coefs.1 + p2*coefs.2
    }

    #[inline]
    fn velocity_at(&self, t: f32) -> Vector2 {
        let coefs = (
            2.0*(1.0 - t),
            2.0*t,
        );
        let (p0, p1, p2) = (self.p1, self.c, self.p2);
        (p1 - p0)*coefs.0 + (p2 - p1)*coefs.1
    }

    #[inline]
    fn acceleration(&self) -> Vector2 {
        let (p0, p1, p2) = (self.p1, self.c, self.p2);
        (p2 - p1*2.0 + p0)*2.0
    }
}

impl Bezier for Quadratic {
    #[inline]
    fn max_bounds(&self) -> Rect2 {
        self.max_bounds()
    }

    #[inline]
    fn min_bounds(&self) -> Rect2 {
        self.min_bounds()
    }

    #[inline]
    fn bounds(&self) -> Rect2 {
        self.bounds()
    }

    #[inline]
    fn position_at(&self, t: f32) -> Vector2 {
        self.position_at(t)
    }

    #[inline]
    fn velocity_at(&self, t: f32) -> Vector2 {
        self.velocity_at(t)
    }

    #[inline]
    fn acceleration_at(&self, _t: f32) -> Vector2 {
        self.acceleration()
    }
}
