# glLineWidth
specify the width of rasterized lines

## Parameters
- `width`
  Specifies the width of rasterized lines. The initial value is 1.

## Description
[`Gl::line_width`] specifies the rasterized width of both aliased and
  antialiased lines. Using a line width other than 1 has different
  effects, depending on whether line antialiasing is enabled. To enable
  and disable line antialiasing, call [`Gl::enable`] and [`Gl::disable`]
  with argument [`gl::LINE_SMOOTH`]. Line antialiasing is initially
  disabled.
If line antialiasing is disabled, the actual width is determined by
  rounding the supplied width to the nearest integer. (If the rounding
  results in the value 0, it is as if the line width were 1.) If $None$,
  $$ $$ ```c ``` *Δ* *x* *>=* ```c ``` *Δ* *y* *i* pixels are filled in
  each column that is rasterized, where *i* is the rounded value of
  `width`. Otherwise, *i* pixels are filled in each row that is
  rasterized.
If antialiasing is enabled, line rasterization produces a fragment for
  each pixel square that intersects the region lying within the
  rectangle having width equal to the current line width, length equal
  to the actual length of the line, and centered on the mathematical
  line segment. The coverage value for each fragment is the window
  coordinate area of the intersection of the rectangular region with the
  corresponding pixel square. This value is saved and used in the final
  rasterization step.
Not all widths can be supported when line antialiasing is enabled. If
  an unsupported width is requested, the nearest supported width is
  used. Only width 1 is guaranteed to be supported; others depend on the
  implementation. Likewise, there is a range for aliased line widths as
  well. To query the range of supported widths and the size difference
  between supported widths within the range, call [`Gl::get`] with
  arguments [`gl::ALIASED_LINE_WIDTH_RANGE`],
  [`gl::SMOOTH_LINE_WIDTH_RANGE`], and
  [`gl::SMOOTH_LINE_WIDTH_GRANULARITY`].

## Notes
The line width specified by [`Gl::line_width`] is always returned when
  [`gl::LINE_WIDTH`] is queried. Clamping and rounding for aliased and
  antialiased lines have no effect on the specified value.
Nonantialiased line width may be clamped to an implementation-
  dependent maximum. Call [`Gl::get`] with
  [`gl::ALIASED_LINE_WIDTH_RANGE`] to determine the maximum width.
In OpenGL 1.2, the tokens [`gl::LINE_WIDTH_RANGE`] and
  [`gl::LINE_WIDTH_GRANULARITY`] were replaced by
  [`gl::ALIASED_LINE_WIDTH_RANGE`], [`gl::SMOOTH_LINE_WIDTH_RANGE`], and
  [`gl::SMOOTH_LINE_WIDTH_GRANULARITY`]. The old names are retained for
  backward compatibility, but should not be used in new code.

## Errors
- [`gl::INVALID_VALUE`] is generated if `width` is less than or equal to
  0.

## See Also
- [`Gl::enable`]
