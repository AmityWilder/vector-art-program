# glGetActiveUniformBlock
query information about an active uniform block

## Parameters
- `program`
  Specifies the name of a program containing the uniform block.

## Description
[`Gl::get_active_uniform_blockiv`] retrieves information about an
  active uniform block within `program`.
`program` must be the name of a program object for which the command
  [`Gl::link_program`] must have been called in the past, although it is
  not required that [`Gl::link_program`] must have succeeded. The link
  could have failed because the number of active uniforms exceeded the
  limit.
`uniformBlockIndex` is an active uniform block index of `program`, and
  must be less than the value of [`gl::ACTIVE_UNIFORM_BLOCKS`].
Upon success, the uniform block parameter(s) specified by `pname` are
  returned in `params`. If an error occurs, nothing will be written to
  `params`.
If `pname` is [`gl::UNIFORM_BLOCK_BINDING`], then the index of the
  uniform buffer binding point last selected by the uniform block
  specified by `uniformBlockIndex` for `program` is returned. If no
  uniform block has been previously specified, zero is returned.
If `pname` is [`gl::UNIFORM_BLOCK_DATA_SIZE`], then the
  implementation-dependent minimum total buffer object size, in basic
  machine units, required to hold all active uniforms in the uniform
  block identified by `uniformBlockIndex` is returned. It is neither
  guaranteed nor expected that a given implementation will arrange
  uniform values as tightly packed in a buffer object. The exception to
  this is the *std140 uniform block layout*, which guarantees specific
  packing behavior and does not require the application to query for
  offsets and strides. In this case the minimum size may still be
  queried, even though it is determined in advance based only on the
  uniform block declaration.
If `pname` is [`gl::UNIFORM_BLOCK_NAME_LENGTH`], then the total length
  (including the nul terminator) of the name of the uniform block
  identified by `uniformBlockIndex` is returned.
If `pname` is [`gl::UNIFORM_BLOCK_ACTIVE_UNIFORMS`], then the number
  of active uniforms in the uniform block identified by
  `uniformBlockIndex` is returned.
If `pname` is [`gl::UNIFORM_BLOCK_ACTIVE_UNIFORM_INDICES`], then a
  list of the active uniform indices for the uniform block identified by
  `uniformBlockIndex` is returned. The number of elements that will be
  written to `params` is the value of
  [`gl::UNIFORM_BLOCK_ACTIVE_UNIFORMS`] for `uniformBlockIndex`.
If `pname` is [`gl::UNIFORM_BLOCK_REFERENCED_BY_VERTEX_SHADER`],
  [`gl::UNIFORM_BLOCK_REFERENCED_BY_TESS_CONTROL_SHADER`],
  [`gl::UNIFORM_BLOCK_REFERENCED_BY_TESS_EVALUATION_SHADER`],
  [`gl::UNIFORM_BLOCK_REFERENCED_BY_GEOMETRY_SHADER`],
  [`gl::UNIFORM_BLOCK_REFERENCED_BY_FRAGMENT_SHADER`], or
  [`gl::UNIFORM_BLOCK_REFERENCED_BY_COMPUTE_SHADER`] then a boolean
  value indicating whether the uniform block identified by
  `uniformBlockIndex` is referenced by the vertex, tessellation control,
  tessellation evaluation, geometry, fragment or compute programming
  stages of program, respectively, is returned.

## Errors
- [`gl::INVALID_VALUE`] is generated if `uniformBlockIndex` is greater
  than or equal to the value of [`gl::ACTIVE_UNIFORM_BLOCKS`] or is not
  the index of an active uniform block in `program`.
- [`gl::INVALID_ENUM`] is generated if `pname` is not one of the
  accepted tokens.
- [`gl::INVALID_OPERATION`] is generated if `program` is not the name of
  a program object for which [`Gl::link_program`] has been called in the
  past.

## Notes
[`Gl::get_active_uniform_blockiv`] is available only if the GL version
  is 3.1 or greater.
[`gl::UNIFORM_BLOCK_REFERENCED_BY_COMPUTE_SHADER`] is accepted only if
  the GL version is 4.3 or greater.

## See Also
- [`Gl::get_active_uniform_block_name`]
- [`Gl::get_uniform_block_index`]
- [`Gl::link_program`]
