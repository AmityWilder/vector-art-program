# glMinmax
define minmax table

## Parameters
- `target`
  The minmax table whose parameters are to be set. Must be
  [`gl::MINMAX`].

## Description
When [`gl::MINMAX`] is enabled, the RGBA components of incoming pixels
  are compared to the minimum and maximum values for each component,
  which are stored in the two-element minmax table. (The first element
  stores the minima, and the second element stores the maxima.) If a
  pixel component is greater than the corresponding component in the
  maximum element, then the maximum element is updated with the pixel
  component value. If a pixel component is less than the corresponding
  component in the minimum element, then the minimum element is updated
  with the pixel component value. (In both cases, if the internal format
  of the minmax table includes luminance, then the R color component of
  incoming pixels is used for comparison.) The contents of the minmax
  table may be retrieved at a later time by calling [`Gl::get_minmax`].
  The minmax operation is enabled or disabled by calling [`Gl::enable`]
  or [`Gl::disable`], respectively, with an argument of [`gl::MINMAX`].
[`Gl::minmax`] redefines the current minmax table to have entries of
  the format specified by `internalformat`. The maximum element is
  initialized with the smallest possible component values, and the
  minimum element is initialized with the largest possible component
  values. The values in the previous minmax table, if any, are lost. If
  `sink` is [`gl::TRUE`], then pixels are discarded after minmax; no
  further processing of the pixels takes place, and no drawing, texture
  loading, or pixel readback will result.


## Notes
[`Gl::minmax`] is present only if ```c ARB_imaging ``` is returned
  when [`Gl::get_string`] is called with an argument of
  [`gl::EXTENSIONS`].

## Errors
- [`gl::INVALID_ENUM`] is generated if `target` is not one of the
  allowable values.
- [`gl::INVALID_ENUM`] is generated if `internalformat` is not one of
  the allowable values.
- [`gl::INVALID_OPERATION`] is generated if [`Gl::minmax`] is executed
  between the execution of [`Gl::begin`] and the corresponding execution
  of [`Gl::end`].

## See Also
- [`Gl::get_minmax`]
- [`Gl::reset_minmax`]
