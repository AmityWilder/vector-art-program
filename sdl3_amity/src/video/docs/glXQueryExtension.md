# glXQueryExtension
indicate whether the GLX extension is supported

## Parameters
- `dpy`
  Specifies the connection to the X server.

## Description
[`Gl::x_query_extension`] returns [`True`] if the X server of
  connection `dpy` supports the GLX extension, [`False`] otherwise. If
  [`True`] is returned, then `errorBase` and `eventBase` return the
  error base and event base of the GLX extension. These values should be
  added to the constant error and event values to determine the actual
  event or error values. Otherwise, `errorBase` and `eventBase` are
  unchanged.
`errorBase` and `eventBase` do not return values if they are specified
  as [`NULL`].

## See Also
- [`Gl::x_query_version`]
