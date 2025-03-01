# gl_LocalInvocationIndex
contains the local linear index of work item currently being operated
  on by a compute shader

## Description
In the compute language, [`Gl::local_invocation_index`] is a derived
  input variable containing the 1-dimensional linearized index of the
  work invocation within the work group that the current shader is
  executing on. The value of [`Gl::local_invocation_index`] is equal to
  [`Gl::local_invocation_id.z`] * [`Gl::work_group_size.x`] *
  [`Gl::work_group_size.y`] + [`Gl::local_invocation_id.y`] *
  [`Gl::work_group_size.x`] + [`Gl::local_invocation_id.x`].

## See Also
- [`Gl::num_work_groups`]
- [`Gl::work_group_id`]
- [`Gl::work_group_size`]
- [`Gl::local_invocation_id`]
