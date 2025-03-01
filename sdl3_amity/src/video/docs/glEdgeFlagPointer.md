# glEdgeFlagPointer
define an array of edge flags

## Parameters
- `stride`
  Specifies the byte offset between consecutive edge flags. If `stride`
  is 0, the edge flags are understood to be tightly packed in the array.
  The initial value is 0.

## Description
[`Gl::edge_flag_pointer`] specifies the location and data format of an
  array of boolean edge flags to use when rendering. `stride` specifies
  the byte stride from one edge flag to the next, allowing vertices and
  attributes to be packed into a single array or stored in separate
  arrays.
If a non-zero named buffer object is bound to the [`gl::ARRAY_BUFFER`]
  target (see [`Gl::bind_buffer`]) while an edge flag array is
  specified, `pointer` is treated as a byte offset into the buffer
  object's data store. Also, the buffer object binding
  ([`gl::ARRAY_BUFFER_BINDING`]) is saved as edge flag vertex array
  client-side state ([`gl::EDGE_FLAG_ARRAY_BUFFER_BINDING`]).
When an edge flag array is specified, `stride` and `pointer` are saved
  as client-side state, in addition to the current vertex array buffer
  object binding.
To enable and disable the edge flag array, call
  [`Gl::enable_client_state`] and [`Gl::disable_client_state`] with the
  argument [`gl::EDGE_FLAG_ARRAY`]. If enabled, the edge flag array is
  used when [`Gl::draw_arrays`], [`Gl::multi_draw_arrays`],
  [`Gl::draw_elements`], [`Gl::multi_draw_elements`],
  [`Gl::draw_range_elements`], or [`Gl::array_element`] is called.

## Notes
[`Gl::edge_flag_pointer`] is available only if the GL version is 1.1
  or greater.
Edge flags are not supported for interleaved vertex array formats (see
  [`Gl::interleaved_arrays`]).
The edge flag array is initially disabled and isn't accessed when
  [`Gl::array_element`], [`Gl::draw_elements`],
  [`Gl::draw_range_elements`], [`Gl::draw_arrays`],
  [`Gl::multi_draw_arrays`], or [`Gl::multi_draw_elements`] is called.
Execution of [`Gl::edge_flag_pointer`] is not allowed between the
  execution of [`Gl::begin`] and the corresponding execution of
  [`Gl::end`], but an error may or may not be generated. If no error is
  generated, the operation is undefined.
[`Gl::edge_flag_pointer`] is typically implemented on the client side.
Edge flag array parameters are client-side state and are therefore not
  saved or restored by [`Gl::push_attrib`] and [`Gl::pop_attrib`]. Use
  [`Gl::push_client_attrib`] and [`Gl::pop_client_attrib`] instead.

## Errors
- [`gl::INVALID_ENUM`] is generated if `stride` is negative.

## See Also
- [`Gl::array_element`]
- [`Gl::bind_buffer`]
- [`Gl::color_pointer`]
- [`Gl::disable_client_state`]
- [`Gl::draw_arrays`]
- [`Gl::draw_elements`]
- [`Gl::draw_range_elements`]
- [`Gl::edge_flag`]
- [`Gl::enable_client_state`]
- [`Gl::fog_coord_pointer`]
- [`Gl::index_pointer`]
- [`Gl::interleaved_arrays`]
- [`Gl::multi_draw_arrays`]
- [`Gl::multi_draw_elements`]
- [`Gl::normal_pointer`]
- [`Gl::pop_client_attrib`]
- [`Gl::push_client_attrib`]
- [`Gl::secondary_color_pointer`]
- [`Gl::tex_coord_pointer`]
- [`Gl::vertex_attrib_pointer`]
- [`Gl::vertex_pointer`]
