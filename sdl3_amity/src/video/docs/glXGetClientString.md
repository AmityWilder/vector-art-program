# glXGetClientString
return a string describing the client

## Parameters
- `dpy`
  Specifies the connection to the X server.

## Description
[`Gl::x_get_client_string`] returns a string describing some aspect of
  the client library. The possible values for `name` are [`GLX_VENDOR`],
  [`GLX_VERSION`], and [`GLX_EXTENSIONS`]. If `name` is not set to one
  of these values, [`Gl::x_get_client_string`] returns [`NULL`]. The
  format and contents of the vendor string is implementation dependent.
The extensions string is null-terminated and contains a space-
  separated list of extension names. (The extension names never contain
  spaces.) If there are no extensions to GLX, then the empty string is
  returned.
The version string is laid out as follows:
<major_version.minor_version><space><vendor-specific info>
Both the major and minor portions of the version number are of
  arbitrary length. The vendor-specific information is optional.
  However, if it is present, the format and contents are implementation
  specific.

## Notes
[`Gl::x_get_client_string`] is available only if the GLX version is
  1.1 or greater.
If the GLX version is 1.1 or 1.0, the GL version must be 1.0. If the
  GLX version is 1.2, then the GL version must be 1.1. If the GLX
  version is 1.3, then the GL version must be 1.2.
[`Gl::x_get_client_string`] only returns information about GLX
  extensions supported by the client. Call [`Gl::get_string`] to get a
  list of GL extensions supported by the server.

## See Also
- [`Gl::x_query_version`]
- [`Gl::x_query_extensions_string`]
- [`Gl::x_query_server_string`]
