# glBindProgramPipeline
bind a program pipeline to the current context

## Parameters
- `pipeline`
  Specifies the name of the pipeline object to bind to the context.

## Description
[`Gl::bind_program_pipeline`] binds a program pipeline object to the
  current context. `pipeline` must be a name previously returned from a
  call to [`Gl::gen_program_pipelines`]. If no program pipeline exists
  with name `pipeline` then a new pipeline object is created with that
  name and initialized to the default state vector.
When a program pipeline object is bound using
  [`Gl::bind_program_pipeline`], any previous binding is broken and is
  replaced with a binding to the specified pipeline object. If
  `pipeline` is zero, the previous binding is broken and is not
  replaced, leaving no pipeline object bound. If no current program
  object has been established by [`Gl::use_program`], the program
  objects used for each stage and for uniform updates are taken from the
  bound program pipeline object, if any. If there is a current program
  object established by [`Gl::use_program`], the bound program pipeline
  object has no effect on rendering or uniform updates. When a bound
  program pipeline object is used for rendering, individual shader
  executables are taken from its program objects.

## Errors
- [`gl::INVALID_OPERATION`] is generated if `pipeline` is not zero or a
  name previously returned from a call to [`Gl::gen_program_pipelines`]
  or if such a name has been deleted by a call to
  [`Gl::delete_program_pipelines`].

## See Also
- [`Gl::create_shader`]
- [`Gl::create_program`]
- [`Gl::compile_shader`]
- [`Gl::link_program`]
- [`Gl::gen_program_pipelines`]
- [`Gl::delete_program_pipelines`]
- [`Gl::is_program_pipeline`]
