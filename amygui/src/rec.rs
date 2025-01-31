use amymath::{rec::Rect2, Arithmetic};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Side {
    Start,
    End,
}

#[allow(non_upper_case_globals)]
impl Side {
    /// Horizontal start
    pub const Left: Self = Self::Start;
    /// Horizontal end
    pub const Right: Self = Self::End;

    /// Vertical start
    pub const Top: Self = Self::Start;
    /// Vertical end
    pub const Bottom: Self = Self::End;
}

#[derive(Clone, Copy, Debug)]
pub enum Align {
    /// Relative to container
    Relative(f32),
    /// Position on screen
    Absolute(f32),
}

#[derive(Clone, Copy, Debug)]
pub enum Axis {
    Position(Align, Align),
    Distance(f32, Option<(Side, Align)>),
}

impl Axis {
    pub fn to_range(&self, container: (f32, f32)) -> (f32, f32) {
        let (c_start, c_end) = container;
        let calculate_start = |x: &Align| -> f32 {
            match x {
                Align::Relative(offset) => c_start + *offset,
                Align::Absolute(start) => *start,
            }
        };
        let calculate_end = |x: &Align| -> f32 {
            match x {
                Align::Relative(offset) => c_end - *offset,
                Align::Absolute(end) => *end,
            }
        };
        match self {
            Axis::Position(start, end) => (
                calculate_start(start),
                calculate_end(end),
            ),
            Axis::Distance(len, anchor) => match anchor {
                Some((side, align)) => match side {
                    Side::Start => {
                        let start = calculate_start(align);
                        (start, start + *len)
                    }
                    Side::End => {
                        let end = calculate_end(align);
                        (end - *len, end)
                    }
                },
                None => {
                    let (center, radius) = ((c_start + c_end).half(), len.half());
                    (center - radius, center + radius)
                }
            },
        }
    }
}

#[derive(Debug)]
pub struct UIRect {
    pub h: Axis,
    pub v: Axis,
}

impl UIRect {
    pub const fn init() -> RectBuilder {
        RectBuilder {
            top:    None,
            bottom: None,
            left:   None,
            right:  None,
            width:  None,
            height: None,
        }
    }

    pub fn rect(&self, container: &Rect2) -> Rect2 {
        let (xmin, xmax) = self.h.to_range((container.xmin, container.xmax));
        let (ymin, ymax) = self.v.to_range((container.ymin, container.ymax));
        Rect2 { xmin, ymin, xmax, ymax }
    }
}

pub struct RectBuilder {
    top:    Option<Align>,
    bottom: Option<Align>,
    left:   Option<Align>,
    right:  Option<Align>,
    width:  Option<f32>,
    height: Option<f32>,
}

impl RectBuilder {
    pub const fn top_at(mut self, value: f32) -> Self {
        self.top = Some(Align::Absolute(value)); self
    }
    pub const fn from_top(mut self, value: f32) -> Self {
        self.top = Some(Align::Relative(value)); self
    }

    pub const fn bottom_at(mut self, value: f32) -> Self {
        self.bottom = Some(Align::Absolute(value)); self
    }
    pub const fn from_bottom(mut self, value: f32) -> Self {
        self.bottom = Some(Align::Relative(value)); self
    }

    pub const fn left_at(mut self, value: f32) -> Self {
        self.left = Some(Align::Absolute(value)); self
    }
    pub const fn from_left(mut self, value: f32) -> Self {
        self.left = Some(Align::Relative(value)); self
    }

    pub const fn right_at(mut self, value: f32) -> Self {
        self.right = Some(Align::Absolute(value)); self
    }
    pub const fn from_right(mut self, value: f32) -> Self {
        self.right = Some(Align::Relative(value)); self
    }

    pub const fn with_width(mut self, value: f32) -> Self {
        self.width = Some(value); self
    }
    pub const fn with_height(mut self, value: f32) -> Self {
        self.height = Some(value); self
    }

    const fn make_axis(start: Option<Align>, end: Option<Align>, size: Option<f32>) -> Axis {
        match size {
            Some(size) => Axis::Distance(size, match (start, end) {
                (None, None) => None,
                (Some(start), None) => Some((Side::Start, start)),
                (None, Some(end)) => Some((Side::End, end)),
                (Some(_), Some(_)) => panic!("invalid axis: start, end, and size should not all be defined at once"),
            }),
            None => Axis::Position(
                if let Some(x) = start { x } else { Align::Relative(0.0) },
                if let Some(x) = end   { x } else { Align::Relative(0.0) },
            ),
        }
    }

    pub const fn build(self) -> UIRect {
        UIRect {
            h: Self::make_axis(self.left, self.right, self.width),
            v: Self::make_axis(self.top, self.bottom, self.height),
        }
    }
}
