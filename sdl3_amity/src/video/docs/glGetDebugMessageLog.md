# glGetDebugMessageLog
retrieve messages from the debug message log

## Parameters
- `count`
  The number of debug messages to retrieve from the log.

## Description
[`Gl::get_debug_message_log`] retrieves messages from the debug
  message log. A maximum of `count` messages are retrieved from the log.
  If `sources` is not NULL then the source of each message is written
  into up to `count` elements of the array. If `types` is not NULL then
  the type of each message is written into up to `count` elements of the
  array. If `id` is not NULL then the identifier of each message is
  written into up to `count` elements of the array. If `severities` is
  not NULL then the severity of each message is written into up to
  `count` elements of the array. If `lengths` is not NULL then the
  length of each message is written into up to `count` elements of the
  array.
`messageLog` specifies the address of a character array into which the
  debug messages will be written. Each message will be concatenated onto
  the array starting at the first element of `messageLog`. `bufSize`
  specifies the size of the array `messageLog`. If a message will not
  fit into the remaining space in `messageLog` then the function
  terminates and returns the number of messages written so far, which
  may be zero.
If [`Gl::get_debug_message_log`] returns zero then no messages are
  present in the debug log, or there was not enough space in
  `messageLog` to retrieve the first message in the queue. If
  `messageLog` is NULL then no messages are written and the value of
  `bufSize` is ignored.

## Notes
Although debug messages may be enabled in a non-debug context, the
  quantity and detail of such messages may be substantially inferior to
  those in a debug context. In particular, a valid implementation of the
  debug message queue in a non-debug context may produce no messages at
  all.

## Errors
- [`gl::INVALID_VALUE`] is generated if `count` or `bufSize` is
  negative.

## See Also
- [`Gl::debug_message_insert`]
- [`Gl::debug_message_callback`]
- [`Gl::debug_message_control`]
