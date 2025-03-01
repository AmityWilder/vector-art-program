# glCreateBuffers
create buffer objects

## Parameters
- `n`
  Specifies the number of buffer objects to create.

## Description
[`Gl::create_buffers`] returns `n` previously unused buffer names in
  `buffers`, each representing a new buffer object initialized as if it
  had been bound to an unspecified target.

## Errors
- [`gl::INVALID_VALUE`] is generated if `n` is negative.

## See Also
- [`Gl::gen_buffers`]
- [`Gl::bind_buffer_base`]
- [`Gl::bind_buffer_range`]
- [`Gl::map_buffer`]
- [`Gl::unmap_buffer`]
- [`Gl::delete_buffers`]
- [`Gl::get`]
- [`Gl::is_buffer`]
