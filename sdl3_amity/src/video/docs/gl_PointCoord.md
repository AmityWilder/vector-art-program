# gl_PointCoord
contains the coordinate of a fragment within a point

## Description
[`Gl::point_coord`] is a fragment language input variable that
  contains the two-dimensional coordinates indicating where within a
  point primitive the current fragment is located. If the current
  primitive is not a point, then values read from [`Gl::point_coord`]
  are undefined.
[`Gl::point_coord.s`] ranges from 0.0 to 1.0 across the point
  horizontally from left to right. If [`gl::POINT_SPRITE_COORD_ORIGIN`]
  is [`gl::LOWER_LEFT`], [`Gl::point_coord.t`] varies from 0.0 to 1.0
  vertically from bottom to top. Otherwise, if
  [`gl::POINT_SPRITE_COORD_ORIGIN`] is [`gl::UPPER_LEFT`] then
  [`Gl::point_coord.t`] varies from 0.0 to 1.0 vertically from top to
  bottom. The default value of [`gl::POINT_SPRITE_COORD_ORIGIN`] is
  [`gl::UPPER_LEFT`].

## See Also
- [`Gl::frag_coord`]
- [`Gl::frag_depth`]
