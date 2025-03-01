# glDeleteTransformFeedbacks
delete transform feedback objects

## Parameters
- `n`
  Specifies the number of transform feedback objects to delete.

## Description
[`Gl::delete_transform_feedbacks`] deletes the `n` transform feedback
  objects whose names are stored in the array `ids`. Unused names in
  `ids` are ignored, as is the name zero. After a transform feedback
  object is deleted, its name is again unused and it has no contents. If
  an active transform feedback object is deleted, its name immediately
  becomes unused, but the underlying object is not deleted until it is
  no longer active.

## See Also
- [`Gl::gen_transform_feedbacks`]
- [`Gl::bind_transform_feedback`]
- [`Gl::is_transform_feedback`]
- [`Gl::begin_transform_feedback`]
- [`Gl::pause_transform_feedback`]
- [`Gl::resume_transform_feedback`]
