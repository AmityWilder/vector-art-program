# glPatchParameter
specifies the parameters for patch primitives

## Parameters
- `pname`
  Specifies the name of the parameter to set. The symbolc constants
  [`gl::PATCH_VERTICES`], [`gl::PATCH_DEFAULT_OUTER_LEVEL`], and
  [`gl::PATCH_DEFAULT_INNER_LEVEL`] are accepted.

## Description
[`Gl::patch_parameter`] specifies the parameters that will be used for
  patch primitives. `pname` specifies the parameter to modify and must
  be either [`gl::PATCH_VERTICES`], [`gl::PATCH_DEFAULT_OUTER_LEVEL`] or
  [`gl::PATCH_DEFAULT_INNER_LEVEL`]. For [`Gl::patch_parameteri`],
  `value` specifies the new value for the parameter specified by
  `pname`. For [`Gl::patch_parameterfv`], `values` specifies the address
  of an array containing the new values for the parameter specified by
  `pname`.
When `pname` is [`gl::PATCH_VERTICES`], `value` specifies the number
  of vertices that will be used to make up a single patch primitive.
  Patch primitives are consumed by the tessellation control shader (if
  present) and subsequently used for tessellation. When primitives are
  specified using [`Gl::draw_arrays`] or a similar function, each patch
  will be made from `parameter` control points, each represented by a
  vertex taken from the enabeld vertex arrays. `parameter` must be
  greater than zero, and less than or equal to the value of
  [`gl::MAX_PATCH_VERTICES`].
When `pname` is [`gl::PATCH_DEFAULT_OUTER_LEVEL`] or
  [`gl::PATCH_DEFAULT_INNER_LEVEL`], `values` contains the address of an
  array contiaining the default outer or inner tessellation levels,
  respectively, to be used when no tessellation control shader is
  present.

## Errors
- [`gl::INVALID_ENUM`] is generated if `pname` is not an accepted value.
- [`gl::INVALID_VALUE`] is generated if `pname` is
  [`gl::PATCH_VERTICES`] and `value` is less than or equal to zero, or
  greater than the value of [`gl::MAX_PATCH_VERTICES`].

## See Also
- [`Gl::draw_arrays`]
- [`Gl::draw_arrays_instanced`]
- [`Gl::draw_elements`]
- [`Gl::draw_range_elements`]
