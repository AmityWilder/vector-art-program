# glInvalidateBufferSubData
invalidate a region of a buffer object's data store

## Parameters
- `buffer`
  The name of a buffer object, a subrange of whose data store to
  invalidate.

## Description
[`Gl::invalidate_buffer_sub_data`] invalidates all or part of the
  content of the data store of a buffer object. After invalidation, the
  content of the specified range of the buffer's data store becomes
  undefined. The start of the range is given by `offset` and its size is
  given by `length`, both measured in basic machine units.

## Errors
- [`gl::INVALID_VALUE`] is generated if `offset` or `length` is
  negative, or if `offset` + `length` is greater than the value of
  [`gl::BUFFER_SIZE`] for `buffer`.
- [`gl::INVALID_VALUE`] is generated if `buffer` is not the name of an
  existing buffer object.
- [`gl::INVALID_OPERATION`] is generated if any part of `buffer` is
  currently mapped.

## See Also
- [`Gl::invalidate_tex_sub_image`]
- [`Gl::invalidate_tex_image`]
- [`Gl::invalidate_buffer_data`]
- [`Gl::invalidate_framebuffer`]
- [`Gl::invalidate_sub_framebuffer`]
