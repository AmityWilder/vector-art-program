# glDebugMessageInsert
inject an application-supplied message into the debug message queue

## Parameters
- `source`
  The source of the debug message to insert.

## Description
[`Gl::debug_message_insert`] inserts a user-supplied message into the
  debug output queue. `source` specifies the source that will be used to
  classify the message and must be [`gl::DEBUG_SOURCE_APPLICATION`] or
  [`gl::DEBUG_SOURCE_THIRD_PARTY`]. All other sources are reserved for
  use by the GL implementation. `type` indicates the type of the message
  to be inserted and may be one of [`gl::DEBUG_TYPE_ERROR`],
  [`gl::DEBUG_TYPE_DEPRECATED_BEHAVIOR`],
  [`gl::DEBUG_TYPE_UNDEFINED_BEHAVIOR`], [`gl::DEBUG_TYPE_PORTABILITY`],
  [`gl::DEBUG_TYPE_PERFORMANCE`], [`gl::DEBUG_TYPE_MARKER`],
  [`gl::DEBUG_TYPE_PUSH_GROUP`], [`gl::DEBUG_TYPE_POP_GROUP`], or
  [`gl::DEBUG_TYPE_OTHER`]. `severity` indicates the severity of the
  message and may be [`gl::DEBUG_SEVERITY_LOW`],
  [`gl::DEBUG_SEVERITY_MEDIUM`], [`gl::DEBUG_SEVERITY_HIGH`] or
  [`gl::DEBUG_SEVERITY_NOTIFICATION`]. `id` is available for application
  defined use and may be any value. This value will be recorded and used
  to identify the message.
`length` contains a count of the characters in the character array
  whose address is given in `message`. If `length` is negative then
  `message` is treated as a null-terminated string. The length of the
  message, whether specified explicitly or implicitly, must be less than
  or equal to the implementation defined constant
  [`gl::MAX_DEBUG_MESSAGE_LENGTH`].

## Notes
[`gl::DEBUG_TYPE_MARKER`], [`gl::DEBUG_TYPE_PUSH_GROUP`],
  [`gl::DEBUG_TYPE_POP_GROUP`], and [`gl::DEBUG_SEVERITY_NOTIFICATION`]
  are available only if the GL version is 4.3 or higher.

## Errors
- [`gl::INVALID_ENUM`] is generated if any of `source`, `type` or
  `severity` is not one of the accepted interface types.
- [`gl::INVALID_VALUE`] is generated if the length of the message is
  greater than the value of [`gl::MAX_DEBUG_MESSAGE_LENGTH`].

## See Also
- [`Gl::debug_message_control`]
- [`Gl::debug_message_callback`]
- [`Gl::get_debug_message_log`]
