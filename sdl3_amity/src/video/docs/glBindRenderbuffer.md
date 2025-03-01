# glBindRenderbuffer
bind a renderbuffer to a renderbuffer target

## Parameters
- `target`
  Specifies the renderbuffer target of the binding operation. `target`
  must be [`gl::RENDERBUFFER`].

## Description
[`Gl::bind_renderbuffer`] binds the renderbuffer object with name
  `renderbuffer` to the renderbuffer target specified by `target`.
  `target` must be [`gl::RENDERBUFFER`]. `renderbuffer` is the name of a
  renderbuffer object previously returned from a call to
  [`Gl::gen_renderbuffers`], or zero to break the existing binding of a
  renderbuffer object to `target`.

## Errors
- [`gl::INVALID_ENUM`] is generated if `target` is not
  [`gl::RENDERBUFFER`].
- [`gl::INVALID_OPERATION`] is generated if `renderbuffer` is not zero
  or the name of a renderbuffer previously returned from a call to
  [`Gl::gen_renderbuffers`].

## See Also
- [`Gl::delete_renderbuffers`]
- [`Gl::gen_renderbuffers`]
- [`Gl::is_renderbuffer`]
- [`Gl::renderbuffer_storage`]
- [`Gl::renderbuffer_storage_multisample`]
