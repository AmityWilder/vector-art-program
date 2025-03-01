# glTexCoordPointer
define an array of texture coordinates

## Parameters
- `size`
  Specifies the number of coordinates per array element. Must be 1, 2,
  3, or 4. The initial value is 4.

## Description
[`Gl::tex_coord_pointer`] specifies the location and data format of an
  array of texture coordinates to use when rendering. `size` specifies
  the number of coordinates per texture coordinate set, and must be 1,
  2, 3, or 4. `type` specifies the data type of each texture coordinate,
  and `stride` specifies the byte stride from one texture coordinate set
  to the next, allowing vertices and attributes to be packed into a
  single array or stored in separate arrays. (Single-array storage may
  be more efficient on some implementations; see
  [`Gl::interleaved_arrays`].)
If a non-zero named buffer object is bound to the [`gl::ARRAY_BUFFER`]
  target (see [`Gl::bind_buffer`]) while a texture coordinate array is
  specified, `pointer` is treated as a byte offset into the buffer
  object's data store. Also, the buffer object binding
  ([`gl::ARRAY_BUFFER_BINDING`]) is saved as texture coordinate vertex
  array client-side state ([`gl::TEXTURE_COORD_ARRAY_BUFFER_BINDING`]).
When a texture coordinate array is specified, `size`, `type`,
  `stride`, and `pointer` are saved as client-side state, in addition to
  the current vertex array buffer object binding.
To enable and disable a texture coordinate array, call
  [`Gl::enable_client_state`] and [`Gl::disable_client_state`] with the
  argument [`gl::TEXTURE_COORD_ARRAY`]. If enabled, the texture
  coordinate array is used when [`Gl::array_element`],
  [`Gl::draw_arrays`], [`Gl::multi_draw_arrays`], [`Gl::draw_elements`],
  [`Gl::multi_draw_elements`], or [`Gl::draw_range_elements`] is called.

## Notes
[`Gl::tex_coord_pointer`] is available only if the GL version is 1.1
  or greater.
For OpenGL versions 1.3 and greater, or when the ```c ARB_multitexture
  ``` extension is supported, [`Gl::tex_coord_pointer`] updates the
  texture coordinate array state of the active client texture unit,
  specified with [`Gl::client_active_texture`].
Each texture coordinate array is initially disabled and isn't accessed
  when [`Gl::array_element`], [`Gl::draw_elements`],
  [`Gl::draw_range_elements`], [`Gl::draw_arrays`],
  [`Gl::multi_draw_arrays`], or [`Gl::multi_draw_elements`] is called.
Execution of [`Gl::tex_coord_pointer`] is not allowed between the
  execution of [`Gl::begin`] and the corresponding execution of
  [`Gl::end`], but an error may or may not be generated. If no error is
  generated, the operation is undefined.
[`Gl::tex_coord_pointer`] is typically implemented on the client side.
Texture coordinate array parameters are client-side state and are
  therefore not saved or restored by [`Gl::push_attrib`] and
  [`Gl::pop_attrib`]. Use [`Gl::push_client_attrib`] and
  [`Gl::pop_client_attrib`] instead.

## Errors
- [`gl::INVALID_VALUE`] is generated if `size` is not 1, 2, 3, or 4.
- [`gl::INVALID_ENUM`] is generated if `type` is not an accepted value.
- [`gl::INVALID_VALUE`] is generated if `stride` is negative.

## See Also
- [`Gl::array_element`]
- [`Gl::bind_buffer`]
- [`Gl::client_active_texture`]
- [`Gl::color_pointer`]
- [`Gl::disable_client_state`]
- [`Gl::draw_arrays`]
- [`Gl::draw_elements`]
- [`Gl::draw_range_elements`]
- [`Gl::edge_flag_pointer`]
- [`Gl::enable_client_state`]
- [`Gl::fog_coord_pointer`]
- [`Gl::index_pointer`]
- [`Gl::interleaved_arrays`]
- [`Gl::multi_draw_arrays`]
- [`Gl::multi_draw_elements`]
- [`Gl::multi_tex_coord`]
- [`Gl::normal_pointer`]
- [`Gl::pop_client_attrib`]
- [`Gl::push_client_attrib`]
- [`Gl::secondary_color_pointer`]
- [`Gl::tex_coord`]
- [`Gl::vertex_attrib_pointer`]
- [`Gl::vertex_pointer`]
