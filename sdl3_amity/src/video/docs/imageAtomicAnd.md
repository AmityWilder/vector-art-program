# imageAtomicAnd
atomically compute the logical AND of a value with an existing value
  in memory, store that value and return the original value

## Parameters
- `image`
  Specify the image unit into which to store `data`.

## Description
[`Gl::image_atomic_and`] atomically computes a new value by logically
  ANDing the value of `data` to the contents of the texel at coordinate
  `P` and `sample` in the image bound to uint `image`, stores that value
  into the image and returns the original value.

## See Also
- [`Gl::image_load`]
- [`Gl::image_store`]
- [`Gl::image_atomic_add`]
- [`Gl::image_atomic_min`]
- [`Gl::image_atomic_max`]
- [`Gl::image_atomic_or`]
- [`Gl::image_atomic_xor`]
- [`Gl::image_atomic_exchange`]
- [`Gl::image_atomic_comp_swap`]
