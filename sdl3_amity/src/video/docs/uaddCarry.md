# uaddCarry
add unsigned integers and generate carry

## Parameters
- `x`
  Specifies the first vector to be used in the summation operation.

## Description
[`Gl::uadd_carry`] adds two 32-bit unsigned integer variables (scalars
  or vectors) and generates a 32-bit unsigned integer result, along with
  a carry output. The result is the sum of `x` and `y` modulo $None$.
  The value $$ None $$ ^{None}232`carry` is set to 0 if the sum is less
  than $None$ and to 1 otherwise. $$ None $$ ^{None}232

## See Also
- [`Gl::usub_borrow`]
