# gl_InvocationID
contains the invocation index of the current shader

## Description
In the tessellation control language, [`Gl::invocation_id`] contains
  the index of the output patch vertex assigned to the shader
  invocation. It is assigned an integer value in the range [0, N-1]
  where N is the number of output patch vertices.
In the geometry language, [`Gl::invocation_id`] identifies the
  invocation number assigned to the geometry shader invocation. It is
  assigned an integer value in the range [0, N-1] where N is the number
  of geometry shader invocations per primitive.

## See Also
- [`Gl::instance_id`]
