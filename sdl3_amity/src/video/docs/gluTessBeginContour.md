# gluTessBeginContour
delimit a contour description

## Parameters
- `tess`
  Specifies the tessellation object (created with [`Gl::u_new_tess`]).

## Description
[`Gl::u_tess_begin_contour`] and [`Gl::u_tess_end_contour`] delimit
  the definition of a polygon contour. Within each
  [`Gl::u_tess_begin_contour`]/[`Gl::u_tess_end_contour`] pair, there
  can be zero or more calls to [`Gl::u_tess_vertex`]. The vertices
  specify a closed contour (the last vertex of each contour is
  automatically linked to the first). See the [`Gl::u_tess_vertex`]
  reference page for more details. [`Gl::u_tess_begin_contour`] can only
  be called between [`Gl::u_tess_begin_polygon`] and
  [`Gl::u_tess_end_polygon`].

## See Also
- [`Gl::u_new_tess`]
- [`Gl::u_tess_begin_polygon`]
- [`Gl::u_tess_callback`]
- [`Gl::u_tess_end_polygon`]
- [`Gl::u_tess_normal`]
- [`Gl::u_tess_property`]
- [`Gl::u_tess_vertex`]
