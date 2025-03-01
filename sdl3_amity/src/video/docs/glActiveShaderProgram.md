# glActiveShaderProgram
set the active program object for a program pipeline object

## Parameters
- `pipeline`
  Specifies the program pipeline object to set the active program object
  for.

## Description
[`Gl::active_shader_program`] sets the linked program named by
  `program` to be the active program for the program pipeline object
  `pipeline`. The active program in the active program pipeline object
  is the target of calls to [`Gl::uniform`] when no program has been
  made current through a call to [`Gl::use_program`].

## Errors
- [`gl::INVALID_OPERATION`] is generated if `pipeline` is not a name
  previously returned from a call to [`Gl::gen_program_pipelines`] or if
  such a name has been deleted by a call to
  [`Gl::delete_program_pipelines`].
- [`gl::INVALID_OPERATION`] is generated if `program` refers to a
  program object that has not been successfully linked.

## See Also
- [`Gl::gen_program_pipelines`]
- [`Gl::delete_program_pipelines`]
- [`Gl::is_program_pipeline`]
- [`Gl::use_program`]
- [`Gl::uniform`]
