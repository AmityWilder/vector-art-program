use std::ops::*;
use raylib::prelude::{*, Vector2 as RlVector2};
use crate::prelude::{IRect2, IVector2, Vector2};
use crate::rlgl::*;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
#[must_use]
pub struct Rect2 {
    pub min: Vector2,
    pub max: Vector2,
}

impl From<Rectangle> for Rect2 {
    #[inline]
    fn from(value: Rectangle) -> Self {
        Self {
            min: Vector2::new(value.x, value.y),
            max: Vector2::new(value.x + value.width, value.y + value.height),
        }
    }
}

impl From<Rect2> for Rectangle {
    #[inline]
    fn from(value: Rect2) -> Self {
        let size = value.size();
        Self {
            x: value.min.x,
            y: value.min.y,
            width:  size.x,
            height: size.y,
        }
    }
}

impl From<Range<Vector2>> for Rect2 {
    #[inline]
    fn from(value: Range<Vector2>) -> Self {
        Self {
            min: value.start,
            max: value.end,
        }
    }
}

impl From<Rect2> for Range<Vector2> {
    #[inline]
    fn from(value: Rect2) -> Self {
        value.min..value.max
    }
}

impl From<(Range<f32>, Range<f32>)> for Rect2 {
    #[inline]
    fn from((x, y): (Range<f32>, Range<f32>)) -> Self {
        Self {
            min: Vector2::new(x.start, y.start),
            max: Vector2::new(x.end, y.end),
        }
    }
}

impl From<Rect2> for (Range<f32>, Range<f32>) {
    #[inline]
    fn from(rec: Rect2) -> Self {
        (rec.min.x..rec.max.x, rec.min.y..rec.max.y)
    }
}

impl From<[Range<f32>; 2]> for Rect2 {
    #[inline]
    fn from([x, y]: [Range<f32>; 2]) -> Self {
        Self {
            min: Vector2::new(x.start, y.start),
            max: Vector2::new(x.end, y.end),
        }
    }
}

impl From<Rect2> for [Range<f32>; 2] {
    #[inline]
    fn from(rec: Rect2) -> Self {
        [rec.min.x..rec.max.x, rec.min.y..rec.max.y]
    }
}

impl Rect2 {
    #[inline]
    pub const fn new(min: Vector2, max: Vector2) -> Self {
        Self { min, max }
    }

    #[inline]
    pub const fn as_irect2(self) -> IRect2 {
        IRect2 {
            min: self.min.as_ivec2(),
            max: self.max.as_ivec2(),
        }
    }

    #[inline]
    pub const fn center_and_extent(self) -> (Vector2, Vector2) {
        let extent = Vector2 {
            x: (self.max.x - self.min.x) * 0.5,
            y: (self.max.y - self.min.y) * 0.5,
        };
        let center = self.min + extent;
        (center, extent)
    }

    /// Sorts the inputs
    #[inline]
    pub const fn from_minmax(a: Vector2, b: Vector2) -> Self {
        let (min, max) = a.minmax_v(b);
        Self { min, max }
    }

    #[inline]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub const fn x_range(&self) -> Range<f32> {
        self.min.x..self.max.x
    }

    #[inline]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub const fn y_range(&self) -> Range<f32> {
        self.min.y..self.max.y
    }

    #[inline]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub const fn size(&self) -> Vector2 {
        self.min.delta(self.max)
    }

    #[inline]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub const fn width(&self) -> f32 {
        self.max.x - self.min.x
    }

    #[inline]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub const fn height(&self) -> f32 {
        self.max.y - self.min.y
    }

    #[inline]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub const fn area(&self) -> f32 {
        self.size().prod()
    }

    #[inline]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub const fn offset(self, amount: f32) -> Self {
        Self {
            min: self.min.offset_iso(amount),
            max: self.max.offset_iso(amount),
        }
    }

    #[inline]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub const fn grow_v(self, amount: Vector2) -> Self {
        Self {
            min: self.min.offset(Vector2 { x: -amount.x, y: -amount.y }),
            max: self.max.offset(amount),
        }
    }

    #[inline]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub const fn grow(self, amount: f32) -> Self {
        Self {
            min: self.min.offset_iso(-amount),
            max: self.max.offset_iso( amount),
        }
    }

    #[inline]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub const fn union(self, rhs: Self) -> Self {
        Self {
            min: Vector2::new(self.min.x.min(rhs.min.x), self.min.y.min(rhs.min.y)),
            max: Vector2::new(self.max.x.max(rhs.max.x), self.max.y.max(rhs.max.y)),
        }
    }

    #[inline]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub const fn union_v(self, rhs: Vector2) -> Self {
        Self {
            min: Vector2::new(self.min.x.min(rhs.x), self.min.y.min(rhs.y)),
            max: Vector2::new(self.max.x.max(rhs.x), self.max.y.max(rhs.y)),
        }
    }

    #[inline]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub const fn intersection(self, rhs: Self) -> Self {
        Self {
            min: Vector2::new(self.min.x.max(rhs.min.x), self.min.y.max(rhs.min.y)),
            max: Vector2::new(self.max.x.min(rhs.max.x), self.max.y.min(rhs.max.y)),
        }
    }

    #[inline]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub const fn intersection_v(self, rhs: Vector2) -> Self {
        Self {
            min: Vector2::new(self.min.x.max(rhs.x), self.min.y.max(rhs.y)),
            max: Vector2::new(self.max.x.min(rhs.x), self.max.y.min(rhs.y)),
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
    pub const fn contains_v(&self, item: &Vector2) -> bool {
        self.min.x <= item.x && item.x <= self.max.x &&
        self.min.y <= item.y && item.y <= self.max.y
    }

    #[inline]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub const fn flipped(self) -> Self {
        Self {
            min: Vector2::new(self.min.x, -self.min.y),
            max: Vector2::new(self.max.x, self.max.y - self.min.y * 2.0),
        }
    }
}

impl BitOr for Rect2 {
    type Output = Self;
    /// Union of `self` and `rhs`
    #[inline]
    fn bitor(self, rhs: Self) -> Self::Output {
        self.union(rhs)
    }
}
impl BitOr<Vector2> for Rect2 {
    type Output = Self;
    /// Union of `self` and `rhs`
    #[inline]
    fn bitor(self, rhs: Vector2) -> Self::Output {
        self.union_v(rhs)
    }
}

impl BitAnd for Rect2 {
    type Output = Self;
    /// Intersection of `self` and `rhs`
    #[inline]
    fn bitand(self, rhs: Self) -> Self::Output {
        self.intersection(rhs)
    }
}
impl BitAnd<Vector2> for Rect2 {
    type Output = Self;
    /// Intersection of `self` and `rhs`
    #[inline]
    fn bitand(self, rhs: Vector2) -> Self::Output {
        self.intersection_v(rhs)
    }
}

impl RangeBounds<Vector2> for Rect2 {
    #[inline]
    fn start_bound(&self) -> Bound<&Vector2> {
        Bound::Included(&self.min)
    }

    #[inline]
    fn end_bound(&self) -> Bound<&Vector2> {
        Bound::Excluded(&self.max)
    }
}

pub trait DrawRect2: RaylibDraw {
    #[inline]
    fn draw_rectangle_rect2(&mut self, rec: &Rect2, color: Color) where Self: RaylibRlglExt {
        let shapes_rec = Rect2::from(tex_shapes_rec());
        let shapes_size = IVector2::from(tex_shapes_size()).as_vec2();

        let mut d = self.begin_rlgl();
        d.rl_set_texture_tex_shapes();
        let mut d = d.rl_begin_quads();

        d.rl_normal3f(0.0, 0.0, 1.0);
        d.rl_color4ub(color.r, color.g, color.b, color.a);

        d.rl_tex_coord2f(shapes_rec.min.x/shapes_size.x, shapes_rec.min.y/shapes_size.y);
        d.rl_vertex2f(rec.min.x, rec.min.y);

        d.rl_tex_coord2f(shapes_rec.min.x/shapes_size.x, shapes_rec.max.y/shapes_size.y);
        d.rl_vertex2f(rec.min.x, rec.max.y);

        d.rl_tex_coord2f(shapes_rec.max.x/shapes_size.x, shapes_rec.max.y/shapes_size.y);
        d.rl_vertex2f(rec.max.x, rec.max.y);

        d.rl_tex_coord2f(shapes_rec.max.x/shapes_size.x, shapes_rec.min.y/shapes_size.y);
        d.rl_vertex2f(rec.max.x, rec.min.y);
    }

    #[inline]
    fn draw_rectangle_lines_rect2(&mut self, rec: &Rect2, color: Color) {
        self.draw_line_strip(&[
            RlVector2::from(rec.min),
            RlVector2::new(rec.max.x, rec.min.y),
            RlVector2::from(rec.max),
            RlVector2::new(rec.min.x, rec.max.y),
            RlVector2::from(rec.min),
        ], color);
    }
}

impl<D: RaylibDraw> DrawRect2 for D {}
