# gluErrorString
produce an error string from a GL or GLU error code

## Parameters
- `error`
  Specifies a GL or GLU error code.

## Description
[`Gl::u_error_string`] produces an error string from a GL or GLU error
  code. The string is in ISO Latin 1 format. For example,
  [`Gl::u_error_string`]([`GLU_OUT_OF_MEMORY`]) returns the string *out
  of memory*.
The standard GLU error codes are [`GLU_INVALID_ENUM`],
  [`GLU_INVALID_VALUE`], and [`GLU_OUT_OF_MEMORY`]. Certain other GLU
  functions can return specialized error codes through callbacks. See
  the [`Gl::get_error`] reference page for the list of GL error codes.

## Errors
- [`NULL`] is returned if `error` is not a valid GL or GLU error code.

## See Also
- [`Gl::u_nurbs_callback`]
- [`Gl::u_quadric_callback`]
- [`Gl::u_tess_callback`]
- [`Gl::get_error`]
