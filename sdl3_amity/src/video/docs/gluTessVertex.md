# gluTessVertex
specify a vertex on a polygon

## Parameters
- `tess`
  Specifies the tessellation object (created with [`Gl::u_new_tess`]).

## Description
[`Gl::u_tess_vertex`] describes a vertex on a polygon that the program
  defines. Successive [`Gl::u_tess_vertex`] calls describe a closed
  contour. For example, to describe a quadrilateral,
  [`Gl::u_tess_vertex`] should be called four times.
  [`Gl::u_tess_vertex`] can only be called between
  [`Gl::u_tess_begin_contour`] and [`Gl::u_tess_end_contour`].
`data` normally points to a structure containing the vertex location,
  as well as other per-vertex attributes such as color and normal. This
  pointer is passed back to the user through the [`GLU_TESS_VERTEX`] or
  [`GLU_TESS_VERTEX_DATA`] callback after tessellation (see the
  [`Gl::u_tess_callback`] reference page).

## Example
A quadrilateral with a triangular hole in it can be described as
  follows: ```c gluTessBeginPolygon(tobj, NULL);
  gluTessBeginContour(tobj); gluTessVertex(tobj, v1, v1);
  gluTessVertex(tobj, v2, v2); gluTessVertex(tobj, v3, v3);
  gluTessVertex(tobj, v4, v4); gluTessEndContour(tobj);
  gluTessBeginContour(tobj); gluTessVertex(tobj, v5, v5);
  gluTessVertex(tobj, v6, v6); gluTessVertex(tobj, v7, v7);
  gluTessEndContour(tobj); gluTessEndPolygon(tobj); ```

## Notes
It is a common error to use a local variable for `location` or `data`
  and store values into it as part of a loop. For example: ```c for (i =
  0; i < NVERTICES; ++i) { GLdouble data[3]; data[0] = vertex[i][0];
  data[1] = vertex[i][1]; data[2] = vertex[i][2]; gluTessVertex(tobj,
  data, data); } ```
This doesn't work. Because the pointers specified by `location` and
  `data` might not be dereferenced until [`Gl::u_tess_end_polygon`] is
  executed, all the vertex coordinates but the very last set could be
  overwritten before tessellation begins.
Two common symptoms of this problem are when the data consists of a
  single point (when a local variable is used for `data`) and a
  [`GLU_TESS_NEED_COMBINE_CALLBACK`] error (when a local variable is
  used for `location`).

## See Also
- [`Gl::u_new_tess`]
- [`Gl::u_tess_begin_contour`]
- [`Gl::u_tess_begin_polygon`]
- [`Gl::u_tess_callback`]
- [`Gl::u_tess_end_polygon`]
- [`Gl::u_tess_normal`]
- [`Gl::u_tess_property`]
