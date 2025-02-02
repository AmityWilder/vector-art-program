#![feature(
    let_chains,
    if_let_guard,
    vec_deque_iter_as_slices,
    vec_pop_if,
    can_vector,
    associated_const_equality,
    associated_type_defaults,
    // specialization,
    array_windows,
    array_chunks,
    array_repeat,
    slice_as_array,
    variant_count,
    const_for,
    const_trait_impl,
    const_format_args,
    inline_const_pat,
    const_destruct,
    // generic_const_items,
    gen_blocks,
    // generic_const_exprs,
    generic_assert,
    // guard_patterns,
    pattern,
    never_type,
    // never_patterns,
    // negative_bounds,
    negative_impls,
    trait_alias,
    auto_traits,
    marker_trait_attr,
    try_with_capacity,
    anonymous_lifetime_in_impl_trait,
    fn_traits,
    float_minimum_maximum,
    more_float_constants,
    sort_floats,
    // const_closures,
    non_zero_count_ones,
    nonzero_ops,
    optimize_attribute,
    iter_advance_by,
    iter_array_chunks,
    iter_chain,
    iter_intersperse,
    iter_is_partitioned,
    iter_collect_into,
    iterator_try_collect,
    iter_map_windows,
    lazy_get,
    lazy_cell_into_inner,
    // lazy_type_alias,
    sync_unsafe_cell,
    sealed,
    default_field_values,
    deref_pure_trait,
    maybe_uninit_array_assume_init,
    maybe_uninit_slice,
    maybe_uninit_fill,
    maybe_uninit_write_slice,
    maybe_uninit_as_bytes,
    maybe_uninit_uninit_array,
    trivial_bounds,
    bound_as_ref,
    box_patterns,
    slice_pattern,
    // deref_patterns,
    exhaustive_patterns,
    unwrap_infallible,
    unsigned_is_multiple_of,
    slice_take,
    slice_as_chunks,
    slice_range,
    box_into_boxed_slice,
    arbitrary_self_types,
    arbitrary_self_types_pointers,
)]

use std::{path::Path, time::Instant};
use amygui::{panel::Panel, rec::UIRect};
use amylib::iter::directed::DirectibleDoubleEndedIterator;
use amymath::prelude::*;
use layer::{BackToFore, LayerType};
use raylib::prelude::*;
use serialize::render_png::DownscaleAlgorithm;
use shaders::ShaderTable;
// use rand::prelude::*;
use ui::layers_panel::*;

mod shaders;
mod vector_path;
mod raster;
mod appearance;
mod document;
mod tool;
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

    rl.set_window_state(WindowState::set_window_maximized(rl.get_window_state(), true));

    rl.set_target_fps(60);

    let shader_table = ShaderTable::new(&mut rl, &thread).unwrap();

    let mut window_rect = IRect2 {
        xmin: 0,
        ymin: 0,
        xmax: rl.get_screen_width (),
        ymax: rl.get_screen_height(),
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
            UIRect::init()
                .from_right(0)
                .with_width(256)
                .build(),
            Color::new(24,24,24,255),
        ),
    );
    layers_panel.panel.update_rec(&window_rect);

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
            window_rect.xmax = width;
            window_rect.ymax = height;
            layers_panel.panel.update_rec(&window_rect);
            println!("{:?}", layers_panel.panel.rect());
            trim_rtex = rl.load_render_texture(&thread, width as u32, height as u32).unwrap();
        }

        let mouse_world_pos = rl.get_screen_to_world2D(mouse_screen_pos, document.camera);

        {
            let is_zooming = rl.is_key_down(KeyboardKey::KEY_LEFT_ALT);

            let mut pan = Vector2::zero();
            if rl.is_mouse_button_down(MouseButton::MOUSE_BUTTON_MIDDLE) {
                pan += mouse_screen_delta;
            }
            if !is_zooming {
                let mut scroll_v: Vector2 = rl.get_mouse_wheel_move_v().into();
                if rl.is_key_down(KeyboardKey::KEY_LEFT_SHIFT) {
                    (scroll_v.x, scroll_v.y) = (scroll_v.y, scroll_v.x);
                }
                pan += scroll_v * 20.0;
            }

            document.camera.target += (mouse_screen_delta - pan) / document.camera.zoom;
            document.camera.offset += mouse_screen_delta; // = rl.get_mouse_position()

            if is_zooming {
                const ZOOM_SPEED: f32 = 1.5;
                let amount = rl.get_mouse_wheel_move();
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

        if layers_panel.panel.rect().is_overlapping_point(mouse_screen_pos.x as i32, mouse_screen_pos.y as i32) {
            layers_panel.tick(&mut rl, &mut document, mouse_screen_pos);
        } else {
            current_tool.tick(&mut rl, &mut document, mouse_world_pos);
        }

        if rl.is_key_pressed(KeyboardKey::KEY_T) {
            is_trim_view = !is_trim_view;
            println!("toggled trim view");
        }

        if (rl.is_key_down(KeyboardKey::KEY_LEFT_CONTROL) || rl.is_key_down(KeyboardKey::KEY_RIGHT_CONTROL)) && rl.is_key_pressed(KeyboardKey::KEY_Z) {
            if let Err(e) = if rl.is_key_down(KeyboardKey::KEY_LEFT_SHIFT) || rl.is_key_down(KeyboardKey::KEY_RIGHT_SHIFT) {
                document.redo()
            } else {
                document.undo()
            } { println!("error: {e:?}"); }
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
                        for layer in document.layers.dfs_iter(|g| !g.settings.is_hidden).cdir::<BackToFore>() {
                            layer.draw_rendered(&mut d);
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

                let mut d = d.begin_mode2D(document.camera);

                current_tool.draw(&mut d, &document, mouse_world_pos, &shader_table);
            }

            // Draw layers panel
            layers_panel.draw(&mut d, &document);

            // d.draw_fps(0, 0);
        }
    }
}
