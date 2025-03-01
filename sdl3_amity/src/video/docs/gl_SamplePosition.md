# gl_SamplePosition
contains the location of the current sample within the current
  fragment

## Description
[`Gl::sample_position`] is a fragment language input variable that
  contains the location within a fragment of the sample currently being
  processed. The x and y components of [`Gl::sample_position`] contain
  the sub-pixel coordinate of the current sample and will have values in
  the range 0.0 to 1.0. The sub-pixel coordinates of the center of the
  pixel are always (0.5, 0.5). Any static use of [`Gl::sample_position`]
  causes the entire fragment shader to be evaluated per-sample rather
  than per-fragment. When rendering to a non-multisample buffer, or if
  multisample rasterization is disabled, [`Gl::sample_position`] will be
  (0.5, 0.5).

## See Also
- [`Gl::sample_id`]
- [`Gl::sample_mask`]
- [`Gl::frag_coord`]
