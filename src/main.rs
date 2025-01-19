use std::time::Instant;

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
                    let mut layer = layer.write().expect("error handling not yet implemented");
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
    document.create_artboard(None, None, 256, 256);
    document.camera.target = Vector2::new(
        0.5 * (document.artboards[0].rect.width  - rl.get_screen_width ()) as f32,
        0.5 * (document.artboards[0].rect.height - rl.get_screen_height()) as f32,
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

        let is_holding_ctrl = rl.is_key_down(KeyboardKey::KEY_LEFT_CONTROL) || rl.is_key_down(KeyboardKey::KEY_RIGHT_CONTROL);
        let is_pressing_s = rl.is_key_pressed(KeyboardKey::KEY_S);
        let is_pressing_o = rl.is_key_pressed(KeyboardKey::KEY_O);
        let is_pressing_p = rl.is_key_pressed(KeyboardKey::KEY_P);
        let is_pressing_r = rl.is_key_pressed(KeyboardKey::KEY_R);
        if (is_pressing_s || is_pressing_o || is_pressing_p || is_pressing_r) && is_holding_ctrl {
            if is_pressing_r {
                // cannot be done without blocking
                match document.render_png("test.png", 0, &mut rl, &thread) {
                    Ok(()) => println!("file rendered successfully"),
                    Err(e) => println!("failed to render file: {e}"),
                }
            } else {
                let mut handle_serialization = || {
                    let start = Instant::now();
                    if is_pressing_s {
                        match document.save_bin("test.amyvec") {
                            Ok(()) => println!("file saved successfully"),
                            Err(e) => println!("failed to save file: {e}"),
                        }
                    } else if is_pressing_o {
                        match Document::load_bin("test.amyvec", mouse_screen_pos) {
                            Ok(data) => {
                                document = data;
                                println!("file loaded successfully");
                            },
                            Err(e) => println!("failed to load file: {e}"),
                        }
                    } else if is_pressing_p {
                        match document.export_svg("test.svg", 0) {
                            Ok(()) => println!("file exported successfully"),
                            Err(e) => println!("failed to export file: {e}"),
                        }
                    } else {
                        unimplemented!()
                    }
                    println!("  finished in {:?}", start.elapsed());
                };
                const IS_SERIALIZATION_NON_BLOCKING: bool = false;
                if IS_SERIALIZATION_NON_BLOCKING {
                    std::thread::scope(|s| {
                        let task = s.spawn(|| handle_serialization());
                        let msg = if is_pressing_s {
                            "saving..."
                        } else if is_pressing_o {
                            "loading..."
                        } else {
                            unimplemented!()
                        };
                        const FONT_SIZE: i32 = 10;
                        const FONT_HALF_SIZE: i32 = FONT_SIZE / 2;
                        let msg_half_width = rl.measure_text(msg, FONT_SIZE) / 2;
                        while !task.is_finished() {
                            let mut d = rl.begin_drawing(&thread);
                            d.clear_background(background_color);
                            d.draw_text(
                                msg,
                                d.get_screen_width() / 2 - msg_half_width,
                                d.get_screen_height() / 2 - FONT_HALF_SIZE,
                                FONT_SIZE,
                                Color::GRAY,
                            );
                        }
                    });
                } else {
                    handle_serialization();
                }
            }
        }

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

        {
            let is_zooming = rl.is_key_down(KeyboardKey::KEY_LEFT_ALT);

            let mut pan = Vector2::zero();
            if rl.is_mouse_button_down(MouseButton::MOUSE_BUTTON_MIDDLE) {
                pan += mouse_screen_delta;
            }
            if !is_zooming {
                pan += (if rl.is_key_down(KeyboardKey::KEY_LEFT_SHIFT) {
                    Vector2::new(1.0, 0.0)
                } else {
                    Vector2::new(0.0, 1.0)
                }) * rl.get_mouse_wheel_move() * 20.0
            }
            document.camera.target -= pan / document.camera.zoom;

            document.camera.target += mouse_screen_delta / document.camera.zoom;
            document.camera.offset += mouse_screen_delta; // = rl.get_mouse_position()

            if is_zooming {
                const ZOOM_SPEED: f32 = 1.5;
                let amount = rl.get_mouse_wheel_move();
                if amount > 0.0 && document.camera.zoom < 16.0 {
                    document.camera.zoom *= ZOOM_SPEED;
                } else if amount < 0.0 && document.camera.zoom > 0.125 {
                    document.camera.zoom /= ZOOM_SPEED;
                }
            }
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
                    d.draw_rectangle_rec(board.rect, if !is_trim_view { document.paper_color } else { background_color });
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
                            layer.read().expect("error handling not yet implemented").draw_rendered(&mut d);
                        }
                    }
                }

                if is_trim_view {
                    let window = window_rect.into();
                    for board in &document.artboards {
                        let rect_world = Rectangle::from(board.rect);
                        if rect_world.check_collision_recs(&window) {
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
                // Artboards foreground
                for board in &document.artboards {
                    let   left_world = board.rect.x as f32;
                    let    top_world = board.rect.y as f32;
                    let  right_world = (board.rect.x + board.rect.width ) as f32;
                    let bottom_world = (board.rect.y + board.rect.height) as f32;
                    let Vector2 { x:  left_screen, y:    top_screen } = d.get_world_to_screen2D(Vector2::new( left_world,    top_world), &document.camera);
                    let Vector2 { x: right_screen, y: bottom_screen } = d.get_world_to_screen2D(Vector2::new(right_world, bottom_world), &document.camera);
                    d.draw_text(&board.name, left_screen as i32, top_screen as i32 - 10, 10, Color::WHITE);
                    d.draw_line_strip(&[
                        Vector2::new( left_screen,    top_screen),
                        Vector2::new(right_screen,    top_screen),
                        Vector2::new(right_screen, bottom_screen),
                        Vector2::new( left_screen, bottom_screen),
                        Vector2::new( left_screen,    top_screen),
                    ], Color::BLACK);
                }

                // todo: make all ui elements draw without 2D mode
                let mut d = d.begin_mode2D(document.camera);

                // todo: use draw_selected only on selection
                match current_tool {
                    Tool::DirectSelection(_) => for layer in document.layers.iter() {
                        layer.read().expect("error handling not yet implemented").draw_selected(&mut d);
                    }
                    _ => (),
                }

                current_tool.draw(&mut d, &document, mouse_world_pos);
            }

            // Draw layers panel
            layers_panel.draw(&mut d, &document);
        }
    }
}
