# glGetActiveUniformBlockName
retrieve the name of an active uniform block

## Parameters
- `program`
  Specifies the name of a program containing the uniform block.

## Description
[`Gl::get_active_uniform_block_name`] retrieves the name of the active
  uniform block at `uniformBlockIndex` within `program`.
`program` must be the name of a program object for which the command
  [`Gl::link_program`] must have been called in the past, although it is
  not required that [`Gl::link_program`] must have succeeded. The link
  could have failed because the number of active uniforms exceeded the
  limit.
`uniformBlockIndex` is an active uniform block index of `program`, and
  must be less than the value of [`gl::ACTIVE_UNIFORM_BLOCKS`].
Upon success, the name of the uniform block identified by
  `unifomBlockIndex` is returned into `uniformBlockName`. The name is
  nul-terminated. The actual number of characters written into
  `uniformBlockName`, excluding the nul terminator, is returned in
  `length`. If `length` is ```c NULL ``` , no length is returned.
`bufSize` contains the maximum number of characters (including the nul
  terminator) that will be written into `uniformBlockName`.
If an error occurs, nothing will be written to `uniformBlockName` or
  `length`.

## Errors
- [`gl::INVALID_OPERATION`] is generated if `program` is not the name of
  a program object for which [`Gl::link_program`] has been called in the
  past.
- [`gl::INVALID_VALUE`] is generated if `uniformBlockIndex` is greater
  than or equal to the value of [`gl::ACTIVE_UNIFORM_BLOCKS`] or is not
  the index of an active uniform block in `program`.

## Notes
[`Gl::get_active_uniform_block_name`] is available only if the GL
  version is 3.1 or greater.

## See Also
- [`Gl::get_active_uniform_block`]
- [`Gl::get_uniform_block_index`]
