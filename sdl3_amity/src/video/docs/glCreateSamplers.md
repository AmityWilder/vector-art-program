# glCreateSamplers
create sampler objects

## Parameters
- `n`
  Number of sampler objects to create.

## Description
[`Gl::create_samplers`] returns `n` previously unused sampler names in
  `samplers`, each representing a new sampler object initialized to the
  default state.

## Errors
- [`gl::INVALID_VALUE`] is generated if `n` is negative.

## See Also
- [`Gl::bind_sampler`]
- [`Gl::bind_texture`]
- [`Gl::delete_samplers`]
- [`Gl::delete_textures`]
- [`Gl::gen_samplers`]
- [`Gl::gen_textures`]
- [`Gl::get`]
- [`Gl::get_sampler_parameter`]
- [`Gl::sampler_parameter`]
