# textureGradOffset
perform a texture lookup with explicit gradients and offset

## Parameters
- `sampler`
  Specifies the sampler to which the texture from which texels will be
  retrieved is bound.

## Description
[`Gl::texture_grad_offset`] performs a texture lookup at coordinate
  `P` from the texture bound to `sampler` with explicit texture
  coordinate gradiends as specified in `dPdx` and `dPdy`. An explicit
  offset is also supplied in `offset`. [`Gl::texture_grad_offset`]
  consumes `dPdx` and `dPdy` as [`Gl::texture_grad`] and `offset` as
  [`Gl::texture_offset`].

## See Also
- [`Gl::texel_fetch`]
- [`Gl::texel_fetch_offset`]
- [`Gl::texture`]
- [`Gl::texture_gather`]
- [`Gl::texture_gather_offset`]
- [`Gl::texture_gather_offsets`]
- [`Gl::texture_grad`]
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
- [`Gl::texture_size`]
