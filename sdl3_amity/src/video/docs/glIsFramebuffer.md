# glIsFramebuffer
determine if a name corresponds to a framebuffer object

## Parameters
- `framebuffer`
  Specifies a value that may be the name of a framebuffer object.

## Description
[`Gl::is_framebuffer`] returns [`gl::TRUE`] if `framebuffer` is
  currently the name of a framebuffer object. If `framebuffer` is zero,
  or if [`framebuffer`] is not the name of a framebuffer object, or if
  an error occurs, [`Gl::is_framebuffer`] returns [`gl::FALSE`]. If
  `framebuffer` is a name returned by [`Gl::gen_framebuffers`], by that
  has not yet been bound through a call to [`Gl::bind_framebuffer`],
  then the name is not a framebuffer object and [`Gl::is_framebuffer`]
  returns [`gl::FALSE`].

## See Also
- [`Gl::gen_framebuffers`]
- [`Gl::bind_framebuffer`]
- [`Gl::delete_framebuffers`]
