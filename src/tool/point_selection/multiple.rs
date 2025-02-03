use std::fmt;

use raylib::prelude::*;
use amymath::prelude::*;
use amylib::rc::prelude::*;
use crate::{document::layer::Layer, layer::LayerType, shaders::ShaderTable, vector_path::{path_point::PathPoint, VectorPath, DrawPathPoint}, Change, Document};
use super::HOVER_RADIUS;

struct EditMultiPointAction {

}

impl fmt::Debug for EditMultiPointAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("EditMultiPointAction").finish()
    }
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
    pub target: StrongMut<VectorPath>,
    pub points: Vec<usize>,
}

pub struct MultiSelect {
    pub pieces: Vec<SelectionPiece>,
}

impl MultiSelect {
    pub fn drag(&mut self, delta: Vector2) {
        for SelectionPiece { target, ref points } in &mut self.pieces {
            let mut path = target.write();
            for idx in points {
                path.curve.points[*idx].move_point(delta);
            }
        }
    }

    pub fn is_selected(&self, mouse_world_pos: Vector2) -> bool {
        self.pieces.iter()
            .any(|SelectionPiece { target, points }| {
                let path = target.read();
                points.iter().any(|&idx| path.curve.points[idx].p.rec_distance_to(mouse_world_pos) <= HOVER_RADIUS)
            })
    }

    pub fn draw(&self, d: &mut impl RaylibDraw, document: &Document, px_world_size: f32, selection_rec: Option<Rectangle>, _shader_table: &ShaderTable) {
        if let Some(selection_rec) = selection_rec {
            for (selected, layer) in document.layers.shallow_iter().enumerate_selected_layers(self) {
                if let Layer::Path(path) = layer {
                    let path = path.read();
                    path.draw_selected(d, px_world_size);
                    if let Some(selected) = selected {
                        for (is_point_selected, pp) in path.enumerate_selected_points(selected) {
                            let is_selected = is_point_selected || selection_rec.check_collision_point_rec(pp.p);
                            d.draw_path_point(pp, px_world_size, path.settings.color, is_selected, false, false);
                        }
                    } else {
                        for pp in &path.curve.points {
                            let is_selected = selection_rec.check_collision_point_rec(pp.p);
                            d.draw_path_point(pp, px_world_size, path.settings.color, is_selected, false, false);
                        }
                    }
                }
            }
        } else {
            for piece in &self.pieces {
                let path = piece.target.read();
                path.draw_selected(d, px_world_size);
                let mut indices = piece.points.iter().copied();
                let mut idx = indices.next();
                for (pp_idx, pp) in path.curve.points.iter().enumerate() {
                    let is_selected = idx.is_some_and(|idx| pp_idx == idx);
                    if is_selected { idx = indices.next(); }
                    d.draw_path_point(pp, px_world_size, path.settings.color, is_selected, false, false);
                }
            }
        }
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

impl<'a, I: Iterator<Item = &'a Layer>> Iterator for LayerSelection<'a, I> {
    type Item = (Option<&'a SelectionPiece>, I::Item);
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().and_then(|item| {
            if let Layer::Path(path) = item {
                let selected_points = self.awaiting_match.and_then(|selected_layer| (path == &selected_layer.target).then_some(selected_layer));
                if selected_points.is_some() {
                    self.awaiting_match = self.selected_iter.next();
                }
                Some((selected_points, item))
            } else {
                None
            }
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
            iter: self.curve.points.iter().enumerate(),
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

impl<I: Iterator> Iterator for PointSelection<'_, I> {
    type Item = (bool, I::Item);
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|item| {
            let is_selected = self.awaiting_match.is_some_and(|selected_idx| item.0 == selected_idx);
            if is_selected {
                self.awaiting_match = self.selected_iter.next();
            }
            (is_selected, item.1)
        })
    }
}
