# gl_PointSize
contains size of rasterized points, in pixels

## Description
In the vertex, tessellation evaluation and geometry languages, a
  single global instance of the [`Gl::per_vertex`] named block is
  available and its [`Gl::point_size`] member is an output that receives
  the intended size of the point to be rasterized, in pixels. It may be
  written at any time during shader execution. If
  [`gl::PROGRAM_POINT_SIZE`] is enabled, [`Gl::point_size`] is used to
  determine the size of rasterized points, otherwise it is ignored by
  the rasterization stage.
In the tessellation control language, the [`Gl::per_vertex`] named
  block is used to construct an array, [`Gl::out[]`], whose members
  become available as inputs to the subsequent tessellation evaluation
  shader.
The value of [`Gl::point_size`] (or the [`Gl::point_size`] member of
  the [`Gl::out[]`] array, in the case of the tessellation control
  shader) is undefined after the vertex, tessellation control, and
  tessellation evaluation shading stages if the corresponding shader
  executable does not write to gl_PointSize. It is also undefined after
  the geometry processing stage if the geometry shader executable calls
  [`Gl::emit_vertex`] without having written [`Gl::point_size`] since
  the last call to [`Gl::emit_vertex`] (or hasn't written it at all).
In the tessellation control, tessellation evaluation and geometry
  languages, the [`Gl::per_vertex`] named block is used to construct an
  array, [`Gl::in[]`] of per-vertex or per-control point inputs whose
  content represents the corresponding outputs written by the previous
  stage.

## See Also
- [`Gl::position`]
- [`Gl::clip_distance`]
