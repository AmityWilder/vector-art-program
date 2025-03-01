# gl_NumSamples
contains the total number of samples in the framebuffer

## Description
[`Gl::num_samples`] is a fragment language input variable that
  contains the number of samples in the framebuffer, or 1 if rendering
  to a non-multisample framebuffer. [`Gl::num_samples`] is the sample
  count of the framebuffer regardless of whether multisample
  rasterization is enabled or not.

## See Also
- [`Gl::sample_position`]
- [`Gl::sample_id`]
