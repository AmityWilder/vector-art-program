# glXDestroyPixmap
destroy an off-screen rendering area

## Parameters
- `dpy`
  Specifies the connection to the X server.

## Description
[`Gl::x_destroy_pixmap`] destroys a GLXPixmap created by
  [`Gl::x_create_pixmap`].

## Notes
[`Gl::x_destroy_pixmap`] is available only if the GLX version is 1.3
  or greater.
If the GLX version is 1.1 or 1.0, the GL version must be 1.0. If the
  GLX version is 1.2, then the GL version must be 1.1. If the GLX
  version is 1.3, then the GL version must be 1.2.

## Errors
- [`GLXBadPixmap`] is generated if `pixmap` is not a valid GLXPixmap.

## See Also
- [`Gl::x_choose_fb_config`]
- [`Gl::x_create_pixmap`]
- [`Gl::x_destroy_glx_pixmap`]
- [`Gl::x_make_context_current`]
