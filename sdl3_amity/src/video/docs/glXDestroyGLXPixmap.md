# glXDestroyGLXPixmap
destroy a GLX pixmap

## Parameters
- `dpy`
  Specifies the connection to the X server.

## Description
If the GLX pixmap `pix` is not current to any client,
  [`Gl::x_destroy_glx_pixmap`] destroys it immediately. Otherwise, `pix`
  is destroyed when it becomes not current to any client. In either
  case, the resource ID is freed immediately.

## Errors
- [`GLXBadPixmap`] is generated if `pix` is not a valid GLX pixmap.

## See Also
- [`Gl::x_create_glx_pixmap`]
- [`Gl::x_destroy_pixmap`]
- [`Gl::x_make_current`]
