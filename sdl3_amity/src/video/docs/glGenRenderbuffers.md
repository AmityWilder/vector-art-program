# glGenRenderbuffers
generate renderbuffer object names

## Parameters
- `n`
  Specifies the number of renderbuffer object names to generate.

## Description
[`Gl::gen_renderbuffers`] returns `n` renderbuffer object names in
  `renderbuffers`. There is no guarantee that the names form a
  contiguous set of integers; however, it is guaranteed that none of the
  returned names was in use immediately before the call to
  [`Gl::gen_renderbuffers`].
Renderbuffer object names returned by a call to
  [`Gl::gen_renderbuffers`] are not returned by subsequent calls, unless
  they are first deleted with [`Gl::delete_renderbuffers`].
The names returned in `renderbuffers` are marked as used, for the
  purposes of [`Gl::gen_renderbuffers`] only, but they acquire state and
  type only when they are first bound.

## Errors
- [`gl::INVALID_VALUE`] is generated if `n` is negative.

## See Also
- [`Gl::framebuffer_renderbuffer`]
- [`Gl::delete_renderbuffers`]
