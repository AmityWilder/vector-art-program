# glStencilMaskSeparate
control the front and/or back writing of individual bits in the
  stencil planes

## Parameters
- `face`
  Specifies whether the front and/or back stencil writemask is updated.
  Three symbolic constants are valid: [`gl::FRONT`], [`gl::BACK`], and
  [`gl::FRONT_AND_BACK`].

## Description
[`Gl::stencil_mask_separate`] controls the writing of individual bits
  in the stencil planes. The least significant $None$ bits of $$ None $$
  *n*`mask`, where $None$ is the number of bits in the stencil buffer,
  specify a mask. Where a 1 appears in the mask, it's possible to write
  to the corresponding bit in the stencil buffer. Where a 0 appears, the
  corresponding bit is write-protected. Initially, all bits are enabled
  for writing. $$ None $$ *n*
There can be two separate `mask` writemasks; one affects back-facing
  polygons, and the other affects front-facing polygons as well as other
  non-polygon primitives. [`Gl::stencil_mask`] sets both front and back
  stencil writemasks to the same values, as if
  [`Gl::stencil_mask_separate`] were called with `face` set to
  [`gl::FRONT_AND_BACK`].

## Errors
- [`gl::INVALID_ENUM`] is generated if `face` is not one of the accepted
  tokens.

## See Also
- [`Gl::color_mask`]
- [`Gl::depth_mask`]
- [`Gl::stencil_func`]
- [`Gl::stencil_func_separate`]
- [`Gl::stencil_mask`]
- [`Gl::stencil_op`]
- [`Gl::stencil_op_separate`]
