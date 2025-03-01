# textureGatherOffset
gathers four texels from a texture with offset

## Parameters
- `sampler`
  Specifies the sampler to which the texture from which texels will be
  retrieved is bound.

## Description
[`Gl::texture_gather_offset`] returns the value:
```c vec4(Sample_i0_j1(P + offset, base).comp, Sample_i1_j1(P +
  offset, base).comp, Sample_i1_j0(P + offset, base).comp,
  Sample_i0_j0(P + offset, base).comp); ```
It perfoms as [`Gl::texture_gather`] but with `offset` applied as
  described in [`Gl::texture_offset`], except that the implementation-
  dependent minimum and maximum offset values are given by
  [`gl::MIN_PROGRRAM_TEXTURE_GATHER_OFFSET`] and
  [`gl::MAX_PROGRAM_TEXTURE_GATHER_OFFSET`], respectively.
If specified, the value of `comp` must be a constant integer
  expression with a value of 0, 1, 2, or 3, identifying the x, y, z or w
  component of the four-component vector lookup result for each texel,
  respectively. If `comp` is not specified, it is treated as 0,
  selecting the x component of each texel to generate the result.

## See Also
- [`Gl::texel_fetch`]
- [`Gl::texel_fetch_offset`]
- [`Gl::texture`]
- [`Gl::texture_gather`]
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
