# glIsList
determine if a name corresponds to a display list

## Parameters
- `list`
  Specifies a potential display list name.

## Description
[`Gl::is_list`] returns [`gl::TRUE`] if `list` is the name of a
  display list and returns [`gl::FALSE`] if it is not, or if an error
  occurs.
A name returned by [`Gl::gen_lists`], but not yet associated with a
  display list by calling [`Gl::new_list`], is not the name of a display
  list.

## Errors
- [`gl::INVALID_OPERATION`] is generated if [`Gl::is_list`] is executed
  between the execution of [`Gl::begin`] and the corresponding execution
  of [`Gl::end`].

## See Also
- [`Gl::call_list`]
- [`Gl::call_lists`]
- [`Gl::delete_lists`]
- [`Gl::gen_lists`]
- [`Gl::new_list`]
