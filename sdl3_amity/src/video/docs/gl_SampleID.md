# gl_SampleID
contains the index of the sample currently being processed

## Description
[`Gl::sample_id`] is a fragment language input variable that contains
  the index of the sample currently being processed. This variable is in
  the range 0 to [`Gl::num_samples`] - 1, where [`Gl::num_samples`] is
  the total number of samples in each fragment for the current
  framebuffer (and thus 1 if rendering to a non-multisample buffer). Any
  static use of this variable in a fragment shader causes the entire
  shader to be evaluated per-sample rather than per-fragment.
When rendering to a non-multisample buffer, or if multisample
  rasterization is disabled, [`Gl::sample_id`] will always be zero.
  [`Gl::num_samples`] is the sample count of the framebuffer regardless
  of whether multisample rasterization is enabled or not.

## See Also
- [`Gl::sample_position`]
- [`Gl::num_samples`]
