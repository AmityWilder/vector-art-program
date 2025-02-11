use amymath::prelude::{DistanceSqr, Rect2};
use raylib::prelude::*;
use crate::polynomial::*;

#[derive(Debug)]
pub struct CubicBezier {
    pub p1:     Vector2,
    pub c1_out: Vector2,
    pub c2_in:  Vector2,
    pub p2:     Vector2,
}

impl CubicBezier {
    pub(super) fn new(p1: Vector2, c1_out: Vector2, c2_in: Vector2, p2: Vector2) -> Self {
        Self { p1, c1_out, c2_in, p2 }
    }
}

fn if_bounded(x: f32) -> Option<f32> {
    (0.0 <= x && x <= 1.0).then_some(x)
}

impl CubicBezier {
    /// Returns a broader bounding box of the curve using only the anchors and velocities.
    ///
    /// Cheaper than [`Self::bounds`], but significantly less accurate.
    ///
    /// [`Self::bounds`] will always produce a bounding box smaller or equal to this.
    ///
    /// **\* Describes the bounding box of *all* elements of the curve, not just the curve itself**
    #[inline]
    pub fn max_bounds(&self) -> Rect2 {
        let (p0, p1, p2, p3) = (self.p1, self.c1_out, self.c2_in, self.p2);
        Rect2 {
            xmin: p0.x.min(p1.x).min(p2.x).min(p3.x),
            ymin: p0.y.min(p1.y).min(p2.y).min(p3.y),
            xmax: p0.x.max(p1.x).max(p2.x).max(p3.x),
            ymax: p0.y.max(p1.y).max(p2.y).max(p3.y),
        }
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
    pub fn min_bounds(&self) -> Rect2 {
        let (p0, _p1, _p2, p3) = (self.p1, self.c1_out, self.c2_in, self.p2);
        Rect2 {
            xmin: p0.x.min(p3.x),
            ymin: p0.y.min(p3.y),
            xmax: p0.x.max(p3.x),
            ymax: p0.y.max(p3.y),
        }
    }

    /// Returns the bounding box of the curve.
    ///
    /// ## Performance
    /// Solves the quadratic equation on both `x` and `y` axes
    #[inline]
    pub fn bounds(&self) -> Rect2 {
        let (p0, p1, p2, p3) = (self.p1, self.c1_out, self.c2_in, self.p2);
        let (a_x, a_y) = (
            -3.0*p0.x + 9.0*p1.x - 9.0*p2.x + 3.0*p3.x,
            -3.0*p0.y + 9.0*p1.y - 9.0*p2.y + 3.0*p3.y,
        );
        let (b_x, b_y) = (
            6.0*p0.x - 12.0*p1.x + 6.0*p2.x,
            6.0*p0.y - 12.0*p1.y + 6.0*p2.y,
        );
        let (c_x, c_y) = (
            -3.0*p0.x + 3.0*p1.x,
            -3.0*p0.y + 3.0*p1.y,
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

    /// Calculate the arclength up to the time along the curve
    ///
    /// Returns [`None`] if `t` is not 0-1 or `samples` is 0
    pub fn length_to(&self, t: f32, samples: usize) -> Option<f32> {
        if samples == 0 || t < 0.0 || 1.0 < t { return None; }
        // todo: i dont like this :c
        // im gonna see if i can make it O(1)

        // update 2025-2-9: something something "elliptic integral of the second form is unbounded"?
        // i still wanna see if it can be done

        let mut length = 0.0;

        let mut sample_prev = self.p1;
        let samples_inv = (samples as f32).recip();
        for i in 1..samples {
            let sample_t = i as f32 * samples_inv;
            let sample = self.position_at(sample_t);
            length += sample_prev.distance_to(sample);
            sample_prev = sample;
        }

        Some(length)
    }

    /// Calculate the position at time along the curve
    #[inline]
    pub fn position_at(&self, t: f32) -> Vector2 {
        let coefs = (
            (1.0 - t)*(1.0 - t)*(1.0 - t),
            3.0*(1.0 - t)*(1.0 - t)*t,
            3.0*(1.0 - t)*t*t,
            t*t*t,
        );
        let (p0, p1, p2, p3) = (self.p1, self.c1_out, self.c2_in, self.p2);
        Vector2 {
            x: coefs.0*p0.x + coefs.1*p1.x + coefs.2*p2.x + coefs.3*p3.x,
            y: coefs.0*p0.y + coefs.1*p1.y + coefs.2*p2.y + coefs.3*p3.y,
        }
    }

    /// First-degree derivative at time along curve
    ///
    /// Normalize to get tangent direction
    #[inline]
    pub fn velocity_at(&self, t: f32) -> Vector2 {
        let coefs = (
            3.0*(1.0 - t)*(1.0 - t),
            6.0*(1.0 - t)*t,
            3.0*t*t,
        );
        let (p0, p1, p2, p3) = (self.p1, self.c1_out, self.c2_in, self.p2);
        Vector2 {
            x: coefs.0*(p1.x - p0.x) + coefs.1*(p2.x - p1.x) + coefs.2*(p3.x - p2.x),
            y: coefs.0*(p1.y - p0.y) + coefs.1*(p2.y - p1.y) + coefs.2*(p3.y - p2.y),
        }
    }

    /// Second-degree derivative at time along curve
    #[inline]
    pub fn acceleration_at(&self, t: f32) -> Vector2 {
        let coefs = (
            6.0*(1.0 - t),
            6.0*t,
        );
        let (p0, p1, p2, p3) = (self.p1, self.c1_out, self.c2_in, self.p2);
        Vector2 {
            x: coefs.0*(p2.x - 2.0*p1.x + p0.x) + coefs.1*(p3.x - 2.0*p2.x + p1.x),
            y: coefs.0*(p2.y - 2.0*p1.y + p0.y) + coefs.1*(p3.y - 2.0*p2.y + p1.y),
        }
    }

    /// Third-degree derivative at time along curve
    #[inline]
    pub fn jerk(&self) -> Vector2 {
        let (p0, p1, p2, p3) = (self.p1, self.c1_out, self.c2_in, self.p2);
        Vector2 {
            x: 6.0*(p3.x + 3.0*(p1.x - p2.x) - p0.x),
            y: 6.0*(p3.y + 3.0*(p1.y - p2.y) - p0.y),
        }
    }

    /// Reciprocal radius (or "radians per meter") at time along curve
    #[inline]
    pub fn curvature_at(&self, t: f32) -> f32 {
        let vel = self.velocity_at(t);
        let acc = self.acceleration_at(t);
        (vel.x*acc.y - acc.x*vel.y) / (vel.x*vel.x + vel.y*vel.y).sqrt().powi(3)
    }

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
