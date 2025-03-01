# frexp
split a floating point number

## Parameters
- `x`
  Specifies the value from which significand and exponent are to be
  extracted.

## Description
[`Gl::frexp`] extracts `x` into a floating-point significand in the
  range [0.5, 1.0) and in integral exponent of two, such that:
$None$ $$ $$ *x* *=* *significand* *\u{22C5}* ^{ } 2 *exponent*
The significand is returned by the function and the exponent is
  returned in the output parameter `exp`. For a floating-point value of
  zero, the significand and exponent are both zero. For a floating-point
  value that is an infinity or a floating-point NaN, the results are
  undefined.

## See Also
- [`Gl::ldexp`]
