# gl_InstanceID
contains the index of the current primitive in an instanced draw
  command

## Description
[`Gl::instance_id`] is a vertex language input variable that holds the
  integer index of the current primitive in an instanced draw command
  such as [`Gl::draw_arrays_instanced`]. If the current primitive does
  not originate from an instanced draw command, the value of
  [`Gl::instance_id`] is zero.

## See Also
- [`Gl::vertex_id`]
