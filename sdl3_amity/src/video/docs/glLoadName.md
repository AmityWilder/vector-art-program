# glLoadName
load a name onto the name stack

## Parameters
- `name`
  Specifies a name that will replace the top value on the name stack.

## Description
The name stack is used during selection mode to allow sets of
  rendering commands to be uniquely identified. It consists of an
  ordered set of unsigned integers and is initially empty.
[`Gl::load_name`] causes `name` to replace the value on the top of the
  name stack.
The name stack is always empty while the render mode is not
  [`gl::SELECT`]. Calls to [`Gl::load_name`] while the render mode is
  not [`gl::SELECT`] are ignored.

## Errors
- [`gl::INVALID_OPERATION`] is generated if [`Gl::load_name`] is called
  while the name stack is empty.
- [`gl::INVALID_OPERATION`] is generated if [`Gl::load_name`] is
  executed between the execution of [`Gl::begin`] and the corresponding
  execution of [`Gl::end`].

## See Also
- [`Gl::init_names`]
- [`Gl::push_name`]
- [`Gl::render_mode`]
- [`Gl::select_buffer`]
