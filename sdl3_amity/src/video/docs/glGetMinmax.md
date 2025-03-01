# glGetMinmax
get minimum and maximum pixel values

## Parameters
- `target`
  Must be [`gl::MINMAX`].

## Description
[`Gl::get_minmax`] returns the accumulated minimum and maximum pixel
  values (computed on a per-component basis) in a one-dimensional image
  of width 2. The first set of return values are the minima, and the
  second set of return values are the maxima. The format of the return
  values is determined by `format`, and their type is determined by
  `types`.
If a non-zero named buffer object is bound to the
  [`gl::PIXEL_PACK_BUFFER`] target (see [`Gl::bind_buffer`]) while
  minimum and maximum pixel values are requested, `values` is treated as
  a byte offset into the buffer object's data store.
No pixel transfer operations are performed on the return values, but
  pixel storage modes that are applicable to one-dimensional images are
  performed. Color components that are requested in the specified
  `format`, but that are not included in the internal format of the
  minmax table, are returned as zero. The assignment of internal color
  components to the components requested by `format` are as follows:

If `reset` is [`gl::TRUE`], the minmax table entries corresponding to
  the return values are reset to their initial values. Minimum and
  maximum values that are not returned are not modified, even if `reset`
  is [`gl::TRUE`].

## Notes
[`Gl::get_minmax`] is present only if ```c ARB_imaging ``` is returned
  when [`Gl::get_string`] is called with an argument of
  [`gl::EXTENSIONS`].

## Errors
- [`gl::INVALID_ENUM`] is generated if `target` is not [`gl::MINMAX`].
- [`gl::INVALID_ENUM`] is generated if `format` is not one of the
  allowable values.
- [`gl::INVALID_ENUM`] is generated if `types` is not one of the
  allowable values.
- [`gl::INVALID_OPERATION`] is generated if `types` is one of
  [`gl::UNSIGNED_BYTE_3_3_2`], [`gl::UNSIGNED_BYTE_2_3_3_REV`],
  [`gl::UNSIGNED_SHORT_5_6_5`], or [`gl::UNSIGNED_SHORT_5_6_5_REV`] and
  `format` is not [`gl::RGB`].
- [`gl::INVALID_OPERATION`] is generated if `types` is one of
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
  name is bound to the [`gl::PIXEL_PACK_BUFFER`] target and `values` is
  not evenly divisible into the number of bytes needed to store in
  memory a datum indicated by `type`.
- [`gl::INVALID_OPERATION`] is generated if [`Gl::get_minmax`] is
  executed between the execution of [`Gl::begin`] and the corresponding
  execution of [`Gl::end`].

## See Also
- [`Gl::minmax`]
- [`Gl::reset_minmax`]
