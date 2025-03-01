# glBindBuffersBase
bind one or more buffer objects to a sequence of indexed buffer
  targets

## Parameters
- `target`
  Specify the target of the bind operation. `target` must be one of
  [`gl::ATOMIC_COUNTER_BUFFER`], [`gl::TRANSFORM_FEEDBACK_BUFFER`],
  [`gl::UNIFORM_BUFFER`] or [`gl::SHADER_STORAGE_BUFFER`].

## Description
[`Gl::bind_buffers_base`] binds a set of `count` buffer objects whose
  names are given in the array `buffers` to the `count` consecutive
  binding points starting from index `first` of the array of targets
  specified by `target`. If `buffers` is [`NULL`] then
  [`Gl::bind_buffers_base`] unbinds any buffers that are currently bound
  to the referenced binding points. Assuming no errors are generated, it
  is equivalent to the following pseudo-code, which calls
  [`Gl::bind_buffer_base`], with the exception that the non-indexed
  `target` is not changed by [`Gl::bind_buffers_base`]:
Each entry in `buffers` will be checked individually and if found to
  be invalid, the state for that buffer binding index will not be
  changed and an error will be generated. However, the state for other
  buffer binding indices referenced by the command will still be
  updated.

## Notes
[`Gl::bind_buffers_base`] is available only if the GL version is 4.4
  or higher.

## Errors
- [`gl::INVALID_ENUM`] is generated if `target` is not
  [`gl::ATOMIC_COUNTER_BUFFER`], [`gl::TRANSFORM_FEEDBACK_BUFFER`],
  [`gl::UNIFORM_BUFFER`] or [`gl::SHADER_STORAGE_BUFFER`].
- [`gl::INVALID_OPERATION`] is generated if `first` + `count` is greater
  than the number of target-specific indexed binding points.
- [`gl::INVALID_OPERATION`] is generated if any value in `buffers` is
  not zero or the name of an existing buffer object.

## See Also
- [`Gl::gen_buffers`]
- [`Gl::delete_buffers`]
- [`Gl::bind_buffer`]
- [`Gl::bind_buffer_base`]
- [`Gl::bind_buffer_range`]
- [`Gl::bind_buffers_range`]
- [`Gl::map_buffer`]
- [`Gl::unmap_buffer`]
