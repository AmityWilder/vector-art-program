# unpackDouble2x32
produce two unsigned integers containing the bit encoding of a double
  precision floating point value

## Parameters
- `d`
  Specifies double precision value to break into a pair of unsigned
  integers.

## Description
[`Gl::unpack_double2x32`] returns a two-component unsigned integer
  vector representation of `d`. The bit-level representation of `d` is
  preserved. The first component of the returned vector contains the 32
  least significant bits of the double; the second component consists
  the 32 most significant bits.

## See Also
- [`Gl::pack_double2x32`]
