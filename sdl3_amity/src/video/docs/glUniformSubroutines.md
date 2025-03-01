# glUniformSubroutines
load active subroutine uniforms

## Parameters
- `shadertype`
  Specifies the shader stage from which to query for subroutine uniform
  index. `shadertype` must be one of [`gl::VERTEX_SHADER`],
  [`gl::TESS_CONTROL_SHADER`], [`gl::TESS_EVALUATION_SHADER`],
  [`gl::GEOMETRY_SHADER`] or [`gl::FRAGMENT_SHADER`].

## Description
[`Gl::uniform_subroutines`] loads all active subroutine uniforms for
  shader stage `shadertype` of the current program with subroutine
  indices from `indices`, storing `indices[i]` into the uniform at
  location `i`. `count` must be equal to the value of
  [`gl::ACTIVE_SUBROUTINE_UNIFORM_LOCATIONS`] for the program currently
  in use at shader stage `shadertype`. Furthermore, all values in
  `indices` must be less than the value of [`gl::ACTIVE_SUBROUTINES`]
  for the shader stage.

## Errors
- [`gl::INVALID_OPERATION`] is generated if no program object is
  current.
- [`gl::INVALID_VALUE`] is generated if `count` is not equal to the
  value of [`gl::ACTIVE_SUBROUTINE_UNIFORM_LOCATIONS`] for the shader
  stage `shadertype` of the current program.
- [`gl::INVALID_VALUE`] is generated if any value in `indices` is geater
  than or equal to the value of [`gl::ACTIVE_SUBROUTINES`] for the
  shader stage `shadertype` of the current program.
- [`gl::INVALID_ENUM`] is generated if `shadertype` is not one of the
  accepted values.

## See Also
- [`Gl::get_program`]
- [`Gl::get_active_subroutine_uniform`]
- [`Gl::get_active_subroutine_uniform_name`]
- [`Gl::get_program_stage`]
