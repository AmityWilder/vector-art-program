# gl_PrimitiveID
contains the index of the current primitive

## Description
[`Gl::primitive_id`] is a tessellation control, tessellation
  evaluation and fragment language input variable. For the tessellation
  control and tessellation evaluation languages, it holds the number of
  primitives processed by the shader since the current set of rendering
  primitives was started. The first primitive processed by the drawing
  command is numbered zero and the primitive ID counter is incremented
  after every individual point, line or triangle primitive is processed.
  For triangles drawn in point or line mode, the primitive ID counter is
  incremented only once, even through multiple points or lines may
  actually be drawn. Restarting a primitive topology using the primitive
  restart index has no effect on the primitive ID counter.
In the geometry language, [`Gl::primitive_id`] is an output variable
  that is passed to the corresponding [`Gl::primitive_id`] input
  variable in the fragment shader. If no geomery shader is present then
  [`Gl::primitive_id`] in the fragment language behaves identically as
  it would in the tessellation control and evaluation languages. If a
  geometry shader is present but does not write to [`Gl::primitive_id`],
  the value of [`Gl::primitive_id`] in the fragment shader is undefined.

## See Also
- [`Gl::instance_id`]
- [`Gl::vertex_id`]
- [`Gl::primitive_id_in`]
