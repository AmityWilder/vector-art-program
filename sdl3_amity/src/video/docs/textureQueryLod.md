# textureQueryLod
compute the level-of-detail that would be used to sample from a
  texture

## Parameters
- `sampler`
  Specifies the sampler to which the texture whose level-of-detail will
  be queried is bound.

## Description
*Available only in the fragment shader*, [`Gl::texture_query_lod`]
  computes the level-of-detail that would be used to sample from a
  texture. The mipmap array(s) that would be accessed is returned in the
  *x* component of the return value. The computed level-of-detail
  relative to the base level is returned in the *y* component of the
  return value.
If called on an incomplete texture, the result of the operation is
  undefined.

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
