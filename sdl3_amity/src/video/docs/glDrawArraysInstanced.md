# glDrawArraysInstanced
draw multiple instances of a range of elements

## Parameters
- `mode`
  Specifies what kind of primitives to render. Symbolic constants
  [`gl::POINTS`], [`gl::LINE_STRIP`], [`gl::LINE_LOOP`], [`gl::LINES`],
  [`gl::TRIANGLE_STRIP`], [`gl::TRIANGLE_FAN`], [`gl::TRIANGLES`]
  [`gl::LINES_ADJACENCY`], [`gl::LINE_STRIP_ADJACENCY`],
  [`gl::TRIANGLES_ADJACENCY`], [`gl::TRIANGLE_STRIP_ADJACENCY`] and
  [`gl::PATCHES`] are accepted.

## Description
[`Gl::draw_arrays_instanced`] behaves identically to
  [`Gl::draw_arrays`] except that `instancecount` instances of the range
  of elements are executed and the value of the internal counter
  `instanceID` advances for each iteration. `instanceID` is an internal
  32-bit integer counter that may be read by a vertex shader as
  [`gl_InstanceID`].
[`Gl::draw_arrays_instanced`] has the same effect as: ```c if ( mode
  or count is invalid ) generate appropriate error else { for (int i =
  0; i < instancecount ; i++) { instanceID = i; glDrawArrays(mode,
  first, count); } instanceID = 0; } ```

## Errors
- [`gl::INVALID_ENUM`] is generated if `mode` is not one of the accepted
  values.
- [`gl::INVALID_OPERATION`] is generated if a geometry shader is active
  and `mode` is incompatible with the input primitive type of the
  geometry shader in the currently installed program object.
- [`gl::INVALID_VALUE`] is generated if `count` or `instancecount` is
  negative.
- [`gl::INVALID_OPERATION`] is generated if a non-zero buffer object
  name is bound to an enabled array and the buffer object's data store
  is currently mapped.

## See Also
- [`Gl::draw_arrays`]
- [`Gl::draw_elements_instanced`]
