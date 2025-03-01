# glDepthFunc
specify the value used for depth buffer comparisons

## Parameters
- `func`
  Specifies the depth comparison function. Symbolic constants
  [`gl::NEVER`], [`gl::LESS`], [`gl::EQUAL`], [`gl::LEQUAL`],
  [`gl::GREATER`], [`gl::NOTEQUAL`], [`gl::GEQUAL`], and [`gl::ALWAYS`]
  are accepted. The initial value is [`gl::LESS`].

## Description
[`Gl::depth_func`] specifies the function used to compare each
  incoming pixel depth value with the depth value present in the depth
  buffer. The comparison is performed only if depth testing is enabled.
  (See [`Gl::enable`] and [`Gl::disable`] of [`gl::DEPTH_TEST`].)
`func` specifies the conditions under which the pixel will be drawn.
  The comparison functions are as follows:
The initial value of `func` is [`gl::LESS`]. Initially, depth testing
  is disabled. If depth testing is disabled or if no depth buffer
  exists, it is as if the depth test always passes.

## Notes
Even if the depth buffer exists and the depth mask is non-zero, the
  depth buffer is not updated if the depth test is disabled. In order to
  unconditionally write to the depth buffer, the depth test should be
  enabled and set to [`gl::ALWAYS`].

## Errors
- [`gl::INVALID_ENUM`] is generated if `func` is not an accepted value.

## See Also
- [`Gl::depth_range`]
- [`Gl::enable`]
- [`Gl::polygon_offset`]
