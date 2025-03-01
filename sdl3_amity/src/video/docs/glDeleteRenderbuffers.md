# glDeleteRenderbuffers
delete renderbuffer objects

## Parameters
- `n`
  Specifies the number of renderbuffer objects to be deleted.

## Description
[`Gl::delete_renderbuffers`] deletes the `n` renderbuffer objects
  whose names are stored in the array addressed by `renderbuffers`. The
  name zero is reserved by the GL and is silently ignored, should it
  occur in `renderbuffers`, as are other unused names. Once a
  renderbuffer object is deleted, its name is again unused and it has no
  contents. If a renderbuffer that is currently bound to the target
  [`gl::RENDERBUFFER`] is deleted, it is as though
  [`Gl::bind_renderbuffer`] had been executed with a `target` of
  [`gl::RENDERBUFFER`] and a `name` of zero.
If a renderbuffer object is attached to one or more attachment points
  in the currently bound framebuffer, then it as if
  [`Gl::framebuffer_renderbuffer`] had been called, with a
  `renderbuffer` of zero for each attachment point to which this image
  was attached in the currently bound framebuffer. In other words, this
  renderbuffer object is first detached from all attachment ponits in
  the currently bound framebuffer. Note that the renderbuffer image is
  specifically *not* detached from any non-bound framebuffers.

## Errors
- [`gl::INVALID_VALUE`] is generated if `n` is negative.

## See Also
- [`Gl::gen_renderbuffers`]
- [`Gl::framebuffer_renderbuffer`]
- [`Gl::renderbuffer_storage`]
- [`Gl::renderbuffer_storage_multisample`]
