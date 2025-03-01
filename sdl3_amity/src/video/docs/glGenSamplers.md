# glGenSamplers
generate sampler object names

## Parameters
- `n`
  Specifies the number of sampler object names to generate.

## Description
[`Gl::gen_samplers`] returns `n` sampler object names in `samplers`.
  There is no guarantee that the names form a contiguous set of
  integers; however, it is guaranteed that none of the returned names
  was in use immediately before the call to [`Gl::gen_samplers`].
Sampler object names returned by a call to [`Gl::gen_samplers`] are
  not returned by subsequent calls, unless they are first deleted with
  [`Gl::delete_samplers`].
The names returned in `samplers` are marked as used, for the purposes
  of [`Gl::gen_samplers`] only, but they acquire state and type only
  when they are first bound.

## Notes
[`Gl::gen_samplers`] is available only if the GL version is 3.3 or
  higher.

## Errors
- [`gl::INVALID_VALUE`] is generated if `n` is negative.

## See Also
- [`Gl::bind_sampler`]
- [`Gl::is_sampler`]
- [`Gl::delete_samplers`]
