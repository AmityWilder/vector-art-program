use amymath::prelude::Vector2;
use raylib::prelude::*;
use amygui::panel::Panel;
use crate::shaders::ShaderTable;

struct EditorHSLWheel {
    h: f32,
    s: f32,
    l: f32,
}

impl EditorHSLWheel {
    pub fn draw() {

    }
}

struct EditorHSL {
    h: f32,
    s: f32,
    l: f32,
}

impl EditorHSL {

}

struct EditorRGB {
    r: f32,
    g: f32,
    b: f32,
}

impl EditorRGB {

}

struct EditorLAB {
    l: f32,
    a: f32,
    b: f32,
}

impl EditorLAB {

}

enum Format {
    HSLWheel(EditorHSLWheel),
    HSL(EditorHSL),
    RGB(EditorRGB),
    LAB(EditorLAB),
}

pub struct ColorWheel {
    pub panel: Panel,
    form: Format,
}

impl ColorWheel {
    pub const fn new(panel: Panel) -> Self {
        Self {
            panel,
            form: Format::HSLWheel(EditorHSLWheel { h: 0.0, s: 0.0, l: 0.0 }),
        }
    }

    pub fn tick(&mut self, _mouse_screen_pos: Vector2) {
        // todo
    }

    pub fn draw(&self, d: &mut impl RaylibDraw, shader_table: &ShaderTable) {
        let (center, extent) = self.panel.rect().as_rect2().center_and_extent();
        let radius = extent.x.min(extent.y);
        let mut d = d.begin_shader_mode(&shader_table.hue_wheel);
        shader_table.draw_uv_tex(&mut d, center, radius, Color::WHITE);
    }
}
