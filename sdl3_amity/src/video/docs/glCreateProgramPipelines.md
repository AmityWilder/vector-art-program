# glCreateProgramPipelines
create program pipeline objects

## Parameters
- `n`
  Number of program pipeline objects to create.

## Description
[`Gl::create_program_pipelines`] returns `n` previously unused program
  pipeline names in `pipelines`, each representing a new program
  pipeline object initialized to the default state.

## Errors
- [`gl::INVALID_VALUE`] is generated if `n` is negative.

## See Also
- [`Gl::bind_program_pipeline`]
- [`Gl::create_shader`]
- [`Gl::create_program`]
- [`Gl::compile_shader`]
- [`Gl::link_program`]
- [`Gl::gen_program_pipelines`]
- [`Gl::delete_program_pipelines`]
- [`Gl::is_program_pipeline`]
