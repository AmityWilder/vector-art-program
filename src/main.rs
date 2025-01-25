use std::{path::Path, time::Instant};
use layer::{rc::StrongRef, tree::TreeIterDir, LayerType};
use raylib::prelude::*;
use serialize::render_png::DownscaleAlgorithm;
// use rand::prelude::*;
use ui::{panel::{Panel, Rect2, UIBox}, specialized::layers_panel::LayersPanel};

mod vector_path;
mod raster;
mod stack;
mod appearance;
mod document;
mod tool;
mod engine;
mod editor;
mod ui;

use self::{document::*, tool::*};

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
    document.create_artboard(None, None, 512, 512);
    document.camera.target = Vector2::new(
        0.5 * (document.artboards[0].rect.width  - rl.get_screen_width ()) as f32,
        0.5 * (document.artboards[0].rect.height - rl.get_screen_height()) as f32,
    );

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

    const MIN_ZOOM_EXP: i32 = -3;
    #[allow(non_snake_case)]
    let MIN_ZOOM: f32 = 2.0f32.powi(MIN_ZOOM_EXP);

    const MAX_ZOOM_EXP: i32 = 6;
    #[allow(non_snake_case)]
    let MAX_ZOOM: f32 = 2.0f32.powi(MAX_ZOOM_EXP);

    while !rl.window_should_close() {
        let mouse_screen_pos = rl.get_mouse_position();
        let mouse_screen_delta = rl.get_mouse_delta();

        if rl.is_key_down(KeyboardKey::KEY_LEFT_CONTROL) || rl.is_key_down(KeyboardKey::KEY_RIGHT_CONTROL) {
            let is_r_pressed = rl.is_key_pressed(KeyboardKey::KEY_R);
            let is_s_pressed = rl.is_key_pressed(KeyboardKey::KEY_S);
            let is_o_pressed = rl.is_key_pressed(KeyboardKey::KEY_O);
            let is_p_pressed = rl.is_key_pressed(KeyboardKey::KEY_P);
            if is_r_pressed || is_s_pressed || is_o_pressed || is_p_pressed {
                let start = Instant::now();
                let save_path = document.path
                    .get_or_insert_with(|| Path::new("test").with_extension("amyvec").to_path_buf())
                    .clone();
                let (result, past_tense, present_tense) =
                    if is_r_pressed {
                        (document.render_png(save_path.with_extension("png"), 0, &mut rl, &thread, Some(DownscaleAlgorithm::Bicubic), Color::WHITE), "rendered", "render")
                    } else if is_s_pressed {
                        (document.save_bin(save_path.with_extension("amyvec")), "saved", "save")
                    } else if is_o_pressed {
                        (Document::load_bin(save_path.with_extension("amyvec"), mouse_screen_pos).and_then(|data| Ok(document = data)), "loaded", "load")
                    } else if is_p_pressed {
                        (document.export_svg(save_path.with_extension("svg"), 0), "exported", "export")
                    } else {
                        unreachable!()
                    };
                let duration = start.elapsed();
                match result {
                    Ok(()) => println!("file {past_tense} successfully"),
                    Err(e) => println!("failed to {present_tense} file: {e}"),
                }
                println!("  finished in {duration:?}");
            }
        }

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

        let mouse_world_pos = rl.get_screen_to_world2D(mouse_screen_pos, document.camera);

        {
            let is_zooming = rl.is_key_down(KeyboardKey::KEY_LEFT_ALT) || rl.get_gesture_pinch_vector().length_sqr() > 0.0;

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
                let scroll = rl.get_mouse_wheel_move();
                let amount = if scroll != 0.0 { scroll } else { rl.get_gesture_pinch_vector().length() };
                if amount > 0.0 && document.camera.zoom < MAX_ZOOM {
                    document.camera.zoom *= ZOOM_SPEED;
                } else if amount < 0.0 && document.camera.zoom > MIN_ZOOM {
                    document.camera.zoom /= ZOOM_SPEED;
                }
            }
        }

        if rl.is_key_pressed(KeyboardKey::KEY_V) {
            current_tool.switch_to_point_selection();
        } else if rl.is_key_pressed(KeyboardKey::KEY_P) {
            current_tool.switch_to_pen();
        } else if rl.is_key_pressed(KeyboardKey::KEY_B) {
            current_tool.switch_to_brush();
        }

        if layers_panel.panel.is_overlapping_point(mouse_screen_pos) {
            layers_panel.tick(&mut rl, &mut document, mouse_screen_pos);
        } else {
            current_tool.tick(&mut rl, &mut document, mouse_world_pos);
        }

        if rl.is_key_pressed(KeyboardKey::KEY_T) {
            is_trim_view = !is_trim_view;
        }

        if (rl.is_key_down(KeyboardKey::KEY_LEFT_CONTROL) || rl.is_key_down(KeyboardKey::KEY_RIGHT_CONTROL)) && rl.is_key_pressed(KeyboardKey::KEY_Z) {
            if rl.is_key_down(KeyboardKey::KEY_LEFT_SHIFT) || rl.is_key_down(KeyboardKey::KEY_RIGHT_SHIFT) {
                document.redo();
            } else {
                document.undo();
            }
        }

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
                        for (layer, _depth) in document.layers.tree_iter(TreeIterDir::BackToFore, |g| !g.settings.is_hidden) {
                            layer.read().draw_rendered(&mut d);
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

                current_tool.draw(&mut d, &document, mouse_world_pos);
            }

            // Draw layers panel
            layers_panel.draw(&mut d, &document);

            d.draw_fps(0, 0);
        }
    }
}
