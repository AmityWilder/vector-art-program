use raylib::prelude::*;
use amymath::prelude::Vector2;
use amylib::rc::prelude::*;
use crate::{editor::Editor, layer::LayerType, shaders::ShaderTable, vector_path::{DrawPathPoint, VectorPath}};
use super::HOVER_RADIUS;

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

    pub fn end_dragging(&mut self) {}

    pub fn is_selected(&self, mouse_world_pos: Vector2, px_world_size: f32) -> bool {
        let hover_radius = HOVER_RADIUS * px_world_size;
        self.pieces.iter()
            .any(|SelectionPiece { target, points }| {
                let path = target.read();
                points.iter().any(|&idx| path.curve.points[idx].p.rec_distance_to(mouse_world_pos) <= hover_radius)
            })
    }

    pub fn draw(&self, d: &mut impl RaylibDraw, _editor: &Editor, px_world_size: f32, _shader_table: &ShaderTable) {
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
