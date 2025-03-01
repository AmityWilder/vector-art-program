# glGetActiveSubroutineName
query the name of an active shader subroutine

## Parameters
- `program`
  Specifies the name of the program containing the subroutine.

## Description
[`Gl::get_active_subroutine_name`] queries the name of an active
  shader subroutine uniform from the program object given in `program`.
  `index` specifies the index of the shader subroutine uniform within
  the shader stage given by `stage`, and must between zero and the value
  of [`gl::ACTIVE_SUBROUTINES`] minus one for the shader stage.
The name of the selected subroutine is returned as a null-terminated
  string in `name`. The actual number of characters written into `name`,
  not including the null-terminator, is returned in `length`. If
  `length` is [`NULL`], no length is returned. The maximum number of
  characters that may be written into `name`, including the null-
  terminator, is given in `bufSize`.

## Errors
- [`gl::INVALID_VALUE`] is generated if `index` is greater than or equal
  to the value of [`gl::ACTIVE_SUBROUTINES`].
- [`gl::INVALID_VALUE`] is generated if `program` is not the name of an
  existing program object.

## See Also
- [`Gl::get_subroutine_index`]
- [`Gl::get_active_subroutine_uniform`]
- [`Gl::get_program_stage`]
