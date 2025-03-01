# glXQueryServerString
return string describing the server

## Parameters
- `dpy`
  Specifies the connection to the X server.

## Description
[`Gl::x_query_server_string`] returns a pointer to a static, null-
  terminated string describing some aspect of the server's GLX
  extension. The possible values for `name` and the format of the
  strings is the same as for [`Gl::x_get_client_string`]. If `name` is
  not set to a recognized value, [`NULL`] is returned.

## Notes
[`Gl::x_query_server_string`] is available only if the GLX version is
  1.1 or greater.
If the GLX version is 1.1 or 1.0, the GL version must be 1.0. If the
  GLX version is 1.2, the GL version must be 1.1. If the GLX version is
  1.3, the GL version must be 1.2.
[`Gl::x_query_server_string`] only returns information about GLX
  extensions supported by the server. Call [`Gl::get_string`] to get a
  list of GL extensions. Call [`Gl::x_get_client_string`] to get a list
  of GLX extensions supported by the client.

## See Also
- [`Gl::x_query_version`]
- [`Gl::x_get_client_string`]
- [`Gl::x_query_extensions_string`]
