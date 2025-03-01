# findMSB
find the index of the most significant bit set to 1 in an integer

## Parameters
- `value`
  Specifies the value whose bits to scan.

## Description
[`Gl::find_msb`] returns the bit number of the most significant bit
  that is set to 1 in the binary representation of `value`. For positive
  integers, the result will be the bit number of the most significant
  bit that is set to 1. For negative integers, the result will be the
  bit number of the most significant bit set to 0. For a `value` of zero
  or negative 1, -1 will be returned.

## See Also
- [`Gl::find_lsb`]
