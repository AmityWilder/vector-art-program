use amygui::{panel::Panel, rec::UIRect};
use amymath::prelude::*;
use raylib::prelude::*;
use crate::{editor::Editor, shaders::ShaderTable, engine::{layers_panel::LayersPanel, tool_panel::{ToolIcon, ToolPanel}}};

#[allow(clippy::enum_glob_use, reason = "every frickin one of these is prefixed with its type name >:T")]
use KeyboardKey::*;

pub mod layers_panel;
pub mod color_wheel;
pub mod tool_panel;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
enum HoverRegion {
    #[default]
    Editor,
    LayersPanel,
    ToolPanel,
}

#[derive(Clone, Copy)]
pub struct Config {
    pub min_zoom: f32,
    pub max_zoom: f32,
    pub background_color: Color,
}

impl Config {
    fn new() -> Self {
        const MIN_ZOOM_EXP: i32 = -3;
        const MAX_ZOOM_EXP: i32 = 6;
        Self {
            min_zoom: 2.0f32.powi(MIN_ZOOM_EXP),
            max_zoom: 2.0f32.powi(MAX_ZOOM_EXP),
            background_color: Color::new(32,32,32,255),
        }
    }
}

pub struct Engine {
    pub config: Config,
    pub is_trim_view: bool,
    shader_table: ShaderTable,
    layers_panel: LayersPanel,
    tool_panel: ToolPanel,
    editors: Vec<Editor>,
    active_editor: Option<usize>,
}

impl Engine {
    pub fn new(rl: &mut RaylibHandle, thread: &RaylibThread) -> Self {
        const LAYER_PANEL_BACKGROUND: Color = Color::new(24,24,24,255);
        let (screen_width, screen_height) = (rl.get_screen_width(), rl.get_screen_height());
        assert!(screen_width >= 0 && screen_height >= 0);
        let window_rect = IRect2 {
            xmin: 0,
            ymin: 0,
            xmax: screen_width,
            ymax: screen_height,
        };
        let layers_panel = LayersPanel::new(
            Panel::new(
                &window_rect,
                UIRect::init()
                    .from_right(0)
                    .with_width(256)
                    .build(),
                LAYER_PANEL_BACKGROUND,
            ),
        );
        let tool_panel = ToolPanel::new(
            rl, thread,
            Panel::new(
                &window_rect,
                UIRect::init()
                    .from_left(0)
                    .with_width(ToolPanel::calculate_width(2))
                    .build(),
                LAYER_PANEL_BACKGROUND,
            ),
            [
                ToolIcon::PointSelection,
                ToolIcon::Pen,
                ToolIcon::VectorBrush,
                ToolIcon::RasterBrush,
            ]
        );
        Self {
            config: Config::new(),
            shader_table: ShaderTable::new(rl, thread).unwrap(),
            is_trim_view: false,
            layers_panel,
            tool_panel,
            editors: Vec::new(),
            active_editor: None,
        }
    }

    /// Automatically makes the new layer active and visible
    pub fn create_editor(&mut self, editor: Editor) -> usize {
        let idx = self.editors.len();
        self.editors.push(editor);
        self.active_editor = Some(idx);
        idx
    }

    pub fn editors(&self) -> &[Editor] {
        &self.editors
    }

    pub fn editors_mut(&mut self) -> &mut [Editor] {
        &mut self.editors
    }

    pub fn set_active_editor(&mut self, idx: Option<usize>) {
        assert!(idx.is_none_or(|i| i < self.editors.len()), "index out of bounds");
        self.active_editor = idx;
    }

    pub fn get_active_editor(&self) -> Option<&Editor> {
        if let Some(idx) = self.active_editor.as_ref().copied() {
            assert!(idx < self.editors.len(), "active_editor should be a valid editor index");
            Some(&self.editors[idx])
        } else { None }
    }

    pub fn get_active_editor_mut(&mut self) -> Option<&mut Editor> {
        if let Some(idx) = self.active_editor.as_ref().copied() {
            assert!(idx < self.editors.len(), "active_editor should be a valid editor index");
            Some(&mut self.editors[idx])
        } else { None }
    }

    pub fn tick(&mut self, rl: &mut RaylibHandle, thread: &RaylibThread, is_window_resized: bool, window_rect: &IRect2, mouse_screen_pos: Vector2, mouse_screen_delta: Vector2) {
        if is_window_resized {
            self.layers_panel.panel.update_rec(window_rect);
        }

        if rl.is_key_pressed(KEY_T) {
            self.is_trim_view = !self.is_trim_view;
            println!("toggled trim view");
        }

        let hover_region = if self.tool_panel.panel.rect().is_overlapping_v(mouse_screen_pos) {
            HoverRegion::ToolPanel
        } else if self.layers_panel.panel.rect().is_overlapping_v(mouse_screen_pos) {
            HoverRegion::LayersPanel
        } else {
            HoverRegion::Editor
        };

        if let Some(active_editor_idx) = self.active_editor {
            assert!(active_editor_idx < self.editors.len(), "`engine.active_editor` should be a valid editor index in `engine.editors`");
            let editor = &mut self.editors[active_editor_idx];
            editor.tick(&self.config, rl, thread, hover_region != HoverRegion::Editor, mouse_screen_pos, mouse_screen_delta);
            match hover_region {
                HoverRegion::ToolPanel => {
                    self.tool_panel.tick(rl, editor, mouse_screen_pos);
                }
                HoverRegion::LayersPanel => {
                    self.layers_panel.tick(rl, &mut editor.document, mouse_screen_pos);
                }
                HoverRegion::Editor => (), // Already handled
            }
        }
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle<'_>, thread: &RaylibThread, trim_rtex: &mut RenderTexture2D, window_rect: &IRect2, #[cfg(dev)] mouse_screen_pos: Vector2) {
        let editors = &self.editors;
        let shader_table = &self.shader_table;

        d.clear_background(self.config.background_color);
        for editor in editors {
            let viewport = Rect2 {
                min: d.get_screen_to_world2D(rvec2(window_rect.xmin, window_rect.ymin), editor.document.camera),
                max: d.get_screen_to_world2D(rvec2(window_rect.xmax, window_rect.ymax), editor.document.camera),
            };

            editor.draw_background(self, d);

            editor.draw_rendered(&mut d.begin_texture_mode(thread, trim_rtex));

            if self.is_trim_view {
                editor.draw_trimmed(d, trim_rtex, window_rect);
            } else {
                draw_artwork(d, trim_rtex);
            }

            editor.draw_foreground(d, shader_table, &viewport, #[cfg(dev)] d.get_screen_to_world2D(mouse_screen_pos, editor.document.camera));
        }

        if let Some(active_editor) = self.get_active_editor() {
            self.layers_panel.draw(d, &active_editor.document);
            self.tool_panel.draw(d, &active_editor);
        }
    }
}

fn draw_artwork(d: &mut RaylibDrawHandle<'_>, trim_rtex: &RenderTexture2D) {
    let (width, height) = (trim_rtex.width(), trim_rtex.height());

    debug_assert!(
        0 <= width  && width .ilog2() < f32::MANTISSA_DIGITS &&
        0 <= height && height.ilog2() < f32::MANTISSA_DIGITS
    );
    let (width, height) =
        #[allow(clippy::cast_precision_loss, reason = "16 million is an absurd number of pixels wide/tall")]
        (width as f32, height as f32);

    {
        let mut d = d.begin_rlgl();
        let mut d = d.rl_set_texture(trim_rtex.texture());
        let mut d = d.rl_begin_quads();

        d.rl_color4ub(255, 0, 255, 255);
        d.rl_normal3f(0.0, 0.0, 1.0);

        d.rl_tex_coord2f(0.0, 1.0);
        d.rl_vertex2f(0.0, 0.0);
        d.rl_tex_coord2f(0.0, 0.0);
        d.rl_vertex2f(0.0, height);
        d.rl_tex_coord2f(1.0, 0.0);
        d.rl_vertex2f(width, height);
        d.rl_tex_coord2f(1.0, 1.0);
        d.rl_vertex2f(width, 0.0);
    }
}
