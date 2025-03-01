# modf
separate a value into its integer and fractional components

## Parameters
- `x`
  Specify the value to separate.

## Description
[`Gl::modf`] separates a floating point value `x` into its integer and
  fractional parts. The fractional part of the number is returned from
  the function and the integer part (as a floating point quantity) is
  returned in the output parameter `i`.

## See Also
- [`Gl::fract`]
- [`Gl::floor`]
