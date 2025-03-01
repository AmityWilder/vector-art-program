# interpolateAtSample
sample a varying at the location of a specified sample

## Parameters
- `interpolant`
  Specifies the interpolant to be sampled at the location of sample
  `sample`.

## Description
[`Gl::interpolate_at_sample`] returns the value of the input varying
  `interpolant` sampled at the location of sample number `sample`. If
  multisample buffers are not available, the input varying will be
  evaluated at the center of the pixel. If sample `sample` does not
  exist, the position used to interpolate the input varying is
  undefined.

## See Also
- [`Gl::interpolate_at_centroid`]
- [`Gl::interpolate_at_offset`]
