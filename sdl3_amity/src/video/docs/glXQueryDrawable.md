# glXQueryDrawable
returns an attribute associated with a GLX drawable

## Parameters
- `dpy`
  Specifies the connection to the X server.

## Description
[`Gl::x_query_drawable`] sets `value` to the value of `attribute` with
  respect to the GLXDrawable `draw`.
`attribute` may be one of the following:
If `draw` is a GLXWindow or GLXPixmap and `attribute` is set to
  [`GLX_PRESERVED_CONTENTS`] or [`GLX_LARGETST_PBUFFER`], the contents
  of `value` are undefined. If `attribute` is not one of the attributes
  listed above, the contents of `value` are unedfined.

## Errors
- A [`GLXBadDrawable`] is generated if `draw` is not a valid
  GLXDrawable.

## See Also
- [`Gl::x_create_window`]
- [`Gl::x_create_pixmap`]
- [`Gl::x_create_pbuffer`]
- [`Gl::x_get_fb_config_attrib`]
- [`Gl::x_choose_fb_config`]
