# removedTypes
Describes types removed and replaced in the OpenGL API in OpenGL 4.2

## Description
In the May, 2012 update of the OpenGL 4.2 Specification, all APIs
  using the types GLclampf and GLclampd were modified to use GLfloat and
  GLdouble, respectively, instead. Language was added to the
  Specification requiring that these parameters be clamped, when
  required, at use time rather than at specification time.
This change allows specifying parameters in ranges appropriate for
  non-fixed-point framebuffers (integer and floating-point formats). The
  change does not require any changes to user code calling these
  functions, because the actual underlying types are identical, the
  behavior is externally unchanged, and the header files continue to
  define the old types for compatibility with older code.

## See Also
- [`Gl::blend_color`]
- [`Gl::clear_color`]
- [`Gl::clear_depth`]
- [`Gl::depth_range`]
- [`Gl::depth_range_array`]
- [`Gl::depth_range_indexed`]
- [`Gl::min_sample_shading`]
- [`Gl::sample_coverage`]
