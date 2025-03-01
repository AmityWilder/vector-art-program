# glIsTexture
determine if a name corresponds to a texture

## Parameters
- `texture`
  Specifies a value that may be the name of a texture.

## Description
[`Gl::is_texture`] returns [`gl::TRUE`] if `texture` is currently the
  name of a texture. If `texture` is zero, or is a non-zero value that
  is not currently the name of a texture, or if an error occurs,
  [`Gl::is_texture`] returns [`gl::FALSE`].
A name returned by [`Gl::gen_textures`], but not yet associated with a
  texture by calling [`Gl::bind_texture`], is not the name of a texture.

## See Also
- [`Gl::bind_texture`]
- [`Gl::copy_tex_image1d`]
- [`Gl::copy_tex_image2d`]
- [`Gl::delete_textures`]
- [`Gl::gen_textures`]
- [`Gl::get`]
- [`Gl::get_tex_parameter`]
- [`Gl::tex_image1d`]
- [`Gl::tex_image2d`]
- [`Gl::tex_image3d`]
- [`Gl::tex_parameter`]
