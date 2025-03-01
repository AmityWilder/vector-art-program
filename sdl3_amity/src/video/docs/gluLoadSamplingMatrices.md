# gluLoadSamplingMatrices
load NURBS sampling and culling matrices

## Parameters
- `nurb`
  Specifies the NURBS object (created with
  [`Gl::u_new_nurbs_renderer`]).

## Description
[`Gl::u_load_sampling_matrices`] uses `model`, `perspective`, and
  `view` to recompute the sampling and culling matrices stored in
  `nurb`. The sampling matrix determines how finely a NURBS curve or
  surface must be tessellated to satisfy the sampling tolerance (as
  determined by the [`GLU_SAMPLING_TOLERANCE`] property). The culling
  matrix is used in deciding if a NURBS curve or surface should be
  culled before rendering (when the [`GLU_CULLING`] property is turned
  on).
[`Gl::u_load_sampling_matrices`] is necessary only if the
  [`GLU_AUTO_LOAD_MATRIX`] property is turned off (see
  [`Gl::u_nurbs_property`]). Although it can be convenient to leave the
  [`GLU_AUTO_LOAD_MATRIX`] property turned on, there can be a
  performance penalty for doing so. (A round trip to the GL server is
  needed to fetch the current values of the modelview matrix, projection
  matrix, and viewport.)

## See Also
- [`Gl::u_get_nurbs_property`]
- [`Gl::u_new_nurbs_renderer`]
- [`Gl::u_nurbs_property`]
