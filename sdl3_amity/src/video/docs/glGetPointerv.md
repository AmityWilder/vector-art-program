# glGetPointerv
return the address of the specified pointer

## Parameters
- `pname`
  Specifies the pointer to be returned. Must be one of
  [`gl::DEBUG_CALLBACK_FUNCTION`] or [`gl::DEBUG_CALLBACK_USER_PARAM`].

## Description
[`Gl::get_pointerv`] returns pointer information. `pname` indicates
  the pointer to be returned, and `params` is a pointer to a location in
  which to place the returned data. The parameters that may be queried
  include:

## Notes
[`Gl::get_pointerv`] is available in the OpenGL core profile only if
  the GL version is 4.3 or later. It is available in the compatibility
  profile for all GL versions, and accepts additional queries. However,
  these reference pages document only the core profile.

## Errors
- [`gl::INVALID_ENUM`] is generated if `pname` is not an accepted value.

## See Also
- [`Gl::debug_message_callback`]
