# gluNurbsCallbackData
set a user data pointer

## Parameters
- `nurb`
  Specifies the NURBS object (created with
  [`Gl::u_new_nurbs_renderer`]).

## Description
[`Gl::u_nurbs_callback_data`] is used to pass a pointer to the
  application's data to NURBS tessellator. A copy of this pointer will
  be passed by the tessellator in the NURBS callback functions (set by
  [`Gl::u_nurbs_callback`]).

## Notes
[`Gl::u_nurbs_callback_data`] is available only if the GLU version is
  1.3 or greater.

## See Also
- [`Gl::u_new_nurbs_renderer`]
- [`Gl::u_nurbs_callback`]
