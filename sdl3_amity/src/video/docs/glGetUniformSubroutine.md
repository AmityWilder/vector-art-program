# glGetUniformSubroutine
retrieve the value of a subroutine uniform of a given shader stage of
  the current program

## Parameters
- `shadertype`
  Specifies the shader stage from which to query for subroutine uniform
  index. `shadertype` must be one of [`gl::VERTEX_SHADER`],
  [`gl::TESS_CONTROL_SHADER`], [`gl::TESS_EVALUATION_SHADER`],
  [`gl::GEOMETRY_SHADER`] or [`gl::FRAGMENT_SHADER`].

## Description
[`Gl::get_uniform_subroutine`] retrieves the value of the subroutine
  uniform at location `location` for shader stage `shadertype` of the
  current program. `location` must be less than the value of
  [`gl::ACTIVE_SUBROUTINE_UNIFORM_LOCATIONS`] for the shader currently
  in use at shader stage `shadertype`. The value of the subroutine
  uniform is returned in `values`.

## Errors
- [`gl::INVALID_ENUM`] is generated if `shadertype` is not one of the
  accepted values.
- [`gl::INVALID_VALUE`] is generated if `location` is greater than or
  equal to the value of [`gl::ACTIVE_SUBROUTINE_UNIFORM_LOCATIONS`] for
  the shader currently in use at shader stage `shadertype`.
- [`gl::INVALID_OPERATION`] is generated if no program is active.

## See Also
- [`Gl::get_program`]
- [`Gl::get_active_subroutine_uniform`]
- [`Gl::get_active_subroutine_uniform_name`]
- [`Gl::get_uniform_location`]
