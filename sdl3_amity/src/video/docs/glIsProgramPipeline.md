# glIsProgramPipeline
determine if a name corresponds to a program pipeline object

## Parameters
- `pipeline`
  Specifies a value that may be the name of a program pipeline object.

## Description
[`Gl::is_program_pipeline`] returns [`gl::TRUE`] if `pipeline` is
  currently the name of a program pipeline object. If `pipeline` is
  zero, or if [`pipeline`] is not the name of a program pipeline object,
  or if an error occurs, [`Gl::is_program_pipeline`] returns
  [`gl::FALSE`]. If `pipeline` is a name returned by
  [`Gl::gen_program_pipelines`], but that has not yet been bound through
  a call to [`Gl::bind_program_pipeline`], then the name is not a
  program pipeline object and [`Gl::is_program_pipeline`] returns
  [`gl::FALSE`].

## See Also
- [`Gl::gen_program_pipelines`]
- [`Gl::bind_program_pipeline`]
- [`Gl::delete_program_pipelines`]
