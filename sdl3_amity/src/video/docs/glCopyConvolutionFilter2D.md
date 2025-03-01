# glCopyConvolutionFilter2D
copy pixels into a two-dimensional convolution filter

## Parameters
- `target`
  Must be [`gl::CONVOLUTION_2D`].

## Description
[`Gl::copy_convolution_filter2d`] defines a two-dimensional
  convolution filter kernel with pixels from the current
  [`gl::READ_BUFFER`] (rather than from main memory, as is the case for
  [`Gl::convolution_filter2d`]).
The screen-aligned pixel rectangle with lower-left corner at ( `x`,\
  `y`), width `width` and height `height` is used to define the
  convolution filter. If any pixels within this region are outside the
  window that is associated with the GL context, the values obtained for
  those pixels are undefined.
The pixels in the rectangle are processed exactly as if
  [`Gl::read_pixels`] had been called with *format* set to RGBA, but the
  process stops just before final conversion. The R, G, B, and A
  components of each pixel are next scaled by the four 2D
  [`gl::CONVOLUTION_FILTER_SCALE`] parameters and biased by the four 2D
  [`gl::CONVOLUTION_FILTER_BIAS`] parameters. (The scale and bias
  parameters are set by [`Gl::convolution_parameter`] using the
  [`gl::CONVOLUTION_2D`] target and the names
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
  format.
Pixel ordering is such that lower x screen coordinates correspond to
  lower *i* filter image coordinates, and lower y screen coordinates
  correspond to lower *j* filter image coordinates.
Note that after a convolution is performed, the resulting color
  components are also scaled by their corresponding
  [`gl::POST_CONVOLUTION_c_SCALE`] parameters and biased by their
  corresponding [`gl::POST_CONVOLUTION_c_BIAS`] parameters (where *c*
  takes on the values *RED*, *GREEN*, *BLUE*, and *ALPHA*). These
  parameters are set by [`Gl::pixel_transfer`].

## Notes
[`Gl::copy_convolution_filter2d`] is present only if ```c ARB_imaging
  ``` is returned when [`Gl::get_string`] is called with an argument of
  [`gl::EXTENSIONS`].

## Errors
- [`gl::INVALID_ENUM`] is generated if `target` is not
  [`gl::CONVOLUTION_2D`].
- [`gl::INVALID_ENUM`] is generated if `internalformat` is not one of
  the allowable values.
- [`gl::INVALID_VALUE`] is generated if `width` is less than zero or
  greater than the maximum supported value. This value may be queried
  with [`Gl::get_convolution_parameter`] using target
  [`gl::CONVOLUTION_2D`] and name [`gl::MAX_CONVOLUTION_WIDTH`].
- [`gl::INVALID_VALUE`] is generated if `height` is less than zero or
  greater than the maximum supported value. This value may be queried
  with [`Gl::get_convolution_parameter`] using target
  [`gl::CONVOLUTION_2D`] and name [`gl::MAX_CONVOLUTION_HEIGHT`].
- [`gl::INVALID_OPERATION`] is generated if
  [`Gl::copy_convolution_filter2d`] is executed between the execution of
  [`Gl::begin`] and the corresponding execution of [`Gl::end`].

## See Also
- [`Gl::convolution_filter2d`]
- [`Gl::convolution_parameter`]
- [`Gl::pixel_transfer`]
