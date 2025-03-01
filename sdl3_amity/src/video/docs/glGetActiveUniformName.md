# glGetActiveUniformName
query the name of an active uniform

## Parameters
- `program`
  Specifies the program containing the active uniform index
  `uniformIndex`.

## Description
[`Gl::get_active_uniform_name`] returns the name of the active uniform
  at `uniformIndex` within `program`. If `uniformName` is not ```c NULL
  ``` , up to `bufSize` characters (including a nul-terminator) will be
  written into the array whose address is specified by `uniformName`. If
  `length` is not ```c NULL ``` , the number of characters that were (or
  would have been) written into `uniformName` (not including the nul-
  terminator) will be placed in the variable whose address is specified
  in `length`. If `length` is ```c NULL ``` , no length is returned. The
  length of the longest uniform name in `program` is given by the value
  of [`gl::ACTIVE_UNIFORM_MAX_LENGTH`], which can be queried with
  [`Gl::get_program`].
If [`Gl::get_active_uniform_name`] is not successful, nothing is
  written to `length` or `uniformName`.
`program` must be the name of a program for which the command
  [`Gl::link_program`] has been issued in the past. It is not necessary
  for `program` to have been linked successfully. The link could have
  failed because the number of active uniforms exceeded the limit.
`uniformIndex` must be an active uniform index of the program
  `program`, in the range zero to the value of [`gl::ACTIVE_UNIFORMS`]
  minus one. The value of [`gl::ACTIVE_UNIFORMS`] can be queried with
  [`Gl::get_program`].

## Errors
- [`gl::INVALID_VALUE`] is generated if `uniformIndex` is greater than
  or equal to the value of [`gl::ACTIVE_UNIFORMS`].
- [`gl::INVALID_VALUE`] is generated if `bufSize` is negative.
- [`gl::INVALID_VALUE`] is generated if `program` is not the name of a
  program object for which [`Gl::link_program`] has been issued.

## See Also
- [`Gl::get_active_uniform`]
- [`Gl::get_uniform_indices`]
- [`Gl::get_program`]
- [`Gl::link_program`]
