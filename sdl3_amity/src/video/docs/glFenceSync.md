# glFenceSync
create a new sync object and insert it into the GL command stream

## Parameters
- `condition`
  Specifies the condition that must be met to set the sync object's
  state to signaled. `condition` must be
  [`gl::SYNC_GPU_COMMANDS_COMPLETE`].

## Description
[`Gl::fence_sync`] creates a new fence sync object, inserts a fence
  command into the GL command stream and associates it with that sync
  object, and returns a non-zero name corresponding to the sync object.
When the specified `condition` of the sync object is satisfied by the
  fence command, the sync object is signaled by the GL, causing any
  [`Gl::wait_sync`], [`Gl::client_wait_sync`] commands blocking in
  `sync` to *unblock*. No other state is affected by [`Gl::fence_sync`]
  or by the execution of the associated fence command.
`condition` must be [`gl::SYNC_GPU_COMMANDS_COMPLETE`]. This condition
  is satisfied by completion of the fence command corresponding to the
  sync object and all preceding commands in the same command stream. The
  sync object will not be signaled until all effects from these commands
  on GL client and server state and the framebuffer are fully realized.
  Note that completion of the fence command occurs once the state of the
  corresponding sync object has been changed, but commands waiting on
  that sync object may not be unblocked until after the fence command
  completes.

## Notes
[`Gl::fence_sync`] is only supported if the GL version is 3.2 or
  greater, or if the ```c ARB_sync ``` extension is supported.

## Errors
- [`gl::INVALID_ENUM`] is generated if `condition` is not
  [`gl::SYNC_GPU_COMMANDS_COMPLETE`].
- [`gl::INVALID_VALUE`] is generated if `flags` is not zero.
- Additionally, if [`Gl::fence_sync`] fails, it will return zero.

## See Also
- [`Gl::delete_sync`]
- [`Gl::get_sync`]
- [`Gl::wait_sync`]
- [`Gl::client_wait_sync`]
