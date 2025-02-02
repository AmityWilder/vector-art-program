use raylib::prelude::*;

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

impl CubicBezier {
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
    pub fn _t_nearest_to(&self, _p: Vector2) -> f32 {
        // let (p0, p1, p2, p3) = (self.p1, self.c1_out, self.c2_in, self.p2);
        // let a = Vector2 {
        //     x: p3.x + 3.0*(p1.x - p2.x) - p0.x,
        //     y: p3.y + 3.0*(p1.y - p2.y) - p0.y,
        // };
        // let b = Vector2 {
        //     x: 3.0*(p2.x - 2.0*p1.x + p0.x),
        //     y: 3.0*(p2.y - 2.0*p1.y + p0.y),
        // };
        // let c = Vector2 {
        //     x: 3.0*(p1.x - p0.x),
        //     y: 3.0*(p1.y - p0.y),
        // };
        // let d = Vector2 {
        //     x: p0.x - p.x,
        //     y: p0.y - p.y,
        // };
        // at^3 + bt^2 + ct + d = 0
        // t = [cubic function]
        // let a_inv = a.x.recip();
        // let t = ().cbrt();
        // t

        todo!("t_nearest_to()")
    }
}
