# glMatrixMode
specify which matrix is the current matrix

## Parameters
- `mode`
  Specifies which matrix stack is the target for subsequent matrix
  operations. Three values are accepted: [`gl::MODELVIEW`],
  [`gl::PROJECTION`], and [`gl::TEXTURE`]. The initial value is
  [`gl::MODELVIEW`]. Additionally, if the ```c ARB_imaging ``` extension
  is supported, [`gl::COLOR`] is also accepted.

## Description
[`Gl::matrix_mode`] sets the current matrix mode. `mode` can assume
  one of four values:
To find out which matrix stack is currently the target of all matrix
  operations, call [`Gl::get`] with argument [`gl::MATRIX_MODE`]. The
  initial value is [`gl::MODELVIEW`].

## Errors
- [`gl::INVALID_ENUM`] is generated if `mode` is not an accepted value.
- [`gl::INVALID_OPERATION`] is generated if [`Gl::matrix_mode`] is
  executed between the execution of [`Gl::begin`] and the corresponding
  execution of [`Gl::end`].

## See Also
- [`Gl::load_matrix`]
- [`Gl::load_transpose_matrix`]
- [`Gl::mult_matrix`]
- [`Gl::mult_transpose_matrix`]
- [`Gl::pop_matrix`]
- [`Gl::push_matrix`]
