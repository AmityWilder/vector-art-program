# glResetMinmax
reset minmax table entries to initial values

## Parameters
- `target`
  Must be [`gl::MINMAX`].

## Description
[`Gl::reset_minmax`] resets the elements of the current minmax table
  to their initial values: the ``maximum'' element receives the minimum
  possible component values, and the ``minimum'' element receives the
  maximum possible component values.

## Notes
[`Gl::reset_minmax`] is present only if ```c ARB_imaging ``` is
  returned when [`Gl::get_string`] is called with an argument of
  [`gl::EXTENSIONS`].

## Errors
- [`gl::INVALID_ENUM`] is generated if `target` is not [`gl::MINMAX`].
- [`gl::INVALID_OPERATION`] is generated if [`Gl::reset_minmax`] is
  executed between the execution of [`Gl::begin`] and the corresponding
  execution of [`Gl::end`].

## See Also
- [`Gl::minmax`]
