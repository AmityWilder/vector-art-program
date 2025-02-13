use amymath::prelude::{CrossProduct, Rect2};
use raylib::prelude::*;
use crate::polynomial::{quadratic_zero, QuadraticZeros};

use super::{cubic::Cubic, if_bounded, Bezier};

#[derive(Debug)]
pub struct Quadratic {
    pub p1: Vector2,
    pub c:  Vector2,
    pub p2: Vector2,
}

impl From<Quadratic> for Cubic<Vector2> {
    fn from(Quadratic { p1: p0, c: p1, p2 }: Quadratic) -> Self {
        const FRAC_1_3: f32 = 1.0/3.0;
        Cubic {
            p1: p0,
            c1_out: Vector2 {
                x: FRAC_1_3*(p0.x + 2.0*p1.x),
                y: FRAC_1_3*(p0.y + 2.0*p1.y),
            },
            c2_in:  Vector2 {
                x: FRAC_1_3*(2.0*p1.x + p2.x),
                y: FRAC_1_3*(2.0*p1.y + p2.y),
            },
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
        Rect2::minmax_rec(self.p1, self.p2)
    }

    #[inline]
    pub fn max_bounds(&self) -> Rect2 {
        let (p0, p1, p2) = (self.p1, self.c, self.p2);
        Rect2::minmax_rec(p0, p1).max_pt(p2)
    }

    pub fn bounds(&self) -> Rect2 {
        let (p0, p1, p2) = (self.p1, self.c, self.p2);

        let (a_x, a_y) = (
            p0.x - 2.0*p1.x + p2.x,
            p0.y - 2.0*p1.y + p2.y,
        );
        let (b_x, b_y) = (
            2.0*(p1.x - p0.x),
            2.0*(p1.y - p0.y),
        );
        let (c_x, c_y) = (
            p0.x,
            p0.y,
        );
        [(a_x, b_x, c_x), (a_y, b_y, c_y)]
            .into_iter()
            .flat_map(|(a, b, c)| {
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
    pub fn position_at(&self, t: f32) -> Vector2 {
        let coefs = (
            (1.0 - t)*(1.0 - t),
            2.0*(1.0 - t)*t,
            t*t,
        );
        let (p0, p1, p2) = (self.p1, self.c, self.p2);
        Vector2 {
            x: coefs.0*p0.x + coefs.1*p1.x + coefs.2*p2.x,
            y: coefs.0*p0.y + coefs.1*p1.y + coefs.2*p2.y,
        }
    }

    #[inline]
    fn velocity_at(&self, t: f32) -> Vector2 {
        let coefs = (
            2.0*(1.0 - t),
            2.0*t,
        );
        let (p0, p1, p2) = (self.p1, self.c, self.p2);
        Vector2 {
            x: coefs.0*(p1.x - p0.x) + coefs.1*(p2.x - p1.x),
            y: coefs.0*(p1.y - p0.y) + coefs.1*(p2.y - p1.y),
        }
    }

    #[inline]
    fn acceleration(&self) -> Vector2 {
        let (p0, p1, p2) = (self.p1, self.c, self.p2);
        Vector2 {
            x: 2.0*(p2.x - 2.0*p1.x + p0.x),
            y: 2.0*(p2.y - 2.0*p1.y + p0.y),
        }
    }
}

impl Bezier<Vector2> for Quadratic {
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

    #[inline]
    fn curvature_at(&self, t: f32) -> f32 {
        let vel = self.velocity_at(t);
        let acc = self.acceleration();
        vel.cross(acc) / vel.length().powi(3)
    }

    #[inline]
    fn estimate_time_at(&self, _pt: Vector2) -> Option<f32> {
        todo!()
    }
}
