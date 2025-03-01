# glGetProgramBinary
return a binary representation of a program object's compiled and
  linked executable source

## Parameters
- `program`
  Specifies the name of a program object whose binary representation to
  retrieve.

## Description
[`Gl::get_program_binary`] returns a binary representation of the
  compiled and linked executable for `program` into the array of bytes
  whose address is specified in `binary`. The maximum number of bytes
  that may be written into `binary` is specified by `bufSize`. If the
  program binary is greater in size than `bufSize` bytes, then an error
  is generated, otherwise the actual number of bytes written into
  `binary` is returned in the variable whose address is given by
  `length`. If `length` is [`NULL`], then no length is returned.
The format of the program binary written into `binary` is returned in
  the variable whose address is given by `binaryFormat`, and may be
  implementation dependent. The binary produced by the GL may
  subsequently be returned to the GL by calling [`Gl::program_binary`],
  with `binaryFormat` and `length` set to the values returned by
  [`Gl::get_program_binary`], and passing the returned binary data in
  the `binary` parameter.

## Errors
- [`gl::INVALID_OPERATION`] is generated if `bufSize` is less than the
  size of [`gl::PROGRAM_BINARY_LENGTH`] for `program`.
- [`gl::INVALID_OPERATION`] is generated if [`gl::LINK_STATUS`] for the
  program object is false.

## See Also
- [`Gl::get_program`]
- [`Gl::program_binary`]
