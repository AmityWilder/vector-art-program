# smoothstep
perform Hermite interpolation between two values

## Parameters
- `edge0`
  Specifies the value of the lower edge of the Hermite function.

## Description
[`Gl::smoothstep`] performs smooth Hermite interpolation between 0 and
  1 when `edge0` < `x` < `edge1`. This is useful in cases where a
  threshold function with a smooth transition is desired.
  [`Gl::smoothstep`] is equivalent to:
```c genType t; /* Or genDType t; */ t = clamp((x - edge0) / (edge1 -
  edge0), 0.0, 1.0); return t * t * (3.0 - 2.0 * t); ```
Results are undefined if `edge0` \u{2265} `edge1`.

## See Also
- [`Gl::mix`]
- [`Gl::step`]
