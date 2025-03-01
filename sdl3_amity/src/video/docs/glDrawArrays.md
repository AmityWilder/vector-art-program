# glDrawArrays
render primitives from array data

## Parameters
- `mode`
  Specifies what kind of primitives to render. Symbolic constants
  [`gl::POINTS`], [`gl::LINE_STRIP`], [`gl::LINE_LOOP`], [`gl::LINES`],
  [`gl::LINE_STRIP_ADJACENCY`], [`gl::LINES_ADJACENCY`],
  [`gl::TRIANGLE_STRIP`], [`gl::TRIANGLE_FAN`], [`gl::TRIANGLES`],
  [`gl::TRIANGLE_STRIP_ADJACENCY`], [`gl::TRIANGLES_ADJACENCY`] and
  [`gl::PATCHES`] are accepted.

## Description
[`Gl::draw_arrays`] specifies multiple geometric primitives with very
  few subroutine calls. Instead of calling a GL procedure to pass each
  individual vertex, normal, texture coordinate, edge flag, or color,
  you can prespecify separate arrays of vertices, normals, and colors
  and use them to construct a sequence of primitives with a single call
  to [`Gl::draw_arrays`].
When [`Gl::draw_arrays`] is called, it uses `count` sequential
  elements from each enabled array to construct a sequence of geometric
  primitives, beginning with element `first`. `mode` specifies what kind
  of primitives are constructed and how the array elements construct
  those primitives.
Vertex attributes that are modified by [`Gl::draw_arrays`] have an
  unspecified value after [`Gl::draw_arrays`] returns. Attributes that
  aren't modified remain well defined.

## Notes
[`gl::LINE_STRIP_ADJACENCY`], [`gl::LINES_ADJACENCY`],
  [`gl::TRIANGLE_STRIP_ADJACENCY`] and [`gl::TRIANGLES_ADJACENCY`] are
  available only if the GL version is 3.2 or greater.

## Errors
- [`gl::INVALID_ENUM`] is generated if `mode` is not an accepted value.
- [`gl::INVALID_VALUE`] is generated if `count` is negative.
- [`gl::INVALID_OPERATION`] is generated if a non-zero buffer object
  name is bound to an enabled array and the buffer object's data store
  is currently mapped.
- [`gl::INVALID_OPERATION`] is generated if a geometry shader is active
  and `mode` is incompatible with the input primitive type of the
  geometry shader in the currently installed program object.

## See Also
- [`Gl::draw_arrays_instanced`]
- [`Gl::draw_elements`]
- [`Gl::draw_range_elements`]
