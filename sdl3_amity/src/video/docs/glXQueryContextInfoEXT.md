# glXQueryContextInfoEXT
query context information

## Parameters
- `dpy`
  Specifies the connection to the X server.

## Description
[`Gl::x_query_context_info_ext`] sets `value` to the value of
  `attribute` with respect to `ctx`. [`Gl::x_query_context_info_ext`]
  returns an error code if it fails for any reason. Otherwise,
  [`Success`] is returned.
`attribute` may be one of the following:
This call may cause a round-trip to the server.
[`Gl::x_query_context_info_ext`] is part of the ```c
  GLX_EXT_import_context ``` extension, not part of the core GLX command
  set. If ```c GLX_EXT_import_context ``` is included in the string
  returned by [`Gl::x_query_extensions_string`], the extension is
  supported.

## Errors
- [`GLXBadContext`] is generated if `ctx` does not refer to a valid
  context.
- [`GLX_BAD_ATTRIBUTE`] is returned if `attribute` is not a valid GLX
  context attribute.
- fred [`GLX_BAD_CONTEXT`] is returned if `attribute` is not a valid
  context.

## See Also
- [`Gl::x_create_context`]
- [`Gl::x_query_version`]
- [`Gl::x_query_extensions_string`]
