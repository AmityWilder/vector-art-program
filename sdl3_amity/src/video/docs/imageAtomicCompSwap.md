# imageAtomicCompSwap
atomically compares supplied data with that in memory and
  conditionally stores it to memory

## Parameters
- `image`
  Specify the image unit into which to compare and conditionally store
  `data`.

## Description
[`Gl::image_atomic_comp_swap`] atomically compares the value of
  `compare` with that of the texel at coordinate `P` and `sample` (for
  multisampled forms) in the image bound to uint `image`. If the values
  are equal, `data` is stored into the texel, otherwise it is discarded.
  It returns the original value of the texel regardless of the result of
  the comparison operation.

## See Also
- [`Gl::image_load`]
- [`Gl::image_store`]
- [`Gl::image_atomic_add`]
- [`Gl::image_atomic_min`]
- [`Gl::image_atomic_max`]
- [`Gl::image_atomic_xor`]
- [`Gl::image_atomic_or`]
- [`Gl::image_atomic_and`]
- [`Gl::image_atomic_comp_swap`]
