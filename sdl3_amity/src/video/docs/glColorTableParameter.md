# glColorTableParameter
set color lookup table parameters

## Parameters
- `target`
  The target color table. Must be [`gl::COLOR_TABLE`],
  [`gl::POST_CONVOLUTION_COLOR_TABLE`], or
  [`gl::POST_COLOR_MATRIX_COLOR_TABLE`].

## Description
[`Gl::color_table_parameter`] is used to specify the scale factors and
  bias terms applied to color components when they are loaded into a
  color table. `target` indicates which color table the scale and bias
  terms apply to; it must be set to [`gl::COLOR_TABLE`],
  [`gl::POST_CONVOLUTION_COLOR_TABLE`], or
  [`gl::POST_COLOR_MATRIX_COLOR_TABLE`].
`pname` must be [`gl::COLOR_TABLE_SCALE`] to set the scale factors. In
  this case, `params` points to an array of four values, which are the
  scale factors for red, green, blue, and alpha, in that order.
`pname` must be [`gl::COLOR_TABLE_BIAS`] to set the bias terms. In
  this case, `params` points to an array of four values, which are the
  bias terms for red, green, blue, and alpha, in that order.
The color tables themselves are specified by calling
  [`Gl::color_table`].

## Notes
[`Gl::color_table_parameter`] is available only if ```c ARB_imaging
  ``` is returned from calling [`Gl::get_string`] with an argument of
  [`gl::EXTENSIONS`].

## Errors
- [`gl::INVALID_ENUM`] is generated if `target` or `pname` is not an
  acceptable value.
- [`gl::INVALID_OPERATION`] is generated if
  [`Gl::color_table_parameter`] is executed between the execution of
  [`Gl::begin`] and the corresponding execution of [`Gl::end`].

## See Also
- [`Gl::color_table`]
- [`Gl::pixel_transfer`]
