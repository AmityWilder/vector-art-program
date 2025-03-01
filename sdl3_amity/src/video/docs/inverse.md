# inverse
calculate the inverse of a matrix

## Parameters
- `m`
  Specifies the matrix of which to take the inverse.

## Description
[`Gl::inverse`] returns the inverse of the matrix `m`. The values in
  the returned matrix are undefined if `m` is singular or poorly-
  conditioned (nearly singular).

## See Also
- [`Gl::transpose`]
- [`Gl::determinant`]
