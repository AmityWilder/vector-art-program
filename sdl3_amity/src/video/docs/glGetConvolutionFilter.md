# glGetConvolutionFilter
get current 1D or 2D convolution filter kernel

## Parameters
- `target`
  The filter to be retrieved. Must be one of [`gl::CONVOLUTION_1D`] or
  [`gl::CONVOLUTION_2D`].

## Description
[`Gl::get_convolution_filter`] returns the current 1D or 2D
  convolution filter kernel as an image. The one- or two-dimensional
  image is placed in `image` according to the specifications in `format`
  and `type`. No pixel transfer operations are performed on this image,
  but the relevant pixel storage modes are applied.
If a non-zero named buffer object is bound to the
  [`gl::PIXEL_PACK_BUFFER`] target (see [`Gl::bind_buffer`]) while a
  convolution filter is requested, `image` is treated as a byte offset
  into the buffer object's data store.
Color components that are present in `format` but not included in the
  internal format of the filter are returned as zero. The assignments of
  internal color components to the components of `format` are as
  follows.


## Notes
[`Gl::get_convolution_filter`] is present only if ```c ARB_imaging ```
  is returned when [`Gl::get_string`] is called with an argument of
  [`gl::EXTENSIONS`].
The current separable 2D filter must be retrieved with
  [`Gl::get_separable_filter`] rather than
  [`Gl::get_convolution_filter`].

## Errors
- [`gl::INVALID_ENUM`] is generated if `target` is not one of the
  allowable values.
- [`gl::INVALID_ENUM`] is generated if `format` is not one of the
  allowable values.
- [`gl::INVALID_ENUM`] is generated if `type` is not one of the
  allowable values.
- [`gl::INVALID_OPERATION`] is generated if `type` is one of
  [`gl::UNSIGNED_BYTE_3_3_2`], [`gl::UNSIGNED_BYTE_2_3_3_REV`],
  [`gl::UNSIGNED_SHORT_5_6_5`], or [`gl::UNSIGNED_SHORT_5_6_5_REV`] and
  `format` is not [`gl::RGB`].
- [`gl::INVALID_OPERATION`] is generated if `type` is one of
  [`gl::UNSIGNED_SHORT_4_4_4_4`], [`gl::UNSIGNED_SHORT_4_4_4_4_REV`],
  [`gl::UNSIGNED_SHORT_5_5_5_1`], [`gl::UNSIGNED_SHORT_1_5_5_5_REV`],
  [`gl::UNSIGNED_INT_8_8_8_8`], [`gl::UNSIGNED_INT_8_8_8_8_REV`],
  [`gl::UNSIGNED_INT_10_10_10_2`], or
  [`gl::UNSIGNED_INT_2_10_10_10_REV`] and `format` is neither
  [`gl::RGBA`] nor [`gl::BGRA`].
- [`gl::INVALID_OPERATION`] is generated if a non-zero buffer object
  name is bound to the [`gl::PIXEL_PACK_BUFFER`] target and the buffer
  object's data store is currently mapped.
- [`gl::INVALID_OPERATION`] is generated if a non-zero buffer object
  name is bound to the [`gl::PIXEL_PACK_BUFFER`] target and the data
  would be packed to the buffer object such that the memory writes
  required would exceed the data store size.
- [`gl::INVALID_OPERATION`] is generated if a non-zero buffer object
  name is bound to the [`gl::PIXEL_PACK_BUFFER`] target and `image` is
  not evenly divisible into the number of bytes needed to store in
  memory a datum indicated by `type`.
- [`gl::INVALID_OPERATION`] is generated if
  [`Gl::get_convolution_filter`] is executed between the execution of
  [`Gl::begin`] and the corresponding execution of [`Gl::end`].

## See Also
- [`Gl::get_separable_filter`]
- [`Gl::convolution_parameter`]
- [`Gl::convolution_filter1d`]
- [`Gl::convolution_filter2d`]
