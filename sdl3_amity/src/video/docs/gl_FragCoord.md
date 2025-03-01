# gl_FragCoord
contains the window-relative coordinates of the current fragment

## Description
Available only in the fragment language, [`Gl::frag_coord`] is an
  input variable that contains the window relative coordinate (x, y, z,
  1/w) values for the fragment. If multi-sampling, this value can be for
  any location within the pixel, or one of the fragment samples. This
  value is the result of fixed functionality that interpolates
  primitives after vertex processing to generate fragments. The z
  component is the depth value that would be used for the fragment's
  depth if no shader contained any writes to [`Gl::frag_depth`].
[`Gl::frag_coord`] may be redeclared with the additional layout
  qualifier identifiers [`Gl::origin_upper_left`] or
  [`Gl::pixel_center_integer`]. By default, [`Gl::frag_coord`] assumes a
  lower-left origin for window coordinates and assumes pixel centers are
  located at half-pixel centers. For example, the (x, y) location (0.5,
  0.5) is returned for the lower-left-most pixel in a window. The origin
  of [`Gl::frag_coord`] may be changed by redeclaring [`Gl::frag_coord`]
  with the [`Gl::origin_upper_left`] identifier. The values returned can
  also be shifted by half a pixel in both x and y by
  [`Gl::pixel_center_integer`] so it appears the pixels are centered at
  whole number pixel offsets. This moves the (x, y) value returned by
  [`Gl::frag_coord`] of (0.5, 0.5) by default to (0.0, 0.0) with
  [`Gl::pixel_center_integer`].
If [`Gl::frag_coord`] is redeclared in any fragment shader in a
  program, it must be redeclared in all fragment shaders in that program
  that have static use of [`Gl::frag_coord`]. Redeclaring
  [`Gl::frag_coord`] with any accepted qualifier affects only
  [`Gl::frag_coord.x`] and [`Gl::frag_coord.y`]. It has no effect on
  rasterization, transformation or any other part of the OpenGL pipeline
  or language features.

## See Also
- [`Gl::frag_depth`]
