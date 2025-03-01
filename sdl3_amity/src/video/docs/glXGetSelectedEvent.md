# glXGetSelectedEvent
returns GLX events that are selected for a window or a GLX pixel
  buffer

## Parameters
- `dpy`
  Specifies the connection to the X server.

## Description
[`Gl::x_get_selected_event`] returns in `event_mask` the events
  selected for `draw`.

## Notes
[`Gl::x_get_selected_event`] is available only if the GLX version is
  1.3 or greater.
If the GLX version is 1.1 or 1.0, the GL version must be 1.0. If the
  GLX version is 1.2, then the GL version must be 1.1. If the GLX
  version is 1.3, then the GL version must be 1.2.

## Errors
- [`GLXBadDrawable`] is generated if `draw` is not a valid window or a
  valid GLX pixel buffer.

## See Also
- [`Gl::x_select_event`]
- [`Gl::x_create_pbuffer`]
