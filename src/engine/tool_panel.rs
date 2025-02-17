use raylib::prelude::*;
use amymath::{prelude::IRect2, rec::RaylibIntRect2Ex};
use std::num::NonZeroUsize;
use amygui::{panel::Panel, rec::UIRect};
use crate::{appearance::{Appearance, StyleItem}, editor::Editor, shaders::ShaderTable, tool::Tool, vector_path::{fill, stroke}};

use super::color_wheel::ColorWheel;

struct ToolIconTextures {
    icon_point_selection: Texture2D,
    icon_pen: Texture2D,
    icon_vector_brush: Texture2D,
    icon_raster_brush: Texture2D,
}

impl ToolIconTextures {
    pub fn load(rl: &mut RaylibHandle, thread: &RaylibThread) -> Self {
        let mut dir = std::env::current_exe().unwrap();
        dir.pop();
        let dir = dir.join("assets");
        Self {
            icon_point_selection: rl.load_texture(thread, dir.join("icon_point_selection.png").to_str().unwrap()).unwrap(),
            icon_pen:             rl.load_texture(thread, dir.join("icon_pen.png"            ).to_str().unwrap()).unwrap(),
            icon_vector_brush:    rl.load_texture(thread, dir.join("icon_vector_brush.png"   ).to_str().unwrap()).unwrap(),
            icon_raster_brush:    rl.load_texture(thread, dir.join("icon_raster_brush.png"   ).to_str().unwrap()).unwrap(),
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
    pub color_wheel: Option<ColorWheel>,
    textures: ToolIconTextures,
}

impl ToolPanel {
    pub const ICON_GAP: i32 = 4;
    pub const ICON_INSET: i32 = 5;
    pub const BUTTON_PADDING: i32 = 3;
    pub const BUTTON_WIDTH: i32 = Self::BUTTON_PADDING * 2 + ToolIcon::WIDTH;
    const SLOT_WIDTH: i32 = ToolPanel::BUTTON_WIDTH + ToolPanel::ICON_GAP;

    pub const fn calculate_width(num_cols: i32) -> i32 {
        Self::ICON_INSET * 2 + Self::SLOT_WIDTH * num_cols - Self::ICON_GAP
    }

    pub const fn calculate_mini_palette_width(num_cols: i32) -> i32 {
        Self::SLOT_WIDTH * num_cols - Self::ICON_GAP
    }

    #[inline]
    pub fn width(&self) -> i32 {
        Self::calculate_width(self.num_cols.get() as i32)
    }

    #[inline]
    pub fn mini_palette_width(&self) -> i32 {
        Self::calculate_mini_palette_width(self.num_cols.get() as i32)
    }

    /// [fill, stroke]
    fn mini_palette_recs(&self, panel_rec: &IRect2, tools_height: i32) -> [IRect2; 2] {
        let base_width = self.mini_palette_width();
        let base_rec = IRect2 {
            xmin: panel_rec.xmin + Self::ICON_INSET,
            ymin: panel_rec.ymin + Self::ICON_INSET + tools_height,
            xmax: panel_rec.xmax - Self::ICON_INSET,
            ymax: panel_rec.ymin + Self::ICON_INSET + tools_height + base_width,
        };
        let offset = base_width / 8;
        let stroke_rec = IRect2 {
            xmin: base_rec.xmin + offset,
            ymin: base_rec.ymin + offset,
            xmax: base_rec.xmax - offset,
            ymax: base_rec.ymax - offset,
        };
        let offset = base_width / 6;
        let fill_rec = IRect2 {
            xmin: stroke_rec.xmin + offset,
            ymin: stroke_rec.ymin + offset,
            xmax: stroke_rec.xmax - offset,
            ymax: stroke_rec.ymax - offset,
        };
        [fill_rec, stroke_rec]
    }

    pub fn new(
        rl: &mut RaylibHandle,
        thread: &RaylibThread,
        panel: Panel,
        items: impl IntoIterator<Item = ToolIcon>,
    ) -> Self {
        Self {
            panel,
            num_cols: unsafe { NonZeroUsize::new_unchecked(2) },
            items: items.into_iter().collect(),
            color_wheel: None,
            textures: ToolIconTextures::load(rl, thread),
        }
    }

    pub fn num_rows(&self) -> usize {
        self.items.len() / self.num_cols
    }

    pub fn tick(&mut self, rl: &mut RaylibHandle, editor: &mut Editor, mouse_screen_pos: Vector2) {
        if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
            let panel_rec = self.panel.rect();
            let tools_height = self.num_rows() as i32 * Self::SLOT_WIDTH;
            if mouse_screen_pos.y as i32 <= panel_rec.ymin + Self::ICON_INSET + tools_height {
                // tool icons
                let (start_x, start_y) = (
                    panel_rec.xmin + Self::ICON_INSET,
                    panel_rec.ymin + Self::ICON_INSET,
                );
                let rel = Vector2 {
                    x: (mouse_screen_pos.x as f32 - start_x as f32) / Self::SLOT_WIDTH as f32,
                    y: (mouse_screen_pos.y as f32 - start_y as f32) / Self::SLOT_WIDTH as f32,
                };
                let col_row = Vector2 {
                    x: rel.x.floor(),
                    y: rel.y.floor(),
                };
                let rem = rel - col_row;
                let is_inside =
                    0.0 <= rem.x && rem.x <= (1.0 - Self::ICON_GAP as f32 / Self::SLOT_WIDTH as f32) &&
                    0.0 <= rem.y && rem.y <= (1.0 - Self::ICON_GAP as f32 / Self::SLOT_WIDTH as f32);
                if is_inside {
                    let (col, row) = (col_row.x as usize, col_row.y as usize);
                    let i = row * self.num_cols.get() + col;
                    if let Some(icon) = self.items.get(i) {
                        match icon {
                            ToolIcon::PointSelection => editor.current_tool.switch_to_point_selection(),
                            ToolIcon::Pen => editor.current_tool.switch_to_pen(),
                            ToolIcon::VectorBrush => editor.current_tool.switch_to_vector_brush(),
                            ToolIcon::RasterBrush => (/*editor.current_tool.switch_to_raster_brush(rl, thread, shader, target, stroke)*/),
                        }
                    }
                }
            } else {
                // palette
                let [fill_rec, stroke_rec] = self.mini_palette_recs(&panel_rec, tools_height);
                let mut handle_mini_palette_click = |current_appearance: &mut Appearance| {
                    if stroke_rec.is_overlapping_v(mouse_screen_pos) {
                        if fill_rec.is_overlapping_v(mouse_screen_pos) {
                            let current_fill = if let Some(current_fill) = current_appearance.items.iter_mut()
                                .find_map(|item| if let StyleItem::Fill(fill) = item { Some(fill) } else { None }) {
                                    current_fill
                                } else {
                                    current_appearance.items.push(StyleItem::Fill(fill::Fill::default()));
                                    let Some(StyleItem::Fill(new_fill)) = current_appearance.items.last_mut() else { panic!("should have just pushed a fill") };
                                    new_fill
                                };
                            println!("clicked fill");
                            self.color_wheel = Some(ColorWheel::new(Panel::new(&panel_rec, UIRect::init().with_width(100).with_height(100).build(), Color::BLACK)));
                            // match &mut current_fill.pattern {
                            //     fill::Pattern::Solid(color) => *color = Color::RED,
                            //     fill::Pattern::Gradient { .. } => todo!(),
                            // }
                        } else {
                            let current_stroke = if let Some(current_stroke) = current_appearance.items.iter_mut()
                                .find_map(|item| if let StyleItem::Stroke(stroke) = item { Some(stroke) } else { None }) {
                                    current_stroke
                                } else {
                                    current_appearance.items.push(StyleItem::Stroke(stroke::Stroke::default()));
                                    let Some(StyleItem::Stroke(new_stroke)) = current_appearance.items.last_mut() else { panic!("should have just pushed a stroke") };
                                    new_stroke
                                };
                            println!("clicked stroke");
                            self.color_wheel = Some(ColorWheel::new(Panel::new(&panel_rec, UIRect::init().with_width(100).with_height(100).build(), Color::BLACK)));
                            // match &mut current_stroke.pattern {
                            //     stroke::Pattern::Solid(color) => *color = Color::BLUE,
                            //     stroke::Pattern::Gradient { .. } => todo!(),
                            // }
                        }
                    }
                };
                if let Some(mut target_path) = editor.current_tool.target_path_mut() {
                    let mut target_borrow = target_path.write();
                    handle_mini_palette_click(&mut target_borrow.appearance);
                } else {
                    handle_mini_palette_click(&mut editor.current_appearance);
                }
            }
        }
    }

    pub fn draw(&self, d: &mut impl RaylibDraw, editor: &Editor, shader_table: &ShaderTable) {
        let panel_rec = self.panel.rect();
        let tools_height = self.num_rows() as i32 * Self::SLOT_WIDTH;
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
            let button_rec = Rectangle::new(
                button_x as f32,
                button_y as f32,
                Self::BUTTON_WIDTH as f32,
                Self::BUTTON_WIDTH as f32,
            );
            d.draw_rectangle_rounded(
                button_rec,
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
        {
            let [fill_rec, stroke_rec] = self.mini_palette_recs(&panel_rec, tools_height);
            let mut draw_mini_palette = |current_appearance: &Appearance| {
                let mut stroke_pattern = None;
                let mut fill_pattern = None;
                for item in &current_appearance.items {
                    match item {
                        StyleItem::Stroke(stroke) => stroke_pattern = Some(&stroke.pattern),
                        StyleItem::Fill(fill) => fill_pattern = Some(&fill.pattern),
                    }
                    if stroke_pattern.is_some() && fill_pattern.is_some() { break; }
                }
                let stroke_pattern = stroke_pattern.unwrap_or(&stroke::Pattern::Solid(Color::BLANK));
                let fill_pattern = fill_pattern.unwrap_or(&fill::Pattern::Solid(Color::BLANK));
                d.draw_rectangle_irect2(&stroke_rec, Color::GRAY); // in case stroke color has transparency
                stroke_pattern.draw_preview_rec(&mut d, &stroke_rec);
                d.draw_rectangle_irect2(&fill_rec, Color::GRAY); // in case fill color has transparency
                fill_pattern.draw_preview_rec(&mut d, &fill_rec);
            };
            if let Some(target_path) = editor.current_tool.target_path() {
                let target_borrow = target_path.read();
                draw_mini_palette(&target_borrow.appearance);
            } else {
                draw_mini_palette(&editor.current_appearance);
            }
        }

        if let Some(color_wheel) = &self.color_wheel {
            color_wheel.draw(&mut d, shader_table);
        }
    }
}
