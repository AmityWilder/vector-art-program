# textureProjGradOffset
perform a texture lookup with projection, explicit gradients and
  offset

## Parameters
- `sampler`
  Specifies the sampler to which the texture from which texels will be
  retrieved is bound.

## Description
[`Gl::texture_proj_grad_offset`] performs a texture lookup with
  projection and explicit gradients and offsets. The texture coordinates
  consumed from `P`, not including the last component of `P`, are
  divided by the last component of `P`. The resulting $None$ component
  of $$ None $$ ^{None}3*rd*`P` in the shadow forms is used as $None$.
  After these values are computed, the texture lookup proceeds as in $$
  None $$ _{None}*D**ref*[`Gl::texture_grad_offset`], passing `dPdx` and
  `dPdy` as gradients, and `offset` as the offset.

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
- [`Gl::texture_proj_lod`]
- [`Gl::texture_proj_lod_offset`]
- [`Gl::texture_proj_offset`]
- [`Gl::texture_query_lod`]
- [`Gl::texture_size`]
