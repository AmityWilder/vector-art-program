# glXFreeContextEXT
free client-side memory for imported context

## Parameters
- `dpy`
  Specifies the connection to the X server.

## Description
[`Gl::x_free_context_ext`] frees the client-side part of a GLXContext
  that was created with [`Gl::x_import_context_ext`].
  [`Gl::x_free_context_ext`] does not free the server-side context
  information or the XID associated with the server-side context.
[`Gl::x_free_context_ext`] is part of the ```c GLX_EXT_import_context
  ``` extension, not part of the core GLX command set. If ```c
  GLX_EXT_import_context ``` is included in the string returned by
  [`Gl::x_query_extensions_string`], the extension is supported.

## Errors
- [`GLXBadContext`] is generated if `ctx` does not refer to a valid
  context.

## See Also
- [`Gl::x_create_context`]
- [`Gl::x_query_version`]
- [`Gl::x_query_extensions_string`]
- [`Gl::x_import_context_ext`]
