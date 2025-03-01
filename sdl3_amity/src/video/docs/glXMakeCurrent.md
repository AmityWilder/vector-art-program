# glXMakeCurrent
attach a GLX context to a window or a GLX pixmap

## Parameters
- `dpy`
  Specifies the connection to the X server.

## Description
[`Gl::x_make_current`] does two things: It makes `ctx` the current GLX
  rendering context of the calling thread, replacing the previously
  current context if there was one, and it attaches `ctx` to a GLX
  drawable, either a window or a GLX pixmap. As a result of these two
  actions, subsequent GL rendering calls use rendering context `ctx` to
  modify GLX drawable `drawable` (for reading and writing). Because
  [`Gl::x_make_current`] always replaces the current rendering context
  with `ctx`, there can be only one current context per thread.
Pending commands to the previous context, if any, are flushed before
  it is released.
The first time `ctx` is made current to any thread, its viewport is
  set to the full size of `drawable`. Subsequent calls by any thread to
  [`Gl::x_make_current`] with `ctx` have no effect on its viewport.
To release the current context without assigning a new one, call
  [`Gl::x_make_current`] with `drawable` set to [`None`] and `ctx` set
  to [`NULL`].
[`Gl::x_make_current`] returns [`True`] if it is successful, [`False`]
  otherwise. If [`False`] is returned, the previously current rendering
  context and drawable (if any) remain unchanged.

## Notes
A *process* is a single-execution environment, implemented in a single
  address space, consisting of one or more threads.
A *thread* is one of a set of subprocesses that share a single address
  space, but maintain separate program counters, stack spaces, and other
  related global data. A *thread* that is the only member of its
  subprocess group is equivalent to a *process*.

## Errors
- [`BadMatch`] is generated if `drawable` was not created with the same
  X screen and visual as `ctx`. It is also generated if `drawable` is
  [`None`] and `ctx` is not [`NULL`].
- [`BadAccess`] is generated if `ctx` was current to another thread at
  the time [`Gl::x_make_current`] was called.
- [`GLXBadDrawable`] is generated if `drawable` is not a valid GLX
  drawable.
- [`GLXBadContext`] is generated if `ctx` is not a valid GLX context.
- [`GLXBadContextState`] is generated if [`Gl::x_make_current`] is
  executed between the execution of [`Gl::begin`] and the corresponding
  execution of [`Gl::end`].
- [`GLXBadContextState`] is also generated if the rendering context
  current to the calling thread has GL renderer state [`GLX_FEEDBACK`]
  or [`GLX_SELECT`].
- [`GLXBadCurrentWindow`] is generated if there are pending GL commands
  for the previous context and the current drawable is a window that is
  no longer valid.
- [`BadAlloc`] may be generated if the server has delayed allocation of
  ancillary buffers until [`Gl::x_make_current`] is called, only to find
  that it has insufficient resources to complete the allocation.

## See Also
- [`Gl::x_create_context`]
- [`Gl::x_create_glx_pixmap`]
- [`Gl::x_get_current_context`]
- [`Gl::x_get_current_display`]
- [`Gl::x_get_current_drawable`]
- [`Gl::x_get_current_read_drawable`]
- [`Gl::x_make_context_current`]
