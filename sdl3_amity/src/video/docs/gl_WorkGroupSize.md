# gl_WorkGroupSize
contains the size of the workgroup operated on by a compute shader

## Description
In the compute language, [`Gl::work_group_size`] contains the size of
  a workgroup declared by a compute shader. The size of the work group
  in the X, Y, and Z dimensions is stored in the x, y, and z components
  of [`Gl::work_group_size`]. The values stored in
  [`Gl::work_group_size`] match those specified in the required
  [`Gl::local_size_x`], [`Gl::local_size_y`], and [`Gl::local_size_z`]
  layout qualifiers for the current shader. This value is constant so
  that it can be used to size arrays of memory that can be shared within
  the local work group.

## See Also
- [`Gl::num_work_groups`]
- [`Gl::work_group_id`]
- [`Gl::local_invocation_id`]
