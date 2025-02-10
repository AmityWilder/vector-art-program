use raylib::prelude::*;

pub trait CrossProduct<Rhs = Self> {
    type Output;
    fn cross(self, rhs: Rhs) -> Self::Output;
}

impl CrossProduct<Vector2> for f32 {
    type Output = Vector2;
    #[inline]
    fn cross(self, rhs: Vector2) -> Self::Output {
        Vector2 {
            x: -self * rhs.y,
            y:  self * rhs.x,
        }
    }
}

impl CrossProduct for Vector2 {
    type Output = f32;
    #[inline]
    fn cross(self, rhs: Vector2) -> Self::Output {
        self.x * rhs.y - self.y * rhs.x
    }
}

pub trait ReflectVector {
    fn reflected_over(&self, across: Self) -> Self;
    fn reflected_at(&self, across: Self, scale: f32) -> Self;
    fn reflected_to(&self, across: Self, length: f32) -> Self;
}

impl ReflectVector for Vector2 {
    #[inline]
    fn reflected_over(&self, across: Self) -> Self {
        Self {
            x: across.x * 2.0 - self.x,
            y: across.y * 2.0 - self.y,
        }
    }

    #[inline]
    fn reflected_at(&self, across: Self, scale: f32) -> Self {
        let delta = *self - across;
        Self {
            x: across.x - scale * delta.x,
            y: across.y - scale * delta.y,
        }
    }

    #[inline]
    fn reflected_to(&self, across: Self, mut length: f32) -> Self {
        let delta = *self - across;
        length /= delta.length();
        Self {
            x: across.x - length * delta.x,
            y: across.y - length * delta.y,
        }
    }
}

pub trait DistanceSqr {
    /// Distance squared
    fn distance_sqr_to(&self, v: Self) -> f32;

    /// Rectangular distance \
    /// Radius to edge
    fn rec_distance_to(&self, v: Self) -> f32;

    /// Diamond distance \
    /// Radius to corner
    fn dia_distance_to(&self, v: Self) -> f32;
}

impl DistanceSqr for Vector2 {
    #[inline]
    fn distance_sqr_to(&self, v: Self) -> f32 {
        (*self - v).length()
    }

    #[inline]
    fn rec_distance_to(&self, v: Self) -> f32 {
        (self.x - v.x).abs().max((self.y - v.y).abs())
    }

    #[inline]
    fn dia_distance_to(&self, v: Self) -> f32 {
        (self.x - v.x).abs() + (self.y - v.y).abs()
    }
}

pub trait SnapTo {
    fn snap_to_segment(&self, start: Self, end: Self) -> Self;
    fn distance_sqr_to_segment(&self, start: Self, end: Self) -> f32;
}

impl SnapTo for Vector2 {
    fn snap_to_segment(&self, start: Self, end: Self) -> Self {
        let delta = end - start;
        let p_delta = *self - start;
        let delta_sqr = delta * delta;
        let p_delta_delta = p_delta * delta;
        let t = (p_delta_delta.x + p_delta_delta.y) / (delta_sqr.x + delta_sqr.y);
        start * (1.0 - t) + *self * t
    }

    #[inline]
    fn distance_sqr_to_segment(&self, start: Self, end: Self) -> f32 {
        self.distance_sqr_to(self.snap_to_segment(start, end))
    }
}

pub trait Rotate90 {
    fn rotate90_cw(&self) -> Self;
    fn rotate90_cc(&self) -> Self;
}

impl Rotate90 for Vector2 {
    #[inline]
    fn rotate90_cw(&self) -> Self {
        Self {
            x: -self.y,
            y:  self.x,
        }
    }

    #[inline]
    fn rotate90_cc(&self) -> Self {
        Self {
            x:  self.y,
            y: -self.x,
        }
    }
}
