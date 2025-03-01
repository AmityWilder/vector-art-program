# glInvalidateTexImage
invalidate the entirety a texture image

## Parameters
- `texture`
  The name of a texture object to invalidate.

## Description
[`Gl::invalidate_tex_sub_image`] invalidates all of a texture image.
  `texture` and `level` indicated which texture image is being
  invalidated. After this command, data in the texture image has
  undefined values.
`level` must be greater than or equal to zero and be less than the
  base 2 logarithm of the maximum texture width, height, or depth.
For textures of targets [`gl::TEXTURE_RECTANGLE`],
  [`gl::TEXTURE_BUFFER`], [`gl::TEXTURE_2D_MULTISAMPLE`], or
  [`gl::TEXTURE_2D_MULTISAMPLE_ARRAY`], level must be zero.

## Errors
- [`gl::INVALID_VALUE`] is generated if `level` is less than zero or if
  it is greater or equal to the base 2 logarithm of the maximum texture
  width, height, or depth.
- [`gl::INVALID_VALUE`] is generated if the target of `texture` is any
  of [`gl::TEXTURE_RECTANGLE`], [`gl::TEXTURE_BUFFER`],
  [`gl::TEXTURE_2D_MULTISAMPLE`], or
  [`gl::TEXTURE_2D_MULTISAMPLE_ARRAY`] and `level` is not zero.
- [`gl::INVALID_VALUE`] is generated if `texture` is not the name of an
  existing texture object.

## See Also
- [`Gl::invalidate_tex_sub_image`]
- [`Gl::invalidate_buffer_sub_data`]
- [`Gl::invalidate_buffer_data`]
- [`Gl::invalidate_framebuffer`]
- [`Gl::invalidate_sub_framebuffer`]
