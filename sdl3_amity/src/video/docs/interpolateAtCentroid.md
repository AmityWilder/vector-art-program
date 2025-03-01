# interpolateAtCentroid
sample a varying at the centroid of a pixel

## Parameters
- `interpolant`
  Specifies the interpolant to be sampled at the pixel centroid.

## Description
[`Gl::interpolate_at_centroid`] returns the value of the input varying
  `interpolant` sampled at a location inside both the pixel and the
  primitive being processed. The value obtained would be the value
  assigned to the input variable if declared with the ```c centroid ```
  qualifier.

## See Also
- [`Gl::interpolate_at_sample`]
- [`Gl::interpolate_at_offset`]
