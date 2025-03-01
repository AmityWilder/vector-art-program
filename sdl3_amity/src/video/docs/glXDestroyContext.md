# glXDestroyContext
destroy a GLX context

## Parameters
- `dpy`
  Specifies the connection to the X server.

## Description
If the GLX rendering context `ctx` is not current to any thread,
  [`Gl::x_destroy_context`] destroys it immediately. Otherwise, `ctx` is
  destroyed when it becomes not current to any thread. In either case,
  the resource ID referenced by `ctx` is freed immediately.

## Errors
- [`GLXBadContext`] is generated if `ctx` is not a valid GLX context.

## See Also
- [`Gl::x_create_context`]
- [`Gl::x_create_new_context`]
- [`Gl::x_make_current`]
