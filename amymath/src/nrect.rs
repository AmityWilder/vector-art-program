use std::ops::*;
use raylib::prelude::*;
use crate::prelude::*;

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

pub trait Rect: Sized + Union<Output = Self> + Intersection<Output = Self> {
    type Item;
    type Vector: Vector;
}

// -----------------------------------------------------------------------------------------
// Rect2
// -----------------------------------------------------------------------------------------

#[must_use]
pub struct Rect2<T = f32> {
    pub min: Vec2<T>,
    pub max: Vec2<T>,
}

impl<T: Min<Output = T> + Max<Output = T>> Rect for Rect2<T> where Vec2<T>: Vector {
    type Item = T;
    type Vector = Vec2<T>;
}

pub type IRect2 = Rect2<i32>;

impl<T> Rect2<T> {
    #[inline]
    pub const fn new(min: Vec2<T>, max: Vec2<T>) -> Self {
        Self { min, max }
    }

    #[inline]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub fn x_range(&self) -> Range<T> where T: Copy {
        self.min.x..self.max.x
    }

    #[inline]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub fn y_range(&self) -> Range<T> where T: Copy {
        self.min.y..self.max.y
    }

    #[inline]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub fn size(&self) -> Vec2<T::Output> where T: Copy + Sub {
        self.max - self.min
    }

    #[inline]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub fn width(&self) -> T::Output where T: Copy + Sub {
        self.max.x - self.min.x
    }

    #[inline]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub fn height(&self) -> T::Output where T: Copy + Sub {
        self.max.y - self.min.y
    }

    #[inline]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub fn area(&self) -> <<T as Sub>::Output as Mul>::Output where T: Copy + Sub<Output: Mul> {
        self.size().prod()
    }

    #[inline]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub fn offset<U: Copy, V>(self, amount: U) -> Rect2<V> where Vec2<T>: Add<U, Output = Vec2<V>> {
        Rect2 {
            min: self.min + amount,
            max: self.max + amount,
        }
    }

    #[inline]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub fn grow<U: Copy, V>(self, amount: U) -> Rect2<V> where Vec2<T>: Add<U, Output = Vec2<V>> + Sub<U, Output = Vec2<V>> {
        Rect2 {
            min: self.min - amount,
            max: self.max + amount,
        }
    }
}

impl<T: Copy + Neg<Output = T> + Sub<Output = T>> FlipRectangle for Rect2<T> {
    type Output = Self;

    #[inline]
    fn flipped(self) -> Self::Output {
        Self {
            min: Vec2 {
                x: self.min.x,
                y: -self.min.y,
            },
            max: Vec2 {
                x: self.max.x,
                y: -self.max.y - self.min.y - self.min.y,
            },
        }
    }
}

impl<T: Min<Output = T> + Max<Output = T>> Union for Rect2<T> {
    type Output = Self;
    #[inline]
    fn union(self, rhs: Self) -> Self::Output {
        Rect2 {
            min: Vec2::new(self.min.x.min(rhs.min.x), self.min.y.min(rhs.min.y)),
            max: Vec2::new(self.max.x.max(rhs.max.x), self.max.y.max(rhs.max.y)),
        }
    }
}

impl<T: Min<Output = T> + Max<Output = T> + Copy> Union<Vec2<T>> for Rect2<T> {
    type Output = Self;
    #[inline]
    fn union(self, rhs: Vec2<T>) -> Self::Output {
        Rect2 {
            min: Vec2::new(self.min.x.min(rhs.x), self.min.y.min(rhs.y)),
            max: Vec2::new(self.max.x.max(rhs.x), self.max.y.max(rhs.y)),
        }
    }
}

impl<T: Min<Output = T> + Max<Output = T>> Intersection for Rect2<T> {
    type Output = Self;
    #[inline]
    fn intersection(self, rhs: Self) -> Self::Output {
        Rect2 {
            min: Vec2::new(self.min.x.max(rhs.min.x), self.min.y.max(rhs.min.y)),
            max: Vec2::new(self.max.x.min(rhs.max.x), self.max.y.min(rhs.max.y)),
        }
    }
}

impl<T: Min<Output = T> + Max<Output = T> + Copy> Intersection<Vec2<T>> for Rect2<T> {
    type Output = Self;
    #[inline]
    fn intersection(self, rhs: Vec2<T>) -> Self::Output {
        Rect2 {
            min: Vec2::new(self.min.x.max(rhs.x), self.min.y.max(rhs.y)),
            max: Vec2::new(self.max.x.min(rhs.x), self.max.y.min(rhs.y)),
        }
    }
}

impl<T: PartialOrd<U>, U: PartialOrd<T>> Overlap<Rect2<U>> for Rect2<T> {
    #[inline]
    fn contains(&self, item: &Rect2<U>) -> bool {
        self.min.x <= item.min.x && item.max.x <= self.max.x &&
        self.min.y <= item.min.y && item.max.y <= self.max.y
    }

    #[inline]
    fn overlaps(&self, item: &Rect2<U>) -> bool {
        self.min.x < item.max.x && item.min.x < self.max.x &&
        self.min.y < item.max.y && item.min.y < self.max.y
    }
}

impl<T: PartialOrd<U>, U: PartialOrd<T>> Overlap<Vec2<U>> for Rect2<T> {
    #[inline]
    fn contains(&self, item: &Vec2<U>) -> bool {
        self.min.x <= item.x && item.x <= self.max.x &&
        self.min.y <= item.y && item.y <= self.max.y
    }
}

impl From<Rectangle> for Rect2<f32> {
    #[inline]
    fn from(value: Rectangle) -> Self {
        Self {
            min: Vec2::new(value.x, value.y),
            max: Vec2::new(value.x + value.width, value.y + value.height),
        }
    }
}

impl From<Rect2<f32>> for Rectangle {
    #[inline]
    fn from(value: Rect2<f32>) -> Self {
        let size = value.size();
        Self {
            x: value.min.x,
            y: value.min.y,
            width:  size.x,
            height: size.y,
        }
    }
}

impl<T> From<Range<Vec2<T>>> for Rect2<T> {
    #[inline]
    fn from(value: Range<Vec2<T>>) -> Self {
        Self {
            min: value.start,
            max: value.end,
        }
    }
}

impl<T> From<Rect2<T>> for Range<Vec2<T>> {
    #[inline]
    fn from(value: Rect2<T>) -> Self {
        value.min..value.max
    }
}

impl<T> From<(Range<T>, Range<T>)> for Rect2<T> {
    #[inline]
    fn from((x, y): (Range<T>, Range<T>)) -> Self {
        Self {
            min: Vec2::new(x.start, y.start),
            max: Vec2::new(x.end, y.end),
        }
    }
}

impl<T> From<Rect2<T>> for (Range<T>, Range<T>) {
    #[inline]
    fn from(rec: Rect2<T>) -> Self {
        (rec.min.x..rec.max.x, rec.min.y..rec.max.y)
    }
}

impl<T> From<[Range<T>; 2]> for Rect2<T> {
    #[inline]
    fn from([x, y]: [Range<T>; 2]) -> Self {
        Self {
            min: Vec2::new(x.start, y.start),
            max: Vec2::new(x.end, y.end),
        }
    }
}

impl<T> From<Rect2<T>> for [Range<T>; 2] {
    #[inline]
    fn from(rec: Rect2<T>) -> Self {
        [rec.min.x..rec.max.x, rec.min.y..rec.max.y]
    }
}

impl<T: std::fmt::Debug> std::fmt::Debug for Rect2<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Rect2")
            .field("min", &self.min)
            .field("max", &self.max)
            .finish()
    }
}

impl<T: Default> Default for Rect2<T> {
    #[inline]
    fn default() -> Self {
        Self {
            min: Default::default(),
            max: Default::default(),
        }
    }
}

impl<T: Clone> Clone for Rect2<T> {
    #[inline]
    fn clone(&self) -> Self {
        Self {
            min: self.min.clone(),
            max: self.max.clone(),
        }
    }
}

impl<T: Copy> Copy for Rect2<T> {}

impl<T: PartialEq> PartialEq for Rect2<T> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.min == other.min && self.max == other.max
    }
}

impl<T: Eq> Eq for Rect2<T> {}

impl<T: std::hash::Hash> std::hash::Hash for Rect2<T> {
    #[inline]
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.min.hash(state);
        self.max.hash(state);
    }
}

impl<T> RangeBounds<Vec2<T>> for Rect2<T> {
    #[inline]
    fn start_bound(&self) -> Bound<&Vec2<T>> {
        Bound::Included(&self.min)
    }

    #[inline]
    fn end_bound(&self) -> Bound<&Vec2<T>> {
        Bound::Excluded(&self.max)
    }
}

pub trait DrawRect2: RaylibDraw {
    #[inline]
    fn draw_rectangle_rect2(&mut self, rec: Rect2, color: Color) {
        self.draw_rectangle_rec(Rectangle::from(rec), color);
    }

    #[inline]
    fn draw_rectangle_lines_rect2(&mut self, rec: &Rect2, color: Color) {
        self.draw_line_strip(&[
            Vector2::from(rec.min),
            Vector2::new(rec.max.x, rec.min.y),
            Vector2::from(rec.max),
            Vector2::new(rec.min.x, rec.max.y),
            Vector2::from(rec.min),
        ], color);
    }
}

impl<D: RaylibDraw> DrawRect2 for D {}

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

// -----------------------------------------------------------------------------------------
// Rect3
// -----------------------------------------------------------------------------------------

#[must_use]
pub struct Rect3<T = f32> {
    pub min: Vec3<T>,
    pub max: Vec3<T>,
}

impl<T: Min<Output = T> + Max<Output = T>> Rect for Rect3<T> where Vec3<T>: Vector {
    type Item = T;
    type Vector = Vec3<T>;
}

pub type IRect3 = Rect3<i32>;

impl<T> Rect3<T> {
    #[inline]
    pub const fn new(min: Vec3<T>, max: Vec3<T>) -> Self {
        Self { min, max }
    }

    #[inline]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub fn x_range(&self) -> Range<T> where T: Copy {
        self.min.x..self.max.x
    }

    #[inline]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub fn y_range(&self) -> Range<T> where T: Copy {
        self.min.y..self.max.y
    }

    #[inline]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub fn z_range(&self) -> Range<T> where T: Copy {
        self.min.z..self.max.z
    }

    #[inline]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub fn size(&self) -> Vec3<T::Output> where T: Copy + Sub {
        self.max - self.min
    }

    #[inline]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub fn width(&self) -> T::Output where T: Copy + Sub {
        self.max.x - self.min.x
    }

    #[inline]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub fn height(&self) -> T::Output where T: Copy + Sub {
        self.max.y - self.min.y
    }

    #[inline]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub fn depth(&self) -> T::Output where T: Copy + Sub {
        self.max.z - self.min.z
    }

    #[inline]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub fn area(&self) -> T where T: Copy + Sub<Output = T> + Mul<Output = T> {
        self.size().prod()
    }

    #[inline]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub fn offset<U: Copy, V>(self, amount: U) -> Rect3<V> where Vec3<T>: Add<U, Output = Vec3<V>> {
        Rect3 {
            min: self.min + amount,
            max: self.max + amount,
        }
    }

    #[inline]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub fn grow<U: Copy, V>(self, amount: U) -> Rect3<V> where Vec3<T>: Add<U, Output = Vec3<V>> + Sub<U, Output = Vec3<V>> {
        Rect3 {
            min: self.min - amount,
            max: self.max + amount,
        }
    }
}

impl<T: Min<Output = T> + Max<Output = T>> Union for Rect3<T> {
    type Output = Self;
    #[inline]
    fn union(self, rhs: Self) -> Self::Output {
        Rect3 {
            min: Vec3::new(self.min.x.min(rhs.min.x), self.min.y.min(rhs.min.y), self.min.z.min(rhs.min.z)),
            max: Vec3::new(self.max.x.max(rhs.max.x), self.max.y.max(rhs.max.y), self.max.z.max(rhs.max.z)),
        }
    }
}

impl<T: Min<Output = T> + Max<Output = T> + Copy> Union<Vec3<T>> for Rect3<T> {
    type Output = Self;
    #[inline]
    fn union(self, rhs: Vec3<T>) -> Self::Output {
        Rect3 {
            min: Vec3::new(self.min.x.min(rhs.x), self.min.y.min(rhs.y), self.min.z.min(rhs.z)),
            max: Vec3::new(self.max.x.max(rhs.x), self.max.y.max(rhs.y), self.max.z.max(rhs.z)),
        }
    }
}

impl<T: Min<Output = T> + Max<Output = T>> Intersection for Rect3<T> {
    type Output = Self;
    #[inline]
    fn intersection(self, rhs: Self) -> Self::Output {
        Rect3 {
            min: Vec3::new(self.min.x.max(rhs.min.x), self.min.y.max(rhs.min.y), self.min.z.max(rhs.min.z)),
            max: Vec3::new(self.max.x.min(rhs.max.x), self.max.y.min(rhs.max.y), self.max.z.min(rhs.max.z)),
        }
    }
}

impl<T: Min<Output = T> + Max<Output = T> + Copy> Intersection<Vec3<T>> for Rect3<T> {
    type Output = Self;
    #[inline]
    fn intersection(self, rhs: Vec3<T>) -> Self::Output {
        Rect3 {
            min: Vec3::new(self.min.x.max(rhs.x), self.min.y.max(rhs.y), self.min.z.max(rhs.z)),
            max: Vec3::new(self.max.x.min(rhs.x), self.max.y.min(rhs.y), self.max.z.min(rhs.z)),
        }
    }
}

impl<T: PartialOrd<U>, U: PartialOrd<T>> Overlap<Rect3<U>> for Rect3<T> {
    #[inline]
    fn contains(&self, item: &Rect3<U>) -> bool {
        self.min.x <= item.min.x && item.max.x <= self.max.x &&
        self.min.y <= item.min.y && item.max.y <= self.max.y &&
        self.min.z <= item.min.z && item.max.z <= self.max.z
    }

    #[inline]
    fn overlaps(&self, item: &Rect3<U>) -> bool {
        self.min.x < item.max.x && item.min.x < self.max.x &&
        self.min.y < item.max.y && item.min.y < self.max.y &&
        self.min.z < item.max.z && item.min.z < self.max.z
    }
}

impl<T: PartialOrd<U>, U: PartialOrd<T>> Overlap<Vec3<U>> for Rect3<T> {
    #[inline]
    fn contains(&self, item: &Vec3<U>) -> bool {
        self.min.x <= item.x && item.x <= self.max.x &&
        self.min.y <= item.y && item.y <= self.max.y &&
        self.min.z <= item.z && item.z <= self.max.z
    }
}

impl From<BoundingBox> for Rect3<f32> {
    #[inline]
    fn from(value: BoundingBox) -> Self {
        Self {
            min: Vec3::from(value.min),
            max: Vec3::from(value.max),
        }
    }
}

impl From<Rect3<f32>> for BoundingBox {
    #[inline]
    fn from(value: Rect3<f32>) -> Self {
        Self {
            min: value.min.into(),
            max: value.max.into(),
        }
    }
}

impl<T> From<Range<Vec3<T>>> for Rect3<T> {
    #[inline]
    fn from(value: Range<Vec3<T>>) -> Self {
        Self {
            min: value.start,
            max: value.end,
        }
    }
}

impl<T> From<Rect3<T>> for Range<Vec3<T>> {
    #[inline]
    fn from(value: Rect3<T>) -> Self {
        value.min..value.max
    }
}

impl<T> From<(Range<T>, Range<T>, Range<T>)> for Rect3<T> {
    #[inline]
    fn from((x, y, z): (Range<T>, Range<T>, Range<T>)) -> Self {
        Self {
            min: Vec3::new(x.start, y.start, z.start),
            max: Vec3::new(x.end, y.end, z.end),
        }
    }
}

impl<T> From<Rect3<T>> for (Range<T>, Range<T>, Range<T>) {
    #[inline]
    fn from(rec: Rect3<T>) -> Self {
        (rec.min.x..rec.max.x, rec.min.y..rec.max.y, rec.min.z..rec.max.z)
    }
}

impl<T> From<[Range<T>; 3]> for Rect3<T> {
    #[inline]
    fn from([x, y, z]: [Range<T>; 3]) -> Self {
        Self {
            min: Vec3::new(x.start, y.start, z.start),
            max: Vec3::new(x.end, y.end, z.end),
        }
    }
}

impl<T> From<Rect3<T>> for [Range<T>; 3] {
    #[inline]
    fn from(rec: Rect3<T>) -> Self {
        [rec.min.x..rec.max.x, rec.min.y..rec.max.y, rec.min.z..rec.max.z]
    }
}

impl<T: std::fmt::Debug> std::fmt::Debug for Rect3<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Rect3")
            .field("min", &self.min)
            .field("max", &self.max)
            .finish()
    }
}

impl<T: Default> Default for Rect3<T> {
    #[inline]
    fn default() -> Self {
        Self {
            min: Default::default(),
            max: Default::default(),
        }
    }
}

impl<T: Clone> Clone for Rect3<T> {
    #[inline]
    fn clone(&self) -> Self {
        Self {
            min: self.min.clone(),
            max: self.max.clone(),
        }
    }
}

impl<T: Copy> Copy for Rect3<T> {}

impl<T: PartialEq> PartialEq for Rect3<T> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.min == other.min && self.max == other.max
    }
}

impl<T: Eq> Eq for Rect3<T> {}

impl<T: std::hash::Hash> std::hash::Hash for Rect3<T> {
    #[inline]
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.min.hash(state);
        self.max.hash(state);
    }
}

impl<T> RangeBounds<Vec3<T>> for Rect3<T> {
    #[inline]
    fn start_bound(&self) -> Bound<&Vec3<T>> {
        Bound::Included(&self.min)
    }

    #[inline]
    fn end_bound(&self) -> Bound<&Vec3<T>> {
        Bound::Excluded(&self.max)
    }
}
