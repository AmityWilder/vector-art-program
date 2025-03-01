# glScissor
define the scissor box

## Parameters
- `x`
  Specify the lower left corner of the scissor box. Initially (0, 0).

## Description
[`Gl::scissor`] defines a rectangle, called the scissor box, in window
  coordinates. The first two arguments, `x` and `y`, specify the lower
  left corner of the box. `width` and `height` specify the width and
  height of the box.
To enable and disable the scissor test, call [`Gl::enable`] and
  [`Gl::disable`] with argument [`gl::SCISSOR_TEST`]. The test is
  initially disabled. While the test is enabled, only pixels that lie
  within the scissor box can be modified by drawing commands. Window
  coordinates have integer values at the shared corners of frame buffer
  pixels. ```c glScissor(0,0,1,1) ``` allows modification of only the
  lower left pixel in the window, and ```c glScissor(0,0,0,0) ```
  doesn't allow modification of any pixels in the window.
When the scissor test is disabled, it is as though the scissor box
  includes the entire window.

## Errors
- [`gl::INVALID_VALUE`] is generated if either `width` or `height` is
  negative.

## See Also
- [`Gl::enable`]
- [`Gl::viewport`]
