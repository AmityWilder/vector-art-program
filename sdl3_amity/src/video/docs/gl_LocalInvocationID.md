# gl_LocalInvocationID
contains the index of work item currently being operated on by a
  compute shader

## Description
In the compute language, [`Gl::local_invocation_id`] is an input
  variable containing the n-dimensional index of the local work
  invocation within the work group that the current shader is executing
  in. The possible values for this variable range across the local work
  group size, i.e., (0,0,0) to ([`Gl::work_group_size.x`] - 1,
  [`Gl::work_group_size.y`] - 1, [`Gl::work_group_size.z`] - 1).

## See Also
- [`Gl::num_work_groups`]
- [`Gl::work_group_id`]
- [`Gl::work_group_size`]
- [`Gl::global_invocation_id`]
