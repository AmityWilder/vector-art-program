# gluNurbsCurve
define the shape of a NURBS curve

## Parameters
- `nurb`
  Specifies the NURBS object (created with
  [`Gl::u_new_nurbs_renderer`]).

## Description
Use [`Gl::u_nurbs_curve`] to describe a NURBS curve.
When [`Gl::u_nurbs_curve`] appears between a
  [`Gl::u_begin_curve`]/[`Gl::u_end_curve`] pair, it is used to describe
  a curve to be rendered. Positional, texture, and color coordinates are
  associated by presenting each as a separate [`Gl::u_nurbs_curve`]
  between a [`Gl::u_begin_curve`]/[`Gl::u_end_curve`] pair. No more than
  one call to [`Gl::u_nurbs_curve`] for each of color, position, and
  texture data can be made within a single
  [`Gl::u_begin_curve`]/[`Gl::u_end_curve`] pair. Exactly one call must
  be made to describe the position of the curve (a `type` of
  [`GLU_MAP1_VERTEX_3`] or [`GLU_MAP1_VERTEX_4`]).
When [`Gl::u_nurbs_curve`] appears between a
  [`Gl::u_begin_trim`]/[`Gl::u_end_trim`] pair, it is used to describe a
  trimming curve on a NURBS surface. If `type` is [`GLU_MAP1_TRIM_2`],
  then it describes a curve in two-dimensional (*u* and *v*) parameter
  space. If it is [`GLU_MAP1_TRIM_3`], then it describes a curve in two-
  dimensional homogeneous (*u*, *v*, and *w*) parameter space. See the
  [`Gl::u_begin_trim`] reference page for more discussion about trimming
  curves.

## Example
The following commands render a textured NURBS curve with normals:
  ```c gluBeginCurve(nobj); gluNurbsCurve(nobj, ...,
  GL_MAP1_TEXTURE_COORD_2); gluNurbsCurve(nobj, ..., GL_MAP1_NORMAL);
  gluNurbsCurve(nobj, ..., GL_MAP1_VERTEX_4); gluEndCurve(nobj); ```

## Notes
To define trim curves that stitch well, use [`Gl::u_pwl_curve`].

## See Also
- [`Gl::u_begin_curve`]
- [`Gl::u_begin_trim`]
- [`Gl::u_new_nurbs_renderer`]
- [`Gl::u_pwl_curve`]
