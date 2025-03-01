# glBindBufferBase
bind a buffer object to an indexed buffer target

## Parameters
- `target`
  Specify the target of the bind operation. `target` must be one of
  [`gl::ATOMIC_COUNTER_BUFFER`], [`gl::TRANSFORM_FEEDBACK_BUFFER`],
  [`gl::UNIFORM_BUFFER`] or [`gl::SHADER_STORAGE_BUFFER`].

## Description
[`Gl::bind_buffer_base`] binds the buffer object `buffer` to the
  binding point at index `index` of the array of targets specified by
  `target`. Each `target` represents an indexed array of buffer binding
  points, as well as a single general binding point that can be used by
  other buffer manipulation functions such as [`Gl::bind_buffer`] or
  [`Gl::map_buffer`]. In addition to binding `buffer` to the indexed
  buffer binding target, [`Gl::bind_buffer_base`] also binds `buffer` to
  the generic buffer binding point specified by `target`.

## Notes
Calling [`Gl::bind_buffer_base`] is equivalent to calling
  [`Gl::bind_buffer_range`] with `offset` zero and `size` equal to the
  size of the buffer.
The [`gl::ATOMIC_COUNTER_BUFFER`] target is available only if the GL
  version is 4.2 or greater.
The [`gl::SHADER_STORAGE_BUFFER`] target is available only if the GL
  version is 4.3 or greater.

## Errors
- [`gl::INVALID_ENUM`] is generated if `target` is not
  [`gl::ATOMIC_COUNTER_BUFFER`], [`gl::TRANSFORM_FEEDBACK_BUFFER`],
  [`gl::UNIFORM_BUFFER`] or [`gl::SHADER_STORAGE_BUFFER`].
- [`gl::INVALID_VALUE`] is generated if `index` is greater than or equal
  to the number of `target`-specific indexed binding points.
- [`gl::INVALID_VALUE`] is generated if `buffer` does not have an
  associated data store, or if the size of that store is zero.

## See Also
- [`Gl::gen_buffers`]
- [`Gl::delete_buffers`]
- [`Gl::bind_buffer`]
- [`Gl::bind_buffer_range`]
- [`Gl::map_buffer`]
- [`Gl::unmap_buffer`]
