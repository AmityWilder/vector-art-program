# gluDeleteTess
destroy a tessellation object

## Parameters
- `tess`
  Specifies the tessellation object to destroy.

## Description
[`Gl::u_delete_tess`] destroys the indicated tessellation object
  (which was created with [`Gl::u_new_tess`]) and frees any memory that
  it used.

## See Also
- [`Gl::u_begin_polygon`]
- [`Gl::u_new_tess`]
- [`Gl::u_tess_callback`]
