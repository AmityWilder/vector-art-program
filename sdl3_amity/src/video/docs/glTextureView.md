# glTextureView
initialize a texture as a data alias of another texture's data store

## Parameters
- `texture`
  Specifies the texture object to be initialized as a view.

## Description
[`Gl::texture_view`] initializes a texture object as an alias, or view
  of another texture object, sharing some or all of the parent texture's
  data store with the initialized texture. `texture` specifies a name
  previously reserved by a successful call to [`Gl::gen_textures`] but
  that has not yet been bound or given a target. `target` specifies the
  target for the newly initialized texture and must be compatible with
  the target of the parent texture, given in `origtexture` as specified
  in the following table:
The value of [`gl::TEXTURE_IMMUTABLE_FORMAT`] for `origtexture` must
  be [`gl::TRUE`]. After initialization, `texture` inherits the data
  store of the parent texture, `origtexture` and is usable as a normal
  texture object with target `target`. Data in the shared store is
  reinterpreted with the new internal format specified by
  `internalformat`. `internalformat` must be compatible with the
  internal format of the parent texture as specified in the following
  table:
If the original texture is an array or has multiple mipmap levels, the
  parameters `minlayer`, `numlayers`, `minlevel`, and `numlevels`
  control which of those slices and levels are considered part of the
  texture. The `minlevel` and `minlayer` parameters are relative to the
  view of the original texture. If `numlayers` or `numlevels` extend
  beyond the original texture, they are clamped to the max extent of the
  original texture.
If the new texture's target is [`gl::TEXTURE_CUBE_MAP`], the clamped
  `numlayers` must be equal to 6. If the new texture's target is
  [`gl::TEXTURE_CUBE_MAP_ARRAY`], then `numlayers` counts layer-faces
  rather than layers, and the clamped `numlayers` must be a multiple of
  6. If the new texture's target is [`gl::TEXTURE_CUBE_MAP`] or
  [`gl::TEXTURE_CUBE_MAP_ARRAY`], the width and height of the original
  texture's levels must be equal.
When the original texture's target is [`gl::TEXTURE_CUBE_MAP`], the
  layer parameters are interpreted in the same order as if it were a
  [`gl::TEXTURE_CUBE_MAP_ARRAY`] with 6 layer-faces.
If `target` is [`gl::TEXTURE_1D`], [`gl::TEXTURE_2D`],
  [`gl::TEXTURE_3D`], [`gl::TEXTURE_RECTANGLE`], or
  [`gl::TEXTURE_2D_MULTISAMPLE`], `numlayers` must equal 1.
The dimensions of the original texture must be less than or equal to
  the maximum supported dimensions of the new target. For example, if
  the original texture has a [`gl::TEXTURE_2D_ARRAY`] target and its
  width is greater than [`gl::MAX_CUBE_MAP_TEXTURE_SIZE`], an error will
  be generated if [`Gl::texture_view`] is called to create a
  [`gl::TEXTURE_CUBE_MAP`] view.
Texture commands that take a `level` or `layer` parameter, such as
  [`Gl::tex_sub_image2d`], interpret that parameter to be relative to
  the view of the texture. i.e. the mipmap level of the data store that
  would be updated via [`Gl::tex_sub_image2d`] would be the sum of
  `level` and the value of [`gl::TEXTURE_VIEW_MIN_LEVEL`].

## Errors
- [`gl::INVALID_VALUE`] is generated if `minlayer` or `minlevel` are
  larger than the greatest layer or level of `origtexture`.
- [`gl::INVALID_OPERATION`] is generated if `target` is not compatible
  with the target of `origtexture`.
- [`gl::INVALID_OPERATION`] is generated if the dimensions of
  `origtexture` are greater than the maximum supported dimensions for
  `target`.
- [`gl::INVALID_OPERATION`] is generated if `internalformat` is not
  compatible with the internal format of `origtexture`.
- [`gl::INVALID_OPERATION`] is generated if `texture` has already been
  bound or otherwise given a target.
- [`gl::INVALID_OPERATION`] is generated if the value of
  [`gl::TEXTURE_IMMUTABLE_FORMAT`] for `origtexture` is not
  [`gl::TRUE`].
- [`gl::INVALID_OPERATION`] is generated if `origtexture` is not the
  name of an existing texture object.
- [`gl::INVALID_VALUE`] is generaged if `target` is
  [`gl::TEXTURE_CUBE_MAP`] and `numlayers` is not 6, or if `target` is
  [`gl::TEXTURE_CUBE_MAP_ARRAY`] and `numlayers` is not an integer
  multiple of 6.
- [`gl::INVALID_VALUE`] is generated if `target` is [`gl::TEXTURE_1D`],
  [`gl::TEXTURE_2D`], [`gl::TEXTURE_3D`], [`gl::TEXTURE_RECTANGLE`], or
  [`gl::TEXTURE_2D_MULTISAMPLE`] and `numlayers` does not equal 1.
- [`gl::INVALID_VALUE`] is generated if `texture` zero or is not the
  name of a texture previously returned from a successful call to
  [`Gl::gen_textures`].

## See Also
- [`Gl::tex_storage1d`]
- [`Gl::tex_storage2d`]
- [`Gl::tex_storage3d`]
- [`Gl::get_tex_parameter`]
