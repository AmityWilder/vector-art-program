# glXIsDirect
indicate whether direct rendering is enabled

## Parameters
- `dpy`
  Specifies the connection to the X server.

## Description
[`Gl::x_is_direct`] returns [`True`] if `ctx` is a direct rendering
  context, [`False`] otherwise. Direct rendering contexts pass rendering
  commands directly from the calling process's address space to the
  rendering system, bypassing the X server. Nondirect rendering contexts
  pass all rendering commands to the X server.

## Errors
- [`GLXBadContext`] is generated if `ctx` is not a valid GLX context.

## See Also
- [`Gl::x_create_context`]
- [`Gl::x_create_new_context`]
