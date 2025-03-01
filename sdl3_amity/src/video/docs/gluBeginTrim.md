# gluBeginTrim
delimit a NURBS trimming loop definition

## Parameters
- `nurb`
  Specifies the NURBS object (created with
  [`Gl::u_new_nurbs_renderer`]).

## Description
Use [`Gl::u_begin_trim`] to mark the beginning of a trimming loop and
  [`Gl::u_end_trim`] to mark the end of a trimming loop. A trimming loop
  is a set of oriented curve segments (forming a closed curve) that
  define boundaries of a NURBS surface. You include these trimming loops
  in the definition of a NURBS surface, between calls to
  [`Gl::u_begin_surface`] and [`Gl::u_end_surface`].
The definition for a NURBS surface can contain many trimming loops.
  For example, if you wrote a definition for a NURBS surface that
  resembled a rectangle with a hole punched out, the definition would
  contain two trimming loops. One loop would define the outer edge of
  the rectangle; the other would define the hole punched out of the
  rectangle. The definitions of each of these trimming loops would be
  bracketed by a [`Gl::u_begin_trim`]/[`Gl::u_end_trim`] pair.
The definition of a single closed trimming loop can consist of
  multiple curve segments, each described as a piecewise linear curve
  (see [`Gl::u_pwl_curve`]) or as a single NURBS curve (see
  [`Gl::u_nurbs_curve`]), or as a combination of both in any order. The
  only library calls that can appear in a trimming loop definition
  (between the calls to [`Gl::u_begin_trim`] and [`Gl::u_end_trim`]) are
  [`Gl::u_pwl_curve`] and [`Gl::u_nurbs_curve`].
The area of the NURBS surface that is displayed is the region in the
  domain to the left of the trimming curve as the curve parameter
  increases. Thus, the retained region of the NURBS surface is inside a
  counterclockwise trimming loop and outside a clockwise trimming loop.
  For the rectangle mentioned earlier, the trimming loop for the outer
  edge of the rectangle runs counterclockwise, while the trimming loop
  for the punched-out hole runs clockwise.
If you use more than one curve to define a single trimming loop, the
  curve segments must form a closed loop (that is, the endpoint of each
  curve must be the starting point of the next curve, and the endpoint
  of the final curve must be the starting point of the first curve). If
  the endpoints of the curve are sufficiently close together but not
  exactly coincident, they will be coerced to match. If the endpoints
  are not sufficiently close, an error results (see
  [`Gl::u_nurbs_callback`]).
If a trimming loop definition contains multiple curves, the direction
  of the curves must be consistent (that is, the inside must be to the
  left of all of the curves). Nested trimming loops are legal as long as
  the curve orientations alternate correctly. If trimming curves are
  self-intersecting, or intersect one another, an error results.
If no trimming information is given for a NURBS surface, the entire
  surface is drawn.

## Example
This code fragment defines a trimming loop that consists of one
  piecewise linear curve, and two NURBS curves: ```c gluBeginTrim(nobj);
  gluPwlCurve(..., GLU_MAP1_TRIM_2); gluNurbsCurve(...,
  GLU_MAP1_TRIM_2); gluNurbsCurve(..., GLU_MAP1_TRIM_3);
  gluEndTrim(nobj); ```

## See Also
- [`Gl::u_begin_surface`]
- [`Gl::u_new_nurbs_renderer`]
- [`Gl::u_nurbs_callback`]
- [`Gl::u_nurbs_curve`]
- [`Gl::u_pwl_curve`]
