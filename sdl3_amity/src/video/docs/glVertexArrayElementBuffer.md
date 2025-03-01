# glVertexArrayElementBuffer
configures element array buffer binding of a vertex array object

## Parameters
- `vaobj`
  Specifies the name of the vertex array object.

## Description
[`Gl::vertex_array_element_buffer`] binds a buffer object with id
  `buffer` to the element array buffer bind point of a vertex array
  object with id `vaobj`. If `buffer` is zero, any existing element
  array buffer binding to `vaobj` is removed.

## Errors
- [`gl::INVALID_OPERATION`] error is generated if `vaobj` is not the
  name of an existing vertex array object.
- [`gl::INVALID_OPERATION`] error is generated if `buffer` is not zero
  or the name of an existing buffer object.

## See Also
- [`Gl::get`]
- [`Gl::get_vertex_arrayiv`]
