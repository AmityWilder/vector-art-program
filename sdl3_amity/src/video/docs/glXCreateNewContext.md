# glXCreateNewContext
create a new GLX rendering context

## Parameters
- `dpy`
  Specifies the connection to the X server.

## Description
[`Gl::x_create_new_context`] creates a GLX rendering context and
  returns its handle. This context can be used to render into GLX
  windows, pixmaps, or pixel buffers. If [`Gl::x_create_new_context`]
  fails to create a rendering context, [`NULL`] is returned.
If `render_type` is [`GLX_RGBA_TYPE`], then a context that supports
  RGBA rendering is created. If `config` is [`GLX_COLOR_INDEX_TYPE`],
  then context supporting color-index rendering is created.
If `render_type` is not [`NULL`], then all display-list indexes and
  definitions are shared by context `render_type` and by the newly
  created context. An arbitrary number of contexts can share a single
  display-list space. However, all rendering contexts that share a
  single display-list space must themselves exist in the same address
  space. Two rendering contexts share an address space if both are
  nondirect using the same server, or if both are direct and owned by a
  single process. Note that in the nondirect case, it is not necessary
  for the calling threads to share an address space, only for their
  related rendering contexts to share an address space.
If `share_list` is [`True`], then a direct-rendering context is
  created if the implementation supports direct rendering, if the
  connection is to an X server that is local, and if a direct-rendering
  context is available. (An implementation may return an indirect
  context when `share_list` is [`True`].) If `share_list` is [`False`],
  then a rendering context that renders through the X server is always
  created. Direct rendering provides a performance advantage in some
  implementations. However, direct-rendering contexts cannot be shared
  outside a single process, and they may be unable to render to GLX
  pixmaps.

## Notes
[`Gl::x_create_new_context`] is available only if the GLX version is
  1.3 or greater.
If the GLX version is 1.1 or 1.0, the GL version must be 1.0. If the
  GLX version is 1.2, then the GL version must be 1.1. If the GLX
  version is 1.3, then the GL version must be 1.2.

## Errors
- [`NULL`] is returned if execution fails on the client side.
- [`GLXBadContext`] is generated if `render_type` is not a GLX context
  and is not [`NULL`].
- [`GLXBadFBConfig`] is generated if `config` is not a valid
  GLXFBConfig.
- [`BadMatch`] is generated if the context to be created would not share
  the address space or the screen of the context specified by
  `render_type`.
- [`BadAlloc`] is generated if the server does not have enough resources
  to allocate the new context.
- [`BadValue`] is generated if `config` is not a valid visual (for
  example, if a particular GLX implementation does not support it).

## See Also
- [`Gl::x_choose_fb_config`]
- [`Gl::x_create_context`]
- [`Gl::x_destroy_context`]
- [`Gl::x_get_fb_configs`]
- [`Gl::x_get_fb_config_attrib`]
- [`Gl::x_is_direct`]
- [`Gl::x_make_context_current`]
