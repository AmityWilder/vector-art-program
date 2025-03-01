# glPointParameter
specify point parameters

## Parameters
- `pname`
  Specifies a single-valued point parameter.
  [`gl::POINT_FADE_THRESHOLD_SIZE`], and
  [`gl::POINT_SPRITE_COORD_ORIGIN`] are accepted.

## Description
The following values are accepted for `pname`:

## Errors
- [`gl::INVALID_VALUE`] is generated if the value specified for
  [`gl::POINT_FADE_THRESHOLD_SIZE`] is less than zero.
- [`gl::INVALID_ENUM`] is generated If the value specified for
  [`gl::POINT_SPRITE_COORD_ORIGIN`] is not [`gl::LOWER_LEFT`] or
  [`gl::UPPER_LEFT`].

## See Also
- [`Gl::point_size`]
