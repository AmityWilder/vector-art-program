# glGetColorTable
retrieve contents of a color lookup table

## Parameters
- `target`
  Must be [`gl::COLOR_TABLE`], [`gl::POST_CONVOLUTION_COLOR_TABLE`], or
  [`gl::POST_COLOR_MATRIX_COLOR_TABLE`].

## Description
[`Gl::get_color_table`] returns in `table` the contents of the color
  table specified by `target`. No pixel transfer operations are
  performed, but pixel storage modes that are applicable to
  [`Gl::read_pixels`] are performed.
If a non-zero named buffer object is bound to the
  [`gl::PIXEL_PACK_BUFFER`] target (see [`Gl::bind_buffer`]) while a
  histogram table is requested, `table` is treated as a byte offset into
  the buffer object's data store.
Color components that are requested in the specified `format`, but
  which are not included in the internal format of the color lookup
  table, are returned as zero. The assignments of internal color
  components to the components requested by `format` are


## Notes
[`Gl::get_color_table`] is present only if ```c ARB_imaging ``` is
  returned when [`Gl::get_string`] is called with an argument of
  [`gl::EXTENSIONS`].


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
  name is bound to the [`gl::PIXEL_PACK_BUFFER`] target and `table` is
  not evenly divisible into the number of bytes needed to store in
  memory a datum indicated by `type`.
- [`gl::INVALID_OPERATION`] is generated if [`Gl::get_color_table`] is
  executed between the execution of [`Gl::begin`] and the corresponding
  execution of [`Gl::end`].

## See Also
- [`Gl::color_table`]
- [`Gl::color_table_parameter`]
