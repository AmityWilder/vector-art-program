# glFrontFace
define front- and back-facing polygons

## Parameters
- `mode`
  Specifies the orientation of front-facing polygons. [`gl::CW`] and
  [`gl::CCW`] are accepted. The initial value is [`gl::CCW`].

## Description
In a scene composed entirely of opaque closed surfaces, back-facing
  polygons are never visible. Eliminating these invisible polygons has
  the obvious benefit of speeding up the rendering of the image. To
  enable and disable elimination of back-facing polygons, call
  [`Gl::enable`] and [`Gl::disable`] with argument [`gl::CULL_FACE`].
The projection of a polygon to window coordinates is said to have
  clockwise winding if an imaginary object following the path from its
  first vertex, its second vertex, and so on, to its last vertex, and
  finally back to its first vertex, moves in a clockwise direction about
  the interior of the polygon. The polygon's winding is said to be
  counterclockwise if the imaginary object following the same path moves
  in a counterclockwise direction about the interior of the polygon.
  [`Gl::front_face`] specifies whether polygons with clockwise winding
  in window coordinates, or counterclockwise winding in window
  coordinates, are taken to be front-facing. Passing [`gl::CCW`] to
  `mode` selects counterclockwise polygons as front-facing; [`gl::CW`]
  selects clockwise polygons as front-facing. By default,
  counterclockwise polygons are taken to be front-facing.

## Errors
- [`gl::INVALID_ENUM`] is generated if `mode` is not an accepted value.

## See Also
- [`Gl::cull_face`]
