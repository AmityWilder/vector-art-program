# glXGetFBConfigs
list all GLX frame buffer configurations for a given screen

## Parameters
- `dpy`
  Specifies the connection to the X server.

## Description
[`Gl::x_get_fb_configs`] returns a list of all GLXFBConfigs available
  on the screen specified by `screen`. Use
  [`Gl::x_get_fb_config_attrib`] to obtain attribute values from a
  specific GLXFBConfig.

## Notes
[`Gl::x_get_fb_configs`] is available only if the GLX version is 1.3
  or greater.
If the GLX version is 1.1 or 1.0, the GL version must be 1.0. If the
  GLX version is 1.2, then the GL version must be 1.1. If the GLX
  version is 1.3, then the GL version must be 1.2.

## See Also
- [`Gl::x_get_fb_config_attrib`]
- [`Gl::x_get_visual_from_fb_config`]
- [`Gl::x_choose_fb_config`]
