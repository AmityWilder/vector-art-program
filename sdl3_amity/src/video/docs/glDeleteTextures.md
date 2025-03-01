# glDeleteTextures
delete named textures

## Parameters
- `n`
  Specifies the number of textures to be deleted.

## Description
[`Gl::delete_textures`] deletes `n` textures named by the elements of
  the array `textures`. After a texture is deleted, it has no contents
  or dimensionality, and its name is free for reuse (for example by
  [`Gl::gen_textures`]). If a texture that is currently bound is
  deleted, the binding reverts to 0 (the default texture).
[`Gl::delete_textures`] silently ignores 0's and names that do not
  correspond to existing textures.

## Errors
- [`gl::INVALID_VALUE`] is generated if `n` is negative.

## See Also
- [`Gl::bind_texture`]
- [`Gl::copy_tex_image1d`]
- [`Gl::copy_tex_image2d`]
- [`Gl::gen_textures`]
- [`Gl::get`]
- [`Gl::get_tex_parameter`]
- [`Gl::tex_image1d`]
- [`Gl::tex_image2d`]
- [`Gl::tex_parameter`]
