# glXChooseVisual
return a visual that matches specified attributes

## Parameters
- `dpy`
  Specifies the connection to the X server.

## Description
[`Gl::x_choose_visual`] returns a pointer to an XVisualInfo structure
  describing the visual that best meets a minimum specification. The
  boolean GLX attributes of the visual that is returned will match the
  specified values, and the integer GLX attributes will meet or exceed
  the specified minimum values. If all other attributes are equivalent,
  then TrueColor and PseudoColor visuals have priority over DirectColor
  and StaticColor visuals, respectively. If no conforming visual exists,
  [`NULL`] is returned. To free the data returned by this function, use
  [`Gl::x_free`].
All boolean GLX attributes default to [`False`] except [`GLX_USE_GL`],
  which defaults to [`True`]. All integer GLX attributes default to
  zero. Default specifications are superseded by attributes included in
  `attribList`. Boolean attributes included in `attribList` are
  understood to be [`True`]. Integer attributes and enumerated type
  attributes are followed immediately by the corresponding desired or
  minimum value. The list must be terminated with [`None`].
The interpretations of the various GLX visual attributes are as
  follows:

## Examples

Specifies a single-buffered RGB visual in the normal frame buffer, not
  an overlay or underlay buffer. The returned visual supports at least
  four bits each of red, green, and blue, and possibly no bits of alpha.
  It does not support color index mode, double-buffering, or stereo
  display. It may or may not have one or more auxiliary color buffers, a
  depth buffer, a stencil buffer, or an accumulation buffer.

## Notes
[`XVisualInfo`] is defined in *Xutil.h.* It is a structure that
  includes *visual*, *visualID*, *screen*, and *depth* elements.
[`Gl::x_choose_visual`] is implemented as a client-side utility using
  only [`Gl::x_get_visual_info`] and [`Gl::x_get_config`]. Calls to
  these two routines can be used to implement selection algorithms other
  than the generic one implemented by [`Gl::x_choose_visual`].
GLX implementations are strongly discouraged, but not proscribed, from
  changing the selection algorithm used by [`Gl::x_choose_visual`].
  Therefore, selections may change from release to release of the
  client-side library.
There is no direct filter for picking only visuals that support
  GLXPixmaps. GLXPixmaps are supported for visuals whose
  [`GLX_BUFFER_SIZE`] is one of the pixmap depths supported by the X
  server.

## Errors
- [`NULL`] is returned if an undefined GLX attribute is encountered in
  `attribList`.

## See Also
- [`Gl::x_create_context`]
- [`Gl::x_get_config`]
