# glCreateVertexArrays
create vertex array objects

## Parameters
- `n`
  Number of vertex array objects to create.

## Description
[`Gl::create_vertex_arrays`] returns `n` previously unused vertex
  array object names in `arrays`, each representing a new vertex array
  object initialized to the default state.

## Errors
- [`gl::INVALID_VALUE`] is generated if `n` is negative.

## See Also
- [`Gl::bind_vertex_array`]
- [`Gl::delete_vertex_arrays`]
- [`Gl::enable_vertex_attrib_array`]
- [`Gl::gen_vertex_arrays`]
- [`Gl::is_vertex_array`]
- [`Gl::vertex_attrib_pointer`]
