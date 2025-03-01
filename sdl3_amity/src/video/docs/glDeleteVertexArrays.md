# glDeleteVertexArrays
delete vertex array objects

## Parameters
- `n`
  Specifies the number of vertex array objects to be deleted.

## Description
[`Gl::delete_vertex_arrays`] deletes `n` vertex array objects whose
  names are stored in the array addressed by `arrays`. Once a vertex
  array object is deleted it has no contents and its name is again
  unused. If a vertex array object that is currently bound is deleted,
  the binding for that object reverts to zero and the default vertex
  array becomes current. Unused names in `arrays` are silently ignored,
  as is the value zero.

## Errors
- [`gl::INVALID_VALUE`] is generated if `n` is negative.

## See Also
- [`Gl::gen_vertex_arrays`]
- [`Gl::is_vertex_array`]
- [`Gl::bind_vertex_array`]
