# glGetSubroutineIndex
retrieve the index of a subroutine uniform of a given shader stage
  within a program

## Parameters
- `program`
  Specifies the name of the program containing shader stage.

## Description
[`Gl::get_subroutine_index`] returns the index of a subroutine uniform
  within a shader stage attached to a program object. `program` contains
  the name of the program to which the shader is attached. `shadertype`
  specifies the stage from which to query shader subroutine index.
  `name` contains the null-terminated name of the subroutine uniform
  whose name to query.
If `name` is not the name of a subroutine uniform in the shader stage,
  [`gl::INVALID_INDEX`] is returned, but no error is generated. If
  `name` is the name of a subroutine uniform in the shader stage, a
  value between zero and the value of [`gl::ACTIVE_SUBROUTINES`] minus
  one will be returned. Subroutine indices are assigned using
  consecutive integers in the range from zero to the value of
  [`gl::ACTIVE_SUBROUTINES`] minus one for the shader stage.

## Errors
- [`gl::INVALID_ENUM`] is generated if `shadertype` or `pname` is not
  one of the accepted values.
- [`gl::INVALID_VALUE`] is generated if `program` is not the name of an
  existing program object.

## See Also
- [`Gl::get_program`]
- [`Gl::get_active_subroutine_uniform`]
- [`Gl::get_active_subroutine_uniform_name`]
