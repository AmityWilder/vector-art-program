# glDepthRangeArray
specify mapping of depth values from normalized device coordinates to
  window coordinates for a specified set of viewports

## Parameters
- `first`
  Specifies the index of the first viewport whose depth range to update.

## Description
After clipping and division by *w*, depth coordinates range from
  $None$ to 1, corresponding to the near and far clipping planes. Each
  viewport has an independent depth range specified as a linear mapping
  of the normalized depth coordinates in this range to window depth
  coordinates. Regardless of the actual depth buffer implementation,
  window coordinate depth values are treated as though they range from 0
  through 1 (like color components). $$ $$ -1 [`Gl::depth_range_array`]
  specifies a linear mapping of the normalized depth coordinates in this
  range to window depth coordinates for each viewport in the range
  [`first`, `first` + `count`). Thus, the values accepted by
  [`Gl::depth_range_array`] are both clamped to this range before they
  are accepted.
The `first` parameter specifies the index of the first viewport whose
  depth range to modify and must be less than the value of
  [`gl::MAX_VIEWPORTS`]. `count` specifies the number of viewports whose
  depth range to modify. `first` + `count` must be less than or equal to
  the value of [`gl::MAX_VIEWPORTS`]. `v` specifies the address of an
  array of pairs of double precision floating point values representing
  the near and far values of the depth range for each viewport, in that
  order.
The setting of (0,1) maps the near plane to 0 and the far plane to 1.
  With this mapping, the depth buffer range is fully utilized.

## Notes
It is not necessary that the near plane distance be less than the far
  plane distance. Reverse mappings such as $None$, and $$ $$ *near* *=*
  1 $None$ are acceptable. $$ $$ *far* *=* 0
The type of the `v` parameter was changed from GLclampd to GLdouble.
  This change is transparent to user code and is described in detail on
  the [`Gl::removed_types`] page.

## Errors
- [`gl::INVALID_VALUE`] is generated if `first` is greater than or equal
  to the value of [`gl::MAX_VIEWPORTS`].
- [`gl::INVALID_VALUE`] is generated if `first` + `count` is greater
  than or equal to the value of [`gl::MAX_VIEWPORTS`].

## See Also
- [`Gl::depth_func`]
- [`Gl::depth_range`]
- [`Gl::depth_range_indexed`]
- [`Gl::polygon_offset`]
- [`Gl::viewport_array`]
- [`Gl::viewport`]
- [`Gl::removed_types`]
