use raylib::prelude::*;

pub trait RectangleCenter {
    fn center(&self) -> Vector2;
    fn extent(&self) -> Vector2;

    #[inline]
    fn center_and_extent(&self) -> (Vector2, Vector2) {
        (self.center(), self.extent())
    }
}

impl RectangleCenter for Rectangle {
    #[inline]
    fn center(&self) -> Vector2 {
        Vector2 {
            x: self.x + 0.5 * self.width,
            y: self.y + 0.5 * self.height,
        }
    }

    #[inline]
    fn extent(&self) -> Vector2 {
        Vector2 {
            x: 0.5 * self.width,
            y: 0.5 * self.height,
        }
    }
}

impl RectangleCenter for Rect2 {
    #[inline]
    fn center(&self) -> Vector2 {
        self.min.midpoint(self.max)
    }

    #[inline]
    fn extent(&self) -> Vector2 {
        Vector2 {
            x: 0.5 * self.width (),
            y: 0.5 * self.height(),
        }
    }
}

impl RectangleCenter for IRect2 {
    #[inline]
    fn center(&self) -> Vector2 {
        Vector2 {
            x: (self.xmin as f32).midpoint(self.xmax as f32),
            y: (self.ymin as f32).midpoint(self.ymax as f32),
        }
    }

    #[inline]
    fn extent(&self) -> Vector2 {
        Vector2 {
            x: 0.5 * self.width () as f32,
            y: 0.5 * self.height() as f32,
        }
    }
}

pub trait MinMaxRectangle {
    fn minmax_rec(&self, other: Self) -> Rectangle;
    fn midpoint(&self, other: Self) -> Self;
}

impl MinMaxRectangle for Vector2 {
    #[inline]
    fn minmax_rec(&self, other: Self) -> Rectangle {
        Rectangle {
            x: self.x.min(other.x),
            y: self.y.min(other.y),
            width:  (other.x - self.x).abs(),
            height: (other.y - self.y).abs(),
        }
    }

    #[inline]
    fn midpoint(&self, other: Self) -> Self {
        Self {
            x: self.x.midpoint(other.x),
            y: self.y.midpoint(other.y),
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Rect2 {
    min: Vector2,
    max: Vector2,
}

impl From<IRect2> for Rect2 {
    #[inline]
    fn from(IRect2 { xmin, ymin, xmax, ymax }: IRect2) -> Self {
        Self {
            min: Vector2 {
                x: xmin as f32,
                y: ymin as f32,
            },
            max: Vector2 {
                x: xmax as f32,
                y: ymax as f32,
            },
        }
    }
}

impl Rect2 {
    pub fn from_center_and_extent(center: Vector2, extent: Vector2) -> Self {
        Self {
            min: center - extent,
            max: center + extent,
        }
    }

    pub fn from_center_and_radius(center: Vector2, radius: f32) -> Self {
        Self {
            min: center - radius,
            max: center + radius,
        }
    }

    #[inline]
    pub fn is_overlapping_point(&self, point: Vector2) -> bool {
        self.min.x <= point.x && point.x < self.max.x &&
        self.min.y <= point.y && point.y < self.max.y
    }

    #[inline]
    pub fn is_overlapping(&self, rec: &Self) -> bool {
        self.min.x < rec.max.x && rec.min.x < self.max.x &&
        self.min.y < rec.max.y && rec.min.y < self.max.y
    }

    #[inline]
    pub fn entirely_contains(&self, rec: &Self) -> bool {
        self.min.x <= rec.min.x && rec.max.x <= self.max.x &&
        self.min.y <= rec.min.y && rec.max.y <= self.max.y
    }

    #[inline]
    pub fn width(&self) -> f32 {
        self.max.x - self.min.x
    }

    #[inline]
    pub fn height(&self) -> f32 {
        self.max.y - self.min.y
    }

    #[inline]
    pub fn grow(self, amnt: f32) -> Self {
        Self {
            min: self.min - amnt,
            max: self.max + amnt,
        }
    }

    #[inline]
    pub fn scale(self, origin: Vector2, amount: f32) -> Self {
        self.scale_ex(origin, Vector2::new(amount, amount))
    }

    #[inline]
    pub fn scale_ex(self, origin: Vector2, amount: Vector2) -> Self {
        Self {
            min: origin + (self.min - origin) * amount,
            max: origin + (self.max - origin) * amount,
        }
    }

    #[inline]
    pub fn scale_from_center(self, amount: f32) -> Self {
        self.scale_from_center_ex(Vector2::new(amount, amount))
    }

    #[inline]
    pub fn scale_from_center_ex(self, amount: Vector2) -> Self {
        self.scale_ex(self.center(), amount)
    }

    /// Returns a rectangle that can entirely contain both input rects
    #[inline]
    pub fn max(self, other: Self) -> Self {
        Self {
            min: Vector2 {
                x: self.min.x.min(other.min.x),
                y: self.min.y.min(other.min.y),
            },
            max: Vector2 {
                x: self.max.x.max(other.max.x),
                y: self.max.y.max(other.max.y),
            },
        }
    }

    /// Adds the point to the bounds of this rect
    #[inline]
    pub fn max_pt(self, p: Vector2) -> Self {
        Self {
            min: Vector2 {
                x: self.min.x.min(p.x),
                y: self.min.y.min(p.y),
            },
            max: Vector2 {
                x: self.max.x.max(p.x),
                y: self.max.y.max(p.y),
            },
        }
    }

    #[inline]
    pub fn minmax_rec(p1: Vector2, p2: Vector2) -> Self {
        Self {
            min: Vector2 {
                x: p1.x.min(p2.x),
                y: p1.y.min(p2.y),
            },
            max: Vector2 {
                x: p1.x.max(p2.x),
                y: p1.y.max(p2.y),
            },
        }
    }
}

pub trait DrawRect2Lines: RaylibDraw {
    #[inline]
    fn draw_rectangle_lines_rect2(&mut self, rec: Rect2, color: Color) {
        self.draw_line_strip(&[
            rec.min,
            Vector2::new(rec.max.x, rec.min.y),
            rec.max,
            Vector2::new(rec.min.x, rec.max.y),
            rec.min,
        ], color);
    }
}

impl<D: RaylibDraw> DrawRect2Lines for D {}

impl From<Rectangle> for Rect2 {
    #[inline]
    fn from(Rectangle { x, y, width, height }: Rectangle) -> Self {
        Self {
            min: Vector2 { x, y },
            max: Vector2 { x: x + width, y: y + height },
        }
    }
}

impl From<Rect2> for Rectangle {
    #[inline]
    fn from(Rect2 { min, max }: Rect2) -> Self {
        Self {
            x: min.x,
            y: min.y,
            width:  max.x - min.x,
            height: max.y - min.y,
        }
    }
}

impl From<Rect2> for ffi::Rectangle {
    #[inline]
    fn from(Rect2 { min, max }: Rect2) -> Self {
        Self {
            x: min.x,
            y: min.y,
            width:  max.x - min.x,
            height: max.y - min.y,
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct IRect2 {
    pub xmin: i32,
    pub ymin: i32,
    pub xmax: i32,
    pub ymax: i32,
}

impl IRect2 {
    #[inline]
    pub fn is_overlapping_v(&self, point: Vector2) -> bool {
        self.is_overlapping_point(point.x.trunc() as i32, point.y.trunc() as i32)
    }

    #[inline]
    pub fn is_overlapping_point(&self, x: i32, y: i32) -> bool {
        self.xmin <= x && x < self.xmax &&
        self.ymin <= y && y < self.ymax
    }

    #[inline]
    pub fn is_overlapping(&self, rec: &Self) -> bool {
        self.xmin < rec.xmax && rec.xmin < self.xmax &&
        self.ymin < rec.ymax && rec.ymin < self.ymax
    }

    #[inline]
    pub fn offset(&mut self, x: i32, y: i32) {
        self.xmin += x;
        self.ymin += y;
        self.xmax += x;
        self.ymax += y;
    }

    #[inline]
    pub fn with_offset(&self, x: i32, y: i32) -> Self {
        Self {
            xmin: self.xmin + x,
            ymin: self.ymin + y,
            xmax: self.xmax + x,
            ymax: self.ymax + y,
        }
    }

    #[inline]
    pub fn intersect(&self, other: &Self) -> Self {
        Self {
            xmin: self.xmin.max(other.xmin),
            ymin: self.ymin.max(other.ymin),
            xmax: self.xmax.min(other.xmax),
            ymax: self.ymax.min(other.ymax),
        }
    }

    #[inline]
    pub fn width(&self) -> i32 {
        self.xmax - self.xmin
    }

    #[inline]
    pub fn height(&self) -> i32 {
        self.ymax - self.ymin
    }

    /// Iterate over `x` coordinates
    #[inline]
    pub fn iter_x(&self) -> impl Iterator<Item = i32> {
        self.xmin..=self.xmax
    }

    /// Iterate over `y` coordinates
    #[inline]
    pub fn iter_y(&self) -> impl Iterator<Item = i32> {
        self.ymin..=self.ymax
    }

    /// Iterate over `x`,`y` coordinates by `for y { for x { ... } }`
    #[inline]
    pub fn iter_xy_row_col(&self) -> impl Iterator<Item = (i32, i32)> {
        let (xmin, xmax) = (self.xmin, self.xmax);
        (self.ymin..=(self.ymax))
            .flat_map(move |y| (xmin..=xmax)
                .map(move |x| (x, y)))
    }

    /// Iterate over `x`,`y` coordinates by `for x { for y { ... } }`
    #[inline]
    pub fn iter_xy_col_row(&self) -> impl Iterator<Item = (i32, i32)> {
        let (ymin, ymax) = (self.ymin, self.ymax);
        (self.xmin..=self.xmax)
            .flat_map(move |x| (ymin..=ymax)
                .map(move |y| (x, y)))
    }
}

impl From<Rectangle> for IRect2 {
    #[inline]
    fn from(Rectangle { x, y, width, height }: Rectangle) -> Self {
        Self {
            xmin: x as i32,
            ymin: y as i32,
            xmax: (x + width) as i32,
            ymax: (y + height) as i32,
        }
    }
}

impl From<IRect2> for Rectangle {
    #[inline]
    fn from(IRect2 { xmin, ymin, xmax, ymax }: IRect2) -> Self {
        Self {
            x: xmin as f32,
            y: ymin as f32,
            width:  xmax as f32 - xmin as f32,
            height: ymax as f32 - ymin as f32,
        }
    }
}

impl From<IRect2> for ffi::Rectangle {
    #[inline]
    fn from(IRect2 { xmin, ymin, xmax, ymax }: IRect2) -> Self {
        Self {
            x: xmin as f32,
            y: ymin as f32,
            width:  xmax as f32 - xmin as f32,
            height: ymax as f32 - ymin as f32,
        }
    }
}

pub trait RaylibIntRect2Ex: RaylibDraw {
    #[inline]
    fn begin_scissor_mode_irect2(&mut self, rect: &IRect2) -> RaylibScissorMode<Self> where Self: RaylibScissorModeExt {
        <Self as RaylibScissorModeExt>::begin_scissor_mode(self, rect.xmin, rect.ymin, rect.width(), rect.height())
    }

    #[inline]
    fn draw_rectangle_irect2(&mut self, rect: &IRect2, color: Color) {
        <Self as RaylibDraw>::draw_rectangle(self, rect.xmin, rect.ymin, rect.width(), rect.height(), color);
    }
}

impl<D: RaylibDraw> RaylibIntRect2Ex for D {}

#[derive(Debug, Clone, Copy)]
pub struct IntRectangle {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

impl From<IntRectangle> for Rectangle {
    #[inline]
    fn from(value: IntRectangle) -> Self {
        Rectangle {
            x:      value.x      as f32,
            y:      value.y      as f32,
            width:  value.width  as f32,
            height: value.height as f32,
        }
    }
}

impl IntRectangle {
    #[inline]
    pub fn offset(&mut self, x: i32, y: i32) {
        self.x += x;
        self.y += y;
    }

    #[inline]
    pub fn with_offset(&self, x: i32, y: i32) -> Self {
        Self {
            x: self.x + x,
            y: self.y + y,
            width:  self.width,
            height: self.height,
        }
    }

    #[inline]
    pub fn intersect(&self, container: &Self) -> Self {
        let x = self.x.max(container.x);
        let y = self.y.max(container.y);
        let width  = (self.x + self.width ).min(container.x + container.width ) - x;
        let height = (self.y + self.height).min(container.y + container.height) - y;
        Self { x, y, width, height }
    }

    #[inline]
    pub const fn left(&self) -> i32 {
        self.x
    }
    #[inline]
    pub const fn right(&self) -> i32 {
        self.x + self.width
    }
    #[inline]
    pub const fn top(&self) -> i32 {
        self.y
    }
    #[inline]
    pub const fn bottom(&self) -> i32 {
        self.y + self.height
    }

    /// Iterate over `x` coordinates
    #[inline]
    pub fn iter_x(&self) -> impl Iterator<Item = i32> {
        self.x..=(self.x + self.width)
    }

    /// Iterate over `y` coordinates
    #[inline]
    pub fn iter_y(&self) -> impl Iterator<Item = i32> {
        self.y..=(self.y + self.height)
    }

    /// Iterate over `x`,`y` coordinates by `for y { for x { ... } }`
    #[inline]
    pub fn iter_xy_row_col(&self) -> impl Iterator<Item = (i32, i32)> {
        let x = self.x;
        let width = self.width;
        (self.y..=(self.y + self.height))
            .flat_map(move |y| (x..=x + width)
                .map(move |x| (x, y)))
    }

    /// Iterate over `x`,`y` coordinates by `for x { for y { ... } }`
    #[inline]
    pub fn iter_xy_col_row(&self) -> impl Iterator<Item = (i32, i32)> {
        let y = self.y;
        let height = self.height;
        (self.x..=(self.x + self.width))
            .flat_map(move |x| (y..=y + height)
                .map(move |y| (x, y)))
    }

    /// Iterate over normalized `x` coordinates (`self.x` = 0, `self.x + self.width` = 1)
    #[inline]
    pub fn iter_u(&self) -> impl Iterator<Item = f32> {
        let inv_width = (self.width as f32).recip();
        (0..=self.width).map(move |x| inv_width * x as f32)
    }

    /// Iterate over normalized `y` coordinates (`self.y` = 0, `self.y + self.height` = 1)
    #[inline]
    pub fn iter_v(&self) -> impl Iterator<Item = f32> {
        let inv_height = (self.height as f32).recip();
        (0..=self.height).map(move |y| inv_height * y as f32)
    }

    /// Iterate over normalized (0-1) `x`,`y` coordinates by `for y { for x { ... } }`
    /// returns ((x,y), (u,v))
    pub fn iter_uv_row_col(&self) -> impl Iterator<Item = ((i32, i32), (f32, f32))> {
        let base_x = self.x;
        let base_y = self.y;
        let width  = self.width;
        let inv_width  = (self.width  as f32).recip();
        let inv_height = (self.height as f32).recip();
        (0..=self.height).flat_map(move |y| {
            let v = inv_height * y as f32;
            (0..=width).map(move |x| {
                let u = inv_width * x as f32;
                ((x + base_x, y + base_y), (u, v))
            })
        })
    }

    /// Iterate over normalized (0-1) `x`,`y` coordinates by `for x { for y { ... } }`
    /// returns ((x,y), (u,v))
    pub fn iter_uv_col_row(&self) -> impl Iterator<Item = ((i32, i32), (f32, f32))> {
        let base_x = self.x;
        let base_y = self.y;
        let height = self.height;
        let inv_width  = (self.width  as f32).recip();
        let inv_height = (self.height as f32).recip();
        (0..=self.width).flat_map(move |x| {
            let u = inv_width * x as f32;
            (0..=height).map(move |y| {
                let v = inv_height * y as f32;
                ((x + base_x, y + base_y), (u, v))
            })
        })
    }

    #[inline]
    pub const fn index_of(&self, x: i32, y: i32) -> usize {
        assert!(self.x <= x && x < self.x + self.width && self.y <= y && y < self.y + self.height);
        (y - self.y) as usize * self.width as usize + (x - self.x) as usize
    }
}

impl From<IntRectangle> for ffi::Rectangle {
    #[inline]
    fn from(value: IntRectangle) -> Self {
        ffi::Rectangle {
            x:      value.x      as f32,
            y:      value.y      as f32,
            width:  value.width  as f32,
            height: value.height as f32,
        }
    }
}

pub trait RaylibIntRectangleEx: RaylibDraw {
    #[inline]
    fn begin_scissor_mode_irec(&mut self, rec: &IntRectangle) -> RaylibScissorMode<Self> where Self: RaylibScissorModeExt {
        <Self as RaylibScissorModeExt>::begin_scissor_mode(self, rec.x, rec.y, rec.width, rec.height)
    }

    #[inline]
    fn draw_rectangle_irec(&mut self, rec: &IntRectangle, color: Color) {
        <Self as RaylibDraw>::draw_rectangle(self, rec.x, rec.y, rec.width, rec.height, color);
    }
}

impl<D: RaylibDraw> RaylibIntRectangleEx for D {}

pub trait FlipRectangle {
    fn flipped(&self) -> Self;
}

impl FlipRectangle for Rectangle {
    #[inline]
    fn flipped(&self) -> Self {
        Self {
            x:  self.x,
            y: -self.y,
            width:  self.width,
            height: -self.height,
        }
    }
}
