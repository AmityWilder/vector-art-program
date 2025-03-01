# glGenVertexArrays
generate vertex array object names

## Parameters
- `n`
  Specifies the number of vertex array object names to generate.

## Description
[`Gl::gen_vertex_arrays`] returns `n` vertex array object names in
  `arrays`. There is no guarantee that the names form a contiguous set
  of integers; however, it is guaranteed that none of the returned names
  was in use immediately before the call to [`Gl::gen_vertex_arrays`].
Vertex array object names returned by a call to
  [`Gl::gen_vertex_arrays`] are not returned by subsequent calls, unless
  they are first deleted with [`Gl::delete_vertex_arrays`].
The names returned in `arrays` are marked as used, for the purposes of
  [`Gl::gen_vertex_arrays`] only, but they acquire state and type only
  when they are first bound.

## Errors
- [`gl::INVALID_VALUE`] is generated if `n` is negative.

## See Also
- [`Gl::bind_vertex_array`]
- [`Gl::delete_vertex_arrays`]
