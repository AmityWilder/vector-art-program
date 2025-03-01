# gluNurbsCallbackDataEXT
set a user data pointer

## Parameters
- `nurb`
  Specifies the NURBS object (created with
  [`Gl::u_new_nurbs_renderer`]).

## Description
[`Gl::u_nurbs_callback_data_ext`] is used to pass a pointer to the
  application's data to NURBS tessellator. A copy of this pointer will
  be passed by the tessellator in the NURBS callback functions (set by
  [`Gl::u_nurbs_callback`]).

## See Also
- [`Gl::u_nurbs_callback`]
