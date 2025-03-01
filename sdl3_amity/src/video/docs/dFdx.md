# dFdx, dFdy
return the partial derivative of an argument with respect to x or y

## Parameters
- `p`
  Specifies the expression of which to take the partial derivative.

## Description
*Available only in the fragment shader*, these functions return the
  partial derivative of expression `p` with respect to the window $x$
  coordinate (for [`Gl::d_fdx*`]) and $y$ coordinate (for
  [`Gl::d_fdy*`]).
[`Gl::d_fdx_fine`] and [`Gl::d_fdy_fine`] calculate derivatives using
  local differencing based on the value of `p` for the current fragment
  and its immediate neighbor(s).
[`Gl::d_fdx_coarse`] and [`Gl::d_fdy_coarse`] calculate derivatives
  using local differencing based on the value of `p` for the current
  fragment's neighbors, and will possibly, but not necessarily, include
  the value for the current fragment. That is, over a given area, the
  implementation can compute derivatives in fewer unique locations than
  would be allowed for the corresponding [`Gl::d_fdx_fine`] and
  [`Gl::d_fdy_fine`] functions.
[`Gl::d_fdx`] returns either [`Gl::d_fdx_coarse`] or
  [`Gl::d_fdx_fine`]. [`Gl::d_fdy`] returns either [`Gl::d_fdy_coarse`]
  or [`Gl::d_fdy_fine`]. The implementation may choose which calculation
  to perform based upon factors such as performance or the value of the
  API [`gl::FRAGMENT_SHADER_DERIVATIVE_HINT`] hint.
Expressions that imply higher order derivatives such as ```c
  dFdx(dFdx(n)) ``` have undefined results, as do mixed-order
  derivatives such as ```c dFdx(dFdy(n)) ``` . It is assumed that the
  expression `p` is continuous and therefore, expressions evaluated via
  non-uniform control flow may be undefined.

## See Also
- [`Gl::fwidth`]
- [`Gl::hint`]
