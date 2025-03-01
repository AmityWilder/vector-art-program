# cross
calculate the cross product of two vectors

## Parameters
- `x`
  Specifies the first of two vectors

## Description
[`Gl::cross`] returns the cross product of two vectors, `x` and `y`,
  i.e. $\begin{pmatrix} { x[1] \times y[2] - y[1] \times x[2] } \\ {
  x[2] \times y[0] - y[2] \times x[0] } \\ { x[0] \times y[1] - y[0]
  \times x[1] } \end{pmatrix}$.

## See Also
- [`Gl::dot`]
