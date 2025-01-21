use brush::Brush;
use direct_selection::DirectSelection;
use pen::Pen;
use raylib::prelude::*;
use crate::Document;

pub mod direct_selection;
pub mod pen;
pub mod brush;

pub trait ToolType {
    fn tick(&mut self, rl: &mut RaylibHandle, document: &mut Document, mouse_world_pos: Vector2, mouse_world_delta: Vector2);
    fn draw(&self, d: &mut impl RaylibDraw, document: &Document, mouse_world_pos: Vector2);
}

pub enum Tool {
    DirectSelection(DirectSelection),
    Pen(Pen),
    Brush(Brush),
}

impl Default for Tool {
    fn default() -> Self {
        Self::DirectSelection(DirectSelection::new())
    }
}

// todo: have direct selection set pen/brush target
impl Tool {
    pub fn switch_to_direct_selection(&mut self) {
        *self = Self::DirectSelection(DirectSelection::new());
    }

    pub fn switch_to_pen(&mut self) {
        *self = Self::Pen(Pen::new());
    }

    pub fn switch_to_brush(&mut self) {
        *self = Self::Brush(Brush::new());
    }
}

impl ToolType for Tool {

    fn tick(&mut self, rl: &mut RaylibHandle, document: &mut Document, mouse_world_pos: Vector2, mouse_world_delta: Vector2) {
        match self {
            Tool::DirectSelection(direct_selection) => direct_selection.tick(rl, document, mouse_world_pos, mouse_world_delta),
            Tool::Pen(pen) => pen.tick(rl, document, mouse_world_pos, mouse_world_delta),
            Tool::Brush(brush) => brush.tick(rl, document, mouse_world_pos, mouse_world_delta),
        }
    }

    fn draw(&self, d: &mut impl RaylibDraw, document: &Document, mouse_world_pos: Vector2) {
        match self {
            Tool::DirectSelection(direct_selection) => direct_selection.draw(d, document, mouse_world_pos),
            Tool::Pen(pen) => pen.draw(d, document, mouse_world_pos),
            Tool::Brush(brush) => brush.draw(d, document, mouse_world_pos),
        }
    }
}
