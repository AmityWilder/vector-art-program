# glGetProgramResource
retrieve values for multiple properties of a single active resource
  within a program object

## Parameters
- `program`
  The name of a program object whose resources to query.

## Description
[`Gl::get_program_resourceiv`] returns values for multiple properties
  of a single active resource with an index of `index` in the interface
  `programInterface` of program object `program`. For each resource,
  values for `propCount` properties specified by the array `props` are
  returned. `propCount` may not be zero. An error is generated if any
  value in `props` is not one of the properties described immediately
  belowor if any value in `props` is not allowed for `programInterface`.
  The set of allowed `programInterface` values for each property can be
  found in the following table:
For the property [`gl::NAME_LENGTH`], a single integer identifying the
  length of the name string associated with an active variable,
  interface block, or subroutine is written to `params`. The name length
  includes a terminating null character.
For the property [`gl::TYPE`], a single integer identifying the type
  of an active variable is written to `params`. The integer returned is
  one of the values found in table 2.16.
For the property [`gl::ARRAY_SIZE`], a single integer identifying the
  number of active array elements of an active variable is written to
  `params`. The array size returned is in units of the type associated
  with the property [`gl::TYPE`]. For active variables not corresponding
  to an array of basic types, the value zero is written to `params`.
For the property [`gl::BLOCK_INDEX`], a single integer identifying the
  index of the active interface block containing an active variable is
  written to `params`. If the variable is not the member of an interface
  block, the value -1 is written to `params`.
For the property [`gl::OFFSET`], a single integer identifying the
  offset of an active variable is written to `params`. For variables in
  the [`gl::UNIFORM`] and [`gl::BUFFER_VARIABLE`] interfaces that are
  backed by a buffer object, the value written is the offset of that
  variable relative to the base of the buffer range holding its value.
  For variables in the [`gl::TRANSFORM_FEEDBACK_VARYING`] interface, the
  value written is the offset in the transform feedback buffer storage
  assigned to each vertex captured in transform feedback mode where the
  value of the variable will be stored. Such offsets are specified via
  the ```c xfb_offset ``` layout qualifier or assigned according to the
  variables position in the list of strings passed to
  [`Gl::transform_feedback_varyings`]. Offsets are expressed in basic
  machine units. For all variables not recorded in transform feedback
  mode, including the special names ```c "gl_NextBuffer" ``` , ```c
  "gl_SkipComponents1" ``` , ```c "gl_SkipComponents2" ``` , ```c
  "gl_SkipComponents3" ``` , and ```c "gl_SkipComponents4" ``` , -1 is
  written to `params`.
For the property [`gl::ARRAY_STRIDE`], a single integer identifying
  the stride between array elements in an active variable is written to
  `params`. For active variables declared as an array of basic types,
  the value written is the difference, in basic machine units, between
  the offsets of consecutive elements in an array. For active variables
  not declared as an array of basic types, zero is written to `params`.
  For active variables not backed by a buffer object, -1 is written to
  `params`, regardless of the variable type.
For the property [`gl::MATRIX_STRIDE`], a single integer identifying
  the stride between columns of a column-major matrix or rows of a row-
  major matrix is written to `params`. For active variables declared a
  single matrix or array of matrices, the value written is the
  difference, in basic machine units, between the offsets of consecutive
  columns or rows in each matrix. For active variables not declared as a
  matrix or array of matrices, zero is written to `params`. For active
  variables not backed by a buffer object, -1 is written to `params`,
  regardless of the variable type.
For the property [`gl::IS_ROW_MAJOR`], a single integer identifying
  whether an active variable is a row-major matrix is written to
  `params`. For active variables backed by a buffer object, declared as
  a single matrix or array of matrices, and stored in row-major order,
  one is written to `params`. For all other active variables, zero is
  written to `params`.
For the property [`gl::ATOMIC_COUNTER_BUFFER_INDEX`], a single integer
  identifying the index of the active atomic counter buffer containing
  an active variable is written to `params`. If the variable is not an
  atomic counter uniform, the value -1 is written to `params`.
For the property [`gl::BUFFER_BINDING`], to index of the buffer
  binding point associated with the active uniform block, shader storage
  block, atomic counter buffer or transform feedback buffer is written
  to `params`.
For the property [`gl::BUFFER_DATA_SIZE`], then the implementation-
  dependent minimum total buffer object size, in basic machine units,
  required to hold all active variables associated with an active
  uniform block, shader storage block, or atomic counter buffer is
  written to `params`. If the final member of an active shader storage
  block is array with no declared size, the minimum buffer size is
  computed assuming the array was declared as an array with one element.
For the property [`gl::NUM_ACTIVE_VARIABLES`], the number of active
  variables associated with an active uniform block, shader storage
  block, atomic counter buffer or transform feedback buffer is written
  to `params`.
For the property [`gl::ACTIVE_VARIABLES`], an array of active variable
  indices associated with an active uniform block, shader storage block,
  atomic counter buffer or transform feedback buffer is written to
  `params`. The number of values written to `params` for an active
  resource is given by the value of the property
  [`gl::NUM_ACTIVE_VARIABLES`] for the resource.
For the properties [`gl::REFERENCED_BY_VERTEX_SHADER`],
  [`gl::REFERENCED_BY_TESS_CONTROL_SHADER`],
  [`gl::REFERENCED_BY_TESS_EVALUATION_SHADER`],
  [`gl::REFERENCED_BY_GEOMETRY_SHADER`],
  [`gl::REFERENCED_BY_FRAGMENT_SHADER`], and
  [`gl::REFERENCED_BY_COMPUTE_SHADER`], a single integer is written to
  `params`, identifying whether the active resource is referenced by the
  vertex, tessellation control, tessellation evaluation, geometry, or
  fragment shaders, respectively, in the program object. The value one
  is written to `params` if an active variable is referenced by the
  corresponding shader, or if an active uniform block, shader storage
  block, or atomic counter buffer contains at least one variable
  referenced by the corresponding shader. Otherwise, the value zero is
  written to `params`.
For the property [`gl::TOP_LEVEL_ARRAY_SIZE`], a single integer
  identifying the number of active array elements of the top-level
  shader storage block member containing to the active variable is
  written to `params`. If the top-level block member is not declared as
  an array, the value one is written to `params`. If the top-level block
  member is an array with no declared size, the value zero is written to
  `params`.
For the property [`gl::TOP_LEVEL_ARRAY_STRIDE`], a single integer
  identifying the stride between array elements of the top-level shader
  storage block member containing the active variable is written to
  `params`. For top-level block members declared as arrays, the value
  written is the difference, in basic machine units, between the offsets
  of the active variable for consecutive elements in the top-level
  array. For top-level block members not declared as an array, zero is
  written to `params`.
For the property [`gl::LOCATION`], a single integer identifying the
  assigned location for an active uniform, input, output, or subroutine
  uniform variable is written to `params`. For input, output, or uniform
  variables with locations specified by a layout qualifier, the
  specified location is used. For vertex shader input or fragment shader
  output variables without a layout qualifier, the location assigned
  when a program is linked is written to `params`. For all other input
  and output variables, the value -1 is written to `params`. For
  uniforms in uniform blocks, the value -1 is written to `params`.
For the property [`gl::LOCATION_INDEX`], a single integer identifying
  the fragment color index of an active fragment shader output variable
  is written to `params`. If the active variable is an output for a non-
  fragment shader, the value -1 will be written to `params`.
For the property [`gl::IS_PER_PATCH`], a single integer identifying
  whether the input or output is a per-patch attribute. If the active
  variable is a per-patch attribute (declared with the ```c patch ```
  qualifier), the value one is written to `params`; otherwise, the value
  zero is written to `params`.
For the property [`gl::LOCATION_COMPONENT`], a single integer
  indicating the first component of the location assigned to an active
  input or output variable is written to `params`. For input and output
  variables with a component specified by a ```c layout ``` qualifier,
  the specified component is written. For all other input and output
  variables, the value zero is written.
For the property [`gl::TRANSFORM_FEEDBACK_BUFFER_INDEX`], a single
  integer identifying the index of the active transform feedback buffer
  associated with an active variable is written to `params`. For
  variables corresponding to the special names ```c "gl_NextBuffer" ```
  , ```c "gl_SkipComponents1" ``` , ```c "gl_SkipComponents2" ``` , ```c
  "gl_SkipComponents3" ``` , and ```c "gl_SkipComponents4" ``` , -1 is
  written to `params`.
For the property [`gl::TRANSFORM_FEEDBACK_BUFFER_STRIDE`], a single
  integer identifying the stride, in basic machine units, between
  consecutive vertices written to the transform feedback buffer is
  written to `params`.

## Errors
- [`gl::INVALID_VALUE`] is generated if `program` is not the name of an
  existing program object.
- [`gl::INVALID_VALUE`] is generated if `propCount` is zero.
- [`gl::INVALID_ENUM`] is generated if `programInterface` is not one of
  the accepted interface types.
- [`gl::INVLALID_ENUM`] is generated if any value in `props` is not one
  of the accepted tokens for the interface `programInterface`

## See Also
- [`Gl::get_program_resource_name`]
- [`Gl::get_program_resource_index`]
- [`Gl::get_program_resource_location`]
- [`Gl::get_program_resource_location_index`]
