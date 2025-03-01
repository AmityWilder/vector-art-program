# glDispatchComputeIndirect
launch one or more compute work groups using parameters stored in a
  buffer

## Parameters
- `indirect`
  The offset into the buffer object currently bound to the
  [`gl::DISPATCH_INDIRECT_BUFFER`] buffer target at which the dispatch
  parameters are stored.

## Description
[`Gl::dispatch_compute_indirect`] launches one or more compute work
  groups using parameters stored in the buffer object currently bound to
  the [`gl::DISPATCH_INDIRECT_BUFFER`] target. Each work group is
  processed by the active program object for the compute shader stage.
  While the individual shader invocations within a work group are
  executed as a unit, work groups are executed completely independently
  and in unspecified order. `indirect` contains the offset into the data
  store of the buffer object bound to the
  [`gl::DISPATCH_INDIRECT_BUFFER`] target at which the parameters are
  stored.
The parameters addressed by `indirect` are packed a structure, which
  takes the form (in C): ```c typedef struct { uint num_groups_x; uint
  num_groups_y; uint num_groups_z; } DispatchIndirectCommand; ```
A call to [`Gl::dispatch_compute_indirect`] is equivalent, assuming no
  errors are generated, to: ```c cmd = (const DispatchIndirectCommand
  *)indirect; glDispatchCompute(cmd->num_groups_x, cmd->num_groups_y,
  cmd->num_groups_z); ```
Unlike [`Gl::dispatch_compute`], no error is generated if any of the
  ```c num_groups_x ``` , ```c num_groups_y ``` or ```c num_groups_z ```
  members of the ```c DispatchIndirectCommand ``` is larger than the
  value of [`gl::MAX_COMPUTE_WORK_GROUP_COUNT`] for the corresponding
  dimension. In such circumstances, behavior is undefined and may lead
  to application termination.

## Errors
- [`gl::INVALID_OPERATION`] is generated if there is no active program
  for the compute shader stage.
- [`gl::INVALID_VALUE`] is generated if `indirect` is less than zero or
  not a multiple of four.
- [`gl::INVALID_OPERATION`] is generated if no buffer is bound to the
  [`gl::DISPATCH_INDIRECT_BUFFER`] target or if the command would source
  data beyond the end of the buffer object's data store.

## See Also
- [`Gl::dispatch_compute`]
