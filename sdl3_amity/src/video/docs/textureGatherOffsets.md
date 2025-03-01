# textureGatherOffsets
gathers four texels from a texture with an array of offsets

## Parameters
- `sampler`
  Specifies the sampler to which the texture from which texels will be
  retrieved is bound.

## Description
[`Gl::texture_gather_offsets`] operates identically to
  [`Gl::texture_gather_offset`], except that `offsets` is used to
  determine the location of the four texels to sample. Each of the four
  texels is obtained by applying the offset in `offsets` as a (u, v)
  coordinate offset to `P`, identifying the four-texel [`gl::LINEAR`]
  footprint, and then selecting the texel $None$ of that footprint. The
  specified values in $$ None $$ _{None}*i*0_{None}*i*0`offsets` must be
  set with constant integral expressions.

## See Also
- [`Gl::texel_fetch`]
- [`Gl::texel_fetch_offset`]
- [`Gl::texture`]
- [`Gl::texture_gather`]
- [`Gl::texture_gather_offset`]
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
- [`Gl::texture_size`]
