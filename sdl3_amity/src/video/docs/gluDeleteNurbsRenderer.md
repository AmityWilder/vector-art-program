# gluDeleteNurbsRenderer
destroy a NURBS object

## Parameters
- `nurb`
  Specifies the NURBS object to be destroyed.

## Description
[`Gl::u_delete_nurbs_renderer`] destroys the NURBS object (which was
  created with [`Gl::u_new_nurbs_renderer`]) and frees any memory it
  uses. Once [`Gl::u_delete_nurbs_renderer`] has been called, `nurb`
  cannot be used again.

## See Also
- [`Gl::u_new_nurbs_renderer`]
