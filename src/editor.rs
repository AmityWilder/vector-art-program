use std::{path::Path, time::Instant};

use amylib::prelude::DirectibleDoubleEndedIterator;
use amymath::prelude::{FlipRectangle, IRect2, MinMaxRectangle};
use raylib::prelude::*;
use crate::{appearance::Blending, document::{layer::{BackToFore, LayerType}, serialize::render_png::DownscaleAlgorithm, Document}, engine::{Config, Engine}, raster::{raster_brush, RasterTex}, shaders::ShaderTable, tool::{Tool, ToolType}};

#[allow(clippy::enum_glob_use, reason = "every frickin one of these is prefixed with its type name >:T")]
use {KeyboardKey::*, MouseButton::*};

#[repr(u8)]
enum SerializationKind {
    RenderPNG,
    SaveBinary,
    LoadBinary,
    ExportSVG,
}

fn handle_serialization(
    rl: &mut RaylibHandle,
    thread: &RaylibThread,
    document: &mut Document,
    kind: &SerializationKind,
    mouse_screen_pos: Vector2,
) {
    let start = Instant::now();
    let save_path = document.path
        .get_or_insert_with(|| Path::new("test").with_extension("amyvec"))
        .clone();
    let (result, past_tense, present_tense) =
        match kind {
            SerializationKind::RenderPNG => (
                document.render_png(
                    save_path.with_extension("png"),
                    0,
                    rl,
                    thread,
                    Some(DownscaleAlgorithm::Bicubic),
                    Color::WHITE,
                ), "rendered", "render"),

            SerializationKind::SaveBinary => (
                document.save_bin(
                    save_path.with_extension("amyvec"),
                ), "saved", "save"),

            SerializationKind::LoadBinary => (
                Document::load_bin(
                    save_path.with_extension("amyvec"),
                    mouse_screen_pos,
                ).map(|data| *document = data), "loaded", "load"),

            SerializationKind::ExportSVG => (
                document.export_svg(
                    save_path.with_extension("svg"),
                    0,
                ), "exported", "export"),
        };
    let duration = start.elapsed();
    match result {
        Ok(()) => println!("file {past_tense} successfully"),
        Err(e) => println!("failed to {present_tense} file: {e}"),
    }
    println!("  finished in {duration:?}");
}

const fn serialization_key(key: Option<KeyboardKey>) -> Option<SerializationKind> {
    match key {
        Some(KEY_R) => Some(SerializationKind::RenderPNG),
        Some(KEY_S) => Some(SerializationKind::SaveBinary),
        Some(KEY_O) => Some(SerializationKind::LoadBinary),
        Some(KEY_P) => Some(SerializationKind::ExportSVG),
        _ => None,
    }
}

pub struct Editor {
    pub document: Document,
    pub current_tool: Tool,
    /// as opposed to being in a background tab
    ///
    /// todo: this feels extremely fragile
    pub is_visible: bool,
}

impl Editor {
    pub fn new(screen_width: i32, screen_height: i32) -> Self {
        let mut document = Document::new();
        document.create_artboard(None, None, 512, 512);
        document.camera.target = Vector2::new(
            0.5 * (document.artboards[0].rect.width () - screen_width ) as f32,
            0.5 * (document.artboards[0].rect.height() - screen_height) as f32,
        );
        Self {
            document,
            current_tool: Tool::default(),
            is_visible: true,
        }
    }

    pub fn tick(&mut self, engine_config: &Config, rl: &mut RaylibHandle, thread: &RaylibThread, is_mouse_event_handled: bool, mouse_screen_pos: Vector2, mouse_screen_delta: Vector2) {
        let is_ctrl_down = rl.is_key_down(KEY_LEFT_CONTROL) || rl.is_key_down(KEY_RIGHT_CONTROL);
        let is_shift_down = rl.is_key_down(KEY_LEFT_SHIFT) || rl.is_key_down(KEY_RIGHT_SHIFT);

        if is_ctrl_down && let Some(kind) = serialization_key(rl.get_key_pressed()) {
            handle_serialization(rl, thread, &mut self.document, &kind, mouse_screen_pos);
        }

        let mouse_world_pos = rl.get_screen_to_world2D(mouse_screen_pos, self.document.camera);

        {
            let is_zooming = rl.is_key_down(KEY_LEFT_ALT);

            let mut pan = Vector2::zero();
            if rl.is_mouse_button_down(MOUSE_BUTTON_MIDDLE) {
                pan += mouse_screen_delta;
            }
            if !is_zooming {
                let mut scroll_v: Vector2 = rl.get_mouse_wheel_move_v().into();
                if rl.is_key_down(KEY_LEFT_SHIFT) {
                    (scroll_v.x, scroll_v.y) = (scroll_v.y, scroll_v.x);
                }
                pan += scroll_v * 20.0;
            }

            self.document.camera.target += (mouse_screen_delta - pan) / self.document.camera.zoom;
            self.document.camera.offset = rl.get_mouse_position();

            if is_zooming {
                const ZOOM_SPEED: f32 = 1.5;
                let amount = rl.get_mouse_wheel_move();
                if amount > 0.0 && self.document.camera.zoom < engine_config.max_zoom {
                    self.document.camera.zoom *= ZOOM_SPEED;
                } else if amount < 0.0 && self.document.camera.zoom > engine_config.min_zoom {
                    self.document.camera.zoom /= ZOOM_SPEED;
                }
            }
        }

        if rl.is_key_pressed(KEY_V) {
            self.current_tool.switch_to_point_selection()
        } else if rl.is_key_pressed(KEY_P) {
            self.current_tool.switch_to_pen()
        } else if rl.is_key_pressed(KEY_B) {
            if is_shift_down {
                let raster = RasterTex::new(rl, thread, 480, 480, Rectangle::new(0.0, 0.0, 480.0, 480.0)).unwrap();
                self.current_tool.switch_to_raster_brush(
                    rl,
                    thread,
                    self.document.create_raster(None, None, raster),
                    raster_brush::Stroke {
                        blend: Blending::default(),
                        pattern: raster_brush::Pattern::Solid(Color::BLACK),
                        thick: 10.0
                    }).unwrap()
            } else {
                self.current_tool.switch_to_vector_brush()
            }
        }

        if !is_mouse_event_handled {
            self.current_tool.tick(rl, thread, &mut self.document, mouse_world_pos);
        }

        if (is_ctrl_down) && rl.is_key_pressed(KEY_Z) {
            if let Err(e) = (if is_shift_down { Document::redo } else { Document::undo })(&mut self.document) {
                println!("error: {e:?}");
            }
        }
    }

    pub fn draw_background(&self, engine: &Engine, d: &mut RaylibDrawHandle<'_>) {
        let mut d = d.begin_mode2D(self.document.camera);

        // Artboards background
        for board in &self.document.artboards {
            d.draw_rectangle_rec(board.rect, if engine.is_trim_view { engine.config.background_color } else { self.document.paper_color });
        }
    }

    pub fn draw_trimmed(&self, d: &mut RaylibDrawHandle<'_>, trim_rtex: &RenderTexture2D, window_rec: &IRect2) {
        for board in &self.document.artboards {
            let board_rec = &board.rect;
            if board_rec.is_overlapping(window_rec) {
                let (tl_screen, br_screen) = board.get_screen_tl_br(|v| d.get_world_to_screen2D(v, self.document.camera));
                let rect_screen = tl_screen.minmax_rec(br_screen);
                let rect_screen_inv = rect_screen.flipped();
                d.draw_texture_pro(trim_rtex, rect_screen_inv, rect_screen, Vector2::zero(), 0.0, Color::WHITE);
            }
        }
    }

    pub fn draw_rendered(&self, d: &mut RaylibTextureMode<'_, RaylibDrawHandle<'_>>) {
        d.clear_background(Color::BLANK);
        {
            let mut d = d.begin_mode2D(self.document.camera);
            for layer in self.document.layers.dfs_iter(|g| !g.settings.is_hidden).cdir::<BackToFore>() {
                layer.draw_rendered(&mut d);
            }
        }
    }

    pub fn draw_foreground(&self, d: &mut RaylibDrawHandle<'_>, shader_table: &ShaderTable) {
        const FONT_SIZE: i32 = 10;
        // Artboards foreground
        for board in &self.document.artboards {
            let (tl, br) = board.get_screen_tl_br(|v| d.get_world_to_screen2D(v, self.document.camera));
            d.draw_text(&board.name, tl.x.floor() as i32, tl.y.floor() as i32 - FONT_SIZE, FONT_SIZE, Color::WHITE);
            d.draw_line_strip(&[
                tl,
                Vector2::new(tl.x, br.y),
                br,
                Vector2::new(br.x, tl.y),
                tl,
            ], Color::BLACK);
        }
        let mut d = d.begin_mode2D(self.document.camera);
        self.current_tool.draw(&mut d, &self.document, &shader_table);
    }
}
