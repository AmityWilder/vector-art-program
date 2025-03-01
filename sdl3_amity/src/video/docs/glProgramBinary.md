# glProgramBinary
load a program object with a program binary

## Parameters
- `program`
  Specifies the name of a program object into which to load a program
  binary.

## Description
[`Gl::program_binary`] loads a program object with a program binary
  previously returned from [`Gl::get_program_binary`]. `binaryFormat`
  and `binary` must be those returned by a previous call to
  [`Gl::get_program_binary`], and `length` must be the length returned
  by [`Gl::get_program_binary`], or by [`Gl::get_program`] when called
  with `pname` set to [`gl::PROGRAM_BINARY_LENGTH`]. If these conditions
  are not met, loading the program binary will fail and `program`'s
  [`gl::LINK_STATUS`] will be set to [`gl::FALSE`].
A program object's program binary is replaced by calls to
  [`Gl::link_program`] or [`Gl::program_binary`]. When linking success
  or failure is concerned, [`Gl::program_binary`] can be considered to
  perform an implicit linking operation. [`Gl::link_program`] and
  [`Gl::program_binary`] both set the program object's
  [`gl::LINK_STATUS`] to [`gl::TRUE`] or [`gl::FALSE`].
A successful call to [`Gl::program_binary`] will reset all uniform
  variables to their initial values. The initial value is either the
  value of the variable's initializer as specified in the original
  shader source, or zero if no initializer was present. Additionally,
  all vertex shader input and fragment shader output assignments that
  were in effect when the program was linked before saving are restored
  with [`Gl::program_binary`] is called.

## Errors
- [`gl::INVALID_OPERATION`] is generated if `program` is not the name of
  an existing program object.
- [`gl::INVALID_ENUM`] is generated if `binaryFormat` is not a value
  recognized by the implementation.

## Notes
A program binary may fail to load if the implementation determines
  that there has been a change in hardware or software configuration
  from when the program binary was produced such as having been compiled
  with an incompatible or outdated version of the compiler.

## See Also
- [`Gl::get_program`]
- [`Gl::get_program_binary`]
