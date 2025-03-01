# gluTessNormal
specify a normal for a polygon

## Parameters
- `tess`
  Specifies the tessellation object (created with [`Gl::u_new_tess`]).

## Description
[`Gl::u_tess_normal`] describes a normal for a polygon that the
  program is defining. All input data will be projected onto a plane
  perpendicular to one of the three coordinate axes before tessellation
  and all output triangles will be oriented CCW with respect to the
  normal (CW orientation can be obtained by reversing the sign of the
  supplied normal). For example, if you know that all polygons lie in
  the x-y plane, call [`Gl::u_tess_normal`](tess, 0.0, 0.0, 1.0) before
  rendering any polygons.
If the supplied normal is (0.0, 0.0, 0.0) (the initial value), the
  normal is determined as follows. The direction of the normal, up to
  its sign, is found by fitting a plane to the vertices, without regard
  to how the vertices are connected. It is expected that the input data
  lies approximately in the plane; otherwise, projection perpendicular
  to one of the three coordinate axes may substantially change the
  geometry. The sign of the normal is chosen so that the sum of the
  signed areas of all input contours is nonnegative (where a CCW contour
  has positive area).
The supplied normal persists until it is changed by another call to
  [`Gl::u_tess_normal`].

## See Also
- [`Gl::u_tess_begin_polygon`]
- [`Gl::u_tess_end_polygon`]
