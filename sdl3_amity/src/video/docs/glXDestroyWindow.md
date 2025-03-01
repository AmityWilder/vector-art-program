# glXDestroyWindow
destroy an on-screen rendering area

## Parameters
- `dpy`
  Specifies the connection to the X server.

## Description
[`Gl::x_destroy_window`] destroys a GLXWindow created by
  [`Gl::x_create_window`].

## Notes
[`Gl::x_destroy_window`] is available only if the GLX version is 1.3
  or greater.
If the GLX version is 1.1 or 1.0, the GL version must be 1.0. If the
  GLX version is 1.2, then the GL version must be 1.1. If the GLX
  version is 1.3, then the GL version must be 1.2.

## Errors
- [`GLXBadWindow`] is generated if `win` is not a valid GLXPixmap.

## See Also
- [`Gl::x_choose_fb_config`]
- [`Gl::x_create_window`]
- [`Gl::x_make_context_current`]
