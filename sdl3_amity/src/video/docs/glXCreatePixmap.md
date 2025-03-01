# glXCreatePixmap
create an off-screen rendering area

## Parameters
- `dpy`
  Specifies the connection to the X server.

## Description
[`Gl::x_create_pixmap`] creates an off-screen rendering area and
  returns its XID. Any GLX rendering context that was created with
  respect to `config` can be used to render into this window. Use
  [`Gl::x_make_current`] to associate the rendering area with a GLX
  rendering context.

## Notes
[`Gl::x_create_pixmap`] is available only if the GLX version is 1.3 or
  greater.
If the GLX version is 1.1 or 1.0, the GL version must be 1.0. If the
  GLX version is 1.2, then the GL version must be 1.1. If the GLX
  version is 1.3, then the GL version must be 1.2.

## Errors
- [`BadMatch`] is generated if `pixmap` was not created with a visual
  that corresponds to `config`.
- [`BadMatch`] is generated if `config` does not support rendering to
  windows (e.g., [`GLX_DRAWABLE_TYPE`] does not contain
  [`GLX_WINDOW_BIT`]).
- [`BadWindow`] is generated if `pixmap` is not a valid window XID.
  [`BadAlloc`] is generated if there is already a GLXFBConfig associated
  with `pixmap`.
- [`BadAlloc`] is generated if the X server cannot allocate a new GLX
  window.
- [`GLXBadFBConfig`] is generated if `config` is not a valid
  GLXFBConfig.
- 

## See Also
- [`Gl::x_choose_fb_config`]
- [`Gl::x_create_glx_pixmap`]
- [`Gl::x_destroy_window`]
- [`Gl::x_make_context_current`]
