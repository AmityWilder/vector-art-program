# barrier
synchronize execution of multiple shader invocations

## Description
*Available only in the Tessellation Control and Compute Shaders*,
  [`Gl::barrier`] provides a partially defined order of execution
  between shader invocations. For any given static instance of
  [`Gl::barrier`], in a tessellation control shader, all invocations for
  a single input patch must enter it before any will be allowed to
  continue beyond it. For any given static instance of [`Gl::barrier`]
  in a compute shader, all invocations within a single work group must
  enter it before any are allowed to continue beyond it. This ensures
  that values written by one invocation prior to a given static instance
  of [`Gl::barrier`] can be safely read by other invocations after their
  call to the same static instance of [`Gl::barrier`]. Because
  invocations may execute in undefined order between these
  [`Gl::barrier`] calls, the values of a per-vertex or per-patch output
  variable, or any shared variable will be undefined in a number of
  cases.
[`Gl::barrier`] may only be placed inside the function ```c main() ```
  of the tessellation control shader, but may be placed anywhere in a
  compute shader. Calls to [`Gl::barrier`] may not be placed within any
  control flow. Barriers are also disallowed after a return statement in
  the function ```c main() ``` .
