# EmitVertex
emit a vertex to the first vertex stream

## Description
*Available only in the Geometry Shader*, [`Gl::emit_vertex`] emits the
  current values of output variables to the current output primitive on
  the first (and possibly only) primitive stream. It is equivalent to
  calling [`Gl::emit_stream_vertex`] with `stream` set to 0.

## See Also
- [`Gl::emit_stream_vertex`]
- [`Gl::end_stream_primitive`]
- [`Gl::end_primitive`]
