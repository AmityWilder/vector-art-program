# gl_PrimitiveIDIn
contains the index of the current primitive

## Description
[`Gl::primitive_id_in`] is a geometry language input variable that
  holds the number of primitives processed by the shader since the
  current set of rendering primitives was started. The first primitive
  processed by the drawing command is numbered zero and the primitive ID
  counter is incremented after every individual point, line or triangle
  primitive is processed. For triangles drawn in point or line mode, the
  primitive ID counter is incremented only once, even through multiple
  points or lines may actually be drawn. Restarting a primitive topology
  using the primitive restart index has no effect on the primitive ID
  counter.

## See Also
- [`Gl::instance_id`]
