# glColorMask, glColorMaski
enable and disable writing of frame buffer color components

## Parameters
- `buf`
  For [`Gl::color_maski`], specifies the index of the draw buffer whose
  color mask to set.

## Description
[`Gl::color_mask`] and [`Gl::color_maski`] specify whether the
  individual color components in the frame buffer can or cannot be
  written. [`Gl::color_maski`] sets the mask for a specific draw buffer,
  whereas [`Gl::color_mask`] sets the mask for all draw buffers. If
  `red` is [`gl::FALSE`], for example, no change is made to the red
  component of any pixel in any of the color buffers, regardless of the
  drawing operation attempted.
Changes to individual bits of components cannot be controlled. Rather,
  changes are either enabled or disabled for entire color components.

## See Also
- [`Gl::clear`]
- [`Gl::depth_mask`]
- [`Gl::stencil_mask`]
