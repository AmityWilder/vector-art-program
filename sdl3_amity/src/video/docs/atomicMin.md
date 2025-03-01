# atomicMin
perform an atomic min operation to a variable

## Parameters
- `mem`
  The variable to use as the target of the operation.

## Description
[`Gl::atomic_min`] performs an atomic comparison of `data` to the
  contents of `mem`, writes the minimum value into `mem` and returns the
  original contents of `mem` from before the comparison occurred. The
  contents of the memory being updated by the atomic operation are
  guaranteed not to be modified by any other assignment or atomic memory
  function in any shader invocation between the time the original value
  is read and the time the new value is written.
Atomic memory functions are supported only for a limited set of
  variables. A shader will fail to compile if the value passed to the
  mem argument of an atomic memory function does not correspond to a
  buffer or shared variable. It is acceptable to pass an element of an
  array or a single component of a vector to the mem argument of an
  atomic memory function, as long as the underlying array or vector is a
  buffer or shared variable.

## See Also
- [`Gl::atomic_add`]
- [`Gl::atomic_and`]
- [`Gl::atomic_or`]
- [`Gl::atomic_xor`]
- [`Gl::atomic_max`]
- [`Gl::atomic_exchange`]
- [`Gl::atomic_comp_swap`]
