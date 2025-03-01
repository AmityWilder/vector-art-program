# glCopyTexImage2D
copy pixels into a 2D texture image

## Parameters
- `target`
  Specifies the target texture. Must be [`gl::TEXTURE_2D`],
  [`gl::TEXTURE_CUBE_MAP_POSITIVE_X`],
  [`gl::TEXTURE_CUBE_MAP_NEGATIVE_X`],
  [`gl::TEXTURE_CUBE_MAP_POSITIVE_Y`],
  [`gl::TEXTURE_CUBE_MAP_NEGATIVE_Y`],
  [`gl::TEXTURE_CUBE_MAP_POSITIVE_Z`], or
  [`gl::TEXTURE_CUBE_MAP_NEGATIVE_Z`].

## Description
[`Gl::copy_tex_image2d`] defines a two-dimensional texture image, or
  cube-map texture image with pixels from the current
  [`gl::READ_BUFFER`].
The screen-aligned pixel rectangle with lower left corner at ( `x`,
  `y`) and with a width of $None$ and a height of $$ $$ *width* $None$
  defines the texture array at the mipmap level specified by $$ $$
  *height* `level`. `internalformat` specifies the internal format of
  the texture array.
The pixels in the rectangle are processed exactly as if
  [`Gl::read_pixels`] had been called, but the process stops just before
  final conversion. At this point all pixel component values are clamped
  to the range $None$ and then converted to the texture's internal
  format for storage in the texel array. $$ $$ ```c ``` 0 1
Pixel ordering is such that lower $None$ and $$ None $$ *x*$None$
  screen coordinates correspond to lower $$ None $$ *y*$None$ and $$
  None $$ *s*$None$ texture coordinates. $$ None $$ *t*
If any of the pixels within the specified rectangle of the current
  [`gl::READ_BUFFER`] are outside the window associated with the current
  rendering context, then the values obtained for those pixels are
  undefined.
When `internalformat` is one of the sRGB types, the GL does not
  automatically convert the source pixels to the sRGB color space. In
  this case, the [`Gl::pixel_map`] function can be used to accomplish
  the conversion.

## Notes
1, 2, 3, and 4 are not accepted values for `internalformat`.
An image with height or width of 0 indicates a NULL texture.
[`gl::STENCIL_INDEX8`] is accepted for `internalformat` only if the GL
  version is 4.4 or higher.

## Errors
- [`gl::INVALID_ENUM`] is generated if `target` is not
  [`gl::TEXTURE_2D`], [`gl::TEXTURE_CUBE_MAP_POSITIVE_X`],
  [`gl::TEXTURE_CUBE_MAP_NEGATIVE_X`],
  [`gl::TEXTURE_CUBE_MAP_POSITIVE_Y`],
  [`gl::TEXTURE_CUBE_MAP_NEGATIVE_Y`],
  [`gl::TEXTURE_CUBE_MAP_POSITIVE_Z`], or
  [`gl::TEXTURE_CUBE_MAP_NEGATIVE_Z`].
- [`gl::INVALID_VALUE`] is generated if `level` is less than 0.
- [`gl::INVALID_VALUE`] may be generated if `level` is greater than
  $None$, where $$ $$ _{None} *log* 2 *\u{2062}* *max* $None$ is the
  returned value of $$ None $$ *max*[`gl::MAX_TEXTURE_SIZE`].
- [`gl::INVALID_VALUE`] is generated if `width` is less than 0 or
  greater than [`gl::MAX_TEXTURE_SIZE`].
- [`gl::INVALID_VALUE`] is generated if `border` is not 0.
- [`gl::INVALID_VALUE`] is generated if `internalformat` is not an
  accepted format.
- [`gl::INVALID_OPERATION`] is generated if `internalformat` is
  [`gl::DEPTH_COMPONENT`], [`gl::DEPTH_COMPONENT16`],
  [`gl::DEPTH_COMPONENT24`], or [`gl::DEPTH_COMPONENT32`] and there is
  no depth buffer.

## See Also
- [`Gl::copy_tex_image1d`]
- [`Gl::copy_tex_sub_image1d`]
- [`Gl::copy_tex_sub_image2d`]
- [`Gl::pixel_store`]
- [`Gl::tex_image1d`]
- [`Gl::tex_image2d`]
- [`Gl::tex_sub_image1d`]
- [`Gl::tex_sub_image2d`]
- [`Gl::tex_parameter`]
