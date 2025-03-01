# glPointSize
specify the diameter of rasterized points

## Parameters
- `size`
  Specifies the diameter of rasterized points. The initial value is 1.

## Description
[`Gl::point_size`] specifies the rasterized diameter of points. If
  point size mode is disabled (see [`Gl::enable`] with parameter
  [`gl::PROGRAM_POINT_SIZE`]), this value will be used to rasterize
  points. Otherwise, the value written to the shading language built-in
  variable ```c gl_PointSize ``` will be used.

## Notes
The point size specified by [`Gl::point_size`] is always returned when
  [`gl::POINT_SIZE`] is queried. Clamping and rounding for points have
  no effect on the specified value.

## Errors
- [`gl::INVALID_VALUE`] is generated if `size` is less than or equal to
  0.

## See Also
- [`Gl::enable`]
- [`Gl::point_parameter`]
