# textureOffset
perform a texture lookup with offset

## Parameters
- `sampler`
  Specifies the sampler to which the texture from which texels will be
  retrieved is bound.

## Description
[`Gl::texture_offset`] performs a texture lookup at coordinate `P`
  from the texture bound to `sampler` with an additional offset,
  specified in texels in `offset` that will be applied to the (u, v, w)
  texture coordinates before looking up each texel. The offset value
  must be a constant expression. A limited range of offset values are
  supported; the minimum and maximum offset values are implementation-
  dependent and may be determined by querying
  [`gl::MIN_PROGRAM_TEXEL_OFFSET`] and [`gl::MAX_PROGRAM_TEXEL_OFFSET`],
  respectively.
Note that `offset` does not apply to the layer coordinate for texture
  arrays. Also note that offsets are not supported for cube maps.

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
- [`Gl::texture_proj`]
- [`Gl::texture_proj_grad`]
- [`Gl::texture_proj_grad_offset`]
- [`Gl::texture_proj_lod`]
- [`Gl::texture_proj_lod_offset`]
- [`Gl::texture_proj_offset`]
- [`Gl::texture_query_lod`]
- [`Gl::texture_size`]
