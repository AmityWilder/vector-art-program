# glSecondaryColorPointer
define an array of secondary colors

## Parameters
- `size`
  Specifies the number of components per color. Must be 3.

## Description
[`Gl::secondary_color_pointer`] specifies the location and data format
  of an array of color components to use when rendering. `size`
  specifies the number of components per color, and must be 3. `type`
  specifies the data type of each color component, and `stride`
  specifies the byte stride from one color to the next, allowing
  vertices and attributes to be packed into a single array or stored in
  separate arrays.
If a non-zero named buffer object is bound to the [`gl::ARRAY_BUFFER`]
  target (see [`Gl::bind_buffer`]) while a secondary color array is
  specified, `pointer` is treated as a byte offset into the buffer
  object's data store. Also, the buffer object binding
  ([`gl::ARRAY_BUFFER_BINDING`]) is saved as secondary color vertex
  array client-side state
  ([`gl::SECONDARY_COLOR_ARRAY_BUFFER_BINDING`]).
When a secondary color array is specified, `size`, `type`, `stride`,
  and `pointer` are saved as client-side state, in addition to the
  current vertex array buffer object binding.
To enable and disable the secondary color array, call
  [`Gl::enable_client_state`] and [`Gl::disable_client_state`] with the
  argument [`gl::SECONDARY_COLOR_ARRAY`]. If enabled, the secondary
  color array is used when [`Gl::array_element`], [`Gl::draw_arrays`],
  [`Gl::multi_draw_arrays`], [`Gl::draw_elements`],
  [`Gl::multi_draw_elements`], or [`Gl::draw_range_elements`] is called.

## Notes
[`Gl::secondary_color_pointer`] is available only if the GL version is
  1.4 or greater.
Secondary colors are not supported for interleaved vertex array
  formats (see [`Gl::interleaved_arrays`]).
The secondary color array is initially disabled and isn't accessed
  when [`Gl::array_element`], [`Gl::draw_elements`],
  [`Gl::draw_range_elements`], [`Gl::draw_arrays`],
  [`Gl::multi_draw_arrays`], or [`Gl::multi_draw_elements`] is called.
Execution of [`Gl::secondary_color_pointer`] is not allowed between
  the execution of [`Gl::begin`] and the corresponding execution of
  [`Gl::end`], but an error may or may not be generated. If no error is
  generated, the operation is undefined.
[`Gl::secondary_color_pointer`] is typically implemented on the client
  side.
Secondary color array parameters are client-side state and are
  therefore not saved or restored by [`Gl::push_attrib`] and
  [`Gl::pop_attrib`]. Use [`Gl::push_client_attrib`] and
  [`Gl::pop_client_attrib`] instead.

## Errors
- [`gl::INVALID_VALUE`] is generated if `size` is not 3.
- [`gl::INVALID_ENUM`] is generated if `type` is not an accepted value.
- [`gl::INVALID_VALUE`] is generated if `stride` is negative.

## See Also
- [`Gl::array_element`]
- [`Gl::bind_buffer`]
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
- [`Gl::normal_pointer`]
- [`Gl::pop_client_attrib`]
- [`Gl::push_client_attrib`]
- [`Gl::secondary_color`]
- [`Gl::tex_coord_pointer`]
- [`Gl::vertex_attrib_pointer`]
- [`Gl::vertex_pointer`]
