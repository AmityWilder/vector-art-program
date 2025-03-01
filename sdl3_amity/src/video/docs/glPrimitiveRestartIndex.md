# glPrimitiveRestartIndex
specify the primitive restart index

## Parameters
- `index`
  Specifies the value to be interpreted as the primitive restart index.

## Description
[`Gl::primitive_restart_index`] specifies a vertex array element that
  is treated specially when primitive restarting is enabled. This is
  known as the primitive restart index.
When one of the [`Gl::draw*`] commands transfers a set of generic
  attribute array elements to the GL, if the index within the vertex
  arrays corresponding to that set is equal to the primitive restart
  index, then the GL does not process those elements as a vertex.
  Instead, it is as if the drawing command ended with the immediately
  preceding transfer, and another drawing command is immediately started
  with the same parameters, but only transferring the immediately
  following element through the end of the originally specified
  elements.
When either [`Gl::draw_elements_base_vertex`],
  [`Gl::draw_elements_instanced_base_vertex`] or
  [`Gl::multi_draw_elements_base_vertex`] is used, the primitive restart
  comparison occurs before the basevertex offset is added to the array
  index.

## Notes
[`Gl::primitive_restart_index`] is available only if the GL version is
  3.1 or greater.

## See Also
- [`Gl::draw_arrays`]
- [`Gl::draw_elements`]
- [`Gl::draw_elements_base_vertex`]
- [`Gl::draw_elements_instanced_base_vertex`]
