# glShaderStorageBlockBinding
change an active shader storage block binding

## Parameters
- `program`
  The name of the program containing the block whose binding to change.

## Description
[`Gl::shader_storage_block_binding`], changes the active shader
  storage block with an assigned index of `storageBlockIndex` in program
  object `program`. `storageBlockIndex` must be an active shader storage
  block index in `program`. `storageBlockBinding` must be less than the
  value of [`gl::MAX_SHADER_STORAGE_BUFFER_BINDINGS`]. If successful,
  [`Gl::shader_storage_block_binding`] specifies that `program` will use
  the data store of the buffer object bound to the binding point
  `storageBlockBinding` to read and write the values of the buffer
  variables in the shader storage block identified by
  `storageBlockIndex`.

## Errors
- [`gl::INVALID_VALUE`] is generated if `program` is not the name of
  either a program or shader object.
- [`gl::INVALID_OPERATION`] is generated if `program` is the name of a
  shader object.
- [`gl::INVALID_VALUE`] is generated if `storageBlockIndex` is not an
  active shader storage block index in `program`, or if
  `storageBlockBinding` is greater than or equal to the value of
  [`MAX_SHADER_STORAGE_BUFFER_BINDINGS`].

## See Also
