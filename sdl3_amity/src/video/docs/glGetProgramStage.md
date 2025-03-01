# glGetProgramStage
retrieve properties of a program object corresponding to a specified
  shader stage

## Parameters
- `program`
  Specifies the name of the program containing shader stage.

## Description
[`Gl::get_program_stage`] queries a parameter of a shader stage
  attached to a program object. `program` contains the name of the
  program to which the shader is attached. `shadertype` specifies the
  stage from which to query the parameter. `pname` specifies which
  parameter should be queried. The value or values of the parameter to
  be queried is returned in the variable whose address is given in
  `values`.
If `pname` is [`gl::ACTIVE_SUBROUTINE_UNIFORMS`], the number of active
  subroutine variables in the stage is returned in `values`.
If `pname` is [`gl::ACTIVE_SUBROUTINE_UNIFORM_LOCATIONS`], the number
  of active subroutine variable locations in the stage is returned in
  `values`.
If `pname` is [`gl::ACTIVE_SUBROUTINES`], the number of active
  subroutines in the stage is returned in `values`.
If `pname` is [`gl::ACTIVE_SUBROUTINE_UNIFORM_MAX_LENGTH`], the length
  of the longest subroutine uniform for the stage is returned in
  `values`.
If `pname` is [`gl::ACTIVE_SUBROUTINE_MAX_LENGTH`], the length of the
  longest subroutine name for the stage is returned in `values`. The
  returned name length includes space for the null-terminator.
If there is no shader present of type `shadertype`, the returned value
  will be consistent with a shader containing no subroutines or
  subroutine uniforms.

## Errors
- [`gl::INVALID_ENUM`] is generated if `shadertype` or `pname` is not
  one of the accepted values.
- [`gl::INVALID_VALUE`] is generated if `program` is not the name of an
  existing program object.

## See Also
- [`Gl::get_program`]
