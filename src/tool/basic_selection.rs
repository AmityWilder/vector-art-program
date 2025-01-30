use raylib::prelude::*;
use crate::shaders::ShaderTable;
use super::ToolType;

pub enum BasicSelection {
    Inactive,
    Active {},
}

impl BasicSelection {
    pub const fn new() -> Self {
        Self::Inactive
    }
}

impl ToolType for BasicSelection {
    fn tick(&mut self, rl: &mut RaylibHandle, document: &mut crate::Document, mouse_world_pos: Vector2) {
        todo!()
    }

    fn draw(&self, d: &mut impl RaylibDraw, document: &crate::Document, mouse_world_pos: Vector2, shader_table: &ShaderTable) {
        todo!()
    }
}
