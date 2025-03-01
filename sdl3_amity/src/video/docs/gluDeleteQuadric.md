# gluDeleteQuadric
destroy a quadrics object

## Parameters
- `quad`
  Specifies the quadrics object to be destroyed.

## Description
[`Gl::u_delete_quadric`] destroys the quadrics object (created with
  [`Gl::u_new_quadric`]) and frees any memory it uses. Once
  [`Gl::u_delete_quadric`] has been called, `quad` cannot be used again.

## See Also
- [`Gl::u_new_quadric`]
