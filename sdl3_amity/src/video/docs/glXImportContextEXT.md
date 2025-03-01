# glXImportContextEXT
import another process's indirect rendering context.

## Parameters
- `dpy`
  Specifies the connection to the X server.

## Description
[`Gl::x_import_context_ext`] creates a GLXContext given the XID of an
  existing GLXContext. It may be used in place of
  [`Gl::x_create_context`], to share another process's indirect
  rendering context.
Only the server-side context information can be shared between X
  clients; client-side state, such as pixel storage modes, cannot be
  shared. Thus, [`Gl::x_import_context_ext`] must allocate memory to
  store client-side information. This memory is freed by calling
  [`Gl::x_free_context_ext`].
This call does not create a new XID. It merely makes an existing
  object available to the importing client (Display *). Like any XID, it
  goes away when the creating client drops its connection or the ID is
  explicitly deleted. Note that this is when the XID goes away. The
  object goes away when the XID goes away AND the context is not current
  to any thread.
If `contextID` refers to a direct rendering context then no error is
  generated but [`Gl::x_import_context_ext`] returns NULL.
[`Gl::x_import_context_ext`] is part of the ```c
  GLX_EXT_import_context ``` extension, not part of the core GLX command
  set. If ```c GLX_EXT_import_context ``` is included in the string
  returned by [`Gl::x_query_extensions_string`], the extension is
  supported.

## Errors
- [`GLXBadContext`] is generated if `contextID` does not refer to a
  valid context.

## See Also
- [`Gl::x_create_context`]
- [`Gl::x_query_version`]
- [`Gl::x_query_extensions_string`]
- [`Gl::x_get_context_idext`]
- [`Gl::x_free_context_ext`]
