# glInvalidateBufferData
invalidate the content of a buffer object's data store

## Parameters
- `buffer`
  The name of a buffer object whose data store to invalidate.

## Description
[`Gl::invalidate_buffer_data`] invalidates all of the content of the
  data store of a buffer object. After invalidation, the content of the
  buffer's data store becomes undefined.

## Errors
- [`gl::INVALID_VALUE`] is generated if `buffer` is not the name of an
  existing buffer object.
- [`gl::INVALID_OPERATION`] is generated if any part of `buffer` is
  currently mapped.

## See Also
- [`Gl::invalidate_tex_sub_image`]
- [`Gl::invalidate_tex_image`]
- [`Gl::invalidate_buffer_sub_data`]
- [`Gl::invalidate_framebuffer`]
- [`Gl::invalidate_sub_framebuffer`]
