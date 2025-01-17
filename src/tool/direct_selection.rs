use raylib::prelude::*;
use crate::Document;
use super::ToolType;

pub struct DirectSelection;

impl DirectSelection {
    pub fn new() -> Self {
        Self
    }
}

impl ToolType for DirectSelection {

    fn tick(&mut self, _rl: &mut RaylibHandle, _document: &mut Document, _mouse_world_pos: Vector2) {
        // todo
    }

    fn draw(&self, _d: &mut impl RaylibDraw, _document: &Document, _mouse_world_pos: Vector2) {
        // todo
    }
}
