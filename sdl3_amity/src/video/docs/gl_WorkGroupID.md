# gl_WorkGroupID
contains the index of the workgroup currently being operated on by a
  compute shader

## Description
In the compute language, [`Gl::work_group_id`] contains the
  3-dimensional index of the global work group that the current compute
  shader invocation is executing within. The possible values range
  across the parameters passed into [`Gl::dispatch_compute`], i.e., from
  (0, 0, 0) to ([`Gl::num_work_groups.x`] - 1, [`Gl::num_work_groups.y`]
  - 1, [`Gl::num_work_groups.z`] - 1).

## See Also
- [`Gl::num_work_groups`]
- [`Gl::work_group_id`]
- [`Gl::local_invocation_id`]
