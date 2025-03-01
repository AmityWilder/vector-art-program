# gluNextContour
mark the beginning of another contour

## Parameters
- `tess`
  Specifies the tessellation object (created with [`Gl::u_new_tess`]).

## Description
[`Gl::u_next_contour`] is used in describing polygons with multiple
  contours. After the first contour has been described through a series
  of [`Gl::u_tess_vertex`] calls, a [`Gl::u_next_contour`] call
  indicates that the previous contour is complete and that the next
  contour is about to begin. Another series of [`Gl::u_tess_vertex`]
  calls is then used to describe the new contour. This process can be
  repeated until all contours have been described.
`type` defines what type of contour follows. The legal contour types
  are as follows:
If one contour is of type [`GLU_CCW`] or [`GLU_CW`], then all contours
  must be of the same type (if they are not, then all [`GLU_CCW`] and
  [`GLU_CW`] contours will be changed to [`GLU_UNKNOWN`]).
Note that there is no real difference between the [`GLU_CCW`] and
  [`GLU_CW`] contour types.
Before the first contour is described, [`Gl::u_next_contour`] can be
  called to define the type of the first contour. If
  [`Gl::u_next_contour`] is not called before the first contour, then
  the first contour is marked [`GLU_EXTERIOR`].
This command is obsolete and is provided for backward compatibility
  only. Calls to [`Gl::u_next_contour`] are mapped to
  [`Gl::u_tess_end_contour`] followed by [`Gl::u_tess_begin_contour`].

## Example
A quadrilateral with a triangular hole in it can be described as
  follows: ```c gluBeginPolygon(tobj); gluTessVertex(tobj, v1, v1);
  gluTessVertex(tobj, v2, v2); gluTessVertex(tobj, v3, v3);
  gluTessVertex(tobj, v4, v4); gluNextContour(tobj, GLU_INTERIOR);
  gluTessVertex(tobj, v5, v5); gluTessVertex(tobj, v6, v6);
  gluTessVertex(tobj, v7, v7); gluEndPolygon(tobj); ```

## See Also
- [`Gl::u_begin_polygon`]
- [`Gl::u_new_tess`]
- [`Gl::u_tess_begin_contour`]
- [`Gl::u_tess_callback`]
- [`Gl::u_tess_vertex`]
