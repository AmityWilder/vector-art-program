#![warn(clippy::all, clippy::unwrap_used)]
#![warn(clippy::pedantic)]
#![allow(dead_code, reason = "many features are still stubs waiting to be implemented later")]
#![allow(incomplete_features)]
#![warn(unsafe_code)]

#![feature(
    stmt_expr_attributes,
    inherent_associated_types,
    let_chains,
    if_let_guard,
    vec_deque_iter_as_slices,
    can_vector,
    associated_const_equality,
    associated_type_defaults,
    specialization,
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
    generic_const_items,
    assert_matches,
    gen_blocks,
    generic_const_exprs,
    pattern,
    never_type,
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

#[allow(clippy::enum_glob_use, reason = "every frickin one of these is prefixed with its type name >:T")]
pub use {KeyboardKey::*, MouseButton::*};

use amymath::prelude::{*, Vector2};
use document::layer::Layer;
use raylib::prelude::*;
use editor::Editor;
use engine::Engine;
use raylib_rs::*;
use self::document::{Document, layer};

mod editor;
mod engine;
mod shaders;
mod vector_path;
mod raster;
mod appearance;
mod document;
mod tool;

fn main() {
    #[cfg(feature = "use-sdl")] {
        use sdl3_amity::*;
        let mut err_buf = sdl_thread().unwrap();
        let sdl = init(&mut err_buf, InitFlags::Video.union(InitFlags::Events)).unwrap();
        let mut window = sdl.create_window(&mut err_buf, c"Amity Vector Art", 1280, 720,
            WindowFlags::Opengl
                .union(WindowFlags::InputFocus)
                .union(WindowFlags::MouseFocus)
                .union(WindowFlags::MouseCapture)
                .union(WindowFlags::Resizable)
                .union(WindowFlags::Maximized)
        ).unwrap();
        let gl_ctx = window.gl_create_context(&mut err_buf).unwrap();
        gl_ctx.destroy(&mut err_buf).unwrap();

    } #[cfg(not(feature = "use-sdl"))] {
        let (mut rl, thread) = init()
            .title("Amity Vector Art")
            .size(1280, 720)
            .resizable()
            .build();

        {
            let icon_img = Image::gen_image_color(32, 32, Color::BLUEVIOLET);
            rl.set_window_icon(&icon_img);
        }

        // maximize window
        rl.set_window_state(WindowState::set_window_maximized(rl.get_window_state(), true));
        rl.set_target_fps(60);

        let mut window_rect = IRect2 {
            min: IVector2 {
                x: 0,
                y: 0,
            },
            max: IVector2 {
                x: rl.get_screen_width(),
                y: rl.get_screen_height(),
            },
        };
        let mut trim_rtex = rl.load_render_texture(&thread, window_rect.max.x as u32, window_rect.max.y as u32).unwrap();
        let mut scratch_rtex = vec![
            // at least one is needed in case a top-level layer has a blend mode
            rl.load_render_texture(&thread, window_rect.max.x as u32, window_rect.max.y as u32).unwrap(),
        ];
        let mut engine = Engine::new(&mut rl, &thread);
        engine.create_editor(Editor::new(window_rect.max));

        while !rl.window_should_close() {
            let is_window_resized = rl.is_window_resized();
            if is_window_resized {
                window_rect.max = IVector2::new(rl.get_screen_width(), rl.get_screen_height());

                trim_rtex = rl.load_render_texture(&thread, window_rect.max.x as u32, window_rect.max.y as u32).unwrap();
                for rtex in &mut scratch_rtex {
                    *rtex = rl.load_render_texture(&thread, window_rect.max.x as u32, window_rect.max.y as u32).unwrap();
                }
            }

            let mouse_screen_pos = rl.get_mouse_position();
            let mouse_screen_delta = rl.get_mouse_delta();

            engine.tick(&mut rl, &thread, is_window_resized, &mut scratch_rtex, &window_rect, Vector2::from(mouse_screen_pos), Vector2::from(mouse_screen_delta));
            {
                #[cfg(debug_assertions)]
                let mut d = rl.begin_drawing(&thread);
                engine.draw(&mut d, &thread, &mut trim_rtex, &mut scratch_rtex[..], &window_rect, #[cfg(dev)] mouse_screen_pos);

                // debug
                {
                    const DRAW_DEBUG: bool = false;

                    if DRAW_DEBUG {
                        if let Some(editor) = engine.get_active_editor() {
                            let mut d = d.begin_mode2D(editor.document.camera);
                            for layer in editor.document.layers.dfs_iter(|_| true) {
                                if let Layer::Path(path) = layer {
                                    let path = path.read();
                                    if let Some(bounds) = path.curve.bounds() {
                                        d.draw_rectangle_lines_rect2(&bounds, Color::MAGENTA);
                                    }
                                    for bounds in path.curve.slices().map(|bez| bez.bounds()) {
                                        d.draw_rectangle_lines_rect2(&bounds, Color::BLUE);
                                    }
                                }
                            }
                        }

                        d.draw_fps(0, 0);
                    }
                }
            }
        }
    }
}
