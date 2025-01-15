use layer::{Layer, LayerPanelTreeItem, LayerPanelTreeItemEx};
use raylib::prelude::*;
use rand::{distributions::Uniform, prelude::*};
use ui::panel::{Panel, Rect2, UIBox};

pub mod appearance;
pub mod vector_path;
pub mod bitmap;
pub mod document;
pub mod tool;
pub mod ui;

use self::{vector_path::*, bitmap::*, document::*, tool::*};

fn main() {
    let background_color: Color = Color::new(32,32,32,255);

    let mut rng = thread_rng();
    let uniform_u8 = Uniform::<u8>::new_inclusive(0, 255);

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

    let mut document = Document::new("untitled".to_string(), 256.0, 256.0);
    document.camera.target = Vector2::new(
        0.5 * (document.art_boards[0].rect.width  - rl.get_screen_width()  as f32),
        0.5 * (document.art_boards[0].rect.height - rl.get_screen_height() as f32),
    );

    let mut current_tool = Tool::default();
    let mut layers_panel = Panel::new(
        &window_rect,
        UIBox::init()
            .width(256.0)
            .from_right(0.0)
            .build(),
        background_color,
    );

    while !rl.window_should_close() {
        let mouse_screen_pos = rl.get_mouse_position();
        let mouse_screen_delta = rl.get_mouse_delta();
        let mouse_world_pos = rl.get_screen_to_world2D(mouse_screen_pos, document.camera);

        if rl.is_window_resized() {
            window_rect.xmax = rl.get_screen_width () as f32;
            window_rect.ymax = rl.get_screen_height() as f32;
            layers_panel.update_rec(&window_rect);
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

        if layers_panel.is_overlapping_point(mouse_screen_pos) {
            if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
                document.for_each_layer_tree_item(&layers_panel.rec_cache.into(), |data| -> Option<()> {
                    let LayerPanelTreeItem {
                        slot,
                        color_rec: _,
                        thumbnail_rec,
                        name_rec,
                        expand_collapse_rec,
                        ex,
                    } = data;
                    match ex {
                        LayerPanelTreeItemEx::Layer { layer } if slot.check_collision_point_rec(mouse_screen_pos) => {
                            if expand_collapse_rec.check_collision_point_rec(mouse_screen_pos) {
                                println!("clicked expand-collapse toggle");
                                layer.is_expanded = !layer.is_expanded;
                            } else if name_rec.check_collision_point_rec(mouse_screen_pos) {
                                println!("clicked name");
                            } else if thumbnail_rec.check_collision_point_rec(mouse_screen_pos) {
                                println!("clicked thumbnail");
                            } else {
                                println!("clicked slot");
                            }
                            Some(())
                        }
                        _ => None,
                    }
                });
            }
        } else {
            current_tool.tick(&mut rl, &mut document, mouse_world_pos, |document| Layer::new(
                format!("layer {}", document.layers.len()),
                Color::new(
                    uniform_u8.sample(&mut rng),
                    uniform_u8.sample(&mut rng),
                    uniform_u8.sample(&mut rng),
                    255,
                ),
            ));
        }

        {
            let mut d = rl.begin_drawing(&thread);
            d.clear_background(background_color);

            {
                let mut d = d.begin_mode2D(document.camera);

                // Artboards background
                for board in &document.art_boards {
                    d.draw_rectangle_rec(board.rect, document.paper_color);
                }

                for layer in &document.layers {
                    layer.borrow().draw_rendered(&mut d);
                }

                // Artboards foreground
                for board in &document.art_boards {
                    d.draw_text(&board.name, board.rect.x as i32, board.rect.y as i32 - 10, 10, Color::WHITE);
                    d.draw_rectangle_lines(board.rect.x as i32, board.rect.y as i32, board.rect.width as i32, board.rect.height as i32, Color::BLACK);
                }

                for layer in &document.layers {
                    layer.borrow().draw_selected(&mut d);
                }

                current_tool.draw(&mut d, &document, mouse_world_pos);
            }

            // Draw layers panel
            document.draw_layer_tree(&mut d, &layers_panel.rec_cache.into());
        }
    }
}
