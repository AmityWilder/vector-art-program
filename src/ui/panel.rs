use raylib::prelude::*;

#[derive(Clone, Copy)]
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

pub enum UIAxis {
    /// Fill container
    Fill,

    /// Fixed start, Fixed end
    FsFe { start: f32, end: f32 },
    /// Fixed start, Relative end
    FsRe { start: f32, end_rel: f32 },
    /// Relative start, Fixed end
    RsFe { start_rel: f32, end: f32 },
    /// Relative start, Relative end
    RsRe { start_rel: f32, end_rel: f32 },


    /// Fixed start, Bounded length
    FsBl { start: f32, size_max: f32 },
    /// Fixed start, Unbounded length
    FsUl { start: f32 },

    /// Relative start, Bounded length
    RsBl { start_rel: f32, size_max: f32 },
    /// Relative start, Unbounded length
    RsUl { start_rel: f32 },


    /// Fixed end, Bounded length
    FeBl { end: f32, size_max: f32 },
    /// Fixed end, Unbounded length
    FeUl { end: f32 },

    /// Relative end, Bounded length
    ReBl { end_rel: f32, size_max: f32 },
    /// Relative end, Unbounded length
    ReUl { end_rel: f32 },
}

impl UIAxis {
    pub fn to_range(&self, container_start: f32, container_end: f32) -> (f32, f32) {
        let start = match self {
            | UIAxis::FsFe { start, .. }
            | UIAxis::FsRe { start, .. }
            | UIAxis::FsBl { start, .. }
            | UIAxis::FsUl { start }
                => *start,

            | UIAxis::RsFe { start_rel, .. }
            | UIAxis::RsRe { start_rel, .. }
            | UIAxis::RsBl { start_rel, .. }
            | UIAxis::RsUl { start_rel }
                => container_start + start_rel,

            _ => container_start,
        };

        let end = match self {
            | UIAxis::FsFe { end, .. }
            | UIAxis::RsFe { end, .. }
            | UIAxis::FeBl { end, .. }
            | UIAxis::FeUl { end }
                => *end,

            | UIAxis::FsRe { end_rel, .. }
            | UIAxis::RsRe { end_rel, .. }
            | UIAxis::ReBl { end_rel, .. }
            | UIAxis::ReUl { end_rel }
                => container_end - end_rel,

            _ => container_end,
        };

        match self {
            | UIAxis::Fill
            | UIAxis::FsFe { .. }
            | UIAxis::FsRe { .. }
            | UIAxis::RsFe { .. }
            | UIAxis::RsRe { .. }
            | UIAxis::FsUl { .. }
            | UIAxis::RsUl { .. }
            | UIAxis::FeUl { .. }
            | UIAxis::ReUl { .. }
                => (start, end),

            | UIAxis::FsBl { size_max, .. }
            | UIAxis::RsBl { size_max, .. }
                => (start, start + (end - start).min(*size_max)),

            | UIAxis::FeBl { size_max, .. }
            | UIAxis::ReBl { size_max, .. }
                => (end - (end - start).min(*size_max), end),
        }
    }
}

pub struct UIBox {
    pub h: UIAxis,
    pub v: UIAxis,
}

impl UIBox {
    pub const fn init() -> UIBoxBuilder {
        UIBoxBuilder::new()
    }

    pub fn rect(&self, container: &Rect2) -> Rect2 {
        let (xmin, xmax) = self.h.to_range(container.xmin, container.xmax);
        let (ymin, ymax) = self.v.to_range(container.ymin, container.ymax);
        Rect2 { xmin, xmax, ymin, ymax }
    }
}

pub struct UIBoxBuilder {
    left:   Option<(f32, bool)>,
    right:  Option<(f32, bool)>,
    top:    Option<(f32, bool)>,
    bottom: Option<(f32, bool)>,
    width:  Option<f32>,
    height: Option<f32>,
}

impl UIBoxBuilder {
    pub const fn new() -> Self {
        Self {
            left:   None,
            right:  None,
            top:    None,
            bottom: None,
            width:  None,
            height: None,
        }
    }

    pub const fn from_left(mut self, value: f32) -> Self {
        self.left = Some((value, true));
        self
    }

    pub const fn from_right(mut self, value: f32) -> Self {
        self.right = Some((value, true));
        self
    }

    pub const fn from_top(mut self, value: f32) -> Self {
        self.top = Some((value, true));
        self
    }

    pub const fn from_bottom(mut self, value: f32) -> Self {
        self.bottom = Some((value, true));
        self
    }

    pub const fn xmin(mut self, value: f32) -> Self {
        self.left = Some((value, false));
        self
    }

    pub const fn xmax(mut self, value: f32) -> Self {
        self.right = Some((value, false));
        self
    }

    pub const fn ymin(mut self, value: f32) -> Self {
        self.top = Some((value, false));
        self
    }

    pub const fn ymax(mut self, value: f32) -> Self {
        self.bottom = Some((value, false));
        self
    }

    pub const fn width(mut self, value: f32) -> Self {
        self.width = Some(value);
        self
    }

    pub const fn height(mut self, value: f32) -> Self {
        self.height = Some(value);
        self
    }

    pub const fn build(self) -> UIBox {
        const fn make_axis(start: Option<(f32, bool)>, end: Option<(f32, bool)>, size: Option<f32>) -> UIAxis {
            match (start, end, size) {
                (None, None, None) => UIAxis::Fill,

                (Some((start,    false)), Some((end,    false)), None) => UIAxis::FsFe { start, end },
                (Some((start,    false)), Some((end_rel, true)), None) => UIAxis::FsRe { start, end_rel },
                (Some((start_rel, true)), Some((end,    false)), None) => UIAxis::RsFe { start_rel, end },
                (Some((start_rel, true)), Some((end_rel, true)), None) => UIAxis::RsRe { start_rel, end_rel },

                (Some((start,    false)), None, Some(size_max)) => UIAxis::FsBl { start, size_max },
                (Some((start,    false)), None, None          ) => UIAxis::FsUl { start },
                (Some((start_rel, true)), None, Some(size_max)) => UIAxis::RsBl { start_rel, size_max },
                (Some((start_rel, true)), None, None          ) => UIAxis::RsUl { start_rel },

                (None, Some((end,    false)), Some(size_max)) => UIAxis::FeBl { end, size_max },
                (None, Some((end,    false)), None          ) => UIAxis::FeUl { end },
                (None, Some((end_rel, true)), Some(size_max)) => UIAxis::ReBl { end_rel, size_max },
                (None, Some((end_rel, true)), None          ) => UIAxis::ReUl { end_rel },

                _ => panic!("invalid UI box"),
            }
        }

        let Self { left, right, top, bottom, width, height } = self;
        UIBox {
            h: make_axis(left, right, width),
            v: make_axis(top, bottom, height),
        }
    }
}

pub struct Panel {
    pub is_hidden: bool,
    pub rec: UIBox,
    pub rec_cache: Rect2,
    pub background: Color,
}

impl Panel {
    pub fn new(container: &Rect2, rec: UIBox, background: Color) -> Self {
        Self {
            is_hidden: false,
            background,
            rec_cache: rec.rect(&container),
            rec,
        }
    }

    pub fn update_rec(&mut self, container: &Rect2) {
        self.rec_cache = self.rec.rect(&container);
    }

    pub fn is_overlapping_point(&self, point: Vector2) -> bool {
        self.rec_cache.is_overlapping_point(point)
    }
}
