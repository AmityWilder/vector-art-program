# glIsSync
determine if a name corresponds to a sync object

## Parameters
- `sync`
  Specifies a value that may be the name of a sync object.

## Description
[`Gl::is_sync`] returns [`gl::TRUE`] if `sync` is currently the name
  of a sync object. If `sync` is not the name of a sync object, or if an
  error occurs, [`Gl::is_sync`] returns [`gl::FALSE`]. Note that zero is
  not the name of a sync object.

## Notes
[`Gl::is_sync`] is available only if the GL version is 3.2 or greater.

## See Also
- [`Gl::fence_sync`]
- [`Gl::wait_sync`]
- [`Gl::client_wait_sync`]
- [`Gl::delete_sync`]
