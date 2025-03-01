# glMultiDrawArrays
render multiple sets of primitives from array data

## Parameters
- `mode`
  Specifies what kind of primitives to render. Symbolic constants
  [`gl::POINTS`], [`gl::LINE_STRIP`], [`gl::LINE_LOOP`], [`gl::LINES`],
  [`gl::LINE_STRIP_ADJACENCY`], [`gl::LINES_ADJACENCY`],
  [`gl::TRIANGLE_STRIP`], [`gl::TRIANGLE_FAN`], [`gl::TRIANGLES`],
  [`gl::TRIANGLE_STRIP_ADJACENCY`], [`gl::TRIANGLES_ADJACENCY`] and
  [`gl::PATCHES`] are accepted.

## Description
[`Gl::multi_draw_arrays`] specifies multiple sets of geometric
  primitives with very few subroutine calls. Instead of calling a GL
  procedure to pass each individual vertex, normal, texture coordinate,
  edge flag, or color, you can prespecify separate arrays of vertices,
  normals, and colors and use them to construct a sequence of primitives
  with a single call to [`Gl::multi_draw_arrays`].
[`Gl::multi_draw_arrays`] behaves identically to [`Gl::draw_arrays`]
  except that `drawcount` separate ranges of elements are specified
  instead.
When [`Gl::multi_draw_arrays`] is called, it uses `count` sequential
  elements from each enabled array to construct a sequence of geometric
  primitives, beginning with element `first`. `mode` specifies what kind
  of primitives are constructed, and how the array elements construct
  those primitives.
Vertex attributes that are modified by [`Gl::multi_draw_arrays`] have
  an unspecified value after [`Gl::multi_draw_arrays`] returns.
  Attributes that aren't modified remain well defined.

## Notes
[`gl::LINE_STRIP_ADJACENCY`], [`gl::LINES_ADJACENCY`],
  [`gl::TRIANGLE_STRIP_ADJACENCY`] and [`gl::TRIANGLES_ADJACENCY`] are
  available only if the GL version is 3.2 or greater.

## Errors
- [`gl::INVALID_ENUM`] is generated if `mode` is not an accepted value.
- [`gl::INVALID_VALUE`] is generated if `drawcount` is negative.
- [`gl::INVALID_OPERATION`] is generated if a non-zero buffer object
  name is bound to an enabled array and the buffer object's data store
  is currently mapped.

## See Also
- [`Gl::draw_elements`]
- [`Gl::draw_range_elements`]
