use std::collections::VecDeque;
use raylib::prelude::*;
use amymath::prelude::{*, Vector2};
use crate::{
    bezier::cubic::Cubic, path_point::{Ctrl, PPPart, PathPoint}
};

#[derive(Debug, Clone)]
pub enum WidthProfile {
    Constant(f32, f32),
    Variable(Curve),
}

impl Default for WidthProfile {
    fn default() -> Self {
        Self::Constant(5.0, 5.0)
    }
}

impl WidthProfile {
    pub const fn new_constant(thick1: f32, thick2: f32) -> Self {
        Self::Constant(thick1, thick2)
    }

    pub fn new_variable(thick1: f32, thick2: f32) -> Self {
        Self::Variable(Curve {
            points: VecDeque::from([
                PathPoint {
                    p: Vector2::new(thick1, thick2),
                    c: None,
                }
            ]),
            is_closed: false,
        })
    }

    pub fn extents_at(&self, t: f32) -> (f32, f32) {
        match self {
            &WidthProfile::Constant(a, b) => (a, b),
            WidthProfile::Variable(curve) => {
                let Vector2 { x, y } = curve.position_at(t).expect("width profile should not be empty");
                (x, y)
            },
        }
    }

    pub fn extents_min(&self) -> (f32, f32) {
        match self {
            &WidthProfile::Constant(a, b) => (a, b),
            WidthProfile::Variable(curve) => {
                let bounds = curve.bounds().expect("width profile should not be empty");
                (bounds.min.x, bounds.min.y)
            },
        }
    }

    pub fn extents_max(&self) -> (f32, f32) {
        match self {
            &WidthProfile::Constant(a, b) => (a, b),

            WidthProfile::Variable(curve) => {
                let bounds = curve.bounds().expect("width profile should not be empty");
                (bounds.max.x, bounds.max.y)
            },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PathPointIdx {
    pub point: usize,
    pub part: PPPart,
}

impl PathPointIdx {
    pub const fn new(point: usize, part: PPPart) -> Self {
        Self { point, part }
    }

    pub const fn new_anchor(point: usize) -> Self {
        Self { point, part: PPPart::Anchor }
    }

    pub const fn new_ctrl(point: usize, side: Ctrl) -> Self {
        Self { point, part: PPPart::Ctrl(side) }
    }
}

impl Ord for PathPointIdx {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let broad = self.point.cmp(&other.point);
        if broad.is_eq() {
            self.part.cmp(&other.part)
        } else {
            broad
        }
    }
}

impl PartialOrd for PathPointIdx {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone)]
pub struct Curve {
    pub points: VecDeque<PathPoint>,
    pub is_closed: bool,
}

impl Default for Curve {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl Curve {
    pub const fn new() -> Self {
        Self {
            points: VecDeque::new(),
            is_closed: false,
        }
    }

    #[inline]
    pub fn num_slices(&self) -> usize {
        self.points.len().saturating_sub(1)
    }

    #[inline]
    pub fn slice(&self, start_index: usize) -> Option<Cubic> {
        if let Some((pp2, pp1)) = start_index.checked_add(1)
            .and_then(|end_index| self.points.get(end_index))
            .map(|pp2| (pp2, self.points.get(start_index).expect("existence of [i+1] should guarantee existence of [i]")))
        {
            let ((_, p1, c1_out), (c2_in, p2, _)) = (pp1.calculate(), pp2.calculate());
            return Some(Cubic::new(p1, c1_out, c2_in, p2))
        }
        None
    }

    #[inline]
    pub fn position_at(&self, t: f32) -> Option<Vector2> {
        if self.points.is_empty() || t < 0.0 || 1.0 < t { return None; }
        if self.points.len() == 1 {
            Some(self.points[0].p)
        } else if t == 0.0 {
            Some(self.slice(0).expect("should have at least one point in this branch").position_at(0.0))
        } else if t == 1.0 {
            Some(self.slice(self.num_slices() - 1).expect("should have at least one point in this branch").position_at(1.0))
        } else {
            let (slice_idx, t) = {
                let t_major = t * self.num_slices() as f32;
                let trunc = (t_major).floor();
                (trunc as usize, t_major - trunc)
            };
            Some(self.slice(slice_idx).unwrap().position_at(t))
        }
    }

    #[inline]
    pub fn max_bounds(&self) -> Option<Rect2> {
        self.slices()
            .map(|bez| bez.max_bounds())
            .reduce(|rec, b| rec.union(b))
    }

    /// Calculate the bounding box of the entire curve
    ///
    /// Returns [`None`] if the curve is empty
    ///
    /// ## Note regarding current implementation
    /// Calls [`Curve::slices`] (which calculates every path point)
    /// and [`Cubic::bounds`] (which solves the quadratic equation) on each.
    #[inline]
    pub fn bounds(&self) -> Option<Rect2> {
        let mut bez_iter = self.slices();
        if let Some(bez) = bez_iter.next() {
            let mut rec = bez.bounds();
            for bez in bez_iter {
                if !rec.contains(&bez.max_bounds()) {
                    rec = rec.union(bez.bounds());
                }
            }
            Some(rec)
        } else { None }
    }

    #[inline]
    pub fn calculate(&self) -> Calculate<impl Iterator<Item = &'_ PathPoint>> {
        Calculate::new(self.points.iter(), self.is_closed)
    }

    #[inline]
    pub fn slices(&self) -> Slices<impl Iterator<Item = &'_ PathPoint>> {
        Slices::new(self.calculate())
    }
}

impl Curve {
    pub fn draw_lines(&self, d: &mut impl RaylibDraw, strips_per_bez: usize, color: Color) {
        if self.points.len() >= 2 {
            let mut d = d.begin_rlgl();
            let mut d = d.rl_begin_lines();
            d.rl_color4ub(color.r, color.g, color.b, color.a);

            let mut prev: Option<Vector2> = None;
            for bez in self.slices() {
                for i in 0..strips_per_bez {
                    let t = i as f32 / (strips_per_bez - 1) as f32;
                    let p = bez.position_at(t);
                    if let Some(prev) = prev.as_mut() {
                        d.rl_vertex2f(prev.x, prev.y);
                        d.rl_vertex2f(p.x, p.y);
                        *prev = p;
                    } else {
                        prev = Some(p);
                    }
                }
            }
        }
    }

    pub fn draw_stroke(&self, d: &mut impl RaylibDraw, strips_per_slice: usize, thick: &WidthProfile, color: Color) {
        if self.points.is_empty() {
        } else if self.points.len() == 1 {
            let extents = thick.extents_max(); // largest thickness would overtake smaller thicknesses
            d.draw_circle_v(self.points[0].p, (extents.0 + extents.1) * 0.5, color);
        } else {
            let num_points = strips_per_slice * 2 * self.points.len();
            if num_points < 3 { return; }
            let total_strips = self.points.len() * strips_per_slice;

            for (t_vel, t, t_full, bez) in [
                (0.001, 0.0, 0.0, self.slice(0).expect("should have at least 2 points in this branch")),
                (0.999, 1.0, 1.0, self.slice(self.num_slices() - 1).expect("should have at least 2 points in this branch"))
            ] {
                let extents = thick.extents_at(t_full);
                let v = bez.velocity_at(t_vel);
                let p = bez.position_at(t);
                if v.magnitude_sqr() >= f32::EPSILON {
                    let tangent = v.normalized();
                    let (normal_cw, normal_cc) = (tangent.rotate_cw(), tangent.rotate_cc());
                    let p1 = p + normal_cc * extents.0;
                    let p2 = p + normal_cw * extents.1;
                    d.draw_circle_v(p1.midpoint_v(p2), (extents.0 + extents.1) * 0.5, color);
                } else {
                    d.draw_circle_v(p, (extents.0 + extents.1) * 0.5, color);
                }
            }

            #[cfg(not(feature = "debug_stroke"))] let mut d = d.begin_rlgl();
            #[cfg(not(feature = "debug_stroke"))] let mut d = d.rl_begin_triangles();
            #[cfg(not(feature = "debug_stroke"))] d.rl_color4ub(color.r, color.g, color.b, color.a);

            let mut past_points: Option<[Vector2; 2]> = None;
            for (n, bez) in self.slices().enumerate() {
                let row = n * strips_per_slice;
                for i in 0..strips_per_slice {
                    let t = i as f32 / (strips_per_slice - 1) as f32;
                    let v = bez.velocity_at(t);
                    if v.magnitude_sqr() < f32::EPSILON { continue; }
                    let tangent = v.normalized();
                    let (normal_cw, normal_cc) = (tangent.rotate_cw(), tangent.rotate_cc());
                    let p = bez.position_at(t);
                    let t_full = (row + i) as f32 / total_strips as f32;
                    let extents = thick.extents_at(t_full);
                    let p1 = p + normal_cc * extents.0;
                    let p2 = p + normal_cw * extents.1;
                    if let Some(past_points) = &mut past_points {
                        let [p3, p4] = *past_points;
                        #[cfg(not(feature = "debug_stroke"))] {
                            d.rl_vertex2f(p1.x, p1.y);
                            d.rl_vertex2f(p3.x, p3.y);
                            d.rl_vertex2f(p4.x, p4.y);

                            d.rl_vertex2f(p2.x, p2.y);
                            d.rl_vertex2f(p1.x, p1.y);
                            d.rl_vertex2f(p4.x, p4.y);
                        } #[cfg(feature = "debug_stroke")] {
                            d.draw_line_strip(&[p1, p3, p4], Color::RED);
                            d.draw_line_strip(&[p2, p1, p4], Color::BLUE);
                        }

                        past_points[0] = p1;
                        past_points[1] = p2;
                    } else {
                        past_points = Some([p1, p2]);
                    }
                }
            }
        }
    }

    pub fn draw_fill(&self, d: &mut impl RaylibDraw, segments_per_slice: usize, color: Color) {
        if self.points.is_empty() { return; }
        let num_points = segments_per_slice * self.points.len();
        if num_points < 3 { return; }

        #[cfg(not(feature = "debug_fill"))] let mut d = d.begin_rlgl();
        #[cfg(not(feature = "debug_fill"))] let mut d = d.rl_begin_triangles();
        #[cfg(not(feature = "debug_fill"))] d.rl_color4ub(color.r, color.g, color.b, color.a);

        let mut first_point: Option<Vector2> = None;
        let mut prev_point: Option<Vector2> = None;
        for bez in self.slices() {
            for i in 0..segments_per_slice {
                let t = i as f32 / (segments_per_slice - 1) as f32;
                let p = bez.position_at(t);
                if let Some((first_point, prev_point)) = first_point.as_ref().zip(prev_point.as_mut()) {
                    #[cfg(not(feature = "debug_fill"))] {
                        d.rl_vertex2f(first_point.x, first_point.y);
                        d.rl_vertex2f(prev_point.x, prev_point.y);
                        d.rl_vertex2f(p.x, p.y);
                    } #[cfg(feature = "debug_fill")] {
                        d.draw_line_strip(&[*first_point, *prev_point, p], color);
                    }
                } else if prev_point.is_none() {
                    first_point = Some(p);
                }
                prev_point = Some(p);
            }
        }
    }
}

pub struct Calculate<I> {
    iter: I,
    wrapped: Option<Option<(Vector2, Vector2, Vector2)>>,
}

impl<I> Calculate<I> {
    const fn new(iter: I, is_closed: bool) -> Self {
        Self {
            iter,
            wrapped: if is_closed { Some(None) } else { None }, // fun fact: `then_some()` isn't const :/
        }
    }
}

impl<'a, I: Iterator<Item = &'a PathPoint>> Iterator for Calculate<I> {
    type Item = (Vector2, Vector2, Vector2);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(next) = self.iter.next() {
            let calculated = next.calculate();
            if let Some(inner @ None) = self.wrapped.as_mut() {
                *inner = Some(calculated);
            }
            Some(calculated)
        } else if let Some(inner @ Some(_)) = self.wrapped.as_mut() {
            inner.take()
        } else {
            None
        }
    }
}

pub struct Slices<I> {
    prev: Option<(Vector2, Vector2, Vector2)>,
    iter: Calculate<I>,
}

impl<I> Slices<I> {
    const fn new(iter: Calculate<I>) -> Self {
        Self { prev: None, iter }
    }
}

impl<'a, I: Iterator<Item = &'a PathPoint>> Iterator for Slices<I> {
    type Item = Cubic;

    fn next(&mut self) -> Option<Self::Item> {
        let mut next = self.iter.next();
        if self.prev.is_none() && next.is_some() {
            self.prev = next;
            next = self.iter.next();
        }
        if let (Some(pp1), Some(pp2)) = (self.prev, next) {
            let ((_, p1, c1_out), (c2_in, p2, _)) = (pp1, pp2);
            self.prev = Some(pp2);
            Some(Cubic::new(p1, c1_out, c2_in, p2))
        } else {
            None
        }
    }
}
