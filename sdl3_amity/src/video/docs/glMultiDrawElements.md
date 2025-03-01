# glMultiDrawElements
render multiple sets of primitives by specifying indices of array data
  elements

## Parameters
- `mode`
  Specifies what kind of primitives to render. Symbolic constants
  [`gl::POINTS`], [`gl::LINE_STRIP`], [`gl::LINE_LOOP`], [`gl::LINES`],
  [`gl::LINE_STRIP_ADJACENCY`], [`gl::LINES_ADJACENCY`],
  [`gl::TRIANGLE_STRIP`], [`gl::TRIANGLE_FAN`], [`gl::TRIANGLES`],
  [`gl::TRIANGLE_STRIP_ADJACENCY`], [`gl::TRIANGLES_ADJACENCY`] and
  [`gl::PATCHES`] are accepted.

## Description
[`Gl::multi_draw_elements`] specifies multiple sets of geometric
  primitives with very few subroutine calls. Instead of calling a GL
  function to pass each individual vertex, normal, texture coordinate,
  edge flag, or color, you can prespecify separate arrays of vertices,
  normals, and so on, and use them to construct a sequence of primitives
  with a single call to [`Gl::multi_draw_elements`].
[`Gl::multi_draw_elements`] is identical in operation to
  [`Gl::draw_elements`] except that `drawcount` separate lists of
  elements are specified.
Vertex attributes that are modified by [`Gl::multi_draw_elements`]
  have an unspecified value after [`Gl::multi_draw_elements`] returns.
  Attributes that aren't modified maintain their previous values.

## Notes
[`gl::LINE_STRIP_ADJACENCY`], [`gl::LINES_ADJACENCY`],
  [`gl::TRIANGLE_STRIP_ADJACENCY`] and [`gl::TRIANGLES_ADJACENCY`] are
  available only if the GL version is 3.2 or greater.

## Errors
- [`gl::INVALID_ENUM`] is generated if `mode` is not an accepted value.
- [`gl::INVALID_VALUE`] is generated if `drawcount` is negative.
- [`gl::INVALID_OPERATION`] is generated if a non-zero buffer object
  name is bound to an enabled array or the element array and the buffer
  object's data store is currently mapped.

## See Also
- [`Gl::draw_arrays`]
- [`Gl::draw_range_elements`]
