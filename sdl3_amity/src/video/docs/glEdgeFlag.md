# glEdgeFlag
flag edges as either boundary or nonboundary

## Parameters
- `flag`
  Specifies the current edge flag value, either [`gl::TRUE`] or
  [`gl::FALSE`]. The initial value is [`gl::TRUE`].

## Parameters
- `flag`
  Specifies a pointer to an array that contains a single boolean
  element, which replaces the current edge flag value.

## Description
Each vertex of a polygon, separate triangle, or separate quadrilateral
  specified between a [`Gl::begin`]/[`Gl::end`] pair is marked as the
  start of either a boundary or nonboundary edge. If the current edge
  flag is true when the vertex is specified, the vertex is marked as the
  start of a boundary edge. Otherwise, the vertex is marked as the start
  of a nonboundary edge. [`Gl::edge_flag`] sets the edge flag bit to
  [`gl::TRUE`] if `flag` is [`gl::TRUE`] and to [`gl::FALSE`] otherwise.
The vertices of connected triangles and connected quadrilaterals are
  always marked as boundary, regardless of the value of the edge flag.
Boundary and nonboundary edge flags on vertices are significant only
  if [`gl::POLYGON_MODE`] is set to [`gl::POINT`] or [`gl::LINE`]. See
  [`Gl::polygon_mode`].

## Notes
The current edge flag can be updated at any time. In particular,
  [`Gl::edge_flag`] can be called between a call to [`Gl::begin`] and
  the corresponding call to [`Gl::end`].

## See Also
- [`Gl::begin`]
- [`Gl::polygon_mode`]
