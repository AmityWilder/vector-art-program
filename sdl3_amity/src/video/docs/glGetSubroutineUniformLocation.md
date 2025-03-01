# glGetSubroutineUniformLocation
retrieve the location of a subroutine uniform of a given shader stage
  within a program

## Parameters
- `program`
  Specifies the name of the program containing shader stage.

## Description
[`Gl::get_subroutine_uniform_location`] returns the location of the
  subroutine uniform variable `name` in the shader stage of type
  `shadertype` attached to `program`, with behavior otherwise identical
  to [`Gl::get_uniform_location`].
If `name` is not the name of a subroutine uniform in the shader stage,
  -1 is returned, but no error is generated. If `name` is the name of a
  subroutine uniform in the shader stage, a value between zero and the
  value of [`gl::ACTIVE_SUBROUTINE_LOCATIONS`] minus one will be
  returned. Subroutine locations are assigned using consecutive integers
  in the range from zero to the value of
  [`gl::ACTIVE_SUBROUTINE_LOCATIONS`] minus one for the shader stage.
  For active subroutine uniforms declared as arrays, the declared array
  elements are assigned consecutive locations.

## Errors
- [`gl::INVALID_ENUM`] is generated if `shadertype` or `pname` is not
  one of the accepted values.
- [`gl::INVALID_VALUE`] is generated if `program` is not the name of an
  existing program object.

## See Also
- [`Gl::get_program`]
- [`Gl::get_active_subroutine_uniform`]
- [`Gl::get_active_subroutine_uniform_name`]
- [`Gl::get_uniform_location`]
