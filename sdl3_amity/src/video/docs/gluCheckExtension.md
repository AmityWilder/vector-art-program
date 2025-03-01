# gluCheckExtension
determines if an extension name is supported

## Parameters
- `extName`
  Specifies an extension name.

## Description
[`Gl::u_check_extension`] returns [`GLU_TRUE`] if `extName` is
  supported otherwise [`GLU_FALSE`] is returned.
This is used to check for the presence for OpenGL, GLU, or GLX
  extension names by passing the extension strings returned by
  [`Gl::get_string`], [`Gl::u_get_string`], [`Gl::x_get_client_string`],
  [`Gl::x_query_extensions_string`], or [`Gl::x_query_server_string`],
  respectively, as `extString`.

## Notes
Cases where one extension name is a substring of another are correctly
  handled.
There may or may not be leading or trailing blanks in `extString`.
Extension names should not contain embedded spaces.
All strings are null-terminated.

## See Also
- [`Gl::u_get_string`]
- [`Gl::get_string`]
- [`Gl::x_get_client_string`]
- [`Gl::x_query_extensions_string`]
- [`Gl::x_query_server_string`]
