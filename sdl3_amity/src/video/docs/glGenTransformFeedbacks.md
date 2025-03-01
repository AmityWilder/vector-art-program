# glGenTransformFeedbacks
reserve transform feedback object names

## Parameters
- `n`
  Specifies the number of transform feedback object names to reserve.

## Description
[`Gl::gen_transform_feedbacks`] returns `n` previously unused
  transform feedback object names in `ids`. These names are marked as
  used, for the purposes of [`Gl::gen_transform_feedbacks`] only, but
  they acquire transform feedback state only when they are first bound.

## See Also
- [`Gl::delete_transform_feedbacks`]
- [`Gl::bind_transform_feedback`]
- [`Gl::is_transform_feedback`]
- [`Gl::begin_transform_feedback`]
- [`Gl::pause_transform_feedback`]
- [`Gl::resume_transform_feedback`]
