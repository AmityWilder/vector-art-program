# glHint
specify implementation-specific hints

## Parameters
- `target`
  Specifies a symbolic constant indicating the behavior to be
  controlled. [`gl::LINE_SMOOTH_HINT`], [`gl::POLYGON_SMOOTH_HINT`],
  [`gl::TEXTURE_COMPRESSION_HINT`], and
  [`gl::FRAGMENT_SHADER_DERIVATIVE_HINT`] are accepted.

## Description
Certain aspects of GL behavior, when there is room for interpretation,
  can be controlled with hints. A hint is specified with two arguments.
  `target` is a symbolic constant indicating the behavior to be
  controlled, and `mode` is another symbolic constant indicating the
  desired behavior. The initial value for each `target` is
  [`gl::DONT_CARE`]. `mode` can be one of the following:
Though the implementation aspects that can be hinted are well defined,
  the interpretation of the hints depends on the implementation. The
  hint aspects that can be specified with `target`, along with suggested
  semantics, are as follows:

## Notes
The interpretation of hints depends on the implementation. Some
  implementations ignore [`Gl::hint`] settings.

## Errors
- [`gl::INVALID_ENUM`] is generated if either `target` or `mode` is not
  an accepted value.

## See Also
