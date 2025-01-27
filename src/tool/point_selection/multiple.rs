use raylib::prelude::*;
use amylib::{iter::directed::*, rc::*};
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

pub trait EnumerateSelectedLayers {
    fn enumerate_selected_layers(self, selection: &MultiSelect) -> LayerSelection<'_, Self> where Self: Sized;
}

impl<I> EnumerateSelectedLayers for I {
    #[inline]
    fn enumerate_selected_layers(self, selection: &MultiSelect) -> LayerSelection<'_, Self> where Self: Sized {
        LayerSelection::new(self, selection)
    }
}

pub struct LayerSelection<'a, I> {
    iter: I,
    selected_iter: std::slice::Iter<'a, SelectionPiece>,
    awaiting_match: Option<&'a SelectionPiece>,
}

impl<'a, I> LayerSelection<'a, I> {
    fn new(iter: I, selection: &'a MultiSelect) -> Self {
        let mut selected_iter = selection.pieces.iter();
        let awaiting_match = selected_iter.next();
        Self {
            iter,
            selected_iter,
            awaiting_match,
        }
    }
}

impl<'a, I: Iterator<Item = Strong<Layer>>> Iterator for LayerSelection<'a, I> {
    type Item = (Option<&'a SelectionPiece>, I::Item);
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().and_then(|item| {
            let selected_points = self.awaiting_match.and_then(|selected_layer| (item == selected_layer.target).then_some(selected_layer));
            if selected_points.is_some() {
                self.awaiting_match = self.selected_iter.next();
            }
            Some((selected_points, item))
        })
    }
}

pub trait EnumerateSelectedPoints<'a> {
    fn enumerate_selected_points(self, selection: &SelectionPiece) -> PointSelection<'_, std::collections::vec_deque::Iter<'a, PathPoint>> where Self: Sized;
}

impl<'a> EnumerateSelectedPoints<'a> for &'a VectorPath {
    fn enumerate_selected_points(self, selection: &SelectionPiece) -> PointSelection<'_, std::collections::vec_deque::Iter<'a, PathPoint>> where Self: Sized {
        let mut selected_iter = selection.points.iter().copied();
        let awaiting_match = selected_iter.next();
        PointSelection {
            iter: self.points.iter().enumerate(),
            selected_iter,
            awaiting_match,
        }
    }
}

pub struct PointSelection<'a, I> {
    iter: std::iter::Enumerate<I>,
    selected_iter: std::iter::Copied<std::slice::Iter<'a, usize>>,
    awaiting_match: Option<usize>,
}

impl<'a, I: Iterator> Iterator for PointSelection<'a, I> {
    type Item = (bool, I::Item);
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().and_then(|item| {
            let is_selected = self.awaiting_match.is_some_and(|selected_idx| item.0 == selected_idx);
            if is_selected {
                self.awaiting_match = self.selected_iter.next();
            }
            Some((is_selected, item.1))
        })
    }
}
