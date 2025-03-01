# glVertexAttribDivisor
modify the rate at which generic vertex attributes advance during
  instanced rendering

## Parameters
- `index`
  Specify the index of the generic vertex attribute.

## Description
[`Gl::vertex_attrib_divisor`] modifies the rate at which generic
  vertex attributes advance when rendering multiple instances of
  primitives in a single draw call. If `divisor` is zero, the attribute
  at slot `index` advances once per vertex. If `divisor` is non-zero,
  the attribute advances once per `divisor` instances of the set(s) of
  vertices being rendered. An attribute is referred to as instanced if
  its [`gl::VERTEX_ATTRIB_ARRAY_DIVISOR`] value is non-zero.
`index` must be less than the value of [`gl::MAX_VERTEX_ATTRIBS`].

## Notes
[`Gl::vertex_attrib_divisor`] is available only if the GL version is
  3.3 or higher.

## Errors
- [`gl::INVALID_VALUE`] is generated if `index` is greater than or equal
  to the value of [`gl::MAX_VERTEX_ATTRIBS`].

## See Also
- [`Gl::vertex_attrib_pointer`]
- [`Gl::enable_vertex_attrib_array`]
