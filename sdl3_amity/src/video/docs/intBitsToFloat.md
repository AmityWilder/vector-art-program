# intBitsToFloat, uintBitsToFloat
produce a floating point using an encoding supplied as an integer

## Parameters
- `x`
  Specifies the bit encoding to return as a floating point value.

## Description
[`Gl::int_bits_to_float`] and [`Gl::uint_bits_to_float`] return the
  encoding passed in parameter `x` as a floating-point value. If the
  encoding of a NaN is passed in `x`, it will not signal and the
  resulting value will be undefined. If the encoding of a floating point
  infinity is passed in parameter `x`, the resulting floating-point
  value is the corresponding (positive or negative) floating point
  infinity.

## See Also
- [`Gl::float_bits_to_int`]
- [`Gl::isnan`]
- [`Gl::isinf`]
