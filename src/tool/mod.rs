use amylib::prelude::StrongMut;
use amymath::prelude::Rect2;
use raylib::prelude::*;
use crate::{raster::{raster_brush::{self, RasterBrush}, Raster}, shaders::ShaderTable, Document};

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
    fn tick(&mut self, rl: &mut RaylibHandle, thread: &RaylibThread, document: &mut Document, mouse_world_pos: Vector2);
    fn draw(&self, d: &mut impl RaylibDraw, document: &Document, shader_table: &ShaderTable, px_world_size: f32, viewport: &Rect2);
}

pub enum Tool {
    BasicSelection(BasicSelection),
    PointSelection(PointSelection),
    Pen(Pen),
    VectorBrush(VectorBrush),
    RasterBrush(RasterBrush),
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

    pub fn switch_to_vector_brush(&mut self) {
        println!("switched to vector brush");
        *self = Self::VectorBrush(VectorBrush::new());
    }

    pub fn switch_to_raster_brush(
        &mut self,
        rl: &mut RaylibHandle,
        thread: &RaylibThread,
        shader: Option<Shader>,
        target: &StrongMut<Raster>,
        stroke: raster_brush::Stroke,
    ) -> Result<(), String> {
        println!("switched to raster brush");
        *self = Self::RasterBrush(RasterBrush::new(rl, thread, shader, target.clone_mut(), stroke)?);
        Ok(())
    }
}

impl ToolType for Tool {
    fn tick(&mut self, rl: &mut RaylibHandle, thread: &RaylibThread, document: &mut Document, mouse_world_pos: Vector2) {
        match self {
            Tool::BasicSelection(tool) => tool.tick(rl, thread, document, mouse_world_pos),
            Tool::PointSelection(tool) => tool.tick(rl, thread, document, mouse_world_pos),
            Tool::Pen           (tool) => tool.tick(rl, thread, document, mouse_world_pos),
            Tool::VectorBrush   (tool) => tool.tick(rl, thread, document, mouse_world_pos),
            Tool::RasterBrush   (tool) => tool.tick(rl, thread, document, mouse_world_pos),
        }
    }

    fn draw(&self, d: &mut impl RaylibDraw, document: &Document, shader_table: &ShaderTable, px_world_size: f32, viewport: &Rect2) {
        match self {
            Tool::BasicSelection(tool) => tool.draw(d, document, shader_table, px_world_size, viewport),
            Tool::PointSelection(tool) => tool.draw(d, document, shader_table, px_world_size, viewport),
            Tool::Pen           (tool) => tool.draw(d, document, shader_table, px_world_size, viewport),
            Tool::VectorBrush   (tool) => tool.draw(d, document, shader_table, px_world_size, viewport),
            Tool::RasterBrush   (tool) => tool.draw(d, document, shader_table, px_world_size, viewport),
        }
    }
}
