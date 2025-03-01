# EmitStreamVertex
emit a vertex to a specified stream

## Parameters
- `stream`
  Specifies the stream upon which the vertex will be emitted.

## Description
*Available only in the Geometry Shader*, [`Gl::emit_stream_vertex`]
  emits the current values of output variables to the current output
  primitive on stream `stream`. The argument `stream` must be a constant
  integral expression. On return from this call, the value of all output
  variables for all streams are undefined.

## See Also
- [`Gl::emit_vertex`]
- [`Gl::end_stream_primitive`]
- [`Gl::end_primitive`]
