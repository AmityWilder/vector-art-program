# gluQuadricCallback
define a callback for a quadrics object

## Parameters
- `quad`
  Specifies the quadrics object (created with [`Gl::u_new_quadric`]).

## Description
[`Gl::u_quadric_callback`] is used to define a new callback to be used
  by a quadrics object. If the specified callback is already defined,
  then it is replaced. If `CallBackFunc` is NULL, then any existing
  callback is erased.
The one legal callback is [`GLU_ERROR`]:

## See Also
- [`Gl::u_error_string`]
- [`Gl::u_new_quadric`]
