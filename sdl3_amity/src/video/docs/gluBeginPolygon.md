# gluBeginPolygon
delimit a polygon description

## Parameters
- `tess`
  Specifies the tessellation object (created with [`Gl::u_new_tess`]).

## Description
[`Gl::u_begin_polygon`] and [`Gl::u_end_polygon`] delimit the
  definition of a nonconvex polygon. To define such a polygon, first
  call [`Gl::u_begin_polygon`]. Then define the contours of the polygon
  by calling [`Gl::u_tess_vertex`] for each vertex and
  [`Gl::u_next_contour`] to start each new contour. Finally, call
  [`Gl::u_end_polygon`] to signal the end of the definition. See the
  [`Gl::u_tess_vertex`] and [`Gl::u_next_contour`] reference pages for
  more details.
Once [`Gl::u_end_polygon`] is called, the polygon is tessellated, and
  the resulting triangles are described through callbacks. See
  [`Gl::u_tess_callback`] for descriptions of the callback functions.

## Notes
This command is obsolete and is provided for backward compatibility
  only. Calls to [`Gl::u_begin_polygon`] are mapped to
  [`Gl::u_tess_begin_polygon`] followed by [`Gl::u_tess_begin_contour`].
  Calls to [`Gl::u_end_polygon`] are mapped to
  [`Gl::u_tess_end_contour`] followed by [`Gl::u_tess_end_polygon`].

## Example
A quadrilateral with a triangular hole in it can be described like
  this: ```c gluBeginPolygon(tobj); gluTessVertex(tobj, v1, v1);
  gluTessVertex(tobj, v2, v2); gluTessVertex(tobj, v3, v3);
  gluTessVertex(tobj, v4, v4); gluNextContour(tobj, GLU_INTERIOR);
  gluTessVertex(tobj, v5, v5); gluTessVertex(tobj, v6, v6);
  gluTessVertex(tobj, v7, v7); gluEndPolygon(tobj); ```

## See Also
- [`Gl::u_new_tess`]
- [`Gl::u_next_contour`]
- [`Gl::u_tess_begin_contour`]
- [`Gl::u_tess_begin_polygon`]
- [`Gl::u_tess_callback`]
- [`Gl::u_tess_vertex`]
