# glDrawElementsInstanced
draw multiple instances of a set of elements

## Parameters
- `mode`
  Specifies what kind of primitives to render. Symbolic constants
  [`gl::POINTS`], [`gl::LINE_STRIP`], [`gl::LINE_LOOP`], [`gl::LINES`],
  [`gl::LINE_STRIP_ADJACENCY`], [`gl::LINES_ADJACENCY`],
  [`gl::TRIANGLE_STRIP`], [`gl::TRIANGLE_FAN`], [`gl::TRIANGLES`],
  [`gl::TRIANGLE_STRIP_ADJACENCY`], [`gl::TRIANGLES_ADJACENCY`] and
  [`gl::PATCHES`] are accepted.

## Description
[`Gl::draw_elements_instanced`] behaves identically to
  [`Gl::draw_elements`] except that `instancecount` instances of the set
  of elements are executed and the value of the internal counter
  `instanceID` advances for each iteration. `instanceID` is an internal
  32-bit integer counter that may be read by a vertex shader as
  [`gl_InstanceID`].
[`Gl::draw_elements_instanced`] has the same effect as: ```c if (mode,
  count, or type is invalid ) generate appropriate error else { for (int
  i = 0; i < instancecount ; i++) { instanceID = i; glDrawElements(mode,
  count, type, indices); } instanceID = 0; } ```

## Notes
[`Gl::draw_elements_instanced`] is available only if the GL version is
  3.1 or greater.
[`gl::LINE_STRIP_ADJACENCY`], [`gl::LINES_ADJACENCY`],
  [`gl::TRIANGLE_STRIP_ADJACENCY`] and [`gl::TRIANGLES_ADJACENCY`] are
  available only if the GL version is 3.2 or greater.

## Errors
- [`gl::INVALID_ENUM`] is generated if `mode` is not one of
  [`gl::POINTS`], [`gl::LINE_STRIP`], [`gl::LINE_LOOP`], [`gl::LINES`],
  [`gl::TRIANGLE_STRIP`], [`gl::TRIANGLE_FAN`], or [`gl::TRIANGLES`].
- [`gl::INVALID_VALUE`] is generated if `count` or `instancecount` is
  negative.
- [`gl::INVALID_OPERATION`] is generated if a geometry shader is active
  and `mode` is incompatible with the input primitive type of the
  geometry shader in the currently installed program object.
- [`gl::INVALID_OPERATION`] is generated if a non-zero buffer object
  name is bound to an enabled array and the buffer object's data store
  is currently mapped.

## See Also
- [`Gl::draw_elements`]
- [`Gl::draw_arrays_instanced`]
