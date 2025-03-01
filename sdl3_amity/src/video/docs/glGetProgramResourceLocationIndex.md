# glGetProgramResourceLocationIndex
query the fragment color index of a named variable within a program

## Parameters
- `program`
  The name of a program object whose resources to query.

## Description
[`Gl::get_program_resource_location_index`] returns the fragment color
  index assigned to the variable named `name` in interface
  `programInterface` of program object `program`. `program` must be the
  name of a program that has been linked successfully.
  `programInterface` must be [`gl::PROGRAM_OUTPUT`].
The value -1 will be returned if an error occurs, if `name` does not
  identify an active variable on `programInterface`, or if `name`
  identifies an active variable that does not have a valid location
  assigned, as described above. The locations returned by these commands
  are the same locations returned when querying the [`gl::LOCATION`] and
  [`gl::LOCATION_INDEX`] resource properties.
A string provided to [`Gl::get_program_resource_location_index`] is
  considered to match an active variable if:
Any other string is considered not to identify an active variable. If
  the string specifies an element of an array variable,
  [`Gl::get_program_resource_location`] returns the location assigned to
  that element. If it specifies the base name of an array, it identifies
  the resources associated with the first element of the array.

## Errors
- [`gl::INVALID_VALUE`] is generated if `program` is not the name of an
  existing program object.
- [`gl::INVALID_ENUM`] is generated if `programInterface` is not one of
  the accepted interface types.
- [`gl::INVALID_OPERATION`] is generated if `program` has not been
  linked successfully.

## See Also
- [`Gl::get_program_resource_name`]
- [`Gl::get_program_resource_index`]
- [`Gl::get_program_resource`]
- [`Gl::get_program_resource_location_index`]
