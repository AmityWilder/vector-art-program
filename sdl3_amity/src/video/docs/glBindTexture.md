# glBindTexture
bind a named texture to a texturing target

## Parameters
- `target`
  Specifies the target to which the texture is bound. Must be one of
  [`gl::TEXTURE_1D`], [`gl::TEXTURE_2D`], [`gl::TEXTURE_3D`],
  [`gl::TEXTURE_1D_ARRAY`], [`gl::TEXTURE_2D_ARRAY`],
  [`gl::TEXTURE_RECTANGLE`], [`gl::TEXTURE_CUBE_MAP`],
  [`gl::TEXTURE_CUBE_MAP_ARRAY`], [`gl::TEXTURE_BUFFER`],
  [`gl::TEXTURE_2D_MULTISAMPLE`] or
  [`gl::TEXTURE_2D_MULTISAMPLE_ARRAY`].

## Description
[`Gl::bind_texture`] lets you create or use a named texture. Calling
  [`Gl::bind_texture`] with `target` set to [`gl::TEXTURE_1D`],
  [`gl::TEXTURE_2D`], [`gl::TEXTURE_3D`], [`gl::TEXTURE_1D_ARRAY`],
  [`gl::TEXTURE_2D_ARRAY`], [`gl::TEXTURE_RECTANGLE`],
  [`gl::TEXTURE_CUBE_MAP`], [`gl::TEXTURE_CUBE_MAP_ARRAY`],
  [`gl::TEXTURE_BUFFER`], [`gl::TEXTURE_2D_MULTISAMPLE`] or
  [`gl::TEXTURE_2D_MULTISAMPLE_ARRAY`] and `texture` set to the name of
  the new texture binds the texture name to the target. When a texture
  is bound to a target, the previous binding for that target is
  automatically broken.
Texture names are unsigned integers. The value zero is reserved to
  represent the default texture for each texture target. Texture names
  and the corresponding texture contents are local to the shared object
  space of the current GL rendering context; two rendering contexts
  share texture names only if they explicitly enable sharing between
  contexts through the appropriate GL windows interfaces functions.
You must use [`Gl::gen_textures`] to generate a set of new texture
  names.
When a texture is first bound, it assumes the specified target: A
  texture first bound to [`gl::TEXTURE_1D`] becomes one-dimensional
  texture, a texture first bound to [`gl::TEXTURE_2D`] becomes two-
  dimensional texture, a texture first bound to [`gl::TEXTURE_3D`]
  becomes three-dimensional texture, a texture first bound to
  [`gl::TEXTURE_1D_ARRAY`] becomes one-dimensional array texture, a
  texture first bound to [`gl::TEXTURE_2D_ARRAY`] becomes two-
  dimensional array texture, a texture first bound to
  [`gl::TEXTURE_RECTANGLE`] becomes rectangle texture, a texture first
  bound to [`gl::TEXTURE_CUBE_MAP`] becomes a cube-mapped texture, a
  texture first bound to [`gl::TEXTURE_CUBE_MAP_ARRAY`] becomes a cube-
  mapped array texture, a texture first bound to [`gl::TEXTURE_BUFFER`]
  becomes a buffer texture, a texture first bound to
  [`gl::TEXTURE_2D_MULTISAMPLE`] becomes a two-dimensional multisampled
  texture, and a texture first bound to
  [`gl::TEXTURE_2D_MULTISAMPLE_ARRAY`] becomes a two-dimensional
  multisampled array texture. The state of a one-dimensional texture
  immediately after it is first bound is equivalent to the state of the
  default [`gl::TEXTURE_1D`] at GL initialization, and similarly for the
  other texture types.
While a texture is bound, GL operations on the target to which it is
  bound affect the bound texture, and queries of the target to which it
  is bound return state from the bound texture. In effect, the texture
  targets become aliases for the textures currently bound to them, and
  the texture name zero refers to the default textures that were bound
  to them at initialization.
A texture binding created with [`Gl::bind_texture`] remains active
  until a different texture is bound to the same target, or until the
  bound texture is deleted with [`Gl::delete_textures`].
Once created, a named texture may be re-bound to its same original
  target as often as needed. It is usually much faster to use
  [`Gl::bind_texture`] to bind an existing named texture to one of the
  texture targets than it is to reload the texture image using
  [`Gl::tex_image1d`], [`Gl::tex_image2d`], [`Gl::tex_image3d`] or
  another similar function.

## Notes
The [`gl::TEXTURE_2D_MULTISAMPLE`] and
  [`gl::TEXTURE_2D_MULTISAMPLE_ARRAY`] targets are available only if the
  GL version is 3.2 or higher.

## Errors
- [`gl::INVALID_ENUM`] is generated if `target` is not one of the
  allowable values.
- [`gl::INVALID_VALUE`] is generated if `texture` is not a name returned
  from a previous call to [`Gl::gen_textures`].
- [`gl::INVALID_OPERATION`] is generated if `texture` was previously
  created with a target that doesn't match that of `target`.

## See Also
- [`Gl::delete_textures`]
- [`Gl::gen_textures`]
- [`Gl::get`]
- [`Gl::get_tex_parameter`]
- [`Gl::is_texture`]
- [`Gl::tex_image1d`]
- [`Gl::tex_image2d`]
- [`Gl::tex_image2d_multisample`]
- [`Gl::tex_image3d`]
- [`Gl::tex_image3d_multisample`]
- [`Gl::tex_buffer`]
- [`Gl::tex_parameter`]
