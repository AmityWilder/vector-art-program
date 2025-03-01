# glTextureBarrier
controls the ordering of reads and writes to rendered fragments across
  drawing commands

## Description
The values of rendered fragments are undefined when a shader stage
  fetches texels and the same texels are written via fragment shader
  outputs, even if the reads and writes are not in the same drawing
  command. To safely read the result of a written texel via a texel
  fetch in a subsequent drawing command, call [`Gl::texture_barrier`]
  between the two drawing commands to guarantee that writes have
  completed and caches have been invalidated before subsequent drawing
  commands are executed.

## Notes
The situation described above is referred to as a *rendering feedback
  loop* and is discussed in more detail in section 9.3 of the OpenGL 4.5
  Specification.

## Errors
- None.

## See Also
- [`Gl::memory_barrier`]
