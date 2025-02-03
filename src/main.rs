#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![allow(dead_code, reason = "many features are still stubs waiting to be implemented later")]
#![allow(incomplete_features)]

#![feature(
    stmt_expr_attributes,
    inherent_associated_types,
    let_chains,
    if_let_guard,
    vec_deque_iter_as_slices,
    vec_pop_if,
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
    generic_assert,
    // guard_patterns,
    pattern,
    never_type,
    // never_patterns,
    // negative_bounds, // internal :c
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

#[allow(clippy::enum_glob_use, reason = "every frickin one of these is prefixed with its type name >:T")]
pub use {KeyboardKey::*, MouseButton::*};

use raylib::prelude::*;
use editor::Editor;
use engine::Engine;
use self::document::{Change, Document, layer};

mod editor;
mod engine;
mod shaders;
mod vector_path;
mod raster;
mod appearance;
mod document;
mod tool;
mod ui;


fn main() {
    let (mut rl, thread) = init()
        .title("Amity Vector Art")
        .size(1280, 720)
        .resizable()
        .build();

    // maximize window
    rl.set_window_state(WindowState::set_window_maximized(rl.get_window_state(), true));
    rl.set_target_fps(60);

    let (window_width, window_height) = (rl.get_screen_width(), rl.get_screen_height());
    assert!(window_width.is_positive() && window_height.is_positive());
    let mut trim_rtex =
        #[allow(clippy::cast_sign_loss, reason = "screen cannot have negative width/height")]
        rl.load_render_texture(&thread, window_width as u32, window_height as u32).unwrap();
    let mut engine = Engine::new(&mut rl, &thread);
    engine.create_editor(Editor::new(window_width, window_height));

    while !rl.window_should_close() {
        engine.tick(&mut rl, &thread, &mut trim_rtex);
        let mut d = rl.begin_drawing(&thread);
        engine.draw(&mut d, &thread, &mut trim_rtex);
    }
}
