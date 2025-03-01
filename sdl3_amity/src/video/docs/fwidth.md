# fwidth
return the sum of the absolute value of derivatives in x and y

## Parameters
- `p`
  Specifies the expression of which to take the partial derivative.

## Description
*Available only in the fragment shader*, these functions return the
  sum of the absolute derivatives in $x$ and $y$ using local
  differencing for the input argument `p`. [`Gl::fwidth`] is equivalent
  to ```c ``` . [`Gl::abs`](`dFdx`(p)) + [`Gl::abs`](`dFdy`(p))
  [`Gl::fwidth_coarse`] is equivalent to ```c ``` .
  [`Gl::abs`](`dFdxCoarse`(p)) + [`Gl::abs`](`dFdyCoarse`(p))
  [`Gl::fwidth_fine`] is equivalent to ```c ``` .
  [`Gl::abs`](`dFdxFine`(p)) + [`Gl::abs`](`dFdyFine`(p))

## See Also
- [`Gl::d_fdx`]
