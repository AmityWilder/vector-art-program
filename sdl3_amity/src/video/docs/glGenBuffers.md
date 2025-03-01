# glGenBuffers
generate buffer object names

## Parameters
- `n`
  Specifies the number of buffer object names to be generated.

## Description
[`Gl::gen_buffers`] returns `n` buffer object names in `buffers`.
  There is no guarantee that the names form a contiguous set of
  integers; however, it is guaranteed that none of the returned names
  was in use immediately before the call to [`Gl::gen_buffers`].
Buffer object names returned by a call to [`Gl::gen_buffers`] are not
  returned by subsequent calls, unless they are first deleted with
  [`Gl::delete_buffers`].
No buffer objects are associated with the returned buffer object names
  until they are first bound by calling [`Gl::bind_buffer`].

## Errors
- [`gl::INVALID_VALUE`] is generated if `n` is negative.

## See Also
- [`Gl::bind_buffer`]
- [`Gl::delete_buffers`]
- [`Gl::get`]
