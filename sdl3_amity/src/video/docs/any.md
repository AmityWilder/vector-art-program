# any
check whether any element of a boolean vector is true

## Parameters
- `x`
  Specifies the vector to be tested for truth.

## Description
[`Gl::any`] returns true if any element of `x` is true and false
  otherwise. It is functionally equivalent to:
```c bool any(bvec x) { // bvec can be bvec2, bvec3 or bvec4 bool
  result = false; int i; for (i = 0; i < x.length(); ++i) { result |=
  x[i]; } return result; } ```

## See Also
- [`Gl::all`]
- [`Gl::not`]
