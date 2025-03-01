# gluQuadricOrientation
specify inside/outside orientation for quadrics

## Parameters
- `quad`
  Specifies the quadrics object (created with [`Gl::u_new_quadric`]).

## Description
[`Gl::u_quadric_orientation`] specifies what kind of orientation is
  desired for quadrics rendered with `quad`. The `orientation` values
  are as follows:
Note that the interpretation of *outward* and *inward* depends on the
  quadric being drawn.

## See Also
- [`Gl::u_new_quadric`]
- [`Gl::u_quadric_draw_style`]
- [`Gl::u_quadric_normals`]
- [`Gl::u_quadric_texture`]
