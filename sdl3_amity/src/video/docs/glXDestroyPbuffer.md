# glXDestroyPbuffer
destroy an off-screen rendering area

## Parameters
- `dpy`
  Specifies the connection to the X server.

## Description
[`Gl::x_destroy_pbuffer`] destroys a GLXPbuffer created by
  [`Gl::x_create_pbuffer`].

## Notes
[`Gl::x_destroy_pbuffer`] is available only if the GLX version is 1.3
  or greater.
If the GLX version is 1.1 or 1.0, the GL version must be 1.0. If the
  GLX version is 1.2, then the GL version must be 1.1. If the GLX
  version is 1.3, then the GL version must be 1.2.

## Errors
- [`GLXBadPbuffer`] is generated if `pbuf` is not a valid GLXPbuffer.

## See Also
- [`Gl::x_choose_fb_config`]
- [`Gl::x_create_pbuffer`]
- [`Gl::x_make_context_current`]
