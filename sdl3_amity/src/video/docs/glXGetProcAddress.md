# glXGetProcAddress
obtain a pointer to an OpenGL or GLX function

## Parameters
- `procName`
  Specifies the name of the OpenGL or GLX function whose address is to
  be returned.

## Description
[`Gl::x_get_proc_address`] returns the address of the function
  specified in `procName`. This is necessary in environments where the
  OpenGL link library exports a different set of functions than the
  runtime library.

## Notes
A [`NULL`] pointer is returned if function requested is not suported
  in the implementation being queried.
GLU functions are not queryable due to the fact that the library may
  not be loaded at the time of the query.
