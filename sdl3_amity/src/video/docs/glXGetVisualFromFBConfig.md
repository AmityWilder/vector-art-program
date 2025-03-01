# glXGetVisualFromFBConfig
return visual that is associated with the frame buffer configuration

## Parameters
- `dpy`
  Specifies the connection to the X server.

## Description
If `config` is a valid GLX frame buffer configuration and it has an
  associated X Visual, then information describing that visual is
  returned; otherwise [`NULL`] is returned. Use [`Gl::x_free`] to free
  the data returned.

## Notes
[`Gl::x_get_visual_from_fb_config`] is available only if the GLX
  version is 1.3 or greater.
If the GLX version is 1.1 or 1.0, the GL version must be 1.0. If the
  GLX version is 1.2, then the GL version must be 1.1. If the GLX
  version is 1.3, then the GL version must be 1.2.
[`XVisualInfo`] is defined in *Xutil.h.* It is a structure that
  includes *visual*, *visualID*, *screen*, and *depth* elements.

## Errors
- Returns [`NULL`] if `config` is not a valid GLXFBConfig.

## See Also
- [`Gl::x_get_fb_config_attrib`]
- [`Gl::x_choose_fb_config`]
- [`Gl::x_choose_visual`]
- [`Gl::x_get_config`]
