# glObjectLabel
label a named object identified within a namespace

## Parameters
- `identifier`
  The namespace from which the name of the object is allocated.

## Description
[`Gl::object_label`] labels the object identified by `name` within the
  namespace given by `identifier`. `identifier` must be one of
  [`gl::BUFFER`], [`gl::SHADER`], [`gl::PROGRAM`], [`gl::VERTEX_ARRAY`],
  [`gl::QUERY`], [`gl::PROGRAM_PIPELINE`], [`gl::TRANSFORM_FEEDBACK`],
  [`gl::SAMPLER`], [`gl::TEXTURE`], [`gl::RENDERBUFFER`],
  [`gl::FRAMEBUFFER`], to indicate the namespace containing the names of
  buffers, shaders, programs, vertex array objects, query objects,
  program pipelines, transform feedback objects, samplers, textures,
  renderbuffers and frame buffers, respectively.
`label` is the address of a string that will be used to label an
  object. `length` contains the number of characters in `label`. If
  `length` is negative, it is implied that `label` contains a null-
  terminated string. If `label` is NULL, any debug label is effectively
  removed from the object.

## Errors
- [`gl::INVALID_ENUM`] is generated if `identifier` is not one of the
  accepted object types.
- [`gl::INVALID_OPERATION`] is generated if `name` is not the name of an
  existing object of the type specified by `identifier`.
- [`gl::INVALID_VALUE`] is generated if the number of characters in
  `label`, excluding the null terminator when `length` is negative, is
  greater than the value of [`gl::MAX_LABEL_LENGTH`].

## See Also
- [`Gl::push_debug_group`]
- [`Gl::pop_debug_group`]
- [`Gl::object_ptr_label`]
