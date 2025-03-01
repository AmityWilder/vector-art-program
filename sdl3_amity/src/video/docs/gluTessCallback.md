# gluTessCallback
define a callback for a tessellation object

## Parameters
- `tess`
  Specifies the tessellation object (created with [`Gl::u_new_tess`]).

## Description
[`Gl::u_tess_callback`] is used to indicate a callback to be used by a
  tessellation object. If the specified callback is already defined,
  then it is replaced. If `CallBackFunc` is NULL, then the existing
  callback becomes undefined.
These callbacks are used by the tessellation object to describe how a
  polygon specified by the user is broken into triangles. Note that
  there are two versions of each callback: one with user-specified
  polygon data and one without. If both versions of a particular
  callback are specified, then the callback with user-specified polygon
  data will be used. Note that the *polygon_data* parameter used by some
  of the functions is a copy of the pointer that was specified when
  [`Gl::u_tess_begin_polygon`] was called. The legal callbacks are as
  follows:

## Example
Polygons tessellated can be rendered directly like this: ```c
  gluTessCallback(tobj, GLU_TESS_BEGIN, glBegin); gluTessCallback(tobj,
  GLU_TESS_VERTEX, glVertex3dv); gluTessCallback(tobj, GLU_TESS_END,
  glEnd); gluTessCallback(tobj, GLU_TESS_COMBINE, myCombine);
  gluTessBeginPolygon(tobj, NULL); gluTessBeginContour(tobj);
  gluTessVertex(tobj, v, v); ... gluTessEndContour(tobj);
  gluTessEndPolygon(tobj); ``` Typically, the tessellated polygon should
  be stored in a display list so that it does not need to be
  retessellated every time it is rendered.

## See Also
- [`Gl::u_error_string`]
- [`Gl::u_new_tess`]
- [`Gl::u_tess_begin_contour`]
- [`Gl::u_tess_begin_polygon`]
- [`Gl::u_tess_normal`]
- [`Gl::u_tess_property`]
- [`Gl::u_tess_vertex`]
- [`Gl::begin`]
- [`Gl::edge_flag`]
- [`Gl::vertex`]
