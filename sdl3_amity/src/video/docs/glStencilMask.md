# glStencilMask
control the front and back writing of individual bits in the stencil
  planes

## Parameters
- `mask`
  Specifies a bit mask to enable and disable writing of individual bits
  in the stencil planes. Initially, the mask is all 1's.

## Description
[`Gl::stencil_mask`] controls the writing of individual bits in the
  stencil planes. The least significant $None$ bits of $$ None $$
  *n*`mask`, where $None$ is the number of bits in the stencil buffer,
  specify a mask. Where a 1 appears in the mask, it's possible to write
  to the corresponding bit in the stencil buffer. Where a 0 appears, the
  corresponding bit is write-protected. Initially, all bits are enabled
  for writing. $$ None $$ *n*
There can be two separate `mask` writemasks; one affects back-facing
  polygons, and the other affects front-facing polygons as well as other
  non-polygon primitives. [`Gl::stencil_mask`] sets both front and back
  stencil writemasks to the same values. Use
  [`Gl::stencil_mask_separate`] to set front and back stencil writemasks
  to different values.

## Notes
[`Gl::stencil_mask`] is the same as calling
  [`Gl::stencil_mask_separate`] with `face` set to
  [`gl::FRONT_AND_BACK`].

## See Also
- [`Gl::color_mask`]
- [`Gl::depth_mask`]
- [`Gl::stencil_func`]
- [`Gl::stencil_func_separate`]
- [`Gl::stencil_mask_separate`]
- [`Gl::stencil_op`]
- [`Gl::stencil_op_separate`]
