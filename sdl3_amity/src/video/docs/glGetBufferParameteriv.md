# glGetBufferParameteriv
return parameters of a buffer object

## Parameters
- `target`
  Specifies the target buffer object. The symbolic constant must be
  [`gl::ARRAY_BUFFER`], [`gl::ELEMENT_ARRAY_BUFFER`],
  [`gl::PIXEL_PACK_BUFFER`], or [`gl::PIXEL_UNPACK_BUFFER`].

## Description
[`Gl::get_buffer_parameteriv`] returns in `data` a selected parameter
  of the buffer object specified by `target`.
`value` names a specific buffer object parameter, as follows:

## Notes
If an error is generated, no change is made to the contents of `data`.
[`Gl::get_buffer_parameteriv`] is available only if the GL version is
  1.5 or greater.
Targets [`gl::PIXEL_PACK_BUFFER`] and [`gl::PIXEL_UNPACK_BUFFER`] are
  available only if the GL version is 2.1 or greater.

## Errors
- [`gl::INVALID_ENUM`] is generated if `target` or `value` is not an
  accepted value.
- [`gl::INVALID_OPERATION`] is generated if the reserved buffer object
  name 0 is bound to `target`.
- [`gl::INVALID_OPERATION`] is generated if
  [`Gl::get_buffer_parameteriv`] is executed between the execution of
  [`Gl::begin`] and the corresponding execution of [`Gl::end`].

## See Also
- [`Gl::bind_buffer`]
- [`Gl::buffer_data`]
- [`Gl::map_buffer`]
- [`Gl::unmap_buffer`]
