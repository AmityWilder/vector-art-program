# glCreateTextures
create texture objects

## Parameters
- `target`
  Specifies the effective texture target of each created texture.

## Description
[`Gl::create_textures`] returns `n` previously unused texture names in
  `textures`, each representing a new texture object of the
  dimensionality and type specified by `target` and initialized to the
  default values for that texture type.
`target` must be one of [`gl::TEXTURE_1D`], [`gl::TEXTURE_2D`],
  [`gl::TEXTURE_3D`], [`gl::TEXTURE_1D_ARRAY`],
  [`gl::TEXTURE_2D_ARRAY`], [`gl::TEXTURE_RECTANGLE`],
  [`gl::TEXTURE_CUBE_MAP`], [`gl::TEXTURE_CUBE_MAP_ARRAY`],
  [`gl::TEXTURE_BUFFER`], [`gl::TEXTURE_2D_MULTISAMPLE`] or
  [`gl::TEXTURE_2D_MULTISAMPLE_ARRAY`].

## Errors
- [`gl::INVALID_ENUM`] is generated if `target` is not one of the
  allowable values.
- [`gl::INVALID_VALUE`] is generated if `n` is negative.

## See Also
- [`Gl::bind_texture`]
- [`Gl::delete_textures`]
- [`Gl::gen_textures`]
- [`Gl::get`]
- [`Gl::get_tex_parameter`]
- [`Gl::is_texture`]
- [`Gl::tex_buffer`]
- [`Gl::tex_image1d`]
- [`Gl::tex_image2d`]
- [`Gl::tex_image2d_multisample`]
- [`Gl::tex_image3d`]
- [`Gl::tex_image3d_multisample`]
- [`Gl::tex_parameter`]
