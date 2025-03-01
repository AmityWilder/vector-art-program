# gl_TessLevelInner
contains the inner tessellation levels for the current patch

## Description
Available only in the tessellation control and evaluation languages,
  [`Gl::tess_level_inner`] is used to assign values to the corresponding
  inner tesellation levels of the current patch. The values written into
  [`Gl::tess_level_inner`] by the tessellation control shader are used
  by the tessellation primitive generator to control primitive
  tessellation and may be read by the subsequent tessellation evaluation
  shader.
As inputs to the tessellation evaluation shader,
  [`Gl::tess_level_inner`] contains the values written by the
  tessellation control shader, if present. If no tessellation control
  shader is present, it contains the default tessellation level.

## See Also
- [`Gl::tess_level_outer`]
- [`Gl::tess_coord`]
