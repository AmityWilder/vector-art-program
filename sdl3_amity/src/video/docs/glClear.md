# glClear
clear buffers to preset values

## Parameters
- `mask`
  Bitwise OR of masks that indicate the buffers to be cleared. The three
  masks are [`gl::COLOR_BUFFER_BIT`], [`gl::DEPTH_BUFFER_BIT`], and
  [`gl::STENCIL_BUFFER_BIT`].

## Description
[`Gl::clear`] sets the bitplane area of the window to values
  previously selected by [`Gl::clear_color`], [`Gl::clear_depth`], and
  [`Gl::clear_stencil`]. Multiple color buffers can be cleared
  simultaneously by selecting more than one buffer at a time using
  [`Gl::draw_buffer`].
The pixel ownership test, the scissor test, dithering, and the buffer
  writemasks affect the operation of [`Gl::clear`]. The scissor box
  bounds the cleared region. Alpha function, blend function, logical
  operation, stenciling, texture mapping, and depth-buffering are
  ignored by [`Gl::clear`].
[`Gl::clear`] takes a single argument that is the bitwise OR of
  several values indicating which buffer is to be cleared.
The values are as follows:
The value to which each buffer is cleared depends on the setting of
  the clear value for that buffer.

## Notes
If a buffer is not present, then a [`Gl::clear`] directed at that
  buffer has no effect.

## Errors
- [`gl::INVALID_VALUE`] is generated if any bit other than the three
  defined bits is set in `mask`.

## See Also
- [`Gl::color_mask`]
- [`Gl::depth_mask`]
- [`Gl::draw_buffer`]
- [`Gl::scissor`]
- [`Gl::stencil_mask`]
