# packHalf2x16
convert two 32-bit floating-point quantities to 16-bit quantities and
  pack them into a single 32-bit integer

## Parameters
- `v`
  Specify a vector of two 32-bit floating point values that are to be
  converted to 16-bit representation and packed into the result.

## Description
[`Gl::pack_half2x16`] returns an unsigned integer obtained by
  converting the components of a two-component floating-point vector to
  the 16-bit floating-point representation found in the OpenGL
  Specification, and then packing these two 16-bit integers into a
  32-bit unsigned integer. The first vector component specifies the 16
  least-significant bits of the result; the second component specifies
  the 16 most-significant bits.

## See Also
- [`Gl::pack_double2x32`]
- [`Gl::unpack_double2x32`]
- [`Gl::unpack_half2x16`]
