# glDrawArraysIndirect
render primitives from array data, taking parameters from memory

## Parameters
- `mode`
  Specifies what kind of primitives to render. Symbolic constants
  [`gl::POINTS`], [`gl::LINE_STRIP`], [`gl::LINE_LOOP`], [`gl::LINES`],
  [`gl::LINE_STRIP_ADJACENCY`], [`gl::LINES_ADJACENCY`],
  [`gl::TRIANGLE_STRIP`], [`gl::TRIANGLE_FAN`], [`gl::TRIANGLES`],
  [`gl::TRIANGLE_STRIP_ADJACENCY`], [`gl::TRIANGLES_ADJACENCY`], and
  [`gl::PATCHES`] are accepted.

## Description
[`Gl::draw_arrays_indirect`] specifies multiple geometric primitives
  with very few subroutine calls. [`Gl::draw_arrays_indirect`] behaves
  similarly to [`Gl::draw_arrays_instanced_base_instance`], execept that
  the parameters to [`Gl::draw_arrays_instanced_base_instance`] are
  stored in memory at the address given by `indirect`.
The parameters addressed by `indirect` are packed into a structure
  that takes the form (in C): ```c typedef struct { uint count; uint
  instanceCount; uint first; uint baseInstance; }
  DrawArraysIndirectCommand; const DrawArraysIndirectCommand *cmd =
  (const DrawArraysIndirectCommand *)indirect;
  glDrawArraysInstancedBaseInstance(mode, cmd->first, cmd->count,
  cmd->instanceCount, cmd->baseInstance); ```
If a buffer is bound to the [`gl::DRAW_INDIRECT_BUFFER`] binding at
  the time of a call to [`Gl::draw_arrays_indirect`], `indirect` is
  interpreted as an offset, in basic machine units, into that buffer and
  the parameter data is read from the buffer rather than from client
  memory.
In contrast to [`Gl::draw_arrays_instanced_base_instance`], the ```c
  first ``` member of the parameter structure is unsigned, and out-of-
  range indices do not generate an error.
Vertex attributes that are modified by [`Gl::draw_arrays_indirect`]
  have an unspecified value after [`Gl::draw_arrays_indirect`] returns.
  Attributes that aren't modified remain well defined.

## Notes
The `baseInstance` member of the `DrawArraysIndirectCommand` structure
  is defined only if the GL version is 4.2 or greater. For versions of
  the GL less than 4.2, this parameter is present but is reserved and
  should be set to zero. On earlier versions of the GL, behavior is
  undefined if it is non-zero.

## Errors
- [`gl::INVALID_ENUM`] is generated if `mode` is not an accepted value.
- [`gl::INVALID_OPERATION`] is generated if a non-zero buffer object
  name is bound to an enabled array or to the
  [`gl::DRAW_INDIRECT_BUFFER`] binding and the buffer object's data
  store is currently mapped.
- [`gl::INVALID_OPERATION`] is generated if a geometry shader is active
  and `mode` is incompatible with the input primitive type of the
  geometry shader in the currently installed program object.
- [`gl::INVALID_OPERATION`] is generated if `mode` is [`gl::PATCHES`]
  and no tessellation control shader is active.

## See Also
- [`Gl::draw_arrays`]
- [`Gl::draw_arrays_instanced`]
- [`Gl::draw_elements`]
- [`Gl::draw_range_elements`]
