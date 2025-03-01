# glXCreateContext
create a new GLX rendering context

## Parameters
- `dpy`
  Specifies the connection to the X server.

## Description
[`Gl::x_create_context`] creates a GLX rendering context and returns
  its handle. This context can be used to render into both windows and
  GLX pixmaps. If [`Gl::x_create_context`] fails to create a rendering
  context, [`NULL`] is returned.
If `direct` is [`True`], then a direct rendering context is created if
  the implementation supports direct rendering, if the connection is to
  an X server that is local, and if a direct rendering context is
  available. (An implementation may return an indirect context when
  `direct` is [`True`].) If `direct` is [`False`], then a rendering
  context that renders through the X server is always created. Direct
  rendering provides a performance advantage in some implementations.
  However, direct rendering contexts cannot be shared outside a single
  process, and they may be unable to render to GLX pixmaps.
If `shareList` is not [`NULL`], then all display-list indexes and
  definitions are shared by context `shareList` and by the newly created
  context. An arbitrary number of contexts can share a single display-
  list space. However, all rendering contexts that share a single
  display-list space must themselves exist in the same address space.
  Two rendering contexts share an address space if both are nondirect
  using the same server, or if both are direct and owned by a single
  process. Note that in the nondirect case, it is not necessary for the
  calling threads to share an address space, only for their related
  rendering contexts to share an address space.
If the GL version is 1.1 or greater, then all texture objects except
  object 0 are shared by any contexts that share display lists.

## Notes
[`XVisualInfo`] is defined in *Xutil.h.* It is a structure that
  includes *visual*, *visualID*, *screen*, and *depth* elements.
A *process* is a single execution environment, implemented in a single
  address space, consisting of one or more threads.
A *thread* is one of a set of subprocesses that share a single address
  space, but maintain separate program counters, stack spaces, and other
  related global data. A *thread* that is the only member of its
  subprocess group is equivalent to a *process*.
It may not be possible to render to a GLX pixmap with a direct
  rendering context.

## Errors
- [`NULL`] is returned if execution fails on the client side.
- [`BadMatch`] is generated if the context to be created would not share
  the address space or the screen of the context specified by
  `shareList`.
- [`BadValue`] is generated if `vis` is not a valid visual (for example,
  if a particular GLX implementation does not support it).
- [`GLXBadContext`] is generated if `shareList` is not a GLX context and
  is not [`NULL`].
- [`BadAlloc`] is generated if the server does not have enough resources
  to allocate the new context.

## See Also
- [`Gl::x_destroy_context`]
- [`Gl::x_get_config`]
- [`Gl::x_is_direct`]
- [`Gl::x_make_current`]
