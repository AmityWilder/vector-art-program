# atomicCompSwap
perform an atomic compare-exchange operation to a variable

## Parameters
- `mem`
  The variable to use as the target of the operation.

## Description
[`Gl::atomic_comp_swap`] performs an atomic comparison of `compare`
  with the contents of `mem`. If the content of `mem` is equal to
  `compare`, then the content of `data` is written into `mem`, otherwise
  the content of `mem` is unmodifed. The function returns the original
  content of `mem` regardless of the outcome of the comparison. The
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
- [`Gl::atomic_min`]
- [`Gl::atomic_max`]
- [`Gl::atomic_exchange`]
