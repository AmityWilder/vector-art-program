# glGetActiveSubroutineUniformName
query the name of an active shader subroutine uniform

## Parameters
- `program`
  Specifies the name of the program containing the subroutine.

## Description
[`Gl::get_active_subroutine_uniform_name`] retrieves the name of an
  active shader subroutine uniform. `program` contains the name of the
  program containing the uniform. `shadertype` specifies the stage for
  which the uniform location, given by `index`, is valid. `index` must
  be between zero and the value of [`gl::ACTIVE_SUBROUTINE_UNIFORMS`]
  minus one for the shader stage.
The uniform name is returned as a null-terminated string in `name`.
  The actual number of characters written into `name`, excluding the
  null terminator is returned in `length`. If `length` is [`NULL`], no
  length is returned. The maximum number of characters that may be
  written into `name`, including the null terminator, is specified by
  `bufSize`. The length of the longest subroutine uniform name in
  `program` and `shadertype` is given by the value of
  [`gl::ACTIVE_SUBROUTINE_UNIFORM_MAX_LENGTH`], which can be queried
  with [`Gl::get_program_stage`].

## Errors
- [`gl::INVALID_ENUM`] is generated if `shadertype` or `pname` is not
  one of the accepted values.
- [`gl::INVALID_VALUE`] is generated if `index` is greater than or equal
  to the value of [`gl::ACTIVE_SUBROUTINE_UNIFORMS`].
- [`gl::INVALID_VALUE`] is generated if `program` is not the name of an
  existing program object.

## See Also
- [`Gl::get_subroutine_index`]
- [`Gl::get_active_subroutine_uniform`]
- [`Gl::get_program_stage`]
