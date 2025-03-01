# glXGetCurrentDisplay
get display for current context

## Description
[`Gl::x_get_current_display`] returns the display for the current
  context. If no context is current, [`NULL`] is returned.
[`Gl::x_get_current_display`] returns client-side information. It does
  not make a round-trip to the server, and therefore does not flush any
  pending events.

## Notes
[`Gl::x_get_current_display`] is only supported if the GLX version is
  1.2 or greater.

## See Also
- [`Gl::x_get_current_context`]
- [`Gl::x_get_current_drawable`]
- [`Gl::x_query_version`]
- [`Gl::x_query_extensions_string`]
