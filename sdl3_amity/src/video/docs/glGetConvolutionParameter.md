# glGetConvolutionParameter
get convolution parameters

## Parameters
- `target`
  The filter whose parameters are to be retrieved. Must be one of
  [`gl::CONVOLUTION_1D`], [`gl::CONVOLUTION_2D`], or
  [`gl::SEPARABLE_2D`].

## Description
[`Gl::get_convolution_parameter`] retrieves convolution parameters.
  `target` determines which convolution filter is queried. `pname`
  determines which parameter is returned:

## Errors
- [`gl::INVALID_ENUM`] is generated if `target` is not one of the
  allowable values.
- [`gl::INVALID_ENUM`] is generated if `pname` is not one of the
  allowable values.
- [`gl::INVALID_ENUM`] is generated if `target` is
  [`gl::CONVOLUTION_1D`] and `pname` is [`gl::CONVOLUTION_HEIGHT`] or
  [`gl::MAX_CONVOLUTION_HEIGHT`].
- [`gl::INVALID_OPERATION`] is generated if
  [`Gl::get_convolution_parameter`] is executed between the execution of
  [`Gl::begin`] and the corresponding execution of [`Gl::end`].

## See Also
- [`Gl::get_convolution_filter`]
- [`Gl::get_separable_filter`]
- [`Gl::convolution_parameter`]
