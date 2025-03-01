# gl_NumWorkGroups
contains the number of workgroups that have been dispatched to a
  compute shader

## Description
In the compute language, [`Gl::num_work_groups`] contains the total
  number of work groups that will execute the compute shader. The
  components of [`Gl::num_work_groups`] are equal to the `num_groups_x`,
  `num_groups_y`, and `num_groups_z` parameters passed to the
  [`Gl::dispatch_compute`] command.

## See Also
- [`Gl::work_group_size`]
- [`Gl::work_group_id`]
- [`Gl::local_invocation_id`]
