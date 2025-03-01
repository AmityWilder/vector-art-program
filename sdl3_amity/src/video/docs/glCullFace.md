# glCullFace
specify whether front- or back-facing facets can be culled

## Parameters
- `mode`
  Specifies whether front- or back-facing facets are candidates for
  culling. Symbolic constants [`gl::FRONT`], [`gl::BACK`], and
  [`gl::FRONT_AND_BACK`] are accepted. The initial value is
  [`gl::BACK`].

## Description
[`Gl::cull_face`] specifies whether front- or back-facing facets are
  culled (as specified by *mode*) when facet culling is enabled. Facet
  culling is initially disabled. To enable and disable facet culling,
  call the [`Gl::enable`] and [`Gl::disable`] commands with the argument
  [`gl::CULL_FACE`]. Facets include triangles, quadrilaterals, polygons,
  and rectangles.
[`Gl::front_face`] specifies which of the clockwise and
  counterclockwise facets are front-facing and back-facing. See
  [`Gl::front_face`].

## Notes
If `mode` is [`gl::FRONT_AND_BACK`], no facets are drawn, but other
  primitives such as points and lines are drawn.

## Errors
- [`gl::INVALID_ENUM`] is generated if `mode` is not an accepted value.

## See Also
- [`Gl::enable`]
- [`Gl::front_face`]
