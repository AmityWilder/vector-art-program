use raylib::prelude::*;
use crate::{shaders::ShaderTable, Document};

pub mod basic_selection;
pub mod point_selection;
pub mod pen;
pub mod vector_brush;

use self::{
    basic_selection::BasicSelection,
    vector_brush::VectorBrush,
    point_selection::PointSelection,
    pen::Pen,
};

pub trait ToolType {
    fn tick(&mut self, rl: &mut RaylibHandle, document: &mut Document, mouse_world_pos: Vector2);
    fn draw(&self, d: &mut impl RaylibDraw, document: &Document, mouse_world_pos: Vector2, shader_table: &ShaderTable);
}

pub enum Tool {
    BasicSelection(BasicSelection),
    PointSelection(PointSelection),
    Pen(Pen),
    Brush(VectorBrush),
}

impl Default for Tool {
    fn default() -> Self {
        Self::PointSelection(PointSelection::new())
    }
}

// todo: have direct selection set pen/brush target
impl Tool {
    pub fn switch_to_basic_selection(&mut self) {
        println!("switched to basic selection");
        *self = Self::BasicSelection(BasicSelection::new());
    }

    pub fn switch_to_point_selection(&mut self) {
        println!("switched to point selection");
        *self = Self::PointSelection(PointSelection::new());
    }

    pub fn switch_to_pen(&mut self) {
        println!("switched to pen");
        *self = Self::Pen(Pen::new());
    }

    pub fn switch_to_brush(&mut self) {
        println!("switched to brush");
        *self = Self::Brush(VectorBrush::new());
    }
}

impl ToolType for Tool {
    fn tick(&mut self, rl: &mut RaylibHandle, document: &mut Document, mouse_world_pos: Vector2) {
        match self {
            Tool::BasicSelection(tool) => tool.tick(rl, document, mouse_world_pos),
            Tool::PointSelection(tool) => tool.tick(rl, document, mouse_world_pos),
            Tool::Pen           (tool) => tool.tick(rl, document, mouse_world_pos),
            Tool::Brush         (tool) => tool.tick(rl, document, mouse_world_pos),
        }
    }

    fn draw(&self, d: &mut impl RaylibDraw, document: &Document, mouse_world_pos: Vector2, shader_table: &ShaderTable) {
        match self {
            Tool::BasicSelection(tool) => tool.draw(d, document, mouse_world_pos, shader_table),
            Tool::PointSelection(tool) => tool.draw(d, document, mouse_world_pos, shader_table),
            Tool::Pen           (tool) => tool.draw(d, document, mouse_world_pos, shader_table),
            Tool::Brush         (tool) => tool.draw(d, document, mouse_world_pos, shader_table),
        }
    }
}
