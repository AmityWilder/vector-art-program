# glXGetCurrentDrawable
return the current drawable

## Description
[`Gl::x_get_current_drawable`] returns the current drawable, as
  specified by [`Gl::x_make_current`]. If there is no current drawable,
  [`None`] is returned.
[`Gl::x_get_current_drawable`] returns client-side information. It
  does not make a round trip to the server.

## See Also
- [`Gl::x_create_glx_pixmap`]
- [`Gl::x_get_current_context`]
- [`Gl::x_get_current_display`]
- [`Gl::x_get_current_read_drawable`]
- [`Gl::x_make_current`]
