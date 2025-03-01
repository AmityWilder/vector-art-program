# EndPrimitive
complete the current output primitive on the first vertex stream

## Description
*Available only in the Geometry Shader*, [`Gl::end_primitive`]
  completes the current output primitive on the first (and possibly
  only) vertex stream and starts a new one.No vertex is emitted. Calling
  [`Gl::end_primitive`] is equivalent to calling
  [`Gl::emit_stream_vertex`] with `stream` set to 0.

## See Also
- [`Gl::emit_vertex`]
- [`Gl::emit_stream_vertex`]
- [`Gl::end_stream_primitive`]
