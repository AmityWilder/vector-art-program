use amymath::prelude::*;

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
    Relative(i32),
    /// Position on screen
    Absolute(i32),
}

#[derive(Clone, Copy, Debug)]
pub enum Axis {
    Position(Align, Align),
    Distance(i32, Option<(Side, Align)>),
}

impl Axis {
    pub fn to_range(&self, container: (i32, i32)) -> (i32, i32) {
        let (c_start, c_end) = container;
        let calculate_start = |x: &Align| -> i32 {
            match x {
                Align::Relative(offset) => c_start + *offset,
                Align::Absolute(start) => *start,
            }
        };
        let calculate_end = |x: &Align| -> i32 {
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
                    let (center, radius) = ((c_start + c_end) / 2, len / 2);
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

    pub fn rect(&self, container: &IRect2) -> IRect2 {
        let (xmin, xmax) = self.h.to_range((container.min.x, container.max.x));
        let (ymin, ymax) = self.v.to_range((container.min.y, container.max.y));
        IRect2 { min: IVector2 { x: xmin, y: ymin }, max: IVector2 { x: xmax, y: ymax } }
    }
}

pub struct RectBuilder {
    top:    Option<Align>,
    bottom: Option<Align>,
    left:   Option<Align>,
    right:  Option<Align>,
    width:  Option<i32>,
    height: Option<i32>,
}

impl RectBuilder {
    pub const fn top_at(mut self, value: i32) -> Self {
        self.top = Some(Align::Absolute(value)); self
    }
    pub const fn from_top(mut self, value: i32) -> Self {
        self.top = Some(Align::Relative(value)); self
    }

    pub const fn bottom_at(mut self, value: i32) -> Self {
        self.bottom = Some(Align::Absolute(value)); self
    }
    pub const fn from_bottom(mut self, value: i32) -> Self {
        self.bottom = Some(Align::Relative(value)); self
    }

    pub const fn left_at(mut self, value: i32) -> Self {
        self.left = Some(Align::Absolute(value)); self
    }
    pub const fn from_left(mut self, value: i32) -> Self {
        self.left = Some(Align::Relative(value)); self
    }

    pub const fn right_at(mut self, value: i32) -> Self {
        self.right = Some(Align::Absolute(value)); self
    }
    pub const fn from_right(mut self, value: i32) -> Self {
        self.right = Some(Align::Relative(value)); self
    }

    pub const fn with_width(mut self, value: i32) -> Self {
        self.width = Some(value); self
    }
    pub const fn with_height(mut self, value: i32) -> Self {
        self.height = Some(value); self
    }

    const fn make_axis(start: Option<Align>, end: Option<Align>, size: Option<i32>) -> Axis {
        match size {
            Some(size) => Axis::Distance(size, match (start, end) {
                (None, None) => None,
                (Some(start), None) => Some((Side::Start, start)),
                (None, Some(end)) => Some((Side::End, end)),
                (Some(_), Some(_)) => panic!("invalid axis: start, end, and size should not all be defined at once"),
            }),
            None => Axis::Position(
                if let Some(x) = start { x } else { Align::Relative(0) },
                if let Some(x) = end   { x } else { Align::Relative(0) },
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
