# glClampColor
specify whether data read via

## Parameters
- `target`
  Target for color clamping. `target` must be [`gl::CLAMP_READ_COLOR`].

## Description
[`Gl::clamp_color`] controls color clamping that is performed during
  [`Gl::read_pixels`]. `target` must be [`gl::CLAMP_READ_COLOR`]. If
  `clamp` is [`gl::TRUE`], read color clamping is enabled; if `clamp` is
  [`gl::FALSE`], read color clamping is disabled. If `clamp` is
  [`gl::FIXED_ONLY`], read color clamping is enabled only if the
  selected read buffer has fixed point components and disabled
  otherwise.

## Errors
- [`gl::INVALID_ENUM`] is generated if `target` is not
  [`gl::CLAMP_READ_COLOR`].
- [`gl::INVALID_ENUM`] is generated if `clamp` is not [`gl::TRUE`] or
  [`gl::FALSE`].

## See Also
