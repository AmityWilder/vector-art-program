# glDeleteFramebuffers
delete framebuffer objects

## Parameters
- `n`
  Specifies the number of framebuffer objects to be deleted.

## Description
[`Gl::delete_framebuffers`] deletes the `n` framebuffer objects whose
  names are stored in the array addressed by `framebuffers`. The name
  zero is reserved by the GL and is silently ignored, should it occur in
  `framebuffers`, as are other unused names. Once a framebuffer object
  is deleted, its name is again unused and it has no attachments. If a
  framebuffer that is currently bound to one or more of the targets
  [`gl::DRAW_FRAMEBUFFER`] or [`gl::READ_FRAMEBUFFER`] is deleted, it is
  as though [`Gl::bind_framebuffer`] had been executed with the
  corresponding `target` and `framebuffer` zero.

## Errors
- [`gl::INVALID_VALUE`] is generated if `n` is negative.

## See Also
- [`Gl::gen_framebuffers`]
- [`Gl::bind_framebuffer`]
- [`Gl::check_framebuffer_status`]
