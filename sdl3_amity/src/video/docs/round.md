# round
find the nearest integer to the parameter

## Parameters
- `x`
  Specify the value to evaluate.

## Description
[`Gl::round`] returns a value equal to the nearest integer to `x`. The
  fraction 0.5 will round in a direction chosen by the implementation,
  presumably the direction that is fastest. This includes the
  possibility that [`Gl::round`](`x`) returns the same value as
  [`Gl::round_even`](`x`) for all values of `x`.

## See Also
- [`Gl::floor`]
- [`Gl::round_even`]
