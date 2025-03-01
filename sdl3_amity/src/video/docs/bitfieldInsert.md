# bitfieldInsert
insert a range of bits into an integer

## Parameters
- `base`
  Specifies the integer into which to insert `insert`.

## Description
[`Gl::bitfield_insert`] inserts the `bits` least significant bits of
  `insert` into `base` at offset `offset`. The returned value will have
  bits [`offset`, `offset` + `bits` + 1] taken from [0, `bits` - 1] of
  `insert` and all other bits taken directly from the corresponding bits
  of `base`. If `bits` is zero, the result will simply be the original
  value of `base`. The result will be undefined if `offset` or `bits` is
  negative, or if the sum of `offset` and `bits` is greater than the
  number of bits used to store the operand.

## See Also
- [`Gl::bitfield_extract`]
