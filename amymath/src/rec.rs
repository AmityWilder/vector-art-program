use raylib::prelude::Rectangle;

pub trait FlipRectangle {
    type Output;
    fn flipped(self) -> Self::Output;
}

impl FlipRectangle for Rectangle {
    type Output = Self;

    #[inline]
    fn flipped(self) -> Self::Output {
        Self {
            x:  self.x,
            y: -self.y,
            width:  self.width,
            height: -self.height,
        }
    }
}

#[const_trait]
pub trait Overlap<Rhs = Self> {
    /// The minimum of `item` is greater than the minimum of `self` and
    /// The maximum of `item` is less than the maximum of `self`
    #[must_use]
    fn contains(&self, item: &Rhs) -> bool;

    /// The minimum of `item` is less than the maximum of `self` and
    /// The maximum of `item` is greater than the minimum of `self`
    #[inline]
    #[must_use]
    fn overlaps(&self, item: &Rhs) -> bool {
        self.contains(item)
    }
}
