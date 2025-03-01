# glBindBufferRange
bind a range within a buffer object to an indexed buffer target

## Parameters
- `target`
  Specify the target of the bind operation. `target` must be one of
  [`gl::ATOMIC_COUNTER_BUFFER`], [`gl::TRANSFORM_FEEDBACK_BUFFER`],
  [`gl::UNIFORM_BUFFER`], or [`gl::SHADER_STORAGE_BUFFER`].

## Description
[`Gl::bind_buffer_range`] binds a range the buffer object `buffer`
  represented by `offset` and `size` to the binding point at index
  `index` of the array of targets specified by `target`. Each `target`
  represents an indexed array of buffer binding points, as well as a
  single general binding point that can be used by other buffer
  manipulation functions such as [`Gl::bind_buffer`] or
  [`Gl::map_buffer`]. In addition to binding a range of `buffer` to the
  indexed buffer binding target, [`Gl::bind_buffer_range`] also binds
  the range to the generic buffer binding point specified by `target`.
`offset` specifies the offset in basic machine units into the buffer
  object `buffer` and `size` specifies the amount of data that can be
  read from the buffer object while used as an indexed target.

## Notes
The [`gl::ATOMIC_COUNTER_BUFFER`] target is available only if the GL
  version is 4.2 or greater.
The [`gl::SHADER_STORAGE_BUFFER`] target is available only if the GL
  version is 4.3 or greater.

## Errors
- [`gl::INVALID_ENUM`] is generated if `target` is not one of
  [`gl::ATOMIC_COUNTER_BUFFER`], [`gl::TRANSFORM_FEEDBACK_BUFFER`],
  [`gl::UNIFORM_BUFFER`] or [`gl::SHADER_STORAGE_BUFFER`].
- [`gl::INVALID_VALUE`] is generated if `index` is greater than or equal
  to the number of `target`-specific indexed binding points.
- [`gl::INVALID_VALUE`] is generated if `size` is less than or equal to
  zero, or if `offset` + `size` is greater than the value of
  [`gl::BUFFER_SIZE`].
- Additional errors may be generated if `offset` violates any
  `target`-specific alignmemt restrictions.

## See Also
- [`Gl::gen_buffers`]
- [`Gl::delete_buffers`]
- [`Gl::bind_buffer`]
- [`Gl::bind_buffer_base`]
- [`Gl::map_buffer`]
- [`Gl::unmap_buffer`]
