# glDebugMessageCallback
specify a callback to receive debugging messages from the GL

## Parameters
- `callback`
  The address of a callback function that will be called when a debug
  message is generated.

## Description
[`Gl::debug_message_callback`] sets the current debug output callback
  function to the function whose address is given in `callback`. The
  callback function should have the following prototype (in C), or be
  otherwise compatible with such a prototype:
This function is defined to have the same calling convention as the GL
  API functions. In most cases this is defined as ```c APIENTRY ``` ,
  although it will vary depending on platform, language and compiler.
Each time a debug message is generated the debug callback function
  will be invoked with `source`, `type`, `id`, and `severity` associated
  with the message, and `length` set to the length of debug message
  whose character string is in the array pointed to by `message`.
  `userParam` will be set to the value passed in the `userParam`
  parameter to the most recent call to [`Gl::debug_message_callback`].

## Notes
When the GL is in use remotely, the server may not be able to call
  functions in the client's address space. In such cases, the callback
  function may not be invoked and the user should retrieve debug
  messages from the context's debug message log by calling
  [`Gl::get_debug_message_log`].

## See Also
- [`Gl::debug_message_control`]
- [`Gl::debug_message_insert`]
- [`Gl::get_debug_message_log`]
