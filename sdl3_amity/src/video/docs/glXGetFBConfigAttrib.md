# glXGetFBConfigAttrib
return information about a GLX frame buffer configuration

## Parameters
- `dpy`
  Specifies the connection to the X server.

## Description
[`Gl::x_get_fb_config_attrib`] sets `value` to the `attribute` value
  of GLX drawables created with respect to `config`.
  [`Gl::x_get_fb_config_attrib`] returns an error code if it fails for
  any reason. Otherwise, [`Success`] is returned.
`attribute` is one of the following:

Applications should choose the frame buffer configuration that most
  closely meets their requirements. Creating windows, GLX pixmaps, or
  GLX pixel buffers with unnecessary buffers can result in reduced
  rendering performance as well as poor resource allocation.

## Notes
[`Gl::x_get_fb_config_attrib`] is available only if the GLX version is
  1.3 or greater.
If the GLX version is 1.1 or 1.0, the GL version must be 1.0. If the
  GLX version is 1.2, then the GL version must be 1.1. If the GLX
  version is 1.3, then the GL version must be 1.2.

## Errors
- [`GLX_NO_EXTENSION`] is returned if `dpy` does not support the GLX
  extension. [`GLX_BAD_ATTRIBUTE`] is returned if `attribute` is not a
  valid GLX attribute.

## See Also
- [`Gl::x_get_fb_configs`]
- [`Gl::x_choose_fb_config`]
- [`Gl::x_get_visual_from_fb_config`]
- [`Gl::x_get_config`]
