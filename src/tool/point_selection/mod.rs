use amymath::prelude::*;
use raylib::prelude::*;
use amylib::{iter::directed::DirectibleDoubleEndedIterator, prelude::StrongMut};
use crate::{document::layer::Layer, layer::{BackToFore, ForeToBack, LayerType}, shaders::ShaderTable, vector_path::{path_point::PPPart, DrawPathPoint, VectorPath}, Document};
use super::ToolType;

pub const HOVER_RADIUS: f32 = 3.0;
pub const HOVER_RADIUS_SQR: f32 = HOVER_RADIUS * HOVER_RADIUS;

pub const SNAP_VERT_RADIUS: f32 = 4.0;
pub const SNAP_VERT_RADIUS_SQR: f32 = SNAP_VERT_RADIUS * SNAP_VERT_RADIUS;

mod singular;
mod multiple;

use singular::SingleSelect;
use multiple::{MultiSelect, SelectionPiece};

/// Pieces should be ordered by unique target layer [`TreeIterDir::BackToFore`]. Points should be ordered by index.
enum Selection {
    Singular(SingleSelect),
    Multiple(MultiSelect),
}

struct SelectionState {
    pub selection: Selection,
    pub drag: Option<(Vector2, Vector2)>, // (start, cumulative)
}

impl SelectionState {
    pub fn drag(&mut self, delta: Vector2) {
        match &mut self.selection {
            Selection::Singular(x) => x.drag(delta),
            Selection::Multiple(x) => x.drag(delta),
        }
    }
}

pub struct PointSelection {
    state: Option<SelectionState>,
    selection_points: Option<(Vector2, Vector2)>,
}

impl PointSelection {
    pub const fn new() -> Self {
        Self {
            state: None,
            selection_points: None,
        }
    }

    pub fn with_target(target: &StrongMut<VectorPath>) -> Self {
        Self {
            state: Some(SelectionState {
                selection: Selection::Singular(SingleSelect {
                    target: target.clone_mut(),
                    pt_idx: None,
                    part: PPPart::Anchor
                }),
                drag: None,
            }),
            selection_points: None,
        }
    }

    fn find_hovered_anchor(document: &mut Document, mouse_world_pos: Vector2) -> Option<(&StrongMut<VectorPath>, Option<usize>)> {
        document.layers
            .shallow_iter_mut()
            .cdir::<ForeToBack>()
            .filter_map(|layer| if let Layer::Path(ref path) = layer { Some(path) } else { None })
            .filter(|path| path.read().curve.bounds().is_some_and(|bounds| bounds.is_overlapping_point(mouse_world_pos)))
            .find_map(|path| {
                let path_borrow = path.read();
                let curve = &path_borrow.curve;

                let idx = curve.points.iter()
                    .enumerate()
                    .map(|(i, pp)| (i, pp.p.rec_distance_to(mouse_world_pos)))
                    .filter(|(_, dist)| *dist <= HOVER_RADIUS)
                    .min_by(|(_, a), (_, b)| a.partial_cmp(b).expect("distance should not be NaN"))
                    .map(|(i, _)| i);

                if idx.is_some() || curve.slices().any(|bez|
                    bez.bounds()
                        .grow(HOVER_RADIUS)
                        .is_overlapping_point(mouse_world_pos) &&
                    bez
                        .estimate_time_at(mouse_world_pos)
                        .is_some_and(|(_, p)| p.distance_sqr_to(mouse_world_pos) <= HOVER_RADIUS_SQR)
                ) {
                    drop(path_borrow);
                    return Some((path, idx));
                }
                None
            })
    }

    fn begin_dragging(&mut self, document: &mut Document, mouse_world_pos: Vector2) {
        let drag = Some((mouse_world_pos, mouse_world_pos));

        if let Some(state) = self.state.as_mut() {
            match &mut state.selection {
                Selection::Singular(x) => {
                    if let Some(part) = x.get_selected(mouse_world_pos) {
                        x.part = part;
                        state.drag = drag;
                        return;
                    }
                },

                Selection::Multiple(x) => {
                    if x.is_selected(mouse_world_pos) {
                        state.drag = drag;
                        return;
                    }
                }
            }
        }

        let hovered_point = Self::find_hovered_anchor(document, mouse_world_pos);

        if let Some((hovered_target, hovered_idx)) = hovered_point {
            self.state = Some(SelectionState {
                selection: Selection::Singular(SingleSelect {
                    target: hovered_target.clone_mut(),
                    pt_idx: hovered_idx,
                    part: PPPart::Anchor,
                }),
                drag,
            });
        } else {
            self.state = None;
            self.selection_points = Some((mouse_world_pos, mouse_world_pos));
        }
    }

    fn end_dragging(&mut self, document: &mut Document, mouse_world_pos: Vector2) {
        if let Some(state) = self.state.as_mut() {
            match &mut state.selection {
                Selection::Singular(single_select) => single_select.end_dragging(document.camera.zoom.recip()),
                Selection::Multiple(multi_select) => multi_select.end_dragging(),
            }
            state.drag = None;
        }

        // finalize selection
        if let Some((selection_start, _)) = self.selection_points.take() {
            let selection_rec = selection_start.minmax_rec(mouse_world_pos);

            let selected = document.layers
                .shallow_iter_mut()
                .filter_map(|layer| {
                    if let Layer::Path(path) = layer {
                        let points = path.read().curve.points.iter()
                            .enumerate()
                            .filter_map(|(idx, pp)|
                                selection_rec.check_collision_point_rec(pp.p)
                                    .then_some(idx))
                            .collect::<Vec<usize>>();

                        if !points.is_empty() {
                            return Some(SelectionPiece { target: path.clone_mut(), points, });
                        }
                    }
                    None
                })
                .collect::<Vec<SelectionPiece>>();

            self.state = (!selected.is_empty())
                .then(|| {
                    SelectionState {
                        selection: match &selected[..] {
                            [SelectionPiece { target, points }] if points.len() == 1
                                => Selection::Singular(SingleSelect { target: target.clone_mut(), pt_idx: Some(points[0]), part: PPPart::Anchor }),
                            [..]
                                => Selection::Multiple(MultiSelect { pieces: selected }),
                        },
                        drag: None,
                    }
                });
        }
    }
}

impl ToolType for PointSelection {
    fn tick(&mut self, rl: &mut RaylibHandle, _thread: &RaylibThread, document: &mut Document, mouse_world_pos: Vector2) {
        if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
            self.begin_dragging(document, mouse_world_pos);
        }

        if let Some((_, curr)) = self.selection_points.as_mut() {
            *curr = mouse_world_pos;
        }

        if let Some(state) = self.state.as_mut() && let Some((_drag_start, drag_cum)) = state.drag.as_mut() {
            let delta = mouse_world_pos - *drag_cum;
            *drag_cum += delta;
            state.drag(delta);
        }

        if rl.is_mouse_button_released(MouseButton::MOUSE_BUTTON_LEFT) {
            self.end_dragging(document, mouse_world_pos);
        }

        if rl.is_key_pressed(KeyboardKey::KEY_DELETE) && let Some(selection_state) = self.state.as_mut() {
            match &mut selection_state.selection {
                Selection::Singular(single_select) => {
                    let mut path = single_select.target.write();
                    if let Some(idx) = single_select.pt_idx {
                        path.curve.points.remove(idx).expect("should not select a point that is not within the curve");
                    } else {
                        path.curve.points.clear();
                    }
                }
                Selection::Multiple(multi_select) => {
                    for piece in &mut multi_select.pieces {
                        let mut path = piece.target.write();
                        let mut remove = piece.points.iter().peekable();
                        let mut keep = (0..path.curve.points.len()).map(|i| remove.next_if_eq(&&i).is_none());
                        path.curve.points.retain(|_| keep.next().expect("should visit 0..len elements"));
                    }
                }
            }
            self.state = None;
        }
    }

    fn draw(&self, d: &mut impl RaylibDraw, document: &Document, shader_table: &ShaderTable, px_world_size: f32, viewport: &Rect2, #[cfg(dev)] _mouse_world_pos: Vector2) {
        let selection_rec = self.selection_points.as_ref().copied().map(|(start, end)|
            start.minmax_rec(end)
        );

        if let Some(selection_rec) = selection_rec.as_ref() {
            d.draw_rectangle_rec(selection_rec, Color::BLUE.alpha(0.125));
        }

        match self.state.as_ref() {
            Some(SelectionState { selection: Selection::Singular(selected), .. }) => {
                selected.draw(d, document, px_world_size, selection_rec, shader_table);
            }

            Some(SelectionState { selection: Selection::Multiple(selected), .. }) => {
                selected.draw(d, document, px_world_size, selection_rec, shader_table);
            }

            None => {
                // draw selection options
                for layer in document.layers.shallow_iter().cdir::<BackToFore>() {
                    if let Layer::Path(path) = layer {
                        let path = path.read();
                        if path.curve.bounds().is_some_and(|bounds| viewport.is_overlapping(&bounds)) {
                            path.draw_selected(d, px_world_size);
                            for pp in &path.curve.points {
                                let is_selected = selection_rec.is_some_and(|rec| rec.check_collision_point_rec(pp.p));
                                if viewport.is_overlapping_point(pp.p) {
                                    d.draw_path_point(pp, px_world_size, path.settings.color, is_selected, false, false);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
