# glCreateTransformFeedbacks
create transform feedback objects

## Parameters
- `n`
  Number of transform feedback objects to create.

## Description
[`Gl::create_transform_feedbacks`] returns `n` previously unused
  transform feedback object names in `ids`, each representing a new
  transform feedback object initialized to the default state.

## Errors
- [`gl::INVALID_VALUE`] is generated if `n` is negative.

## See Also
- [`Gl::begin_transform_feedback`]
- [`Gl::bind_transform_feedback`]
- [`Gl::delete_transform_feedbacks`]
- [`Gl::gen_transform_feedbacks`]
- [`Gl::is_transform_feedback`]
- [`Gl::pause_transform_feedback`]
- [`Gl::resume_transform_feedback`]
