# reflect
calculate the reflection direction for an incident vector

## Parameters
- `I`
  Specifies the incident vector.

## Description
For a given incident vector `I` and surface normal `N` [`Gl::reflect`]
  returns the reflection direction calculated as ```c None ``` . `I` -
  2.0 * [`Gl::dot`](`N`, `I`) * `N`
`N` should be normalized in order to achieve the desired result.

## See Also
- [`Gl::dot`]
- [`Gl::refract`]
