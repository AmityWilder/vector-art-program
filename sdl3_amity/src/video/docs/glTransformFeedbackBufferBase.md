# glTransformFeedbackBufferBase
bind a buffer object to a transform feedback buffer object

## Parameters
- `xfb`
  Name of the transform feedback buffer object.

## Description
[`Gl::transform_feedback_buffer_base`] binds the buffer object
  `buffer` to the binding point at index `index` of the transform
  feedback object `xfb`.

## Notes
Calling [`Gl::transform_feedback_buffer_base`] is equivalent to
  calling [`Gl::transform_feedback_buffer_range`] with `offset` zero and
  `size` equal to the size of `buffer`.

## Errors
- [`gl::INVALID_OPERATION`] is generated if `xfb` is not the name of an
  existing transform feedback object.
- [`gl::INVALID_VALUE`] is generated if in `buffer` is not zero or the
  name of an existing buffer object.
- [`gl::INVALID_VALUE`] is generated if `index` is greater than or equal
  to the number of transform feedback buffer binding points (the value
  of [`gl::TRANSFORM_FEEDBACK_BUFFER_BINDING`]).

## See Also
- [`Gl::bind_buffer_range`]
- [`Gl::bind_buffer_base`]
- [`Gl::transform_feedback_buffer_range`]
