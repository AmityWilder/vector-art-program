# glXGetContextIDEXT
get the XID for a context.

## Parameters
- `ctx`
  Specifies a GLX rendering context.

## Description
[`Gl::x_get_context_idext`] returns the XID associated with a
  GLXContext.
No round trip is forced to the server; unlike most X calls that return
  a value, [`Gl::x_get_context_idext`] does not flush any pending
  events.
[`Gl::x_get_context_idext`] is part of the ```c GLX_EXT_import_context
  ``` extension, not part of the core GLX command set. If ```c
  GLX_EXT_import_context ``` is included in the string returned by
  [`Gl::x_query_extensions_string`], the extension is supported.

## Errors
- [`None`] is returned if `ctx` is [`NULL`]. Otherwise, if `ctx` does
  not refer to a valid context, undefined behavior results.

## See Also
- [`Gl::x_create_context`]
- [`Gl::x_query_version`]
- [`Gl::x_query_extensions_string`]
