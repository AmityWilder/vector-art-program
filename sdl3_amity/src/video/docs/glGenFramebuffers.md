# glGenFramebuffers
generate framebuffer object names

## Parameters
- `n`
  Specifies the number of framebuffer object names to generate.

## Description
[`Gl::gen_framebuffers`] returns `n` framebuffer object names in
  `ids`. There is no guarantee that the names form a contiguous set of
  integers; however, it is guaranteed that none of the returned names
  was in use immediately before the call to [`Gl::gen_framebuffers`].
Framebuffer object names returned by a call to
  [`Gl::gen_framebuffers`] are not returned by subsequent calls, unless
  they are first deleted with [`Gl::delete_framebuffers`].
The names returned in `ids` are marked as used, for the purposes of
  [`Gl::gen_framebuffers`] only, but they acquire state and type only
  when they are first bound.

## Errors
- [`gl::INVALID_VALUE`] is generated if `n` is negative.

## See Also
- [`Gl::bind_framebuffer`]
- [`Gl::delete_framebuffers`]
