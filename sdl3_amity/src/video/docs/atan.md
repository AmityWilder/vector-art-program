# atan
return the arc-tangent of the parameters

## Parameters
- `y`
  Specify the numerator of the fraction whose arctangent to return.

## Description
[`Gl::atan`] returns either the angle whose trigonometric arctangent
  is $y \over x$ or `y_over_x`, depending on which overload is invoked.
  In the first overload, the signs of $y$ and $x$ are used to determine
  the quadrant that the angle lies in. The value returned by
  [`Gl::atan`] in this case is in the range $[-\pi,\pi]$. The result is
  undefined if $x = 0$.
For the second overload, [`Gl::atan`] returns the angle whose tangent
  is `y_over_x`. The value returned in this case is in the range $[-{\pi
  \over 2 },{\pi \over 2}]$.

## See Also
- [`Gl::sin`]
- [`Gl::cos`]
- [`Gl::tan`]
