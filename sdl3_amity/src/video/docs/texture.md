# texture
retrieves texels from a texture

## Parameters
- `sampler`
  Specifies the sampler to which the texture from which texels will be
  retrieved is bound.

## Description
[`Gl::texture`] samples texels from the texture bound to `sampler` at
  texture coordinate `P`. An optional bias, specified in `bias` is
  included in the level-of-detail computation that is used to choose
  mipmap(s) from which to sample.
For *shadow* forms, when `compare` is present, it is used as $None$
  and the array layer is specified in $$ None $$ _{None}*D**sub* ```c
  None ``` . When `P`.w`compare` is not present, the last component of
  `P` is used as $None$ and the array layer is specified in the second
  to last component of $$ None $$ _{None}*D**sub*`P`. (The second
  component of `P` is unused for *1D* shadow lookups.)
For non-shadow variants, the array layer comes from the last component
  of `P`.

## See Also
- [`Gl::texel_fetch`]
- [`Gl::texel_fetch_offset`]
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
- [`Gl::texture_size`]
