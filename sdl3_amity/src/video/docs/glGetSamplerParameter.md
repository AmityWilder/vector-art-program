# glGetSamplerParameter
return sampler parameter values

## Parameters
- `sampler`
  Specifies name of the sampler object from which to retrieve
  parameters.

## Description
[`Gl::get_sampler_parameter`] returns in `params` the value or values
  of the sampler parameter specified as `pname`. `sampler` defines the
  target sampler, and must be the name of an existing sampler object,
  returned from a previous call to [`Gl::gen_samplers`]. `pname` accepts
  the same symbols as [`Gl::sampler_parameter`], with the same
  interpretations:

## Notes
If an error is generated, no change is made to the contents of
  `params`.
[`Gl::get_sampler_parameter`] is available only if the GL version is
  3.3 or higher.

## Errors
- [`gl::INVALID_VALUE`] is generated if `sampler` is not the name of a
  sampler object returned from a previous call to [`Gl::gen_samplers`].
- [`gl::INVALID_ENUM`] is generated if `pname` is not an accepted value.

## See Also
- [`Gl::sampler_parameter`]
- [`Gl::gen_samplers`]
- [`Gl::delete_samplers`]
- [`Gl::sampler_parameter`]
