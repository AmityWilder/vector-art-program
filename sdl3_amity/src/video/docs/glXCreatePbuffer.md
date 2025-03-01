# glXCreatePbuffer
create an off-screen rendering area

## Parameters
- `dpy`
  Specifies the connection to the X server.

## Description
[`Gl::x_create_pbuffer`] creates an off-screen rendering area and
  returns its XID. Any GLX rendering context that was created with
  respect to `config` can be used to render into this window. Use
  [`Gl::x_make_context_current`] to associate the rendering area with a
  GLX rendering context.
The accepted attributes for a GLXPbuffer are:
GLXPbuffers contain the color and ancillary buffers specified by
  `config`. It is possible to create a pixel buffer with back buffers
  and to swap those buffers using [`Gl::x_swap_buffers`].

## Notes
[`Gl::x_create_pbuffer`] is available only if the GLX version is 1.3
  or greater.
If the GLX version is 1.1 or 1.0, the GL version must be 1.0. If the
  GLX version is 1.2, then the GL version must be 1.1. If the GLX
  version is 1.3, then the GL version must be 1.2.
GLXPbuffers are allocated from frame buffer resources; applications
  should consider deallocating them when they are not in use.

## Errors
- [`BadAlloc`] is generated if there are insufficient resources to
  allocate the requested GLXPbuffer.
- [`GLXBadFBConfig`] is generated if `config` is not a valid
  GLXFBConfig.
- [`BadMatch`] is generated if `config` does not support rendering to
  pixel buffers (e.g., [`GLX_DRAWABLE_TYPE`] does not contain
  [`GLX_PBUFFER_BIT`]).

## See Also
- [`Gl::x_choose_fb_config`]
- [`Gl::x_make_context_current`]
- [`Gl::x_select_event`]
