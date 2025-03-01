# glBindFramebuffer
bind a framebuffer to a framebuffer target

## Parameters
- `target`
  Specifies the framebuffer target of the binding operation.

## Description
[`Gl::bind_framebuffer`] binds the framebuffer object with name
  `framebuffer` to the framebuffer target specified by `target`.
  `target` must be either [`gl::DRAW_FRAMEBUFFER`],
  [`gl::READ_FRAMEBUFFER`] or [`gl::FRAMEBUFFER`]. If a framebuffer
  object is bound to [`gl::DRAW_FRAMEBUFFER`] or
  [`gl::READ_FRAMEBUFFER`], it becomes the target for rendering or
  readback operations, respectively, until it is deleted or another
  framebuffer is bound to the corresponding bind point. Calling
  [`Gl::bind_framebuffer`] with `target` set to [`gl::FRAMEBUFFER`]
  binds `framebuffer` to both the read and draw framebuffer targets.
  `framebuffer` is the name of a framebuffer object previously returned
  from a call to [`Gl::gen_framebuffers`], or zero to break the existing
  binding of a framebuffer object to `target`.

## Errors
- [`gl::INVALID_ENUM`] is generated if `target` is not
  [`gl::DRAW_FRAMEBUFFER`], [`gl::READ_FRAMEBUFFER`] or
  [`gl::FRAMEBUFFER`].
- [`gl::INVALID_OPERATION`] is generated if `framebuffer` is not zero or
  the name of a framebuffer previously returned from a call to
  [`Gl::gen_framebuffers`].

## See Also
- [`Gl::gen_framebuffers`]
- [`Gl::framebuffer_renderbuffer`]
- [`Gl::framebuffer_texture`]
- [`Gl::framebuffer_texture_layer`]
- [`Gl::delete_framebuffers`]
- [`Gl::is_framebuffer`]
