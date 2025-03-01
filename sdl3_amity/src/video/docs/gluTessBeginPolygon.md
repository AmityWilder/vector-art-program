# gluTessBeginPolygon
delimit a polygon description

## Parameters
- `tess`
  Specifies the tessellation object (created with [`Gl::u_new_tess`]).

## Description
[`Gl::u_tess_begin_polygon`] and [`Gl::u_tess_end_polygon`] delimit
  the definition of a convex, concave or self-intersecting polygon.
  Within each [`Gl::u_tess_begin_polygon`]/[`Gl::u_tess_end_polygon`]
  pair, there must be one or more calls to
  [`Gl::u_tess_begin_contour`]/[`Gl::u_tess_end_contour`]. Within each
  contour, there are zero or more calls to [`Gl::u_tess_vertex`]. The
  vertices specify a closed contour (the last vertex of each contour is
  automatically linked to the first). See the [`Gl::u_tess_vertex`],
  [`Gl::u_tess_begin_contour`], and [`Gl::u_tess_end_contour`] reference
  pages for more details.
`data` is a pointer to a user-defined data structure. If the
  appropriate callback(s) are specified (see [`Gl::u_tess_callback`]),
  then this pointer is returned to the callback function(s). Thus, it is
  a convenient way to store per-polygon information.
Once [`Gl::u_tess_end_polygon`] is called, the polygon is tessellated,
  and the resulting triangles are described through callbacks. See
  [`Gl::u_tess_callback`] for descriptions of the callback functions.

## Example
A quadrilateral with a triangular hole in it can be described as
  follows: ```c gluTessBeginPolygon(tobj, NULL);
  gluTessBeginContour(tobj); gluTessVertex(tobj, v1, v1);
  gluTessVertex(tobj, v2, v2); gluTessVertex(tobj, v3, v3);
  gluTessVertex(tobj, v4, v4); gluTessEndContour(tobj);
  gluTessBeginContour(tobj); gluTessVertex(tobj, v5, v5);
  gluTessVertex(tobj, v6, v6); gluTessVertex(tobj, v7, v7);
  gluTessEndContour(tobj); gluTessEndPolygon(tobj); ```


## See Also
- [`Gl::u_new_tess`]
- [`Gl::u_tess_begin_contour`]
- [`Gl::u_tess_callback`]
- [`Gl::u_tess_end_polygon`]
- [`Gl::u_tess_normal`]
- [`Gl::u_tess_property`]
- [`Gl::u_tess_vertex`]
