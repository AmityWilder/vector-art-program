# gl_PatchVerticesIn
contains the number of vertices in the current patch

## Description
Available only in the tessellation control and evaluation languages,
  [`Gl::patch_vertices_in`] contains the number of vertices in the input
  being processed by the shader. A single tessellation control or
  evaluation shader can read patches of differing sizes, and so th value
  of [`Gl::patch_vertex_in`] may differ between patches.

## See Also
- [`Gl::tess_level_outer`]
- [`Gl::tess_level_inner`]
- [`Gl::tess_coord`]
