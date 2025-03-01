# glInitNames
initialize the name stack

## Description
The name stack is used during selection mode to allow sets of
  rendering commands to be uniquely identified. It consists of an
  ordered set of unsigned integers. [`Gl::init_names`] causes the name
  stack to be initialized to its default empty state.
The name stack is always empty while the render mode is not
  [`gl::SELECT`]. Calls to [`Gl::init_names`] while the render mode is
  not [`gl::SELECT`] are ignored.

## Errors
- [`gl::INVALID_OPERATION`] is generated if [`Gl::init_names`] is
  executed between the execution of [`Gl::begin`] and the corresponding
  execution of [`Gl::end`].

## See Also
- [`Gl::load_name`]
- [`Gl::push_name`]
- [`Gl::render_mode`]
- [`Gl::select_buffer`]
