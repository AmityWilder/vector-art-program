# glXWaitGL
complete GL execution prior to subsequent X calls

## Description
GL rendering calls made prior to [`Gl::x_wait_gl`] are guaranteed to
  be executed before X rendering calls made after [`Gl::x_wait_gl`].
  Although this same result can be achieved using [`Gl::finish`],
  [`Gl::x_wait_gl`] does not require a round trip to the server, and it
  is therefore more efficient in cases where client and server are on
  separate machines.
[`Gl::x_wait_gl`] is ignored if there is no current GLX context.

## Notes
[`Gl::x_wait_gl`] may or may not flush the X stream.

## Errors
- [`GLXBadCurrentWindow`] is generated if the drawable associated with
  the current context of the calling thread is a window, and that window
  is no longer valid.

## See Also
- [`Gl::finish`]
- [`Gl::flush`]
- [`Gl::x_wait_x`]
