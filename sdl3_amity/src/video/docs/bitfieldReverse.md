# bitfieldReverse
reverse the order of bits in an integer

## Parameters
- `value`
  Specifies the value whose bits to reverse.

## Description
[`Gl::bitfield_reverse`] returns the reversal of the bits of value.
  The bit numbered *n* will be taken from bit (*bits* - 1) - *n* of
  `value`, where *bits* is the total number of bits used to represent
  `value`.

## See Also
- [`Gl::bitfield_extract`]
- [`Gl::bitfield_insert`]
- [`Gl::bit_count`]
