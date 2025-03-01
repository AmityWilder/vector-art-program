# glAreTexturesResident
determine if textures are loaded in texture memory

## Parameters
- `n`
  Specifies the number of textures to be queried.

## Description
GL establishes a ``working set'' of textures that are resident in
  texture memory. These textures can be bound to a texture target much
  more efficiently than textures that are not resident.
[`Gl::are_textures_resident`] queries the texture residence status of
  the `n` textures named by the elements of `textures`. If all the named
  textures are resident, [`Gl::are_textures_resident`] returns
  [`gl::TRUE`], and the contents of `residences` are undisturbed. If not
  all the named textures are resident, [`Gl::are_textures_resident`]
  returns [`gl::FALSE`], and detailed status is returned in the `n`
  elements of `residences`. If an element of `residences` is
  [`gl::TRUE`], then the texture named by the corresponding element of
  `textures` is resident.
The residence status of a single bound texture may also be queried by
  calling [`Gl::get_tex_parameter`] with the *target* argument set to
  the target to which the texture is bound, and the *pname* argument set
  to [`gl::TEXTURE_RESIDENT`]. This is the only way that the residence
  status of a default texture can be queried.

## Notes
[`Gl::are_textures_resident`] is available only if the GL version is
  1.1 or greater.
[`Gl::are_textures_resident`] returns the residency status of the
  textures at the time of invocation. It does not guarantee that the
  textures will remain resident at any other time.
If textures reside in virtual memory (there is no texture memory),
  they are considered always resident.
Some implementations may not load a texture until the first use of
  that texture.

## Errors
- [`gl::INVALID_VALUE`] is generated if `n` is negative.
- [`gl::INVALID_VALUE`] is generated if any element in `textures` is 0
  or does not name a texture. In that case, the function returns
  [`gl::FALSE`] and the contents of `residences` is indeterminate.
- [`gl::INVALID_OPERATION`] is generated if
  [`Gl::are_textures_resident`] is executed between the execution of
  [`Gl::begin`] and the corresponding execution of [`Gl::end`].

## See Also
- [`Gl::bind_texture`]
- [`Gl::get_tex_parameter`]
- [`Gl::prioritize_textures`]
- [`Gl::tex_image1d`]
- [`Gl::tex_image2d`]
- [`Gl::tex_image3d`]
- [`Gl::tex_parameter`]
