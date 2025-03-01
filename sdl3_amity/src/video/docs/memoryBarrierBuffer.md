# memoryBarrierBuffer
controls the ordering of operations on buffer variables issued by a
  single shader invocation

## Description
[`Gl::memory_barrier_buffer`] waits on the completion of all memory
  accesses resulting from the use of buffer variables and then returns
  with no other effect. When this function returns, the results of any
  modifications to the content of buffer variables will be visible to
  any access to the same buffer from other shader invocations. In
  particular, any modifications made in one shader stage are guaranteed
  to be visible to accesses performed by shader invocations in
  subsequent stages when those invocations were triggered by the
  execution of the original shader invocation (e.g., fragment shader
  invocations for a primitive resulting from a particular geometry
  shader invocation).

## See Also
- [`Gl::memory_barrier`]
- [`Gl::memory_barrier_image`]
- [`Gl::memory_barrier_shared`]
- [`Gl::group_memory_barrier`]
