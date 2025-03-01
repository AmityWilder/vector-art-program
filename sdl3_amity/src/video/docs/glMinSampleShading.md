# glMinSampleShading
specifies minimum rate at which sample shading takes place

## Parameters
- `value`
  Specifies the rate at which samples are shaded within each covered
  pixel.

## Description
[`Gl::min_sample_shading`] specifies the rate at which samples are
  shaded within a covered pixel. Sample-rate shading is enabled by
  calling [`Gl::enable`] with the parameter [`gl::SAMPLE_SHADING`]. If
  [`gl::MULTISAMPLE`] or [`gl::SAMPLE_SHADING`] is disabled, sample
  shading has no effect. Otherwise, an implementation must provide at
  least as many unique color values for each covered fragment as
  specified by `value` times `samples` where `samples` is the value of
  [`gl::SAMPLES`] for the current framebuffer. At least 1 sample for
  each covered fragment is generated.
A `value` of 1.0 indicates that each sample in the framebuffer should
  be independently shaded. A `value` of 0.0 effectively allows the GL to
  ignore sample rate shading. Any value between 0.0 and 1.0 allows the
  GL to shade only a subset of the total samples within each covered
  fragment. Which samples are shaded and the algorithm used to select
  that subset of the fragment's samples is implementation dependent.

## Notes
The type of the `value` parameter was changed from GLclampf to
  GLfloat. This change is transparent to user code and is described in
  detail on the [`Gl::removed_types`] page.

## Errors
- None.

## See Also
- [`Gl::removed_types`]
