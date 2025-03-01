# glPushDebugGroup
push a named debug group into the command stream

## Parameters
- `source`
  The source of the debug message.

## Description
[`Gl::push_debug_group`] pushes a debug group described by the string
  `message` into the command stream. The value of `id` specifies the ID
  of messages generated. The parameter `length` contains the number of
  characters in `message`. If `length` is negative, it is implied that
  `message` contains a null terminated string. The message has the
  specified `source` and `id`, the `type` [`gl::DEBUG_TYPE_PUSH_GROUP`],
  and `severity` [`gl::DEBUG_SEVERITY_NOTIFICATION`]. The GL will put a
  new debug group on top of the debug group stack which inherits the
  control of the volume of debug output of the debug group previously
  residing on the top of the debug group stack. Because debug groups are
  strictly hierarchical, any additional control of the debug output
  volume will only apply within the active debug group and the debug
  groups pushed on top of the active debug group.

## Errors
- [`gl::INVALID_ENUM`] is generated if the value of `source` is neither
  [`gl::DEBUG_SOURCE_APPLICATION`] nor [`gl::DEBUG_SOURCE_THIRD_PARTY`].
- [`gl::INVALID_VALUE`] is generated if `length` is negative and the
  number of characters in `message`, excluding the null-terminator, is
  not less than the value of [`gl::MAX_DEBUG_MESSAGE_LENGTH`].

## See Also
- [`Gl::pop_debug_group`]
- [`Gl::object_label`]
- [`Gl::object_ptr_label`]
