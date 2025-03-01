# glIsTransformFeedback
determine if a name corresponds to a transform feedback object

## Parameters
- `id`
  Specifies a value that may be the name of a transform feedback object.

## Description
[`Gl::is_transform_feedback`] returns [`gl::TRUE`] if `id` is
  currently the name of a transform feedback object. If `id` is zero, or
  if [`id`] is not the name of a transform feedback object, or if an
  error occurs, [`Gl::is_transform_feedback`] returns [`gl::FALSE`]. If
  `id` is a name returned by [`Gl::gen_transform_feedbacks`], but that
  has not yet been bound through a call to
  [`Gl::bind_transform_feedback`], then the name is not a transform
  feedback object and [`Gl::is_transform_feedback`] returns
  [`gl::FALSE`].

## See Also
- [`Gl::gen_transform_feedbacks`]
- [`Gl::bind_transform_feedback`]
- [`Gl::delete_transform_feedbacks`]
