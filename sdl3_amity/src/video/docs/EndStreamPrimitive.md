# EndStreamPrimitive
complete the current output primitive on a specified stream

## Parameters
- `stream`
  Specifies the stream upon which the current primitive will be ended.

## Description
*Available only in the Geometry Shader*, [`Gl::end_stream_primitive`]
  completes the current output primitive on stream `stream` and starts a
  new one. The argument to `stream` must be a constant integral
  expression. No vertex is emitted.

## See Also
- [`Gl::emit_vertex`]
- [`Gl::emit_stream_vertex`]
- [`Gl::end_primitive`]
