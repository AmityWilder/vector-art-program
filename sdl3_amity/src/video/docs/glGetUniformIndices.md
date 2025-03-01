# glGetUniformIndices
retrieve the index of a named uniform block

## Parameters
- `program`
  Specifies the name of a program containing uniforms whose indices to
  query.

## Description
[`Gl::get_uniform_indices`] retrieves the indices of a number of
  uniforms within `program`.
`program` must be the name of a program object for which the command
  [`Gl::link_program`] must have been called in the past, although it is
  not required that [`Gl::link_program`] must have succeeded. The link
  could have failed because the number of active uniforms exceeded the
  limit.
`uniformCount` indicates both the number of elements in the array of
  names `uniformNames` and the number of indices that may be written to
  `uniformIndices`.
`uniformNames` contains a list of `uniformCount` name strings
  identifying the uniform names to be queried for indices. For each name
  string in `uniformNames`, the index assigned to the active uniform of
  that name will be written to the corresponding element of
  `uniformIndices`. If a string in `uniformNames` is not the name of an
  active uniform, the special value [`gl::INVALID_INDEX`] will be
  written to the corresponding element of `uniformIndices`.
If an error occurs, nothing is written to `uniformIndices`.

## Errors
- [`gl::INVALID_OPERATION`] is generated if `program` is not the name of
  a program object for which [`Gl::link_program`] has been called in the
  past.

## Notes
[`Gl::get_uniform_indices`] is available only if the GL version is 3.1
  or greater.

## See Also
- [`Gl::get_active_uniform`]
- [`Gl::get_active_uniform_name`]
- [`Gl::link_program`]
