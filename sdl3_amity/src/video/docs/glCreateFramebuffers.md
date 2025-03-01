# glCreateFramebuffers
create framebuffer objects

## Parameters
- `n`
  Number of framebuffer objects to create.

## Description
[`Gl::create_framebuffers`] returns `n` previously unused framebuffer
  names in `framebuffers`, each representing a new framebuffer object
  initialized to the default state.

## Errors
- [`gl::INVALID_VALUE`] is generated if `n` is negative.

## See Also
- [`Gl::gen_framebuffers`]
- [`Gl::bind_framebuffer`]
- [`Gl::framebuffer_renderbuffer`]
- [`Gl::framebuffer_texture`]
- [`Gl::framebuffer_texture_layer`]
- [`Gl::delete_framebuffers`]
- [`Gl::is_framebuffer`]
