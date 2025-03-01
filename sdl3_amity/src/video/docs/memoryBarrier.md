# memoryBarrier
controls the ordering of memory transactions issued by a single shader
  invocation

## Description
[`Gl::memory_barrier`] waits on the completion of all memory accesses
  resulting from the use of image variables or atomic counters and then
  returns with no other effect. When this function returns, the results
  of any memory stores performed using coherent variables performed
  prior to the call will be visible to any future coherent memory access
  to the same addresses from other shader invocations. In particular,
  the values written this way in one shader stage are guaranteed to be
  visible to coherent memory accesses performed by shader invocations in
  subsequent stages when those invocations were triggered by the
  execution of the original shader invocation (e.g., fragment shader
  invocations for a primitive resulting from a particular geometry
  shader invocation).

## See Also
- [`Gl::image_load`]
- [`Gl::image_store`]
- [`Gl::image_atomic_add`]
- [`Gl::image_atomic_min`]
- [`Gl::image_atomic_max`]
- [`Gl::image_atomic_xor`]
- [`Gl::image_atomic_or`]
- [`Gl::image_atomic_and`]
- [`Gl::image_atomic_exchange`]
- [`Gl::image_atomic_comp_swap`]
- [`Gl::group_memory_barrier`]
- [`Gl::memory_barrier_image`]
- [`Gl::memory_barrier_buffer`]
- [`Gl::memory_barrier_shared`]
