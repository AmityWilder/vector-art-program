# memoryBarrierAtomicCounter
controls the ordering of operations on atomic counters issued by a
  single shader invocation

## Description
[`Gl::memory_barrier_atomic_counter`] waits on the completion of all
  accesses resulting from the use of atomic counters and then returns
  with no other effect. When this function returns, the results of any
  modifications to the value of atomic counters will be visible to any
  access to the same counter from other shader invocations. In
  particular, any modifications made in one shader stage are guaranteed
  to be visible to accesses performed by shader invocations in
  subsequent stages when those invocations were triggered by the
  execution of the original shader invocation (e.g., fragment shader
  invocations for a primitive resulting from a particular geometry
  shader invocation).

## See Also
- [`Gl::memory_barrier`]
