# glDepthMask
enable or disable writing into the depth buffer

## Parameters
- `flag`
  Specifies whether the depth buffer is enabled for writing. If `flag`
  is [`gl::FALSE`], depth buffer writing is disabled. Otherwise, it is
  enabled. Initially, depth buffer writing is enabled.

## Description
[`Gl::depth_mask`] specifies whether the depth buffer is enabled for
  writing. If `flag` is [`gl::FALSE`], depth buffer writing is disabled.
  Otherwise, it is enabled. Initially, depth buffer writing is enabled.

## Notes
Even if the depth buffer exists and the depth mask is non-zero, the
  depth buffer is not updated if the depth test is disabled. In order to
  unconditionally write to the depth buffer, the depth test should be
  enabled and set to [`gl::ALWAYS`] (see [`Gl::depth_func`]).

## See Also
- [`Gl::color_mask`]
- [`Gl::depth_func`]
- [`Gl::depth_range`]
- [`Gl::stencil_mask`]
