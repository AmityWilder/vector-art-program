# glDrawElementsInstancedBaseVertex
render multiple instances of a set of primitives from array data with
  a per-element offset

## Parameters
- `mode`
  Specifies what kind of primitives to render. Symbolic constants
  [`gl::POINTS`], [`gl::LINE_STRIP`], [`gl::LINE_LOOP`], [`gl::LINES`],
  [`gl::TRIANGLE_STRIP`], [`gl::TRIANGLE_FAN`], [`gl::TRIANGLES`],
  [`gl::LINES_ADJACENCY`], [`gl::LINE_STRIP_ADJACENCY`],
  [`gl::TRIANGLES_ADJACENCY`], [`gl::TRIANGLE_STRIP_ADJACENCY`] and
  [`gl::PATCHES`] are accepted.

## Description
[`Gl::draw_elements_instanced_base_vertex`] behaves identically to
  [`Gl::draw_elements_instanced`] except that the *i*th element
  transferred by the corresponding draw call will be taken from element
  `indices`[i] + `basevertex` of each enabled array. If the resulting
  value is larger than the maximum value representable by `type`, it is
  as if the calculation were upconverted to 32-bit unsigned integers
  (with wrapping on overflow conditions). The operation is undefined if
  the sum would be negative.

## Notes
[`Gl::draw_elements_instanced_base_vertex`] is only supported if the
  GL version is 3.2 or greater.

## Errors
- [`gl::INVALID_ENUM`] is generated if `mode` is not an accepted value.
- [`gl::INVALID_VALUE`] is generated if `count` or `instancecount` is
  negative.
- [`gl::INVALID_OPERATION`] is generated if a geometry shader is active
  and `mode` is incompatible with the input primitive type of the
  geometry shader in the currently installed program object.
- [`gl::INVALID_OPERATION`] is generated if a non-zero buffer object
  name is bound to an enabled array or the element array and the buffer
  object's data store is currently mapped.

## See Also
- [`Gl::draw_elements`]
- [`Gl::draw_range_elements`]
- [`Gl::draw_range_elements_base_vertex`]
- [`Gl::draw_elements_instanced`]
- [`Gl::draw_elements_instanced_base_vertex`]
