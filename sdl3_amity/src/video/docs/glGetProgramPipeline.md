# glGetProgramPipeline
retrieve properties of a program pipeline object

## Parameters
- `pipeline`
  Specifies the name of a program pipeline object whose parameter
  retrieve.

## Description
[`Gl::get_program_pipelineiv`] retrieves the value of a property of
  the program pipeline object `pipeline`. `pname` specifies the name of
  the parameter whose value to retrieve. The value of the parameter is
  written to the variable whose address is given by `params`.
If `pname` is [`gl::ACTIVE_PROGRAM`], the name of the active program
  object of the program pipeline object is returned in `params`.
If `pname` is [`gl::VERTEX_SHADER`], the name of the current program
  object for the vertex shader type of the program pipeline object is
  returned in `params`.
If `pname` is [`gl::TESS_CONTROL_SHADER`], the name of the current
  program object for the tessellation control shader type of the program
  pipeline object is returned in `params`.
If `pname` is [`gl::TESS_EVALUATION_SHADER`], the name of the current
  program object for the tessellation evaluation shader type of the
  program pipeline object is returned in `params`.
If `pname` is [`gl::GEOMETRY_SHADER`], the name of the current program
  object for the geometry shader type of the program pipeline object is
  returned in `params`.
If `pname` is [`gl::FRAGMENT_SHADER`], the name of the current program
  object for the fragment shader type of the program pipeline object is
  returned in `params`.
If `pname` is [`gl::INFO_LOG_LENGTH`], the length of the info log,
  including the null terminator, is returned in `params`. If there is no
  info log, zero is returned.

## Errors
- [`gl::INVALID_OPERATION`] is generated if `pipeline` is not zero or a
  name previously returned from a call to [`Gl::gen_program_pipelines`]
  or if such a name has been deleted by a call to
  [`Gl::delete_program_pipelines`].
- [`gl::INVALID_ENUM`] is generated if `pname` is not one of the
  accepted values.

## See Also
- [`Gl::gen_program_pipelines`]
- [`Gl::bind_program_pipeline`]
- [`Gl::delete_program_pipelines`]
