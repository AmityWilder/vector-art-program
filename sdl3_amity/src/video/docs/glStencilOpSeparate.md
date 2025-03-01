# glStencilOpSeparate
set front and/or back stencil test actions

## Parameters
- `face`
  Specifies whether front and/or back stencil state is updated. Three
  symbolic constants are valid: [`gl::FRONT`], [`gl::BACK`], and
  [`gl::FRONT_AND_BACK`].

## Description
Stenciling, like depth-buffering, enables and disables drawing on a
  per-pixel basis. You draw into the stencil planes using GL drawing
  primitives, then render geometry and images, using the stencil planes
  to mask out portions of the screen. Stenciling is typically used in
  multipass rendering algorithms to achieve special effects, such as
  decals, outlining, and constructive solid geometry rendering.
The stencil test conditionally eliminates a pixel based on the outcome
  of a comparison between the value in the stencil buffer and a
  reference value. To enable and disable the test, call [`Gl::enable`]
  and [`Gl::disable`] with argument [`gl::STENCIL_TEST`]; to control it,
  call [`Gl::stencil_func`] or [`Gl::stencil_func_separate`].
There can be two separate sets of `sfail`, `dpfail`, and `dppass`
  parameters; one affects back-facing polygons, and the other affects
  front-facing polygons as well as other non-polygon primitives.
  [`Gl::stencil_op`] sets both front and back stencil state to the same
  values, as if [`Gl::stencil_op_separate`] were called with `face` set
  to [`gl::FRONT_AND_BACK`].
[`Gl::stencil_op_separate`] takes three arguments that indicate what
  happens to the stored stencil value while stenciling is enabled. If
  the stencil test fails, no change is made to the pixel's color or
  depth buffers, and `sfail` specifies what happens to the stencil
  buffer contents. The following eight actions are possible.
Stencil buffer values are treated as unsigned integers. When
  incremented and decremented, values are clamped to 0 and $None$, where
  $$ $$ ^{None} 2 *n* *-* 1 $None$ is the value returned by querying $$
  None $$ *n*[`gl::STENCIL_BITS`].
The other two arguments to [`Gl::stencil_op_separate`] specify stencil
  buffer actions that depend on whether subsequent depth buffer tests
  succeed (`dppass`) or fail (`dpfail`) (see [`Gl::depth_func`]). The
  actions are specified using the same eight symbolic constants as
  `sfail`. Note that `dpfail` is ignored when there is no depth buffer,
  or when the depth buffer is not enabled. In these cases, `sfail` and
  `dppass` specify stencil action when the stencil test fails and
  passes, respectively.

## Notes
Initially the stencil test is disabled. If there is no stencil buffer,
  no stencil modification can occur and it is as if the stencil test
  always passes.

## Errors
- [`gl::INVALID_ENUM`] is generated if `face` is any value other than
  [`gl::FRONT`], [`gl::BACK`], or [`gl::FRONT_AND_BACK`].
- [`gl::INVALID_ENUM`] is generated if `sfail`, `dpfail`, or `dppass` is
  any value other than the eight defined constant values.

## See Also
- [`Gl::blend_func`]
- [`Gl::depth_func`]
- [`Gl::enable`]
- [`Gl::logic_op`]
- [`Gl::stencil_func`]
- [`Gl::stencil_func_separate`]
- [`Gl::stencil_mask`]
- [`Gl::stencil_mask_separate`]
- [`Gl::stencil_op`]
