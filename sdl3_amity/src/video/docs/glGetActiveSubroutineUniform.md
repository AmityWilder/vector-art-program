# glGetActiveSubroutineUniform
query a property of an active shader subroutine uniform

## Parameters
- `program`
  Specifies the name of the program containing the subroutine.

## Description
[`Gl::get_active_subroutine_uniform`] queries a parameter of an active
  shader subroutine uniform. `program` contains the name of the program
  containing the uniform. `shadertype` specifies the stage which the
  uniform location, given by `index`, is valid. `index` must be between
  zero and the value of [`gl::ACTIVE_SUBROUTINE_UNIFORMS`] minus one for
  the shader stage.
If `pname` is [`gl::NUM_COMPATIBLE_SUBROUTINES`], a single integer
  indicating the number of subroutines that can be assigned to the
  uniform is returned in `values`.
If `pname` is [`gl::COMPATIBLE_SUBROUTINES`], an array of integers is
  returned in `values`, with each integer specifying the index of an
  active subroutine that can be assigned to the selected subroutine
  uniform. The number of integers returned is the same as the value
  returned for [`gl::NUM_COMPATIBLE_SUBROUTINES`].
If `pname` is [`gl::UNIFORM_SIZE`], a single integer is returned in
  `values`. If the selected subroutine uniform is an array, the declared
  size of the array is returned; otherwise, one is returned.
If `pname` is [`gl::UNIFORM_NAME_LENGTH`], a single integer specifying
  the length of the subroutine uniform name (including the terminating
  null character) is returned in `values`.

## Errors
- [`gl::INVALID_ENUM`] is generated if `shadertype` or `pname` is not
  one of the accepted values.
- [`gl::INVALID_VALUE`] is generated if `index` is greater than or equal
  to the value of [`gl::ACTIVE_SUBROUTINES`].
- [`gl::INVALID_VALUE`] is generated if `program` is not the name of an
  existing program object.

## See Also
- [`Gl::get_subroutine_index`]
- [`Gl::get_active_subroutine_uniform_name`]
- [`Gl::get_program_stage`]
