# textureQueryLevels
compute the number of accessible mipmap levels of a texture

## Parameters
- `sampler`
  Specifies the sampler to which the texture whose mipmap level count
  will be queried is bound.

## Description
[`Gl::texture_query_levels`] returns the number of accessible mipmap
  levels in the texture associated with `sampler`.
If called on an incomplete texture, or if no texture is associated
  with `sampler`, zero is returned.

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
- [`Gl::texture_size`]
