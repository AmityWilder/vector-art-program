# glGetFragDataIndex
query the bindings of color indices to user-defined varying out
  variables

## Parameters
- `program`
  The name of the program containing varying out variable whose binding
  to query

## Description
[`Gl::get_frag_data_index`] returns the index of the fragment color to
  which the variable `name` was bound when the program object `program`
  was last linked. If `name` is not a varying out variable of `program`,
  or if an error occurs, -1 will be returned.

## Notes
[`Gl::get_frag_data_index`] is available only if the GL version is 3.3
  or greater.

## Errors
- [`gl::INVALID_OPERATION`] is generated if `program` is not the name of
  a program object.

## See Also
- [`Gl::create_program`]
- [`Gl::bind_frag_data_location`]
- [`Gl::bind_frag_data_location_indexed`]
- [`Gl::get_frag_data_location`]
