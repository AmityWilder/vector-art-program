use std::{collections::VecDeque, fmt};
use raylib::prelude::*;
use amylib::rc::*;
use amymath::prelude::*;
use crate::{layer::LayerType, shaders::ShaderTable, vector_path::{path_point::{Ctrl, Ctrl1, Ctrl2, PathPoint}, VectorPath, DrawPathPoint}, Change, Document};
use super::{point_selection::SNAP_VERT_RADIUS_SQR, ToolType};

struct BrushAction {
    target: StrongMut<VectorPath>,
    stroke: VecDeque<PathPoint>,
}

impl fmt::Debug for BrushAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("BrushAction").finish()
    }
}

impl Change for BrushAction {
    fn redo(&mut self, _document: &mut Document) -> Result<(), String> {
        self.target.write().curve.points.clone_from(&self.stroke);
        Ok(())
    }

    fn undo(&mut self, _document: &mut Document) -> Result<(), String> {
        self.target.write().curve.points.clear();
        Ok(())
    }
}

pub enum VectorBrush {
    Inactive(Option<StrongMut<VectorPath>>),
    Active {
        /// If [`Some`], continue seleted.
        /// If [`None`], find a hovered path or create a new path upon clicking.
        /// Must be a `VectorPath` layer.
        /// If there is a layer, it must not die before the pen dies.
        target: StrongMut<VectorPath>,

        /// History of confirmed positions
        trail: Vec<Vector2>,

        /// The position the last time its dot product was 1
        last_failing: Option<Vector2>,
    }
}

impl VectorBrush {
    pub fn new() -> Self {
        Self::Inactive(None)
    }
}

const MIN_DISTANCE: f32 = 5.0;
const MIN_DISTANCE_SQR: f32 = MIN_DISTANCE * MIN_DISTANCE;
const MIN_OPP_LENGTH: f32 = 1.0;
const MIN_OPP_LENGTH_SQR: f32 = MIN_OPP_LENGTH * MIN_OPP_LENGTH;
const MIN_OPP_LENGTH_SQR_CHANGE: f32 = 1.0;
const IS_CURVATURE_SUPPORTED: bool = false;

fn is_position_changed(last_changed: &Vector2, p: &Vector2) -> bool {
    todo!()
}

fn is_direction_changed(last_straight: &Vector2, last_changed: &Vector2, p: &Vector2) -> bool {
    todo!()
}

fn is_curvature_changed(last_curved: &Vector2, last_straight: &Vector2, last_changed: &Vector2, p: &Vector2) -> bool {
    todo!()
}

impl ToolType for VectorBrush {
    fn tick(&mut self, rl: &mut RaylibHandle, document: &mut Document, mouse_world_pos: Vector2) {
        if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) && matches!(self, Self::Inactive(_)) {
            // create a new path
            *self = Self::Active {
                target: document.create_path(None, None).clone_mut(),
                trail: Vec::new(),
                last_failing: None,
            };
        }

        if let Self::Active { target, trail, last_failing } = self {
            if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
                trail.push(mouse_world_pos);
                let mut path = target.write();
                for _ in 0..2 {
                    path.curve.points.push_back(PathPoint { p: mouse_world_pos, c: None });
                }
            }

            if rl.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT) {
                let new_pos = mouse_world_pos;
                let num_points = trail.len();

                let mut should_add_point = true;
                let mut is_too_close = false;
                let is_too_straight;
                let is_inflecting;
                if let Some(pos_prev) = num_points.checked_sub(1).and_then(|i| trail.get(i)).copied() {
                    let delta = new_pos - pos_prev;
                    let distance_sqr = delta.length_sqr();

                    // change in position
                    is_too_close = distance_sqr <= MIN_DISTANCE_SQR;
                    if is_too_close {
                        should_add_point = false;
                    } else if let Some(pos_pprev) = num_points.checked_sub(2).and_then(|i| trail.get(i)).copied() {
                        let delta_prev = pos_prev - pos_pprev;
                        let distance_sqr_prev = delta_prev.length_sqr();
                        assert!(distance_sqr_prev.is_normal() && distance_sqr_prev > 0.0, "delta_prev: {delta_prev:?}");
                        let adj_length = (delta_prev / distance_sqr_prev.sqrt()).dot(delta);
                        let opp_length_sqr = distance_sqr - adj_length * adj_length;
                        // change in direction
                        is_too_straight = adj_length >= 0.0 && opp_length_sqr <= MIN_OPP_LENGTH_SQR;
                        if is_too_straight {
                            should_add_point = false;
                        } else if IS_CURVATURE_SUPPORTED {if let Some(pos_ppprev) = num_points.checked_sub(3).and_then(|i| trail.get(i)).copied() {
                            let delta_pprev = pos_pprev - pos_ppprev;
                            let distance_sqr_pprev = delta_pprev.length_sqr();
                            // change in curvature
                            let adj_length_prev = (delta_pprev / distance_sqr_pprev.sqrt()).dot(delta_prev);
                            let opp_length_sqr_prev = distance_sqr_prev - adj_length_prev * adj_length_prev;
                            is_inflecting = (opp_length_sqr - opp_length_sqr_prev).abs() <= MIN_OPP_LENGTH_SQR_CHANGE;
                            if is_inflecting {
                                should_add_point = false;
                            }
                        }}
                    }
                } else {
                    println!("warning: mouse is down without having been pressed");
                    should_add_point = false;
                }

                {
                    let mut path = target.write();

                    if let Some(back) = path.curve.points.back_mut() {
                        back.p = new_pos;
                    }
                    if let Some(idx) = path.curve.points.len().checked_sub(3) {
                        let (prev, curr, next) = (path.curve.points[idx].p, path.curve.points[idx + 1].p, path.curve.points[idx + 2].p);
                        let speed_in  = (curr - prev).length();
                        let speed_out = (next - curr).length();
                        let t_hat = (next - prev).normalized();
                        let c_out = curr + t_hat * speed_out / 3.0;
                        {
                            let curr = &mut path.curve.points[idx + 1].c;
                            *curr = Some(Ctrl1 {
                                c1: (Ctrl::Out, c_out),
                                c2: Some(Ctrl2::Mirror(speed_in / 3.0)),
                            });
                        }
                    }
                    // join points confirmed to be no longer editing
                    if let Some(idx) = path.curve.points.len().checked_sub(4) {
                        if path.curve.points[idx].p.distance_sqr_to(path.curve.points[idx + 1].p) < 0.001 {
                            let b = path.curve.points.remove(idx + 1).expect("checked sub should ensure element existence");
                            let a = &mut path.curve.points[idx];
                            println!("merging points\n  {a:?}\n  {b:?}");
                            if let Some((c_a, c_b)) = a.c.as_mut().zip(b.c) {
                                assert_eq!((c_a.c1.0, c_b.c1.0), (Ctrl::Out, Ctrl::Out), "brush should be producing Out ctrl only");
                                c_a.c1.1 = a.p + c_b.c1.1 - b.p; // merge output velocity, keep input velocity
                            }
                            println!("result:\n  {a:?}");
                        }
                    }
                }

                if should_add_point {
                    let pos = last_failing.take().unwrap_or(new_pos);
                    trail.push(pos);

                    target.write().curve.points.push_back(PathPoint { p: new_pos, c: None });
                } else if !is_too_close {
                    *last_failing = Some(new_pos);
                }
            }

            // stroke complete
            if rl.is_mouse_button_released(MouseButton::MOUSE_BUTTON_LEFT) {
                let stroke = {
                    let mut path = target.write();

                    if let Some(last) = trail.last() {
                        if last.distance_sqr_to(mouse_world_pos) > MIN_DISTANCE_SQR {
                            trail.push(mouse_world_pos);
                        }
                    }

                    if let Some(p) = trail.last().copied() {
                        path.curve.points.push_back(PathPoint { p, c: None });
                    }

                    path.curve.points.clone()
                };
                document.push_change(
                    Box::new(BrushAction {
                        target: target.clone_mut(),
                        stroke,
                    })
                );
                *self = Self::Inactive(None);
            }
        }
    }

    fn draw(&self, d: &mut impl RaylibDraw, document: &Document, _mouse_world_pos: Vector2, shader_table: &ShaderTable) {
        if let Self::Active { target, .. } = self {
            let px_world_size = document.camera.zoom.recip();
            let path = target.read();
            path.draw_selected(d, px_world_size);
            let color = path.settings.color;
            for pp in &path.curve.points {
                d.draw_path_point(pp, px_world_size, color, false, true, true);
            }
        }
    }
}
