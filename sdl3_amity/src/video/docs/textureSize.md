# textureSize
retrieve the dimensions of a level of a texture

## Parameters
- `sampler`
  Specifies the sampler to which the texture whose dimensions to
  retrieve is bound.

## Description
[`Gl::texture_size`] returns the dimensions of level `lod` (if
  present) of the texture bound to `sampler`. The components in the
  return value are filled in, in order, with the width, height and depth
  of the texture. For the array forms, the last component of the return
  value is the number of layers in the texture array.

## See Also
- [`Gl::texel_fetch`]
- [`Gl::texel_fetch_offset`]
- [`Gl::texture`]
- [`Gl::texture_gather`]
- [`Gl::texture_gather_offset`]
- [`Gl::texture_gather_offsets`]
- [`Gl::texture_grad`]
- [`Gl::texture_grad_offset`]
- [`Gl::texture_lod`]
- [`Gl::texture_lod_offset`]
- [`Gl::texture_offset`]
- [`Gl::texture_proj`]
- [`Gl::texture_proj_grad`]
- [`Gl::texture_proj_grad_offset`]
- [`Gl::texture_proj_lod`]
- [`Gl::texture_proj_lod_offset`]
- [`Gl::texture_proj_offset`]
- [`Gl::texture_query_lod`]
