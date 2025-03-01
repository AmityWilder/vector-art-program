# bitfieldExtract
extract a range of bits from an integer

## Parameters
- `value`
  Specifies the integer from which to extract bits.

## Description
[`Gl::bitfield_extract`] extracts a subset of the bits of `value` and
  returns it in the least significant bits of the result. The range of
  bits extracted is [`offset`, `offset` + `bits` - 1].
For unsigned data types, the most significant bits of the result will
  be set to zero. For signed data types, the most significant bits will
  be set to the value of bit `offset` + `base` - 1 (i.e., it is *sign
  extended* to the width of the return type).
If `bits` is zero, the result will be zero. The result will be
  undefined if `offset` or `bits` is negative, or if the sum of `offset`
  and `bits` is greater than the number of bits used to store the
  operand.

## See Also
- [`Gl::bitfield_extract`]
