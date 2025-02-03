use amygui::{panel::Panel, rec::UIRect};
use amymath::prelude::*;
use raylib::prelude::*;
use crate::{editor::Editor, shaders::ShaderTable, ui::layers_panel::LayersPanel};

#[allow(clippy::enum_glob_use, reason = "every frickin one of these is prefixed with its type name >:T")]
use KeyboardKey::*;

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
    pub window_rect: IRect2,
    pub is_trim_view: bool,
    shader_table: ShaderTable,
    layers_panel: LayersPanel,
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
        Self {
            config: Config::new(),
            window_rect,
            shader_table: ShaderTable::new(rl, thread).unwrap(),
            is_trim_view: false,
            layers_panel,
            editors: Vec::new(),
            active_editor: None,
        }
    }

    /// Automatically makes the new layer active and visible
    pub fn create_editor(&mut self, mut editor: Editor) -> usize {
        let idx = self.editors.len();
        editor.is_visible = true;
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
        assert!(idx.is_none_or(|i| i < self.editors.len()));
        self.active_editor = idx;
    }

    pub fn tick(&mut self, rl: &mut RaylibHandle, thread: &RaylibThread, trim_rtex: &mut RenderTexture2D) {
        let mouse_screen_pos = rl.get_mouse_position();
        let mouse_screen_delta = rl.get_mouse_delta();

        if rl.is_window_resized() {
            let (width, height) = (rl.get_screen_width(), rl.get_screen_height());
            self.window_rect.xmax = width;
            self.window_rect.ymax = height;
            self.layers_panel.panel.update_rec(&self.window_rect);
            println!("{:?}", self.layers_panel.panel.rect());

            assert!(width.is_positive() && height.is_positive());
            *trim_rtex =
                #[allow(clippy::cast_sign_loss, reason = "guarded by `width.is_positive() && height.is_positive()` assertion")]
                rl.load_render_texture(thread, width as u32, height as u32).unwrap();
        }

        if rl.is_key_pressed(KEY_T) {
            self.is_trim_view = !self.is_trim_view;
            println!("toggled trim view");
        }

        let is_hovering_layers_panel = self.layers_panel.panel.rect().is_overlapping_v(mouse_screen_pos);

        if let Some(active_editor_idx) = self.active_editor {
            assert!(active_editor_idx < self.editors.len(), "`engine.active_editor` should be a valid editor index in `engine.editors`");
            self.editors[active_editor_idx].tick(&self.config, rl, thread, is_hovering_layers_panel, mouse_screen_pos, mouse_screen_delta);
            if is_hovering_layers_panel {
                self.layers_panel.tick(rl, &mut self.editors[active_editor_idx].document, mouse_screen_pos);
            }
        }
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle<'_>, thread: &RaylibThread, trim_rtex: &mut RenderTexture2D) {
        d.clear_background(self.config.background_color);
        for editor in self.editors.iter().filter(|e| e.is_visible) {
            editor.draw_background(self, d);

            editor.draw_rendered(&mut d.begin_texture_mode(thread, trim_rtex));

            if self.is_trim_view {
                editor.draw_trimmed(d, trim_rtex, self.window_rect.into());
            } else {
                draw_artwork(d, trim_rtex);
            }

            editor.draw_foreground(d, &self.shader_table);
        }
        if let Some(active_editor_idx) = self.active_editor && let Some(active_editor) = self.editors.get(active_editor_idx) {
            self.layers_panel.draw(d, &active_editor.document);
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

    let rect = Rectangle::new(0.0, 0.0, width, height);
    let mut rect_inv = rect;
    rect_inv.height = -rect_inv.height;
    d.draw_texture_pro(trim_rtex, rect_inv, rect, Vector2::zero(), 0.0, Color::MAGENTA);
}
