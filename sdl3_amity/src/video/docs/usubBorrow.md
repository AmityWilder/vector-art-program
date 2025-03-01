# usubBorrow
subtract unsigned integers and generate borrow

## Parameters
- `x`
  Specifies the first vector to be used in the subtraction operation.

## Description
[`Gl::usub_borrow`] subtracts two 32-bit unsigned integer variables
  (scalars or vectors) and generates a 32-bit unsigned integer result,
  along with a borrow output. The result is the difference of `x` and
  `y` if non-negative, or $None$ plus that difference otherwise. The
  value $$ None $$ ^{None}232`borrow` is set to 0 if `x` \u{2265} `y`
  and to 1 otherwise.

## See Also
- [`Gl::uadd_carry`]
