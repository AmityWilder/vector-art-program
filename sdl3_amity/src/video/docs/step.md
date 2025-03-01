# step
generate a step function by comparing two values

## Parameters
- `edge`
  Specifies the location of the edge of the step function.

## Description
[`Gl::step`] generates a step function by comparing `x` to `edge`.
For element *i* of the return value, 0.0 is returned if `x`[*i*] <
  `edge`[*i*], and 1.0 is returned otherwise.

## See Also
- [`Gl::mix`]
- [`Gl::smoothstep`]
