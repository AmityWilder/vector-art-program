# glGetObjectLabel
retrieve the label of a named object identified within a namespace

## Parameters
- `identifier`
  The namespace from which the name of the object is allocated.

## Description
[`Gl::get_object_label`] retrieves the label of the object identified
  by `name` within the namespace given by `identifier`. `identifier`
  must be one of [`gl::BUFFER`], [`gl::SHADER`], [`gl::PROGRAM`],
  [`gl::VERTEX_ARRAY`], [`gl::QUERY`], [`gl::PROGRAM_PIPELINE`],
  [`gl::TRANSFORM_FEEDBACK`], [`gl::SAMPLER`], [`gl::TEXTURE`],
  [`gl::RENDERBUFFER`], [`gl::FRAMEBUFFER`], to indicate the namespace
  containing the names of buffers, shaders, programs, vertex array
  objects, query objects, program pipelines, transform feedback objects,
  samplers, textures, renderbuffers and frame buffers, respectively.
`label` is the address of a string that will be used to store the
  object label. `bufSize` specifies the number of characters in the
  array identified by `label`. `length` contains the address of a
  variable which will receive the number of characters in the object
  label. If `length` is NULL, then it is ignored and no data is written.
  Likewise, if `label` is NULL, or if `bufSize` is zero then no data is
  written to `label`.

## Errors
- [`gl::INVALID_ENUM`] is generated if `identifier` is not one of the
  accepted object types.
- [`gl::INVALID_OPERATION`] is generated if `name` is not the name of an
  existing object of the type specified by `identifier`.
- [`gl::INVALID_VALUE`] is generated if `bufSize` is zero.
- If not NULL, `length` and `label` should be addresses to which the
  client has write access, otherwise undefined behavior, including
  process termination may occur.

## See Also
- [`Gl::push_debug_group`]
- [`Gl::pop_debug_group`]
- [`Gl::object_label`]
- [`Gl::get_object_ptr_label`]
