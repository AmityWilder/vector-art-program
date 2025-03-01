# glCallList
execute a display list

## Parameters
- `list`
  Specifies the integer name of the display list to be executed.

## Description
[`Gl::call_list`] causes the named display list to be executed. The
  commands saved in the display list are executed in order, just as if
  they were called without using a display list. If `list` has not been
  defined as a display list, [`Gl::call_list`] is ignored.
[`Gl::call_list`] can appear inside a display list. To avoid the
  possibility of infinite recursion resulting from display lists calling
  one another, a limit is placed on the nesting level of display lists
  during display-list execution. This limit is at least 64, and it
  depends on the implementation.
GL state is not saved and restored across a call to [`Gl::call_list`].
  Thus, changes made to GL state during the execution of a display list
  remain after execution of the display list is completed. Use
  [`Gl::push_attrib`], [`Gl::pop_attrib`], [`Gl::push_matrix`], and
  [`Gl::pop_matrix`] to preserve GL state across [`Gl::call_list`]
  calls.

## Notes
Display lists can be executed between a call to [`Gl::begin`] and the
  corresponding call to [`Gl::end`], as long as the display list
  includes only commands that are allowed in this interval.

## See Also
- [`Gl::delete_lists`]
- [`Gl::gen_lists`]
- [`Gl::new_list`]
- [`Gl::push_attrib`]
- [`Gl::push_matrix`]
