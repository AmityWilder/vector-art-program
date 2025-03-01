# glPushMatrix
push and pop the current matrix stack

## Description
There is a stack of matrices for each of the matrix modes. In
  [`gl::MODELVIEW`] mode, the stack depth is at least 32. In the other
  modes, [`gl::COLOR`], [`gl::PROJECTION`], and [`gl::TEXTURE`], the
  depth is at least 2. The current matrix in any mode is the matrix on
  the top of the stack for that mode.
[`Gl::push_matrix`] pushes the current matrix stack down by one,
  duplicating the current matrix. That is, after a [`Gl::push_matrix`]
  call, the matrix on top of the stack is identical to the one below it.
[`Gl::pop_matrix`] pops the current matrix stack, replacing the
  current matrix with the one below it on the stack.
Initially, each of the stacks contains one matrix, an identity matrix.
It is an error to push a full matrix stack or to pop a matrix stack
  that contains only a single matrix. In either case, the error flag is
  set and no other change is made to GL state.

## Errors
- [`gl::STACK_OVERFLOW`] is generated if [`Gl::push_matrix`] is called
  while the current matrix stack is full.
- [`gl::STACK_UNDERFLOW`] is generated if [`Gl::pop_matrix`] is called
  while the current matrix stack contains only a single matrix.
- [`gl::INVALID_OPERATION`] is generated if [`Gl::push_matrix`] or
  [`Gl::pop_matrix`] is executed between the execution of [`Gl::begin`]
  and the corresponding execution of [`Gl::end`].

## See Also
- [`Gl::frustum`]
- [`Gl::load_identity`]
- [`Gl::load_matrix`]
- [`Gl::load_transpose_matrix`]
- [`Gl::matrix_mode`]
- [`Gl::mult_matrix`]
- [`Gl::mult_transpose_matrix`]
- [`Gl::ortho`]
- [`Gl::rotate`]
- [`Gl::scale`]
- [`Gl::translate`]
- [`Gl::viewport`]
