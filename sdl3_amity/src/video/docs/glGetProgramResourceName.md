# glGetProgramResourceName
query the name of an indexed resource within a program

## Parameters
- `program`
  The name of a program object whose resources to query.

## Description
[`Gl::get_program_resource_name`] retrieves the name string assigned
  to the single active resource with an index of `index` in the
  interface `programInterface` of program object `program`. `index` must
  be less than the number of entries in the active resource list for
  `programInterface`.
`program` must be the name of an existing program object.
  `programInterface` is the name of the interface within `program` which
  contains the resource and must be one of the following values:
The name string assigned to the active resource identified by `index`
  is returned as a null-terminated string in the character array whose
  address is given in `name`. The actual number of characters written
  into `name`, excluding the null terminator, is returned in `length`.
  If `length` is NULL, no length is returned. The maximum number of
  characters that may be written into `name`, including the null
  terminator, is specified by `bufSize`. If the length of the name
  string *including the null terminator* is greater than `bufSize`, the
  first `bufSize`-1 characters of the name string will be written to
  `name`, followed by a null terminator. If `bufSize` is zero, no error
  will be generated but no characters will be written to `name`. The
  length of the longest name string for `programInterface`>, including a
  null terminator, can be queried by calling
  [`Gl::get_program_interface`] with a `pname` of
  [`gl::MAX_NAME_LENGTH`].

## Errors
- [`gl::INVALID_ENUM`] is generated if `programInterface` is not one of
  the accepted interface types.
- [`gl::INVALID_VALUE`] is generated if `progam` is not the name of an
  existing program.
- [`gl::INVALID_VALUE`] is generated if `index` is greater than or equal
  to the number of entries in the active resource list for
  `programInterface`.
- [`gl::INVALID_ENUM`] is generated if `programInterface` is
  [`gl::ATOMIC_COUNTER_BUFFER`] or [`gl::TRANSFORM_FEEDBACK_BUFFER`],
  since active atomic counter and transform feedback buffer resources
  are not assigned name strings.

## See Also
- [`Gl::get_program_resource_index`]
- [`Gl::get_program_resource`]
- [`Gl::get_program_resource_location`]
- [`Gl::get_program_resource_location_index`]
