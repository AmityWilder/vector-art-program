# glBlendColor
set the blend color

## Parameters
- `red`
  specify the components of [`gl::BLEND_COLOR`]

## Description
The [`gl::BLEND_COLOR`] may be used to calculate the source and
  destination blending factors. The color components are clamped to the
  range $None$ before being stored. See $$ $$ ```c ``` 0 1
  [`Gl::blend_func`] for a complete description of the blending
  operations. Initially the [`gl::BLEND_COLOR`] is set to (0, 0, 0, 0).

## Notes
The type of the `red`, `green`, `blue`, and `alpha` parameters was
  changed from GLclampf to GLfloat. This change is transparent to user
  code and is described in detail on the [`Gl::removed_types`] page.

## See Also
- [`Gl::blend_equation`]
- [`Gl::blend_func`]
- [`Gl::get_string`]
- [`Gl::removed_types`]
