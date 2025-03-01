# glXGetConfig
return information about GLX visuals

## Parameters
- `dpy`
  Specifies the connection to the X server.

## Description
[`Gl::x_get_config`] sets `value` to the `attrib` value of windows or
  GLX pixmaps created with respect to `vis`. [`Gl::x_get_config`]
  returns an error code if it fails for any reason. Otherwise, zero is
  returned.
`attrib` is one of the following:

The X protocol allows a single visual ID to be instantiated with
  different numbers of bits per pixel. Windows or GLX pixmaps that will
  be rendered with OpenGL, however, must be instantiated with a color
  buffer depth of [`GLX_BUFFER_SIZE`].
Although a GLX implementation can export many visuals that support GL
  rendering, it must support at least one RGBA visual. This visual must
  have at least one color buffer, a stencil buffer of at least 1 bit, a
  depth buffer of at least 12 bits, and an accumulation buffer. Alpha
  bitplanes are optional in this visual. However, its color buffer size
  must be as great as that of the deepest [`TrueColor`],
  [`DirectColor`], [`PseudoColor`], or [`StaticColor`] visual supported
  on level zero, and it must itself be made available on level zero.
In addition, if the X server exports a [`PseudoColor`] or
  [`StaticColor`] visual on framebuffer level 0, a color index visual is
  also required on that level. It must have at least one color buffer, a
  stencil buffer of at least 1 bit, and a depth buffer of at least 12
  bits. This visual must have as many color bitplanes as the deepest
  [`PseudoColor`] or [`StaticColor`] visual supported on level 0.
Applications are best written to select the visual that most closely
  meets their requirements. Creating windows or GLX pixmaps with
  unnecessary buffers can result in reduced rendering performance as
  well as poor resource allocation.

## Notes
[`XVisualInfo`] is defined in *Xutil.h.* It is a structure that
  includes *visual*, *visualID*, *screen*, and *depth* elements.

## Errors
- [`GLX_NO_EXTENSION`] is returned if `dpy` does not support the GLX
  extension.
- [`GLX_BAD_SCREEN`] is returned if the screen of `vis` does not
  correspond to a screen.
- [`GLX_BAD_ATTRIBUTE`] is returned if `attrib` is not a valid GLX
  attribute.
- [`GLX_BAD_VISUAL`] is returned if `vis` doesn't support GLX and an
  attribute other than [`GLX_USE_GL`] is requested.

## See Also
- [`Gl::x_choose_visual`]
- [`Gl::x_create_context`]
