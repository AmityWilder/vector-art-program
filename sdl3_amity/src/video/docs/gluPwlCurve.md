# gluPwlCurve
describe a piecewise linear NURBS trimming curve

## Parameters
- `nurb`
  Specifies the NURBS object (created with
  [`Gl::u_new_nurbs_renderer`]).

## Description
[`Gl::u_pwl_curve`] describes a piecewise linear trimming curve for a
  NURBS surface. A piecewise linear curve consists of a list of
  coordinates of points in the parameter space for the NURBS surface to
  be trimmed. These points are connected with line segments to form a
  curve. If the curve is an approximation to a curve that is not
  piecewise linear, the points should be close enough in parameter space
  that the resulting path appears curved at the resolution used in the
  application.
If `type` is [`GLU_MAP1_TRIM_2`], then it describes a curve in two-
  dimensional (*u* and *v*) parameter space. If it is
  [`GLU_MAP1_TRIM_3`], then it describes a curve in two-dimensional
  homogeneous (*u*, *v*, and *w*) parameter space. See the
  [`Gl::u_begin_trim`] reference page for more information about
  trimming curves.

## Notes
To describe a trim curve that closely follows the contours of a NURBS
  surface, call [`Gl::u_nurbs_curve`].

## See Also
- [`Gl::u_begin_curve`]
- [`Gl::u_begin_trim`]
- [`Gl::u_new_nurbs_renderer`]
- [`Gl::u_nurbs_curve`]
