use layer::{group::Group, tree::LayerIterDir, Layer, LayerType};
use raylib::prelude::*;
// use rand::prelude::*;
use ui::panel::{Panel, Rect2, UIBox};

pub mod vector_path;
pub mod raster;
pub mod stack;
pub mod appearance;
pub mod document;
pub mod tool;
pub mod ui;

use self::{document::*, tool::*};

pub struct LayersPanel {
    pub panel: Panel,
}

impl LayersPanel {
    pub const fn new(panel: Panel) -> Self {
        Self {
            panel,
        }
    }

    pub fn tick(&mut self, rl: &mut RaylibHandle, document: &mut Document, mouse_screen_pos: Vector2) {
        if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
            document.layers
                .tree_iter(LayerIterDir::ForeToBack, |group| group.is_expanded)
                .find_map(|(layer, _depth)| {
                    let mut layer = layer.borrow_mut();
                    if layer.settings().slot_rec.check_collision_point_rec(mouse_screen_pos) {
                        if let Layer::Group(Group { is_expanded, expand_button_rec, .. }) = &mut *layer {
                            if expand_button_rec.check_collision_point_rec(mouse_screen_pos) {
                                *is_expanded = !*is_expanded;
                                return Some(());
                            }
                        }
                    }
                    None
                });
        }
    }

    pub fn draw(&self, d: &mut impl RaylibDraw, document: &Document) {
        document.draw_layer_tree(d, &self.panel);
    }
}

fn main() {
    let background_color: Color = Color::new(32,32,32,255);

    // let mut rng = thread_rng();

    let (mut rl, thread) = init()
        .title("Amity Vector Art")
        .size(1280, 720)
        .resizable()
        .build();

    rl.set_target_fps(60);

    let mut window_rect = Rect2 {
        xmin: 0.0,
        ymin: 0.0,
        xmax: rl.get_screen_width () as f32,
        ymax: rl.get_screen_height() as f32,
    };

    let mut document = Document::new();
    document.create_artboard(None, None, 256.0, 256.0);
    document.camera.target = Vector2::new(
        0.5 * (document.artboards[0].rect.width  - rl.get_screen_width()  as f32),
        0.5 * (document.artboards[0].rect.height - rl.get_screen_height() as f32),
    );
    // _ = document.create_group(None, None);

    let mut is_trim_view = false;
    let mut trim_rtex = rl.load_render_texture(&thread, rl.get_screen_width() as u32, rl.get_screen_height() as u32).unwrap();

    let mut current_tool = Tool::default();
    let mut layers_panel = LayersPanel::new(
        Panel::new(
            &window_rect,
            UIBox::init()
                .width(256.0)
                .from_right(0.0)
                .build(),
            Color::new(24,24,24,255),
        ),
    );
    document.update_layer_tree_recs(&layers_panel.panel.rec_cache.into());

    while !rl.window_should_close() {
        let mouse_screen_pos = rl.get_mouse_position();
        let mouse_screen_delta = rl.get_mouse_delta();
        let mouse_world_pos = rl.get_screen_to_world2D(mouse_screen_pos, document.camera);

        if rl.is_window_resized() {
            let (width, height) = (
                rl.get_screen_width (),
                rl.get_screen_height(),
            );
            window_rect.xmax = width  as f32;
            window_rect.ymax = height as f32;
            layers_panel.panel.update_rec(&window_rect);
            trim_rtex = rl.load_render_texture(&thread, width as u32, height as u32)
                .expect("failed to load new render texture");
        }

        document.pan(
            if rl.is_mouse_button_down(MouseButton::MOUSE_BUTTON_MIDDLE) { mouse_screen_delta } else { Vector2::zero() } +
            if rl.is_key_down(KeyboardKey::KEY_LEFT_SHIFT) { Vector2::new(1.0, 0.0) } else { Vector2::new(0.0, 1.0) } * rl.get_mouse_wheel_move() * 20.0,
        );

        document.camera.target += mouse_screen_delta / document.camera.zoom;
        document.camera.offset += mouse_screen_delta;

        if rl.is_key_down(KeyboardKey::KEY_LEFT_ALT) {
            document.zoom(rl.get_mouse_wheel_move());
        }

        if rl.is_key_pressed(KeyboardKey::KEY_V) {
            current_tool.switch_to_direct_selection();
        } else if rl.is_key_pressed(KeyboardKey::KEY_P) {
            current_tool.switch_to_pen();
        }

        if layers_panel.panel.is_overlapping_point(mouse_screen_pos) {
            layers_panel.tick(&mut rl, &mut document, mouse_screen_pos);
        } else {
            current_tool.tick(&mut rl, &mut document, mouse_world_pos);
        }

        if rl.is_key_pressed(KeyboardKey::KEY_T) {
            is_trim_view = !is_trim_view;
        }

        document.update_layer_tree_recs(&layers_panel.panel.rec_cache.into());

        {
            let mut d = rl.begin_drawing(&thread);
            d.clear_background(background_color);

            {
                let mut d = d.begin_mode2D(document.camera);

                // Artboards background
                for board in &document.artboards {
                    d.draw_rectangle_rec(board.rect, document.paper_color);
                }
            }

            // Render to texture
            {
                {
                    let mut d = d.begin_texture_mode(&thread, &mut trim_rtex);
                    d.clear_background(Color::BLANK);
                    {
                        let mut d = d.begin_mode2D(document.camera);
                        for layer in document.layers.iter() {
                            layer.borrow().draw_rendered(&mut d);
                        }
                    }
                }

                if is_trim_view {
                    let window = window_rect.into();
                    for board in &document.artboards {
                        if board.rect.check_collision_recs(&window) {
                            let rect_world = board.rect;
                            let tl_world = Vector2::new(rect_world.x, rect_world.y);
                            let br_world = tl_world + Vector2::new(rect_world.width, rect_world.height);
                            let tl_screen = d.get_world_to_screen2D(tl_world, &document.camera);
                            let br_screen = d.get_world_to_screen2D(br_world, &document.camera);
                            let rect_screen = Rectangle {
                                x: tl_screen.x,
                                y: tl_screen.y,
                                width:  br_screen.x - tl_screen.x,
                                height: br_screen.y - tl_screen.y,
                            };
                            let rect_screen_inv = Rectangle {
                                x: tl_screen.x,
                                y: -br_screen.y,
                                width:  br_screen.x - tl_screen.x,
                                height: tl_screen.y - br_screen.y,
                            };
                            d.draw_texture_pro(&trim_rtex, rect_screen_inv, rect_screen, Vector2::zero(), 0.0, Color::WHITE);
                        }
                    }
                } else {
                    let rect = Rectangle::new(0.0, 0.0, trim_rtex.width() as f32, trim_rtex.height() as f32);
                    let mut rect_inv = rect;
                    rect_inv.height = -rect_inv.height;
                    d.draw_texture_pro(&trim_rtex, rect_inv, rect, Vector2::zero(), 0.0, Color::MAGENTA);
                }
            }

            {
                let mut d = d.begin_mode2D(document.camera);

                // Artboards foreground
                for board in &document.artboards {
                    d.draw_text(&board.name, board.rect.x as i32, board.rect.y as i32 - 10, 10, Color::WHITE);
                    d.draw_rectangle_lines(board.rect.x as i32, board.rect.y as i32, board.rect.width as i32, board.rect.height as i32, Color::BLACK);
                }

                for layer in document.layers.iter() {
                    layer.borrow().draw_selected(&mut d);
                }

                current_tool.draw(&mut d, &document, mouse_world_pos);
            }

            // Draw layers panel
            layers_panel.draw(&mut d, &document);
        }
    }
}
