# gluTessProperty
set a tessellation object property

## Parameters
- `tess`
  Specifies the tessellation object (created with [`Gl::u_new_tess`]).

## Description
[`Gl::u_tess_property`] is used to control properties stored in a
  tessellation object. These properties affect the way that the polygons
  are interpreted and rendered. The legal values for `which` are as
  follows:

## See Also
- [`Gl::u_get_tess_property`]
- [`Gl::u_new_tess`]
