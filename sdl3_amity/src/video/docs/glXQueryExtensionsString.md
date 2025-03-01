# glXQueryExtensionsString
return list of supported extensions

## Parameters
- `dpy`
  Specifies the connection to the X server.

## Description
[`Gl::x_query_extensions_string`] returns a pointer to a string
  describing which GLX extensions are supported on the connection. The
  string is null-terminated and contains a space-separated list of
  extension names. (The extension names themselves never contain
  spaces.) If there are no extensions to GLX, then the empty string is
  returned.

## Notes
[`Gl::x_query_extensions_string`] is available only if the GLX version
  is 1.1 or greater.
[`Gl::x_query_extensions_string`] only returns information about GLX
  extensions. Call [`Gl::get_string`] to get a list of GL extensions.

## See Also
- [`Gl::get_string`]
- [`Gl::x_query_version`]
- [`Gl::x_query_server_string`]
- [`Gl::x_get_client_string`]
