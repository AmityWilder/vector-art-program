# glInterleavedArrays
simultaneously specify and enable several interleaved arrays

## Parameters
- `format`
  Specifies the type of array to enable. Symbolic constants [`gl::V2F`],
  [`gl::V3F`], [`gl::C4UB_V2F`], [`gl::C4UB_V3F`], [`gl::C3F_V3F`],
  [`gl::N3F_V3F`], [`gl::C4F_N3F_V3F`], [`gl::T2F_V3F`],
  [`gl::T4F_V4F`], [`gl::T2F_C4UB_V3F`], [`gl::T2F_C3F_V3F`],
  [`gl::T2F_N3F_V3F`], [`gl::T2F_C4F_N3F_V3F`], and
  [`gl::T4F_C4F_N3F_V4F`] are accepted.

## Description
[`Gl::interleaved_arrays`] lets you specify and enable individual
  color, normal, texture and vertex arrays whose elements are part of a
  larger aggregate array element. For some implementations, this is more
  efficient than specifying the arrays separately.
If `stride` is 0, the aggregate elements are stored consecutively.
  Otherwise, `stride` bytes occur between the beginning of one aggregate
  array element and the beginning of the next aggregate array element.
`format` serves as a ``key'' describing the extraction of individual
  arrays from the aggregate array. If `format` contains a T, then
  texture coordinates are extracted from the interleaved array. If C is
  present, color values are extracted. If N is present, normal
  coordinates are extracted. Vertex coordinates are always extracted.
The digits 2, 3, and 4 denote how many values are extracted. F
  indicates that values are extracted as floating-point values. Colors
  may also be extracted as 4 unsigned bytes if 4UB follows the C. If a
  color is extracted as 4 unsigned bytes, the vertex array element which
  follows is located at the first possible floating-point aligned
  address.

## Notes
[`Gl::interleaved_arrays`] is available only if the GL version is 1.1
  or greater.
If [`Gl::interleaved_arrays`] is called while compiling a display
  list, it is not compiled into the list, and it is executed
  immediately.
Execution of [`Gl::interleaved_arrays`] is not allowed between the
  execution of [`Gl::begin`] and the corresponding execution of
  [`Gl::end`], but an error may or may not be generated. If no error is
  generated, the operation is undefined.
[`Gl::interleaved_arrays`] is typically implemented on the client
  side.
Vertex array parameters are client-side state and are therefore not
  saved or restored by [`Gl::push_attrib`] and [`Gl::pop_attrib`]. Use
  [`Gl::push_client_attrib`] and [`Gl::pop_client_attrib`] instead.
For OpenGL versions 1.3 and greater, or when the ```c ARB_multitexture
  ``` extension is supported, [`Gl::interleaved_arrays`] only updates
  the texture coordinate array for the client active texture unit. The
  texture coordinate state for other client texture units is not
  updated, regardless of whether the client texture unit is enabled or
  not.
Secondary color values are not supported in interleaved vertex array
  formats.

## Errors
- [`gl::INVALID_ENUM`] is generated if `format` is not an accepted
  value.
- [`gl::INVALID_VALUE`] is generated if `stride` is negative.

## See Also
- [`Gl::array_element`]
- [`Gl::client_active_texture`]
- [`Gl::color_pointer`]
- [`Gl::draw_arrays`]
- [`Gl::draw_elements`]
- [`Gl::edge_flag_pointer`]
- [`Gl::enable_client_state`]
- [`Gl::get_pointerv`]
- [`Gl::index_pointer`]
- [`Gl::normal_pointer`]
- [`Gl::secondary_color_pointer`]
- [`Gl::tex_coord_pointer`]
- [`Gl::vertex_pointer`]
