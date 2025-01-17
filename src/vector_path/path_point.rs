use raylib::prelude::*;

use super::mat2::Matrix2x2;

pub struct PathPoint {
    /// Absolute, not relative
    pub c_in: Vector2,
    pub p: Vector2,
    /// Absolute, not relative
    pub c_out: Vector2,
}

impl PathPoint {
    pub fn new(c_in: Vector2, p: Vector2, c_out: Vector2) -> Self {
        Self {
            c_in,
            p,
            c_out,
        }
    }

    pub fn new_corner(p: Vector2) -> Self {
        Self {
            c_in: p,
            p,
            c_out: p,
        }
    }

    /// Entry velocity is reflection of exit velocity
    pub fn new_smooth(p: Vector2, c_out: Vector2) -> Self {
        Self {
            c_in: p * 2.0 - c_out,
            p,
            c_out,
        }
    }

    /// Exit velocity is reflection of entry velocity
    pub fn edit_control_smooth_in(&mut self, c_in: Vector2) {
        self.c_out = self.p * 2.0 - c_in;
        self.c_in = c_in;
    }

    /// Entry velocity is reflection of exit velocity
    pub fn edit_control_smooth_out(&mut self, c_out: Vector2) {
        self.c_in = self.p * 2.0 - c_out;
        self.c_out = c_out;
    }

    /// Translate the point and controls while keeping the controls' relative positions
    pub fn move_point(&mut self, p: Vector2) {
        let delta = p - self.p;
        self.c_in  += delta;
        self.p     += delta;
        self.c_out += delta;
    }

    /// Move the point and controls while keeping the controls' relative positions
    pub fn set_point(&mut self, p: Vector2) {
        let delta = p - self.p;
        self.c_in  += delta;
        self.p     += delta;
        self.c_out += delta;
    }

    pub fn transform(&mut self, mat: Matrix2x2) {
        self.c_in  = mat * self.c_in;
        self.p     = mat * self.p;
        self.c_out = mat * self.c_out;
    }
}

impl std::ops::Mul<PathPoint> for Matrix2x2 {
    type Output = PathPoint;
    fn mul(self, rhs: PathPoint) -> Self::Output {
        PathPoint {
            c_in:  self * rhs.c_in,
            p:     self * rhs.p,
            c_out: self * rhs.c_out,
        }
    }
}
