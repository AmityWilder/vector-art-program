# glDepthRange
specify mapping of depth values from normalized device coordinates to
  window coordinates

## Parameters
- `nearVal`
  Specifies the mapping of the near clipping plane to window
  coordinates. The initial value is 0.

## Description
After clipping and division by *w*, depth coordinates range from
  $None$ to 1, corresponding to the near and far clipping planes. $$ $$
  -1 [`Gl::depth_range`] specifies a linear mapping of the normalized
  depth coordinates in this range to window depth coordinates.
  Regardless of the actual depth buffer implementation, window
  coordinate depth values are treated as though they range from 0
  through 1 (like color components). Thus, the values accepted by
  [`Gl::depth_range`] are both clamped to this range before they are
  accepted.
The setting of (0,1) maps the near plane to 0 and the far plane to 1.
  With this mapping, the depth buffer range is fully utilized.

## Notes
It is not necessary that `nearVal` be less than `farVal`. Reverse
  mappings such as $None$, and $$ $$ *nearVal* *=* 1 $None$ are
  acceptable. $$ $$ *farVal* *=* 0
The type of the `nearVal` and `farVal` parameters was changed from
  GLclampf to GLfloat for [`Gl::depth_rangef`] and from GLclampd to
  GLdouble for [`Gl::depth_range`]. This change is transparent to user
  code and is described in detail on the [`Gl::removed_types`] page.

## See Also
- [`Gl::depth_func`]
- [`Gl::polygon_offset`]
- [`Gl::viewport`]
- [`Gl::removed_types`]
