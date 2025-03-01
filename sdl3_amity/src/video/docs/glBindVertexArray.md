# glBindVertexArray
bind a vertex array object

## Parameters
- `array`
  Specifies the name of the vertex array to bind.

## Description
[`Gl::bind_vertex_array`] binds the vertex array object with name
  `array`. `array` is the name of a vertex array object previously
  returned from a call to [`Gl::gen_vertex_arrays`], or zero to break
  the existing vertex array object binding.
If no vertex array object with name `array` exists, one is created
  when `array` is first bound. If the bind is successful no change is
  made to the state of the vertex array object, and any previous vertex
  array object binding is broken.

## Errors
- [`gl::INVALID_OPERATION`] is generated if `array` is not zero or the
  name of a vertex array object previously returned from a call to
  [`Gl::gen_vertex_arrays`].

## See Also
- [`Gl::delete_vertex_arrays`]
- [`Gl::enable_vertex_attrib_array`]
- [`Gl::gen_vertex_arrays`]
- [`Gl::is_vertex_array`]
- [`Gl::vertex_attrib_pointer`]
