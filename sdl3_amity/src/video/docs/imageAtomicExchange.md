# imageAtomicExchange
atomically store supplied data into memory and return the original
  value from memory

## Parameters
- `image`
  Specify the image unit into which to store `data`.

## Description
[`Gl::image_atomic_exchange`] atomically stores the value of `data`
  into the texel at coordinate `P` and `sample` in the image bound to
  unit `image`, and returns the original value of the texel.

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
