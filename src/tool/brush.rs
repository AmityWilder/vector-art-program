use std::collections::VecDeque;
use raylib::prelude::*;
use crate::{layer::{Layer, LayerType, StrongLayer}, vector_path::path_point::{CtrlPoint, DistanceSqr, PathPoint}, Document};
use super::ToolType;

struct Trail2ndDerivDebugData {
    distance_sqr_pprev: f32,
    adj_length_prev: f32,
    opp_length_sqr_prev: f32,
}

struct TrailDerivDebugData {
    distance_sqr_prev: f32,
    adj_length: f32,
    opp_length_sqr: f32,
    deriv: Option<Trail2ndDerivDebugData>,
}

struct TrailDebugData {
    distance_sqr: f32,
    deriv: Option<TrailDerivDebugData>,
}

pub struct Brush {
    /// If [`Some`], continue seleted.
    /// If [`None`], find a hovered path or create a new path upon clicking.
    /// Must be a `VectorPath` layer.
    /// If there is a layer, it must not die before the pen dies.
    pub target: Option<StrongLayer>,

    /// [`Some`] while dragging, [`None`] otherwise. \
    /// (Position, Exit velocity)
    trail: Vec<(Vector2, Vector2)>,

    trail_debug: Option<TrailDebugData>,

    /// The position the last time its dot product was 1
    last_failing: Option<Vector2>,

    layer_color: Color,
}

impl Brush {
    pub fn new() -> Self {
        Self {
            target: None,
            trail: Vec::new(),
            trail_debug: None,
            last_failing: None,
            layer_color: Color::default(),
        }
    }

    fn add_trail_point(&mut self, point: Vector2, velocity: Vector2) {
        self.trail.push((point, velocity));
    }
}

const MIN_DISTANCE: f32 = 1.0;
const MIN_DISTANCE_SQR: f32 = MIN_DISTANCE * MIN_DISTANCE;
const MIN_OPP_LENGTH: f32 = 1.0;
const MIN_OPP_LENGTH_SQR: f32 = MIN_OPP_LENGTH * MIN_OPP_LENGTH;
const MIN_OPP_LENGTH_SQR_CHANGE: f32 = 1.0;
const IS_CURVATURE_SUPPORTED: bool = false;

impl ToolType for Brush {
    fn tick(&mut self, rl: &mut RaylibHandle, document: &mut Document, mouse_world_pos: Vector2, mouse_world_delta: Vector2) {
        if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
            if self.target.is_none() {
                // create a new path
                let new_path = document.create_path(None, None);
                self.layer_color = new_path.read().expect("error handling not yet implemented").settings().color;
                self.target = Some(new_path);
            }
            self.add_trail_point(mouse_world_pos, Vector2::zero());
        }

        if rl.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT) {
            let new_pos = mouse_world_pos;
            let num_points = self.trail.len();

            let mut should_add_point = true;
            let mut is_too_close = false;
            let is_too_straight;
            let is_inflecting;
            if let Some((pos_prev, _vel_prev)) = num_points.checked_sub(1).and_then(|i| self.trail.get(i)).copied() {
                let delta = new_pos - pos_prev;
                let distance_sqr = delta.length_sqr();
                self.trail_debug = Some(TrailDebugData {
                    distance_sqr,
                    deriv: None,
                });

                // change in position
                is_too_close = distance_sqr <= MIN_DISTANCE_SQR;
                if is_too_close {
                    should_add_point = false;
                } else if let Some((pos_pprev, _vel_pprev)) = num_points.checked_sub(2).and_then(|i| self.trail.get(i)).copied() {
                    let delta_prev = pos_prev - pos_pprev;
                    let distance_sqr_prev = delta_prev.length_sqr();
                    assert!(distance_sqr_prev.is_normal() && distance_sqr_prev > 0.0, "delta_prev: {delta_prev:?}");
                    let adj_length = (delta_prev / distance_sqr_prev.sqrt()).dot(delta);
                    let opp_length_sqr = distance_sqr - adj_length * adj_length;
                    self.trail_debug.as_mut().unwrap().deriv = Some(TrailDerivDebugData {
                        distance_sqr_prev,
                        adj_length,
                        opp_length_sqr,
                        deriv: None,
                    });
                    // change in direction
                    is_too_straight = adj_length >= 0.0 && opp_length_sqr <= MIN_OPP_LENGTH_SQR;
                    if is_too_straight {
                        should_add_point = false;
                    } else if IS_CURVATURE_SUPPORTED {if let Some((pos_ppprev, _vel_ppprev)) = num_points.checked_sub(3).and_then(|i| self.trail.get(i)).copied() {
                        let delta_pprev = pos_pprev - pos_ppprev;
                        let distance_sqr_pprev = delta_pprev.length_sqr();
                        // change in curvature
                        let adj_length_prev = (delta_pprev / distance_sqr_pprev.sqrt()).dot(delta_prev);
                        let opp_length_sqr_prev = distance_sqr_prev - adj_length_prev * adj_length_prev;
                        is_inflecting = (opp_length_sqr - opp_length_sqr_prev).abs() <= MIN_OPP_LENGTH_SQR_CHANGE;
                        self.trail_debug.as_mut().unwrap().deriv.as_mut().unwrap().deriv = Some(Trail2ndDerivDebugData {
                            distance_sqr_pprev,
                            adj_length_prev,
                            opp_length_sqr_prev,
                        });
                        if is_inflecting {
                            should_add_point = false;
                        }
                    }}
                }
            } else {
                println!("warning: mouse is down without having been pressed");
                should_add_point = false;
                self.trail_debug = None;
            }

            if should_add_point {
                let pos = self.last_failing.take().unwrap_or(new_pos);
                self.add_trail_point(pos, mouse_world_delta);
            } else if !is_too_close {
                self.last_failing = Some(new_pos);
            }
        }

        // stroke complete
        if rl.is_mouse_button_released(MouseButton::MOUSE_BUTTON_LEFT) {
            let target = self.target.take().expect("should have when pressed");
            let mut path = target.write().expect("error handling not yet implemented");
            let path = path.expect_path_mut("brush target should be a path");

            if let Some(last) = self.trail.last() {
                if last.0.distance_sqr_to(mouse_world_pos) > MIN_DISTANCE_SQR {
                    self.add_trail_point(mouse_world_pos, mouse_world_delta);
                }
            }

            for i in 0..self.trail.len() {
                let slice = &self.trail[i.saturating_sub(1)..(i + 1).min(self.trail.len())];
                let inv_len = (slice.len() as f32).recip();
                self.trail[i].1 = self.trail[i].1.normalized() * (slice.iter().copied().fold(0.0, |acc, (_p, v)| acc + v.length_sqr()) * inv_len).sqrt();
            }

            path.points = self.trail.iter().map(|(p, _v)| PathPoint::new(CtrlPoint::Corner, *p, CtrlPoint::Corner)).collect();

            // using a heuristic because wow this is hard
            for i in 1..self.trail.len() - 1 {
                let (prev, _) = self.trail[i - 1];
                let (curr, _) = self.trail[i];
                let (next, _) = self.trail[i + 1];
                let speed_in  = (curr - prev).length();
                let speed_out = (next - curr).length();
                let t_hat = (next - prev).normalized();
                let c_in  = curr - t_hat * speed_in  / 3.0;
                let c_out = curr + t_hat * speed_out / 3.0;
                path.points[i].c_out = CtrlPoint::Exact(c_out);
                path.points[i].c_in  = CtrlPoint::Exact(c_in);
            }

            self.last_failing = None;
            self.trail.clear();
        }
    }

    fn draw(&self, d: &mut impl RaylibDraw, document: &Document, mouse_world_pos: Vector2) {
        const SHOW_DEBUG: bool = true;

        let inv_zoom = document.camera.zoom.recip();
        for i in 1..self.trail.len() {
            d.draw_line_v(self.trail[i - 1].0, self.trail[i].0, self.layer_color);
        }
        if let Some((prev, _)) = self.trail.len().checked_sub(1).and_then(|i| self.trail.get(i)) {
            d.draw_line_v(prev, mouse_world_pos, self.layer_color);
        }
        for (p, v) in self.trail.iter().copied() {
            d.draw_circle_v(p, 3.0 * inv_zoom, self.layer_color);
            if SHOW_DEBUG {
                d.draw_line_v(p, p + v, self.layer_color.alpha(0.5));
            }
        }

        if SHOW_DEBUG {
            let p0 = mouse_world_pos;
            if let Some(((p1, _v1), data)) = self.trail.len().checked_sub(1).and_then(|i| self.trail_debug.as_ref().map(|d| (self.trail[i], d))) {
                d.draw_ring(p1, MIN_DISTANCE, MIN_DISTANCE + inv_zoom, 0.0, 360.0, 16, Color::MAGENTA.alpha(0.5));
                d.draw_line_v(p1, p1 + (p0 - p1).normalized() * MIN_DISTANCE, Color::MAGENTA.alpha(0.5));
                d.draw_line_v(p0, p0 + (p1 - p0).normalized() * data.distance_sqr.sqrt(), Color::MAGENTA);
                if let Some(((p2, _v2), data)) = self.trail.len().checked_sub(2).and_then(|i| data.deriv.as_ref().map(|d| (self.trail[i], d))) {
                    let adj_end = p1 + (p1 - p2).normalized() * data.adj_length;
                    d.draw_line_v(p1, adj_end, Color::YELLOW);
                    let opp_dir = (p0 - adj_end).normalized();
                    d.draw_line_v(adj_end, adj_end + opp_dir * MIN_OPP_LENGTH, Color::ORANGE.alpha(0.5));
                    d.draw_line_v(p1 + opp_dir * MIN_OPP_LENGTH, adj_end + opp_dir * MIN_OPP_LENGTH, Color::ORANGE.alpha(0.5));
                    d.draw_line_v(p1 - opp_dir * MIN_OPP_LENGTH, adj_end - opp_dir * MIN_OPP_LENGTH, Color::ORANGE.alpha(0.5));
                    d.draw_line_v(adj_end, adj_end + opp_dir * data.opp_length_sqr.sqrt(), Color::ORANGE);
                    if IS_CURVATURE_SUPPORTED {if let Some(((p3, _v3), data)) = self.trail.len().checked_sub(3).and_then(|i| data.deriv.as_ref().map(|d| (self.trail[i], d))) {
                        let adj_end_prev = p2 + (p2 - p3).normalized() * data.adj_length_prev;
                        let opp_dir_prev = (p2 - adj_end_prev).normalized();
                        d.draw_line_v(adj_end_prev, adj_end_prev + opp_dir_prev * MIN_OPP_LENGTH, Color::DODGERBLUE.alpha(0.5));
                        d.draw_line_v(p2 + opp_dir_prev * MIN_OPP_LENGTH, adj_end_prev + opp_dir_prev * MIN_OPP_LENGTH, Color::DODGERBLUE.alpha(0.5));
                        d.draw_line_v(p2 - opp_dir_prev * MIN_OPP_LENGTH, adj_end_prev - opp_dir_prev * MIN_OPP_LENGTH, Color::DODGERBLUE.alpha(0.5));
                        d.draw_line_v(adj_end_prev, adj_end_prev + opp_dir_prev * data.opp_length_sqr_prev.sqrt(), Color::DODGERBLUE);
                    }}
                }
            }
            // // this always just follows the mouse
            // if let Some(failed) = self.last_failing.as_ref() {
            //     d.draw_line_v(*failed + Vector2::new(-1.0, -1.0), *failed + Vector2::new(1.0,  1.0), Color::RED);
            //     d.draw_line_v(*failed + Vector2::new(-1.0,  1.0), *failed + Vector2::new(1.0, -1.0), Color::RED);
            // }
        }
    }
}
