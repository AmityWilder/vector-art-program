# glIndex
set the current color index

## Parameters
- `c`
  Specifies the new value for the current color index.

  

## Parameters
- `c`
  Specifies a pointer to a one-element array that contains the new value
  for the current color index.

## Description
[`Gl::index`] updates the current (single-valued) color index. It
  takes one argument, the new value for the current color index.
The current index is stored as a floating-point value. Integer values
  are converted directly to floating-point values, with no special
  mapping. The initial value is 1.
Index values outside the representable range of the color index buffer
  are not clamped. However, before an index is dithered (if enabled) and
  written to the frame buffer, it is converted to fixed-point format.
  Any bits in the integer portion of the resulting fixed-point value
  that do not correspond to bits in the frame buffer are masked out.

## Notes
[`Gl::indexub`] and [`Gl::indexubv`] are available only if the GL
  version is 1.1 or greater.
The current index can be updated at any time. In particular,
  [`Gl::index`] can be called between a call to [`Gl::begin`] and the
  corresponding call to [`Gl::end`].

## See Also
- [`Gl::color`]
- [`Gl::index_pointer`]
