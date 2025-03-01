# glResetHistogram
reset histogram table entries to zero

## Parameters
- `target`
  Must be [`gl::HISTOGRAM`].

## Description
[`Gl::reset_histogram`] resets all the elements of the current
  histogram table to zero.

## Notes
[`Gl::reset_histogram`] is present only if ```c ARB_imaging ``` is
  returned when [`Gl::get_string`] is called with an argument of
  [`gl::EXTENSIONS`].

## Errors
- [`gl::INVALID_ENUM`] is generated if `target` is not
  [`gl::HISTOGRAM`].
- [`gl::INVALID_OPERATION`] is generated if [`Gl::reset_histogram`] is
  executed between the execution of [`Gl::begin`] and the corresponding
  execution of [`Gl::end`].

## See Also
- [`Gl::histogram`]
