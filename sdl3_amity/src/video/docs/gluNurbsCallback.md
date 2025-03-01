# gluNurbsCallback
define a callback for a NURBS object

## Parameters
- `nurb`
  Specifies the NURBS object (created with
  [`Gl::u_new_nurbs_renderer`]).

## Description
[`Gl::u_nurbs_callback`] is used to define a callback to be used by a
  NURBS object. If the specified callback is already defined, then it is
  replaced. If `CallBackFunc` is NULL, then this callback will not get
  invoked and the related data, if any, will be lost.
Except the error callback, these callbacks are used by NURBS
  tessellator (when [`GLU_NURBS_MODE`] is set to be
  [`GLU_NURBS_TESSELLATOR`]) to return back the OpenGL polygon
  primitives resulting from the tessellation. Note that there are two
  versions of each callback: one with a user data pointer and one
  without. If both versions for a particular callback are specified then
  the callback with the user data pointer will be used. Note that
  ``userData'' is a copy of the pointer that was specified at the last
  call to [`Gl::u_nurbs_callback_data`].
The error callback function is effective no matter which value that
  [`GLU_NURBS_MODE`] is set to. All other callback functions are
  effective only when [`GLU_NURBS_MODE`] is set to
  [`GLU_NURBS_TESSELLATOR`].
The legal callbacks are as follows:

## Notes
[`Gl::u_nurbs_callback`] is available only if the GLU version is 1.2
  or greater.
GLU version 1.2 supports only the [`GLU_ERROR`] parameter for `which`.
  The [`GLU_ERROR`] value is deprecated in GLU version 1.3 in favor of
  [`GLU_NURBS_ERROR`]. All other accepted values for `CallBackFunc` are
  available only if the GLU version is 1.3 or greater.

## See Also
- [`Gl::u_error_string`]
- [`Gl::u_new_nurbs_renderer`]
- [`Gl::u_nurbs_property`]
