use raylib::prelude::*;
use crate::{layer::{rc::{StrongMut, StrongRef}, Layer, LayerType}, vector_path::path_point::{Ctrl, CtrlPt1, CtrlPt2, DistanceSqr, PathPoint}, Document};
use super::ToolType;

pub struct Brush {
    /// If [`Some`], continue seleted.
    /// If [`None`], find a hovered path or create a new path upon clicking.
    /// Must be a `VectorPath` layer.
    /// If there is a layer, it must not die before the pen dies.
    pub target: Option<StrongMut<Layer>>,

    /// History of confirmed positions
    trail: Vec<Vector2>,

    /// The position the last time its dot product was 1
    last_failing: Option<Vector2>,

    layer_color: Color,
}

impl Brush {
    pub fn new() -> Self {
        Self {
            target: None,
            trail: Vec::new(),
            last_failing: None,
            layer_color: Color::default(),
        }
    }

    fn add_trail_point(&mut self, point: Vector2) {
        self.trail.push(point);
    }
}

const MIN_DISTANCE: f32 = 1.0;
const MIN_DISTANCE_SQR: f32 = MIN_DISTANCE * MIN_DISTANCE;
const MIN_OPP_LENGTH: f32 = 1.0;
const MIN_OPP_LENGTH_SQR: f32 = MIN_OPP_LENGTH * MIN_OPP_LENGTH;
const MIN_OPP_LENGTH_SQR_CHANGE: f32 = 1.0;
const IS_CURVATURE_SUPPORTED: bool = false;

impl ToolType for Brush {
    fn tick(&mut self, rl: &mut RaylibHandle, document: &mut Document, mouse_world_pos: Vector2, _mouse_world_delta: Vector2) {
        if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
            if self.target.is_none() {
                // create a new path
                let new_path = document.create_path(None, None);
                self.layer_color = new_path.read().settings().color;
                self.target = Some(new_path);
            }
            self.add_trail_point(mouse_world_pos);
        }

        if rl.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT) {
            let new_pos = mouse_world_pos;
            let num_points = self.trail.len();

            let mut should_add_point = true;
            let mut is_too_close = false;
            let is_too_straight;
            let is_inflecting;
            if let Some(pos_prev) = num_points.checked_sub(1).and_then(|i| self.trail.get(i)).copied() {
                let delta = new_pos - pos_prev;
                let distance_sqr = delta.length_sqr();

                // change in position
                is_too_close = distance_sqr <= MIN_DISTANCE_SQR;
                if is_too_close {
                    should_add_point = false;
                } else if let Some(pos_pprev) = num_points.checked_sub(2).and_then(|i| self.trail.get(i)).copied() {
                    let delta_prev = pos_prev - pos_pprev;
                    let distance_sqr_prev = delta_prev.length_sqr();
                    assert!(distance_sqr_prev.is_normal() && distance_sqr_prev > 0.0, "delta_prev: {delta_prev:?}");
                    let adj_length = (delta_prev / distance_sqr_prev.sqrt()).dot(delta);
                    let opp_length_sqr = distance_sqr - adj_length * adj_length;
                    // change in direction
                    is_too_straight = adj_length >= 0.0 && opp_length_sqr <= MIN_OPP_LENGTH_SQR;
                    if is_too_straight {
                        should_add_point = false;
                    } else if IS_CURVATURE_SUPPORTED {if let Some(pos_ppprev) = num_points.checked_sub(3).and_then(|i| self.trail.get(i)).copied() {
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

            if should_add_point {
                let pos = self.last_failing.take().unwrap_or(new_pos);
                self.add_trail_point(pos);
            } else if !is_too_close {
                self.last_failing = Some(new_pos);
            }
        }

        // stroke complete
        if rl.is_mouse_button_released(MouseButton::MOUSE_BUTTON_LEFT) {
            let mut target = self.target.take().expect("should have when pressed");
            let mut path = target.write();
            let Layer::Path(path) = &mut *path else { panic!("brush target should be a path") };

            if let Some(last) = self.trail.last() {
                if last.distance_sqr_to(mouse_world_pos) > MIN_DISTANCE_SQR {
                    self.add_trail_point(mouse_world_pos);
                }
            }

            path.points.reserve(self.trail.len());
            if let Some(p) = self.trail.first().copied() {
                path.points.push_back(PathPoint { p, ctrls: None });
            }
            for i in 1..self.trail.len() - 1 {
                let prev = self.trail[i - 1];
                let curr = self.trail[i];
                let next = self.trail[i + 1];
                let speed_in  = (curr - prev).length();
                let speed_out = (next - curr).length();
                let t_hat = (next - prev).normalized();
                let c_out = curr + t_hat * speed_out / 3.0;
                path.points.push_back(PathPoint {
                    p: curr,
                    ctrls: Some(CtrlPt1 {
                        c1: (Ctrl::Out, c_out),
                        c2: Some(CtrlPt2::Mirror(speed_in / 3.0)),
                    })
                });
            }
            if let Some(p) = self.trail.last().copied() {
                path.points.push_back(PathPoint { p, ctrls: None });
            }
            self.last_failing = None;
            self.trail.clear();
        }
    }

    fn draw(&self, d: &mut impl RaylibDraw, document: &Document, mouse_world_pos: Vector2) {
        let inv_zoom = document.camera.zoom.recip();
        for i in 1..self.trail.len() {
            d.draw_line_v(self.trail[i - 1], self.trail[i], self.layer_color);
        }
        if let Some(prev) = self.trail.len().checked_sub(1).and_then(|i| self.trail.get(i)) {
            d.draw_line_v(prev, mouse_world_pos, self.layer_color);
        }
        for p in self.trail.iter().copied() {
            d.draw_circle_v(p, 3.0 * inv_zoom, self.layer_color);
        }
    }
}
