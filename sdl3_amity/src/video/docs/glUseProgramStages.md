# glUseProgramStages
bind stages of a program object to a program pipeline

## Parameters
- `pipeline`
  Specifies the program pipeline object to which to bind stages from
  `program`.

## Description
[`Gl::use_program_stages`] binds executables from a program object
  associated with a specified set of shader stages to the program
  pipeline object given by `pipeline`. `pipeline` specifies the program
  pipeline object to which to bind the executables. `stages` contains a
  logical combination of bits indicating the shader stages to use within
  `program` with the program pipeline object `pipeline`. `stages` must
  be a logical combination of [`gl::VERTEX_SHADER_BIT`],
  [`gl::TESS_CONTROL_SHADER_BIT`], [`gl::TESS_EVALUATION_SHADER_BIT`],
  [`gl::GEOMETRY_SHADER_BIT`], [`gl::FRAGMENT_SHADER_BIT`] and
  [`gl::COMPUTE_SHADER_BIT`]. Additionally, the special value
  [`gl::ALL_SHADER_BITS`] may be specified to indicate that all
  executables contained in `program` should be installed in `pipeline`.
If `program` refers to a program object with a valid shader attached
  for an indicated shader stage, [`Gl::use_program_stages`] installs the
  executable code for that stage in the indicated program pipeline
  object `pipeline`. If `program` is zero, or refers to a program object
  with no valid shader executable for a given stage, it is as if the
  pipeline object has no programmable stage configured for the indicated
  shader stages. If `stages` contains bits other than those listed
  above, and is not equal to [`gl::ALL_SHADER_BITS`], an error is
  generated.

## Notes
The [`gl::COMPUTE_SHADER_BIT`] bit is available only if the GL version
  is 4.3 or greater.

## Errors
- [`gl::INVALID_VALUE`] is generated if `shaders` contains set bits that
  are not recognized, and is not the reserved value
  [`gl::ALL_SHADER_BITS`].
- [`gl::INVALID_OPERATION`] is generated if `program` refers to a
  program object that was not linked with its [`gl::PROGRAM_SEPARABLE`]
  status set.
- [`gl::INVALID_OPERATION`] is generated if `program` refers to a
  program object that has not been successfully linked.
- [`gl::INVALID_OPERATION`] is generated if `pipeline` is not a name
  previously returned from a call to [`Gl::gen_program_pipelines`] or if
  such a name has been deleted by a call to
  [`Gl::delete_program_pipelines`].

## See Also
- [`Gl::gen_program_pipelines`]
- [`Gl::delete_program_pipelines`]
- [`Gl::is_program_pipeline`]
