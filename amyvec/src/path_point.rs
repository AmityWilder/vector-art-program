use raylib::prelude::*;
use amymath::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Ctrl {
    In,
    Out,
}

impl Ctrl {
    #[inline]
    pub const fn opposite(self) -> Self {
        match self {
            Ctrl::In  => Ctrl::Out,
            Ctrl::Out => Ctrl::In,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PPPart {
    Ctrl(Ctrl),
    Anchor,
}

impl Ord for PPPart {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (PPPart::Anchor, PPPart::Anchor) => std::cmp::Ordering::Equal,

            (PPPart::Ctrl(side1), PPPart::Ctrl(side2)) => side1.cmp(side2),

            (PPPart::Ctrl(Ctrl::In ), PPPart::Anchor) | (PPPart::Anchor, PPPart::Ctrl(Ctrl::Out)) => std::cmp::Ordering::Less,
            (PPPart::Ctrl(Ctrl::Out), PPPart::Anchor) | (PPPart::Anchor, PPPart::Ctrl(Ctrl::In )) => std::cmp::Ordering::Greater,
        }
    }
}

impl PartialOrd for PPPart {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Ctrl2 {
    // corner: G0 continuity

    // todo: add G2 continuity option

    /// Reflection of `c1` across `p` \
    /// G1 continuity
    Reflect,

    /// Reflection of `c1` across `p`, with a specific length \
    /// G1 continuity
    Mirror(f32),

    /// A transformation of the relative vector from `p` to `c1` \
    /// G0 continuity
    Transformed(Matrix2x2),

    /// A directly given position \
    /// G0 continuity
    Exact(Vector2),
}
use Ctrl2::*;

impl Ctrl2 {
    #[inline]
    pub const fn is_reflect(&self) -> bool {
        matches!(self, Reflect)
    }

    #[inline]
    pub const fn is_mirror(&self) -> bool {
        matches!(self, Mirror(_))
    }

    #[inline]
    pub const fn is_exact(&self) -> bool {
        matches!(self, Exact(_))
    }

    #[inline]
    pub fn calculate(&self, p: Vector2, c1: Vector2) -> Vector2 {
        match self {
            &Exact(c2) => c2,
            &Reflect => c1.reflected_over(p),
            &Mirror(s2) => c1.reflected_to(p, s2),
            Transformed(m2) => p + m2 * (c1 - p),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Ctrl1 {
    pub c1: (Ctrl, Vector2),
    pub c2: Option<Ctrl2>,
}

#[derive(Debug, Clone, Copy)]
pub struct PathPoint {
    pub p: Vector2,
    pub c: Option<Ctrl1>,
}

impl PathPoint {
    #[inline]
    pub const fn is_c1_corner(&self) -> bool {
        self.c.is_none()
    }

    #[inline]
    pub const fn is_c2_corner(&self) -> bool {
        self.c2().is_none()
    }

    #[inline]
    pub const fn c2(&self) -> Option<&Ctrl2> {
        match &self.c {
            Some(Ctrl1 { c2, .. }) => c2.as_ref(),
            _ => None,
        }
    }

    #[inline]
    pub const fn c2_mut(&mut self) -> Option<&mut Ctrl2> {
        match &mut self.c {
            Some(Ctrl1 { c2, .. }) => c2.as_mut(),
            _ => None,
        }
    }

    #[inline]
    pub fn calculate(&self) -> (Vector2, Vector2, Vector2) {
        let (c_in, c_out) = match &self.c {
            Some(Ctrl1 { c1: (c1_side, c1), c2 }) => {
                // debug_assert_ne!(&self.p, c1);
                let c_opp = c2.map_or(self.p, |c2| c2.calculate(self.p, *c1));
                match c1_side {
                    Ctrl::In  => (*c1, c_opp),
                    Ctrl::Out => (c_opp, *c1),
                }
            }
            None => (self.p, self.p),
        };

        (c_in, self.p, c_out)
    }

    #[inline]
    pub fn ctrl(&self, side: Ctrl) -> Option<Ctrl2> {
        self.c.as_ref().and_then(|Ctrl1 { c1: (c1_side, c1), c2 }|
            if c1_side == &side { Some(Exact(*c1)) } else { *c2 }
        )
    }

    #[inline]
    pub fn ctrl_pov(&self, side: Ctrl) -> (Option<Ctrl2>, Option<Ctrl2>) {
        if let Some(Ctrl1 { c1: (c1_side, c1), c2 }) = self.c.as_ref() {
            let (c1, c2) = (Some(Exact(*c1)), *c2);
            if c1_side == &side { (c1, c2) } else { (c2, c1) }
        } else {
            (None, None)
        }
    }

    /// Translate the point and controls while keeping the controls' relative positions
    #[inline]
    pub fn move_point(&mut self, delta: Vector2) {
        self.p += delta;
        if let Some(Ctrl1 { c1: (_, c1), c2 }) = self.c.as_mut() {
            *c1 += delta;
            if let Some(Exact(c2)) = c2.as_mut() {
                *c2 += delta;
            }
        }
    }

    /// Move the point and controls while keeping the controls' relative positions
    #[inline]
    pub fn set_point(&mut self, p: Vector2) {
        self.move_point(p - self.p);
    }

    pub fn _transform(&mut self, _mat: Matrix2x2) {
        todo!("im not updating this anymore until it gets used")
    }
}
