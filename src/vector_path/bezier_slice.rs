use raylib::prelude::*;
use super::path_point::PathPoint;

pub struct BezierSlice<'a> {
    a: &'a PathPoint,
    b: &'a PathPoint,
}

impl<'a> BezierSlice<'a> {
    pub(super) fn new(a: &'a PathPoint, b: &'a PathPoint) -> Self {
        Self { a, b }
    }
}

impl BezierSlice<'_> {
    pub fn position_at(&self, t: f32) -> Vector2 {
        let coefs = (
            (1.0 - t).powi(3),
            3.0*(1.0 - t).powi(2)*t,
            3.0*(1.0 - t)*t*t,
            t*t*t,
        );
        let ((_, p0, p1), (p2, p3, _)) = (self.a.calculate(), self.b.calculate());
        Vector2 {
            x: coefs.0*p0.x + coefs.1*p1.x + coefs.2*p2.x + coefs.3*p3.x,
            y: coefs.0*p0.y + coefs.1*p1.y + coefs.2*p2.y + coefs.3*p3.y,
        }
    }

    pub fn velocity_at(&self, t: f32) -> Vector2 {
        let coefs = (
            3.0*(1.0 - t).powi(2),
            6.0*(1.0 - t)*t,
            3.0*t*t,
        );
        let ((_, p0, p1), (p2, p3, _)) = (self.a.calculate(), self.b.calculate());
        Vector2 {
            x: coefs.0*(p1.x - p0.x) + coefs.1*(p2.x - p1.x) + coefs.2*(p3.x - p2.x),
            y: coefs.0*(p1.y - p0.y) + coefs.1*(p2.y - p1.y) + coefs.2*(p3.y - p2.y),
        }
    }

    pub fn acceleration_at(&self, t: f32) -> Vector2 {
        let coefs = (
            6.0*(1.0 - t),
            6.0*t,
        );
        let ((_, p0, p1), (p2, p3, _)) = (self.a.calculate(), self.b.calculate());
        Vector2 {
            x: coefs.0*(p2.x - 2.0*p1.x + p0.x) + coefs.1*(p3.x - 2.0*p2.x + p1.x),
            y: coefs.0*(p2.y - 2.0*p1.y + p0.y) + coefs.1*(p3.y - 2.0*p2.y + p1.y),
        }
    }

    pub fn jerk(&self) -> Vector2 {
        let ((_, p0, p1), (p2, p3, _)) = (self.a.calculate(), self.b.calculate());
        Vector2 {
            x: 6.0*(p3.x + 3.0*(p1.x - p2.x) - p0.x),
            y: 6.0*(p3.y + 3.0*(p1.y - p2.y) - p0.y),
        }
    }

    pub fn t_nearest_to(&self, p: Vector2) -> f32 {
        let ((_, p0, p1), (p2, p3, _)) = (self.a.calculate(), self.b.calculate());
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
