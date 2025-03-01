# glTransformFeedbackBufferRange
bind a range within a buffer object to a transform feedback buffer
  object

## Parameters
- `xfb`
  Name of the transform feedback buffer object.

## Description
[`Gl::transform_feedback_buffer_range`] binds a range of the buffer
  object `buffer` represented by `offset` and `size` to the binding
  point at index `index` of the transform feedback object `xfb`.
`offset` specifies the offset in basic machine units into the buffer
  object `buffer` and `size` specifies the amount of data that can be
  read from the buffer object while used as an indexed target.

## Notes


## Errors
- [`gl::INVALID_OPERATION`] is generated if `xfb` is not the name of an
  existing transform feedback object.
- [`gl::INVALID_VALUE`] is generated if in `buffer` is not zero or the
  name of an existing buffer object.
- [`gl::INVALID_VALUE`] is generated if `index` is greater than or equal
  to the number of transform feedback buffer binding points (the value
  of [`gl::TRANSFORM_FEEDBACK_BUFFER_BINDING`]).
- [`gl::INVALID_VALUE`] is generated if `offset` is negative.
- [`gl::INVALID_VALUE`] is generated if `buffer` is non-zero and either
  `size` is less than or equal to zero, or `offset` + `size` is greater
  than the value of [`gl::BUFFER_SIZE`] for `buffer`.

## See Also
- [`Gl::bind_buffer_range`]
- [`Gl::bind_buffer_base`]
- [`Gl::transform_feedback_buffer_base`]
