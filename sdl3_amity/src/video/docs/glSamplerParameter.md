# glSamplerParameter
set sampler parameters

## Parameters
- `sampler`
  Specifies the sampler object whose parameter to modify.

## Description
[`Gl::sampler_parameter`] assigns the value or values in `params` to
  the sampler parameter specified as `pname`. `sampler` specifies the
  sampler object to be modified, and must be the name of a sampler
  object previously returned from a call to [`Gl::gen_samplers`]. The
  following symbols are accepted in `pname`:





## Notes
[`Gl::sampler_parameter`] is available only if the GL version is 3.3
  or higher.
If a sampler object is bound to a texture unit and that unit is used
  to sample from a texture, the parameters in the sampler are used to
  sample from the texture, rather than the equivalent parameters in the
  texture object bound to that unit. This introduces the possibility of
  sampling from the same texture object with different sets of sampler
  state, which may lead to a condition where a texture is *incomplete*
  with respect to one sampler object and not with respect to another.
  Thus, completeness can be considered a function of a sampler object
  and a texture object bound to a single texture unit, rather than a
  property of the texture object itself.
[`gl::MIRROR_CLAMP_TO_EDGE`] is available only if the GL version is
  4.4 or greater.

## Errors
- [`gl::INVALID_VALUE`] is generated if `sampler` is not the name of a
  sampler object previously returned from a call to
  [`Gl::gen_samplers`].
- [`gl::INVALID_ENUM`] is generated if `params` should have a defined
  constant value (based on the value of `pname`) and does not.

## See Also
- [`Gl::gen_samplers`]
- [`Gl::bind_sampler`]
- [`Gl::delete_samplers`]
- [`Gl::is_sampler`]
- [`Gl::bind_texture`]
- [`Gl::tex_parameter`]
