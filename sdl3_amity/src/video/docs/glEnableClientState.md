# glEnableClientState
enable or disable client-side capability

## Parameters
- `cap`
  Specifies the capability to enable. Symbolic constants
  [`gl::COLOR_ARRAY`], [`gl::EDGE_FLAG_ARRAY`], [`gl::FOG_COORD_ARRAY`],
  [`gl::INDEX_ARRAY`], [`gl::NORMAL_ARRAY`],
  [`gl::SECONDARY_COLOR_ARRAY`], [`gl::TEXTURE_COORD_ARRAY`], and
  [`gl::VERTEX_ARRAY`] are accepted.

## Parameters
- `cap`
  Specifies the capability to disable.

## Description
[`Gl::enable_client_state`] and [`Gl::disable_client_state`] enable or
  disable individual client-side capabilities. By default, all client-
  side capabilities are disabled. Both [`Gl::enable_client_state`] and
  [`Gl::disable_client_state`] take a single argument, `cap`, which can
  assume one of the following values:

## Notes
[`Gl::enable_client_state`] is available only if the GL version is 1.1
  or greater.
[`gl::FOG_COORD_ARRAY`] and [`gl::SECONDARY_COLOR_ARRAY`] are
  available only if the GL version is 1.4 or greater.
For OpenGL versions 1.3 and greater, or when ```c ARB_multitexture ```
  is supported, enabling and disabling [`gl::TEXTURE_COORD_ARRAY`]
  affects the active client texture unit. The active client texture unit
  is controlled with [`Gl::client_active_texture`].

## Errors
- [`gl::INVALID_ENUM`] is generated if `cap` is not an accepted value.
- [`Gl::enable_client_state`] is not allowed between the execution of
  [`Gl::begin`] and the corresponding [`Gl::end`], but an error may or
  may not be generated. If no error is generated, the behavior is
  undefined.

## See Also
- [`Gl::array_element`]
- [`Gl::client_active_texture`]
- [`Gl::color_pointer`]
- [`Gl::draw_arrays`]
- [`Gl::draw_elements`]
- [`Gl::edge_flag_pointer`]
- [`Gl::fog_coord_pointer`]
- [`Gl::enable`]
- [`Gl::get_pointerv`]
- [`Gl::index_pointer`]
- [`Gl::interleaved_arrays`]
- [`Gl::normal_pointer`]
- [`Gl::secondary_color_pointer`]
- [`Gl::tex_coord_pointer`]
- [`Gl::vertex_pointer`]
