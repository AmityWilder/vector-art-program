# outerProduct
calculate the outer product of a pair of vectors

## Parameters
- `c`
  Specifies the parameter to be treated as a column vector.

## Description
[`Gl::outer_product`] treats the first parameter `c` as a column
  vector (matrix with one column) and the second parameter `r` as a row
  vector (matrix with one row) and does a linear algebraic matrix
  multiply `c` * `r`, yielding a matrix whose number of rows is the
  number of components in `c` and whose number of columns is the number
  of components in `r`.

## See Also
- [`Gl::dot`]
