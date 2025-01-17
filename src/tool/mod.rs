use direct_selection::DirectSelection;
use pen::Pen;
use raylib::prelude::*;
use crate::Document;

pub mod direct_selection;
pub mod pen;

pub trait ToolType {
    fn tick(&mut self, rl: &mut RaylibHandle, document: &mut Document, mouse_world_pos: Vector2);
    fn draw(&self, d: &mut impl RaylibDraw, document: &Document, mouse_world_pos: Vector2);
}

pub enum Tool {
    DirectSelection(DirectSelection),
    Pen(Pen),
}

impl Default for Tool {
    fn default() -> Self {
        Self::DirectSelection(DirectSelection::new())
    }
}

impl Tool {
    pub fn switch_to_direct_selection(&mut self) {
        *self = Self::DirectSelection(DirectSelection);
    }

    pub fn switch_to_pen(&mut self) {
        *self = Self::Pen(Pen::new());
    }
}

impl ToolType for Tool {

    fn tick(&mut self, rl: &mut RaylibHandle, document: &mut Document, mouse_world_pos: Vector2) {
        match self {
            Tool::DirectSelection(direct_selection) => direct_selection.tick(rl, document, mouse_world_pos),
            Tool::Pen(pen) => pen.tick(rl, document, mouse_world_pos),
        }
    }

    fn draw(&self, d: &mut impl RaylibDraw, document: &Document, mouse_world_pos: Vector2) {
        match self {
            Tool::DirectSelection(direct_selection) => direct_selection.draw(d, document, mouse_world_pos),
            Tool::Pen(pen) => pen.draw(d, document, mouse_world_pos),
        }
    }
}
