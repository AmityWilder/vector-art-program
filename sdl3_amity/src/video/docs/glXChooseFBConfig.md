# glXChooseFBConfig
return a list of GLX frame buffer configurations that match the
  specified attributes

## Parameters
- `dpy`
  Specifies the connection to the X server.

## Description
[`Gl::x_choose_fb_config`] returns GLX frame buffer configurations
  that match the attributes specified in `attrib_list`, or [`NULL`] if
  no matches are found. If `attrib_list` is [`NULL`], then
  [`Gl::x_choose_fb_config`] returns an array of GLX frame buffer
  configurations that are available on the specified screen. If an error
  occurs, no frame buffer configurations exist on the specified screen,
  or if no frame buffer configurations match the specified attributes,
  then [`NULL`] is returned. Use [`Gl::x_free`] to free the memory
  returned by [`Gl::x_choose_fb_config`].
All attributes in `attrib_list`, including boolean attributes, are
  immediately followed by the corresponding desired value. The list is
  terminated with [`None`]. If an attribute is not specified in
  `attrib_list`, then the default value (see below) is used (and the
  attribute is said to be specified implicitly). For example, if
  [`GLX_STEREO`] is not specified, then it is assumed to be [`False`].
  For some attributes, the default is [`GLX_DONT_CARE`], meaning that
  any value is OK for this attribute, so the attribute will not be
  checked.
Attributes are matched in an attribute-specific manner. Some of the
  attributes, such as [`GLX_LEVEL`], must match the specified value
  exactly; others, such as, [`GLX_RED_SIZE`] must meet or exceed the
  specified minimum values. If more than one GLX frame buffer
  configuration is found, then a list of configurations, sorted
  according to the ``best'' match criteria, is returned. The match
  criteria for each attribute and the exact sorting order is defined
  below.
The interpretations of the various GLX visual attributes are as
  follows:
When more than one GLX frame buffer configuration matches the
  specified attributes, a list of matching configurations is returned.
  The list is sorted according to the following precedence rules, which
  are applied in ascending order (i.e., configurations that are
  considered equal by a lower numbered rule are sorted by the higher
  numbered rule):

## Examples

Specifies a frame buffer configuration that supports RGBA rendering
  and exists in the normal frame buffer, not an overlay or underlay
  buffer. The returned visual supports at least four bits each of red,
  green, and blue, and possibly no bits of alpha. It does not support
  stereo display. It may or may not have one or more auxiliary color
  buffers, a back buffer, a depth buffer, a stencil buffer, or an
  accumulation buffer.

## Notes
[`Gl::x_choose_fb_config`] is available only if the GLX version is 1.3
  or greater.
If the GLX version is 1.1 or 1.0, the GL version must be 1.0. If the
  GLX version is 1.2, then the GL version must be 1.1. If the GLX
  version is 1.3, then the GL version must be 1.2.
[`Gl::x_get_fb_configs`] and [`Gl::x_get_fb_config_attrib`] can be
  used to implement selection algorithms other than the generic one
  implemented by [`Gl::x_choose_fb_config`]. Call
  [`Gl::x_choose_fb_config`] to retrieve all the frame buffer
  configurations on a particular screen or, alternatively, all the frame
  buffer configurations with a particular set of attributes. Next, call
  [`Gl::x_get_fb_config_attrib`] to retrieve additional attributes for
  the frame buffer configurations and then select between them.
GLX implementations are strongly discouraged, but not proscribed, from
  changing the selection algorithm used by [`Gl::x_choose_fb_config`].
  Therefore, selections may change from release to release of the
  client-side library.

## Errors
- [`NULL`] is returned if an undefined GLX attribute is encountered in
  `attrib_list`, if `screen` is invalid, or if `dpy` does not support
  the GLX extension.

## See Also
- [`Gl::x_get_fb_config_attrib`]
- [`Gl::x_get_fb_configs`]
- [`Gl::x_get_visual_from_fb_config`]
