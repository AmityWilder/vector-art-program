# glCopyTexImage1D
copy pixels into a 1D texture image

## Parameters
- `target`
  Specifies the target texture. Must be [`gl::TEXTURE_1D`].

## Description
[`Gl::copy_tex_image1d`] defines a one-dimensional texture image with
  pixels from the current [`gl::READ_BUFFER`].
The screen-aligned pixel row with left corner at $None$ and with a
  length of $$ $$ ```c ``` *x* *y* $None$ defines the texture array at
  the mipmap level specified by $$ $$ *width* `level`. `internalformat`
  specifies the internal format of the texture array.
The pixels in the row are processed exactly as if [`Gl::read_pixels`]
  had been called, but the process stops just before final conversion.
  At this point all pixel component values are clamped to the range
  $None$ and then converted to the texture's internal format for storage
  in the texel array. $$ $$ ```c ``` 0 1
Pixel ordering is such that lower $None$ screen coordinates correspond
  to lower texture coordinates. $$ None $$ *x*
If any of the pixels within the specified row of the current
  [`gl::READ_BUFFER`] are outside the window associated with the current
  rendering context, then the values obtained for those pixels are
  undefined.
[`Gl::copy_tex_image1d`] defines a one-dimensional texture image with
  pixels from the current [`gl::READ_BUFFER`].
When `internalformat` is one of the sRGB types, the GL does not
  automatically convert the source pixels to the sRGB color space. In
  this case, the [`Gl::pixel_map`] function can be used to accomplish
  the conversion.

## Notes
1, 2, 3, and 4 are not accepted values for `internalformat`.
An image with 0 width indicates a NULL texture.
[`gl::STENCIL_INDEX8`] is accepted for `internalformat` only if the GL
  version is 4.4 or higher.

## Errors
- [`gl::INVALID_ENUM`] is generated if `target` is not one of the
  allowable values.
- [`gl::INVALID_VALUE`] is generated if `level` is less than 0.
- [`gl::INVALID_VALUE`] may be generated if `level` is greater than
  $None$, where $$ $$ _{None} *log* 2 *\u{2062}* *max* $None$ is the
  returned value of $$ None $$ *max*[`gl::MAX_TEXTURE_SIZE`].
- [`gl::INVALID_VALUE`] is generated if `internalformat` is not an
  allowable value.
- [`gl::INVALID_VALUE`] is generated if `width` is less than 0 or
  greater than [`gl::MAX_TEXTURE_SIZE`].
- [`gl::INVALID_VALUE`] is generated if `border` is not 0.
- [`gl::INVALID_OPERATION`] is generated if `internalformat` is
  [`gl::DEPTH_COMPONENT`], [`gl::DEPTH_COMPONENT16`],
  [`gl::DEPTH_COMPONENT24`], or [`gl::DEPTH_COMPONENT32`] and there is
  no depth buffer.

## See Also
- [`Gl::copy_tex_image2d`]
- [`Gl::copy_tex_sub_image1d`]
- [`Gl::copy_tex_sub_image2d`]
- [`Gl::pixel_store`]
- [`Gl::tex_image1d`]
- [`Gl::tex_image2d`]
- [`Gl::tex_sub_image1d`]
- [`Gl::tex_sub_image2d`]
- [`Gl::tex_parameter`]
