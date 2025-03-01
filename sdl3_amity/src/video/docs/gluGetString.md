# gluGetString
return a string describing the GLU version or GLU extensions

## Parameters
- `name`
  Specifies a symbolic constant, one of [`GLU_VERSION`], or
  [`GLU_EXTENSIONS`].

## Description
[`Gl::u_get_string`] returns a pointer to a static string describing
  the GLU version or the GLU extensions that are supported.
The version number is one of the following forms:
*major_number.minor_number*
  *major_number.minor_number.release_number*.
The version string is of the following form:
*version number<space>vendor-specific information*
Vendor-specific information is optional. Its format and contents
  depend on the implementation.
The standard GLU contains a basic set of features and capabilities. If
  a company or group of companies wish to support other features, these
  may be included as extensions to the GLU. If `name` is
  [`GLU_EXTENSIONS`], then [`Gl::u_get_string`] returns a space-
  separated list of names of supported GLU extensions. (Extension names
  never contain spaces.)
All strings are null-terminated.

## Notes
[`Gl::u_get_string`] only returns information about GLU extensions.
  Call [`Gl::get_string`] to get a list of GL extensions.
[`Gl::u_get_string`] is an initialization routine. Calling it after a
  [`Gl::new_list`] results in undefined behavior.

## Errors
- NULL is returned if `name` is not [`GLU_VERSION`] or
  [`GLU_EXTENSIONS`].
- 

## See Also
- [`Gl::get_string`]
