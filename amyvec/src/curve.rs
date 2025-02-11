use std::collections::VecDeque;
use amymath::{prelude::{Rect2, Rotate90}, rlgl::*};
use raylib::prelude::*;
use crate::{
    cubic_bezier::CubicBezier,
    path_point::PathPoint,
};

pub enum WidthProfile {
    Constant(Vector2),
    Variable(Curve),
}

impl WidthProfile {
    pub fn extents_at(&self, t: f32) -> Vector2 {
        match self {
            WidthProfile::Constant(v) => *v,
            WidthProfile::Variable(curve) => curve.position_at(t).expect("width profile should not be empty"),
        }
    }
}

#[derive(Debug)]
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
    pub fn new() -> Self {
        Self {
            points: VecDeque::new(),
            is_closed: false,
        }
    }

    #[inline]
    pub fn slice(&self, start_index: usize) -> Option<CubicBezier> {
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
    pub fn bounds(&self) -> Option<Rect2> {
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
    pub fn position_at(&self, t: f32) -> Option<Vector2> {
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
    pub fn max_bounds(&self) -> Option<Rect2> {
        self.slices()
            .map(|bez| bez.max_bounds())
            .reduce(|rec, b| rec.max(b))
    }

    #[inline]
    pub fn calculate(&self) -> Calculate<impl Iterator<Item = &'_ PathPoint>> {
        Calculate::new(self.points.iter(), self.is_closed)
    }

    #[inline]
    pub fn slices(&self) -> Slices<impl Iterator<Item = &'_ PathPoint>> {
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

    pub fn draw_stroke(&self, d: &mut impl RaylibDraw, strips_per_bez: usize, thick: &WidthProfile, color: Color) {
        if self.points.is_empty() { return; }
        let num_points = strips_per_bez * 2 * self.points.len();
        if num_points < 3 { return; }
        let total_strips = self.points.len() * strips_per_bez;

        let extents = thick.extents_at(0.0);
        d.draw_circle_v(self.points[0].p, (extents.x + extents.y) * 0.5, color);
        let extents = thick.extents_at(1.0);
        d.draw_circle_v(self.points[self.points.len() - 1].p, (extents.x + extents.y) * 0.5, color);

        let mut d = d.begin_rlgl();
        let mut d = d.rl_begin_triangles();
        d.rl_color4ub(color.r, color.g, color.b, color.a);

        let mut past_points: Option<[Vector2; 2]> = None;
        for (n, bez) in self.slices().enumerate() {
            let row = n * strips_per_bez;
            for i in 0..strips_per_bez {
                let t = i as f32 / (strips_per_bez - 1) as f32;
                let t_full = (row + i) as f32 / total_strips as f32;
                let extents = thick.extents_at(t_full);
                let p = bez.position_at(t);
                let v = bez.velocity_at(t);
                let tangent = v.normalized();
                let (normal_cw, normal_cc) = (tangent.rotate90_cw(), tangent.rotate90_cc());
                let p1 = p + normal_cc * extents.x;
                let p2 = p + normal_cw * extents.y;
                if let Some(past_points) = &mut past_points {
                    let [p3, p4] = *past_points;
                    d.rl_vertex2f(p1.x, p1.y);
                    d.rl_vertex2f(p3.x, p3.y);
                    d.rl_vertex2f(p4.x, p4.y);

                    d.rl_vertex2f(p2.x, p2.y);
                    d.rl_vertex2f(p1.x, p1.y);
                    d.rl_vertex2f(p4.x, p4.y);

                    past_points[0] = p1;
                    past_points[1] = p2;
                } else {
                    past_points = Some([p1, p2]);
                }
            }
        }
    }

    pub fn draw_fill(&self, d: &mut impl RaylibDraw, color: Color) {
        const DENSITY: usize = 10;
        if self.points.is_empty() { return; }
        let num_points = DENSITY * self.points.len();
        if num_points < 3 { return; }

        let mut d = d.begin_rlgl();
        let mut d = d.rl_begin_triangles();
        d.rl_color4ub(color.r, color.g, color.b, color.a);

        let mut first_point: Option<Vector2> = None;
        let mut prev_point: Option<Vector2> = None;
        for bez in self.slices() {
            for i in 0..DENSITY {
                let t = i as f32 / (DENSITY - 1) as f32;
                let p = bez.position_at(t);
                if let Some((first_point, prev_point)) = first_point.as_ref().zip(prev_point.as_mut()) {
                    d.rl_vertex2f(first_point.x, first_point.y);
                    d.rl_vertex2f(prev_point.x, prev_point.y);
                    d.rl_vertex2f(p.x, p.y);
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
    type Item = CubicBezier;

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
