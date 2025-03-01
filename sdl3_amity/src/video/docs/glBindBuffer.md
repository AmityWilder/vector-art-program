# glBindBuffer
bind a named buffer object

## Parameters
- `target`
  Specifies the target to which the buffer object is bound, which must
  be one of the buffer binding targets in the following table:

## Description
[`Gl::bind_buffer`] binds a buffer object to the specified buffer
  binding point. Calling [`Gl::bind_buffer`] with `target` set to one of
  the accepted symbolic constants and `buffer` set to the name of a
  buffer object binds that buffer object name to the target. If no
  buffer object with name `buffer` exists, one is created with that
  name. When a buffer object is bound to a target, the previous binding
  for that target is automatically broken.
Buffer object names are unsigned integers. The value zero is reserved,
  but there is no default buffer object for each buffer object target.
  Instead, `buffer` set to zero effectively unbinds any buffer object
  previously bound, and restores client memory usage for that buffer
  object target (if supported for that target). Buffer object names and
  the corresponding buffer object contents are local to the shared
  object space of the current GL rendering context; two rendering
  contexts share buffer object names only if they explicitly enable
  sharing between contexts through the appropriate GL windows interfaces
  functions.
[`Gl::gen_buffers`] must be used to generate a set of unused buffer
  object names.
The state of a buffer object immediately after it is first bound is an
  unmapped zero-sized memory buffer with [`gl::READ_WRITE`] access and
  [`gl::STATIC_DRAW`] usage.
While a non-zero buffer object name is bound, GL operations on the
  target to which it is bound affect the bound buffer object, and
  queries of the target to which it is bound return state from the bound
  buffer object. While buffer object name zero is bound, as in the
  initial state, attempts to modify or query state on the target to
  which it is bound generates an [`gl::INVALID_OPERATION`] error.
When a non-zero buffer object is bound to the [`gl::ARRAY_BUFFER`]
  target, the vertex array pointer parameter is interpreted as an offset
  within the buffer object measured in basic machine units.
When a non-zero buffer object is bound to the
  [`gl::DRAW_INDIRECT_BUFFER`] target, parameters for draws issued
  through [`Gl::draw_arrays_indirect`] and
  [`Gl::draw_elements_indirect`] are sourced from the specified offset
  in that buffer object's data store.
When a non-zero buffer object is bound to the
  [`gl::DISPATCH_INDIRECT_BUFFER`] target, the parameters for compute
  dispatches issued through [`Gl::dispatch_compute_indirect`] are
  sourced from the specified offset in that buffer object's data store.
While a non-zero buffer object is bound to the
  [`gl::ELEMENT_ARRAY_BUFFER`] target, the indices parameter of
  [`Gl::draw_elements`], [`Gl::draw_elements_instanced`],
  [`Gl::draw_elements_base_vertex`], [`Gl::draw_range_elements`],
  [`Gl::draw_range_elements_base_vertex`], [`Gl::multi_draw_elements`],
  or [`Gl::multi_draw_elements_base_vertex`] is interpreted as an offset
  within the buffer object measured in basic machine units.
While a non-zero buffer object is bound to the
  [`gl::PIXEL_PACK_BUFFER`] target, the following commands are affected:
  [`Gl::get_compressed_tex_image`], [`Gl::get_tex_image`], and
  [`Gl::read_pixels`]. The pointer parameter is interpreted as an offset
  within the buffer object measured in basic machine units.
While a non-zero buffer object is bound to the
  [`gl::PIXEL_UNPACK_BUFFER`] target, the following commands are
  affected: [`Gl::compressed_tex_image1d`],
  [`Gl::compressed_tex_image2d`], [`Gl::compressed_tex_image3d`],
  [`Gl::compressed_tex_sub_image1d`],
  [`Gl::compressed_tex_sub_image2d`],
  [`Gl::compressed_tex_sub_image3d`], [`Gl::tex_image1d`],
  [`Gl::tex_image2d`], [`Gl::tex_image3d`], [`Gl::tex_sub_image1d`],
  [`Gl::tex_sub_image2d`], and [`Gl::tex_sub_image3d`]. The pointer
  parameter is interpreted as an offset within the buffer object
  measured in basic machine units.
The buffer targets [`gl::COPY_READ_BUFFER`] and
  [`gl::COPY_WRITE_BUFFER`] are provided to allow
  [`Gl::copy_buffer_sub_data`] to be used without disturbing the state
  of other bindings. However, [`Gl::copy_buffer_sub_data`] may be used
  with any pair of buffer binding points.
The [`gl::TRANSFORM_FEEDBACK_BUFFER`] buffer binding point may be
  passed to [`Gl::bind_buffer`], but will not directly affect transform
  feedback state. Instead, the indexed [`gl::TRANSFORM_FEEDBACK_BUFFER`]
  bindings must be used through a call to [`Gl::bind_buffer_base`] or
  [`Gl::bind_buffer_range`]. This will affect the generic
  [`gl::TRANSFORM_FEEDBACK_BUFFER`] binding.
Likewise, the [`gl::UNIFORM_BUFFER`], [`gl::ATOMIC_COUNTER_BUFFER`]
  and [`gl::SHADER_STORAGE_BUFFER`] buffer binding points may be used,
  but do not directly affect uniform buffer, atomic counter buffer or
  shader storage buffer state, respectively. [`Gl::bind_buffer_base`] or
  [`Gl::bind_buffer_range`] must be used to bind a buffer to an indexed
  uniform buffer, atomic counter buffer or shader storage buffer binding
  point.
The [`gl::QUERY_BUFFER`] binding point is used to specify a buffer
  object that is to receive the results of query objects through calls
  to the [`Gl::get_query_object`] family of commands.
A buffer object binding created with [`Gl::bind_buffer`] remains
  active until a different buffer object name is bound to the same
  target, or until the bound buffer object is deleted with
  [`Gl::delete_buffers`].
Once created, a named buffer object may be re-bound to any target as
  often as needed. However, the GL implementation may make choices about
  how to optimize the storage of a buffer object based on its initial
  binding target.

## Notes
The [`gl::COPY_READ_BUFFER`], [`gl::UNIFORM_BUFFER`] and
  [`gl::TEXTURE_BUFFER`] targets are available only if the GL version is
  3.1 or greater.
The [`gl::ATOMIC_COUNTER_BUFFER`] target is available only if the GL
  version is 4.2 or greater.
The [`gl::DISPATCH_INDIRECT_BUFFER`] and [`gl::SHADER_STORAGE_BUFFER`]
  targets are available only if the GL version is 4.3 or greater.
The [`gl::QUERY_BUFFER`] target is available only if the GL version is
  4.4 or greater.

## Errors
- [`gl::INVALID_ENUM`] is generated if `target` is not one of the
  allowable values.
- [`gl::INVALID_VALUE`] is generated if `buffer` is not a name
  previously returned from a call to [`Gl::gen_buffers`].

## See Also
- [`Gl::gen_buffers`]
- [`Gl::bind_buffer_base`]
- [`Gl::bind_buffer_range`]
- [`Gl::map_buffer`]
- [`Gl::unmap_buffer`]
- [`Gl::delete_buffers`]
- [`Gl::get`]
- [`Gl::is_buffer`]
