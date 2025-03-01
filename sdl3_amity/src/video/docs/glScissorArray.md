# glScissorArray
define the scissor box for multiple viewports

## Parameters
- `first`
  Specifies the index of the first viewport whose scissor box to modify.

## Description
[`Gl::scissor_arrayv`] defines rectangles, called scissor boxes, in
  window coordinates for each viewport. `first` specifies the index of
  the first scissor box to modify and `count` specifies the number of
  scissor boxes to modify. `first` must be less than the value of
  [`gl::MAX_VIEWPORTS`], and `first` + `count` must be less than or
  equal to the value of [`gl::MAX_VIEWPORTS`]. `v` specifies the address
  of an array containing integers specifying the lower left corner of
  the scissor boxes, and the width and height of the scissor boxes, in
  that order.
To enable and disable the scissor test, call [`Gl::enable`] and
  [`Gl::disable`] with argument [`gl::SCISSOR_TEST`]. The test is
  initially disabled for all viewports. While the test is enabled, only
  pixels that lie within the scissor box can be modified by drawing
  commands. Window coordinates have integer values at the shared corners
  of frame buffer pixels. ```c glScissor(0,0,1,1) ``` allows
  modification of only the lower left pixel in the window, and ```c
  glScissor(0,0,0,0) ``` doesn't allow modification of any pixels in the
  window.
When the scissor test is disabled, it is as though the scissor box
  includes the entire window.

## Errors
- [`gl::INVALID_VALUE`] is generated if `first` is greater than or equal
  to the value of [`gl::MAX_VIEWPORTS`].
- [`gl::INVALID_VALUE`] is generated if `first` + `count` is greater
  than or equal to the value of [`gl::MAX_VIEWPORTS`].
- [`gl::INVALID_VALUE`] is generated if any width or height specified in
  the array `v` is negative.

## See Also
- [`Gl::enable`]
- [`Gl::viewport`]
- [`Gl::viewport_indexed`]
- [`Gl::viewport_array`]
