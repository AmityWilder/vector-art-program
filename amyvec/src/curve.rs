use std::collections::VecDeque;
use raylib::prelude::*;
use crate::{
    cubic_bezier::CubicBezier,
    path_point::{Ctrl, PathPoint},
};

pub struct Curve {
    pub points: VecDeque<PathPoint>,
    pub is_closed: bool,
}

impl Curve {
    pub fn new() -> Self {
        Self {
            points: VecDeque::new(),
            is_closed: false,
        }
    }

    pub fn slice(&self, start_index: usize) -> Option<CubicBezier> {
        if let (Some(pp1), Some(pp2)) = (
            self.points.get(start_index),
            start_index.checked_add(1)
                .and_then(|end_index| self.points.get(end_index)),
        ) {
            let ((_, p1, c1_out), (c2_in, p2, _)) = (pp1.calculate(), pp2.calculate());
            return Some(CubicBezier::new(p1, c1_out, c2_in, p2))
        }
        None
    }

    pub fn edge_distance_to(&self, _pt: Vector2) -> f32 {
        todo!()
    }

    pub fn calculate(&self) -> Calculate<impl Iterator<Item = &'_ PathPoint>> {
        Calculate::new(self.points.iter(), self.is_closed)
    }

    pub fn slices(&self) -> Slices<impl Iterator<Item = &'_ PathPoint>> {
        Slices::new(self.calculate())
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
            wrapped: if is_closed { Some(None) } else { None },
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
