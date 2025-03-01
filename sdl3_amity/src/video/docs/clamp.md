# clamp
constrain a value to lie between two further values

## Parameters
- `x`
  Specify the value to constrain.

## Description
[`Gl::clamp`] returns the value of `x` constrained to the range
  `minVal` to `maxVal`. The returned value is computed as
  [`Gl::min`]([`Gl::max`](`x`, `minVal`), `maxVal`).

## See Also
- [`Gl::min`]
- [`Gl::max`]
