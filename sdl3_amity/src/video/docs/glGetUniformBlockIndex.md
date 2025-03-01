# glGetUniformBlockIndex
retrieve the index of a named uniform block

## Parameters
- `program`
  Specifies the name of a program containing the uniform block.

## Description
[`Gl::get_uniform_block_index`] retrieves the index of a uniform block
  within `program`.
`program` must be the name of a program object for which the command
  [`Gl::link_program`] must have been called in the past, although it is
  not required that [`Gl::link_program`] must have succeeded. The link
  could have failed because the number of active uniforms exceeded the
  limit.
`uniformBlockName` must contain a nul-terminated string specifying the
  name of the uniform block.
[`Gl::get_uniform_block_index`] returns the uniform block index for
  the uniform block named `uniformBlockName` of `program`. If
  `uniformBlockName` does not identify an active uniform block of
  `program`, [`Gl::get_uniform_block_index`] returns the special
  identifier, [`gl::INVALID_INDEX`]. Indices of the active uniform
  blocks of a program are assigned in consecutive order, beginning with
  zero.

## Errors
- [`gl::INVALID_OPERATION`] is generated if `program` is not the name of
  a program object for which [`Gl::link_program`] has been called in the
  past.

## Notes
[`Gl::get_uniform_block_index`] is available only if the GL version is
  3.1 or greater.

## See Also
- [`Gl::get_active_uniform_block_name`]
- [`Gl::get_active_uniform_block`]
- [`Gl::link_program`]
