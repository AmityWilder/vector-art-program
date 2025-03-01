# glGetProgramResourceIndex
query the index of a named resource within a program

## Parameters
- `program`
  The name of a program object whose resources to query.

## Description
[`Gl::get_program_resource_index`] returns the unsigned integer index
  assigned to a resource named `name` in the interface type
  `programInterface` of program object `program`.
`program` must be the name of an existing program object.
  `programInterface` is the name of the interface within `program` which
  contains the resource named `name`and must be one of the following
  values:
If `name` exactly matches the name string of one of the active
  resources for `programInterface`, the index of the matched resource is
  returned. Additionally, if `name` would exactly match the name string
  of an active resource if "[0]" were appended to `name`, the index of
  the matched resource is returned. Otherwise, `name` is considered not
  to be the name of an active resource, and [`gl::INVALID_INDEX`] is
  returned.
For the interface [`gl::TRANSFORM_FEEDBACK_VARYING`], the value
  [`gl::INVALID_INDEX`] should be returned when querying the index
  assigned to the special names ```c gl_NextBuffer ``` , ```c
  gl_SkipComponents1 ``` , ```c gl_SkipComponents2 ``` , ```c
  gl_SkipComponents3 ``` , or ```c gl_SkipComponents4 ``` .

## Errors
- [`gl::INVALID_ENUM`] is generated if `programInterface` is not one of
  the accepted interface types.
- [`gl::INVALID_ENUM`] is generated if `programInterface` is
  [`gl::ATOMIC_COUNTER_BUFFER`] or [`gl::TRANSFORM_FEEDBACK_BUFFER`],
  since active atomic counter and transform feedback buffer resources
  are not assigned name strings.
- Although not an error, [`gl::INVALID_INDEX`] is returned if `name` is
  not the name of a resource within the interface identified by
  `programInterface`.

## See Also
- [`Gl::get_program_resource_name`]
- [`Gl::get_program_resource`]
- [`Gl::get_program_resource_location`]
- [`Gl::get_program_resource_location_index`]
