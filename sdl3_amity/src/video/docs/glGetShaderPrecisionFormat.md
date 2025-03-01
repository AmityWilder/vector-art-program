# glGetShaderPrecisionFormat
retrieve the range and precision for numeric formats supported by the
  shader compiler

## Parameters
- `shaderType`
  Specifies the type of shader whose precision to query. `shaderType`
  must be [`gl::VERTEX_SHADER`] or [`gl::FRAGMENT_SHADER`].

## Description
[`Gl::get_shader_precision_format`] retrieves the numeric range and
  precision for the implementation's representation of quantities in
  different numeric formats in specified shader type. `shaderType`
  specifies the type of shader for which the numeric precision and range
  is to be retrieved and must be one of [`gl::VERTEX_SHADER`] or
  [`gl::FRAGMENT_SHADER`]. `precisionType` specifies the numeric format
  to query and must be one of [`gl::LOW_FLOAT`], [`gl::MEDIUM_FLOAT`]
  [`gl::HIGH_FLOAT`], [`gl::LOW_INT`], [`gl::MEDIUM_INT`], or
  [`gl::HIGH_INT`].
`range` points to an array of two integers into which the format's
  numeric range will be returned. If min and max are the smallest values
  representable in the format, then the values returned are defined to
  be: `range`[0] = floor(log2(|min|)) and `range`[1] =
  floor(log2(|max|)).
`precision` specifies the address of an integer into which will be
  written the log2 value of the number of bits of precision of the
  format. If the smallest representable value greater than 1 is 1 +
  *eps*, then the integer addressed by `precision` will contain
  floor(-log2(eps)).

## Errors
- [`gl::INVALID_ENUM`] is generated if `shaderType` or `precisionType`
  is not an accepted value.

## See Also
