# glGetFragDataLocation
query the bindings of color numbers to user-defined varying out
  variables

## Parameters
- `program`
  The name of the program containing varying out variable whose binding
  to query

## Description
[`Gl::get_frag_data_location`] retrieves the assigned color number
  binding for the user-defined varying out variable `name` for program
  `program`. `program` must have previously been linked. `name` must be
  a null-terminated string. If `name` is not the name of an active user-
  defined varying out fragment shader variable within `program`, -1 will
  be returned.

## Errors
- [`gl::INVALID_OPERATION`] is generated if `program` is not the name of
  a program object.

## See Also
- [`Gl::create_program`]
- [`Gl::bind_frag_data_location`]
