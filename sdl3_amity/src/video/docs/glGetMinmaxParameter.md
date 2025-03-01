# glGetMinmaxParameter
get minmax parameters

## Parameters
- `target`
  Must be [`gl::MINMAX`].

## Description
[`Gl::get_minmax_parameter`] retrieves parameters for the current
  minmax table by setting `pname` to one of the following values:



## Notes
[`Gl::get_minmax_parameter`] is present only if ```c ARB_imaging ```
  is returned when [`Gl::get_string`] is called with an argument of
  [`gl::EXTENSIONS`].

## Errors
- [`gl::INVALID_ENUM`] is generated if `target` is not [`gl::MINMAX`].
- [`gl::INVALID_ENUM`] is generated if `pname` is not one of the
  allowable values.
- [`gl::INVALID_OPERATION`] is generated if [`Gl::get_minmax_parameter`]
  is executed between the execution of [`Gl::begin`] and the
  corresponding execution of [`Gl::end`].

## See Also
- [`Gl::minmax`]
- [`Gl::get_minmax`]
