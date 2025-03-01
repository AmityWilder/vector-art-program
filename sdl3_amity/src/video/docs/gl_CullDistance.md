# gl_CullDistance
provides a mechanism for controlling user culling

## Description
The [`Gl::cull_distance`] variable provides a mechanism for
  controlling user culling. The element `gl_CullDistance`[*i*] specifies
  a cull distance for each plane *i*. A distance of 0.0 means that the
  vertex is on the plane, a positive distance means that the vertex is
  inside the cull volume, and a negative distance means that the point
  is outside the cull volume. Primitives whose vertices all have a
  negative clip distance for plane *i* will be discarded.
The [`Gl::cull_distance`] array is predeclared as unsized and must be
  sized by the shader either by redeclaring it with an size or by
  indexing it only with integral constant expressions. The size
  determines the number and set of enabled cull distances and can be at
  most [`Gl::max_cull_distances`]. The number of varying components
  consumed by [`Gl::cull_distance`] will match the size of the array.
  Shaders writing [`Gl::cull_distance`] must write all enabled
  distances, or culling results are undefined.
As an output variable, [`Gl::cull_distance`] provides the place for
  the shader to write these distances. As an input in all but the
  fragment language, it reads the values written in the previous shader
  stage. In the fragment language, the [`Gl::cull_distance`] array
  contains linearly interpolated values for the vertex values written by
  a shader to the [`Gl::cull_distance`] vertex output variable.
It is a compile-time or link-time error for the set of shaders forming
  a program to have the sum of the sizes of the [`Gl::clip_distance`]
  and [`Gl::cull_distance`] arrays to be larger than
  [`Gl::max_combined_clip_and_cull_distances`].

## See Also
- [`Gl::clip_distance`]
- [`Gl::point_size`]
