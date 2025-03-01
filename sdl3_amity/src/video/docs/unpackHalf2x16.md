# unpackHalf2x16
convert two 16-bit floating-point values packed into a single 32-bit
  integer into a vector of two 32-bit floating-point quantities

## Parameters
- `v`
  Specify a single 32-bit unsigned integer values that contains two
  16-bit floating point values to be unpacked.

## Description
[`Gl::unpack_half2x16`] returns a two-component floating-point vector
  with components obtained by unpacking a 32-bit unsigned integer into a
  pair of 16-bit values, interpreting those values as 16-bit floating-
  point numbers according to the OpenGL Specification, and converting
  them to 32-bit floating-point values. The first component of the
  vector is obtained from the 16 least-significant bits of v; the second
  component is obtained from the 16 most-significant bits of v.

## See Also
- [`Gl::pack_double2x32`]
- [`Gl::unpack_double2x32`]
- [`Gl::pack_half2x16`]
