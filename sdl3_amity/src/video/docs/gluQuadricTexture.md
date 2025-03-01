# gluQuadricTexture
specify if texturing is desired for quadrics

## Parameters
- `quad`
  Specifies the quadrics object (created with [`Gl::u_new_quadric`]).

## Description
[`Gl::u_quadric_texture`] specifies if texture coordinates should be
  generated for quadrics rendered with `quad`. If the value of `texture`
  is [`GLU_TRUE`], then texture coordinates are generated, and if
  `texture` is [`GLU_FALSE`], they are not. The initial value is
  [`GLU_FALSE`].
The manner in which texture coordinates are generated depends upon the
  specific quadric rendered.

## See Also
- [`Gl::u_new_quadric`]
- [`Gl::u_quadric_draw_style`]
- [`Gl::u_quadric_normals`]
- [`Gl::u_quadric_orientation`]
