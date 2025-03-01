# [`Gl::active_texture`]
select active texture unit

### Parameters
- `texture`
  Specifies which texture unit to make active. The number of texture units is implementation dependent, but must be at least 80. `texture` must be one of [`TEXTURE`]*i*, where *i* ranges from zero to the value of [`MAX_COMBINED_TEXTURE_IMAGE_UNITS`] minus one. The initial value is [`TEXTURE0`].

### Description
[`Gl::active_texture`] selects which texture unit subsequent texture state calls will affect. The number of texture units an implementation supports is implementation dependent, but must be at least 80.

### Errors
- [`INVALID_ENUM`] is generated if `texture` is not one of [`TEXTURE`]*i*, where *i* ranges from zero to the value of [`MAX_COMBINED_TEXTURE_IMAGE_UNITS`] minus one.

### See Also
- [`Gl::gen_textures`]
- [`Gl::bind_texture`]
- [`Gl::compressed_tex_image1d`]
- [`Gl::compressed_tex_image2d`]
- [`Gl::compressed_tex_image3d`]
- [`Gl::compressed_tex_sub_image1d`]
- [`Gl::compressed_tex_sub_image2d`]
- [`Gl::compressed_tex_sub_image3d`]
- [`Gl::copy_tex_image1d`]
- [`Gl::copy_tex_image2d`]
- [`Gl::copy_tex_sub_image1d`]
- [`Gl::copy_tex_sub_image2d`]
- [`Gl::copy_tex_sub_image3d`]
- [`Gl::delete_textures`]
- [`Gl::is_texture`]
- [`Gl::tex_image1d`]
- [`Gl::tex_image2d`]
- [`Gl::tex_image2d_multisample`]
- [`Gl::tex_image3d`]
- [`Gl::tex_image3d_multisample`]
- [`Gl::tex_sub_image1d`]
- [`Gl::tex_sub_image2d`]
- [`Gl::tex_sub_image3d`]
- [`Gl::tex_parameter`]
