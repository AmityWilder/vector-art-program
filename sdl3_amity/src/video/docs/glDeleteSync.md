# glDeleteSync
delete a sync object

## Parameters
- `sync`
  The sync object to be deleted.

## Description
[`Gl::delete_sync`] deletes the sync object specified by `sync`. If
  the fence command corresponding to the specified sync object has
  completed, or if no [`Gl::wait_sync`] or [`Gl::client_wait_sync`]
  commands are blocking on `sync`, the object is deleted immediately.
  Otherwise, `sync` is flagged for deletion and will be deleted when it
  is no longer associated with any fence command and is no longer
  blocking any [`Gl::wait_sync`] or [`Gl::client_wait_sync`] command. In
  either case, after [`Gl::delete_sync`] returns, the name `sync` is
  invalid and can no longer be used to refer to the sync object.
[`Gl::delete_sync`] will silently ignore a `sync` value of zero.

## Notes
[`Gl::sync`] is only supported if the GL version is 3.2 or greater, or
  if the ```c ARB_sync ``` extension is supported.

## Errors
- [`gl::INVALID_VALUE`] is generated if `sync` is neither zero or the
  name of a sync object.

## See Also
- [`Gl::fence_sync`]
- [`Gl::wait_sync`]
- [`Gl::client_wait_sync`]
