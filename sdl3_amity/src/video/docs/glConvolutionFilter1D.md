# glConvolutionFilter1D
define a one-dimensional convolution filter

## Parameters
- `target`
  Must be [`gl::CONVOLUTION_1D`].

## Description
[`Gl::convolution_filter1d`] builds a one-dimensional convolution
  filter kernel from an array of pixels.
The pixel array specified by `width`, `format`, `type`, and `data` is
  extracted from memory and processed just as if [`Gl::draw_pixels`]
  were called, but processing stops after the final expansion to RGBA is
  completed.
If a non-zero named buffer object is bound to the
  [`gl::PIXEL_UNPACK_BUFFER`] target (see [`Gl::bind_buffer`]) while a
  convolution filter is specified, `data` is treated as a byte offset
  into the buffer object's data store.
The R, G, B, and A components of each pixel are next scaled by the
  four 1D [`gl::CONVOLUTION_FILTER_SCALE`] parameters and biased by the
  four 1D [`gl::CONVOLUTION_FILTER_BIAS`] parameters. (The scale and
  bias parameters are set by [`Gl::convolution_parameter`] using the
  [`gl::CONVOLUTION_1D`] target and the names
  [`gl::CONVOLUTION_FILTER_SCALE`] and [`gl::CONVOLUTION_FILTER_BIAS`].
  The parameters themselves are vectors of four values that are applied
  to red, green, blue, and alpha, in that order.) The R, G, B, and A
  values are not clamped to [0,1] at any time during this process.
Each pixel is then converted to the internal format specified by
  `internalformat`. This conversion simply maps the component values of
  the pixel (R, G, B, and A) to the values included in the internal
  format (red, green, blue, alpha, luminance, and intensity). The
  mapping is as follows:

The red, green, blue, alpha, luminance, and/or intensity components of
  the resulting pixels are stored in floating-point rather than integer
  format. They form a one-dimensional filter kernel image indexed with
  coordinate *i* such that *i* starts at 0 and increases from left to
  right. Kernel location *i* is derived from the *i*th pixel, counting
  from 0.
Note that after a convolution is performed, the resulting color
  components are also scaled by their corresponding
  [`gl::POST_CONVOLUTION_c_SCALE`] parameters and biased by their
  corresponding [`gl::POST_CONVOLUTION_c_BIAS`] parameters (where *c*
  takes on the values *RED*, *GREEN*, *BLUE*, and *ALPHA*). These
  parameters are set by [`Gl::pixel_transfer`].

## Notes
[`Gl::convolution_filter1d`] is present only if ```c ARB_imaging ```
  is returned when [`Gl::get_string`] is called with an argument of
  [`gl::EXTENSIONS`].

## Errors
- [`gl::INVALID_ENUM`] is generated if `target` is not
  [`gl::CONVOLUTION_1D`].
- [`gl::INVALID_ENUM`] is generated if `internalformat` is not one of
  the allowable values.
- [`gl::INVALID_ENUM`] is generated if `format` is not one of the
  allowable values.
- [`gl::INVALID_ENUM`] is generated if `type` is not one of the
  allowable values.
- [`gl::INVALID_VALUE`] is generated if `width` is less than zero or
  greater than the maximum supported value. This value may be queried
  with [`Gl::get_convolution_parameter`] using target
  [`gl::CONVOLUTION_1D`] and name [`gl::MAX_CONVOLUTION_WIDTH`].
- [`gl::INVALID_OPERATION`] is generated if `type` is one of
  [`gl::UNSIGNED_BYTE_3_3_2`], [`gl::UNSIGNED_BYTE_2_3_3_REV`],
  [`gl::UNSIGNED_SHORT_5_6_5`], or [`gl::UNSIGNED_SHORT_5_6_5_REV`] and
  `format` is not [`gl::RGB`].
- [`gl::INVALID_OPERATION`] is generated if `type` is one of
  [`gl::UNSIGNED_SHORT_4_4_4_4`], [`gl::UNSIGNED_SHORT_4_4_4_4_REV`],
  [`gl::UNSIGNED_SHORT_5_5_5_1`], [`gl::UNSIGNED_SHORT_1_5_5_5_REV`],
  [`gl::UNSIGNED_INT_8_8_8_8`], [`gl::UNSIGNED_INT_8_8_8_8_REV`],
  [`gl::UNSIGNED_INT_10_10_10_2`], or
  [`gl::UNSIGNED_INT_2_10_10_10_REV`] and `format` is neither
  [`gl::RGBA`] nor [`gl::BGRA`].
- [`gl::INVALID_OPERATION`] is generated if a non-zero buffer object
  name is bound to the [`gl::PIXEL_UNPACK_BUFFER`] target and the buffer
  object's data store is currently mapped.
- [`gl::INVALID_OPERATION`] is generated if a non-zero buffer object
  name is bound to the [`gl::PIXEL_UNPACK_BUFFER`] target and the data
  would be unpacked from the buffer object such that the memory reads
  required would exceed the data store size.
- [`gl::INVALID_OPERATION`] is generated if a non-zero buffer object
  name is bound to the [`gl::PIXEL_UNPACK_BUFFER`] target and `data` is
  not evenly divisible into the number of bytes needed to store in
  memory a datum indicated by `type`.
- [`gl::INVALID_OPERATION`] is generated if [`Gl::convolution_filter1d`]
  is executed between the execution of [`Gl::begin`] and the
  corresponding execution of [`Gl::end`].

## See Also
- [`Gl::convolution_filter2d`]
- [`Gl::separable_filter2d`]
- [`Gl::convolution_parameter`]
- [`Gl::pixel_transfer`]
