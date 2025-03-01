# glXGetCurrentReadDrawable
return the current drawable

## Description
[`Gl::x_get_current_read_drawable`] returns the current read drawable,
  as specified by [`read`] parameter of [`Gl::x_make_context_current`].
  If there is no current drawable, [`None`] is returned.
[`Gl::x_get_current_read_drawable`] returns client-side information.
  It does not make a round-trip to the server.

## Notes
[`Gl::x_get_current_read_drawable`] is only supported if the GLX
  version is 1.3 or greater.

## See Also
- [`Gl::x_get_current_context`]
- [`Gl::x_get_current_display`]
- [`Gl::x_get_current_drawable`]
- [`Gl::x_make_context_current`]
