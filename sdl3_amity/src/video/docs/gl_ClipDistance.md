# gl_ClipDistance
provides a forward-compatible mechanism for vertex clipping

## Description
The [`Gl::clip_distance`] variable provides a forward compatible
  mechanism for controlling user clipping. The element
  `gl_ClipDistance`[*i*] specifies a clip distance for each user clip
  plane *i*. A distance of 0.0 means that the vertex is on the plane, a
  positive distance means that the vertex is inside the clip plane, and
  a negative distance means that the point is outside the clip plane.
  The clip distances will be linearly interpolated across the primitive
  and the portion of the primitive with interpolated distances less than
  0.0 will be clipped.
The [`Gl::clip_distance`] array is initially predeclared as unsized
  and must be sized by the shader either by redeclaring it with an
  explicit size, or by indexing it with only integral constant
  expressions. The array must be sized to include all clip planes that
  are enabled via the OpenGL API; if the size does not include all
  enabled planes, results are undefined. The size may be at most
  [`Gl::max_clip_distances`]. The number of varying components consumed
  by [`Gl::clip_distance`] will match the size of the array, no matter
  how many planes are enabled. The shader must also set all values in
  [`Gl::clip_distance`] that have been enabled via the OpenGL API, or
  results are undefined. Values written into [`Gl::clip_distance`]
  planes that are not enabled have no effect.
In the vertex, tessellation evaluation and geometry languages, a
  single global instance of the [`Gl::per_vertex`] named block is
  available and its [`Gl::clip_distance`] member is an output that
  receives the clip distances for the current vertex. It may be written
  at any time during shader execution. The value written to
  [`Gl::clip_distance`] will be used by primitive assembly, clipping,
  culling and other fixed functionality operations, if present, that
  operate on primitives after vertex processing has occurred.
In the tessellation control language, the [`Gl::per_vertex`] named
  block is used to construct an array, [`Gl::out[]`], whose
  [`Gl::clip_distance`] members hold clip distances for each of the
  control points, which become available as inputs to the subsequent
  tessellation evaluation shader.
The value of [`Gl::clip_distance`] (or the [`Gl::clip_distance`]
  member of the [`Gl::out[]`] array, in the case of the tessellation
  control shader) is undefined after the vertex, tessellation control,
  and tessellation evaluation shading stages if the corresponding shader
  executable does not write to gl_ClipDistance. It is also undefined
  after the geometry processing stage if the geometry shader executable
  calls [`Gl::emit_vertex`] without having written [`Gl::clip_distance`]
  since the last call to [`Gl::emit_vertex`] (or hasn't written it at
  all).
In the tessellation control, tessellation evaluation and geoemetry
  languages, the [`Gl::per_vertex`] named block is used to construct an
  array, [`Gl::in[]`] of per-vertex or per-control point inputs whose
  content represents the corresponding outputs written by the previous
  stage. Only elements of the [`Gl::clip_distance`] array that
  correspond to enabled clip planes have defined values.

## See Also
- [`Gl::cull_distance`]
- [`Gl::point_size`]
