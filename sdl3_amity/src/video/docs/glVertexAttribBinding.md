# glVertexAttribBinding
associate a vertex attribute and a vertex buffer binding for a vertex
  array object

## Parameters
- `vaobj`
  Specifies the name of the vertex array object for
  [`Gl::vertex_array_attrib_binding`].

## Description
[`Gl::vertex_attrib_binding`] and [`Gl::vertex_array_attrib_binding`]
  establishes an association between the generic vertex attribute of a
  vertex array object whose index is given by `attribindex`, and a
  vertex buffer binding whose index is given by `bindingindex`. For
  [`Gl::vertex_attrib_binding`], the vertex array object affected is
  that currently bound. For [`Gl::vertex_array_attrib_binding`], `vaobj`
  is the name of the vertex array object.
`attribindex` must be less than the value of
  [`gl::MAX_VERTEX_ATTRIBS`] and `bindingindex` must be less than the
  value of [`gl::MAX_VERTEX_ATTRIB_BINDINGS`].

## Errors
- [`gl::INVALID_OPERATION`] is generated by
  [`Gl::vertex_attrib_binding`] if no vertex array object is bound.
- [`gl::INVALID_OPERATION`] is generated by
  [`Gl::vertex_array_attrib_binding`] if `vaobj` is not the name of an
  existing vertex array object.
- [`gl::INVALID_VALUE`] is generated if `attribindex` is greater than or
  equal to the value of [`gl::MAX_VERTEX_ATTRIBS`].
- [`gl::INVALID_VALUE`] is generated if `bindingindex` is greater than
  or equal to the value of [`gl::MAX_VERTEX_ATTRIB_BINDINGS`].

## See Also
- [`Gl::bind_vertex_buffer`]
- [`Gl::vertex_attrib_format`]
- [`Gl::vertex_binding_divisor`]
- [`Gl::vertex_attrib_pointer`]
