use std::collections::{vec_deque, VecDeque};
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
    /// Cannot be closed with fewer than 2 points
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
        self.manifold_len().saturating_sub(1)
    }

    /// points.len (+1 if `is_closed` is true)
    #[inline]
    pub fn manifold_len(&self) -> usize {
        let len = self.points.len();
        if len > 1 && self.is_closed { len + 1 } else { len }
    }

    #[inline]
    pub fn slice(&self, start_index: usize) -> Option<Cubic> {
        if let Some((pp2, pp1)) = start_index.checked_add(1)
            .and_then(|end_index| self.points.get(if self.is_closed && end_index == self.points.len() { 0 } else { end_index }))
            .map(|pp2| (pp2, self.points.get(start_index).expect("existence of [i+1] should guarantee existence of [i]")))
        {
            let ((_, p1, c1_out), (c2_in, p2, _)) = (pp1.calculate(), pp2.calculate());
            return Some(Cubic::new(p1, c1_out, c2_in, p2))
        }
        None
    }

    pub fn fn_at<T, F: Fn(Cubic, f32) -> T, G: Fn(&PathPoint) -> T>(&self, t: f32, f: F, if_orphan: G) -> Option<T> {
        if self.points.is_empty() || t < 0.0 || 1.0 < t {
            None
        } else if self.points.len() == 1 {
            Some(if_orphan(&self.points[0]))
        } else {
            let (slice_idx, t) = {
                let t_major = t * self.num_slices() as f32;
                let t_index = (t_major as usize).min(self.num_slices() - 1);
                let t_minor = t_major - t_index as f32;
                (t_index, t_minor)
            };
            Some(f(self.slice(slice_idx).unwrap(), t))
        }
    }

    #[inline]
    pub fn position_at(&self, t: f32) -> Option<Vector2> {
        self.fn_at(t, |bez, t| bez.position_at(t), |pp| pp.p)
    }

    #[inline]
    pub fn velocity_at(&self, t: f32) -> Option<Vector2> {
        self.fn_at(t, |bez, t| bez.velocity_at(t), |_| Vector2::ZERO)
    }

    #[inline]
    pub fn acceleration_at(&self, t: f32) -> Option<Vector2> {
        self.fn_at(t, |bez, t| bez.acceleration_at(t), |_| Vector2::ZERO)
    }

    #[inline]
    pub fn curvature_at(&self, t: f32) -> Option<f32> {
        self.fn_at(t, |bez, t| bez.curvature_at(t), |_| 0.0)
    }

    #[inline]
    pub fn max_bounds(&self) -> Option<Rect2> {
        if self.points.len() == 1 {
            let p = self.points[0].p;
            Some(Rect2::new(p, p))
        } else {
            self.slices()
                .map(|bez| bez.max_bounds())
                .reduce(|rec, b| rec.union(b))
        }
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
        if self.points.len() == 1 {
            let p = self.points[0].p;
            return Some(Rect2::new(p, p));
        } else {
            let mut bez_iter = self.slices();
            if let Some(bez) = bez_iter.next() {
                let mut rec = bez.bounds();
                for bez in bez_iter {
                    if !rec.contains(&bez.max_bounds()) {
                        rec = rec.union(bez.bounds());
                    }
                }
                return Some(rec);
            }
        }
        None
    }

    #[inline]
    pub fn calculate(&self) -> Calculate<'_> {
        Calculate::new(self.points.iter(), self.is_closed)
    }

    #[inline]
    pub fn slices(&self) -> Slices<'_> {
        Slices::new(self.calculate())
    }
}

impl Curve {
    pub fn draw_lines(&self, d: &mut impl RaylibDraw, strips_per_bez: usize, color: Color) {
        if self.manifold_len() >= 2 {
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
        if self.points.is_empty() { return; }
        let num_points = strips_per_slice * 2 * self.num_slices();
        if num_points < 3 { return; }
        let total_strips = self.num_slices() * strips_per_slice;

        // let mut first_points: Option<[Vector2; 2]> = None;
        let mut past_points: Option<[Vector2; 2]> = None;
        {
            #[cfg(not(feature = "debug_stroke"))] let mut d = d.rl_begin_triangles();
            #[cfg(not(feature = "debug_stroke"))] d.rl_color4ub(color.r, color.g, color.b, color.a);

            for i in 0..total_strips {
                let t = i as f32 / (total_strips - 1) as f32;
                let v = self.velocity_at(t).expect("t should be 0-1");
                if v.magnitude_sqr() < f32::EPSILON { continue; }
                let tangent = v.normalized();
                let normal = tangent.rotate_cc();
                let p = self.position_at(t).expect("t should be 0-1");
                let extents = thick.extents_at(t);
                let p1 = p + normal * extents.0;
                let p2 = p - normal * extents.1;
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
                        d.draw_line_strip(&[p1.into(), p3.into(), p4.into()], Color::RED);
                        d.draw_line_strip(&[p2.into(), p1.into(), p4.into()], Color::BLUE);
                    }

                    past_points[0] = p1;
                    past_points[1] = p2;
                } else {
                    past_points = Some([p1, p2]);
                }
            }
        }

        if !self.is_closed {
            for (t, bez) in [
                (0.0, self.slice(0)),
                (1.0, self.slice(self.num_slices().saturating_sub(1))),
            ] {
                if let Some(bez) = bez {
                    let extents = thick.extents_at(t);
                    let v = bez.velocity_at(t);
                    let p = bez.position_at(t);
                    if v.magnitude_sqr() < f32::EPSILON { continue; }
                    let tangent = v.normalized();
                    let normal = tangent.rotate_cc();
                    let p1 = p + normal * extents.0;
                    let p2 = p - normal * extents.1;
                    d.draw_circle_v(p1.midpoint_v(p2), (extents.0 + extents.1) * 0.5, color);
                }
            }
        }
    }

    pub fn draw_fill(&self, d: &mut impl RaylibDraw, segments_per_slice: usize, color: Color) {
        if self.points.is_empty() { return; }
        let num_points = segments_per_slice * self.num_slices();
        if num_points < 3 { return; }
        let total_segments = self.num_slices() * segments_per_slice;

        #[cfg(not(any(feature = "debug_fill", feature = "debug_curvature")))] let mut d = d.rl_begin_triangles();
        #[cfg(not(any(feature = "debug_fill", feature = "debug_curvature")))] d.rl_color4ub(color.r, color.g, color.b, color.a);

        let mut first_point: Option<Vector2> = None;
        let mut prev_point: Option<(Vector2, f32)> = None;
        for i in 0..total_segments {
            let t = i as f32 / (total_segments - 1) as f32;
            let p = self.position_at(t).expect("t should be 0-1");
            let curvature = self.curvature_at(t).expect("t should be 0-1");
            if let Some((first_point, (prev_point, prev_curvature))) = first_point.as_mut().zip(prev_point.as_mut()) {
                #[cfg(not(any(feature = "debug_fill", feature = "debug_curvature")))] {
                    d.rl_vertex2f(first_point.x, first_point.y);
                    d.rl_vertex2f(prev_point.x, prev_point.y);
                    d.rl_vertex2f(p.x, p.y);
                } #[cfg(feature = "debug_fill")] {
                    d.draw_line_strip(&[first_point.into(), prev_point.into(), p.into()], color);
                } #[cfg(feature = "debug_curvature")] {
                    let tangent = self.velocity_at(t).expect("t should be 0-1");
                    let normal = tangent.rotate_cc();
                    d.draw_line_v(p, p - normal * curvature, Color::BLUE);
                    d.draw_circle_v(*first_point, 3.0, Color::RED);
                }
                if prev_curvature.is_sign_negative() != curvature.is_sign_negative() { // inflection
                    *first_point = p;
                }
            } else if prev_point.is_none() {
                first_point = Some(p);
            }
            prev_point = Some((p, curvature));
        }
    }
}

pub struct Calculate<'a> {
    iter: vec_deque::Iter<'a, PathPoint>,
    wrapped: Option<Option<(Vector2, Vector2, Vector2)>>,
}

impl<'a> Calculate<'a> {
    const fn new(iter: vec_deque::Iter<'a, PathPoint>, is_closed: bool) -> Self {
        Self {
            iter,
            wrapped: if is_closed { Some(None) } else { None }, // fun fact: `then_some()` isn't const :/
        }
    }
}

impl<'a> ExactSizeIterator for Calculate<'a> {
    fn len(&self) -> usize {
        let base_len = self.iter.len();
        base_len + if self.wrapped.is_none_or(|wrapped| base_len == 0 && wrapped.is_none()) { 0 } else { 1 }
    }
}

impl<'a> Iterator for Calculate<'a> {
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

    fn last(self) -> Option<Self::Item>
        where
            Self: Sized,
    {
        if let Some(inner) = self.wrapped {
            inner
        } else {
            self.iter.last().map(|x| x.calculate())
        }
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        let len = self.len();
        if let Some(next) = self.iter.nth(n) {
            let calculated = next.calculate();
            if let Some(inner @ None) = self.wrapped.as_mut() {
                *inner = Some(calculated);
            }
            return Some(calculated);
        } else if n == len - 1 {
            if let Some(inner) = self.wrapped.as_mut() {
                if inner.is_some() {
                    return inner.take();
                } else if len != 0 { // jump straight to the end of a new iterator
                    panic!("unsure what wrapped element is")
                }
            }
        }
        None
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let n = self.len();
        (n, Some(n))
    }
}

pub struct Slices<'a> {
    prev: Option<(Vector2, Vector2, Vector2)>,
    iter: Calculate<'a>,
}

impl<'a> Slices<'a> {
    const fn new(iter: Calculate<'a>) -> Self {
        Self { prev: None, iter }
    }
}

impl<'a> ExactSizeIterator for Slices<'a> {
    fn len(&self) -> usize {
        self.iter.len().saturating_sub(1) // 1 point does not a slice make
    }
}

impl<'a> Iterator for Slices<'a> {
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

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }
}
