# ldexp
assemble a floating point number from a value and exponent

## Parameters
- `x`
  Specifies the value to be used as a source of significand.

## Description
[`Gl::ldexp`] builds a floating point number from `x` and the
  corresponding integral exponent of two in `exp`, returning:
$None$ $$ $$ *significand* *\u{22C5}* ^{ } 2 *exponent*
If this product is too large to be represented in the floating point
  type, the result is undefined.

## See Also
- [`Gl::frexp`]
