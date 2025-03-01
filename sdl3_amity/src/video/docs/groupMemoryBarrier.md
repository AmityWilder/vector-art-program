# groupMemoryBarrier
controls the ordering of memory transaction issued shader invocation
  relative to a work group

## Description
[`Gl::group_memory_barrier`] waits on the completion of all memory
  accesses performed by an invocation of a compute shader relative to
  the same access performed by other invocations in the same work group
  and then returns with no other effect.

## See Also
- [`Gl::memory_barrier`]
- [`Gl::memory_barrier_image`]
- [`Gl::memory_barrier_buffer`]
- [`Gl::memory_barrier_shared`]
