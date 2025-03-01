# glMultiDrawElementsIndirect
render indexed primitives from array data, taking parameters from
  memory

## Parameters
- `mode`
  Specifies what kind of primitives to render. Symbolic constants
  [`gl::POINTS`], [`gl::LINE_STRIP`], [`gl::LINE_LOOP`], [`gl::LINES`],
  [`gl::LINE_STRIP_ADJACENCY`], [`gl::LINES_ADJACENCY`],
  [`gl::TRIANGLE_STRIP`], [`gl::TRIANGLE_FAN`], [`gl::TRIANGLES`],
  [`gl::TRIANGLE_STRIP_ADJACENCY`], [`gl::TRIANGLES_ADJACENCY`], and
  [`gl::PATCHES`] are accepted.

## Description
[`Gl::multi_draw_elements_indirect`] specifies multiple indexed
  geometric primitives with very few subroutine calls.
  [`Gl::multi_draw_elements_indirect`] behaves similarly to a multitude
  of calls to [`Gl::draw_elements_instanced_base_vertex_base_instance`],
  execpt that the parameters to
  [`Gl::draw_elements_instanced_base_vertex_base_instance`] are stored
  in an array in memory at the address given by `indirect`, separated by
  the stride, in basic machine units, specified by `stride`. If `stride`
  is zero, then the array is assumed to be tightly packed in memory.
The parameters addressed by `indirect` are packed into a structure
  that takes the form (in C): ```c typedef struct { uint count; uint
  instanceCount; uint firstIndex; int baseVertex; uint baseInstance; }
  DrawElementsIndirectCommand; ```
A single call to [`Gl::multi_draw_elements_indirect`] is equivalent,
  assuming no errors are generated to: ```c GLsizei n; for (n = 0; n <
  drawcount; n++) { const DrawElementsIndirectCommand *cmd; if (stride
  != 0) { cmd = (const DrawElementsIndirectCommand *)((uintptr)indirect
  + n * stride); } else { cmd = (const DrawElementsIndirectCommand
  *)indirect + n; } glDrawElementsInstancedBaseVertexBaseInstance(mode,
  cmd->count, type, cmd->firstIndex * size-of-type, cmd->instanceCount,
  cmd->baseVertex, cmd->baseInstance); } ```
If a buffer is bound to the [`gl::DRAW_INDIRECT_BUFFER`] binding at
  the time of a call to [`Gl::draw_elements_indirect`], `indirect` is
  interpreted as an offset, in basic machine units, into that buffer and
  the parameter data is read from the buffer rather than from client
  memory.
Note that indices stored in client memory are not supported. If no
  buffer is bound to the [`gl::ELEMENT_ARRAY_BUFFER`] binding, an error
  will be generated.
The results of the operation are undefined if the ```c
  reservedMustBeZero ``` member of the parameter structure is non-zero.
  However, no error is generated in this case.
Vertex attributes that are modified by [`Gl::draw_elements_indirect`]
  have an unspecified value after [`Gl::draw_elements_indirect`]
  returns. Attributes that aren't modified remain well defined.

## Notes
The `baseInstance` member of the `DrawElementsIndirectCommand`
  structure is defined only if the GL version is 4.2 or greater. For
  versions of the GL less than 4.2, this parameter is present but is
  reserved and should be set to zero. On earlier versions of the GL,
  behavior is undefined if it is non-zero.

## Errors
- [`gl::INVALID_ENUM`] is generated if `mode` is not an accepted value.
- [`gl::INVALID_VALUE`] is generated if `stride` is not a multiple of
  four.
- [`gl::INVALID_VALUE`] is generated if `drawcount` is negative.
- [`gl::INVALID_OPERATION`] is generated if no buffer is bound to the
  [`gl::ELEMENT_ARRAY_BUFFER`] binding, or if such a buffer's data store
  is currently mapped.
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
- [`Gl::draw_arrays_indirect`]
- [`Gl::draw_elements`]
- [`Gl::draw_range_elements`]
- [`Gl::draw_elements_indirect`]
- [`Gl::multi_draw_arrays_indirect`]
