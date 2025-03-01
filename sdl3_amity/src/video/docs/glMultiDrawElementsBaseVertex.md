# glMultiDrawElementsBaseVertex
render multiple sets of primitives by specifying indices of array data
  elements and an index to apply to each index

## Parameters
- `mode`
  Specifies what kind of primitives to render. Symbolic constants
  [`gl::POINTS`], [`gl::LINE_STRIP`], [`gl::LINE_LOOP`], [`gl::LINES`],
  [`gl::LINE_STRIP_ADJACENCY`], [`gl::LINES_ADJACENCY`],
  [`gl::TRIANGLE_STRIP`], [`gl::TRIANGLE_FAN`], [`gl::TRIANGLES`],
  [`gl::TRIANGLE_STRIP_ADJACENCY`], [`gl::TRIANGLES_ADJACENCY`] and
  [`gl::PATCHES`] are accepted.

## Description
[`Gl::multi_draw_elements_base_vertex`] behaves identically to
  [`Gl::draw_elements_base_vertex`], except that `drawcount` separate
  lists of elements are specifried instead.
It has the same effect as: ```c for (int i = 0; i < ``` `drawcount`;
  i++) if (`count`[i] > 0) glDrawElementsBaseVertex(`mode`, `count`[i],
  `type`, `indices[i]`, `basevertex[i]`);

## Notes
[`Gl::multi_draw_elements_base_vertex`] is available only if the GL
  version is 3.1 or greater.
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
- [`Gl::multi_draw_elements`]
- [`Gl::draw_elements_base_vertex`]
- [`Gl::draw_arrays`]
- [`Gl::vertex_attrib_pointer`]
