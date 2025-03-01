# glBindTransformFeedback
bind a transform feedback object

## Parameters
- `target`
  Specifies the target to which to bind the transform feedback object
  `id`. `target` must be [`gl::TRANSFORM_FEEDBACK`].

## Description
[`Gl::bind_transform_feedback`] binds the transform feedback object
  with name `id` to the current GL state. `id` must be a name previously
  returned from a call to [`Gl::gen_transform_feedbacks`]. If `id` has
  not previously been bound, a new transform feedback object with name
  `id` and initialized with the default transform state vector is
  created.
In the initial state, a default transform feedback object is bound and
  treated as a transform feedback object with a name of zero. If the
  name zero is subsequently bound, the default transform feedback object
  is again bound to the GL state.
While a transform feedback buffer object is bound, GL operations on
  the target to which it is bound affect the bound transform feedback
  object, and queries of the target to which a transform feedback object
  is bound return state from the bound object. When buffer objects are
  bound for transform feedback, they are attached to the currently bound
  transform feedback object. Buffer objects are used for trans- form
  feedback only if they are attached to the currently bound transform
  feedback object.

## Errors
- [`gl::INVALID_ENUM`] is generated if `target` is not
  [`gl::TRANSFORM_FEEDBACK`].
- [`gl::INVALID_OPERATION`] is generated if the transform feedback
  operation is active on the currently bound transform feedback object,
  and that operation is not paused.
- [`gl::INVALID_OPERATION`] is generated if `id` is not zero or the name
  of a transform feedback object returned from a previous call to
  [`Gl::gen_transform_feedbacks`], or if such a name has been deleted by
  [`Gl::delete_transform_feedbacks`].

## See Also
- [`Gl::gen_transform_feedbacks`]
- [`Gl::delete_transform_feedbacks`]
- [`Gl::is_transform_feedback`]
- [`Gl::begin_transform_feedback`]
- [`Gl::pause_transform_feedback`]
- [`Gl::resume_transform_feedback`]
