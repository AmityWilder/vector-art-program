# glDeleteSamplers
delete named sampler objects

## Parameters
- `n`
  Specifies the number of sampler objects to be deleted.

## Description
[`Gl::delete_samplers`] deletes `n` sampler objects named by the
  elements of the array `samplers`. After a sampler object is deleted,
  its name is again unused. If a sampler object that is currently bound
  to a sampler unit is deleted, it is as though [`Gl::bind_sampler`] is
  called with unit set to the unit the sampler is bound to and sampler
  zero. Unused names in samplers are silently ignored, as is the
  reserved name zero.

## Notes
[`Gl::delete_samplers`] is available only if the GL version is 3.3 or
  higher.

## Errors
- [`gl::INVALID_VALUE`] is generated if `n` is negative.

## See Also
- [`Gl::gen_samplers`]
- [`Gl::bind_sampler`]
- [`Gl::delete_samplers`]
- [`Gl::is_sampler`]
