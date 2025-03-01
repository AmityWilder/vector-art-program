# all
check whether all elements of a boolean vector are true

## Parameters
- `x`
  Specifies the vector to be tested for truth.

## Description
[`Gl::all`] returns true if all elements of `x` are true and false
  otherwise. It is functionally equivalent to:
```c bool all(bvec x) // bvec can be bvec2, bvec3 or bvec4 { bool
  result = true; int i; for (i = 0; i < x.length(); ++i) { result &=
  x[i]; } return result; } ```

## See Also
- [`Gl::any`]
- [`Gl::not`]
