# glGetActiveAtomicCounterBufferiv
retrieve information about the set of active atomic counter buffers
  for a program

## Parameters
- `program`
  The name of a program object from which to retrieve information.

## Description
[`Gl::get_active_atomic_counter_bufferiv`] retrieves information about
  the set of active atomic counter buffers for a program object.
  `program` is the name of a program object for which the command
  [`Gl::link_program`] has been issued in the past. It is not necessary
  for `program` to have been linked successfully. The link may have
  failed because the number of active atomic counters exceeded the
  limits.
`bufferIndex` specifies the index of an active atomic counter buffer
  and must be in the range zero to the value of
  [`gl::ACTIVE_ATOMIC_COUNTER_BUFFERS`] minus one. The value of
  [`gl::ACTIVE_ATOMIC_COUNTER_BUFFERS`] for `program` indicates the
  number of active atomic counter buffer and can be queried with
  [`Gl::get_program`].
If no error occurs, the parameter(s) specified by `pname` are returned
  in `params`. If an error is generated, the contents of `params` are
  not modified.
If `pname` is [`gl::ATOMIC_COUNTER_BUFFER_BINDING`], then the index of
  the counter buffer binding point associated with the active atomic
  counter buffer `bufferIndex` for `program` is returned.
If `pname` is [`gl::ATOMIC_COUNTER_BUFFER_DATA_SIZE`], then the
  implementation-dependent minimum total buffer object size, in baseic
  machine units, required to hold all active atomic counters in the
  atomic counter binding point identified by `bufferIndex` is returned.
If `pname` is [`gl::ATOMIC_COUNTER_BUFFER_ACTIVE_ATOMIC_COUNTERS`],
  then the number of active atomic counters for the atomic counter
  buffer identified by `bufferIndex` is returned.
If `pname` is
  [`gl::ATOMIC_COUNTER_BUFFER_ACTIVE_ATOMIC_COUNTER_INDICES`], then a
  list of the active atomic counter indices for the atomic counter
  buffer identified by `bufferIndex` is returned. The number of elements
  that will be written into `params` is the value of
  [`gl::ATOMIC_COUNTER_BUFFER_ACTIVE_ATOMIC_COUNTERS`] for
  `bufferIndex`.
If `pname` is
  [`gl::ATOMIC_COUNTER_BUFFER_REFERENCED_BY_VERTEX_SHADER`],
  [`gl::ATOMIC_COUNTER_BUFFER_REFERENCED_BY_TESS_CONTROL_SHADER`],
  [`gl::ATOMIC_COUNTER_BUFFER_REFERENCED_BY_TESS_EVALUATION_SHADER`],
  [`gl::ATOMIC_COUNTER_BUFFER_REFERENCED_BY_GEOMETRY_SHADER`],
  [`gl::ATOMIC_COUNTER_BUFFER_REFERENCED_BY_FRAGMENT_SHADER`],
  [`gl::ATOMIC_COUNTER_BUFFER_REFERENCED_BY_COMPUTE_SHADER`] then a
  boolean value indicating whether the atomic counter buffer identified
  by `bufferIndex` is referenced by the vertex, tessellation control,
  tessellation evaluation, geometry, fragment or compute processing
  stages of `program`, respectively, is returned.

## Notes
[`Gl::get_active_atomic_counter_bufferiv`] is available only if the GL
  version is 4.2 or higher.
[`gl::ATOMIC_COUNTER_BUFFER_REFERENCED_BY_COMPUTE_SHADER`] is
  available only of the GL version is 4.3 or higher.

## Errors
- [`gl::INVALID_VALUE`] is generated if `program` is not the name of a
  program object for which [`Gl::link_program`] has been called in the
  past.
- [`gl::INVALID_VALUE`] is generated if `bufferIndex` is greater than or
  equal to the value of [`gl::ACTIVE_ATOMIC_COUNTER_BUFFERS`] for
  `program`.
- [`gl::INVALID_ENUM`] is generated if `pname` is not one of the
  accepted tokens.

## See Also
- [`Gl::get_program`]
- [`Gl::get_active_subroutine_uniform`]
- [`Gl::get_active_subroutine_uniform_name`]
- [`Gl::get_uniform_location`]
