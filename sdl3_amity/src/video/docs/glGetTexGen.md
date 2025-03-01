# glGetTexGen
return texture coordinate generation parameters

## Parameters
- `coord`
  Specifies a texture coordinate. Must be [`gl::S`], [`gl::T`],
  [`gl::R`], or [`gl::Q`].

## Description
[`Gl::get_tex_gen`] returns in `params` selected parameters of a
  texture coordinate generation function that was specified using
  [`Gl::tex_gen`]. `coord` names one of the (*s*, *t*, *r*, *q*) texture
  coordinates, using the symbolic constant [`gl::S`], [`gl::T`],
  [`gl::R`], or [`gl::Q`].
`pname` specifies one of three symbolic names:

## Notes
If an error is generated, no change is made to the contents of
  `params`.
For OpenGL versions 1.3 and greater, or when the ```c ARB_multitexture
  ``` extension is supported, [`Gl::get_tex_gen`] returns the texture
  coordinate generation parameters for the active texture unit.

## Errors
- [`gl::INVALID_ENUM`] is generated if `coord` or `pname` is not an
  accepted value.
- [`gl::INVALID_OPERATION`] is generated if [`Gl::get_tex_gen`] is
  executed between the execution of [`Gl::begin`] and the corresponding
  execution of [`Gl::end`].

## See Also
- [`Gl::active_texture`]
- [`Gl::tex_gen`]
