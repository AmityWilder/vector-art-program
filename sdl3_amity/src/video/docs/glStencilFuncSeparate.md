# glStencilFuncSeparate
set front and/or back function and reference value for stencil testing

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
  of a comparison between the reference value and the value in the
  stencil buffer. To enable and disable the test, call [`Gl::enable`]
  and [`Gl::disable`] with argument [`gl::STENCIL_TEST`]. To specify
  actions based on the outcome of the stencil test, call
  [`Gl::stencil_op`] or [`Gl::stencil_op_separate`].
There can be two separate sets of `func`, `ref`, and `mask`
  parameters; one affects back-facing polygons, and the other affects
  front-facing polygons as well as other non-polygon primitives.
  [`Gl::stencil_func`] sets both front and back stencil state to the
  same values, as if [`Gl::stencil_func_separate`] were called with
  `face` set to [`gl::FRONT_AND_BACK`].
`func` is a symbolic constant that determines the stencil comparison
  function. It accepts one of eight values, shown in the following list.
  `ref` is an integer reference value that is used in the stencil
  comparison. It is clamped to the range $None$, where $$ $$ ```c ``` 0
  ^{None} 2 *n* *-* 1 $None$ is the number of bitplanes in the stencil
  buffer. $$ None $$ *n*`mask` is bitwise ANDed with both the reference
  value and the stored stencil value, with the ANDed values
  participating in the comparison.
If *stencil* represents the value stored in the corresponding stencil
  buffer location, the following list shows the effect of each
  comparison function that can be specified by `func`. Only if the
  comparison succeeds is the pixel passed through to the next stage in
  the rasterization process (see [`Gl::stencil_op`]). All tests treat
  *stencil* values as unsigned integers in the range $None$, where $$ $$
  ```c ``` 0 ^{None} 2 *n* *-* 1 $None$ is the number of bitplanes in
  the stencil buffer. $$ None $$ *n*
The following values are accepted by `func`:

## Notes
Initially, the stencil test is disabled. If there is no stencil
  buffer, no stencil modification can occur and it is as if the stencil
  test always passes.

## Errors
- [`gl::INVALID_ENUM`] is generated if `func` is not one of the eight
  accepted values.

## See Also
- [`Gl::blend_func`]
- [`Gl::depth_func`]
- [`Gl::enable`]
- [`Gl::logic_op`]
- [`Gl::stencil_func`]
- [`Gl::stencil_mask`]
- [`Gl::stencil_mask_separate`]
- [`Gl::stencil_op`]
- [`Gl::stencil_op_separate`]
