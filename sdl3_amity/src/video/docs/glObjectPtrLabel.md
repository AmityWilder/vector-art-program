# glObjectPtrLabel
label a sync object identified by a pointer

## Parameters
- `ptr`
  A pointer identifying a sync object.

## Description
[`Gl::object_ptr_label`] labels the sync object identified by `ptr`.
`label` is the address of a string that will be used to label the
  object. `length` contains the number of characters in `label`. If
  `length` is negative, it is implied that `label` contains a null-
  terminated string. If `label` is NULL, any debug label is effectively
  removed from the object.

## Errors
- [`gl::INVALID_VALUE`] is generated if `ptr` is not a valid sync
  object.
- [`gl::INVALID_VALUE`] is generated if the number of characters in
  `label`, excluding the null terminator when `length` is negative, is
  greater than the value of [`gl::MAX_LABEL_LENGTH`].

## See Also
- [`Gl::push_debug_group`]
- [`Gl::pop_debug_group`]
- [`Gl::object_label`]
