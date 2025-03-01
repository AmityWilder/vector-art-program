# glClearStencil
specify the clear value for the stencil buffer

## Parameters
- `s`
  Specifies the index used when the stencil buffer is cleared. The
  initial value is 0.

## Description
[`Gl::clear_stencil`] specifies the index used by [`Gl::clear`] to
  clear the stencil buffer. `s` is masked with $None$, where $$ $$
  ^{None} 2 *m* *-* 1 $None$ is the number of bits in the stencil
  buffer. $$ None $$ *m*

## See Also
- [`Gl::clear`]
- [`Gl::stencil_func`]
- [`Gl::stencil_func_separate`]
- [`Gl::stencil_mask`]
- [`Gl::stencil_mask_separate`]
- [`Gl::stencil_op`]
- [`Gl::stencil_op_separate`]
