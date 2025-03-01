# findLSB
find the index of the least significant bit set to 1 in an integer

## Parameters
- `value`
  Specifies the value whose bits to scan.

## Description
[`Gl::find_lsb`] returns the bit number of the least significant bit
  that is set to 1 in the binary representation of `value`. If `value`
  is zero, -1 will be returned.

## See Also
- [`Gl::find_msb`]
