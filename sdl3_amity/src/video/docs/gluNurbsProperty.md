# gluNurbsProperty
set a NURBS property

## Parameters
- `nurb`
  Specifies the NURBS object (created with
  [`Gl::u_new_nurbs_renderer`]).

## Description
[`Gl::u_nurbs_property`] is used to control properties stored in a
  NURBS object. These properties affect the way that a NURBS curve is
  rendered. The accepted values for `property` are as follows:

## Notes
If [`GLU_AUTO_LOAD_MATRIX`] is true, sampling and culling may be
  executed incorrectly if NURBS routines are compiled into a display
  list.
A `property` of [`GLU_PARAMETRIC_TOLERANCE`], [`GLU_SAMPLING_METHOD`],
  [`GLU_U_STEP`], or [`GLU_V_STEP`], or a `value` of
  [`GLU_PATH_LENGTH`], [`GLU_PARAMETRIC_ERROR`], [`GLU_DOMAIN_DISTANCE`]
  are only available if the GLU version is 1.1 or greater. They are not
  valid parameters in GLU 1.0.
[`Gl::u_get_string`] can be used to determine the GLU version.
[`GLU_NURBS_MODE`] is only available if the GLU version is 1.3 or
  greater.
The [`GLU_OBJECT_PATH_LENGTH`] and [`GLU_OBJECT_PARAMETRIC_ERROR`]
  values for the [`GLU_SAMPLING_METHOD`] property are only available if
  the GLU version is 1.3 or greater.

## See Also
- [`Gl::u_get_nurbs_property`]
- [`Gl::u_get_string`]
- [`Gl::u_load_sampling_matrices`]
- [`Gl::u_new_nurbs_renderer`]
- [`Gl::u_nurbs_callback`]
