# umulExtended
perform a 32- by 32-bit multiply to produce a 64-bit result

## Parameters
- `x`
  Specifies the first multiplicand.

## Description
[`Gl::umul_extended`] and [`Gl::imul_extended`] perform multiplication
  of the two 32-bit integer quantities `x` and `y`, producing a 64-bit
  integer result. The 32 least significant bits of this product are
  returned in `lsb` and the 32 most significant bits are returned in
  `msb`. [`Gl::umul_extended`] and [`Gl::imul_extended`] perform
  unsigned and signed multiplication, respectively.

## See Also
- [`Gl::uadd_carry`]
