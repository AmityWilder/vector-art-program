# imageStore
write a single texel into an image

## Parameters
- `image`
  Specify the image unit into which to store a texel.

## Description
[`Gl::image_store`] stores `data` into the texel at the coordinate `P`
  from the image specified by `image`. For multi-sample stores, the
  sample number is given by `sample`. When `image`, `P`, and `sample`
  identify a valid texel, the bits used to represent data are converted
  to the format of the image unit in the manner described in of the
  OpenGL Specification and stored to the specified texel.

## See Also
- [`Gl::image_load`]
- [`Gl::image_atomic_add`]
- [`Gl::image_atomic_min`]
- [`Gl::image_atomic_max`]
- [`Gl::image_atomic_and`]
- [`Gl::image_atomic_or`]
- [`Gl::image_atomic_xor`]
- [`Gl::image_atomic_exchange`]
- [`Gl::image_atomic_comp_swap`]
