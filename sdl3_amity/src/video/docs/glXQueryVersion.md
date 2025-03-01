# glXQueryVersion
return the version numbers of the GLX extension

## Parameters
- `dpy`
  Specifies the connection to the X server.

## Description
[`Gl::x_query_version`] returns the major and minor version numbers of
  the GLX extension implemented by the server associated with connection
  `dpy`. Implementations with the same major version number are upward
  compatible, meaning that the implementation with the higher minor
  number is a superset of the version with the lower minor number.
`major` and `minor` do not return values if they are specified as
  [`NULL`].

## Errors
- [`Gl::x_query_version`] returns [`False`] if it fails, [`True`]
  otherwise.
- `major` and `minor` are not updated when [`False`] is returned.

## See Also
- [`Gl::x_query_extension`]
