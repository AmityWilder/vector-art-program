# glGetObjectPtrLabel
retrieve the label of a sync object identified by a pointer

## Parameters
- `ptr`
  The name of the sync object whose label to retrieve.

## Description
[`Gl::get_object_ptr_label`] retrieves the label of the sync object
  identified by `ptr`.
`label` is the address of a string that will be used to store the
  object label. `bufSize` specifies the number of characters in the
  array identified by `label`. `length` contains the address of a
  variable which will receive the number of characters in the object
  label. If `length` is NULL, then it is ignored and no data is written.
  Likewise, if `label` is NULL, or if `bufSize` is zero then no data is
  written to `label`.

## Errors
- [`gl::INVALID_ENUM`] is generated if `identifier` is not one of the
  accepted object types.
- [`gl::INVALID_VALUE`] is generated if `ptr` is not the name of an
  existing sync object.
- [`gl::INVALID_VALUE`] is generated if `bufSize` is zero.
- If not NULL, `length` and `label` should be addresses to which the
  client has write access, otherwise undefined behavior, including
  process termination may occur.

## See Also
- [`Gl::push_debug_group`]
- [`Gl::pop_debug_group`]
- [`Gl::object_label`]
- [`Gl::get_object_label`]
