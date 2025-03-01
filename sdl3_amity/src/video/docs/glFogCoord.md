# glFogCoord
set the current fog coordinates

## Parameters
- `coord`
  Specify the fog distance.

## Parameters
- `coord`
  Specifies a pointer to an array containing a single value representing
  the fog distance.

## Description
[`Gl::fog_coord`] specifies the fog coordinate that is associated with
  each vertex and the current raster position. The value specified is
  interpolated and used in computing the fog color (see [`Gl::fog`]).

## Notes
[`Gl::fog_coord`] is available only if the GL version is 1.4 or
  greater.
The current fog coordinate can be updated at any time. In particular,
  [`Gl::fog_coord`] can be called between a call to [`Gl::begin`] and
  the corresponding call to [`Gl::end`].

## See Also
- [`Gl::fog`]
- [`Gl::fog_coord_pointer`]
- [`Gl::vertex`]
