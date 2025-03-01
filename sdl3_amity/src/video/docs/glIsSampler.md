# glIsSampler
determine if a name corresponds to a sampler object

## Parameters
- `id`
  Specifies a value that may be the name of a sampler object.

## Description
[`Gl::is_sampler`] returns [`gl::TRUE`] if `id` is currently the name
  of a sampler object. If `id` is zero, or is a non-zero value that is
  not currently the name of a sampler object, or if an error occurs,
  [`Gl::is_sampler`] returns [`gl::FALSE`].
A name returned by [`Gl::gen_samplers`], is the name of a sampler
  object.

## Notes
[`Gl::is_sampler`] is available only if the GL version is 3.3 or
  higher.

## See Also
- [`Gl::gen_samplers`]
- [`Gl::bind_sampler`]
- [`Gl::delete_samplers`]
