# glPopDebugGroup
pop the active debug group

## Description
[`Gl::pop_debug_group`] pops the active debug group. After popping a
  debug group, the GL will also generate a debug output message
  describing its cause based on the `message` string, the source
  `source`, and an ID `id` submitted to the corresponding
  [`Gl::push_debug_group`] command. [`gl::DEBUG_TYPE_PUSH_GROUP`] and
  [`gl::DEBUG_TYPE_POP_GROUP`] share a single namespace for message
  `id`. `severity` has the value [`gl::DEBUG_SEVERITY_NOTIFICATION`].
  The `type` has the value [`gl::DEBUG_TYPE_POP_GROUP`]. Popping a debug
  group restores the debug output volume control of the parent debug
  group.

## Errors
- [`gl::STACK_UNDERFLOW`] is generated if an attempt is made to pop the
  default debug group from the stack.

## See Also
- [`Gl::push_debug_group`]
- [`Gl::object_label`]
- [`Gl::object_ptr_label`]
