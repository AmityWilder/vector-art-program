# interpolateAtOffset
sample a varying at specified offset from the center of a pixel

## Parameters
- `interpolant`
  Specifies the interpolant to be sampled at the specified offset.

## Description
[`Gl::interpolate_at_offset`] returns the value of the input varying
  `interpolant` sampled at an offset from the center of the pixel
  specified by `offset`. The two floating-point components of `offset`
  give the offset in pixels in the *x* and *y* directions from the
  center of the pixel, respectively. An offset of (0, 0) identifies the
  center of the pixel. The range and granularity of offsets supported by
  this function is implementation-dependent.

## See Also
- [`Gl::interpolate_at_centroid`]
- [`Gl::interpolate_at_sample`]
