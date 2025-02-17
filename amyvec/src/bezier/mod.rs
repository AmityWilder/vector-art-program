use amymath::prelude::*;

/// Order 1 Bezier curve
pub mod linear;
/// Order 2 Bezier curve
pub mod quadratic;
/// Order 3 Bezier curve
pub mod cubic;

pub trait Bezier {
    /// Returns a broader bounding box of the curve using only the anchors and velocities.
    ///
    /// [`Bezier::bounds`] will always produce a bounding box smaller or equal to this.
    #[inline]
    fn max_bounds(&self) -> Rect2 {
        self.bounds()
    }

    /// Returns a narrower bounding box of the curve using only the anchors.
    ///
    /// [`Bezier::bounds`] will always produce a bounding box larger or equal to this.
    #[inline]
    fn min_bounds(&self) -> Rect2 {
        self.bounds()
    }

    /// Returns the bounding box of the curve.
    fn bounds(&self) -> Rect2;

    /// Calculate the position at time along the curve.
    fn position_at(&self, t: f32) -> Vector2;

    /// First-degree derivative at time along curve
    ///
    /// Normalize to get tangent direction.
    fn velocity_at(&self, t: f32) -> Vector2;

    /// Second-degree derivative at time along curve.
    #[inline]
    #[allow(unused)]
    fn acceleration_at(&self, t: f32) -> Vector2 {
        Vector2::ZERO
    }

    /// Third-degree derivative at time along curve.
    #[inline]
    #[allow(unused)]
    fn jerk_at(&self, t: f32) -> Vector2 {
        Vector2::ZERO
    }
}

#[inline]
pub(self) fn if_bounded(x: f32) -> Option<f32> {
    (0.0 <= x && x <= 1.0).then_some(x)
}
