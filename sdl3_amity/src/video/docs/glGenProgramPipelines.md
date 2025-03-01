# glGenProgramPipelines
reserve program pipeline object names

## Parameters
- `n`
  Specifies the number of program pipeline object names to reserve.

## Description
[`Gl::gen_program_pipelines`] returns `n` previously unused program
  pipeline object names in `pipelines`. These names are marked as used,
  for the purposes of [`Gl::gen_program_pipelines`] only, but they
  acquire program pipeline state only when they are first bound.

## See Also
- [`Gl::delete_program_pipelines`]
- [`Gl::bind_program_pipeline`]
- [`Gl::is_program_pipeline`]
- [`Gl::use_program_stages`]
- [`Gl::use_program`]
