# glClearTexImage
fills all a texture image with a constant value

## Parameters
- `texture`
  The name of an existing texture object containing the image to be
  cleared.

## Description
[`Gl::clear_tex_image`] fills all an image contained in a texture with
  an application supplied value. `texture` must be the name of an
  existing texture. Further, `texture` may not be the name of a buffer
  texture, nor may its internal format be compressed.
`format` and `type` specify the format and type of the source data and
  are interpreted as they are for [`Gl::tex_image3d`]. Textures with a
  base internal format of [`gl::DEPTH_COMPONENT`],
  [`gl::STENCIL_INDEX`], or [`gl::DEPTH_STENCIL`] require depth
  component, stencil, or depth-stencil component data respectively.
  Textures with other base internal formats require RGBA formats.
  Textures with integer internal formats require integer data.
`data` is a pointer to an array of between one and four components of
  texel data that will be used as the source for the constant fill
  value. The elements of data are converted by the GL into the internal
  format of the texture image (that was specified when the level was
  defined by any of the [`Gl::tex_image*`], [`Gl::tex_storage*`] or
  [`Gl::copy_tex_image*`] commands), and then used to fill the specified
  range of the destination texture level. If `data` is [`NULL`], then
  the pointer is ignored and the sub-range of the texture image is
  filled with zeros. If texture is a multisample texture, all the
  samples in a texel are cleared to the value specified by data.

## Notes
[`Gl::clear_tex_image`] is available only if the GL version is 4.4 or
  greater.

## Errors
- [`gl::INVALID_OPERATION`] is generated if `texture` is zero or not the
  name of an existing texture object.
- [`gl::INVALID_OPERATION`] is generated if `texture` is a buffer
  texture.
- [`gl::INVALID_OPERATION`] is generated if `texture` has a compressed
  internal format.
- [`gl::INVALID_OPERATION`] is generated if the base internal format is
  [`gl::DEPTH_COMPONENT`] and `format` is not [`gl::DEPTH_COMPONENT`].
- [`gl::INVALID_OPERATION`] is generated if the base internal format is
  [`gl::DEPTH_STENCIL`] and `format` is not [`gl::DEPTH_STENCIL`].
- [`gl::INVALID_OPERATION`] is generated if the base internal format is
  [`gl::STENCIL_INDEX`] and `format` is not [`gl::STENCIL_INDEX`].
- [`gl::INVALID_OPERATION`] is generated if the base internal format is
  [`gl::RGBA`] and `format` is [`gl::DEPTH_COMPONENT`],
  [`gl::STENCIL_INDEX`], or [`gl::DEPTH_STENCIL`].
- [`gl::INVALID_OPERATION`] is generated if the internal format is
  integer and `format` does not specify integer data.
- [`gl::INVALID_OPERATION`] is generated if the internal format is not
  integer and `format` specifies integer data.
- [`gl::INVALID_OPERATION`] is generated if the image array identified
  by `level` has not previously been defined by a call to
  [`Gl::tex_image*`] or [`Gl::tex_storage*`].

## See Also
- [`Gl::clear_tex_sub_image`]
- [`Gl::tex_storage1d`]
- [`Gl::tex_storage2d`]
- [`Gl::tex_storage3d`]
- [`Gl::tex_image1d`]
- [`Gl::tex_image2d`]
- [`Gl::tex_image3d`]
