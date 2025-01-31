use raylib::prelude::*;

pub trait MinMaxRectangle {
    fn minmax_rec(&self, other: Self) -> Rectangle;
}

impl MinMaxRectangle for Vector2 {
    fn minmax_rec(&self, other: Self) -> Rectangle {
        Rectangle {
            x: self.x.min(other.x),
            y: self.y.min(other.y),
            width:  (other.x - self.x).abs(),
            height: (other.y - self.y).abs(),
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Rect2 {
    pub xmin: f32,
    pub ymin: f32,
    pub xmax: f32,
    pub ymax: f32,
}

impl Rect2 {
    pub fn is_overlapping_point(&self, point: Vector2) -> bool {
        self.xmin <= point.x && point.x < self.xmax &&
        self.ymin <= point.y && point.y < self.ymax
    }

    pub fn is_overlapping(&self, rec: &Self) -> bool {
        self.xmin < rec.xmax && rec.xmin < self.xmax &&
        self.ymin < rec.ymax && rec.ymin < self.ymax
    }

    pub fn width(&self) -> f32 {
        self.xmax - self.xmin
    }

    pub fn height(&self) -> f32 {
        self.ymax - self.ymin
    }
}

impl From<Rectangle> for Rect2 {
    fn from(Rectangle { x, y, width, height }: Rectangle) -> Self {
        Self {
            xmin: x,
            ymin: y,
            xmax: x + width,
            ymax: y + height,
        }
    }
}

impl From<Rect2> for Rectangle {
    fn from(Rect2 { xmin, ymin, xmax, ymax }: Rect2) -> Self {
        Self {
            x: xmin,
            y: ymin,
            width:  xmax - xmin,
            height: ymax - ymin,
        }
    }
}

impl From<Rect2> for ffi::Rectangle {
    fn from(Rect2 { xmin, ymin, xmax, ymax }: Rect2) -> Self {
        Self {
            x: xmin,
            y: ymin,
            width:  xmax - xmin,
            height: ymax - ymin,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct IntRect2 {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

impl From<IntRect2> for Rectangle {
    fn from(value: IntRect2) -> Self {
        Rectangle {
            x:      value.x      as f32,
            y:      value.y      as f32,
            width:  value.width  as f32,
            height: value.height as f32,
        }
    }
}

impl IntRect2 {
    pub fn offset(&mut self, x: i32, y: i32) {
        self.x += x;
        self.y += y;
    }

    pub fn with_offset(&self, x: i32, y: i32) -> Self {
        Self {
            x: self.x + x,
            y: self.y + y,
            width:  self.width,
            height: self.height,
        }
    }

    pub fn intersect(&self, container: &Self) -> Self {
        let x = self.x.max(container.x);
        let y = self.y.max(container.y);
        let width  = (self.x + self.width ).min(container.x + container.width ) - x;
        let height = (self.y + self.height).min(container.y + container.height) - y;
        Self { x, y, width, height }
    }

    pub const fn left(&self) -> i32 {
        self.x
    }
    pub const fn right(&self) -> i32 {
        self.x + self.width
    }
    pub const fn top(&self) -> i32 {
        self.y
    }
    pub const fn bottom(&self) -> i32 {
        self.y + self.height
    }

    /// Iterate over `x` coordinates
    pub fn iter_x(&self) -> impl Iterator<Item = i32> {
        self.x..=(self.x + self.width)
    }

    /// Iterate over `y` coordinates
    pub fn iter_y(&self) -> impl Iterator<Item = i32> {
        self.y..=(self.y + self.height)
    }

    /// Iterate over `x`,`y` coordinates by `for y { for x { ... } }`
    pub fn iter_xy_row_col(&self) -> impl Iterator<Item = (i32, i32)> {
        let x = self.x;
        let width = self.width;
        (self.y..=(self.y + self.height))
            .flat_map(move |y| (x..=x + width)
                .map(move |x| (x, y)))
    }

    /// Iterate over `x`,`y` coordinates by `for x { for y { ... } }`
    pub fn iter_xy_col_row(&self) -> impl Iterator<Item = (i32, i32)> {
        let y = self.y;
        let height = self.height;
        (self.x..=(self.x + self.width))
            .flat_map(move |x| (y..=y + height)
                .map(move |y| (x, y)))
    }

    /// Iterate over normalized `x` coordinates (`self.x` = 0, `self.x + self.width` = 1)
    pub fn iter_u(&self) -> impl Iterator<Item = f32> {
        let inv_width = (self.width as f32).recip();
        (0..=self.width).map(move |x| inv_width * x as f32)
    }

    /// Iterate over normalized `y` coordinates (`self.y` = 0, `self.y + self.height` = 1)
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

    pub const fn index_of(&self, x: i32, y: i32) -> usize {
        assert!(self.x <= x && x < self.x + self.width && self.y <= y && y < self.y + self.height);
        (y - self.y) as usize * self.width as usize + (x - self.x) as usize
    }
}

impl From<IntRect2> for ffi::Rectangle {
    fn from(value: IntRect2) -> Self {
        ffi::Rectangle {
            x:      value.x      as f32,
            y:      value.y      as f32,
            width:  value.width  as f32,
            height: value.height as f32,
        }
    }
}
