# gluBeginCurve
delimit a NURBS curve definition

## Parameters
- `nurb`
  Specifies the NURBS object (created with
  [`Gl::u_new_nurbs_renderer`]).

## Description
Use [`Gl::u_begin_curve`] to mark the beginning of a NURBS curve
  definition. After calling [`Gl::u_begin_curve`], make one or more
  calls to [`Gl::u_nurbs_curve`] to define the attributes of the curve.
  Exactly one of the calls to [`Gl::u_nurbs_curve`] must have a curve
  type of [`GLU_MAP1_VERTEX_3`] or [`GLU_MAP1_VERTEX_4`]. To mark the
  end of the NURBS curve definition, call [`Gl::u_end_curve`].
GL evaluators are used to render the NURBS curve as a series of line
  segments. Evaluator state is preserved during rendering with
  [`Gl::push_attrib`]([`GLU_EVAL_BIT`]) and [`Gl::pop_attrib`](). See
  the [`Gl::push_attrib`] reference page for details on exactly what
  state these calls preserve.

## Example
The following commands render a textured NURBS curve with normals;
  texture coordinates and normals are also specified as NURBS curves:
  ```c gluBeginCurve(nobj); gluNurbsCurve(nobj, ...,
  GL_MAP1_TEXTURE_COORD_2); gluNurbsCurve(nobj, ..., GL_MAP1_NORMAL);
  gluNurbsCurve(nobj, ..., GL_MAP1_VERTEX_4); gluEndCurve(nobj); ```

## See Also
- [`Gl::u_begin_surface`]
- [`Gl::u_begin_trim`]
- [`Gl::u_new_nurbs_renderer`]
- [`Gl::u_nurbs_curve`]
- [`Gl::pop_attrib`]
- [`Gl::push_attrib`]
