# glPassThrough
place a marker in the feedback buffer

## Parameters
- `token`
  Specifies a marker value to be placed in the feedback buffer following
  a [`gl::PASS_THROUGH_TOKEN`].

## Description

Feedback is a GL render mode. The mode is selected by calling
  [`Gl::render_mode`] with [`gl::FEEDBACK`]. When the GL is in feedback
  mode, no pixels are produced by rasterization. Instead, information
  about primitives that would have been rasterized is fed back to the
  application using the GL. See the [`Gl::feedback_buffer`] reference
  page for a description of the feedback buffer and the values in it.
[`Gl::pass_through`] inserts a user-defined marker in the feedback
  buffer when it is executed in feedback mode. `token` is returned as if
  it were a primitive; it is indicated with its own unique identifying
  value: [`gl::PASS_THROUGH_TOKEN`]. The order of [`Gl::pass_through`]
  commands with respect to the specification of graphics primitives is
  maintained.

## Notes
[`Gl::pass_through`] is ignored if the GL is not in feedback mode.

## Errors
- [`gl::INVALID_OPERATION`] is generated if [`Gl::pass_through`] is
  executed between the execution of [`Gl::begin`] and the corresponding
  execution of [`Gl::end`].

## See Also
- [`Gl::feedback_buffer`]
- [`Gl::render_mode`]
