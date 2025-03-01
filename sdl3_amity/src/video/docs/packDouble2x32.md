# packDouble2x32
create a double-precision value from a pair of unsigned integers

## Parameters
- `v`
  Specifies a vector of two unsigned integers to be packed into a single
  double-precision result.

## Description
[`Gl::pack_double2x32`] packs the component of parameter `v` into a
  64-bit value. If an IEEE-754 infinity or NaN is created, it will not
  signal and the resulting floating-point value is undefined. Otherwise,
  the bit-level representation of `v` is preserved. The first vector
  component ( ```c v[0] ``` ) specifies the 32 least significant bits of
  the result; the second component ( ```c v[1] ``` ) specifies the 32
  most significant bits.

## See Also
- [`Gl::unpack_double2x32`]
