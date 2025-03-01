# glPushName
push and pop the name stack

## Parameters
- `name`
  Specifies a name that will be pushed onto the name stack.

## Description
The name stack is used during selection mode to allow sets of
  rendering commands to be uniquely identified. It consists of an
  ordered set of unsigned integers and is initially empty.
[`Gl::push_name`] causes `name` to be pushed onto the name stack.
  [`Gl::pop_name`] pops one name off the top of the stack.
The maximum name stack depth is implementation-dependent; call
  [`gl::MAX_NAME_STACK_DEPTH`] to find out the value for a particular
  implementation. It is an error to push a name onto a full stack or to
  pop a name off an empty stack. It is also an error to manipulate the
  name stack between the execution of [`Gl::begin`] and the
  corresponding execution of [`Gl::end`]. In any of these cases, the
  error flag is set and no other change is made to GL state.
The name stack is always empty while the render mode is not
  [`gl::SELECT`]. Calls to [`Gl::push_name`] or [`Gl::pop_name`] while
  the render mode is not [`gl::SELECT`] are ignored.

## Errors
- [`gl::STACK_OVERFLOW`] is generated if [`Gl::push_name`] is called
  while the name stack is full.
- [`gl::STACK_UNDERFLOW`] is generated if [`Gl::pop_name`] is called
  while the name stack is empty.
- [`gl::INVALID_OPERATION`] is generated if [`Gl::push_name`] or
  [`Gl::pop_name`] is executed between a call to [`Gl::begin`] and the
  corresponding call to [`Gl::end`].

## See Also
- [`Gl::init_names`]
- [`Gl::load_name`]
- [`Gl::render_mode`]
- [`Gl::select_buffer`]
