# glXSwapBuffers
exchange front and back buffers

## Parameters
- `dpy`
  Specifies the connection to the X server.

## Description
[`Gl::x_swap_buffers`] promotes the contents of the back buffer of
  `drawable` to become the contents of the front buffer of `drawable`.
  The contents of the back buffer then become undefined. The update
  typically takes place during the vertical retrace of the monitor,
  rather than immediately after [`Gl::x_swap_buffers`] is called.
[`Gl::x_swap_buffers`] performs an implicit [`Gl::flush`] before it
  returns. Subsequent OpenGL commands may be issued immediately after
  calling [`Gl::x_swap_buffers`], but are not executed until the buffer
  exchange is completed.
If `drawable` was not created with respect to a double-buffered
  visual, [`Gl::x_swap_buffers`] has no effect, and no error is
  generated.

## Notes
The contents of the back buffer become undefined after a swap. Note
  that this applies to pixel buffers as well as windows.
All GLX rendering contexts share the same notion of which are front
  buffers and which are back buffers. One consequence is that when
  multiple clients are rendering to the same double-buffered window, all
  of them should finish rendering before one of them issues the command
  to swap buffers. The clients are responsible for implementing this
  synchronization. Typically this is accomplished by executing
  [`Gl::finish`] and then using a semaphore in shared memory to
  rendezvous before swapping.

## Errors
- [`GLXBadDrawable`] is generated if `drawable` is not a valid GLX
  drawable.
- [`GLXBadCurrentWindow`] is generated if `dpy` and `drawable` are
  respectively the display and drawable associated with the current
  context of the calling thread, and `drawable` identifies a window that
  is no longer valid.

## See Also
- [`Gl::flush`]
