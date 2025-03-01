# imageLoad
load a single texel from an image

## Parameters
- `image`
  Specify the image unit from which to load a texel.

## Description
[`Gl::image_load`] loads the texel at the coordinate `P` from the
  image unit `image`. For multi-sample loads, the sample number is given
  by `sample`. When `image`, `P`, `sample` identify a valid texel, the
  bits used to represent the selected texel in memory are converted to a
  vec4, ivec4, or uvec4 in the manner described in the OpenGL
  Specification and returned.

## See Also
- [`Gl::image_store`]
- [`Gl::image_atomic_add`]
- [`Gl::image_atomic_min`]
- [`Gl::image_atomic_max`]
- [`Gl::image_atomic_and`]
- [`Gl::image_atomic_or`]
- [`Gl::image_atomic_xor`]
- [`Gl::image_atomic_exchange`]
- [`Gl::image_atomic_comp_swap`]
