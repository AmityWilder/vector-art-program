# glIsVertexArray
determine if a name corresponds to a vertex array object

## Parameters
- `array`
  Specifies a value that may be the name of a vertex array object.

## Description
[`Gl::is_vertex_array`] returns [`gl::TRUE`] if `array` is currently
  the name of a vertex array object. If `array` is zero, or if `array`
  is not the name of a vertex array object, or if an error occurs,
  [`Gl::is_vertex_array`] returns [`gl::FALSE`]. If `array` is a name
  returned by [`Gl::gen_vertex_arrays`], by that has not yet been bound
  through a call to [`Gl::bind_vertex_array`], then the name is not a
  vertex array object and [`Gl::is_vertex_array`] returns [`gl::FALSE`].

## See Also
- [`Gl::gen_vertex_arrays`]
- [`Gl::bind_vertex_array`]
- [`Gl::delete_vertex_arrays`]
