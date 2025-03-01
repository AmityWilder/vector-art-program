# glPixelStore
set pixel storage modes

## Parameters
- `pname`
  Specifies the symbolic name of the parameter to be set. Six values
  affect the packing of pixel data into memory: [`gl::PACK_SWAP_BYTES`],
  [`gl::PACK_LSB_FIRST`], [`gl::PACK_ROW_LENGTH`],
  [`gl::PACK_IMAGE_HEIGHT`], [`gl::PACK_SKIP_PIXELS`],
  [`gl::PACK_SKIP_ROWS`], [`gl::PACK_SKIP_IMAGES`], and
  [`gl::PACK_ALIGNMENT`]. Six more affect the unpacking of pixel data
  *from* memory: [`gl::UNPACK_SWAP_BYTES`], [`gl::UNPACK_LSB_FIRST`],
  [`gl::UNPACK_ROW_LENGTH`], [`gl::UNPACK_IMAGE_HEIGHT`],
  [`gl::UNPACK_SKIP_PIXELS`], [`gl::UNPACK_SKIP_ROWS`],
  [`gl::UNPACK_SKIP_IMAGES`], and [`gl::UNPACK_ALIGNMENT`].

## Description
[`Gl::pixel_store`] sets pixel storage modes that affect the operation
  of subsequent [`Gl::read_pixels`] as well as the unpacking of texture
  patterns (see [`Gl::tex_image1d`], [`Gl::tex_image2d`],
  [`Gl::tex_image3d`], [`Gl::tex_sub_image1d`], [`Gl::tex_sub_image2d`],
  [`Gl::tex_sub_image3d`]), [`Gl::compressed_tex_image1d`],
  [`Gl::compressed_tex_image2d`], [`Gl::compressed_tex_image3d`],
  [`Gl::compressed_tex_sub_image1d`], [`Gl::compressed_tex_sub_image2d`]
  or [`Gl::compressed_tex_sub_image1d`].
`pname` is a symbolic constant indicating the parameter to be set, and
  `param` is the new value. Six of the twelve storage parameters affect
  how pixel data is returned to client memory. They are as follows:
The other six of the twelve storage parameters affect how pixel data
  is read from client memory. These values are significant for
  [`Gl::tex_image1d`], [`Gl::tex_image2d`], [`Gl::tex_image3d`],
  [`Gl::tex_sub_image1d`], [`Gl::tex_sub_image2d`], and
  [`Gl::tex_sub_image3d`]
They are as follows:
The following table gives the type, initial value, and range of valid
  values for each storage parameter that can be set with
  [`Gl::pixel_store`].

[`Gl::pixel_storef`] can be used to set any pixel store parameter. If
  the parameter type is boolean, then if `param` is 0, the parameter is
  false; otherwise it is set to true. If `pname` is an integer type
  parameter, `param` is rounded to the nearest integer.
Likewise, [`Gl::pixel_storei`] can also be used to set any of the
  pixel store parameters. Boolean parameters are set to false if `param`
  is 0 and true otherwise.

## Errors
- [`gl::INVALID_ENUM`] is generated if `pname` is not an accepted value.
- [`gl::INVALID_VALUE`] is generated if a negative row length, pixel
  skip, or row skip value is specified, or if alignment is specified as
  other than 1, 2, 4, or 8.

## See Also
- [`Gl::read_pixels`]
- [`Gl::tex_image1d`]
- [`Gl::tex_image2d`]
- [`Gl::tex_image3d`]
- [`Gl::tex_sub_image1d`]
- [`Gl::tex_sub_image2d`]
- [`Gl::tex_sub_image3d`]
- [`Gl::compressed_tex_image1d`]
- [`Gl::compressed_tex_image2d`]
- [`Gl::compressed_tex_image3d`]
- [`Gl::compressed_tex_sub_image1d`]
- [`Gl::compressed_tex_sub_image2d`]
- [`Gl::compressed_tex_sub_image1d`]
