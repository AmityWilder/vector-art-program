# glGenTextures
generate texture names

## Parameters
- `n`
  Specifies the number of texture names to be generated.

## Description
[`Gl::gen_textures`] returns `n` texture names in `textures`. There is
  no guarantee that the names form a contiguous set of integers;
  however, it is guaranteed that none of the returned names was in use
  immediately before the call to [`Gl::gen_textures`].
The generated textures have no dimensionality; they assume the
  dimensionality of the texture target to which they are first bound
  (see [`Gl::bind_texture`]).
Texture names returned by a call to [`Gl::gen_textures`] are not
  returned by subsequent calls, unless they are first deleted with
  [`Gl::delete_textures`].

## Errors
- [`gl::INVALID_VALUE`] is generated if `n` is negative.

## See Also
- [`Gl::bind_texture`]
- [`Gl::copy_tex_image1d`]
- [`Gl::copy_tex_image2d`]
- [`Gl::delete_textures`]
- [`Gl::get`]
- [`Gl::get_tex_parameter`]
- [`Gl::tex_image1d`]
- [`Gl::tex_image2d`]
- [`Gl::tex_image3d`]
- [`Gl::tex_parameter`]
