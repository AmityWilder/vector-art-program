# gl_SampleMaskIn
contains the computed sample coverage mask for the current fragment

## Description
[`Gl::sample_mask_in`] is a fragment language that indicates the set
  of samples covered by the primitive generating the fragment during
  multisample rasterization. It has a sample bit set if and only if the
  sample is considered covered for this fragment shader invocation. Bit
  *B* of mask [`Gl::sample_mask[m]`] corresponds to sample 32 * *M* +
  *B*. The array has [`Gl::ceil`](*s* / 32) elements where *s* is the
  maximum number of color samples supported by the implementation.

## See Also
- [`Gl::sample_mask`]
- [`Gl::sample_id`]
- [`Gl::sample_position`]
- [`Gl::frag_coord`]
