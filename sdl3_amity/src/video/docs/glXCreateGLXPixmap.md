# glXCreateGLXPixmap
create an off-screen GLX rendering area

## Parameters
- `dpy`
  Specifies the connection to the X server.

## Description
[`Gl::x_create_glx_pixmap`] creates an off-screen rendering area and
  returns its XID. Any GLX rendering context that was created with
  respect to `vis` can be used to render into this off-screen area. Use
  [`Gl::x_make_current`] to associate the rendering area with a GLX
  rendering context.
The X pixmap identified by `pixmap` is used as the front left buffer
  of the resulting off-screen rendering area. All other buffers
  specified by `vis`, including color buffers other than the front left
  buffer, are created without externally visible names. GLX pixmaps with
  double-buffering are supported. However, [`Gl::x_swap_buffers`] is
  ignored by these pixmaps.
Some implementations may not support GLX pixmaps with direct rendering
  contexts.

## Notes
[`XVisualInfo`] is defined in *Xutil.h.* It is a structure that
  includes *visual*, *visualID*, *screen*, and *depth* elements.

## Errors
- [`BadMatch`] is generated if the depth of `pixmap` does not match the
  depth value reported by core X11 for `vis`, or if `pixmap` was not
  created with respect to the same screen as `vis`.
- [`BadValue`] is generated if `vis` is not a valid XVisualInfo pointer
  (for example, if a particular GLX implementation does not support this
  visual).
- [`BadPixmap`] is generated if `pixmap` is not a valid pixmap.
- [`BadAlloc`] is generated if the server cannot allocate the GLX
  pixmap.

## See Also
- [`Gl::x_create_context`]
- [`Gl::x_create_pixmap`]
- [`Gl::x_destroy_glx_pixmap`]
- [`Gl::x_is_direct`]
- [`Gl::x_make_current`]
