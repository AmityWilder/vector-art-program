# glDrawTransformFeedbackStreamInstanced
render multiple instances of primitives using a count derived from a
  specifed stream of a transform feedback object

## Parameters
- `mode`
  Specifies what kind of primitives to render. Symbolic constants
  [`gl::POINTS`], [`gl::LINE_STRIP`], [`gl::LINE_LOOP`], [`gl::LINES`],
  [`gl::LINE_STRIP_ADJACENCY`], [`gl::LINES_ADJACENCY`],
  [`gl::TRIANGLE_STRIP`], [`gl::TRIANGLE_FAN`], [`gl::TRIANGLES`],
  [`gl::TRIANGLE_STRIP_ADJACENCY`], [`gl::TRIANGLES_ADJACENCY`], and
  [`gl::PATCHES`] are accepted.

## Description
[`Gl::draw_transform_feedback_stream_instanced`] draws multiple copies
  of a range of primitives of a type specified by `mode` using a count
  retrieved from the transform feedback stream specified by `stream` of
  the transform feedback object specified by `id`. Calling
  [`Gl::draw_transform_feedback_stream_instanced`] is equivalent to
  calling [`Gl::draw_arrays_instanced`] with `mode` and `instancecount`
  as specified, `first` set to zero, and `count` set to the number of
  vertices captured on vertex stream `stream` the last time transform
  feedback was active on the transform feedback object named by `id`.
Calling [`Gl::draw_transform_feedback_instanced`] is equivalent to
  calling [`Gl::draw_transform_feedback_stream_instanced`] with `stream`
  set to zero.

## Errors
- [`gl::INVALID_ENUM`] is generated if `mode` is not an accepted value.
- [`gl::INVALID_VALUE`] is generated if `id` is not the name of a
  transform feedback object.
- [`gl::INVALID_VALUE`] is generated if `stream` is greater than or
  equal to the value of [`gl::MAX_VERTEX_STREAMS`].
- [`gl::INVALID_OPERATION`] is generated if a non-zero buffer object
  name is bound to an enabled array and the buffer object's data store
  is currently mapped.
- [`gl::INVALID_OPERATION`] is generated if a geometry shader is active
  and `mode` is incompatible with the input primitive type of the
  geometry shader in the currently installed program object.
- [`gl::INVALID_OPERATION`] is generated if `mode` is [`gl::PATCHES`]
  and no tessellation control shader is active.
- [`gl::INVALID_OPERATION`] is generated if
  [`Gl::end_transform_feedback`] has never been called while the
  transform feedback object named by `id` was bound.

## See Also
- [`Gl::draw_arrays`]
- [`Gl::draw_arrays_instanced`]
- [`Gl::draw_elements`]
- [`Gl::draw_range_elements`]
- [`Gl::draw_transform_feedback`]
- [`Gl::draw_transform_feedback_stream`]
