# glCreateShaderProgramv
create a stand-alone program from an array of null-terminated source
  code strings

## Parameters
- `type`
  Specifies the type of shader to create.

## Description
[`Gl::create_shader_program`] creates a program object containing
  compiled and linked shaders for a single stage specified by `type`.
  `strings` refers to an array of `count` strings from which to create
  the shader executables.
[`Gl::create_shader_program`] is equivalent (assuming no errors are
  generated) to:
The program object created by [`Gl::create_shader_program`] has its
  [`gl::PROGRAM_SEPARABLE`] status set to [`gl::TRUE`].

## Errors
- [`gl::INVALID_ENUM`] is generated if `type` is not an accepted shader
  type.
- [`gl::INVALID_VALUE`] is generated if `count` is negative.
- Other errors are generated if the supplied shader code fails to
  compile and link, as described for the commands in the pseudocode
  sequence above, but all such errors are generated without any side
  effects of executing those commands.

## See Also
- [`Gl::create_shader`]
- [`Gl::create_program`]
- [`Gl::compile_shader`]
- [`Gl::link_program`]
