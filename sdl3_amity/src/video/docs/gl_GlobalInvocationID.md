# gl_GlobalInvocationID
contains the global index of work item currently being operated on by
  a compute shader

## Description
In the compute language, [`Gl::global_invocation_id`] is a derived
  input variable containing the n-dimensional index of the work
  invocation within the global work group that the current shader is
  executing on. The value of [`Gl::global_invocation_id`] is equal to
  [`Gl::work_group_id`] * [`Gl::work_group_size`] +
  [`Gl::local_invocation_id`].

## See Also
- [`Gl::num_work_groups`]
- [`Gl::work_group_id`]
- [`Gl::work_group_size`]
- [`Gl::local_invocation_id`]
