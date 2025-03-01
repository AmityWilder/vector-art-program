# glDeleteBuffers
delete named buffer objects

## Parameters
- `n`
  Specifies the number of buffer objects to be deleted.

## Description
[`Gl::delete_buffers`] deletes `n` buffer objects named by the
  elements of the array `buffers`. After a buffer object is deleted, it
  has no contents, and its name is free for reuse (for example by
  [`Gl::gen_buffers`]). If a buffer object that is currently bound is
  deleted, the binding reverts to 0 (the absence of any buffer object).
[`Gl::delete_buffers`] silently ignores 0's and names that do not
  correspond to existing buffer objects.

## Errors
- [`gl::INVALID_VALUE`] is generated if `n` is negative.

## See Also
- [`Gl::bind_buffer`]
- [`Gl::gen_buffers`]
- [`Gl::get`]
