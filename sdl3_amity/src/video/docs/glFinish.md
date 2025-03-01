# glFinish
block until all GL execution is complete

## Description
[`Gl::finish`] does not return until the effects of all previously
  called GL commands are complete. Such effects include all changes to
  GL state, all changes to connection state, and all changes to the
  frame buffer contents.

## Notes
[`Gl::finish`] requires a round trip to the server.

## See Also
- [`Gl::flush`]
