# refract
calculate the refraction direction for an incident vector

## Parameters
- `I`
  Specifies the incident vector.

## Description
For a given incident vector `I`, surface normal `N` and ratio of
  indices of refraction, `eta`, [`Gl::refract`] returns the refraction
  vector, `R`.
`R` is calculated as:
```c k = 1.0 - eta * eta * (1.0 - dot(N, I) * dot(N, I)); if (k < 0.0)
  R = genType(0.0); // or genDType(0.0) else R = eta * I - (eta * dot(N,
  I) + sqrt(k)) * N; ```
The input parameters `I` and `N` should be normalized in order to
  achieve the desired result.

## See Also
- [`Gl::dot`]
- [`Gl::reflect`]
