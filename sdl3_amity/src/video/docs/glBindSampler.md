# glBindSampler
bind a named sampler to a texturing target

## Parameters
- `unit`
  Specifies the index of the texture unit to which the sampler is bound.

## Description
[`Gl::bind_sampler`] binds `sampler` to the texture unit at index
  `unit`. `sampler` must be zero or the name of a sampler object
  previously returned from a call to [`Gl::gen_samplers`]. `unit` must
  be less than the value of [`gl::MAX_COMBINED_TEXTURE_IMAGE_UNITS`].
When a sampler object is bound to a texture unit, its state supersedes
  that of the texture object bound to that texture unit. If the sampler
  name zero is bound to a texture unit, the currently bound texture's
  sampler state becomes active. A single sampler object may be bound to
  multiple texture units simultaneously.

## Notes
[`Gl::bind_sampler`] is available only if the GL version is 3.3 or
  higher.

## Errors
- [`gl::INVALID_VALUE`] is generated if `unit` is greater than or equal
  to the value of [`gl::MAX_COMBINED_TEXTURE_IMAGE_UNITS`].
- [`gl::INVALID_OPERATION`] is generated if `sampler` is not zero or a
  name previously returned from a call to [`Gl::gen_samplers`], or if
  such a name has been deleted by a call to [`Gl::delete_samplers`].

## See Also
- [`Gl::gen_samplers`]
- [`Gl::delete_samplers`]
- [`Gl::get`]
- [`Gl::sampler_parameter`]
- [`Gl::get_sampler_parameter`]
- [`Gl::gen_textures`]
- [`Gl::bind_texture`]
- [`Gl::delete_textures`]
