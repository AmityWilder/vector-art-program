# glBindSamplers
bind one or more named sampler objects to a sequence of consecutive
  sampler units

## Parameters
- `first`
  Specifies the first sampler unit to which a sampler object is to be
  bound.

## Description
[`Gl::bind_samplers`] binds samplers from an array of existing sampler
  objects to a specified number of consecutive sampler units. `count`
  specifies the number of sampler objects whose names are stored in the
  array `samplers`. That number of sampler names is read from the array
  and bound to the `count` consecutive sampler units starting from
  `first`.
If the name zero appears in the `samplers` array, any existing binding
  to the sampler unit is reset. Any non-zero entry in `samplers` must be
  the name of an existing sampler object. When a non-zero entry in
  `samplers` is present, that sampler object is bound to the
  corresponding sampler unit. If `samplers` is [`NULL`] then it is as if
  an appropriately sized array containing only zeros had been specified.
[`Gl::bind_samplers`] is equivalent to the following pseudo code:
Each entry in `samplers` will be checked individually and if found to
  be invalid, the state for that sampler unit will not be changed and an
  error will be generated. However, the state for other sampler units
  referenced by the command will still be updated.

## Notes
[`Gl::bind_samplers`] is available only if the GL version is 4.4 or
  higher.

## Errors
- [`gl::INVALID_OPERATION`] is generated if `first` + `count` is greater
  than the number of sampler units supported by the implementation.
- [`gl::INVALID_OPERATION`] is generated if any value in `samplers` is
  not zero or the name of an existing sampler object.

## See Also
- [`Gl::gen_samplers`]
- [`Gl::bind_sampler`]
- [`Gl::delete_samplers`]
- [`Gl::get`]
- [`Gl::sampler_parameter`]
- [`Gl::get_sampler_parameter`]
- [`Gl::gen_textures`]
- [`Gl::bind_texture`]
- [`Gl::delete_textures`]
