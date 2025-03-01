# gl_TessCoord
contains the coordinate of the vertex within the current patch

## Description
Available only in the tessellation cevaluation language,
  [`Gl::tess_coord`] specifies the three component (u, v, w) vector
  identifying the position of the vertex being processed by the shader
  relative to the primitive being tessellated.

## See Also
- [`Gl::tess_level_outer`]
- [`Gl::tess_level_inner`]
- [`Gl::patch_vertices_in`]
