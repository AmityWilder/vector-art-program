use raylib::prelude::*;
use amymath::rec::RaylibIntRect2Ex;
use std::num::NonZeroUsize;
use amygui::panel::Panel;
use crate::{editor::Editor, tool::Tool};

struct ToolIconTextures {
    icon_point_selection: Texture2D,
    icon_pen: Texture2D,
    icon_vector_brush: Texture2D,
    icon_raster_brush: Texture2D,
}

impl ToolIconTextures {
    pub fn load(rl: &mut RaylibHandle, thread: &RaylibThread) -> Self {
        const WIDTH: f32 = ToolIcon::WIDTH as f32;
        Self {
            icon_point_selection: {
                let mut img = Image::gen_image_color(ToolIcon::WIDTH, ToolIcon::WIDTH, Color::BLANK);
                img.draw_line_v(
                    Vector2::new(WIDTH * 0.5, 0.0),
                    Vector2::new(0.0, WIDTH),
                    Color::WHITE);
                img.draw_line_v(
                    Vector2::new(WIDTH * 0.5, 0.0),
                    Vector2::new(WIDTH, WIDTH),
                    Color::WHITE);
                img.draw_line_v(
                    Vector2::new(0.0, WIDTH),
                    Vector2::new(WIDTH, WIDTH),
                    Color::WHITE);
                rl.load_texture_from_image(thread, &img).unwrap()
            },
            icon_pen: {
                let mut img = Image::gen_image_color(ToolIcon::WIDTH, ToolIcon::WIDTH, Color::BLANK);
                img.draw_line_v(
                    Vector2::new(0.0, WIDTH),
                    Vector2::new(WIDTH, 0.0),
                    Color::WHITE);
                rl.load_texture_from_image(thread, &img).unwrap()
            },
            icon_vector_brush: {
                let mut img = Image::gen_image_color(ToolIcon::WIDTH, ToolIcon::WIDTH, Color::BLANK);
                img.draw_circle_lines_v(
                    Vector2::new(WIDTH * 0.5, WIDTH * 0.5),
                    ToolIcon::WIDTH / 2 - 1,
                    Color::WHITE);
                rl.load_texture_from_image(thread, &img).unwrap()
            },
            icon_raster_brush: {
                let mut img = Image::gen_image_color(ToolIcon::WIDTH, ToolIcon::WIDTH, Color::BLANK);
                img.draw_rectangle_lines(
                    Rectangle::new(0.0, 0.0, WIDTH, WIDTH),
                    1,
                    Color::WHITE);
                rl.load_texture_from_image(thread, &img).unwrap()
            },
        }
    }

    pub fn get(&self, icon: ToolIcon) -> &Texture2D {
        match icon {
            ToolIcon::PointSelection => &self.icon_point_selection,
            ToolIcon::Pen => &self.icon_pen,
            ToolIcon::VectorBrush => &self.icon_vector_brush,
            ToolIcon::RasterBrush => &self.icon_raster_brush,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToolIcon {
    PointSelection,
    Pen,
    VectorBrush,
    RasterBrush,
}

impl PartialEq<Tool> for ToolIcon {
    fn eq(&self, other: &Tool) -> bool {
        matches!((self, other),
            | (ToolIcon::PointSelection, Tool::PointSelection(_))
            | (ToolIcon::Pen, Tool::Pen(_))
            | (ToolIcon::VectorBrush, Tool::VectorBrush(_))
            | (ToolIcon::RasterBrush, Tool::RasterBrush(_))
        )
    }
}

impl ToolIcon {
    pub const WIDTH: i32 = 24;
}

pub struct ToolPanel {
    pub panel: Panel,
    pub num_cols: NonZeroUsize,
    pub items: Vec<ToolIcon>,
    textures: ToolIconTextures,
}

impl ToolPanel {
    pub const ICON_GAP: i32 = 4;
    pub const ICON_INSET: i32 = 5;
    pub const BUTTON_PADDING: i32 = 3;
    pub const BUTTON_WIDTH: i32 = Self::BUTTON_PADDING * 2 + ToolIcon::WIDTH;

    pub const fn width(num_cols: i32) -> i32 {
        Self::ICON_INSET * 2 + Self::BUTTON_WIDTH * num_cols + Self::ICON_GAP * (num_cols - 1)
    }

    pub fn new(rl: &mut RaylibHandle, thread: &RaylibThread, panel: Panel, items: impl IntoIterator<Item = ToolIcon>) -> Self {
        Self {
            panel,
            num_cols: unsafe { NonZeroUsize::new_unchecked(2) },
            items: items.into_iter().collect(),
            textures: ToolIconTextures::load(rl, thread),
        }
    }

    pub fn tick(&mut self, _rl: &mut RaylibHandle, _editor: &mut Editor, _mouse_screen_pos: Vector2) {

    }

    pub fn draw(&self, d: &mut impl RaylibDraw, editor: &Editor) {
        let panel_rec = self.panel.rect();
        let mut d = d.begin_scissor_mode_irect2(panel_rec);
        d.draw_rectangle_irect2(panel_rec, self.panel.background);
        let (start_x, start_y) = (
            panel_rec.xmin + Self::ICON_INSET,
            panel_rec.ymin + Self::ICON_INSET,
        );
        for (i, &icon) in self.items.iter().enumerate() {
            let (y, x) = (i / self.num_cols, i % self.num_cols);
            let (button_x, button_y) = (
                start_x + x as i32 * (Self::BUTTON_WIDTH + Self::ICON_GAP),
                start_y + y as i32 * (Self::BUTTON_WIDTH + Self::ICON_GAP),
            );
            d.draw_rectangle_rounded(
                Rectangle::new(
                    button_x as f32,
                    button_y as f32,
                    Self::BUTTON_WIDTH as f32,
                    Self::BUTTON_WIDTH as f32,
                ),
                0.125,
                3,
                if icon.eq(&editor.current_tool) { Color::DODGERBLUE } else { Color::GRAY },
            );
            let (icon_x, icon_y) = (
                button_x + Self::BUTTON_PADDING,
                button_y + Self::BUTTON_PADDING,
            );
            d.draw_texture(
                self.textures.get(icon),
                icon_x,
                icon_y,
                Color::WHITE,
            );
        }
    }
}
