# glGetColorTableParameter
get color lookup table parameters

## Parameters
- `target`
  The target color table. Must be [`gl::COLOR_TABLE`],
  [`gl::POST_CONVOLUTION_COLOR_TABLE`],
  [`gl::POST_COLOR_MATRIX_COLOR_TABLE`], [`gl::PROXY_COLOR_TABLE`],
  [`gl::PROXY_POST_CONVOLUTION_COLOR_TABLE`], or
  [`gl::PROXY_POST_COLOR_MATRIX_COLOR_TABLE`].

## Description
Returns parameters specific to color table `target`.
When `pname` is set to [`gl::COLOR_TABLE_SCALE`] or
  [`gl::COLOR_TABLE_BIAS`], [`Gl::get_color_table_parameter`] returns
  the color table scale or bias parameters for the table specified by
  `target`. For these queries, `target` must be set to
  [`gl::COLOR_TABLE`], [`gl::POST_CONVOLUTION_COLOR_TABLE`], or
  [`gl::POST_COLOR_MATRIX_COLOR_TABLE`] and `params` points to an array
  of four elements, which receive the scale or bias factors for red,
  green, blue, and alpha, in that order.
[`Gl::get_color_table_parameter`] can also be used to retrieve the
  format and size parameters for a color table. For these queries, set
  `target` to either the color table target or the proxy color table
  target. The format and size parameters are set by [`Gl::color_table`].
The following table lists the format and size parameters that may be
  queried. For each symbolic constant listed below for `pname`, `params`
  must point to an array of the given length and receive the values
  indicated.



## Notes
[`Gl::get_color_table_parameter`] is present only if ```c ARB_imaging
  ``` is returned when [`Gl::get_string`] is called with an argument of
  [`gl::EXTENSIONS`].

## Errors
- [`gl::INVALID_ENUM`] is generated if `target` or `pname` is not an
  acceptable value.
- [`gl::INVALID_OPERATION`] is generated if
  [`Gl::get_color_table_parameter`] is executed between the execution of
  [`Gl::begin`] and the corresponding execution of [`Gl::end`].

## See Also
- [`Gl::color_table`]
- [`Gl::tex_parameter`]
- [`Gl::color_table_parameter`]
