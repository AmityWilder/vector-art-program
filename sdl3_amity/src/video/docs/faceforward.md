# faceforward
return a vector pointing in the same direction as another

## Parameters
- `N`
  Specifies the vector to orient.

## Description
[`Gl::faceforward`] orients a vector to point away from a surface as
  defined by its normal. If [`Gl::dot`] ```c ( ``` `Nref`, `I`) <
  0[`Gl::faceforward`] returns `N`, otherwise it returns ```c - ``` .
  `N`

## See Also
- [`Gl::reflect`]
- [`Gl::refract`]
