# glPolygonOffset
set the scale and units used to calculate depth values

## Parameters
- `factor`
  Specifies a scale factor that is used to create a variable depth
  offset for each polygon. The initial value is 0.

## Description
When [`gl::POLYGON_OFFSET_FILL`], [`gl::POLYGON_OFFSET_LINE`], or
  [`gl::POLYGON_OFFSET_POINT`] is enabled, each fragment's *depth* value
  will be offset after it is interpolated from the *depth* values of the
  appropriate vertices. The value of the offset is $None$, where $$ $$
  *factor* *\u{00D7}* *DZ* *+* *r* *\u{00D7}* *units* $None$ is a
  measurement of the change in depth relative to the screen area of the
  polygon, and $$ $$ *DZ* $None$ is the smallest value that is
  guaranteed to produce a resolvable offset for a given implementation.
  The offset is added before the depth test is performed and before the
  value is written into the depth buffer. $$ None $$ *r*
[`Gl::polygon_offset`] is useful for rendering hidden-line images, for
  applying decals to surfaces, and for rendering solids with highlighted
  edges.

## See Also
- [`Gl::depth_func`]
- [`Gl::enable`]
- [`Gl::get`]
- [`Gl::is_enabled`]
