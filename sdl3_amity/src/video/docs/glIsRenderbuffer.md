# glIsRenderbuffer
determine if a name corresponds to a renderbuffer object

## Parameters
- `renderbuffer`
  Specifies a value that may be the name of a renderbuffer object.

## Description
[`Gl::is_renderbuffer`] returns [`gl::TRUE`] if `renderbuffer` is
  currently the name of a renderbuffer object. If `renderbuffer` is
  zero, or if `renderbuffer` is not the name of a renderbuffer object,
  or if an error occurs, [`Gl::is_renderbuffer`] returns [`gl::FALSE`].
  If `renderbuffer` is a name returned by [`Gl::gen_renderbuffers`], by
  that has not yet been bound through a call to
  [`Gl::bind_renderbuffer`] or [`Gl::framebuffer_renderbuffer`], then
  the name is not a renderbuffer object and [`Gl::is_renderbuffer`]
  returns [`gl::FALSE`].

## See Also
- [`Gl::gen_renderbuffers`]
- [`Gl::bind_renderbuffer`]
- [`Gl::framebuffer_renderbuffer`]
- [`Gl::delete_renderbuffers`]
