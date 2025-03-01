# texelFetchOffset
perform a lookup of a single texel within a texture with an offset

## Parameters
- `sampler`
  Specifies the sampler to which the texture from which texels will be
  retrieved is bound.

## Description
[`Gl::texel_fetch_offset`] performs a lookup of a single texel from
  texture coordinate `P` in the texture bound to `sampler`. Before
  fetching the texel, the offset specified in `offset` is added to `P`.
  `offset` must be a constant expression. The array layer is specified
  in the last component of `P` for array forms. The `lod` parameter (if
  present) specifies the level-of-detail from which the texel will be
  fetched. The `sample` parameter specifies which sample within the
  texel will be returned when reading from a multi-sample texure.

## See Also
- [`Gl::texel_fetch`]
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
- [`Gl::texture_size`]
