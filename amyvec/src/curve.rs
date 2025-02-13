use std::collections::VecDeque;
use amymath::{prelude::{Matrix2x2, MinMaxRectangle, Rect2, Rotate90}, rlgl::*};
use raylib::prelude::*;
use crate::{
    bezier::cubic::Cubic as CubicBezier,
    path_point::{Ctrl, Maternal, PPPart, PathPoint, Vectoral},
};

#[derive(Debug)]
pub enum WidthProfile {
    Constant1Sided(f32),
    Constant2Sided(f32, f32),
    Variable1Sided(Curve<Vector2, Matrix2x2>),
    Variable2Sided(Curve<Vector3, Matrix>),
}

pub struct WidthProfileBuilder {
    points: Vec<(f32, (f32, Option<f32>))>,
}

impl WidthProfileBuilder {
    pub fn with_point(mut self, t: f32, thick1: f32, thick2: f32) -> Self {
        self.points.push((t, (thick1, Some(thick2))));
        self
    }

    pub fn build(mut self) -> WidthProfile {
        assert!(!self.points.is_empty(), "width profile cannot be empty");
        self.points.sort_by(|(a, _), (b, _)| a.partial_cmp(&b).expect("time should be normal"));
        if self.points.iter().any(|(_, (_, x))| x.is_some()) {
            WidthProfile::Variable2Sided(Curve {
                is_closed: false,
                points: self.points.into_iter()
                    .map(|(t, (thick1, thick2))| Vector3::new(t, thick1, thick2.unwrap_or_default()))
                    .collect(),
            })
        } else {
            WidthProfile::Variable1Sided(Curve {
                is_closed: false,
                points: self.points.into_iter()
                    .map(|(t, (thick1, _))| Vector2::new(t, thick1))
                    .collect(),
            })
        }
    }
}

impl WidthProfile {
    pub const fn new_constant2(thick1: f32, thick2: f32) -> Self {
        Self::Constant2Sided(thick1, thick2)
    }

    pub fn init() -> WidthProfileBuilder {
        WidthProfileBuilder {
            points: Vec::with_capacity(1),
        }
    }

    pub fn extents_at(&self, t: f32) -> (f32, f32) {
        match self {
            &WidthProfile::Constant1Sided(x) => (x, x),
            &WidthProfile::Constant2Sided(a, b) => (a, b),

            WidthProfile::Variable1Sided(curve) => {
                let Vector2 { x: _, y } = curve.position_at(t).expect("width profile should not be empty");
                (y, y)
            },
            WidthProfile::Variable1Sided(curve) => {
                let Vector3 { x: _, y, z } = curve.position_at(t).expect("width profile should not be empty");
                (y, z)
            },
        }
    }

    pub fn extents_min(&self) -> (f32, f32) {
        match self {
            &WidthProfile::Constant1Sided(x) => (x, x),
            &WidthProfile::Constant2Sided(a, b) => (a, b),

            WidthProfile::Variable(curve) => {
                let bounds = curve.bounds().expect("width profile should not be empty");
                Vector2::new(bounds.xmin, bounds.ymin)
            },
        }
    }

    pub fn extents_max(&self) -> (f32, f32) {
        match self {
            &WidthProfile::Constant1Sided(x) => (x, x),
            &WidthProfile::Constant2Sided(a, b) => (a, b),

            WidthProfile::Variable(curve) => {
                let bounds = curve.bounds().expect("width profile should not be empty");
                Vector2::new(bounds.xmax, bounds.ymax)
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

#[derive(Debug)]
pub struct Curve<V: Vectoral, M: Maternal<V>> {
    pub points: VecDeque<PathPoint<V, M>>,
    pub is_closed: bool,
}

impl<V: Vectoral, M: Maternal<V>> Default for Curve<V, M> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl<V: Vectoral, M: Maternal<V>> Curve<V, M> {
    pub fn new() -> Self {
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
    pub fn slice(&self, start_index: usize) -> Option<CubicBezier<V>> {
        if let Some((pp2, pp1)) = start_index.checked_add(1)
            .and_then(|end_index| self.points.get(end_index))
            .map(|pp2| (pp2, self.points.get(start_index).expect("existence of [i+1] should guarantee existence of [i]")))
        {
            let ((_, p1, c1_out), (c2_in, p2, _)) = (pp1.calculate(), pp2.calculate());
            return Some(CubicBezier::new(p1, c1_out, c2_in, p2))
        }
        None
    }

    /// Calculate the bounding box of the entire curve
    ///
    /// Returns [`None`] if the curve is empty
    ///
    /// ## Note regarding current implementation
    /// Calls [`Curve::slices`] (which calculates every path point)
    /// and [`CubicBezier::bounds`] (which solves the quadratic equation) on each.
    #[inline]
    pub fn bounds(&self) -> Option<V::Rect> {
        let mut bez_iter = self.slices();
        if let Some(bez) = bez_iter.next() {
            let mut rec = bez.bounds();
            for bez in bez_iter {
                if !rec.entirely_contains(&bez.max_bounds()) {
                    rec = rec.max(bez.bounds());
                }
            }
            Some(rec)
        } else { None }
    }

    #[inline]
    pub fn position_at(&self, t: f32) -> Option<V> {
        if self.points.len() == 0 { return None; }
        assert!(0.0 <= t && t <= 1.0, "t out of bounds");
        if self.points.len() == 1 {
            return Some(self.points[0].p);
        }
        let t_major = t * (self.points.len() - 2) as f32;
        let (slice_idx, t) = if t_major == 1.0 {
            (self.points.len() - 2, 1.0)
        } else {
            (t_major.floor() as usize, t_major.fract())
        };
        Some(self.slice(slice_idx).unwrap().position_at(t))
    }

    #[inline]
    pub fn max_bounds(&self) -> Option<V::Rect> {
        self.slices()
            .map(|bez| bez.max_bounds())
            .reduce(|rec, b| rec.max(b))
    }

    #[inline]
    pub fn calculate(&self) -> Calculate<V, impl Iterator<Item = &'_ PathPoint<V, M>>> {
        Calculate::new(self.points.iter(), self.is_closed)
    }

    #[inline]
    pub fn slices(&self) -> Slices<V, impl Iterator<Item = &'_ PathPoint<V, M>>> {
        Slices::new(self.calculate())
    }

    pub fn draw_lines(&self, d: &mut impl RaylibDraw, strips_per_bez: usize, color: Color) {
        if self.points.is_empty() { return; }
        let mut points = Vec::with_capacity(strips_per_bez * self.points.len());
        for bez in self.slices() {
            for i in 0..strips_per_bez {
                let t = i as f32 / (strips_per_bez - 1) as f32;
                let p = bez.position_at(t);
                points.push_within_capacity(p).expect("should not realloc");
            }
        }
        d.draw_line_strip(&points[..], color);
    }

    pub fn draw_stroke(&self, d: &mut impl RaylibDraw, strips_per_slice: usize, thick: &WidthProfile, color: Color) {
        if self.points.is_empty() {
        } else if self.points.len() == 1 {
            let extents = thick.extents_max(); // largest thickness would overtake smaller thicknesses
            d.draw_circle_v(self.points[0].p, (extents.x + extents.y) * 0.5, color);
        } else {
            let num_points = strips_per_slice * 2 * self.points.len();
            if num_points < 3 { return; }
            let total_strips = self.points.len() * strips_per_slice;

            for (t, t_full, bez) in [
                (0.0, 0.0, self.slice(0).expect("should have at least 2 points in this branch")),
                (1.0, 1.0, self.slice(self.num_slices() - 1).expect("should have at least 2 points in this branch"))
            ] {
                let extents = thick.extents_at(t_full);
                let v = bez.velocity_at(t);
                let p = bez.position_at(t);
                if v.length_sqr() >= f32::EPSILON {
                    let tangent = v.normalized();
                    let (normal_cw, normal_cc) = (tangent.rotate90_cw(), tangent.rotate90_cc());
                    let p1 = p + normal_cc * extents.x;
                    let p2 = p + normal_cw * extents.y;
                    d.draw_circle_v(p1.midpoint(p2), (extents.x + extents.y) * 0.5, color);
                } else {
                    d.draw_circle_v(p, (extents.x + extents.y) * 0.5, color);
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
                    if v.length_sqr() < f32::EPSILON { continue; }
                    let tangent = v.normalized();
                    let (normal_cw, normal_cc) = (tangent.rotate90_cw(), tangent.rotate90_cc());
                    let p = bez.position_at(t);
                    let t_full = (row + i) as f32 / total_strips as f32;
                    let extents = thick.extents_at(t_full);
                    let p1 = p + normal_cc * extents.x;
                    let p2 = p + normal_cw * extents.y;
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

pub struct Calculate<V, I> {
    iter: I,
    wrapped: Option<Option<(V, V, V)>>,
}

impl<V, I> Calculate<V, I> {
    const fn new(iter: I, is_closed: bool) -> Self {
        Self {
            iter,
            wrapped: if is_closed { Some(None) } else { None }, // fun fact: `then_some()` isn't const :/
        }
    }
}

impl<'a, V: Vectoral + 'a, M: Maternal<V> + 'a, I: Iterator<Item = &'a PathPoint<V, M>>> Iterator for Calculate<V, I> {
    type Item = (V, V, V);

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

pub struct Slices<V, I> {
    prev: Option<(V, V, V)>,
    iter: Calculate<V, I>,
}

impl<V, I> Slices<V, I> {
    const fn new(iter: Calculate<V, I>) -> Self {
        Self { prev: None, iter }
    }
}

impl<'a, V: Vectoral + 'a, M: Maternal<V> + 'a, I: Iterator<Item = &'a PathPoint<V, M>>> Iterator for Slices<V, I> {
    type Item = CubicBezier<V>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut next = self.iter.next();
        if self.prev.is_none() && next.is_some() {
            self.prev = next;
            next = self.iter.next();
        }
        if let (Some(pp1), Some(pp2)) = (self.prev, next) {
            let ((_, p1, c1_out), (c2_in, p2, _)) = (pp1, pp2);
            self.prev = Some(pp2);
            Some(CubicBezier::new(p1, c1_out, c2_in, p2))
        } else {
            None
        }
    }
}
