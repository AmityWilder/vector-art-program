# glGetSeparableFilter
get separable convolution filter kernel images

## Parameters
- `target`
  The separable filter to be retrieved. Must be [`gl::SEPARABLE_2D`].

## Description
[`Gl::get_separable_filter`] returns the two one-dimensional filter
  kernel images for the current separable 2D convolution filter. The row
  image is placed in `row` and the column image is placed in `column`
  according to the specifications in `format` and `type`. (In the
  current implementation, `span` is not affected in any way.) No pixel
  transfer operations are performed on the images, but the relevant
  pixel storage modes are applied.
If a non-zero named buffer object is bound to the
  [`gl::PIXEL_PACK_BUFFER`] target (see [`Gl::bind_buffer`]) while a
  separable convolution filter is requested, `row`, `column`, and `span`
  are treated as a byte offset into the buffer object's data store.
Color components that are present in `format` but not included in the
  internal format of the filters are returned as zero. The assignments
  of internal color components to the components of `format` are as
  follows:



## Notes
[`Gl::get_separable_filter`] is present only if ```c ARB_imaging ```
  is returned when [`Gl::get_string`] is called with an argument of
  [`gl::EXTENSIONS`].
Non-separable 2D filters must be retrieved with
  [`Gl::get_convolution_filter`].

## Errors
- [`gl::INVALID_ENUM`] is generated if `target` is not
  [`gl::SEPARABLE_2D`].
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
  name is bound to the [`gl::PIXEL_PACK_BUFFER`] target and `row` or
  `column` is not evenly divisible into the number of bytes needed to
  store in memory a datum indicated by `type`.
- [`gl::INVALID_OPERATION`] is generated if [`Gl::get_separable_filter`]
  is executed between the execution of [`Gl::begin`] and the
  corresponding execution of [`Gl::end`].

## See Also
- [`Gl::get_convolution_filter`]
- [`Gl::convolution_parameter`]
- [`Gl::separable_filter2d`]
