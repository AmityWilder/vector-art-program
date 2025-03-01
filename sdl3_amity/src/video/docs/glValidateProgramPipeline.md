# glValidateProgramPipeline
validate a program pipeline object against current GL state

## Parameters
- `pipeline`
  Specifies the name of a program pipeline object to validate.

## Description
[`Gl::validate_program_pipeline`] instructs the implementation to
  validate the shader executables contained in `pipeline` against the
  current GL state. The implementation may use this as an opportunity to
  perform any internal shader modifications that may be required to
  ensure correct operation of the installed shaders given the current GL
  state.
After a program pipeline has been validated, its validation status is
  set to [`gl::TRUE`]. The validation status of a program pipeline
  object may be queried by calling [`Gl::get_program_pipeline`] with
  parameter [`gl::VALIDATE_STATUS`].
If `pipeline` is a name previously returned from a call to
  [`Gl::gen_program_pipelines`] but that has not yet been bound by a
  call to [`Gl::bind_program_pipeline`], a new program pipeline object
  is created with name `pipeline` and the default state vector.

## Errors
- [`gl::INVALID_OPERATION`] is generated if `pipeline` is not a name
  previously returned from a call to [`Gl::gen_program_pipelines`] or if
  such a name has been deleted by a call to
  [`Gl::delete_program_pipelines`].

## See Also
- [`Gl::gen_program_pipelines`]
- [`Gl::bind_program_pipeline`]
- [`Gl::delete_program_pipelines`]
