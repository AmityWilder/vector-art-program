# glGetProgramInterface
query a property of an interface in a program

## Parameters
- `program`
  The name of a program object whose interface to query.

## Description
[`Gl::get_program_interfaceiv`] queries the property of the interface
  identifed by `programInterface` in `program`, the property name of
  which is given by `pname`.
`program` must be the name of an existing program object.
  `programInterface` is the name of the interface within `program` to
  query and must be one of the following values:
`pname` identifies the property of `programInterface` to return in
  `params`.
If `pname` is [`gl::ACTIVE_RESOURCES`], the value returned is the
  number of resources in the active resource list for
  `programInterface`. If the list of active resources for
  `programInterface` is empty, zero is returned.
If `pname` is [`gl::MAX_NAME_LENGTH`], the value returned is the
  length of the longest active name string for an active resource in
  `programInterface`. This length includes an extra character for the
  null terminator. If the list of active resources for
  `programInterface` is empty, zero is returned. It is an error to
  specify [`gl::MAX_NAME_LENGTH`] when `programInterface` is
  [`gl::ATOMIC_COUNTER_BUFFER`], as active atomic counter buffer
  resources are not assigned name strings.
If `pname` is [`gl::MAX_NUM_ACTIVE_VARIABLES`], the value returned is
  the number of active variables belonging to the interface block or
  atomic counter buffer resource in `programInterface` with the most
  active variables. If the list of active resources for
  `programInterface` is empty, zero is returned. When `pname` is
  [`gl::MAX_NUM_ACTIVE_VARIABLES`], `programInterface` must be
  [`gl::UNIFORM_BLOCK`], [`gl::ATOMIC_COUNTER_BUFFER`], or
  [`gl::SHADER_STORAGE_BLOCK`].
If `pname` is [`gl::MAX_NUM_COMPATIBLE_SUBROUTINES`], the value
  returned is the number of compatible subroutines belonging to the
  active subroutine uniform in `programInterface` with the most
  compatible subroutines. If the list of active resources for
  `programInterface` is empty, zero is returned. When `pname` is
  [`gl::MAX_NUM_COMPATIBLE_SUBROUTINES`], `programInterface` must be one
  of [`gl::VERTEX_SUBROUTINE_UNIFORM`],
  [`gl::TESS_CONTROL_SUBROUTINE_UNIFORM`],
  [`gl::TESS_EVALUATION_SUBROUTINE_UNIFORM`],
  [`gl::GEOMETRY_SUBROUTINE_UNIFORM`],
  [`gl::FRAGMENT_SUBROUTINE_UNIFORM`], or
  [`gl::COMPUTE_SUBROUTINE_UNIFORM`].

## Errors
- [`gl::INVALID_ENUM`] is generated if `identifier` is not one of the
  accepted object types.
- [`gl::INVALID_VALUE`] is generated if `program` is not the name of an
  existing sync object.
- [`gl::INVALID_VALUE`] is generated if `bufSize` is zero.
- [`gl::INVALID_OPERATION`] is generated if `pname` is
  [`gl::MAX_NAME_LENGTH`] and `programInterface` is
  [`gl::ATOMIC_COUNTER_BUFFER`] or [`gl::TRANSFORM_FEEDBACK_BUFFER`],
  since active atomic counter and transform feedback buffer resources
  are not assigned name strings.
- [`gl::INVALID_OPERATION`] error is generated if `pname` is
  [`gl::MAX_NUM_ACTIVE_VARIABLES`] and `programInterface` is not
  [`gl::UNIFORM_BLOCK`], [`gl::SHADER_STORAGE_BLOCK`],
  [`gl::ATOMIC_COUNTER_BUFFER`], or [`gl::TRANSFORM_FEEDBACK_BUFFER`].
- If not NULL, `length` and `label` should be addresses to which the
  client has write access, otherwise undefined behavior, including
  process termination may occur.

## See Also
- [`Gl::push_debug_group`]
- [`Gl::pop_debug_group`]
- [`Gl::object_label`]
- [`Gl::get_object_label`]
