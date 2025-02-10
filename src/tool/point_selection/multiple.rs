use std::fmt;

use raylib::prelude::*;
use amymath::prelude::*;
use amylib::rc::prelude::*;
use crate::{document::layer::Layer, layer::LayerType, shaders::ShaderTable, vector_path::{VectorPath, DrawPathPoint}, Change, Document};
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

/// Allows manipulating many points at once, even from separate paths, but not velocity controls
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
            let mut selected_layers = self.pieces.iter().peekable();
            for (selected, path) in document.layers.shallow_iter()
                .filter_map(|layer|
                    if let Layer::Path(path) = layer {
                        Some((selected_layers.next_if(|selected| path == &selected.target), path.clone_ref()))
                    } else { None })
            {
                let path = path.read();
                path.draw_selected(d, px_world_size);
                if let Some(selected) = selected {
                    let mut selected_points = selected.points.iter().peekable();
                    for (is_point_selected, pp) in path.curve.points.iter()
                        .enumerate()
                        .map(|(i, pp)| (selected_points.next_if_eq(&&i).is_some(), pp))
                    {
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
