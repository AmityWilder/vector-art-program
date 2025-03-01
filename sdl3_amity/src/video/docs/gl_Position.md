# gl_Position
contains the position of the current vertex

## Description
In the vertex, tessellation evaluation and geometry languages, a
  single global instance of the [`Gl::per_vertex`] named block is
  available and its [`Gl::position`] member is an output that receives
  the homogeneous vertex position. It may be written at any time during
  shader execution. The value written to [`Gl::position`] will be used
  by primitive assembly, clipping, culling and other fixed functionality
  operations, if present, that operate on primitives after vertex
  processing has occurred.
In the tessellation control language, the [`Gl::per_vertex`] named
  block is used to construct an array, [`Gl::out[]`], whose
  [`Gl::position`] members hold the homogeneous control point position,
  which become available as inputs to the subsequent tessellation
  evaluation shader.
The value of [`Gl::position`] (or the [`Gl::position`] member of the
  [`Gl::out[]`] array, in the case of the tessellation control shader)
  is undefined after the vertex, tessellation control, and tessellation
  evaluation shading stages if the corresponding shader executable does
  not write to gl_Position. It is also undefined after the geometry
  processing stage if the geometry shader executable calls
  [`Gl::emit_vertex`] without having written [`Gl::position`] since the
  last call to [`Gl::emit_vertex`] (or hasn't written it at all).
In the tessellation control, tessellation evaluation and geometry
  languages, the [`Gl::per_vertex`] named block is used to construct an
  array, [`Gl::in[]`] of per-vertex or per-control point inputs whose
  content represents the corresponding outputs written by the previous
  stage.

## See Also
- [`Gl::point_size`]
- [`Gl::clip_distance`]
