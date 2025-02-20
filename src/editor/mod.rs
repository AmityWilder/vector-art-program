use std::{path::Path, time::Instant};
use amylib::prelude::DirectibleDoubleEndedIterator;
use amymath::prelude::{*, Vector2};
use amyvec::{curve::{Curve, WidthProfile}, path_point::PathPoint};
use raylib::prelude::{*, Vector2 as RlVector2};
use undo_redo::{Action, EditHistory, RedoError, UndoError};
use crate::{appearance::{Appearance, Blending, StyleItem}, document::{layer::{BackToFore, LayerType}, serialize::render_png::DownscaleAlgorithm, Document}, engine::{Config, Engine}, raster::RasterTex, shaders::ShaderTable, tool::{raster_brush, Tool, ToolType}, vector_path::{fill, stroke}};

#[allow(clippy::enum_glob_use, reason = "every frickin one of these is prefixed with its type name >:T")]
use {KeyboardKey::*, MouseButton::*};

pub mod undo_redo;

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
        Some(KeyboardKey::KEY_R) => Some(SerializationKind::RenderPNG),
        Some(KeyboardKey::KEY_S) => Some(SerializationKind::SaveBinary),
        Some(KeyboardKey::KEY_O) => Some(SerializationKind::LoadBinary),
        Some(KeyboardKey::KEY_P) => Some(SerializationKind::ExportSVG),
        _ => None,
    }
}

pub struct Editor {
    pub document: Document,
    history: EditHistory,
    pub current_tool: Tool,
    pub current_appearance: Appearance,
}

impl Editor {
    pub fn new(screen_size: IVector2) -> Self {
        let mut document = Document::new();
        document.create_artboard(None, None, IVector2::new(512, 512));
        document.camera.target = (0.5 * (document.artboards[0].rect.size() - screen_size).as_vec2()).into();
        Self {
            document,
            history: EditHistory::with_capacity(128),
            current_tool: Tool::default(),
            current_appearance: Appearance {
                blend: Blending {
                    opacity: 0.5,
                    mode: BlendMode::BLEND_MULTIPLIED,
                },
                items: Vec::from([
                    StyleItem::Fill(fill::Fill {
                        pattern: fill::Pattern::Solid(Color::SLATEBLUE),
                        ..Default::default()
                    }),
                    StyleItem::Stroke(stroke::Stroke {
                        pattern: stroke::Pattern::Solid(Color::BLACK),
                        thick: WidthProfile::Variable({
                            let mut c = Curve::new();
                            c.points.push_back(PathPoint { p: Vector2::new(1.0, 1.0), c: None });
                            c.points.push_back(PathPoint { p: Vector2::new(10.0, 10.0), c: None });
                            c.points.push_back(PathPoint { p: Vector2::new(1.0, 1.0), c: None });
                            c
                        }),
                        ..Default::default()
                    }),
                ]),
            },
        }
    }

    pub fn push_change(&mut self, change: Action) {
        self.history.push_change(change);
    }

    pub fn undo(&mut self) -> Option<Result<(), UndoError>> {
        if let Some(change) = self.history.prev() {
            println!("undo: {change}");
            return Some(change.undo(&mut self.document));
        }
        None
    }

    pub fn redo(&mut self) -> Option<Result<(), RedoError>> {
        if let Some(change) = self.history.next() {
            println!("redo: {change}");
            return Some(change.redo(&mut self.document));
        }
        None
    }

    pub fn tick(
        &mut self,
        engine_config: &Config,
        rl: &mut RaylibHandle,
        thread: &RaylibThread,
        scratch_rtex: &mut Vec<RenderTexture2D>,
        is_mouse_event_handled: bool,
        mouse_screen_pos: Vector2,
        mouse_screen_delta: Vector2,
    ) {
        let is_ctrl_down = rl.is_key_down(KEY_LEFT_CONTROL) || rl.is_key_down(KEY_RIGHT_CONTROL);
        let is_shift_down = rl.is_key_down(KEY_LEFT_SHIFT) || rl.is_key_down(KEY_RIGHT_SHIFT);

        if is_ctrl_down && let Some(kind) = serialization_key(rl.get_key_pressed()) {
            handle_serialization(rl, thread, &mut self.document, &kind, mouse_screen_pos);
        }

        let mouse_world_pos = rl.get_screen_to_world2D(mouse_screen_pos, self.document.camera).into();

        {
            let is_zooming = rl.is_key_down(KEY_LEFT_ALT);

            let mut pan = Vector2::ZERO;
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

            self.document.camera.target += RlVector2::from((mouse_screen_delta - pan) / self.document.camera.zoom);
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
                let shader = rl.load_shader_from_memory(thread, None, Some(include_str!("../shaders/blur.frag")));
                self.current_tool.switch_to_raster_brush(
                    rl,
                    thread,
                    Some(shader),
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
            let px_world_size = self.document.camera.zoom.recip();
            self.current_tool.tick(rl, thread, &mut self.current_appearance, &mut self.document, scratch_rtex, mouse_world_pos, px_world_size);
        }

        // if (is_ctrl_down) && rl.is_key_pressed(KEY_Z) {
        //     if is_shift_down {
        //         let result = self.redo();
        //         if let Some(result) = result {
        //             if let Err(e) = result {
        //                 println!("error: {e:?}");
        //             }
        //         } else {
        //             println!("nothing to redo");
        //         }
        //     } else {
        //         let result = self.undo();
        //         if let Some(result) = result {
        //             if let Err(e) = result {
        //                 println!("error: {e:?}");
        //             }
        //         } else {
        //             println!("nothing to undo");
        //         }
        //     }
        // }
    }

    pub fn draw_background(&self, engine: &Engine, d: &mut RaylibDrawHandle<'_>) {
        let mut d = d.begin_mode2D(self.document.camera);

        // Artboards background
        for board in &self.document.artboards {
            d.draw_rectangle_irect2(&board.rect, if engine.is_trim_view { engine.config.background_color } else { self.document.paper_color });
        }
    }

    pub fn draw_trimmed(&self, d: &mut RaylibDrawHandle<'_>, trim_rtex: &RenderTexture2D, window_rec: &IRect2) {
        for board in &self.document.artboards {
            let board_rec = &board.rect;
            if board_rec.overlaps(window_rec) {
                let (Vector2 { y: top, x: left }, Vector2 { y: bottom, x: right })
                    = board.get_screen_tl_br(|v| d.get_world_to_screen2D(v, self.document.camera));
                let inv_width  = (trim_rtex.width () as f32).recip();
                let inv_height = (trim_rtex.height() as f32).recip();

                {
                    let mut d = d.rl_set_texture(trim_rtex.texture.id);
                    let mut d = d.rl_begin_quads();

                    d.rl_color4ub(255, 255, 255, 255);
                    d.rl_normal3f(0.0, 0.0, 1.0);

                    d.rl_tex_coord2f( left*inv_width, -   top*inv_height); d.rl_vertex2f( left,    top);
                    d.rl_tex_coord2f( left*inv_width, -bottom*inv_height); d.rl_vertex2f( left, bottom);
                    d.rl_tex_coord2f(right*inv_width, -bottom*inv_height); d.rl_vertex2f(right, bottom);
                    d.rl_tex_coord2f(right*inv_width, -   top*inv_height); d.rl_vertex2f(right,    top);
                }
            }
        }
    }

    pub fn draw_rendered<'a: 'b, 'b: 'c, 'c>(&self, d: &'c mut RaylibTextureMode<'b, RaylibDrawHandle<'a>>, scratch_rtex: &mut [RenderTexture2D]) {
        d.clear_background(Color::BLANK);
        {
            let mut d = d.begin_mode2D(self.document.camera);
            for layer in self.document.layers.dfs_iter(|g| !g.settings.is_hidden).cdir::<BackToFore>() {
                layer.draw_rendered(&mut d, &self.document.camera, scratch_rtex);
            }
        }
    }

    pub fn draw_foreground(&self, d: &mut RaylibDrawHandle<'_>, shader_table: &ShaderTable, viewport: &Rect2, #[cfg(dev)] mouse_world_pos: Vector2) {
        const FONT_SIZE: i32 = 10;
        // Artboards foreground
        for board in &self.document.artboards {
            let (tl, br) = board.get_screen_tl_br(|v| d.get_world_to_screen2D(v, self.document.camera));
            d.draw_text(&board.name, tl.x.floor() as i32, tl.y.floor() as i32 - FONT_SIZE, FONT_SIZE, Color::WHITE);
            d.draw_rectangle_lines_rect2(&Rect2::from_minmax(tl, br), Color::BLACK);
        }
        let mut d = d.begin_mode2D(self.document.camera);
        let px_world_size = self.document.camera.zoom.recip();
        self.current_tool.draw(&mut d, &self, &shader_table, px_world_size, viewport, #[cfg(dev)] mouse_world_pos);
    }
}
