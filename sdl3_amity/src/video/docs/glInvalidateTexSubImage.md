# glInvalidateTexSubImage
invalidate a region of a texture image

## Parameters
- `texture`
  The name of a texture object a subregion of which to invalidate.

## Description
[`Gl::invalidate_tex_sub_image`] invalidates all or part of a texture
  image. `texture` and `level` indicated which texture image is being
  invalidated. After this command, data in that subregion have undefined
  values. `xoffset`, `yoffset`, `zoffset`, `width`, `height`, and
  `depth` are interpreted as they are in [`Gl::tex_sub_image3d`]. For
  texture targets that don't have certain dimensions, this command
  treats those dimensions as having a size of 1. For example, to
  invalidate a portion of a two- dimensional texture, the application
  would use `zoffset` equal to zero and `depth` equal to one. Cube map
  textures are treated as an array of six slices in the z-dimension,
  where a value of `zoffset` is interpreted as specifying face
  [`gl::TEXTURE_CUBE_MAP_POSITIVE_X`] + `zoffset`.
`level` must be greater than or equal to zero and be less than the
  base 2 logarithm of the maximum texture width, height, or depth.
  `xoffset`, `yoffset` and `zoffset` must be greater than or equal to
  zero and be less than the width, height or depth of the image,
  respectively. Furthermore, `xoffset` + `width`, `yoffset` + `height`,
  and `zoffset` + `depth` must be less than or equal to the width,
  height or depth of the image, respectively.
For textures of targets [`gl::TEXTURE_RECTANGLE`],
  [`gl::TEXTURE_BUFFER`], [`gl::TEXTURE_2D_MULTISAMPLE`], or
  [`gl::TEXTURE_2D_MULTISAMPLE_ARRAY`], level must be zero.

## Errors
- [`gl::INVALID_VALUE`] is generated if `xoffset`, `yoffset` or
  `zoffset` is less than zero, or if any of them is greater than the
  size of the image in the corresponding dimension.
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
- [`Gl::invalidate_tex_image`]
- [`Gl::invalidate_buffer_sub_data`]
- [`Gl::invalidate_buffer_data`]
- [`Gl::invalidate_framebuffer`]
- [`Gl::invalidate_sub_framebuffer`]
