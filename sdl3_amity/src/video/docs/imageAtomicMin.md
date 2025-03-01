# imageAtomicMin
atomically compute the minimum of a value with an existing value in
  memory, store that value and return the original value

## Parameters
- `image`
  Specify the image unit into which to store `data`.

## Description
[`Gl::image_atomic_min`] atomically computes a new value by finding
  the minimum of the value of `data` and the contents of the texel at
  coordinate `P` and `sample` in the image bound to uint `image`, stores
  that value into the image and returns the original value.

## See Also
- [`Gl::image_load`]
- [`Gl::image_store`]
- [`Gl::image_atomic_add`]
- [`Gl::image_atomic_max`]
- [`Gl::image_atomic_xor`]
- [`Gl::image_atomic_or`]
- [`Gl::image_atomic_and`]
- [`Gl::image_atomic_exchange`]
- [`Gl::image_atomic_comp_swap`]
