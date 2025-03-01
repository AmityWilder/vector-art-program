# glGetTransformFeedbackVarying
retrieve information about varying variables selected for transform
  feedback

## Parameters
- `program`
  The name of the target program object.

## Description
Information about the set of varying variables in a linked program
  that will be captured during transform feedback may be retrieved by
  calling [`Gl::get_transform_feedback_varying`].
  [`Gl::get_transform_feedback_varying`] provides information about the
  varying variable selected by `index`. An `index` of 0 selects the
  first varying variable specified in the `varyings` array passed to
  [`Gl::transform_feedback_varyings`], and an `index` of the value of
  [`gl::TRANSFORM_FEEDBACK_VARYINGS`] minus one selects the last such
  variable.
The name of the selected varying is returned as a null-terminated
  string in `name`. The actual number of characters written into `name`,
  excluding the null terminator, is returned in `length`. If `length` is
  NULL, no length is returned. The maximum number of characters that may
  be written into `name`, including the null terminator, is specified by
  `bufSize`.
The length of the longest varying name in program is given by
  [`gl::TRANSFORM_FEEDBACK_VARYING_MAX_LENGTH`], which can be queried
  with [`Gl::get_program`].
For the selected varying variable, its type is returned into `type`.
  The size of the varying is returned into `size`. The value in `size`
  is in units of the type returned in `type`. The type returned can be
  any of the scalar, vector, or matrix attribute types returned by
  [`Gl::get_active_attrib`]. If an error occurred, the return parameters
  `length`, `size`, `type` and `name` will be unmodified. This command
  will return as much information about the varying variables as
  possible. If no information is available, `length` will be set to zero
  and `name` will be an empty string. This situation could arise if
  [`Gl::get_transform_feedback_varying`] is called after a failed link.

## Errors
- [`gl::INVALID_VALUE`] is generated if `program` is not the name of a
  program object.
- [`gl::INVALID_VALUE`] is generated if `index` is greater or equal to
  the value of [`gl::TRANSFORM_FEEDBACK_VARYINGS`].
- [`gl::INVALID_OPERATION`] is generated `program` has not been linked.

## See Also
- [`Gl::begin_transform_feedback`]
- [`Gl::transform_feedback_varyings`]
- [`Gl::get_program`]
