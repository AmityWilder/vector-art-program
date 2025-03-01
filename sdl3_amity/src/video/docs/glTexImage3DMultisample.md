# glTexImage3DMultisample
establish the data storage, format, dimensions, and number of samples
  of a multisample texture's image

## Parameters
- `target`
  Specifies the target of the operation. `target` must be
  [`gl::TEXTURE_2D_MULTISAMPLE_ARRAY`] or
  [`gl::PROXY_TEXTURE_2D_MULTISAMPLE_ARRAY`].

## Description
[`Gl::tex_image3d_multisample`] establishes the data storage, format,
  dimensions and number of samples of a multisample texture's image.
`target` must be [`gl::TEXTURE_2D_MULTISAMPLE_ARRAY`] or
  [`gl::PROXY_TEXTURE_2D_MULTISAMPLE_ARRAY`]. `width` and `height`are
  the dimensions in texels of the texture, and must be in the range zero
  to the value of [`gl::MAX_TEXTURE_SIZE`] minus one. `depth` is the
  number of array slices in the array texture's image. `samples`
  specifies the number of samples in the image and must be in the range
  zero to the value of [`gl::MAX_SAMPLES`] minus one.
`internalformat` must be a color-renderable, depth-renderable, or
  stencil-renderable format.
If `fixedsamplelocations` is [`gl::TRUE`], the image will use
  identical sample locations and the same number of samples for all
  texels in the image, and the sample locations will not depend on the
  internal format or size of the image.
When a multisample texture is accessed in a shader, the access takes
  one vector of integers describing which texel to fetch and an integer
  corresponding to the sample numbers describing which sample within the
  texel to fetch. No standard sampling instructions are allowed on the
  multisample texture targets.

## Notes
[`Gl::tex_image2d_multisample`] is available only if the GL version is
  3.2 or greater.

## Errors
- [`gl::INVALID_OPERATION`] is generated if `internalformat` is a depth-
  or stencil-renderable format and `samples` is greater than the value
  of [`gl::MAX_DEPTH_TEXTURE_SAMPLES`].
- [`gl::INVALID_OPERATION`] is generated if `internalformat` is a color-
  renderable format and `samples` is greater than the value of
  [`gl::MAX_COLOR_TEXTURE_SAMPLES`].
- [`gl::INVALID_OPERATION`] is generated if `internalformat` is a signed
  or unsigned integer format and `samples` is greater than the value of
  [`gl::MAX_INTEGER_SAMPLES`].
- [`gl::INVALID_VALUE`] is generated if either `width` or `height`
  negative or is greater than [`gl::MAX_TEXTURE_SIZE`].
- [`gl::INVALID_VALUE`] is generated if `depth` is negative or is
  greater than [`gl::MAX_ARRAY_TEXTURE_LAYERS`].
- [`gl::INVALID_VALUE`] is generated if `samples` is zero.
- [`gl::INVALID_OPERATION`] is generated if `samples` is greater than
  the maximum number of samples supported for this `target` and
  `internalformat`.

## See Also
- [`Gl::tex_image3d`]
- [`Gl::tex_image2d_multisample`]
