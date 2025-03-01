# glProgramParameter
specify a parameter for a program object

## Parameters
- `program`
  Specifies the name of a program object whose parameter to modify.

## Description
[`Gl::program_parameter`] specifies a new value for the parameter
  nameed by `pname` for the program object `program`.
If `pname` is [`gl::PROGRAM_BINARY_RETRIEVABLE_HINT`], `value` should
  be [`gl::FALSE`] or [`gl::TRUE`] to indicate to the implementation the
  intention of the application to retrieve the program's binary
  representation with [`Gl::get_program_binary`]. The implementation may
  use this information to store information that may be useful for a
  future query of the program's binary. It is recommended to set
  [`gl::PROGRAM_BINARY_RETRIEVABLE_HINT`] for the program to
  [`gl::TRUE`] before calling [`Gl::link_program`], and using the
  program at run-time if the binary is to be retrieved later.
If `pname` is [`gl::PROGRAM_SEPARABLE`], `value` must be [`gl::TRUE`]
  or [`gl::FALSE`] and indicates whether `program` can be bound to
  individual pipeline stages via [`Gl::use_program_stages`]. A program's
  [`gl::PROGRAM_SEPARABLE`] parameter must be set to [`gl::TRUE`]
  *before* [`Gl::link_program`] is called in order for it to be usable
  with a program pipeline object. The initial state of
  [`gl::PROGRAM_SEPARABLE`] is [`gl::FALSE`].

## Errors
- [`gl::INVALID_OPERATION`] is generated if `program` is not the name of
  an existing program object.
- [`gl::INVALID_ENUM`] is generated if `pname` is not one of the
  accepted values.
- [`gl::INVALID_VALUE`] is generated if `value` is not a valid value for
  the parameter named by `pname`.

## See Also
- [`Gl::get_program`]
- [`Gl::get_program_binary`]
- [`Gl::program_binary`]
