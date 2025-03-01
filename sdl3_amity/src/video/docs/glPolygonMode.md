# glPolygonMode
select a polygon rasterization mode

## Parameters
- `face`
  Specifies the polygons that `mode` applies to. Must be
  [`gl::FRONT_AND_BACK`] for front- and back-facing polygons.

## Description
[`Gl::polygon_mode`] controls the interpretation of polygons for
  rasterization. `face` describes which polygons `mode` applies to: both
  front and back-facing polygons ([`gl::FRONT_AND_BACK`]). The polygon
  mode affects only the final rasterization of polygons. In particular,
  a polygon's vertices are lit and the polygon is clipped and possibly
  culled before these modes are applied.
Three modes are defined and can be specified in `mode`:

## Examples
To draw a surface with outlined polygons, call ```c glPolygonMode( ```
  [`gl::FRONT_AND_BACK`], [`gl::LINE`]);

## Notes
Vertices are marked as boundary or nonboundary with an edge flag. Edge
  flags are generated internally by the GL when it decomposes triangle
  stips and fans.

## Errors
- [`gl::INVALID_ENUM`] is generated if either `face` or `mode` is not an
  accepted value.

## See Also
- [`Gl::line_width`]
- [`Gl::point_size`]
