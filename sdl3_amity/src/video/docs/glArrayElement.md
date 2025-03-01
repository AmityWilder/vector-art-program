# glArrayElement
render a vertex using the specified vertex array element

## Parameters
- `i`
  Specifies an index into the enabled vertex data arrays.

## Description
[`Gl::array_element`] commands are used within
  [`Gl::begin`]/[`Gl::end`] pairs to specify vertex and attribute data
  for point, line, and polygon primitives. If [`gl::VERTEX_ARRAY`] is
  enabled when [`Gl::array_element`] is called, a single vertex is
  drawn, using vertex and attribute data taken from location `i` of the
  enabled arrays. If [`gl::VERTEX_ARRAY`] is not enabled, no drawing
  occurs but the attributes corresponding to the enabled arrays are
  modified.
Use [`Gl::array_element`] to construct primitives by indexing vertex
  data, rather than by streaming through arrays of data in first-to-last
  order. Because each call specifies only a single vertex, it is
  possible to explicitly specify per-primitive attributes such as a
  single normal for each triangle.
Changes made to array data between the execution of [`Gl::begin`] and
  the corresponding execution of [`Gl::end`] may affect calls to
  [`Gl::array_element`] that are made within the same
  [`Gl::begin`]/[`Gl::end`] period in nonsequential ways. That is, a
  call to [`Gl::array_element`] that precedes a change to array data may
  access the changed data, and a call that follows a change to array
  data may access original data.

## Notes
[`Gl::array_element`] is available only if the GL version is 1.1 or
  greater.
[`Gl::array_element`] is included in display lists. If
  [`Gl::array_element`] is entered into a display list, the necessary
  array data (determined by the array pointers and enables) is also
  entered into the display list. Because the array pointers and enables
  are client-side state, their values affect display lists when the
  lists are created, not when the lists are executed.

## Errors
- [`gl::INVALID_VALUE`] may be generated if `i` is negative.
- [`gl::INVALID_OPERATION`] is generated if a non-zero buffer object
  name is bound to an enabled array and the buffer object's data store
  is currently mapped.

## See Also
- [`Gl::client_active_texture`]
- [`Gl::color_pointer`]
- [`Gl::draw_arrays`]
- [`Gl::edge_flag_pointer`]
- [`Gl::fog_coord_pointer`]
- [`Gl::get_pointerv`]
- [`Gl::index_pointer`]
- [`Gl::interleaved_arrays`]
- [`Gl::normal_pointer`]
- [`Gl::secondary_color_pointer`]
- [`Gl::tex_coord_pointer`]
- [`Gl::vertex_pointer`]
