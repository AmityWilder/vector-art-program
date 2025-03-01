# glGet
return the value or values of a selected parameter

## Parameters
- `pname`
  Specifies the parameter value to be returned for non-indexed versions
  of [`Gl::get`]. The symbolic constants in the list below are accepted.

## Description
These commands return values for simple state variables in GL. `pname`
  is a symbolic constant indicating the state variable to be returned,
  and `data` is a pointer to an array of the indicated type in which to
  place the returned data.
Type conversion is performed if `data` has a different type than the
  state variable value being requested. If [`Gl::get_booleanv`] is
  called, a floating-point (or integer) value is converted to
  [`gl::FALSE`] if and only if it is 0.0 (or 0). Otherwise, it is
  converted to [`gl::TRUE`]. If [`Gl::get_integerv`] is called, boolean
  values are returned as [`gl::TRUE`] or [`gl::FALSE`], and most
  floating-point values are rounded to the nearest integer value.
  Floating-point colors and normals, however, are returned with a linear
  mapping that maps 1.0 to the most positive representable integer value
  and $None$ to the most negative representable integer value. If $$ $$
  -1.0 [`Gl::get_floatv`] or [`Gl::get_doublev`] is called, boolean
  values are returned as [`gl::TRUE`] or [`gl::FALSE`], and integer
  values are converted to floating-point values.
The following symbolic constants are accepted by `pname`:
Many of the boolean parameters can also be queried more easily using
  [`Gl::is_enabled`].

## Notes
The following parameters return the associated value for the active
  texture unit: [`gl::TEXTURE_1D`], [`gl::TEXTURE_BINDING_1D`],
  [`gl::TEXTURE_2D`], [`gl::TEXTURE_BINDING_2D`], [`gl::TEXTURE_3D`] and
  [`gl::TEXTURE_BINDING_3D`].
[`gl::MAX_VIEWPORTS`], [`gl::VIEWPORT_SUBPIXEL_BITS`],
  [`gl::VIEWPORT_BOUNDS_RANGE`], [`gl::LAYER_PROVOKING_VERTEX`], and
  [`gl::VIEWPORT_INDEX_PROVOKING_VERTEX`] are available only if the GL
  version is 4.1 or greater.
[`gl::MAX_VERTEX_ATOMIC_COUNTERS`],
  [`gl::MAX_TESS_CONTROL_ATOMIC_COUNTERS`],
  [`gl::MAX_TESS_EVALUATION_ATOMIC_COUNTERS`],
  [`gl::MAX_GEOMETRY_ATOMIC_COUNTERS`],
  [`gl::MAX_FRAMGENT_ATOMIC_COUNTERS`], and
  [`gl::MIN_MAP_BUFFER_ALIGNMENT`] are accepted by `pname` only if the
  GL version is 4.2 or greater.
[`gl::MAX_ELEMENT_INDEX`] is accepted by `pname` only if the GL
  version is 4.3 or greater.
[`gl::MAX_COMPUTE_UNIFORM_BLOCKS`],
  [`gl::MAX_COMPUTE_TEXTURE_IMAGE_UNITS`],
  [`gl::MAX_COMPUTE_UNIFORM_COMPONENTS`],
  [`gl::MAX_COMPUTE_ATOMIC_COUNTERS`],
  [`gl::MAX_COMPUTE_ATOMIC_COUNTER_BUFFERS`],
  [`gl::MAX_COMBINED_COMPUTE_UNIFORM_COMPONENTS`],
  [`gl::MAX_COMPUTE_WORK_GROUP_INVOCATIONS`],
  [`gl::MAX_COMPUTE_WORK_GROUP_COUNT`], and
  [`gl::MAX_COMPUTE_WORK_GROUP_SIZE`] and
  [`gl::DISPATCH_INDIRECT_BUFFER_BINDING`] are available only if the GL
  version is 4.3 or greater.
[`gl::MAX_DEBUG_GROUP_STACK_DEPTH`], [`gl::DEBUG_GROUP_STACK_DEPTH`]
  and [`gl::MAX_LABEL_LENGTH`] are accepted only if the GL version is
  4.3 or greater.
[`gl::MAX_UNIFORM_LOCATIONS`] is accepted only if the GL version is
  4.3 or greater.
[`gl::MAX_FRAMEBUFFER_WIDTH`], [`gl::MAX_FRAMEBUFFER_HEIGHT`],
  [`gl::MAX_FRAMEBUFFER_LAYERS`], and [`gl::MAX_FRAMEBUFFER_SAMPLES`]
  are available only if the GL version is 4.3 or greater.
[`gl::MAX_VERTEX_SHADER_STORAGE_BLOCKS`],
  [`gl::MAX_TESS_CONTROL_SHADER_STORAGE_BLOCKS`],
  [`gl::MAX_TESS_EVALUATION_SHADER_STORAGE_BLOCKS`],
  [`gl::MAX_GEOMETRY_SHADER_STORAGE_BLOCKS`],
  [`gl::MAX_FRAGMENT_SHADER_STORAGE_BLOCKS`], and
  [`gl::MAX_COMPUTE_SHADER_STORAGE_BLOCKS`] are available only if the GL
  version is 4.3 or higher.
[`gl::TEXTURE_BUFFER_OFFSET_ALIGNMENT`] is available only if the GL
  version is 4.3 or greater.
[`gl::VERTEX_BINDING_DIVISOR`], [`gl::VERTEX_BINDING_OFFSET`],
  [`gl::VERTEX_BINDING_STRIDE`], [`gl::VERTEX_BINDING_BUFFER`],
  [`gl::MAX_VERTEX_ATTRIB_RELATIVE_OFFSET`] and
  [`gl::MAX_VERTEX_ATTRIB_BINDINGS`] are available only if the GL
  version is 4.3 or greater.

## Errors
- [`gl::INVALID_ENUM`] is generated if `pname` is not an accepted value.
- [`gl::INVALID_VALUE`] is generated on any of [`Gl::get_booleani_v`],
  [`Gl::get_integeri_v`], or [`Gl::get_integer64i_v`] if `index` is
  outside of the valid range for the indexed state `target`.

## See Also
- [`Gl::get_active_uniform`]
- [`Gl::get_attached_shaders`]
- [`Gl::get_attrib_location`]
- [`Gl::get_buffer_parameter`]
- [`Gl::get_buffer_pointerv`]
- [`Gl::get_buffer_sub_data`]
- [`Gl::get_compressed_tex_image`]
- [`Gl::get_error`]
- [`Gl::get_program`]
- [`Gl::get_program_info_log`]
- [`Gl::get_queryiv`]
- [`Gl::get_query_object`]
- [`Gl::get_shader`]
- [`Gl::get_shader_info_log`]
- [`Gl::get_shader_source`]
- [`Gl::get_string`]
- [`Gl::get_tex_image`]
- [`Gl::get_tex_level_parameter`]
- [`Gl::get_tex_parameter`]
- [`Gl::get_uniform`]
- [`Gl::get_uniform_location`]
- [`Gl::get_vertex_attrib`]
- [`Gl::get_vertex_attrib_pointerv`]
- [`Gl::is_enabled`]
