# glDispatchCompute
launch one or more compute work groups

## Parameters
- `num_groups_x`
  The number of work groups to be launched in the X dimension.

## Description
[`Gl::dispatch_compute`] launches one or more compute work groups.
  Each work group is processed by the active program object for the
  compute shader stage. While the individual shader invocations within a
  work group are executed as a unit, work groups are executed completely
  independently and in unspecified order. `num_groups_x`, `num_groups_y`
  and `num_groups_z` specify the number of local work groups that will
  be dispatched in the X, Y and Z dimensions, respectively.

## Errors
- [`gl::INVALID_OPERATION`] is generated if there is no active program
  for the compute shader stage.
- [`gl::INVALID_VALUE`] is generated if any of `num_groups_x`,
  `num_groups_y`, or `num_groups_z` is greater than or equal to the
  maximum work-group count for the corresponding dimension.

## See Also
- [`Gl::dispatch_compute_indirect`]
