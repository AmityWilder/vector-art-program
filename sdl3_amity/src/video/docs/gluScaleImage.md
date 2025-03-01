# gluScaleImage
scale an image to an arbitrary size

## Parameters
- `format`
  Specifies the format of the pixel data. The following symbolic values
  are valid: [`GLU_COLOR_INDEX`], [`GLU_STENCIL_INDEX`],
  [`GLU_DEPTH_COMPONENT`], [`GLU_RED`], [`GLU_GREEN`], [`GLU_BLUE`],
  [`GLU_ALPHA`], [`GLU_RGB`], [`GLU_RGBA`], [`GLU_BGR`], [`GLU_BGRA`],
  [`GLU_LUMINANCE`], and [`GLU_LUMINANCE_ALPHA`].

## Description
[`Gl::u_scale_image`] scales a pixel image using the appropriate pixel
  store modes to unpack data from the source image and pack data into
  the destination image.
When shrinking an image, [`Gl::u_scale_image`] uses a box filter to
  sample the source image and create pixels for the destination image.
  When magnifying an image, the pixels from the source image are
  linearly interpolated to create the destination image.
A return value of zero indicates success, otherwise a GLU error code
  is returned (see [`Gl::u_error_string`]).
See the [`Gl::read_pixels`] reference page for a description of the
  acceptable values for the `format`, `typeIn`, and `typeOut`
  parameters.

## Notes
Formats [`GLU_BGR`], and [`GLU_BGRA`], and types
  [`GLU_UNSIGNED_BYTE_3_3_2`], [`GLU_UNSIGNED_BYTE_2_3_3_REV`],
  [`GLU_UNSIGNED_SHORT_5_6_5`], [`GLU_UNSIGNED_SHORT_5_6_5_REV`],
  [`GLU_UNSIGNED_SHORT_4_4_4_4`], [`GLU_UNSIGNED_SHORT_4_4_4_4_REV`],
  [`GLU_UNSIGNED_SHORT_5_5_5_1`], [`GLU_UNSIGNED_SHORT_1_5_5_5_REV`],
  [`GLU_UNSIGNED_INT_8_8_8_8`], [`GLU_UNSIGNED_INT_8_8_8_8_REV`],
  [`GLU_UNSIGNED_INT_10_10_10_2`], and
  [`GLU_UNSIGNED_INT_2_10_10_10_REV`] are only available if the GL
  version is 1.2 or greater.

## Errors
- [`GLU_INVALID_VALUE`] is returned if `wIn`, `hIn`, `wOut`, or `hOut`
  is negative.
- [`GLU_INVALID_ENUM`] is returned if `format`, `typeIn`, or `typeOut`
  is not legal.
- [`GLU_INVALID_OPERATION`] is returned if `typeIn` or `typeOut` is
  [`GLU_UNSIGNED_BYTE_3_3_2`] or [`GLU_UNSIGNED_BYTE_2_3_3_REV`] and
  `format` is not [`GLU_RGB`].
- [`GLU_INVALID_OPERATION`] is returned if `typeIn` or `typeOut` is
  [`GLU_UNSIGNED_SHORT_5_6_5`] or [`GLU_UNSIGNED_SHORT_5_6_5_REV`] and
  `format` is not [`GLU_RGB`].
- [`GLU_INVALID_OPERATION`] is returned if `typeIn` or `typeOut` is
  [`GLU_UNSIGNED_SHORT_4_4_4_4`] or [`GLU_UNSIGNED_SHORT_4_4_4_4_REV`]
  and `format` is neither [`GLU_RGBA`] nor [`GLU_BGRA`].
- [`GLU_INVALID_OPERATION`] is returned if `typeIn` or `typeOut` is
  [`GLU_UNSIGNED_SHORT_5_5_5_1`] or [`GLU_UNSIGNED_SHORT_1_5_5_5_REV`]
  and `format` is neither [`GLU_RGBA`] nor [`GLU_BGRA`].
- [`GLU_INVALID_OPERATION`] is returned if `typeIn` or `typeOut` is
  [`GLU_UNSIGNED_INT_8_8_8_8`] or [`GLU_UNSIGNED_INT_8_8_8_8_REV`] and
  `format` is neither [`GLU_RGBA`] nor [`GLU_BGRA`].
- [`GLU_INVALID_OPERATION`] is returned if `typeIn` or `typeOut` is
  [`GLU_UNSIGNED_INT_10_10_10_2`] or [`GLU_UNSIGNED_INT_2_10_10_10_REV`]
  and `format` is neither [`GLU_RGBA`] nor [`GLU_BGRA`].

## See Also
- [`Gl::u_build1d_mipmaps`]
- [`Gl::u_build2d_mipmaps`]
- [`Gl::u_build3d_mipmaps`]
- [`Gl::u_error_string`]
- [`Gl::draw_pixels`]
- [`Gl::read_pixels`]
