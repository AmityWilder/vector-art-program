# glXMakeContextCurrent
attach a GLX context to a GLX drawable

## Parameters
- `display`
  Specifies the connection to the X server.

## Description
[`Gl::x_make_context_current`] binds `ctx` to the current rendering
  thread and to the `draw` and `read` GLX drawables. `draw` and `read`
  may be the same.
`draw` is used for all OpenGL operations except:
Any pixel data that are read based on the value of
  [`GLX_READ_BUFFER`]. Note that accumulation operations use the value
  of [`GLX_READ_BUFFER`], but are not allowed unless `draw` is identical
  to `read`.
Any depth values that are retrieved by [`Gl::read_pixels`] or
  [`Gl::copy_pixels`].
Any stencil values that are retrieved by [`Gl::read_pixels`] or
  [`Gl::copy_pixels`].
Frame buffer values are taken from `draw`.
If the current rendering thread has a current rendering context, that
  context is flushed and replaced by `ctx`.
The first time that `ctx` is made current, the viewport and scissor
  dimensions are set to the size of the `draw` drawable. The viewport
  and scissor are not modified when `ctx` is subsequently made current.
To release the current context without assigning a new one, call
  [`Gl::x_make_context_current`] with `draw` and `read` set to [`None`]
  and `ctx` set to [`NULL`].
[`Gl::x_make_context_current`] returns [`True`] if it is successful,
  [`False`] otherwise. If [`False`] is returned, the previously current
  rendering context and drawable (if any) remain unchanged.

## Notes
[`Gl::x_make_context_current`] is available only if the GLX version is
  1.3 or greater.
If the GLX version is 1.1 or 1.0, the GL version must be 1.0. If the
  GLX version is 1.2, then the GL version must be 1.1. If the GLX
  version is 1.3, then the GL version must be 1.2.


## Errors
- [`BadMatch`] is generated if `draw` and `read` are not compatible.
- [`BadAccess`] is generated if `ctx` is current to some other thread.
- [`GLXContextState`] is generated if there is a current rendering
  context and its render mode is either [`GLX_FEEDBACK`] or
  [`GLX_SELECT`].
- [`GLXBadContext`] is generated if `ctx` is not a valid GLX rendering
  context.
- [`GLXBadDrawable`] is generated if `draw` or `read` is not a valid GLX
  drawable.
- [`GLXBadWindow`] is generated if the underlying X window for either
  `draw` or `read` is no longer valid.
- [`GLXBadCurrentDrawable`] is generated if the previous context of the
  calling thread has unflushed commands and the previous drawable is no
  longer valid.
- [`BadAlloc`] is generated if the X server does not have enough
  resources to allocate the buffers.
- [`BadMatch`] is generated if:
- `draw` and `read` cannot fit into frame buffer memory simultaneously.
- `draw` or `read` is a GLXPixmap and `ctx` is a direct-rendering
  context.
- `draw` or `read` is a GLXPixmap and `ctx` was previously bound to a
  GLXWindow or GLXPbuffer.
- `draw` or `read` is a GLXWindow or GLXPbuffer and `ctx` was previously
  bound to a GLXPixmap.

## See Also
- [`Gl::x_create_new_context`]
- [`Gl::x_create_window`]
- [`Gl::x_create_pixmap`]
- [`Gl::x_create_pbuffer`]
- [`Gl::x_destroy_context`]
- [`Gl::x_get_current_context`]
- [`Gl::x_get_current_display`]
- [`Gl::x_get_current_drawable`]
- [`Gl::x_get_current_read_drawable`]
- [`Gl::x_make_current`]
