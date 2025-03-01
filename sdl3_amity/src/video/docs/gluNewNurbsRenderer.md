# gluNewNurbsRenderer
create a NURBS object

## Description
[`Gl::u_new_nurbs_renderer`] creates and returns a pointer to a new
  NURBS object. This object must be referred to when calling NURBS
  rendering and control functions. A return value of 0 means that there
  is not enough memory to allocate the object.

## See Also
- [`Gl::u_begin_curve`]
- [`Gl::u_begin_surface`]
- [`Gl::u_begin_trim`]
- [`Gl::u_delete_nurbs_renderer`]
- [`Gl::u_nurbs_callback`]
- [`Gl::u_nurbs_property`]
