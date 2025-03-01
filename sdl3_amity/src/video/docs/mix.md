# mix
linearly interpolate between two values

## Parameters
- `x`
  Specify the start of the range in which to interpolate.

## Description
[`Gl::mix`] performs a linear interpolation between `x` and `y` using
  `a` to weight between them. The return value is computed as $x \times
  (1 - a) + y \times a$.
The variants of [`Gl::mix`] where `a` is [`genBType`] select which
  vector each returned component comes from. For a component of `a` that
  is false, the corresponding component of `x` is returned. For a
  component of `a` that is true, the corresponding component of `y` is
  returned. Components of `x` and `y` that are not selected are allowed
  to be invalid floating-point values and will have no effect on the
  results.

## See Also
- [`Gl::min`]
- [`Gl::max`]
