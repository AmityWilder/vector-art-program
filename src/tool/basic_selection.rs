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
    fn tick(&mut self, _rl: &mut RaylibHandle, _thread: &RaylibThread, _document: &mut crate::Document, _mouse_world_pos: Vector2) {
        todo!()
    }

    fn draw(&self, _d: &mut impl RaylibDraw, _document: &crate::Document, _shader_table: &ShaderTable) {
        todo!()
    }
}
