# glXGetCurrentContext
return the current context

## Description
[`Gl::x_get_current_context`] returns the current context, as
  specified by [`Gl::x_make_current`]. If there is no current context,
  [`NULL`] is returned.
[`Gl::x_get_current_context`] returns client-side information. It does
  not make a round trip to the server.


## See Also
- [`Gl::x_create_context`]
- [`Gl::x_get_current_display`]
- [`Gl::x_get_current_drawable`]
- [`Gl::x_make_current`]
