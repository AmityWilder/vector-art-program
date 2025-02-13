use amymath::prelude::{CrossProduct, DistanceSqr};
use raylib::prelude::*;
use crate::{generics::*, polynomial::*};

use super::{if_bounded, linear::Linear, quadratic::Quadratic, Bezier};

#[derive(Debug)]
pub struct Cubic<V: Vector> {
    pub p1:     V,
    pub c1_out: V,
    pub c2_in:  V,
    pub p2:     V,
}

impl<V: Vector> Cubic<V> {
    pub const fn new(p1: V, c1_out: V, c2_in: V, p2: V) -> Self {
        Self { p1, c1_out, c2_in, p2 }
    }

    /// Calculate the position at time along the curve
    #[inline]
    pub fn position_at(&self, t: f32) -> V {
        let coefs = (
            (1.0 - t)*(1.0 - t)*(1.0 - t),
            3.0*(1.0 - t)*(1.0 - t)*t,
            3.0*(1.0 - t)*t*t,
            t*t*t,
        );
        let (p0, p1, p2, p3) = (self.p1, self.c1_out, self.c2_in, self.p2);
        p0*coefs.0 + p1*coefs.1 + p2*coefs.2 + p3*coefs.3
    }

    /// First-degree derivative at time along curve
    ///
    /// Normalize to get tangent direction
    #[inline]
    pub fn velocity_at(&self, t: f32) -> V {
        let coefs = (
            3.0*(1.0 - t)*(1.0 - t),
            6.0*(1.0 - t)*t,
            3.0*t*t,
        );
        let (p0, p1, p2, p3) = (self.p1, self.c1_out, self.c2_in, self.p2);
        (p1 - p0)*coefs.0 + (p2 - p1)*coefs.1 + (p3 - p2)*coefs.2
    }

    /// Second-degree derivative at time along curve
    #[inline]
    pub fn acceleration_at(&self, t: f32) -> V {
        let coefs = (
            6.0*(1.0 - t),
            6.0*t,
        );
        let (p0, p1, p2, p3) = (self.p1, self.c1_out, self.c2_in, self.p2);
        (p2 - p1*2.0 + p0)*coefs.0 + (p3 - p2*2.0 + p1)*coefs.1
    }

    /// Third-degree derivative at time along curve
    #[inline]
    pub fn jerk(&self) -> V {
        let (p0, p1, p2, p3) = (self.p1, self.c1_out, self.c2_in, self.p2);
        (p3 + (p1 - p2)*3.0 - p0)*6.0
    }

    /// Tests if the curve can be expressed as a perfectly straight line from p0 to p3
    pub fn as_linear(&self) -> Option<Linear<V>> {
        let (p0, p1, p2, p3) = (self.p1, self.c1_out, self.c2_in, self.p2);
        (p1 == p0 && p2 == p3).then(|| Linear::new(p0, p3))
    }

    /// Tests if the curve can be expressed as a cubic bezier
    pub fn is_quadratic(&self) -> Option<Quadratic<V>> {
        let (p0, p1, p2, p3) = (self.p1, self.c1_out, self.c2_in, self.p2);
        (p1 == p0).then(|| Quadratic::new(p0, p2, p3)).or_else(|| (p2 == p3).then(|| Quadratic::new(p0, p1, p3)))
    }
}

impl<V: Vector> Cubic<V> {
    /// Returns a broader bounding box of the curve using only the anchors and velocities.
    ///
    /// Cheaper than [`Self::bounds`], but significantly less accurate.
    ///
    /// [`Self::bounds`] will always produce a bounding box smaller or equal to this.
    ///
    /// **\* Describes the bounding box of *all* elements of the curve, not just the curve itself**
    #[inline]
    pub fn max_bounds(&self) -> V::Rect {
        let (p0, p1, p2, p3) = (self.p1, self.c1_out, self.c2_in, self.p2);
        V::Rect::minmax_rec(p0, p1).max(V::Rect::minmax_rec(p2, p3))
    }

    /// Returns a narrower bounding box of the curve using only the anchors.
    ///
    /// Cheaper than [`Self::bounds`] (significantly) and [`Self::max_bounds`] (barely), but
    /// inaccurate to the point of near meaninglessness.
    ///
    /// [`Self::bounds`] will always produce a bounding box larger or equal to this.
    ///
    /// **\* Only describes the bounding box of the start and end of the curve, not the curve itself**
    #[inline]
    pub fn min_bounds(&self) -> V::Rect {
        V::Rect::minmax_rec(self.p1, self.p2)
    }

    /// Returns the bounding box of the curve.
    ///
    /// ## Performance
    /// Solves the quadratic equation on both `x` and `y` axes
    pub fn bounds(&self) -> V::Rect {
        let (p0, p1, p2, p3) = (self.p1, self.c1_out, self.c2_in, self.p2);
        let a = p0*-3.0 + p1* 9.0 - p2*9.0 + p3*3.0;
        let b = p0* 6.0 - p1*12.0 + p2*6.0;
        let c = p0*-3.0 + p1* 3.0;
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

    /// Reciprocal radius (or "radians per meter") at time along curve
    #[inline]
    pub fn curvature_at(&self, t: f32) -> f32 where V: CrossProduct<Output = f32> {
        let vel = self.velocity_at(t);
        let acc = self.acceleration_at(t);
        vel.cross(acc) / vel.length().powi(3)
    }
}

impl Cubic<Vector2> {
    /// Time along curve closest to the position
    ///
    /// ## Performance
    /// Solves the cubic equation, including complex roots.
    ///
    /// ## Note regarding current implementation
    /// - Loses accuracy the further the point is from the curve.
    /// - Excessively sensitive to this at start and end of the curve.
    /// - **Does not consistently produce a valid output.**
    pub fn estimate_time_at(&self, pt: Vector2) -> Option<(f32, Vector2)> {
        let (p0, p1, p2, p3) = (self.p1, self.c1_out, self.c2_in, self.p2);
        let (p0, p1, p2, p3, p) = (p0.x - p0.y, p1.x - p1.y, p2.x - p2.y, p3.x - p3.y, pt.x - pt.y);
        let a = p3 + 3.0*(p1 - p2) - p0;
        let b = 3.0*(p2 - 2.0*p1 + p0);
        let c = 3.0*(p1 - p0);
        let d = p0 - p;
        let zeros = match cubic_zero(a, b, c, d) {
            CubicZeros::NoSolution | CubicZeros::InfiniteSolutions => [None, None, None],
            CubicZeros::OneSolution(x0) => [Some(x0), None, None],
            CubicZeros::TwoSolutions(x0, x1) => [Some(x0), Some(x1), None],
            CubicZeros::ThreeSolutions(x0, x1, x2) => [Some(x0), Some(x1), Some(x2)],
        };
        zeros.into_iter()
            .filter_map(|x| x.and_then(|x| if_bounded(x)))
            .map(|t| {
                let p = self.position_at(t);
                (t, p, p.distance_sqr_to(pt))
            })
            .reduce(|best, curr| if curr.2 < best.2 { curr } else { best })
            .map(|(t, p, _dist_sqr)| (t, p))
    }
}

impl<V: Vector> Bezier<V> for Cubic<V> {
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
    fn acceleration_at(&self, t: f32) -> V {
        self.acceleration_at(t)
    }

    #[inline]
    fn jerk_at(&self, _t: f32) -> V {
        self.jerk()
    }
}
