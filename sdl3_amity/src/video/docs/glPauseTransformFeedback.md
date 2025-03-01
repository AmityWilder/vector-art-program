# glPauseTransformFeedback
pause transform feedback operations

## Description
[`Gl::pause_transform_feedback`] pauses transform feedback operations
  on the currently active transform feedback object. When transform
  feedback operations are paused, transform feedback is still considered
  active and changing most transform feedback state related to the
  object results in an error. However, a new transform feedback object
  may be bound while transform feedback is paused.

## Errors
- [`gl::INVALID_OPERATION`] is generated if the currently bound
  transform feedback object is not active or is paused.

## See Also
- [`Gl::gen_transform_feedbacks`]
- [`Gl::bind_transform_feedback`]
- [`Gl::begin_transform_feedback`]
- [`Gl::resume_transform_feedback`]
- [`Gl::delete_transform_feedbacks`]
