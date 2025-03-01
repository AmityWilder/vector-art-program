# glIsBuffer
determine if a name corresponds to a buffer object

## Parameters
- `buffer`
  Specifies a value that may be the name of a buffer object.

## Description
[`Gl::is_buffer`] returns [`gl::TRUE`] if `buffer` is currently the
  name of a buffer object. If `buffer` is zero, or is a non-zero value
  that is not currently the name of a buffer object, or if an error
  occurs, [`Gl::is_buffer`] returns [`gl::FALSE`].
A name returned by [`Gl::gen_buffers`], but not yet associated with a
  buffer object by calling [`Gl::bind_buffer`], is not the name of a
  buffer object.

## See Also
- [`Gl::bind_buffer`]
- [`Gl::delete_buffers`]
- [`Gl::gen_buffers`]
- [`Gl::get`]
