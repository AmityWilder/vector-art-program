use crate::generics::Vector;

/// Order 1 Bezier curve
pub mod linear;
/// Order 2 Bezier curve
pub mod quadratic;
/// Order 3 Bezier curve
pub mod cubic;

pub trait Bezier<V: Vector> {
    /// Returns a broader bounding box of the curve using only the anchors and velocities.
    ///
    /// [`Bezier::bounds`] will always produce a bounding box smaller or equal to this.
    #[inline]
    fn max_bounds(&self) -> V::Rect {
        self.bounds()
    }

    /// Returns a narrower bounding box of the curve using only the anchors.
    ///
    /// [`Bezier::bounds`] will always produce a bounding box larger or equal to this.
    #[inline]
    fn min_bounds(&self) -> V::Rect {
        self.bounds()
    }

    /// Returns the bounding box of the curve.
    fn bounds(&self) -> V::Rect;

    /// Calculate the position at time along the curve.
    fn position_at(&self, t: f32) -> V;

    /// First-degree derivative at time along curve
    ///
    /// Normalize to get tangent direction.
    fn velocity_at(&self, t: f32) -> V;

    /// Second-degree derivative at time along curve.
    #[inline]
    #[allow(unused)]
    fn acceleration_at(&self, t: f32) -> V {
        V::ZERO
    }

    /// Third-degree derivative at time along curve.
    #[inline]
    #[allow(unused)]
    fn jerk_at(&self, t: f32) -> V {
        V::ZERO
    }

    /// Reciprocal radius (or "radians per meter") at time along curve.
    fn curvature_at(&self, t: f32) -> f32;

    /// Time along curve closest to the position.
    fn estimate_time_at(&self, pt: V) -> Option<f32>;
}

#[inline]
pub(self) fn if_bounded(x: f32) -> Option<f32> {
    (0.0 <= x && x <= 1.0).then_some(x)
}
