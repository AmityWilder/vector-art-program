# glSampleMaski
set the value of a sub-word of the sample mask

## Parameters
- `maskNumber`
  Specifies which 32-bit sub-word of the sample mask to update.

## Description
[`Gl::sample_maski`] sets one 32-bit sub-word of the multi-word sample
  mask, [`gl::SAMPLE_MASK_VALUE`].
`maskIndex` specifies which 32-bit sub-word of the sample mask to
  update, and `mask` specifies the new value to use for that sub-word.
  `maskIndex` must be less than the value of
  [`gl::MAX_SAMPLE_MASK_WORDS`]. Bit *B* of mask word *M* corresponds to
  sample 32 x *M* + *B*.

## Notes
[`Gl::sample_maski`] is available only if the GL version is 3.2 or
  greater, or if the ```c ARB_texture_multisample ``` extension is
  supported.

## Errors
- [`gl::INVALID_VALUE`] is generated if `maskIndex` is greater than or
  equal to the value of [`gl::MAX_SAMPLE_MASK_WORDS`].

## See Also
- [`Gl::gen_renderbuffers`]
- [`Gl::bind_renderbuffer`]
- [`Gl::renderbuffer_storage_multisample`]
- [`Gl::framebuffer_renderbuffer`]
- [`Gl::delete_renderbuffers`]
