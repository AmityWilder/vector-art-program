# glDrawRangeElements
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
[`Gl::draw_range_elements`] is a restricted form of
  [`Gl::draw_elements`]. `mode`, and `count` match the corresponding
  arguments to [`Gl::draw_elements`], with the additional constraint
  that all values in the arrays `count` must lie between `start` and
  `end`, inclusive.
Implementations denote recommended maximum amounts of vertex and index
  data, which may be queried by calling [`Gl::get`] with argument
  [`gl::MAX_ELEMENTS_VERTICES`] and [`gl::MAX_ELEMENTS_INDICES`]. If
  $None$ is greater than the value of $$ $$ *end* *-* *start* *+* 1
  [`gl::MAX_ELEMENTS_VERTICES`], or if `count` is greater than the value
  of [`gl::MAX_ELEMENTS_INDICES`], then the call may operate at reduced
  performance. There is no requirement that all vertices in the range
  $None$ be referenced. However, the implementation may partially
  process unused vertices, reducing performance from what could be
  achieved with an optimal index set. $$ $$ ```c ``` *start* *end*
When [`Gl::draw_range_elements`] is called, it uses `count` sequential
  elements from an enabled array, starting at `start` to construct a
  sequence of geometric primitives. `mode` specifies what kind of
  primitives are constructed, and how the array elements construct these
  primitives. If more than one array is enabled, each is used.
Vertex attributes that are modified by [`Gl::draw_range_elements`]
  have an unspecified value after [`Gl::draw_range_elements`] returns.
  Attributes that aren't modified maintain their previous values.

## Notes
[`gl::LINE_STRIP_ADJACENCY`], [`gl::LINES_ADJACENCY`],
  [`gl::TRIANGLE_STRIP_ADJACENCY`] and [`gl::TRIANGLES_ADJACENCY`] are
  available only if the GL version is 3.2 or greater.

## Errors
- It is an error for indices to lie outside the range $None$, but
  implementations may not check for this situation. Such indices cause
  implementation-dependent behavior. $$ $$ ```c ``` *start* *end*
- [`gl::INVALID_ENUM`] is generated if `mode` is not an accepted value.
- [`gl::INVALID_VALUE`] is generated if `count` is negative.
- [`gl::INVALID_VALUE`] is generated if $None$. $$ $$ *end* *<* *start*
- [`gl::INVALID_OPERATION`] is generated if a geometry shader is active
  and `mode` is incompatible with the input primitive type of the
  geometry shader in the currently installed program object.
- [`gl::INVALID_OPERATION`] is generated if a non-zero buffer object
  name is bound to an enabled array or the element array and the buffer
  object's data store is currently mapped.

## See Also
- [`Gl::draw_arrays`]
- [`Gl::draw_elements`]
- [`Gl::draw_elements_base_vertex`]
