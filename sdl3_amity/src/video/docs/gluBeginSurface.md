# gluBeginSurface
delimit a NURBS surface definition

## Parameters
- `nurb`
  Specifies the NURBS object (created with
  [`Gl::u_new_nurbs_renderer`]).

## Description
Use [`Gl::u_begin_surface`] to mark the beginning of a NURBS surface
  definition. After calling [`Gl::u_begin_surface`], make one or more
  calls to [`Gl::u_nurbs_surface`] to define the attributes of the
  surface. Exactly one of these calls to [`Gl::u_nurbs_surface`] must
  have a surface type of [`GLU_MAP2_VERTEX_3`] or [`GLU_MAP2_VERTEX_4`].
  To mark the end of the NURBS surface definition, call
  [`Gl::u_end_surface`].
Trimming of NURBS surfaces is supported with [`Gl::u_begin_trim`],
  [`Gl::u_pwl_curve`], [`Gl::u_nurbs_curve`], and [`Gl::u_end_trim`].
  See the [`Gl::u_begin_trim`] reference page for details.
GL evaluators are used to render the NURBS surface as a set of
  polygons. Evaluator state is preserved during rendering with
  [`Gl::push_attrib`]([`GLU_EVAL_BIT`]) and [`Gl::pop_attrib`]. See the
  [`Gl::push_attrib`] reference page for details on exactly what state
  these calls preserve.

## Example
The following commands render a textured NURBS surface with normals;
  the texture coordinates and normals are also described as NURBS
  surfaces: ```c gluBeginSurface(nobj); gluNurbsSurface(nobj, ...,
  GL_MAP2_TEXTURE_COORD_2); gluNurbsSurface(nobj, ..., GL_MAP2_NORMAL);
  gluNurbsSurface(nobj, ..., GL_MAP2_VERTEX_4); gluEndSurface(nobj); ```

## See Also
- [`Gl::u_begin_curve`]
- [`Gl::u_begin_trim`]
- [`Gl::u_new_nurbs_renderer`]
- [`Gl::u_nurbs_curve`]
- [`Gl::u_nurbs_surface`]
- [`Gl::u_pwl_curve`]
- [`Gl::push_attrib`]
