use raylib::prelude::*;
use std::ops::*;
use crate::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[must_use]
pub struct IRect2 {
    pub min: IVector2,
    pub max: IVector2,
}

impl IRect2 {
    #[inline]
    pub const fn new(min: IVector2, max: IVector2) -> Self {
        Self { min, max }
    }

    #[inline]
    pub const fn as_rect2(self) -> Rect2 {
        Rect2 {
            min: self.min.as_vec2(),
            max: self.max.as_vec2(),
        }
    }

    #[inline]
    pub const fn center_and_extent(self) -> (IVector2, IVector2) {
        let extent = IVector2 {
            x: (self.max.x - self.min.x) / 2,
            y: (self.max.y - self.min.y) / 2,
        };
        let center = self.min + extent;
        (center, extent)
    }

    #[inline]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub fn x_range(&self) -> Range<i32> {
        self.min.x..self.max.x
    }

    #[inline]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub fn y_range(&self) -> Range<i32> {
        self.min.y..self.max.y
    }

    #[inline]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub fn size(&self) -> IVector2 {
        self.max - self.min
    }

    #[inline]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub fn width(&self) -> i32 {
        self.max.x - self.min.x
    }

    #[inline]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub fn height(&self) -> i32 {
        self.max.y - self.min.y
    }

    #[inline]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub fn area(&self) -> i32 {
        self.size().prod()
    }

    #[inline]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub fn offset<T: Copy>(self, amount: T) -> IRect2 where IVector2: Add<T, Output = IVector2> {
        IRect2 {
            min: self.min + amount,
            max: self.max + amount,
        }
    }

    #[inline]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub fn grow<T: Copy>(self, amount: T) -> IRect2 where IVector2: Add<T, Output = IVector2> + Sub<T, Output = IVector2> {
        IRect2 {
            min: self.min - amount,
            max: self.max + amount,
        }
    }

    #[inline]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub fn union(self, rhs: Self) -> Self {
        Self {
            min: IVector2::new(self.min.x.min(rhs.min.x), self.min.y.min(rhs.min.y)),
            max: IVector2::new(self.max.x.max(rhs.max.x), self.max.y.max(rhs.max.y)),
        }
    }

    #[inline]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub fn union_v(self, rhs: IVector2) -> Self {
        Self {
            min: IVector2::new(self.min.x.min(rhs.x), self.min.y.min(rhs.y)),
            max: IVector2::new(self.max.x.max(rhs.x), self.max.y.max(rhs.y)),
        }
    }

    #[inline]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub fn intersection(self, rhs: Self) -> Self {
        Self {
            min: IVector2::new(self.min.x.max(rhs.min.x), self.min.y.max(rhs.min.y)),
            max: IVector2::new(self.max.x.min(rhs.max.x), self.max.y.min(rhs.max.y)),
        }
    }

    #[inline]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub fn intersection_v(self, rhs: IVector2) -> Self {
        Self {
            min: IVector2::new(self.min.x.max(rhs.x), self.min.y.max(rhs.y)),
            max: IVector2::new(self.max.x.min(rhs.x), self.max.y.min(rhs.y)),
        }
    }

    #[inline]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub const fn contains(&self, item: &Self) -> bool {
        self.min.x <= item.min.x && item.max.x <= self.max.x &&
        self.min.y <= item.min.y && item.max.y <= self.max.y
    }

    #[inline]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub const fn overlaps(&self, item: &Self) -> bool {
        self.min.x < item.max.x && item.min.x < self.max.x &&
        self.min.y < item.max.y && item.min.y < self.max.y
    }

    #[inline]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub const fn contains_v(&self, item: &IVector2) -> bool {
        self.min.x <= item.x && item.x <= self.max.x &&
        self.min.y <= item.y && item.y <= self.max.y
    }

    #[inline]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub const fn flipped(self) -> Self {
        Self {
            min: IVector2::new(self.min.x, -self.min.y),
            max: IVector2::new(self.max.x, self.max.y - self.min.y * 2),
        }
    }
}

impl BitOr for IRect2 {
    type Output = Self;
    /// Union of `self` and `rhs`
    #[inline]
    fn bitor(self, rhs: Self) -> Self::Output {
        self.union(rhs)
    }
}
impl BitOr<IVector2> for IRect2 {
    type Output = Self;
    /// Union of `self` and `rhs`
    #[inline]
    fn bitor(self, rhs: IVector2) -> Self::Output {
        self.union_v(rhs)
    }
}

impl BitAnd for IRect2 {
    type Output = Self;
    /// Intersection of `self` and `rhs`
    #[inline]
    fn bitand(self, rhs: Self) -> Self::Output {
        self.intersection(rhs)
    }
}
impl BitAnd<IVector2> for IRect2 {
    type Output = Self;
    /// Intersection of `self` and `rhs`
    #[inline]
    fn bitand(self, rhs: IVector2) -> Self::Output {
        self.intersection_v(rhs)
    }
}

impl RangeBounds<IVector2> for IRect2 {
    #[inline]
    fn start_bound(&self) -> Bound<&IVector2> {
        Bound::Included(&self.min)
    }

    #[inline]
    fn end_bound(&self) -> Bound<&IVector2> {
        Bound::Excluded(&self.max)
    }
}

impl From<Range<IVector2>> for IRect2 {
    #[inline]
    fn from(value: Range<IVector2>) -> Self {
        Self {
            min: value.start,
            max: value.end,
        }
    }
}

impl From<IRect2> for Range<IVector2> {
    #[inline]
    fn from(value: IRect2) -> Self {
        value.min..value.max
    }
}

impl From<(Range<i32>, Range<i32>)> for IRect2 {
    #[inline]
    fn from((x, y): (Range<i32>, Range<i32>)) -> Self {
        Self {
            min: IVector2::new(x.start, y.start),
            max: IVector2::new(x.end, y.end),
        }
    }
}

impl From<IRect2> for (Range<i32>, Range<i32>) {
    #[inline]
    fn from(rec: IRect2) -> Self {
        (rec.min.x..rec.max.x, rec.min.y..rec.max.y)
    }
}

impl From<[Range<i32>; 2]> for IRect2 {
    #[inline]
    fn from([x, y]: [Range<i32>; 2]) -> Self {
        Self {
            min: IVector2::new(x.start, y.start),
            max: IVector2::new(x.end, y.end),
        }
    }
}

impl From<IRect2> for [Range<i32>; 2] {
    #[inline]
    fn from(rec: IRect2) -> Self {
        [rec.min.x..rec.max.x, rec.min.y..rec.max.y]
    }
}

pub trait DrawIRect2: RaylibDraw {
    #[inline]
    fn draw_rectangle_irect2(&mut self, rec: &IRect2, color: Color) {
        let size = rec.size();
        self.draw_rectangle(rec.min.x, rec.min.y, size.x, size.y, color);
    }
}

impl<D: RaylibDraw> DrawIRect2 for D {}

pub trait RaylibIRect2ScissorExt: RaylibScissorModeExt {
    #[inline]
    fn begin_scissor_mode_irect2(&mut self, rec: &IRect2) -> RaylibScissorMode<Self> where Self: RaylibScissorModeExt {
        let size = rec.size();
        self.begin_scissor_mode(rec.min.x, rec.min.y, size.x, size.y)
    }
}

impl<D: RaylibDraw> RaylibIRect2ScissorExt for D {}
