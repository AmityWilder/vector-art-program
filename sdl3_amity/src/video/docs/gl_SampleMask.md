# gl_SampleMask
specifies the sample coverage mask for the current fragment

## Description
[`Gl::sample_mask`] is a fragment language output array that may be
  used to set the sample mask for the fragment being processed. Coverage
  for the current fragment will become the logical AND of the coverage
  mask and the output [`Gl::sample_mask`]. That is, setting a bit in
  [`Gl::sample_mask`] to zero will cause the corresponding sample to be
  considered uncovered for the purposes of multisample fragment
  operations. However, setting sample mask bits back to one will never
  enable samples not covered by the original primitive. Bit *B* of mask
  [`Gl::sample_mask[m]`] corresponds to sample 32 * *M* + *B*. This
  array must be sized in the fragment shader either implicitly or
  explicitly to be the same size as the implementation-dependent maximum
  sample-mask (as an array of 32-bit elements), determined by the
  maximum number of samples. If the fragment shader statically assigns a
  value to [`Gl::sample_mask`], the sample mask will be undefined for
  any array elements of any fragment shader invocation that fails to
  assign a value. If a shader does not statically assign a value to
  [`Gl::sample_mask`], the sample mask has no effect on the processing
  of a fragment. If the fragment shader is being evaluated at any
  frequency other than per-fragment, bits within the sample mask not
  corresponding to the current fragment shader invocation are ignored.

## See Also
- [`Gl::sample_mask_in`]
- [`Gl::sample_id`]
- [`Gl::sample_position`]
- [`Gl::frag_coord`]
