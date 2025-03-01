# atomicCounterDecrement
atomically decrement a counter and return the prior value

## Parameters
- `c`
  Specify the handle to the atomic counter to decrement.

## Description
[`Gl::atomic_counter_decrement`] atomically decrements the value of
  the atomic counter `c` and returns its new value.

## See Also
- [`Gl::atomic_counter_increment`]
- [`Gl::atomic_counter`]
