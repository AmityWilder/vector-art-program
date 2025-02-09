use amymath::prelude::{DistanceSqr, Rect2};
use raylib::prelude::*;
use num::complex::Complex32;

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

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LinearZeros {
    NoSolution,
    InfiniteSolutions,
    OneSolution(f32),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum QuadraticZeros {
    NoSolution,
    InfiniteSolutions,
    OneSolution(f32),
    TwoSolutions(f32, f32),
}

impl From<LinearZeros> for QuadraticZeros {
    fn from(value: LinearZeros) -> Self {
        match value {
            LinearZeros::NoSolution => Self::NoSolution,
            LinearZeros::InfiniteSolutions => Self::InfiniteSolutions,
            LinearZeros::OneSolution(x) => Self::OneSolution(x),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CubicZeros {
    NoSolution,
    InfiniteSolutions,
    OneSolution(f32),
    TwoSolutions(f32, f32),
    ThreeSolutions(f32, f32, f32),
}

impl From<QuadraticZeros> for CubicZeros {
    fn from(value: QuadraticZeros) -> Self {
        match value {
            QuadraticZeros::NoSolution => Self::NoSolution,
            QuadraticZeros::InfiniteSolutions => Self::InfiniteSolutions,
            QuadraticZeros::OneSolution(x) => Self::OneSolution(x),
            QuadraticZeros::TwoSolutions(x0, x1) => Self::TwoSolutions(x0, x1),
        }
    }
}

pub fn linear_zero(m: f32, b: f32) -> LinearZeros {
    if m != 0.0 {
        LinearZeros::OneSolution(-b / m)
    } else if b == 0.0 {
        LinearZeros::InfiniteSolutions
    } else {
        LinearZeros::NoSolution
    }
}

pub fn quadratic_zero(a: f32, b: f32, c: f32) -> QuadraticZeros {
    if a != 0.0 {
        let b2_4ac = b*b - 4.0*a*c;
        if b2_4ac > 0.0 {
            let denom = (2.0 * a).recip();
            let parts = (-b * denom, b2_4ac.sqrt() * denom);
            QuadraticZeros::TwoSolutions(parts.0 + parts.1, parts.0 - parts.1)
        } else if b2_4ac == 0.0 {
            QuadraticZeros::OneSolution(-b / (2.0 * a))
        } else {
            QuadraticZeros::NoSolution
        }
    } else {
        linear_zero(b, c).into()
    }
}

pub fn cubic_zero(a: f32, b: f32, c: f32, d: f32) -> CubicZeros {
    #[allow(non_snake_case, non_upper_case_globals)]
    if a != 0.0 {
        const w1: Complex32 = Complex32::new(-0.5, std::f32::consts::SQRT_3 *  0.5);
        const w2: Complex32 = Complex32::new(-0.5, std::f32::consts::SQRT_3 * -0.5);
        let b_3a = -b/(3.0*a);
        let P = b_3a*b_3a*b_3a + (b*c)/(6.0*a*a) - d/(2.0*a);
        let Q = c/(3.0*a) - b_3a*b_3a;
        let sqrt = Complex32::new(P*P + Q*Q*Q, 0.0).sqrt();
        let radicals = ((P - sqrt).cbrt(), (P + sqrt).cbrt());
        let x1 =    radicals.0 +    radicals.1 + b_3a;
        let x2 = w1*radicals.0 + w2*radicals.1 + b_3a;
        let x3 = w2*radicals.0 + w1*radicals.1 + b_3a;
        match (
            (x1.im.abs() <= f32::EPSILON && x1.re.is_normal()).then_some(x1.re),
            (x2.im.abs() <= f32::EPSILON && x2.re.is_normal()).then_some(x2.re),
            (x3.im.abs() <= f32::EPSILON && x3.re.is_normal()).then_some(x3.re),
        ) {
            (None, None, None) => CubicZeros::NoSolution,
            (Some(x1), Some(x2), Some(x3)) => CubicZeros::ThreeSolutions(x1, x2, x3),

            | (Some(x1), None, None)
            | (None, Some(x1), None)
            | (None, None, Some(x1))
                => CubicZeros::OneSolution(x1),

            | (Some(x1), Some(x2), None)
            | (Some(x1), None, Some(x2))
            | (None, Some(x1), Some(x2))
                => CubicZeros::TwoSolutions(x1, x2),
        }
    } else {
        quadratic_zero(b, c, d).into()
    }
}

#[cfg(test)]
mod quadratic_tests {
    use super::*;

    #[test]
    fn test_two_solutions() {
        // x^2 + 5x + 6 = 0
        assert_eq!(quadratic_zero(1.0, 5.0, 6.0), QuadraticZeros::TwoSolutions(-2.0, -3.0));
    }

    #[test]
    fn test_one_solution() {
        // 1x^2 + 0x + 0 = 0
        assert_eq!(quadratic_zero(1.0, 0.0, 0.0), QuadraticZeros::OneSolution(0.0));
    }

    #[test]
    fn test_no_solutions() {
        // 0.5x^2 + 3x + 7 = 0
        assert_eq!(quadratic_zero(0.5, 3.0, 7.0), QuadraticZeros::NoSolution);
    }

    #[test]
    fn test_three_solutions() {
        let p0 = Vector2::new(-0.3, 0.2);
        let p1 = Vector2::new(0.117, 0.55);
        let p2 = Vector2::new(0.98, 0.62);
        let p3 = Vector2::new(1.46, 0.043);
        let pt = Vector2::new(0.492, 0.475);
        let (p0, p1, p2, p3, p) = (p0.x - p0.y, p1.x - p1.y, p2.x - p2.y, p3.x - p3.y, pt.x - pt.y);
        let a = p3 + 3.0*(p1 - p2) - p0;
        assert!((a - -0.462).abs() < 0.001);
        let b = 3.0*(p2 - 2.0*p1 + p0);
        assert!((b - 2.178).abs() < 0.001);
        let c = 3.0*(p1 - p0);
        assert!((c - 0.201).abs() < 0.001);
        let d = p0 - p;
        assert!((d - -0.517).abs() < 0.001);
        let zeros = cubic_zero(a, b, c, d);
        if let CubicZeros::ThreeSolutions(x1, x2, x3) = zeros {
            assert!((x1 - 4.7562).abs() < 0.0001);
            assert!((x2 - 0.4645).abs() < 0.0001);
            assert!((x3 + 0.5065).abs() < 0.0001);
        } else {
            assert!(false, "{:?}", zeros);
        }
    }
}

fn if_bounded(x: f32) -> Option<f32> {
    (0.0 <= x && x <= 1.0).then_some(x)
}

impl CubicBezier {
    pub fn bounds(&self) -> Rect2 {
        let (p0, p1, p2, p3) = (self.p1, self.c1_out, self.c2_in, self.p2);
        let a = Vector2 {
            x: -3.0*p0.x + 9.0*p1.x - 9.0*p2.x + 3.0*p3.x,
            y: -3.0*p0.y + 9.0*p1.y - 9.0*p2.y + 3.0*p3.y,
        };
        let b = Vector2 {
            x: 6.0*p0.x - 12.0*p1.x + 6.0*p2.x,
            y: 6.0*p0.y - 12.0*p1.y + 6.0*p2.y,
        };
        let c = Vector2 {
            x: -3.0*p0.x + 3.0*p1.x,
            y: -3.0*p0.y + 3.0*p1.y,
        };
        let mut rec = Rect2 {
            xmin: p0.x.min(p3.x),
            ymin: p0.y.min(p3.y),
            xmax: p0.x.max(p3.x),
            ymax: p0.y.max(p3.y),
        };
        for (a, b, c) in [(a.x, b.x, c.x), (a.y, b.y, c.y)] {
            let zeros = match quadratic_zero(a, b, c) {
                QuadraticZeros::NoSolution | QuadraticZeros::InfiniteSolutions => [None, None],
                QuadraticZeros::OneSolution(x0) => [Some(x0), None],
                QuadraticZeros::TwoSolutions(x0, x1) => [Some(x0), Some(x1)],
            };
            for t in zeros.into_iter().filter_map(|x| x.and_then(|x| if_bounded(x))) {
                let p = self.position_at(t);
                rec.xmin = rec.xmin.min(p.x);
                rec.ymin = rec.ymin.min(p.y);
                rec.xmax = rec.xmax.max(p.x);
                rec.ymax = rec.ymax.max(p.y);
            }
        }
        rec
    }

    /// Calculate the arclength up to the time along the curve
    ///
    /// Returns [`None`] if `t` is not 0-1 or `samples` is 0
    pub fn length_to(&self, t: f32, samples: usize) -> Option<f32> {
        if samples == 0 || t < 0.0 || 1.0 < t { return None; }
        // todo: i dont like this :c
        // im gonna see if i can make it O(1)
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
    pub fn position_at(&self, t: f32) -> Vector2 {
        let coefs = (
            (1.0 - t).powi(3),
            3.0*(1.0 - t).powi(2)*t,
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
    pub fn velocity_at(&self, t: f32) -> Vector2 {
        let coefs = (
            3.0*(1.0 - t).powi(2),
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
    pub fn jerk(&self) -> Vector2 {
        let (p0, p1, p2, p3) = (self.p1, self.c1_out, self.c2_in, self.p2);
        Vector2 {
            x: 6.0*(p3.x + 3.0*(p1.x - p2.x) - p0.x),
            y: 6.0*(p3.y + 3.0*(p1.y - p2.y) - p0.y),
        }
    }

    /// Time along curve closest to the position
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
