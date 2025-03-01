# glXQueryContext
query context information

## Parameters
- `dpy`
  Specifies the connection to the X server.

## Description
[`Gl::x_query_context`] sets `value` to the value of `attribute` with
  respect to `ctx`. `attribute` may be one of the following:
[`Success`] is returned unless `attribute` is not a valid GLX context
  attribute, in which case [`GLX_BAD_ATTRIBUTE`] is returned.
This call may cause a round-trip to the server.

## Notes
[`Gl::x_query_context`] is available only if the GLX version is 1.3 or
  greater.
If the GLX version is 1.1 or 1.0, the GL version must be 1.0. If the
  GLX version is 1.2, then the GL version must be 1.1. If the GLX
  version is 1.3, then the GL version must be 1.2.

## Errors
- [`GLXBadContext`] is generated if `ctx` does not refer to a valid
  context.

## See Also
- [`Gl::x_create_new_context`]
- [`Gl::x_get_current_context`]
- [`Gl::x_query_version`]
- [`Gl::x_query_extensions_string`]
