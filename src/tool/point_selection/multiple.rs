use raylib::prelude::*;
use amylib::rc::*;
use crate::{layer::{group::Group, BackToFore, Layer}, vector_path::{path_point::{DistanceSqr, PathPoint}, VectorPath}, Change, Document};
use super::{DepthFirstIter, HOVER_RADIUS_SQR};

struct EditMultiPointAction {

}

impl Change for EditMultiPointAction {
    fn redo(&mut self, _document: &mut Document) -> Result<(), String> {
        todo!()
    }

    fn undo(&mut self, _document: &mut Document) -> Result<(), String> {
        todo!()
    }
}

pub struct SelectionPiece {
    pub target: StrongMut<Layer>,
    pub points: Vec<usize>,
}

pub struct MultiSelect {
    pub pieces: Vec<SelectionPiece>,
}

impl MultiSelect {
    pub fn drag(&mut self, delta: Vector2) {
        for SelectionPiece { target, ref points } in &mut self.pieces {
            let mut layer = target.write();
            let Layer::Path(path) = &mut *layer else { panic!("point selection must target path") };
            for idx in points {
                path.points[*idx].move_point(delta);
            }
        }
    }

    pub fn is_selected(&self, mouse_world_pos: Vector2) -> bool {
        self.pieces.iter()
            .any(|SelectionPiece { target, points }| {
                let layer = target.read();
                let Layer::Path(path) = &*layer else { panic!("point selection must target path") };
                points.iter().any(|&idx| path.points[idx].p.distance_sqr_to(mouse_world_pos) <= HOVER_RADIUS_SQR)
            })
    }

    pub fn draw(&self, d: &mut impl RaylibDraw, document: &Document, mouse_world_pos: Vector2) {

    }
}

pub trait EnumerateSelectedLayers<T, U> {
    type Iter: Iterator<Item = T>;
    type Key: for<'k> Fn(&'k T) -> &'k Strong<Layer>;
    type Val: Fn(T) -> U;
    fn enumerate_selected_layers(self, selection: &MultiSelect) -> LayerSelection<'_, T, U, Self::Iter, Self::Key, Self::Val>;
}

impl<'a> EnumerateSelectedLayers<(usize, Strong<Layer>), Strong<Layer>> for &'a Document {
    type Iter = DepthFirstIter<Layer, for<'g> fn(&'g Group) -> bool>;
    type Key = for<'k> fn(&'k (usize, Strong<Layer>)) -> &'k Strong<Layer>;
    type Val = fn((usize, Strong<Layer>)) -> Strong<Layer>;
    fn enumerate_selected_layers(self, selection: &MultiSelect) -> LayerSelection<'_, (usize, Strong<Layer>), Strong<Layer>, Self::Iter, Self::Key, Self::Val> {
        let mut selected_iter = selection.pieces.iter();
        let awaiting_match = selected_iter.next();
        LayerSelection {
            iter: self.layers.tree_iter(BackToFore, |_| false),
            key: |(_, layer)| layer,
            val: |(_, layer)| layer,
            selected_iter,
            awaiting_match,
        }
    }
}

pub struct LayerSelection<'a, T, U, I: Iterator<Item = T>, K: Fn(&T) -> &Strong<Layer>, V: Fn(T) -> U> {
    iter: I,
    key: K,
    val: V,
    selected_iter: std::slice::Iter<'a, SelectionPiece>,
    awaiting_match: Option<&'a SelectionPiece>,
}

impl<'a, T, U, I: Iterator<Item = T>, K: Fn(&T) -> &Strong<Layer>, V: Fn(T) -> U> Iterator for LayerSelection<'a, T, U, I, K, V> {
    type Item = (Option<&'a SelectionPiece>, U);
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().and_then(|item| {
            let target = (self.key)(&item);
            let selected_points = self.awaiting_match.and_then(|selected_layer| (*target == selected_layer.target).then_some(selected_layer));
            if selected_points.is_some() {
                self.awaiting_match = self.selected_iter.next();
            }
            Some((selected_points, (self.val)(item)))
        })
    }
}

pub trait EnumerateSelectedPoints<T, U> {
    type Iter: Iterator<Item = T>;
    type Key: for<'k> Fn(&'k T) -> usize;
    type Val: Fn(T) -> U;
    fn enumerate_selected_points(self, selection: &SelectionPiece) -> PointSelection<'_, T, U, Self::Iter, Self::Key, Self::Val>;
}

impl<'a> EnumerateSelectedPoints<(usize, &'a PathPoint), &'a PathPoint> for &'a VectorPath {
    type Iter = std::iter::Enumerate<std::collections::vec_deque::Iter<'a, PathPoint>>;
    type Key = for<'k> fn(&'k (usize, &'a PathPoint)) -> usize;
    type Val = fn((usize, &'a PathPoint)) -> &'a PathPoint;
    fn enumerate_selected_points(self, selection: &SelectionPiece) -> PointSelection<'_, (usize, &'a PathPoint), &'a PathPoint, Self::Iter, Self::Key, Self::Val> {
        let mut selected_iter = selection.points.iter().copied();
        let awaiting_match = selected_iter.next();
        PointSelection {
            iter: self.points.iter().enumerate(),
            key: |(idx, _)| *idx,
            val: |(_, pp)| pp,
            selected_iter,
            awaiting_match,
        }
    }
}

pub struct PointSelection<'a, T, U, I: Iterator<Item = T>, K: for<'b> Fn(&'b T) -> usize, V: Fn(T) -> U> {
    iter: I,
    key: K,
    val: V,
    selected_iter: std::iter::Copied<std::slice::Iter<'a, usize>>,
    awaiting_match: Option<usize>,
}

impl<'a, T, U, I: Iterator<Item = T>, K: for<'b> Fn(&'b T) -> usize, V: Fn(T) -> U> Iterator for PointSelection<'a, T, U, I, K, V> {
    type Item = (bool, U);
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().and_then(|item| {
            let idx = (self.key)(&item);
            let is_selected = self.awaiting_match.is_some_and(|selected_idx| idx == selected_idx);
            if is_selected {
                self.awaiting_match = self.selected_iter.next();
            }
            Some((is_selected, (self.val)(item)))
        })
    }
}
