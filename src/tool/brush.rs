use std::collections::VecDeque;
use raylib::prelude::*;
use crate::{layer::{Layer, LayerType, StrongLayer}, vector_path::path_point::{CtrlPoint, PathPoint}, Document};

use super::ToolType;

pub struct Brush {
    /// If [`Some`], continue seleted.
    /// If [`None`], find a hovered path or create a new path upon clicking.
    /// Must be a `VectorPath` layer.
    /// If there is a layer, it must not die before the pen dies.
    pub target: Option<StrongLayer>,

    /// [`Some`] while dragging, [`None`] otherwise.
    pub trail: VecDeque<Vector2>,

    layer_color: Color,
}

impl Brush {
    pub fn new() -> Self {
        Self {
            target: None,
            trail: VecDeque::with_capacity(256),
            layer_color: Color::default(),
        }
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
            if self.trail.len() == self.trail.capacity() {
                self.trail.pop_front();
            }
            self.trail.push_back(mouse_world_pos);
        }

        if rl.is_mouse_button_released(MouseButton::MOUSE_BUTTON_LEFT) {


            self.trail.clear();
        }
    }

    fn draw(&self, d: &mut impl RaylibDraw, _document: &Document, _mouse_world_pos: Vector2) {
        for i in 1..self.trail.len() {
            d.draw_line_v(self.trail[i - 1], self.trail[i], self.layer_color);
        }
        for p in self.trail.iter() {
            d.draw_circle_v(p, 3.0, self.layer_color);
        }
    }
}
