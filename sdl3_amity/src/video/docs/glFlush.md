# glFlush
force execution of GL commands in finite time

## Description
Different GL implementations buffer commands in several different
  locations, including network buffers and the graphics accelerator
  itself. [`Gl::flush`] empties all of these buffers, causing all issued
  commands to be executed as quickly as they are accepted by the actual
  rendering engine. Though this execution may not be completed in any
  particular time period, it does complete in finite time.
Because any GL program might be executed over a network, or on an
  accelerator that buffers commands, all programs should call
  [`Gl::flush`] whenever they count on having all of their previously
  issued commands completed. For example, call [`Gl::flush`] before
  waiting for user input that depends on the generated image.

## Notes
[`Gl::flush`] can return at any time. It does not wait until the
  execution of all previously issued GL commands is complete.

## See Also
- [`Gl::finish`]
