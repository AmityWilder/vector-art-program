use num::complex::Complex32;

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
mod tests {
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
        let (p0_x, p0_y): (f32, f32) = (-0.3,   0.2  );
        let (p1_x, p1_y): (f32, f32) = ( 0.117, 0.55 );
        let (p2_x, p2_y): (f32, f32) = ( 0.98,  0.62 );
        let (p3_x, p3_y): (f32, f32) = ( 1.46,  0.043);
        let (pt_x, pt_y): (f32, f32) = ( 0.492, 0.475);
        let (p0, p1, p2, p3, p) = (p0_x - p0_y, p1_x - p1_y, p2_x - p2_y, p3_x - p3_y, pt_x - pt_y);
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
