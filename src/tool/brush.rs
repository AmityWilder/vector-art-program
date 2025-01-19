use std::collections::VecDeque;
use raylib::prelude::*;
use crate::{layer::{Layer, LayerType, StrongLayer}, vector_path::path_point::{CtrlPoint, DistanceSqr, PathPoint}, Document};

use super::ToolType;

pub struct Brush {
    /// If [`Some`], continue seleted.
    /// If [`None`], find a hovered path or create a new path upon clicking.
    /// Must be a `VectorPath` layer.
    /// If there is a layer, it must not die before the pen dies.
    pub target: Option<StrongLayer>,

    /// [`Some`] while dragging, [`None`] otherwise.
    trail: VecDeque<Vector2>,

    /// The position the last time its dot product was 1
    last_failing: Option<Vector2>,

    layer_color: Color,
}

impl Brush {
    pub fn new() -> Self {
        Self {
            target: None,
            trail: VecDeque::with_capacity(8),
            last_failing: None,
            layer_color: Color::default(),
        }
    }

    fn add_trail_point(&mut self, point: Vector2) {
        if self.trail.len() == self.trail.capacity() {
            self.trail.pop_front();
        }
        self.trail.push_back(point);
    }
}

impl ToolType for Brush {
    fn tick(&mut self, rl: &mut RaylibHandle, document: &mut Document, mouse_world_pos: Vector2) {
        if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
            if self.target.is_none() {
                // create a new path
                let new_path = document.create_path(None, None);
                self.layer_color = new_path.read().expect("error handling not yet implemented").settings().color;
                self.target = Some(new_path);
            }
            self.trail.push_back(mouse_world_pos);
        }

        if rl.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT) {
            const MIN_DISTANCE: f32 = 3.0;
            const MIN_DISTANCE_SQR: f32 = MIN_DISTANCE * MIN_DISTANCE;
            let new_pos = mouse_world_pos;
            let num_points = self.trail.len();

            let mut is_too_close = false;
            let mut should_add_point = true;
            if let Some(&prev_pos) = num_points.checked_sub(1).and_then(|i| self.trail.get(i)) {
                let delta = new_pos - prev_pos;
                let distance_sqr = delta.length_sqr();

                // change in position
                is_too_close = distance_sqr <= MIN_DISTANCE_SQR;
                if is_too_close {
                    should_add_point = false;
                } else {
                    if let Some(&sec_prev_pos) = num_points.checked_sub(2).and_then(|i| self.trail.get(i)) {
                        let delta_prev = prev_pos - sec_prev_pos;
                        let distance_sqr_prev = delta_prev.length_sqr();
                        assert!(distance_sqr_prev.is_normal() && distance_sqr_prev > 0.0, "delta_prev: {delta_prev:?}");
                        let adj_length = (delta_prev / distance_sqr_prev.sqrt()).dot(delta);
                        // TODO
                        // if let Some(&third_prev_pos) = num_points.checked_sub(3).and_then(|i| self.trail.get(i)) {
                        //     let delta_pprev = sec_prev_pos - third_prev_pos;
                        //     let distance_sqr_pprev = delta_pprev.length_sqr();
                        //     // change in curvature
                        //     let adj_length_prev = (delta_pprev / distance_sqr_pprev.sqrt()).dot(delta_prev);
                        //     should_add_point &= (adj_length - adj_length_prev).abs() > 1.0;
                        // }
                        // change in direction
                        should_add_point &=
                            adj_length <= 0.0
                            || (distance_sqr - adj_length * adj_length > 0.8);
                    }
                    is_too_close = false
                }
            } else {
                println!("warning: mouse is down without having been pressed");
                should_add_point = false;
            };

            if should_add_point {
                let pos = self.last_failing.take().unwrap_or(new_pos);
                self.add_trail_point(pos);
            } else if !is_too_close {
                self.last_failing = Some(new_pos);
            }

            if self.trail.len() >= 3 {
                let target = self.target.as_ref().expect("should have when pressed");
                let mut path = target.write().expect("error handling not yet implemented");
                let path = path.expect_path_mut("brush target should be a path");

                // todo: use control points for smoothing
                path.points.extend(self.trail.iter().take(self.trail.len() - 3).map(|&p| PathPoint::new(CtrlPoint::Corner, p, CtrlPoint::Corner)));
                while self.trail.len() > 3 { // remove used points
                    self.trail.pop_front();
                }
            }
        }

        if rl.is_mouse_button_released(MouseButton::MOUSE_BUTTON_LEFT) {
            let target = self.target.take().expect("should have when pressed");
            let mut path = target.write().expect("error handling not yet implemented");
            let path = path.expect_path_mut("brush target should be a path");

            // todo: use control points for smoothing
            path.points.extend(self.trail.iter().map(|&p| PathPoint::new(CtrlPoint::Corner, p, CtrlPoint::Corner)));

            self.last_failing = None;
            self.trail.clear();
        }
    }

    fn draw(&self, d: &mut impl RaylibDraw, _document: &Document, mouse_world_pos: Vector2) {
        for i in 1..self.trail.len() {
            d.draw_line_v(self.trail[i - 1], self.trail[i], self.layer_color);
        }
        if let Some(prev) = self.trail.len().checked_sub(1).and_then(|i| self.trail.get(i)) {
            d.draw_line_v(prev, mouse_world_pos, self.layer_color);
        }
        for p in self.trail.iter() {
            d.draw_circle_v(p, 3.0, self.layer_color);
        }
    }
}
