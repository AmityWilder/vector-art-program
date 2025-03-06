use amymath::prelude::{*, Vector2};
use amyvec::{curve::PathPointIdx, path_point::{Ctrl, Ctrl1, Ctrl2}};
use raylib::prelude::*;
use amylib::iter::directed::DirectibleDoubleEndedIterator;
use crate::{appearance::Appearance, document::{layer::Layer, Document}, editor::Editor, layer::{BackToFore, ForeToBack, LayerType}, shaders::ShaderTable, vector_path::{path_point::PPPart, DrawPathPoint, VectorPath, ANCHOR_EXTENT_OUTER}};
use super::ToolType;

pub const HOVER_RADIUS: f32 = 6.0;
pub const HOVER_RADIUS_SQR: f32 = HOVER_RADIUS * HOVER_RADIUS;

const _: () = assert!(HOVER_RADIUS >= ANCHOR_EXTENT_OUTER);

pub const SNAP_VERT_RADIUS: f32 = 4.0;
pub const SNAP_VERT_RADIUS_SQR: f32 = SNAP_VERT_RADIUS * SNAP_VERT_RADIUS;

mod singular;
mod multiple;

use singular::SingleSelect;
use multiple::{MultiSelect, SelectionPiece};

/// Pieces should be ordered by unique target layer [`TreeIterDir::BackToFore`]. Points should be ordered by index.
enum Selection<'a> {
    Singular(SingleSelect<'a>),
    Multiple(MultiSelect<'a>),
}

struct SelectionState<'a> {
    pub selection: Selection<'a>,
    pub drag: Option<(Vector2, Vector2)>, // (start, cumulative)
}

impl<'a> SelectionState<'a> {
    pub fn drag(&mut self, delta: Vector2) {
        match &mut self.selection {
            Selection::Singular(x) => x.drag(delta),
            Selection::Multiple(x) => x.drag(delta),
        }
    }
}

pub struct PointSelection<'a> {
    state: Option<SelectionState<'a>>,
    selection_points: Option<(Vector2, Vector2)>,
}

impl<'a> PointSelection<'a> {
    pub const fn new() -> Self {
        Self {
            state: None,
            selection_points: None,
        }
    }

    /// Returns the target only if there is exactly one path selected
    pub fn only_target(&self) -> Option<&VectorPath> {
        if let Some(SelectionState { selection: Selection::Singular(single_select), .. }) = &self.state {
            return Some(single_select.target);
        }
        None
    }

    /// Returns the target only if there is exactly one path selected
    pub fn only_target_mut(&mut self) -> Option<&mut VectorPath> {
        if let Some(SelectionState { selection: Selection::Singular(single_select), .. }) = &mut self.state {
            return Some(single_select.target);
        }
        None
    }

    pub fn with_target(target: &'a mut VectorPath) -> Self {
        Self {
            state: Some(SelectionState {
                selection: Selection::Singular(SingleSelect {
                    target,
                    point: None,
                }),
                drag: None,
            }),
            selection_points: None,
        }
    }

    fn begin_dragging(&mut self, rl: &mut RaylibHandle, document: &'a mut Document, mouse_world_pos: Vector2, px_world_size: f32) {
        let hover_radius = HOVER_RADIUS * px_world_size;
        let hover_radius_sqr = hover_radius * hover_radius;
        let drag = Some((mouse_world_pos, mouse_world_pos));

        // check if clicking a point already in selection.
        // this means we want to start dragging that selection.
        if let Some(state) = self.state.as_mut() {
            match &mut state.selection {
                Selection::Singular(x) => {
                    if let Some(mut idx) = x.get_selected(mouse_world_pos, px_world_size) {
                        if matches!(idx.part, PPPart::Anchor) && rl.is_key_down(KeyboardKey::KEY_LEFT_ALT) {
                            let path = &mut *x.target;
                            let c = &mut path.curve.points[idx.point].c;
                            if let Some(Ctrl1 { c1: (c1_side, _), c2: c2 @ None }) = c {
                                *c2 = Some(Ctrl2::Exact(mouse_world_pos));
                                idx.part = PPPart::Ctrl(c1_side.opposite());
                            } else if c.is_none() {
                                let side = Ctrl::Out;
                                *c = Some(Ctrl1 { c1: (Ctrl::Out, mouse_world_pos), c2: Some(Ctrl2::Reflect) });
                                idx.part = PPPart::Ctrl(side);
                            }
                        }
                        x.point = Some(idx);
                        state.drag = drag;
                        return;
                    }
                },

                Selection::Multiple(x) => {
                    if x.is_selected(mouse_world_pos, px_world_size) {
                        state.drag = drag;
                        return;
                    }
                }
            }
        }

        // check if clicking a point at all (presumably unselected).
        // this means we want to clear the current selection and replace it with that point.
        let hovered_point = document.layers
            .shallow_iter_mut()
            .cdir::<ForeToBack>()
            .filter_map(|layer| if let Layer::Path(path) = layer { Some(path) } else { None })
            .find_map(|path| {
                let curve = &path.curve;

                if !curve.max_bounds().is_some_and(|bounds| bounds.grow(hover_radius).contains_v(&mouse_world_pos)) {
                    return None;
                }

                let idx = curve.points.iter()
                    .enumerate()
                    .filter_map(|(i, pp)| {
                        let dist = pp.p.rec_distance_to(mouse_world_pos);
                        (dist <= hover_radius).then_some((i, dist))
                    })
                    .min_by(|(_, a), (_, b)| a.partial_cmp(b).expect("distance should not be NaN"))
                    .map(|(i, _)| i);

                if let Some(idx) = idx {
                    Some((path, Some(PathPointIdx::new_anchor(idx))))
                } else if curve.slices().any(|bez| bez
                    .estimate_time_at(mouse_world_pos)
                    .is_some_and(|(_, p)| p.distance_sqr(mouse_world_pos) <= hover_radius_sqr)
                ) {
                    Some((path, None))
                } else {
                    None
                }
            });

        if let Some((hovered_target, hovered_idx)) = hovered_point {
            self.state = Some(SelectionState {
                selection: Selection::Singular(SingleSelect {
                    target: hovered_target,
                    point: hovered_idx,
                }),
                drag,
            });
        } else {
            self.state = None;
            self.selection_points = Some((mouse_world_pos, mouse_world_pos));
        }
    }

    fn end_dragging(&mut self, document: &'a mut Document, mouse_world_pos: Vector2) {
        if let Some(state) = self.state.as_mut() {
            match &mut state.selection {
                Selection::Singular(single_select) => single_select.end_dragging(document.camera.zoom.recip()),
                Selection::Multiple(multi_select) => multi_select.end_dragging(),
            }
            state.drag = None;
        }

        // finalize selection
        if let Some((selection_start, _)) = self.selection_points.take() {
            let selection_rec = Rect2::from_minmax(selection_start, mouse_world_pos);

            let mut selected = document.layers
                .shallow_iter_mut()
                .filter_map(|layer| {
                    if let Layer::Path(path) = layer {
                        let points = path.curve.points.iter()
                            .enumerate()
                            .filter_map(|(idx, pp)|
                                selection_rec.contains_v(&pp.p)
                                    .then_some(idx))
                            .collect::<Vec<usize>>();

                        if !points.is_empty() {
                            return Some(SelectionPiece { target: path, points, });
                        }
                    }
                    None
                })
                .collect::<Vec<SelectionPiece>>();

            self.state = (!selected.is_empty())
                .then(|| {
                    SelectionState {
                        selection: (|| {
                            if selected.len() == 1 {
                                let SelectionPiece { target, points, } = selected.remove(0);
                                return Selection::Singular(SingleSelect {
                                    target,
                                    point: Some(PathPointIdx::new(points[0], PPPart::Anchor)),
                                });
                            }
                            Selection::Multiple(MultiSelect { pieces: selected })
                        })(),
                        drag: None,
                    }
                });
        }
    }
}

impl<'a> ToolType<'a> for PointSelection<'a> {
    fn tick<'b: 'a>(
        &mut self,
        rl: &mut RaylibHandle,
        _thread: &RaylibThread,
        _current_appearance: &mut Appearance,
        document: &'b mut Document,
        _scratch_rtex: &mut Vec<RenderTexture2D>,
        mouse_world_pos: Vector2,
        px_world_size: f32,
    ) {
        if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
            self.begin_dragging(rl, document, mouse_world_pos, px_world_size);
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

        if (rl.is_key_pressed(KeyboardKey::KEY_DELETE) || rl.is_key_pressed(KeyboardKey::KEY_BACKSPACE)) &&
            let Some(selection_state) = self.state.as_mut()
        {
            match &mut selection_state.selection {
                Selection::Singular(single_select) => {
                    let path = &mut *single_select.target;
                    if let Some(idx) = single_select.point {
                        path.curve.points.remove(idx.point)
                            .expect("should not select a point that is not within the curve");
                    } else {
                        path.curve.points.clear();
                    }
                }
                Selection::Multiple(multi_select) => {
                    for piece in &mut multi_select.pieces {
                        let path = &mut *piece.target;
                        let mut remove = piece.points.iter().peekable();
                        let mut keep = (0..path.curve.points.len()).map(|i| remove.next_if_eq(&&i).is_none());
                        path.curve.points.retain(|_| keep.next().expect("should visit 0..len elements"));
                    }
                }
            }
            self.state = None;
        }
    }

    fn draw(&self, d: &mut impl RaylibDraw, editor: &Editor, shader_table: &ShaderTable, px_world_size: f32, viewport: &Rect2, #[cfg(dev)] _mouse_world_pos: Vector2) {
        let selection_rec = self.selection_points.as_ref().copied().map(|(start, end)|
            Rect2::from_minmax(start, end)
        );

        if let Some(selection_rec) = selection_rec.as_ref() {
            d.draw_rectangle_rect2(selection_rec, Color::BLUE.alpha(0.125));
        }

        match self.state.as_ref() {
            Some(SelectionState { selection: Selection::Singular(selected), .. }) => {
                selected.draw(d, px_world_size, shader_table);
            }

            Some(SelectionState { selection: Selection::Multiple(selected), .. }) => {
                selected.draw(d, editor, px_world_size, shader_table);
            }

            None => {
                // draw selection options
                for layer in editor.document.layers.shallow_iter().cdir::<BackToFore>() {
                    if let Layer::Path(path) = layer {
                        if path.curve.bounds().is_some_and(|bounds| viewport.overlaps(&bounds)) {
                            path.draw_selected(d, px_world_size);
                            for pp in &path.curve.points {
                                let is_selected = selection_rec.is_some_and(|rec| rec.contains_v(&pp.p));
                                if viewport.contains_v(&pp.p) {
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
