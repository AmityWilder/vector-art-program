# glXWaitX
complete X execution prior to subsequent GL calls

## Description
X rendering calls made prior to [`Gl::x_wait_x`] are guaranteed to be
  executed before GL rendering calls made after [`Gl::x_wait_x`].
  Although the same result can be achieved using [`Gl::x_sync`],
  [`Gl::x_wait_x`] does not require a round trip to the server, and it
  is therefore more efficient in cases where client and server are on
  separate machines.
[`Gl::x_wait_x`] is ignored if there is no current GLX context.

## Notes
[`Gl::x_wait_x`] may or may not flush the GL stream.

## Errors
- [`GLXBadCurrentWindow`] is generated if the drawable associated with
  the current context of the calling thread is a window, and that window
  is no longer valid.

## See Also
- [`Gl::finish`]
- [`Gl::flush`]
- [`Gl::x_wait_gl`]
