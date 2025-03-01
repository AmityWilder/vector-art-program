# atomicCounterIncrement
atomically increment a counter and return the prior value

## Parameters
- `c`
  Specify the handle to the atomic counter to increment.

## Description
[`Gl::atomic_counter_increment`] atomically increments the value of
  the atomic counter `c` and returns its prior value.

## See Also
- [`Gl::atomic_counter_decrement`]
- [`Gl::atomic_counter`]
