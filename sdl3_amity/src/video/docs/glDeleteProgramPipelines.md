# glDeleteProgramPipelines
delete program pipeline objects

## Parameters
- `n`
  Specifies the number of program pipeline objects to delete.

## Description
[`Gl::delete_program_pipelines`] deletes the `n` program pipeline
  objects whose names are stored in the array `pipelines`. Unused names
  in `pipelines` are ignored, as is the name zero. After a program
  pipeline object is deleted, its name is again unused and it has no
  contents. If program pipeline object that is currently bound is
  deleted, the binding for that object reverts to zero and no program
  pipeline object becomes current.

## See Also
- [`Gl::gen_program_pipelines`]
- [`Gl::bind_program_pipeline`]
- [`Gl::is_program_pipeline`]
- [`Gl::use_program_stages`]
- [`Gl::use_program`]
